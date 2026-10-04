//! B-08a: assemble one composition frame from a saved project and render it.
//!
//! Every unit before this one was reachable only from a test. The transform, the exposure map,
//! the decoder, the blend modes and the compositor each work, and nothing joined them to a
//! project a person could open. This module is that join, and nothing more: it is document 20's
//! evaluation order at one frame, ending in the [`crate::render::FramePlan`] the renderer
//! already consumes.
//!
//! Document 20's order, and where each step is:
//!
//! 1. validate the composition frame — [`plan_frame`]'s first two checks
//! 2. snapshot the document revision — the caller holds `&Project`; nothing here mutates
//! 3. resolve layer order — [`crate::model::Composition::layers_in_order`], bottom first
//! 4. derive the layer-local frame — [`crate::time::LayerTiming::local_frame`]
//! 5. resolve the exposure and the source drawing — [`crate::time::resolve_in`]
//! 6. evaluate animated properties — [`crate::model::Transform::value_at`]
//! 7. per-layer source, transform and opacity — document 21 steps 1, 4 and 6
//! 8. composite the ordered result — [`crate::render::render`]
//!
//! Steps 2 and 5 of document 21 - mask and matte - arrived with B-06, and step 3, the effect
//! stack, with B-07. An effect this build does not have is still bypassed, and says so per frame
//! rather than rendering silently.
//!
//! Nothing here is the viewer. There is no transport, no playback, no work area and no window:
//! those are the rest of B-08 and they need decisions this build has not been given.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::cache::CelCache;
use crate::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use crate::model::{AssetKind, Id, Project, Prop, Value};
use crate::preview::PreviewQuality;
use crate::render::{self, Affine, FramePlan, LayerDraw};
use crate::time::{self, ExposureMap, SourceAt};
use crate::{ImageBuffer, WorkingBuffer};

/// The tile size this build uses when the caller has no opinion.
///
/// Document 21: "Tile size is a tunable measured on the reference machine, not a constant chosen
/// in advance." It was measured. `verification/B-05a_scaling_table.md` renders the same frame at
/// four tile sizes and two thread counts: 128 pixels is the fastest at 12 threads (17.1 ms) and
/// within a millisecond of the best at 24, and every size produced byte-identical output. This
/// constant is that measurement, not a guess, and moving it changes speed only.
pub const DEFAULT_TILE_SIZE: usize = 128;

/// The tile size a draft preview is cut into (P-03(f)).
///
/// [`DEFAULT_TILE_SIZE`] was measured on a 1920x1080 frame. A draft preview is a quarter of that
/// in each direction, and 128 pixels cuts 480x270 into four columns of three: twelve pieces of
/// work for twenty-four hardware threads, so half the machine waits however fast each piece is.
///
/// 48 pixels is that measurement redone at the draft extent by
/// `verification/P-03f_draft_tile_size.md`, which sweeps seven sizes twice on both fixtures and
/// compares every render against the 128px one byte for byte. It is the size that is best or
/// within noise of best in all four columns, at sixty tiles; the sizes below it buy nothing more.
/// Export is not affected - `DEFAULT_TILE_SIZE` is what an export still renders in, measured on
/// the extent an export renders at - and no tile size may change a picture, which is ADR-011 and
/// what the byte comparison in that artifact re-checks.
pub const DRAFT_TILE_SIZE: usize = 48;

/// Document 20's evaluation order at one frame: a project and a frame number in, a frame plan out.
///
/// `root` is the directory the project's relative media paths are resolved against — the project
/// file's own directory. `log` collects the per-layer diagnostics: a frame is not abandoned
/// because one layer's drawing is missing, so those are recorded and the frame renders without
/// that layer, exactly as document 20 requires of a sequence gap.
///
/// The `Err` cases are the two the caller got wrong, not the media: a composition that is not in
/// the project, and a frame the composition does not contain.
pub fn plan_frame(
    project: &Project,
    composition_id: &Id,
    frame: i32,
    root: &Path,
    log: &mut FrameLog,
) -> Result<FramePlan, Diagnostic> {
    plan_frame_cached(
        project,
        composition_id,
        frame,
        root,
        log,
        &mut CelCache::none(),
    )
}

/// [`plan_frame`], with somewhere to remember the decoded cels (B-08b, R-06b).
///
/// The only difference between this and [`plan_frame`] is where a decoded cel comes from, and
/// document 27 requires that difference to be invisible in the result: "A cold render and a fully
/// warm render for the same immutable request must produce equivalent pixels and diagnostics."
/// `verification/B-08b_cache_table.md` checks that as a byte comparison over every frame of the
/// reference shot, at several budgets, rather than asserting it.
///
/// ADR-015 confines the cache to the preview path, which is why this is a second function and not
/// a parameter added to the first: [`plan_frame`] and [`crate::export`] pass
/// [`CelCache::none`], so no exported sample can depend on what was remembered.
pub fn plan_frame_cached(
    project: &Project,
    composition_id: &Id,
    frame: i32,
    root: &Path,
    log: &mut FrameLog,
    cache: &mut CelCache,
) -> Result<FramePlan, Diagnostic> {
    plan_frame_at(
        project,
        composition_id,
        frame,
        root,
        PreviewQuality::Full,
        log,
        cache,
    )
}

/// [`plan_frame_cached`], told what the plan is for (D-67).
///
/// The plan itself is always at full size and [`crate::preview::scale_plan`] makes it a draft.
/// What `quality` changes is a composition layer's picture: D-67 renders the inner composition
/// at the draft divisor too, so a draft frame never pays for a full-size frame inside it. At
/// `Full` this is `plan_frame_cached` exactly.
pub fn plan_frame_at(
    project: &Project,
    composition_id: &Id,
    frame: i32,
    root: &Path,
    quality: PreviewQuality,
    log: &mut FrameLog,
    cache: &mut CelCache,
) -> Result<FramePlan, Diagnostic> {
    plan_inside(
        project,
        composition_id,
        frame,
        root,
        quality,
        log,
        cache,
        &mut Vec::new(),
        false,
    )
}

/// B-46: [`plan_frame_at`] for a frame the graphics card will draw. A drawn layer whose stack
/// ends in a Radial Blur, (B-47) a Bloom or (B-49) a Directional Blur, has the effects before it
/// run here and the last left in [`LayerDraw::on_card`] for the card. [`render::render`] runs an
/// effect left there itself, so the plan is still the same frame if the CPU draws it after all.
pub fn plan_frame_for_card(
    project: &Project,
    composition_id: &Id,
    frame: i32,
    root: &Path,
    quality: PreviewQuality,
    log: &mut FrameLog,
    cache: &mut CelCache,
) -> Result<FramePlan, Diagnostic> {
    plan_inside(
        project,
        composition_id,
        frame,
        root,
        quality,
        log,
        cache,
        &mut Vec::new(),
        true,
    )
}

/// `above` is the compositions this frame is being drawn inside of, outermost first. The
/// loader and the commands refuse a composition cycle (D-67), so it is only ever a guard: a
/// cycle reached some other way ends the render instead of hanging it.
#[allow(clippy::too_many_arguments)]
/// B-171 (G12, D-243): everything a frame of `inner` is drawn from other than the files it reads,
/// as text: this build of the program, the folder, the quality, the frame, every composition it
/// can reach and every asset, as they stand. `None` for a build whose identity cannot be read, and
/// for a composition that reaches one it is being drawn inside, whose frame depends on where.
// ponytail: the whole text is made on every inner frame asked for, which is a few kilobytes;
// digest it once per edit if a project with thousands of assets makes it show.
fn inner_key(
    project: &Project,
    inner: &Id,
    root: &Path,
    quality: PreviewQuality,
    frame: i32,
    above: &[Id],
    outer: &Id,
) -> Option<String> {
    let mut reach = vec![inner.clone()];
    let mut next = 0;
    while let Some(id) = reach.get(next).cloned() {
        next += 1;
        for id in project.composition(&id).into_iter().flat_map(|c| c.layers_in_order()).filter_map(|l| l.composition_id.as_ref()) {
            if !reach.contains(id) {
                reach.push(id.clone());
            }
        }
    }
    if reach.iter().any(|id| id == outer || above.contains(id)) {
        return None;
    }
    let mut key = format!("{}\n{root:?}\n{quality:?}\n{frame}\n", crate::cache::build()?);
    for id in &reach {
        key.push_str(&format!("{:?}\n", project.composition(id)));
    }
    key.push_str(&format!("{:?}", project.assets));
    Some(key)
}

fn plan_inside(
    project: &Project,
    composition_id: &Id,
    frame: i32,
    root: &Path,
    quality: PreviewQuality,
    log: &mut FrameLog,
    cache: &mut CelCache,
    above: &mut Vec<Id>,
    card: bool,
) -> Result<FramePlan, Diagnostic> {
    let Some(comp) = project.composition(composition_id) else {
        return Err(Diagnostic::new(
            DiagnosticId::CommandTargetMissing,
            Severity::Error,
            format!(
                "No composition {} in this project.",
                composition_id.as_str()
            ),
            "A frame was requested of a composition the project does not contain.".to_string(),
        ));
    };

    // Step 1: validate the composition frame. Document 28 names no identifier for a render
    // request outside the composition's own range; D-26 registers that gap and this reuse.
    let span = time::Composition {
        start_frame: comp.start_frame,
        duration_frames: comp.duration_frames,
        frame_rate: comp.frame_rate,
    };
    if !span.contains(frame) {
        return Err(Diagnostic::new(
            DiagnosticId::CommandInvalidValue,
            Severity::Error,
            format!(
                "Frame {frame} is outside {}, which runs {} to {}.",
                comp.name,
                span.start_frame,
                span.last_frame()
            ),
            "The frame is not clamped to the nearest end: a request for a frame the composition \
             does not have is a mistake in the caller, and rendering the nearest one it does have \
             would hide it."
                .to_string(),
        ));
    }

    let matte_only = matte_only(comp);

    // P-03(b): decode this frame's cels together, before the loop that wants them one at a
    // time. `CelCache::prewarm` skips what it already holds, decodes one file once however many
    // layers name it, and quietly leaves anything unusual to the loop, which diagnoses it.
    cache.prewarm(&cels_at(project, comp, frame, root));

    // D-58: each drawn layer with the plane it ends up on, so the vector can be put in draw
    // order once every layer is resolved.
    let mut layers: Vec<(f64, LayerDraw)> = Vec::new();
    for (prop, e) in camera_expression_failures(comp, frame) {
        log.record(
            frame,
            format!("camera/{prop}"),
            e.diagnostic("the camera", prop, frame),
        );
    }
    // Step 3: composition order, bottom of the stack first, which is `FramePlan.layers`' order.
    for layer in comp.layers_in_order() {
        if !layer.enabled || matte_only.contains(&&layer.id) {
            continue;
        }
        let Some(mut resolved) = resolve_layer(
            project, comp, layer, frame, root, quality, cache, log, above, card, false,
        ) else {
            continue;
        };
        // B-156b (D-226): a plan for the card leaves a drawn layer's moments for the card to add
        // up, with the transform `settle` would have given the average.
        let (resolved, moments) = if card && !layer.is_adjustment() && !resolved.moments.is_empty() {
            let d = quality.divisor() as f64;
            let moments = std::mem::take(&mut resolved.moments);
            let transform = if d == 1.0 { Affine::IDENTITY } else { Affine::scaling(d, d) };
            (ResolvedLayer { transform, ..resolved }, moments)
        } else {
            (resolved, Vec::new())
        };
        let mut motion_blur = !resolved.moments.is_empty();
        let mut mixed = resolved.mixed;
        let resolved = settle(resolved, comp, quality, false);

        // Document 21 step 5. The matte layer is looked up whether or not it is enabled or
        // matte-only: neither of those stops it shaping this layer, they only decide whether it
        // is also drawn on its own.
        let matte = match &layer.matte {
            None => None,
            Some(reference) => match comp.layer(&reference.layer_id) {
                Some(matte_layer) => {
                    // Only one level deep, and deliberately: document 21 evaluates the matte
                    // layer "through its own source, mask, effects and transform", which does
                    // not include its own matte. A matte's matte is not a chain, so there is
                    // nothing here to recurse into and no cycle to guard against.
                    resolve_layer(
                        project,
                        comp,
                        matte_layer,
                        frame,
                        root,
                        quality,
                        cache,
                        log,
                        above,
                        false,
                        false,
                    )
                    .map(|m| {
                        motion_blur |= !m.moments.is_empty();
                        mixed |= m.mixed;
                        let m = settle(m, comp, quality, true);
                        Box::new(matte_coverage(
                            render::MatteDraw { source: m.source, transform: m.transform },
                            reference.mode,
                            comp,
                        ))
                    })
                }
                None => {
                    // Document 28: MATTE_REFERENCE_MISSING is a WARNING that preserves the
                    // reference and renders a defined fallback. The fallback defined here is
                    // unmatted, and it is the safe direction: an unresolved matte that hid the
                    // layer would look exactly like a layer someone had turned off, while an
                    // unmatted layer looks wrong in a way that sends you to the log.
                    log.record(
                        frame,
                        layer.name.clone(),
                        Diagnostic::new(
                            DiagnosticId::MatteReferenceMissing,
                            Severity::Warning,
                            format!(
                                "Layer {} uses layer {} as a matte, which is not in this \
                                 composition.",
                                layer.name,
                                reference.layer_id.as_str()
                            ),
                            "The matte reference is preserved in the project. This frame draws \
                             the layer unmatted, so it covers more than it will once the matte \
                             layer is back."
                                .to_string(),
                        )
                        .with_remediation(
                            "Put the matte layer back in this composition, or clear the matte.",
                        ),
                    );
                    None
                }
            },
        };

        layers.push((
            world_depth(comp, &layer.id, frame),
            LayerDraw {
                id: layer.id.clone(),
                source: resolved.source,
                transform: resolved.transform,
                opacity: resolved.opacity,
                matte,
                blend: layer.blend_mode,
                // D-182: with each lookup file read. What kept one from being read was said by
                // `resolve_rest`, which an adjustment layer goes through as well.
                adjust: layer.is_adjustment().then(|| {
                    let mut effects: Vec<_> = layer
                        .effects
                        .iter()
                        .map(|i| effect_now(comp, layer, i, frame, frame, frame as f64, log))
                        .collect();
                    crate::lut::fill(&mut effects, project, root, &layer.name);
                    // D-191: an adjustment layer's maps lie on the frame it runs on.
                    let size = quality.extent(comp.width as usize, comp.height as usize);
                    fill_maps(&mut effects, project, root, comp, layer, frame, quality, size, cache, log);
                    effects
                }),
                nested: resolved.nested,
                on_card: resolved.on_card,
                motion_blur,
                mixed,
                moments,
                wrap: if layer.is_adjustment() {
                    Vec::new()
                } else {
                    layer
                        .effects
                        .iter()
                        .filter(|i| matches!(i.effect, crate::effects::Effect::LightWrap { .. }))
                        .map(|i| {
                            let at = posterized(comp, layer, frame);
                            crate::expr::effect_at(comp, &layer.id, i, at, layer.key_time(at as f64)).0
                        })
                        .collect()
                },
            },
        ));
    }

    // D-58: far to near, and layers at equal depth keep composition order. A **stable** sort is
    // what gives that second half for free, so `layer_order` stays the authority for ties
    // without a word of code saying so.
    //
    // Sorted at every frame rather than once when the project opens, because a depth is an
    // animatable property: two planes can cross mid-shot and what is in front of what changes
    // with them. FX-CAM-010 is the case that fails if this moves anywhere cheaper.
    layers.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    Ok(FramePlan {
        width: comp.width as usize,
        height: comp.height as usize,
        layers: layers.into_iter().map(|(_, draw)| draw).collect(),
    })
}

/// One layer resolved to pixels: document 21's steps 1, 2, 4 and 6 for a single layer.
struct ResolvedLayer {
    source: std::sync::Arc<WorkingBuffer>,
    transform: Affine,
    opacity: f32,
    /// D-67: the composition and the frame of it that `source` is, for a composition layer.
    nested: Option<(Id, i32)>,
    /// B-46, B-47: the effects left for the graphics card, in stack order (B-155).
    on_card: Vec<render::OnCard>,
    /// D-188: step 4 at each moment of a motion-blurred layer's shutter, `None` for a moment it
    /// is behind the camera. Empty when it is taken once, through `transform`. [`settle`]
    /// averages them into `source`.
    moments: Vec<Option<Affine>>,
    /// D-216: `source` is a frame mix or a drawing dissolve, which the graphics card leaves to
    /// the CPU.
    mixed: bool,
}

/// Steps 1 through 6 of document 21 for one layer: find its drawing at this frame, decode it,
/// apply its mask in source space, and evaluate its transform and opacity.
///
/// `None` means the layer takes no part in this frame. Every reason for that is either recorded
/// in `log` first or is not a fault at all — a layer outside its own in/out range contributes
/// nothing and there is nothing to say about it.
///
/// This is called twice: once for a layer being drawn, and once for a layer being used as
/// another's matte. That second call is the whole reason it is a function. A matte layer has to
/// go through the same source resolution, the same decode and the same mask as a drawn one,
/// because document 21 says the matte layer is evaluated "through its own source, mask, effects
/// and transform" — and a matte that resolved its drawing by a different route would be a second
/// implementation of the first six steps, drifting from this one at whatever the next change is.
///
/// The caller decides what to do with `opacity`: a drawn layer applies it at step 6, and a matte
/// ignores it, because step 5 asks for the matte layer's post-transform *alpha* and opacity is a
/// later step about how a layer joins the stack.
/// D-293: a luma or inverted matte as the cover it gives, drawn once at the composition's size
/// through its own transform and kept as alpha, so every renderer reads it as it reads an alpha
/// matte. An alpha matte is returned as it is, so it draws exactly as before D-293.
fn matte_coverage(
    matte: render::MatteDraw,
    mode: crate::model::MatteMode,
    comp: &crate::model::Composition,
) -> render::MatteDraw {
    use crate::model::MatteMode as M;
    use rayon::prelude::*;
    if mode == M::Alpha {
        return matte;
    }
    let (w, h) = (comp.width as usize, comp.height as usize);
    let mut flat = render::average(&matte.source, &[Some(matte.transform)], w, h);
    flat.data_mut().par_chunks_exact_mut(4).for_each(|p| {
        let v = match mode {
            // The picture luma of a premultiplied pixel: its colour over black, encoded.
            M::Luma | M::LumaInverted => crate::grade::to_srgb(
                (0.2126 * p[0] as f64 + 0.7152 * p[1] as f64 + 0.0722 * p[2] as f64).clamp(0.0, 1.0),
            ),
            M::Alpha | M::AlphaInverted => p[3] as f64,
        };
        let v = if matches!(mode, M::AlphaInverted | M::LumaInverted) { 1.0 - v } else { v };
        p.copy_from_slice(&[0.0, 0.0, 0.0, v as f32]);
    });
    render::MatteDraw { source: std::sync::Arc::new(flat), transform: Affine::IDENTITY }
}

/// D-42: the layers some other layer uses as a matte-only source. They are still resolved as
/// mattes; what the flag buys is that they are not also drawn in their own right, which is
/// document 21's "not separately composited into the final stack".
fn matte_only(comp: &crate::model::Composition) -> Vec<&Id> {
    comp.layers_in_order()
        .filter_map(|l| l.matte.as_ref())
        .filter(|m| m.matte_only)
        .map(|m| &m.layer_id)
        .collect()
}

/// The cels `comp`'s own layers will ask for at `frame`, in the order its layer loop asks: each
/// drawn layer's cel, and the cel of any layer it uses as a matte, whether or not that one is
/// itself drawn (P-03(b)). This decides when decoding happens and nothing about which decoding
/// happens. The viewer also asks it for the frame after the one it just showed, to read that
/// frame's drawings while the page is busy with this one (P-18). Best effort, as
/// [`exposed_cel`] is: a composition layer's inner cels are left to its own frame.
pub fn cels_at(
    project: &Project,
    comp: &crate::model::Composition,
    frame: i32,
    root: &Path,
) -> Vec<(PathBuf, crate::model::Interpretation)> {
    let matte_only = matte_only(comp);
    let mut wanted = Vec::new();
    for layer in comp.layers_in_order() {
        if !layer.enabled || matte_only.contains(&&layer.id) {
            continue;
        }
        wanted.extend(exposed_cel(project, layer, posterized(comp, layer, frame), root));
        if let Some(matte) = &layer.matte {
            if let Some(matte_layer) = comp.layer(&matte.layer_id) {
                wanted.extend(exposed_cel(project, matte_layer, posterized(comp, matte_layer, frame), root));
            }
        }
    }
    wanted
}

/// Which file a layer shows at `frame`, and how to interpret it, or nothing.
///
/// Best effort on purpose (P-03(b)). This exists to tell [`CelCache::prewarm`] what to decode,
/// and every case it declines to answer -- an asset the project does not contain, an exposure
/// that resolves to no drawing, a file that is not where the project says it is -- is handled,
/// diagnosed and logged against the right layer by [`resolve_layer`] a moment later. The only
/// consequence of it returning nothing where `resolve_layer` would have found something is that
/// that one cel decodes serially, the way every cel did before P-03(b). It must therefore never
/// raise a diagnostic of its own: two diagnostics for one problem is worse than one.
fn exposed_cel(
    project: &Project,
    layer: &crate::model::Layer,
    frame: i32,
    root: &Path,
) -> Option<(PathBuf, crate::model::Interpretation)> {
    let asset = project.assets.iter().find(|a| a.id == layer.asset_id)?;
    let local = layer.source_time(frame)?.floor() as i32;
    let relative = source_at(layer.exposure_spans.clone(), asset, local, frame).ok()??;
    Some((root.join(relative), asset.interpretation))
}

/// D-57's `M_world` for a chain of layers, with the two things keeping place needs that the
/// matrix cannot give back: the chain's total rotation, and the product of its scales.
///
/// `uniform` and `unrotated` are what document 21's exactness rule is read from: the conversion
/// is exact when every layer in the chain scales equally in x and y, or when nothing along it
/// is turned.
#[derive(Clone, Copy)]
pub(crate) struct Chain {
    pub matrix: Affine,
    pub rotation: f64,
    pub scale: (f64, f64),
    pub uniform: bool,
    pub unrotated: bool,
}

impl Chain {
    pub const IDENTITY: Chain = Chain {
        matrix: Affine::IDENTITY,
        rotation: 0.0,
        scale: (1.0, 1.0),
        uniform: true,
        unrotated: true,
    };
}

/// The layers' transforms at `frame`, accumulated nearest first.
///
/// Every layer is read at this same composition frame whatever its own in and out points, its
/// switch or its exposures say (document 20, D-57): what a child inherits is a transform, not a
/// picture. A property holding the wrong kind of value is skipped rather than guessed at; the
/// layer's own transform reports that case one step further down.
///
/// D-188: `t` is the moment inside frame `frame`'s shutter the keys are read at, which is
/// `frame` itself everywhere but a motion-blurred layer's moments.
fn chain_of(
    comp: &crate::model::Composition,
    layers: &[&crate::model::Layer],
    frame: i32,
    t: f64,
) -> Chain {
    let mut chain = Chain::IDENTITY;
    for layer in layers {
        // D-59: a parent is inherited after its expressions. A failing one is reported by the
        // parent's own frame, not again by every child.
        let at = |prop| {
            let property = layer
                .transform
                .get(prop)
                .expect("the four are transform properties");
            let owner = || crate::expr::Target::Layer(layer.id.clone());
            // D-216: each layer of the chain by its own stretch.
            crate::expr::resolve_at(comp, property, owner, prop, frame, layer.key_time(t)).0
        };
        let (Some(anchor), Some(position), Some(scale), Some(rotation)) = (
            at(Prop::Anchor).as_vec2(),
            at(Prop::Position).as_vec2(),
            at(Prop::Scale).as_vec2(),
            at(Prop::Rotation).as_scalar(),
        ) else {
            continue;
        };
        chain.matrix = chain
            .matrix
            .then(Affine::from_transform(anchor, position, scale, rotation));
        chain.rotation += rotation;
        chain.scale.0 *= scale.0;
        chain.scale.1 *= scale.1;
        chain.uniform &= scale.0 == scale.1;
        chain.unrotated &= rotation == 0.0;
    }
    chain
}

/// D-57: the chain above `layer_id`, identity for a layer with no parent.
pub(crate) fn parent_chain_at(
    comp: &crate::model::Composition,
    layer_id: &crate::model::Id,
    frame: i32,
) -> Chain {
    chain_of(comp, &comp.parent_chain(layer_id), frame, frame as f64)
}

/// D-58's camera at one frame: where it is, how far back it sits, and its zoom.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct CameraAt {
    pub position: (f64, f64),
    pub depth: f64,
    pub zoom: f64,
}

/// The camera a composition is seen through at `frame`, which is the default one when the file
/// carries none (D-58).
///
/// `None` means a camera property holds the wrong kind of value, which persistence refuses, so
/// it can only come from a project built in memory. The caller reports it rather than guessing,
/// exactly as a layer's transform does.
pub fn camera_at(comp: &crate::model::Composition, frame: i32) -> Option<CameraAt> {
    camera_at_time(comp, frame, frame as f64)
}

/// D-188: [`camera_at`] at a moment `t` inside frame `frame`'s shutter, the layer it rides read
/// at the same moment.
pub fn camera_at_time(comp: &crate::model::Composition, frame: i32, t: f64) -> Option<CameraAt> {
    let default;
    let camera = match &comp.camera {
        Some(camera) => camera,
        None => {
            default = crate::model::Camera::default_for(comp.width, comp.height);
            &default
        }
    };
    let at = |prop, property| {
        crate::expr::resolve_at(comp, property, || crate::expr::Target::Camera, prop, frame, t).0
    };
    let (x, y) = at(Prop::Position, &camera.position).as_vec2()?;
    let depth = at(Prop::Depth, &camera.depth).as_scalar()?;
    let zoom = at(Prop::Zoom, &camera.zoom).as_scalar()?;
    // D-171: riding a layer moves where the camera stands and how far back, never the view.
    Some(match camera.parent.as_ref().filter(|p| comp.layer(p).is_some()) {
        Some(parent) => CameraAt {
            position: world_at_time(comp, parent, frame, t).matrix.apply(x, y),
            depth: depth + world_depth_at(comp, parent, frame, t),
            zoom,
        },
        None => CameraAt { position: (x, y), depth, zoom },
    })
}

/// D-59: every expression in `comp` that fails at `frame`, with the property it sits on. The
/// camera's are reported once a frame from here; a layer's are reported as that layer is drawn.
pub fn camera_expression_failures(
    comp: &crate::model::Composition,
    frame: i32,
) -> Vec<(Prop, crate::expr::ExprError)> {
    crate::expr::live_properties(comp)
        .into_iter()
        .filter(|(target, _)| *target == crate::expr::Target::Camera)
        .filter_map(|(target, prop)| {
            crate::expr::evaluate(comp, &target, prop, frame)
                .err()
                .map(|e| (prop, e))
        })
        .collect()
}

/// D-58's `world_depth(L) = depth(L) + world_depth(parent(L))`: the plane a layer ends up on,
/// its own depth plus every plane it rides on.
///
/// Absent means 0, and a depth holding the wrong kind of value contributes 0 rather than
/// stopping the walk: the layer's own transform is what reports a bad property, and a depth that
/// cannot be read leaves the layer on the plane it would have had without one.
pub fn world_depth(
    comp: &crate::model::Composition,
    layer_id: &crate::model::Id,
    frame: i32,
) -> f64 {
    world_depth_at(comp, layer_id, frame, frame as f64)
}

/// D-188: [`world_depth`] at a moment `t` inside frame `frame`'s shutter.
fn world_depth_at(
    comp: &crate::model::Composition,
    layer_id: &crate::model::Id,
    frame: i32,
    t: f64,
) -> f64 {
    let own = |layer: &crate::model::Layer| {
        layer
            .depth
            .as_ref()
            .and_then(|d| {
                let owner = || crate::expr::Target::Layer(layer.id.clone());
                crate::expr::resolve_at(comp, d, owner, Prop::Depth, frame, layer.key_time(t))
                    .0
                    .as_scalar()
            })
            .unwrap_or(0.0)
    };
    let mut total = comp.layer(layer_id).map_or(0.0, own);
    for parent in comp.parent_chain(layer_id) {
        total += own(parent);
    }
    total
}

/// What the camera does to a plane at `world_depth`.
///
/// Three outcomes and not two, because D-58 distinguishes the projection that is the identity
/// from the one that merely lands within a tolerance of it.
pub(crate) enum Projection {
    /// Leave it out. **Not** an identity matrix to multiply through**:** `960 + (x - 960) * 1.0`
    /// is not bitwise `x`, so applying one would put every fixture written before D-58 within a
    /// tolerance of its expected value instead of exactly on it. FX-CAM-001 is that case.
    Identity,
    Scaled(Affine),
    /// Level with the camera or behind it: no size, not drawn, `CAMERA_PLANE_BEHIND`.
    Behind,
}

/// D-58's two lines: `s = zoom / (world_depth - camera_depth)`, and
/// `screen = centre + (p_comp - camera_position) * s`.
///
/// A uniform scale about a point followed by a shift, which is affine on purpose: it composes
/// into the one matrix document 21 step 4 already samples through, so a far plane is minified
/// once by the bilinear filter rather than twice.
pub(crate) fn projection(cam: CameraAt, centre: (f64, f64), world_depth: f64) -> Projection {
    let ahead = world_depth - cam.depth;
    if ahead <= 0.0 {
        return Projection::Behind;
    }
    let s = cam.zoom / ahead;
    if s == 1.0 && cam.position == centre {
        return Projection::Identity;
    }
    Projection::Scaled(Affine::scaling(s, s).then(Affine::translation(
        centre.0 - cam.position.0 * s,
        centre.1 - cam.position.1 * s,
    )))
}

/// D-57's `M_world(L)`: the layer's own transform and then everything above it.
pub(crate) fn world_at(
    comp: &crate::model::Composition,
    layer_id: &crate::model::Id,
    frame: i32,
) -> Chain {
    world_at_time(comp, layer_id, frame, frame as f64)
}

fn world_at_time(
    comp: &crate::model::Composition,
    layer_id: &crate::model::Id,
    frame: i32,
    t: f64,
) -> Chain {
    let Some(layer) = comp.layer(layer_id) else {
        return Chain::IDENTITY;
    };
    let mut chain = vec![layer];
    chain.extend(comp.parent_chain(layer_id));
    chain_of(comp, &chain, frame, t)
}

/// D-57's `M_world(L)`: the map from a layer's own pixels into composition pixels at `frame`,
/// its parent chain included.
///
/// The same map the renderer builds at step 4, minus the translation an effect's bounds growth
/// adds, which is a fact about a buffer rather than about where the layer is. Public because
/// where a layer lands is a question worth asking of a project whose drawings are not at hand.
pub fn world_transform(
    comp: &crate::model::Composition,
    layer_id: &crate::model::Id,
    frame: i32,
) -> Affine {
    world_at(comp, layer_id, frame).matrix
}

/// D-58's `M_world(L)` carried on to the screen: where a layer's own pixels land in the
/// rendered frame at `frame`, its parent chain and the composition's camera included.
///
/// `None` when the layer is level with the camera or behind it, which is the frame in which it
/// is not drawn at all. A projection that is the identity is left out rather than applied, so a
/// composition with the default camera and no depths gives exactly [`world_transform`].
pub fn screen_transform(
    comp: &crate::model::Composition,
    layer_id: &crate::model::Id,
    frame: i32,
) -> Option<Affine> {
    screen_transform_at(comp, layer_id, frame, frame as f64)
}

/// D-188: [`screen_transform`] at a moment `t` inside frame `frame`'s shutter: the layer, its
/// parents and the camera each read at `t` (FX-MB-050).
pub fn screen_transform_at(
    comp: &crate::model::Composition,
    layer_id: &crate::model::Id,
    frame: i32,
    t: f64,
) -> Option<Affine> {
    let world = world_at_time(comp, layer_id, frame, t).matrix;
    let cam = camera_at_time(comp, frame, t)?;
    let centre = (comp.width as f64 / 2.0, comp.height as f64 / 2.0);
    match projection(cam, centre, world_depth_at(comp, layer_id, frame, t)) {
        Projection::Identity => Some(world),
        Projection::Scaled(p) => Some(world.then(p)),
        Projection::Behind => None,
    }
}

/// B-46..B-155: whether the card can draw `instance`, run at a draft divisor `pre`.
fn card_can(instance: &crate::effects::EffectInstance, pre: f64) -> bool {
    // D-202: an effect mixed below 100 is drawn here; a card version is a later unit.
    instance.mix == 100.0
        && matches!(
            instance.effect,
            crate::effects::Effect::RadialBlur { .. }
                | crate::effects::Effect::Bloom { .. }
                | crate::effects::Effect::DirectionalBlur { .. }
                | crate::effects::Effect::GaussianBlur { .. }
                | crate::effects::Effect::Glow { .. }
                | crate::effects::Effect::Curves { .. }
                | crate::effects::Effect::Levels { .. }
                | crate::effects::Effect::HueSaturation { .. }
                | crate::effects::Effect::Gradient { .. }
                | crate::effects::Effect::DropShadow { .. }
                | crate::effects::Effect::LensBlur { .. }
                | crate::effects::Effect::RimLight { .. }
                | crate::effects::Effect::Outline { .. }
                | crate::effects::Effect::Noise { .. }
                | crate::effects::Effect::ChromaticAberration { .. }
                | crate::effects::Effect::DistanceGradation { .. }
                | crate::effects::Effect::LightRays { .. }
                | crate::effects::Effect::ExposureFlicker { .. }
                | crate::effects::Effect::Vignette { .. }
                | crate::effects::Effect::TurbulentDisplace { .. }
                | crate::effects::Effect::FractalNoise { .. }
                | crate::effects::Effect::GradientMap { .. }
                | crate::effects::Effect::ColorBalance { .. }
                | crate::effects::Effect::Offset { .. }
                | crate::effects::Effect::Invert { .. }
                | crate::effects::Effect::BrightnessContrast { .. }
                | crate::effects::Effect::BlackWhite { .. }
                | crate::effects::Effect::Posterize { .. }
                | crate::effects::Effect::Threshold { .. }
                | crate::effects::Effect::ChannelMixer { .. }
                | crate::effects::Effect::Vibrance { .. }
                | crate::effects::Effect::LeaveColor { .. }
                | crate::effects::Effect::Solarize { .. }
                | crate::effects::Effect::Halftone { .. }
                | crate::effects::Effect::Mosaic { .. }
                | crate::effects::Effect::Emboss { .. }
                | crate::effects::Effect::FindEdges { .. }
                | crate::effects::Effect::Sharpen { .. }
                | crate::effects::Effect::Diffusion { .. }
                | crate::effects::Effect::WaveWarp { .. }
                | crate::effects::Effect::Ripple { .. }
                | crate::effects::Effect::Twirl { .. }
                | crate::effects::Effect::Bulge { .. }
                | crate::effects::Effect::Mirror { .. }
                | crate::effects::Effect::MotionTile { .. }
                | crate::effects::Effect::LinearWipe { .. }
                | crate::effects::Effect::RadialWipe { .. }
                | crate::effects::Effect::VenetianBlinds { .. }
                | crate::effects::Effect::IrisWipe { .. }
                | crate::effects::Effect::SimpleChoker { .. }
                | crate::effects::Effect::SpeedLines { .. }
                | crate::effects::Effect::CrossGlare { .. }
                | crate::effects::Effect::CameraShake { .. }
                | crate::effects::Effect::Rain { .. }
                | crate::effects::Effect::ColorLookup { .. }
                | crate::effects::Effect::LineBlur { .. }
                | crate::effects::Effect::HsvKey { .. }
                | crate::effects::Effect::Paraffin { .. }
                | crate::effects::Effect::KiraKira { .. }
                | crate::effects::Effect::Median { .. }
                | crate::effects::Effect::SmartBlur { .. }
                | crate::effects::Effect::RoughenEdges { .. }
                | crate::effects::Effect::RadialShadow { .. }
                | crate::effects::Effect::BevelAlpha { .. }
                | crate::effects::Effect::Snowfall { .. }
                | crate::effects::Effect::CellPattern { .. }
                | crate::effects::Effect::PolarCoordinates { .. }
                | crate::effects::Effect::OpticsCompensation { .. }
                | crate::effects::Effect::CornerPin { .. }
        )
        // D-122: a Levels whose input white is its black is a threshold, which a rounding
        // either side of would turn from black to white, so it stays on the CPU.
        && !matches!(instance.effect, crate::effects::Effect::Levels { input_black, input_white, .. } if input_black == input_white)
        // D-303: the card blurs along both axes only.
        // ponytail: give the card's two passes a still one if a one-way blur is slow here.
        && !matches!(&instance.effect, crate::effects::Effect::GaussianBlur { dimensions, .. } if dimensions != "both")
        // D-302: the card bends the colour only, so a Curves whose alpha curve bends is drawn here.
        // ponytail: give the card's grade a fifth curve if one is slow here.
        && !matches!(&instance.effect, crate::effects::Effect::Curves { alpha, .. } if !crate::grade::is_straight(alpha))
        // B-107: valid as it runs, at the draft's distances, since a draft can take a
        // distance below its least (a Rain's spacing), which the CPU then reports and skips.
        && {
            let mut effect = instance.effect.clone();
            effect.scale_distances(|d| d / pre);
            effect.is_valid()
        }
}

/// `effect`, `instance`'s at the draft's distances, as the card draws it on a drawing `size`
/// whose corner is at `offset`, which it moves by what the effect grows. `None` for one that
/// changes nothing and grows nothing. Not Bloom or Glow, which look at the drawing first.
fn card_effect(
    effect: crate::effects::Effect,
    instance: &crate::effects::EffectInstance,
    size: (usize, usize),
    offset: &mut (usize, usize),
) -> Option<render::OnCard> {
    match effect {
        crate::effects::Effect::RadialBlur { kind, amount, center, edges } => Some(render::OnCard::Radial(render::Radial {
            spin: kind == "spin",
            amount,
            center: crate::effects::radial_center(center, size, *offset),
            repeat: edges == "repeat",
        })),
        // B-49: length 0 changes nothing and grows nothing, so it is not left either.
        crate::effects::Effect::DirectionalBlur { direction, length, edges } => (length != 0.0).then(|| {
            let d = render::Directional { direction, length, repeat: edges == "repeat" };
            *offset = (offset.0 + d.grow(), offset.1 + d.grow());
            render::OnCard::Directional(d)
        }),
        // B-50: a sigma too small to reach a neighbour changes nothing, so it is not left either.
        crate::effects::Effect::GaussianBlur { sigma_px, edges, .. } => {
            (crate::effects::kernel_radius(sigma_px) != 0).then(|| {
                let g = render::Gaussian { sigma: sigma_px, repeat: edges == "repeat" };
                *offset = (offset.0 + g.grow(), offset.1 + g.grow());
                render::OnCard::Gaussian(g)
            })
        }
        // B-65: the batch of ten, run as `apply_stack` runs them, from the drawing's corner so
        // far. One that changes nothing and grows nothing, as each says of its settings, is
        // not left.
        effect => {
            use crate::effects::Effect as E;
            let nothing = match &effect {
                E::Curves { master, red, green, blue, alpha } => {
                    [master, red, green, blue, alpha].iter().all(|c| crate::grade::is_straight(c))
                }
                E::Levels { input_black, input_white, gamma, output_black, output_white } => {
                    [*input_black, *input_white, *gamma, *output_black, *output_white] == [0.0, 255.0, 1.0, 0.0, 255.0]
                }
                E::HueSaturation { hue, saturation, lightness } => [*hue, *saturation, *lightness] == [0.0; 3],
                E::Gradient { start_opacity, end_opacity, .. } => [*start_opacity, *end_opacity] == [0.0; 2],
                E::RimLight { intensity, .. } => *intensity == 0.0,
                E::Outline { width, .. } => *width == 0.0,
                E::Noise { amount, .. } | E::ChromaticAberration { amount, .. } => *amount == 0.0,
                E::DistanceGradation { width, opacity, .. } => *width == 0.0 || *opacity == 0.0,
                E::LightRays { intensity, .. } => *intensity == 0.0,
                E::ExposureFlicker { amount, .. }
                | E::Vignette { amount, .. }
                | E::TurbulentDisplace { amount, .. }
                | E::GradientMap { amount, .. } => *amount == 0.0,
                E::FractalNoise { opacity, .. } => *opacity == 0.0,
                E::ColorBalance { shadows, midtones, highlights, .. } => {
                    [shadows, midtones, highlights].iter().all(|t| t.iter().all(|v| *v == 0.0))
                }
                E::Offset { shift } => *shift == [0.0, 0.0],
                // B-107: the third batch, each as its own function returns at once.
                E::Invert { amount, .. }
                | E::LeaveColor { amount, .. }
                | E::Halftone { amount, .. } => *amount == 0.0,
                E::BrightnessContrast { brightness, contrast } => [*brightness, *contrast] == [0.0; 2],
                E::ChannelMixer { red, green, blue, monochrome } => {
                    monochrome != "on"
                        && [&red[..4], &green[..4], &blue[..4]]
                            == [[100.0, 0.0, 0.0, 0.0], [0.0, 100.0, 0.0, 0.0], [0.0, 0.0, 100.0, 0.0]]
                }
                E::Vibrance { vibrance, saturation } => [*vibrance, *saturation] == [0.0; 2],
                E::Mosaic { size } => *size <= 1.0,
                E::FindEdges { amount, .. } => *amount <= 0.0,
                E::Sharpen { amount, radius } | E::Diffusion { amount, radius, .. } => *amount <= 0.0 || *radius <= 0.0,
                E::WaveWarp { height, .. } | E::Bulge { height, .. } if *height == 0.0 => true,
                E::Bulge { radius, .. } => *radius <= 0.0,
                E::Ripple { amplitude, .. } => *amplitude == 0.0,
                E::Twirl { angle, radius, .. } => *angle == 0.0 || *radius <= 0.0,
                E::LinearWipe { completion, .. }
                | E::RadialWipe { completion, .. }
                | E::VenetianBlinds { completion, .. }
                | E::IrisWipe { completion, .. } => *completion == 0.0,
                E::SimpleChoker { choke } => *choke == 0.0,
                E::CrossGlare { length, intensity, .. } => length.floor() == 0.0 || *intensity == 0.0,
                E::CameraShake { amount, rotation, .. } => [*amount, *rotation] == [0.0; 2],
                E::Rain { density, opacity, .. } => *density == 0.0 || *opacity == 0.0,
                // B-115: a tile that grows nothing, as `layer_fx::motion_tile` returns at once.
                E::MotionTile { output_width, output_height, .. } => {
                    crate::layer_fx::tile_growth((*output_width, *output_height), size) == (0, 0)
                }
                // B-123: the batch's five new ones, each as its own function returns at once.
                E::ColorLookup { table, .. } => table.is_none(),
                E::LineBlur { length, .. } => *length == 0.0,
                E::Paraffin { spread, opacity, .. } => *spread == 0.0 || *opacity == 0.0,
                E::KiraKira { size, density, opacity, .. } => [*size, *density, *opacity].contains(&0.0),
                // B-151: ten of the fourth batch, each as its own function returns at once.
                E::Median { radius, .. } | E::SmartBlur { radius, .. } => *radius < 1.0,
                E::RoughenEdges { border, .. } => *border == 0.0,
                E::BevelAlpha { edge_thickness, light_intensity, .. } => *edge_thickness <= 0.0 || *light_intensity <= 0.0,
                E::Snowfall { density, size, opacity, .. } => [*density, *size, *opacity].contains(&0.0),
                E::CellPattern { opacity, .. } => *opacity == 0.0,
                E::PolarCoordinates { interpolation, .. } => *interpolation == 0.0,
                E::OpticsCompensation { field_of_view, .. } => *field_of_view == 0.0,
                E::CornerPin { upper_left, upper_right, lower_left, lower_right } => {
                    [*upper_left, *upper_right, *lower_left, *lower_right] == [[0.0, 0.0], [100.0, 0.0], [0.0, 100.0], [100.0, 100.0]]
                }
                _ => false,
            };
            // B-107: a shake grows by how far it can carry a corner, which its settings and
            // the drawing's size say, not its settings alone. B-115: so does a Motion Tile,
            // and not the same across as down.
            let grow = match &effect {
                E::CameraShake { amount, rotation, .. } => {
                    let g = crate::layer_fx::shake_reach(*amount, *rotation, size, *offset).1;
                    (g, g)
                }
                E::MotionTile { output_width, output_height, .. } => {
                    crate::layer_fx::tile_growth((*output_width, *output_height), size)
                }
                // B-151: a Radial Shadow grows by its cast and the cast's blur, a Corner Pin
                // by its corners; both as their own functions do.
                E::RadialShadow { light, distance, softness, .. } => {
                    let (_, _, _, gx, gy) = crate::layer_fx::radial_cast(*light, *distance, size, *offset);
                    let r = crate::effects::kernel_radius(softness / 3.0);
                    (gx + r, gy + r)
                }
                E::CornerPin { upper_left, upper_right, lower_left, lower_right } => {
                    let pins = [*upper_left, *upper_right, *lower_left, *lower_right];
                    crate::layer_fx::corner_map(pins, size, *offset).map_or((0, 0), |m| m.2)
                }
                _ => (effect.bounds_expansion(), effect.bounds_expansion()),
            };
            let instance = crate::effects::EffectInstance { effect, ..instance.clone() };
            let fx = render::Fx { instance, origin: *offset, grow };
            (!nothing).then(|| {
                *offset = (offset.0 + grow.0, offset.1 + grow.1);
                render::OnCard::Fx(fx)
            })
        }
    }
}

/// B-156 (D-225): an adjustment layer's stack as the card runs it on the frame beneath, `size`,
/// from its corner; or `None` when the CPU must run it, for an effect the card cannot draw, one
/// that looks at its drawing before the card is asked (Bloom, Glow, Paraffin, Kira-kira), which
/// the frame beneath is not yet, or an HSV Key, which only begins a run on a drawing the CPU made
/// (D-224). `stack` is the plan's, its distances already the draft's (`preview::scale_plan`).
pub fn adjust_run(stack: &[crate::effects::EffectInstance], size: (usize, usize)) -> Option<Vec<render::OnCard>> {
    use crate::effects::Effect as E;
    let mut offset = (0, 0);
    let mut run = Vec::new();
    // A Light Wrap does nothing in a stack (D-132), as `apply_stack` has it.
    for instance in stack.iter().filter(|i| i.enabled && !matches!(i.effect, E::LightWrap { .. })) {
        let first = matches!(instance.effect, E::Bloom { .. } | E::Glow { .. } | E::Paraffin { .. } | E::KiraKira { .. } | E::HsvKey { .. });
        if first || !card_can(instance, 1.0) {
            return None;
        }
        let grown = (size.0 + 2 * offset.0, size.1 + 2 * offset.1);
        run.extend(card_effect(instance.effect.clone(), instance, grown, &mut offset));
    }
    Some(run)
}

/// `map` (D-189): stop after step 3, the picture cut back to its step-1 rectangle, for an
/// effect's layer setting; at a draft divisor any picture is then taken down, effects or not.
#[allow(clippy::too_many_arguments)]
fn resolve_layer(
    project: &Project,
    comp: &crate::model::Composition,
    layer: &crate::model::Layer,
    frame: i32,
    root: &Path,
    quality: PreviewQuality,
    cache: &mut CelCache,
    log: &mut FrameLog,
    above: &mut Vec<Id>,
    card: bool,
    map: bool,
) -> Option<ResolvedLayer> {
    // D-196: a Posterize Time takes the layer's content from an earlier frame. Whether the layer
    // is shown is still this frame's, and what resolving the held frame says belongs to this one,
    // because an export decides what to block by the frame it is writing.
    let at = posterized(comp, layer, frame);
    if at == frame {
        return resolve_held(project, comp, layer, frame, at, root, quality, cache, log, above, card, map);
    }
    layer.timing().local_frame(frame)?;
    let mut inside = FrameLog::new(usize::MAX);
    let resolved =
        resolve_held(project, comp, layer, frame, at, root, quality, cache, &mut inside, above, card, map);
    log.retime(inside, frame);
    resolved
}

/// D-196: the frame whose content `layer` shows at composition frame `frame`: each switched-on,
/// valid Posterize Time of its stack in turn steps it back to its rate, counted from the
/// composition's first frame, and never before the layer's in point. An adjustment layer holds
/// nothing.
pub(crate) fn posterized(comp: &crate::model::Composition, layer: &crate::model::Layer, frame: i32) -> i32 {
    if layer.is_adjustment() {
        return frame;
    }
    let fps = comp.frame_rate.numerator() as f64 / comp.frame_rate.denominator() as f64;
    let start = comp.start_frame as f64;
    let mut h = frame;
    for i in layer.effects.iter().filter(|i| i.enabled && matches!(i.effect, crate::effects::Effect::PosterizeTime { .. })) {
        let now = crate::expr::effect_at(comp, &layer.id, i, h, layer.key_time(h as f64)).0;
        if !now.is_valid() {
            continue;
        }
        let crate::effects::Effect::PosterizeTime { frame_rate: r } = now.effect else {
            continue;
        };
        if r < fps {
            let s = ((h as f64 - start) * r / fps + 1e-9).floor();
            h = (start + (s * fps / r - 1e-9).ceil()) as i32;
        }
        h = h.max(layer.in_frame);
    }
    h
}

/// [`resolve_layer`] with the layer's content, steps 1 to 3, taken at frame `at` and the rest at
/// `frame`.
#[allow(clippy::too_many_arguments)]
fn resolve_held(
    project: &Project,
    comp: &crate::model::Composition,
    layer: &crate::model::Layer,
    frame: i32,
    at: i32,
    root: &Path,
    quality: PreviewQuality,
    cache: &mut CelCache,
    log: &mut FrameLog,
    above: &mut Vec<Id>,
    card: bool,
    map: bool,
) -> Option<ResolvedLayer> {
    // D-71: an audio layer draws nothing, so no frame is any different for it (FX-AUD-020).
    // D-82: nor does a null, whatever its switch, opacity or timing say (FX-NULL-001, 002).
    if matches!(
        layer.kind,
        crate::model::LayerKind::Audio | crate::model::LayerKind::Null
    ) {
        return None;
    }
    // D-67: a composition layer's drawing is the inner composition, rendered at the layer's
    // local frame, at its own size, through its own camera. From the mask on it is a drawing.
    if let Some(inner_id) = &layer.composition_id {
        layer.timing().local_frame(at)?;
        let Some(inner) = project.composition(inner_id) else {
            log.record(
                frame,
                layer.name.clone(),
                Diagnostic::new(
                    DiagnosticId::CompositionReferenceMissing,
                    Severity::Warning,
                    format!(
                        "Layer {} shows composition {}, which is not in this project.",
                        layer.name,
                        inner_id.as_str()
                    ),
                    "The reference is preserved in the project and the layer draws nothing."
                        .to_string(),
                )
                .with_remediation(
                    "Delete the layer, or put the composition it shows back in the project.",
                ),
            );
            return None;
        };
        // Outside the inner composition's own frames there is nothing to show, which is not a
        // fault: D-67 has the layer transparent there, as a drawn layer is where it exposes
        // nothing. D-216: the inner frame is `f` at the layer's source time, and `f + 1` too when
        // it is mixed in; the inner composition's end is the layer's source's.
        let end = inner.start_frame + inner.duration_frames as i32;
        let (local, w) = source_frames(comp, layer, at, Some(end))?;
        let inside = |f: i32| (inner.start_frame..end).contains(&f);
        if !inside(local) && !(w > 0.0 && inside(local + 1)) {
            return None;
        }
        if above.contains(inner_id) || &comp.id == inner_id {
            log.record(
                frame,
                layer.name.clone(),
                Diagnostic::new(
                    DiagnosticId::CompositionCycle,
                    Severity::Error,
                    format!("Layer {} shows a composition it is inside of.", layer.name),
                    format!(
                        "Composition {} leads back to itself. The layer draws nothing.",
                        inner_id.as_str()
                    ),
                ),
            );
            return None;
        }
        let mut draw = |local: i32| {
            if !inside(local) {
                return None;
            }
            // B-171 (G12, D-243): the viewer keeps each inner frame it draws, in memory and on
            // disk, and draws it again only when something it was drawn from has changed.
            let key = (cache.effect_budget() > 0)
                .then(|| inner_key(project, inner_id, root, quality, local, above, &comp.id))
                .flatten();
            if let Some(picture) = key.as_deref().and_then(|key| cache.inner_frame(key)) {
                return Some(picture);
            }
            let started = std::time::Instant::now();
            let (drawn, files) = crate::cache::files_read(|| {
                above.push(comp.id.clone());
                let mut within = FrameLog::new(usize::MAX);
                let plan = plan_inside(
                    project,
                    inner_id,
                    local,
                    root,
                    quality,
                    &mut within,
                    cache,
                    above,
                    false,
                );
                above.pop();
                // A frame that said something is not kept: a kept frame says nothing when used.
                let quiet = within.is_empty();
                // What went wrong inside belongs to the frame that was asked for, not to the inner
                // frame's number: an export decides what to block by the frame it is writing.
                log.absorb(within, frame, &layer.name);
                let plan = match plan {
                    Ok(plan) => crate::preview::scale_plan(plan, quality),
                    Err(d) => {
                        log.record(frame, layer.name.clone(), d);
                        return None;
                    }
                };
                let tile = match quality {
                    PreviewQuality::Full | PreviewQuality::Half => DEFAULT_TILE_SIZE,
                    PreviewQuality::Draft => DRAFT_TILE_SIZE,
                };
                Some((std::sync::Arc::new(render::render(&plan, tile)), quiet))
            });
            let (picture, quiet) = drawn?;
            if let (Some(key), true) = (key, quiet) {
                cache.store_inner(key, files, std::sync::Arc::clone(&picture), started.elapsed());
            }
            Some(picture)
        };
        let picture = if w > 0.0 {
            let a = draw(local);
            std::sync::Arc::new(mix(a.as_deref(), draw(local + 1).as_deref(), w)?)
        } else {
            draw(local)?
        };
        let mut resolved = resolve_rest(
            project,
            root,
            comp,
            layer,
            frame,
            at,
            cache,
            log,
            picture,
            None,
            quality.divisor() as f64,
            false,
            map,
        )?;
        resolved.nested = Some((inner_id.clone(), local));
        resolved.mixed = w > 0.0;
        return Some(resolved);
    }
    // D-66: an adjustment layer has no drawing. Its shape is an opaque rectangle the size of
    // the composition in its own layer space, and from the mask on it goes the way a drawn layer
    // goes.
    let mut mixed = false;
    let (source, cel) = if layer.is_adjustment() {
        layer.timing().local_frame(frame)?;
        let shape = WorkingBuffer::opaque(comp.width as usize, comp.height as usize);
        (std::sync::Arc::new(shape), None)
    } else if let Some(solid) = &layer.solid {
        // D-74: a solid's step 1 is its record, `width` by `height` of its colour, opaque.
        layer.timing().local_frame(frame)?;
        let mut shape = WorkingBuffer::opaque(solid.width as usize, solid.height as usize);
        for px in shape.data_mut().chunks_exact_mut(4) {
            px[..3].copy_from_slice(&solid.color.map(|c| c as f32));
        }
        (std::sync::Arc::new(shape), None)
    } else if let Some(words) = &layer.text {
        // D-263: a text layer's step 1 is the composition's size in transparent black with its
        // words drawn into it, as a shape layer's is. A font this machine does not have is said
        // per frame, for the reason an undrawable shape is, and nothing is drawn in its place.
        layer.timing().local_frame(frame)?;
        let (w, h) = (comp.width as usize, comp.height as usize);
        let picture = crate::text::drawn(words, w, h).unwrap_or_else(|| {
            log.record(
                frame,
                layer.name.clone(),
                Diagnostic::new(
                    DiagnosticId::TextFontMissing,
                    Severity::Warning,
                    format!(
                        "Layer {}'s font \"{}\" is not on this computer, so its words are not drawn.",
                        layer.name, words.font
                    ),
                    format!(
                        "D-263: no other font is put in its place. Looked for it among the fonts \
                         that come with the program and in {:?}. The words are kept as they are.",
                        crate::text::installed_fonts()
                    ),
                )
                .with_remediation("Install the font, or choose another in Effect controls."),
            );
            std::sync::Arc::new(WorkingBuffer::transparent(w, h))
        });
        (picture, None)
    } else if layer.kind == crate::model::LayerKind::Shape {
        // D-78: a shape layer's step 1 is the composition's size in transparent black with its
        // shapes drawn into it. It has no size of its own, which is why `comp` is asked and not
        // the layer. The paths are resolved to this frame first, exactly as a mask's are and for
        // the same reason: what the rasterizer sees holds plain numbers.
        layer.timing().local_frame(frame)?;
        let now: Vec<crate::shape::Shape> = layer.shapes.iter().map(|s| s.at(at)).collect();
        for s in &now {
            // Said per frame, as an undrawable mask is, because that is what marks an export's
            // fidelity incomplete. The warning raised when the file opened is not enough: a
            // picture would otherwise go out with a shape silently missing from it.
            if s.enabled && !s.has_enough_points() {
                log.record(
                    frame,
                    layer.name.clone(),
                    Diagnostic::new(
                        DiagnosticId::ShapeInvalidOutline,
                        Severity::Warning,
                        format!(
                            "Layer {}'s shape \"{}\" cannot be drawn, so it is not.",
                            layer.name, s.name
                        ),
                        format!(
                            "D-78: a shape is filled inside its path and stroked along it, and \
                             neither means anything with fewer than two points. The shape has {} \
                             and is left out of frame {frame}. Its record is untouched.",
                            s.points.len()
                        ),
                    )
                    .with_remediation(
                        "Add points to the shape, or switch it off if it is not wanted.",
                    ),
                );
            }
        }
        let shape = crate::shape::draw(&now, comp.width as usize, comp.height as usize);
        (std::sync::Arc::new(shape), None)
    } else {
        // D-216: the source frame, then its dissolve toward the next drawing, then its mix
        // toward the next frame. A picture made that way is no file, so it skips the cel cache.
        let (f, w) = source_frames(comp, layer, at, raster_end(layer))?;
        if w == 0.0 && dissolving(layer, f).is_none() {
            let (source, cel) = decode_cel(project, layer, f, at, root, cache, log)?;
            (source, Some(cel))
        } else {
            // B-156c: a drawing that is only read is not copied.
            let mut cel_at = |f: i32| {
                let a = decode_cel(project, layer, f, at, root, cache, log).map(|(p, _)| p);
                match dissolving(layer, f) {
                    Some((e, k)) => {
                        let b = decode_cel(project, layer, e, at, root, cache, log).map(|(p, _)| p);
                        mix(a.as_deref(), b.as_deref(), k).map(std::sync::Arc::new)
                    }
                    None => a,
                }
            };
            let a = cel_at(f);
            let picture = if w > 0.0 { mix(a.as_deref(), cel_at(f + 1).as_deref(), w).map(std::sync::Arc::new) } else { a }?;
            mixed = true;
            (picture, None)
        }
    };
    // D-99: in a draft preview a drawing with effects is taken down to the draft size first,
    // sampled exactly as the draft frame samples it, and its stack runs on that - a sixteenth of
    // the pixels - as D-67 already does for a composition layer's picture. D-189: a map at a
    // draft divisor is taken down so, whatever its layer. `Full` never enters here, so a full
    // preview and an export are what they were.
    if quality != PreviewQuality::Full
        && (map || ((cel.is_some() || mixed) && layer.effects.iter().any(|i| i.enabled)))
    {
        let d = quality.divisor();
        let small = crate::preview::scale_plan(
            FramePlan {
                width: source.width(),
                height: source.height(),
                layers: vec![LayerDraw {
                    id: layer.id.clone(),
                    source,
                    transform: Affine::IDENTITY,
                    opacity: 1.0,
                    matte: None,
                    blend: crate::model::BlendMode::Normal,
                    adjust: None,
                    nested: None,
                    on_card: Vec::new(),
                    wrap: Vec::new(),
                    motion_blur: false,
                    mixed: false,
                    moments: Vec::new(),
                }],
            },
            quality,
        );
        let small = std::sync::Arc::new(render::render(&small, DRAFT_TILE_SIZE));
        return resolve_rest(
            project, root, comp, layer, frame, at, cache, log, small, cel, d as f64, card, map,
        )
        .map(|r| ResolvedLayer { mixed, ..r });
    }
    resolve_rest(project, root, comp, layer, frame, at, cache, log, source, cel, 1.0, card, map)
        .map(|r| ResolvedLayer { mixed, ..r })
}

/// Document 21 step 1 for a drawn layer: which file it shows at layer-local frame `local`,
/// decoded, or `None` with the reason logged against composition frame `frame`. The path and
/// interpretation come back with the pixels because the effect cache is keyed by them.
#[allow(clippy::too_many_arguments)]
fn decode_cel(
    project: &Project,
    layer: &crate::model::Layer,
    local: i32,
    frame: i32,
    root: &Path,
    cache: &mut CelCache,
    log: &mut FrameLog,
) -> Option<(
    std::sync::Arc<WorkingBuffer>,
    (PathBuf, crate::model::Interpretation),
)> {
    let Some(asset) = project.assets.iter().find(|a| a.id == layer.asset_id) else {
        log.record(
            frame,
            layer.name.clone(),
            schema_invalid(format!(
                "Layer {} names asset {}, which is not in the project.",
                layer.name,
                layer.asset_id.as_str()
            )),
        );
        return None;
    };

    // Step 5: the drawing exposed at the layer-local frame.
    let relative = match source_at(layer.exposure_spans.clone(), asset, local, frame) {
        Ok(Some(path)) => path,
        Ok(None) => return None,
        Err(d) => {
            log.record(frame, layer.name.clone(), d);
            return None;
        }
    };
    let path = root.join(&relative);
    if crate::cache::looked_at(&path).is_err() {
        log.record(
            frame,
            layer.name.clone(),
            Diagnostic::new(
                DiagnosticId::MediaMissing,
                Severity::Warning,
                format!(
                    "{} is not where the project says it is.",
                    relative.display()
                ),
                format!(
                    "Layer {} looked for it at {} for frame {frame}. The reference is kept and \
                     the layer is left out of this frame; no neighbouring drawing is substituted \
                     for it.",
                    layer.name,
                    path.display()
                ),
            )
            .with_remediation("Relink the sequence, or put the file back where it was."),
        );
        return None;
    }
    // Document 21 step 1: decode, then interpret. `decode_png` tags what PNG guarantees —
    // sRGB, straight — and the asset record is what overrides it, so a project that says a
    // sequence was rendered premultiplied is believed here and nowhere else. All three of
    // those steps happen inside the cache, because all three are what a hit skips.
    let source = match cache.decoded(&path, asset.interpretation) {
        Ok(buffer) => buffer,
        Err(d) => {
            log.record(frame, layer.name.clone(), d);
            return None;
        }
    };
    Some((source, (path, asset.interpretation)))
}

/// Document 21 from step 2 for one layer, given its decoded cel or, with `cel` `None`, an
/// adjustment layer's shape or a composition layer's picture.
///
/// `pre` is how many layer-space pixels one pixel of `source` covers. It is 1 except for a
/// composition layer in a draft preview, whose picture was rendered at the draft divisor
/// (D-67): the mask and a blur's sigma are divided by it and the transform multiplies it back.
/// At 1 none of that is entered, so a full-size frame is computed exactly as it was before.
///
/// `map` (D-189) returns after step 3, the picture cut back to its step-1 rectangle and
/// placed nowhere: what an effect's layer setting reads.
///
/// `at` (D-196) is the frame the masks and effects are taken at, `frame` unless the layer is
/// held by a Posterize Time.
#[allow(clippy::too_many_arguments)]
/// D-291: one effect as it is at the frame `at` (its keys read at `u`), its expressions run, and
/// each that fails reported against the composition frame `frame`, as a layer's property is.
fn effect_now(
    comp: &crate::model::Composition,
    layer: &crate::model::Layer,
    instance: &crate::effects::EffectInstance,
    frame: i32,
    at: i32,
    u: f64,
    log: &mut FrameLog,
) -> crate::effects::EffectInstance {
    let (now, failed) = crate::expr::effect_at(comp, &layer.id, instance, at, u);
    for (name, e) in failed {
        let what = format!("{} {name}", instance.effect.name());
        log.record(frame, format!("{}/{what}", layer.name), e.diagnostic(&layer.name, &what, frame));
    }
    now
}

fn resolve_rest(
    project: &Project,
    root: &Path,
    comp: &crate::model::Composition,
    layer: &crate::model::Layer,
    frame: i32,
    at: i32,
    cache: &mut CelCache,
    log: &mut FrameLog,
    mut source: std::sync::Arc<WorkingBuffer>,
    cel: Option<(PathBuf, crate::model::Interpretation)>,
    pre: f64,
    card: bool,
    map: bool,
) -> Option<ResolvedLayer> {
    let step1 = (source.width(), source.height());
    // D-188: the moments of a motion-blurred layer's shutter. Its effects all run here, once,
    // because the card does not draw motion blur yet.
    let times = if layer.motion_blur && !layer.is_adjustment() {
        comp.motion_blur.times(frame)
    } else {
        Vec::new()
    };
    let card = card && times.len() < 2;
    // D-68: every setting is its value at this composition frame, so the stack below, its
    // bounds and the effect cache's key all hold plain numbers.
    // D-291: and each setting with an expression, what it gives on that frame.
    let mut effects: Vec<crate::effects::EffectInstance> =
        layer.effects.iter().map(|i| effect_now(comp, layer, i, frame, at, layer.key_time(at as f64), log)).collect();
    // D-182: each Color Lookup's file, read, and what kept one from being read said once a frame.
    for d in crate::lut::fill(&mut effects, project, root, &layer.name) {
        log.record(frame, layer.name.clone(), d);
    }
    // D-191: each layer setting's map, made at the size the effects run at. An adjustment
    // layer's are made where its stack runs, on the frame.
    if !layer.is_adjustment() {
        // D-259: `pre` is one over the divisor, so it names the quality it came from.
        let quality = if pre == 1.0 {
            PreviewQuality::Full
        } else if pre == 0.5 {
            PreviewQuality::Half
        } else {
            PreviewQuality::Draft
        };
        fill_maps(&mut effects, project, root, comp, layer, at, quality, step1, cache, log);
        fill_echoes(&mut effects, project, root, comp, layer, at, quality, step1, cache, log);
    }
    // B-24d: a mask whose path has keys is resolved to its shape at this frame here, before the
    // draft divisor, before the rasterizer and before document 27's cache key, exactly as an
    // effect's settings are on the line above. A path that stands still is not copied at all.
    // D-298: nor is a mask whose feather, opacity and expansion have no keys.
    let moving = layer.masks.iter().any(|m| !m.keys.is_empty() || !m.tracks.is_empty());
    let moved: Vec<crate::mask::Mask> = if moving {
        layer.masks.iter().map(|m| m.at_time(layer.key_time(at as f64))).collect()
    } else {
        Vec::new()
    };
    let now: &[crate::mask::Mask] = if moving { &moved } else { &layer.masks };
    // D-77: a mask's feather and expansion are distances in layer-space pixels, so a draft
    // picture divides them by the divisor exactly as it divides the points, or a feather would
    // be four times as wide at quarter size.
    let draft_masks: Vec<crate::mask::Mask> = if pre == 1.0 {
        Vec::new()
    } else {
        now
            .iter()
            .map(|m| {
                let mut m = m.clone();
                for p in &mut m.points {
                    p.point = (p.point.0 / pre, p.point.1 / pre);
                    p.in_handle = (p.in_handle.0 / pre, p.in_handle.1 / pre);
                    p.out_handle = (p.out_handle.0 / pre, p.out_handle.1 / pre);
                }
                m.feather_px /= pre;
                m.expansion_px /= pre;
                m
            })
            .collect()
    };
    // Document 21 step 2: the polygon mask, in layer/source space, before the transform.
    //
    // `CelCache::decoded` hands back a *shared* buffer since P-03(a), so writing on it directly
    // would cut every other layer using the same drawing, and only on a cache hit. `make_mut` is
    // what prevents that: it copies when anyone else holds the buffer — which the cache always
    // does when the cel came from a hit or was admitted on a miss — and writes in place when
    // nobody does. That is the single most damaging thing these lines could get wrong, so
    // `tests/b06_mask.rs` asserts it against the cache rather than trusting this comment.
    //
    // The two `make_mut` calls below are the only writes to a cel in the whole render, which is
    // why the copy P-01 measured at up to 55.6% of a warm frame could be removed at all: a layer
    // with neither a mask nor an effect never writes, and now never copies.
    let masks: &[crate::mask::Mask] = if pre == 1.0 { now } else { &draft_masks };
    for mask in masks {
        // A mask that is switched on but cannot be drawn -- fewer than three corners, or an
        // outline that crosses itself -- is a feature bypassed, not a shape to guess at.
        // Saying so per frame is what puts the incomplete-fidelity mark on an export; leaving
        // it to the warning raised when the file was opened would let a picture go out with a
        // mask silently missing from it.
        if mask.enabled && !mask.is_renderable() {
            log.record(
                frame,
                layer.name.clone(),
                Diagnostic::new(
                    DiagnosticId::MaskInvalidOutline,
                    Severity::Warning,
                    format!(
                        "Layer {}'s mask \"{}\" cannot be drawn, so it is not.",
                        layer.name, mask.name
                    ),
                    format!(
                        "The mask has {} corners and its outline {} itself. Document 19 requires \
                         at least three corners and an outline that does not cross. The layer is \
                          drawn without that mask for frame {frame} and the mask is kept in the \
                          project.",
                        mask.points.len(),
                        if crate::mask::is_simple(&mask.vertices()) {
                            "does not cross"
                        } else {
                            "crosses"
                        }
                    ),
                )
                .with_remediation("Redraw the mask so its outline does not cross itself."),
            );
        }
    }
    // The mask itself is drawn below, once it is known whether the effect cache already holds
    // the drawing it makes (B-170).
    let masked = masks.iter().any(|m| m.is_renderable() && m.mode != crate::mask::MaskMode::None);

    // Document 21 step 3: the ordered effect stack, in layer space, after the mask and before
    // the transform.
    //
    // A blur grows the buffer by its kernel radius, so `offset` is where the layer's old origin
    // ended up inside the new one. It is applied to the transform below rather than by cropping
    // back: cropping is exactly the fault document 21's "bounds expand by the kernel radius"
    // exists to prevent, and it would cut a straight edge through the glow of anything blurred
    // near the edge of its cel.
    //
    // An empty stack is skipped rather than called with nothing in it, because reaching
    // `apply_stack` at all means taking the copy `Arc::make_mut` exists to avoid - and the first
    // measurement of P-03(a) showed exactly that: the reference shot, which has no effects on any
    // layer, spent 15.541 ms a warm frame copying cels for a loop with no iterations. An empty
    // slice also reports nothing, so nothing is lost by not entering it.
    //
    // What raising a bypassed effect looks like, hoisted out of the call below because a frame
    // served from the effect cache has to raise exactly the same warnings as the frame that
    // filled it (P-11). Document 27: a cache "must never define correctness", and a diagnostic
    // that appeared only on a cache miss would be a cache defining one.
    let mut report = |instance: &crate::effects::EffectInstance, why: crate::effects::Bypassed| {
        let (id, what, detail) = match why {
            crate::effects::Bypassed::NotImplemented => (
                DiagnosticId::EffectUnsupported,
                format!(
                    "Layer {} uses the effect \"{}\", which this build does not have.",
                    layer.name,
                    instance.type_id()
                ),
                format!(
                    "Frame {frame} is drawn without it. The effect is kept in the project \
                     exactly as it was."
                ),
            ),
            crate::effects::Bypassed::InvalidParameter => (
                DiagnosticId::EffectParameterInvalid,
                format!(
                    "Layer {}'s {} has a setting this build cannot use, so it is not drawn.",
                    layer.name,
                    instance.type_id()
                ),
                format!(
                    "{} Frame {frame} is drawn without the effect, which is kept as it was.",
                    instance.fault().unwrap_or_default()
                ),
            ),
        };
        log.record(
            frame,
            layer.name.clone(),
            Diagnostic::new(id, Severity::Warning, what, detail).with_remediation(
                "The frame is missing what the effect would have done. Remove the effect, or \
                 correct it, to have the picture match the project.",
            ),
        );
    };

    // The masks that were drawn into the cel above, and therefore part of what the stack ran on.
    // The ones that could not be drawn are left out, because they wrote nothing: two projects
    // that differ only in a mask neither of them can draw share a cel and may share a result.
    let drawn_masks: Vec<crate::mask::Mask> = masks
        .iter()
        .filter(|m| m.is_renderable())
        .cloned()
        .collect();

    // B-46: a drawing whose last effect switched on is a Radial Blur, (B-47) a Bloom, (B-49) a
    // Directional Blur, (B-50) a Gaussian Blur, (B-51) a Glow, (B-65, B-76) one of the two
    // batches of ten, (B-107, B-115) one of the third batch's thirty this build can draw or
    // (B-123) one of the fourth batch's five has only the effects before it run here, when the
    // plan is for the card.
    // Those are what the effect cache is asked for, a stack of their own, so it never hands one
    // path's result to the other.
    // B-76: a Light Wrap is not in the layer's own stack (D-132; it runs as the layer is laid), so
    // the effect before it can be the last.
    // B-155 (D-224): not only the last. Every effect from the last one the card cannot draw to the
    // end of the stack is left, and the card runs them one after another. Bloom, Glow, Paraffin
    // and Kira-kira look at the drawing they are given here, before the card is asked, so one of
    // them can only begin that run. So can an HSV Key: the hue of a nearly grey pixel swings
    // with the smallest change, and given the card's picture rather than the CPU's it keyed
    // pixels the CPU did not (B-155's table, 255 levels).
    let can = |i: usize| card && cel.is_some() && card_can(&effects[i], pre);
    let mut chain = Vec::new();
    for i in (0..effects.len()).rev() {
        use crate::effects::Effect as E;
        if !effects[i].enabled || matches!(effects[i].effect, E::LightWrap { .. }) {
            continue;
        }
        if !can(i) {
            break;
        }
        chain.push(i);
        if matches!(effects[i].effect, E::Bloom { .. } | E::Glow { .. } | E::Paraffin { .. } | E::KiraKira { .. } | E::HsvKey { .. }) {
            break;
        }
    }
    chain.reverse();
    let before = chain.first().copied().unwrap_or(effects.len());
    // B-46: a draft cel whose only effect is left to the card still goes to the effect cache,
    // under an empty stack, so the card is handed the same small drawing every frame and sends it
    // once. B-170 (D-242): so does a masked cel, at either quality: a held drawing is then neither
    // masked again nor sent to the card again, where before each frame masked a new copy.
    let skip = !masked && effects[..before].is_empty() && (chain.is_empty() || pre == 1.0);
    let divisor = pre as usize;
    let hit = match &cel {
        Some((path, interpretation)) if !skip => {
            cache.effect_result(path, *interpretation, &drawn_masks, &effects[..before], divisor)
        }
        _ => None,
    };
    // The guard is `mask::apply`'s own first line, called here rather than restated: a mask
    // that cannot be drawn writes nothing, and a copy taken to write nothing is the whole
    // cost P-03(a) removed. D-77 keeps that: with no mask that takes part, `coverage` is
    // `None` and `apply` returns before the copy. A cache hit already holds the mask's work.
    if masked && hit.is_none() {
        crate::perf::time(crate::perf::Stage::Mask, || {
            crate::mask::apply(std::sync::Arc::make_mut(&mut source), masks)
        });
    }
    let offset = match &cel {
        // D-66: an adjustment layer's stack runs on the frame beneath it, in the renderer. What
        // that run would have reported is reported here instead, so a bypassed effect reaches
        // the log from the plan, where every other diagnostic of a frame comes from.
        None if layer.is_adjustment() => {
            for instance in effects.iter().filter(|i| i.enabled) {
                match &instance.effect {
                    crate::effects::Effect::Unsupported { .. } => {
                        report(instance, crate::effects::Bypassed::NotImplemented)
                    }
                    _ if !instance.is_valid() => {
                        report(instance, crate::effects::Bypassed::InvalidParameter)
                    }
                    _ => {}
                }
            }
            (0, 0)
        }
        _ if skip => (0, 0),
        // D-67: a composition layer's stack runs on the inner picture as one. The effect cache
        // is keyed by a cel's file, and this picture has none, so it is not asked.
        None => {
            let mut stack = effects.clone();
            if pre != 1.0 {
                for instance in &mut stack {
                    instance.effect.scale_distances(|d| d / pre);
                }
            }
            crate::effects::apply_stack(
                std::sync::Arc::make_mut(&mut source),
                &stack,
                |_, instance, why| report(instance, why),
            )
        }
        Some((path, interpretation)) => {
            // D-99: a cel taken down to draft size runs its stack with every distance divided
            // by the divisor, as the composition layer's picture above does. The key holds the
            // settings as the project states them, and the divisor beside them.
            let mut stack = effects[..before].to_vec();
            if pre != 1.0 {
                for instance in &mut stack {
                    instance.effect.scale_distances(|d| d / pre);
                }
            }
            if let Some(hit) = hit {
                // P-11. ADR-017 fixes an evaluation's whole input to the cel, the mask and the stack, all
                // three of which are in the key, so this buffer is the one `apply_stack` would have
                // produced. It is handed back shared: the cache holds it too, so the transform below,
                // which only reads, never copies it, and anything that did write would copy through
                // `Arc::make_mut` exactly as it does for a cel.
                for (index, why) in &hit.bypassed {
                    report(&effects[*index], *why);
                }
                source = hit.buffer;
                hit.offset
            } else {
                // The copy is timed on its own and the effects are timed one kind at a time inside
                // `apply_stack` (P-11), so nothing here wraps anything that is timed below it.
                let pixels = crate::perf::time(crate::perf::Stage::EffectCopy, || {
                    std::sync::Arc::make_mut(&mut source)
                });
                let mut bypassed: Vec<(usize, crate::effects::Bypassed)> = Vec::new();
                let offset = crate::effects::apply_stack(pixels, &stack, |at, instance, why| {
                    bypassed.push((at, why));
                    report(instance, why);
                });
                cache.store_effect(
                    path,
                    *interpretation,
                    &drawn_masks,
                    &effects[..before],
                    divisor,
                    crate::cache::EffectResult {
                        buffer: std::sync::Arc::clone(&source),
                        offset,
                        bypassed,
                    },
                );
                offset
            }
        }
    };
    if map {
        let source = if offset == (0, 0) && (source.width(), source.height()) == step1 {
            source
        } else {
            std::sync::Arc::new(crate::layer_map::cut(&source, offset, step1))
        };
        return Some(ResolvedLayer {
            source,
            transform: Affine::IDENTITY,
            opacity: 1.0,
            nested: None,
            on_card: Vec::new(),
            moments: Vec::new(),
            mixed: false,
        });
    }
    let mut offset = offset;
    let start = offset;
    let mut on_card = Vec::new();
    for &i in &chain {
        // B-155: the buffer each is given, grown by the ones before it in the run.
        let size = (source.width() + 2 * (offset.0 - start.0), source.height() + 2 * (offset.1 - start.1));
        let mut effect = effects[i].effect.clone();
        effect.scale_distances(|d| d / pre);
        on_card.extend(match effect {
            // B-47: a Bloom that lights nothing changes nothing and grows nothing, on the CPU too,
            // so it is not left at all. One that does grows the drawing by its reach.
            crate::effects::Effect::Bloom { threshold, radius, intensity, streaks, length, angle } => {
                use rayon::prelude::*;
                let lit = intensity != 0.0
                    && source.data().par_chunks_exact(4).any(|px| crate::bloom::bright(px, threshold));
                let lines = crate::bloom::lines(&streaks);
                let grow = crate::bloom::reach(radius, lines, length);
                lit.then(|| {
                    offset = (offset.0 + grow, offset.1 + grow);
                    render::OnCard::Bloom(render::Bloom { threshold, radius, intensity, lines, length, angle })
                })
            }
            // B-51: as a Bloom, a Glow with nothing that glows changes nothing and is not left.
            crate::effects::Effect::Glow { based_on, threshold, colors, tolerance, radius, intensity, operation, tint } => {
                use rayon::prelude::*;
                let g = crate::glow::settings(&based_on, threshold, &colors, tolerance, radius, intensity, &operation, &tint);
                let lit = intensity != 0.0 && source.data().par_chunks_exact(4).any(|px| crate::glow::glows(px, &g));
                lit.then(|| {
                    let grow = crate::effects::kernel_radius(radius / 3.0);
                    offset = (offset.0 + grow, offset.1 + grow);
                    render::OnCard::Glow(g)
                })
            }
            effect => card_effect(effect, &effects[i], size, &mut offset),
        });
    }

    // Step 6: the animated properties at this frame. A property holding the wrong kind of
    // value cannot come from a loaded project — persistence refuses it — so this reports
    // rather than guesses a default, which would put a layer somewhere nobody asked for.
    // D-59: each after its expression; one that fails is drawn at its keys and said so.
    let mut at = |prop: Prop| {
        let property = match prop {
            Prop::Depth => layer.depth.as_ref()?,
            _ => layer
                .transform
                .get(prop)
                .expect("the five are transform properties"),
        };
        let owner = || crate::expr::Target::Layer(layer.id.clone());
        // D-216: keys at the layer's key time, so they stretch with it.
        let (value, failed) =
            crate::expr::resolve_at(comp, property, owner, prop, frame, layer.key_time(frame as f64));
        if let Some(e) = failed {
            log.record(
                frame,
                format!("{}/{prop}", layer.name),
                e.diagnostic(&layer.name, prop, frame),
            );
        }
        Some(value)
    };
    // Depth is read again by `world_depth` below; this reads it only to report it.
    at(Prop::Depth);
    let (Some(anchor), Some(position), Some(scale), Some(rotation), Some(opacity)) = (
        at(Prop::Anchor).and_then(|v| v.as_vec2()),
        at(Prop::Position).and_then(|v| v.as_vec2()),
        at(Prop::Scale).and_then(|v| v.as_vec2()),
        at(Prop::Rotation).and_then(|v| v.as_scalar()),
        at(Prop::Opacity).and_then(|v| v.as_scalar()),
    ) else {
        log.record(
            frame,
            layer.name.clone(),
            schema_invalid(format!(
                "Layer {}'s transform holds a value of the wrong kind at frame {frame}: {}.",
                layer.name,
                wrong_kinds(layer, frame)
            )),
        );
        return None;
    };

    // D-58. Resolved here rather than once per frame because a matte is projected at its own
    // depth too, and a matte reaches this function by its own call.
    let Some(cam) = camera_at(comp, frame) else {
        log.record(
            frame,
            layer.name.clone(),
            schema_invalid(format!(
                "The camera of {} holds a value of the wrong kind at frame {frame}.",
                comp.name
            )),
        );
        return None;
    };
    let centre = (comp.width as f64 / 2.0, comp.height as f64 / 2.0);
    let camera = projection(cam, centre, world_depth(comp, &layer.id, frame));
    let before = Affine::translation(-(offset.0 as f64), -(offset.1 as f64)).then(if pre == 1.0 {
        Affine::IDENTITY
    } else {
        Affine::scaling(pre, pre)
    });
    // D-188: step 4 at each moment. When every moment is the same matrix it is taken once,
    // through that matrix (FX-MB-010).
    let moments: Vec<Option<Affine>> = if times.len() > 1 {
        times.iter().map(|&t| moment(comp, layer, frame, t, before)).collect()
    } else {
        Vec::new()
    };
    let bits = |m: &Option<Affine>| m.map(|m| [m.a, m.b, m.c, m.d, m.tx, m.ty].map(f64::to_bits));
    let once = moments
        .first()
        .filter(|first| moments.iter().all(|m| bits(m) == bits(first)))
        .copied();
    if moments.iter().any(Option::is_none) {
        let behind = moments.iter().filter(|m| m.is_none()).count();
        log.record(
            frame,
            layer.name.clone(),
            Diagnostic::new(
                DiagnosticId::CameraPlaneBehind,
                Severity::Warning,
                format!(
                    "Layer {} is level with the camera or behind it at {behind} of its {} \
                     motion-blur moments.",
                    layer.name,
                    moments.len()
                ),
                format!(
                    "Those moments add nothing to frame {frame}'s average, and the layer is left \
                     out only if every moment is. The project is unchanged."
                ),
            )
            .with_remediation(
                "Move the layer in front of the camera, or move the camera back, to see it \
                 again.",
            ),
        );
        if once.is_some() {
            return None;
        }
    }
    if let Some(Some(still)) = once {
        return Some(ResolvedLayer {
            source,
            on_card,
            transform: still,
            opacity: opacity as f32,
            nested: None,
            moments: Vec::new(),
            mixed: false,
        });
    }
    if let (true, Projection::Behind) = (moments.is_empty(), &camera) {
        // Document 28: the record is untouched and nothing is clamped into a working value. A
        // plane one pixel in front of the camera is not this case and is drawn enormous.
        log.record(
            frame,
            layer.name.clone(),
            Diagnostic::new(
                DiagnosticId::CameraPlaneBehind,
                Severity::Warning,
                format!(
                    "Layer {} is level with the camera or behind it, so it has no size.",
                    layer.name
                ),
                format!(
                    "Its plane is at depth {} and the camera is at {} at frame {frame}. The \
                     layer is left out of this frame and the project is unchanged.",
                    world_depth(comp, &layer.id, frame),
                    cam.depth
                ),
            )
            .with_remediation(
                "Move the layer in front of the camera, or move the camera back, to see it \
                 again.",
            ),
        );
        return None;
    }

    Some(ResolvedLayer {
        source,
        on_card,
        // Document 21 step 4. Scale is a unit factor in the model (D-22); the divide by 100
        // lives at the file and UI boundaries, not here.
        //
        // The leading translation undoes the bounds expansion an effect asked for. Pixel `p` of
        // the grown buffer held what pixel `p - offset` held before, so shifting by `-offset`
        // first puts every pixel back exactly where it was and leaves only the new margin, which
        // holds what the blur pushed outside the old extent. Zero offset makes it the identity.
        //
        // D-57 closes step 4 with the parent chain: `M_world(L) = M_world(parent(L)) * M(L)`.
        // A parent that is not in the composition is reported when the project is opened and
        // leaves an identity here, so the layer draws where it would with no parent rather than
        // vanishing. There is no second report per frame: nothing can lose a parent while the
        // program runs, because deleting one lets its children go in place.
        //
        // D-58 closes it with the camera. The projection is the last thing multiplied in, so a
        // far plane is minified by the same single resampling as everything else. An identity
        // projection is left out rather than applied, which is what keeps every fixture written
        // before D-58 landing exactly on its number instead of within a tolerance.
        transform: before
            .then(Affine::from_transform(anchor, position, scale, rotation))
            .then(parent_chain_at(comp, &layer.id, frame).matrix)
            .then(match camera {
                Projection::Scaled(p) => p,
                _ => Affine::IDENTITY,
            }),
        // Document 21 step 6. Opacity is normalized 0..1 in the model (document 19).
        opacity: opacity as f32,
        nested: None,
        moments,
        mixed: false,
    })
}

/// D-188: document 21's step 4 at a moment `t` inside frame `frame`'s shutter, multiplied
/// exactly as [`resolve_rest`] multiplies the frame's own, so a layer that holds still gives
/// that matrix bit for bit. `before` is the bounds shift and draft scale. `None` is a moment
/// the layer is level with the camera or behind it.
fn moment(
    comp: &crate::model::Composition,
    layer: &crate::model::Layer,
    frame: i32,
    t: f64,
    before: Affine,
) -> Option<Affine> {
    let at = |prop| {
        let property = layer.transform.get(prop).expect("the four are transform properties");
        let owner = || crate::expr::Target::Layer(layer.id.clone());
        crate::expr::resolve_at(comp, property, owner, prop, frame, layer.key_time(t)).0
    };
    let own = Affine::from_transform(
        at(Prop::Anchor).as_vec2()?,
        at(Prop::Position).as_vec2()?,
        at(Prop::Scale).as_vec2()?,
        at(Prop::Rotation).as_scalar()?,
    );
    let centre = (comp.width as f64 / 2.0, comp.height as f64 / 2.0);
    let camera = projection(
        camera_at_time(comp, frame, t)?,
        centre,
        world_depth_at(comp, &layer.id, frame, t),
    );
    if let Projection::Behind = camera {
        return None;
    }
    Some(
        before
            .then(own)
            .then(chain_of(comp, &comp.parent_chain(&layer.id), frame, t).matrix)
            .then(match camera {
                Projection::Scaled(p) => p,
                _ => Affine::IDENTITY,
            }),
    )
}

/// D-188: a motion-blurred layer's moments drawn one by one, summed in order and divided by
/// their number once, in linear premultiplied light. The average becomes the layer's picture,
/// drawn through the identity, so its matte, opacity and blend come after it, once. A moment
/// behind the camera counts and adds nothing.
///
/// A drawn layer is averaged at the preview's size, so a draft pays for a draft's pixels; its
/// transform is then the draft scale's inverse, which [`crate::preview::scale_plan`] undoes
/// exactly. A matte is averaged at the composition's size, because a matte's transform is not
/// scaled for a draft.
fn settle(
    mut resolved: ResolvedLayer,
    comp: &crate::model::Composition,
    quality: PreviewQuality,
    matte: bool,
) -> ResolvedLayer {
    if resolved.moments.is_empty() {
        return resolved;
    }
    let d = if matte { 1 } else { quality.divisor() };
    let (width, height) = if d == 1 {
        (comp.width as usize, comp.height as usize)
    } else {
        quality.extent(comp.width as usize, comp.height as usize)
    };
    let moments: Vec<Option<Affine>> = if d == 1 {
        resolved.moments.clone()
    } else {
        resolved.moments.iter().map(|m| m.map(|m| m.then(Affine::scaling(1.0 / d as f64, 1.0 / d as f64)))).collect()
    };
    resolved.source = std::sync::Arc::new(render::average(&resolved.source, &moments, width, height));
    resolved.transform = if d == 1 {
        Affine::IDENTITY
    } else {
        Affine::scaling(d as f64, d as f64)
    };
    resolved
}

/// D-189: the map an effect on layer `holder` reads from the layer its setting `named`, at
/// composition frame `frame`: document 21's four steps for a layer setting, fitted by `fit`.
///
/// `None` is no map: an empty name, which says nothing; a name that is no layer of the holder's
/// composition, which says `EFFECT_LAYER_MISSING`; or a fit word that is none of the three. A
/// layer with nothing to show at `frame` - a null, a layer not in, a missing drawing or
/// composition - gives a transparent map, and its own diagnostic says why.
#[allow(clippy::too_many_arguments)]
pub fn layer_map(
    project: &Project,
    composition_id: &Id,
    holder: &Id,
    named: &str,
    fit: &str,
    frame: i32,
    root: &Path,
    quality: PreviewQuality,
    log: &mut FrameLog,
) -> Option<WorkingBuffer> {
    let comp = project.composition(composition_id)?;
    let holder = comp.layer(holder)?;
    // Step 3: the map is made at the size the holder's own effects run at, the draft divisor
    // where D-99, D-67 or D-66 runs them there and full size for a solid or a shape layer.
    let small = holder.composition_id.is_some()
        || holder.is_adjustment()
        || (holder.kind == crate::model::LayerKind::Raster && holder.effects.iter().any(|i| i.enabled));
    let quality = if small { quality } else { PreviewQuality::Full };
    let size = resolve_layer(
        project,
        comp,
        &bare(holder, false),
        frame,
        root,
        quality,
        &mut CelCache::none(),
        &mut FrameLog::new(usize::MAX),
        &mut Vec::new(),
        false,
        true,
    )
    .map_or((0, 0), |r| (r.source.width(), r.source.height()));
    setting_map(project, comp, holder, named, fit, frame, root, quality, size, &mut CelCache::none(), log)
}

/// A layer with its effects taken off, and its masks as well unless `masks`.
fn bare(layer: &crate::model::Layer, masks: bool) -> crate::model::Layer {
    let mut layer = layer.clone();
    layer.effects.clear();
    if !masks {
        layer.masks.clear();
    }
    layer
}

/// D-191: each switched-on effect's layer setting read into its map for this frame, fitted to
/// `size`. The loader and the commands refuse a circle of settings, so the check here is only
/// a guard: one reached some other way is said and drawn without, rather than never ending.
#[allow(clippy::too_many_arguments)]
fn fill_maps(
    effects: &mut [crate::effects::EffectInstance],
    project: &Project,
    root: &Path,
    comp: &crate::model::Composition,
    holder: &crate::model::Layer,
    frame: i32,
    quality: PreviewQuality,
    size: (usize, usize),
    cache: &mut CelCache,
    log: &mut FrameLog,
) {
    for instance in effects.iter_mut().filter(|i| i.enabled) {
        let Some((named, fit)) = instance.effect.layer_setting().map(|(n, f)| (n.to_string(), f.to_string())) else {
            continue;
        };
        let map = if comp.effect_layer_cycle_from(&holder.id) {
            log.record(
                frame,
                holder.name.clone(),
                Diagnostic::new(
                    DiagnosticId::EffectLayerCycle,
                    Severity::Error,
                    format!("The effects on layer {} read layers that read it back.", holder.name),
                    format!("Frame {frame} is drawn without the effect: D-189 refuses a circle of layer settings."),
                ),
            );
            None
        } else {
            setting_map(project, comp, holder, &named, &fit, frame, root, quality, size, cache, log)
        };
        if let Some((_, slot)) = instance.effect.layer_setting_mut() {
            *slot = map.map(|m| crate::layer_map::Map(std::sync::Arc::new(m)));
        }
    }
}

/// D-195: each switched-on Echo's copies, the holder's drawing through its masks at the frames
/// its settings name, put together at `size`. What drawing them logged belongs to this frame,
/// once for each kind.
#[allow(clippy::too_many_arguments)]
fn fill_echoes(
    effects: &mut [crate::effects::EffectInstance],
    project: &Project,
    root: &Path,
    comp: &crate::model::Composition,
    holder: &crate::model::Layer,
    frame: i32,
    quality: PreviewQuality,
    size: (usize, usize),
    cache: &mut CelCache,
    log: &mut FrameLog,
) {
    for instance in effects.iter_mut().filter(|i| i.enabled && i.is_valid()) {
        let crate::effects::Effect::Echo { echo_time, echoes, intensity, decay, operator, picture } =
            &mut instance.effect
        else {
            continue;
        };
        let drawing = bare(holder, true);
        let mut inside = FrameLog::new(usize::MAX);
        let mut fold = crate::layer_fx::EchoFold::new(operator, size);
        for k in 0..=echoes.floor() as i32 {
            let at = frame + k * echo_time.floor() as i32;
            let p = resolve_layer(project, comp, &drawing, at, root, quality, cache, &mut inside, &mut Vec::new(), false, true)
                .map(|r| crate::layer_map::cut(&r.source, (0, 0), size));
            crate::perf::time(crate::perf::Stage::EffectEcho, || fold.add(p.as_ref(), *intensity * decay.powi(k)));
        }
        let mut said = Vec::new();
        for d in inside.finish() {
            if !said.contains(&d.id) {
                said.push(d.id);
                log.record(frame, holder.name.clone(), d);
            }
        }
        let done = crate::perf::time(crate::perf::Stage::EffectEcho, || fold.finish());
        *picture = Some(crate::layer_map::Map(std::sync::Arc::new(done)));
    }
}

/// D-189's map of layer `named` for an effect on `holder`, fitted by `fit` to `size`.
#[allow(clippy::too_many_arguments)]
fn setting_map(
    project: &Project,
    comp: &crate::model::Composition,
    holder: &crate::model::Layer,
    named: &str,
    fit: &str,
    frame: i32,
    root: &Path,
    quality: PreviewQuality,
    size: (usize, usize),
    cache: &mut CelCache,
    log: &mut FrameLog,
) -> Option<WorkingBuffer> {
    if named.is_empty() {
        return None;
    }
    let Some(layer) = comp.layer(&Id::new(named)) else {
        log.record(
            frame,
            holder.name.clone(),
            crate::layer_map::missing(&holder.name, named, &format!("frame {frame} is drawn")),
        );
        return None;
    };
    // Step 1: the holder itself gives its drawing and masks, and an adjustment layer its white
    // through its masks; their effects are not run.
    let stripped;
    let layer = if layer.id == holder.id || layer.is_adjustment() {
        stripped = bare(layer, true);
        &stripped
    } else {
        layer
    };
    let picture = resolve_layer(project, comp, layer, frame, root, quality, cache, log, &mut Vec::new(), false, true)
        .map_or_else(|| std::sync::Arc::new(WorkingBuffer::transparent(0, 0)), |resolved| resolved.source);
    crate::layer_map::fit(&picture, fit, size)
}

/// [`plan_frame`], then document 20's step 8.
///
/// The result is in the working space. Turning it into a file is the display transform, which is
/// step 9 and belongs to whoever asked for the frame — the viewer wants one destination, an
/// export wants another, and doing it here would do it twice.
pub fn render_frame(
    project: &Project,
    composition_id: &Id,
    frame: i32,
    root: &Path,
    tile_size: usize,
    log: &mut FrameLog,
) -> Result<WorkingBuffer, Diagnostic> {
    let plan = plan_frame(project, composition_id, frame, root, log)?;
    Ok(render::render(&plan, tile_size))
}

/// D-216: the whole source frame `f` a layer shows at composition frame `at`, and how far
/// toward `f + 1` it mixes: 0 unless the layer and the composition both blend, `t` is between
/// frames and `f + 1` is still inside a source that ends at `end` (a still has no end).
fn source_frames(
    comp: &crate::model::Composition,
    layer: &crate::model::Layer,
    at: i32,
    end: Option<i32>,
) -> Option<(i32, f64)> {
    let t = layer.source_time(at)?;
    let f = t.floor();
    let w = t - f;
    let f = f as i32;
    let blends = layer.frame_blend && comp.frame_blending && w > 0.0;
    Some((f, if blends && end.is_none_or(|e| f + 1 < e) { w } else { 0.0 }))
}

/// D-216's mix, `a + w (b - a)` number by number, on the larger of the two sizes from the top
/// left, a missing picture or pixel counting as transparent. `None` only when both are.
fn mix(a: Option<&WorkingBuffer>, b: Option<&WorkingBuffer>, w: f64) -> Option<WorkingBuffer> {
    if a.is_none() && b.is_none() {
        return None;
    }
    let size = |p: Option<&WorkingBuffer>| p.map_or((0, 0), |p| (p.width(), p.height()));
    let ((aw, ah), (bw, bh)) = (size(a), size(b));
    let (width, height) = (aw.max(bw), ah.max(bh));
    let px = |p: Option<&WorkingBuffer>, x: usize, y: usize| match p {
        Some(p) if x < p.width() && y < p.height() => p.pixel(x, y),
        _ => [0.0; 4],
    };
    let mut out = WorkingBuffer::transparent(width, height);
    // B-156c: each number stands alone, so the rows are shared among the processor's threads;
    // the sum is the same one, so the bytes are too.
    use rayon::prelude::*;
    out.data_mut().par_chunks_exact_mut(4).enumerate().for_each(|(i, o)| {
        let (x, y) = (i % width, i / width);
        let (p, q) = (px(a, x, y), px(b, x, y));
        for c in 0..4 {
            o[c] = (p[c] as f64 + w * (q[c] as f64 - p[c] as f64)) as f32;
        }
    });
    Some(out)
}

/// The first local frame past a drawn layer's exposures, or `None` for a still, which has none.
fn raster_end(layer: &crate::model::Layer) -> Option<i32> {
    layer.exposure_spans.iter().map(|s| s.end_frame_exclusive).max()
}

/// D-216's Drawing Dissolve at whole local frame `f`: the local frame where the next drawing is
/// exposed and how far toward it `f` is, when `f` is in the last `d` frames of its span and a
/// span starts where it ends.
fn dissolving(layer: &crate::model::Layer, f: i32) -> Option<(i32, f64)> {
    if layer.drawing_dissolve == 0 {
        return None;
    }
    let spans = &layer.exposure_spans;
    let here = spans.iter().find(|s| s.start_frame <= f && f < s.end_frame_exclusive)?;
    let (s, e) = (here.start_frame, here.end_frame_exclusive);
    spans.iter().find(|x| x.start_frame == e)?;
    let d = (layer.drawing_dissolve as i32).min(e - s - 1);
    (d > 0 && f >= e - d).then(|| (e, (f - (e - d) + 1) as f64 / (d + 1) as f64))
}

/// `Ok(None)` is a frame this layer is transparent at: inactive, or exposing nothing. `local` is
/// the layer-local frame, `frame` the composition frame the diagnostics name.
fn source_at(
    spans: Vec<crate::time::ExposureSpan>,
    asset: &crate::model::Asset,
    local: i32,
    frame: i32,
) -> Result<Option<PathBuf>, Diagnostic> {
    match asset.kind {
        // D-71: a sound file is not a drawing. D-182: nor is a lookup file.
        AssetKind::Audio | AssetKind::Lut => Ok(None),
        AssetKind::Still => {
            match &asset.path {
                Some(p) => Ok(Some(PathBuf::from(p))),
                None => Err(schema_invalid(format!(
                    "Still asset {} has no file path.",
                    asset.name
                ))),
            }
        }
        AssetKind::ImageSequence => {
            let exposures = ExposureMap::new(spans).map_err(|e| {
                schema_invalid(format!(
                    "Asset {}'s exposure spans are invalid: {e}",
                    asset.name
                ))
            })?;
            let frames: BTreeMap<u32, PathBuf> = asset
                .frames
                .iter()
                .map(|(n, p)| (*n, PathBuf::from(p)))
                .collect();
            let pattern = asset.pattern.as_deref().unwrap_or(&asset.name);
            match time::resolve_local(&exposures, &frames, pattern, local, frame)? {
                SourceAt::Transparent => Ok(None),
                SourceAt::Drawing { path, .. } => Ok(Some(path)),
            }
        }
    }
}

/// Re-tag a decoded buffer with what the asset record says it is, if that differs from what the
/// decoder assumed. The pixels are untouched: this changes the claim, and `into_working` is what
/// acts on it.
pub(crate) fn retag(
    buffer: ImageBuffer,
    interpretation: crate::model::Interpretation,
) -> ImageBuffer {
    if buffer.color_space() == interpretation.color_space
        && buffer.alpha_mode() == interpretation.alpha
    {
        return buffer;
    }
    let (w, h) = (buffer.width(), buffer.height());
    ImageBuffer::new(
        w,
        h,
        interpretation.color_space,
        interpretation.alpha,
        buffer.data().to_vec(),
    )
    .expect("the data came from a buffer of the same extent")
}

fn wrong_kinds(layer: &crate::model::Layer, frame: i32) -> String {
    let names: Vec<&str> = layer
        .transform
        .value_at(frame)
        .iter()
        .filter(|(prop, value)| match prop {
            Prop::Rotation | Prop::Opacity => !matches!(value, Value::Scalar(_)),
            _ => !matches!(value, Value::Vec2(_, _)),
        })
        .map(|(prop, _)| prop.as_str())
        .collect();
    names.join(", ")
}

fn schema_invalid(message: String) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::ProjectSchemaInvalid,
        Severity::Error,
        message,
        "The layer is left out of this frame rather than drawn from a guess.".to_string(),
    )
}
