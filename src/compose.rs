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
//! Steps 2, 3 and 5 of document 21 — mask, effects, matte — are parked (document 23, R-04 and
//! R-05). A layer carrying a matte still renders; what it does not do is render silently, which
//! is why the matte earns a `PROJECT_FEATURE_UNSUPPORTED` line in the log.
//!
//! Nothing here is the viewer. There is no transport, no playback, no work area and no window:
//! those are the rest of B-08 and they need decisions this build has not been given.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use crate::cache::CelCache;
use crate::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use crate::model::{AssetKind, Id, Project, Prop, Value};
use crate::render::{self, Affine, FramePlan, LayerDraw};
use crate::time::{self, ExposureMap, LayerTiming, SourceAt};
use crate::{ImageBuffer, WorkingBuffer};

/// The tile size this build uses when the caller has no opinion.
///
/// Document 21: "Tile size is a tunable measured on the reference machine, not a constant chosen
/// in advance." It was measured. `verification/B-05a_scaling_table.md` renders the same frame at
/// four tile sizes and two thread counts: 128 pixels is the fastest at 12 threads (17.1 ms) and
/// within a millisecond of the best at 24, and every size produced byte-identical output. This
/// constant is that measurement, not a guess, and moving it changes speed only.
pub const DEFAULT_TILE_SIZE: usize = 128;

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

    let mut layers = Vec::new();
    // Step 3: composition order, bottom of the stack first, which is `FramePlan.layers`' order.
    for layer in comp.layers_in_order() {
        if !layer.enabled {
            continue;
        }
        if layer.matte.is_some() {
            log.record(
                frame,
                layer.name.clone(),
                Diagnostic::new(
                    DiagnosticId::ProjectFeatureUnsupported,
                    Severity::Warning,
                    format!(
                        "Layer {} has a track matte, which this build does not render.",
                        layer.name
                    ),
                    "The matte reference is preserved in the project and takes no part in this \
                     frame. The layer is drawn as if it had none, so it may cover more than it \
                     will once mattes are implemented."
                        .to_string(),
                )
                .with_remediation("Document 23 parks mattes with R-04; nothing to do here yet."),
            );
        }

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
            continue;
        };

        let timing = layer.timing();
        // Steps 4 and 5: the layer-local frame, then the drawing exposed at it.
        let relative = match source_at(layer.exposure_spans.clone(), &timing, asset, frame) {
            Ok(Some(path)) => path,
            Ok(None) => continue,
            Err(d) => {
                log.record(frame, layer.name.clone(), d);
                continue;
            }
        };
        let path = root.join(&relative);
        if !path.exists() {
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
                        "Layer {} looked for it at {} for frame {frame}. The reference is kept \
                         and the layer is left out of this frame; no neighbouring drawing is \
                         substituted for it.",
                        layer.name,
                        path.display()
                    ),
                )
                .with_remediation("Relink the sequence, or put the file back where it was."),
            );
            continue;
        }
        // Document 21 step 1: decode, then interpret. `decode_png` tags what PNG guarantees —
        // sRGB, straight — and the asset record is what overrides it, so a project that says a
        // sequence was rendered premultiplied is believed here and nowhere else. All three of
        // those steps happen inside the cache, because all three are what a hit skips.
        let source = match cache.decoded(&path, asset.interpretation) {
            Ok(buffer) => buffer,
            Err(d) => {
                log.record(frame, layer.name.clone(), d);
                continue;
            }
        };

        // Step 6: the animated properties at this frame. A property holding the wrong kind of
        // value cannot come from a loaded project — persistence refuses it — so this reports
        // rather than guesses a default, which would put a layer somewhere nobody asked for.
        let t = &layer.transform;
        let (Some(anchor), Some(position), Some(scale), Some(rotation), Some(opacity)) = (
            t.anchor.value_at(frame).as_vec2(),
            t.position.value_at(frame).as_vec2(),
            t.scale.value_at(frame).as_vec2(),
            t.rotation.value_at(frame).as_scalar(),
            t.opacity.value_at(frame).as_scalar(),
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
            continue;
        };

        layers.push(LayerDraw {
            id: layer.id.clone(),
            source,
            // Document 21 step 4. Scale is a unit factor in the model (D-22); the divide by 100
            // lives at the file and UI boundaries, not here.
            transform: Affine::from_transform(anchor, position, scale, rotation),
            // Document 21 step 6. Opacity is normalized 0..1 in the model (document 19).
            opacity: opacity as f32,
            blend: layer.blend_mode,
        });
    }

    Ok(FramePlan {
        width: comp.width as usize,
        height: comp.height as usize,
        layers,
    })
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

/// `Ok(None)` is a frame this layer is transparent at: inactive, or exposing nothing.
fn source_at(
    spans: Vec<crate::time::ExposureSpan>,
    timing: &LayerTiming,
    asset: &crate::model::Asset,
    frame: i32,
) -> Result<Option<PathBuf>, Diagnostic> {
    match asset.kind {
        AssetKind::Still => {
            if timing.local_frame(frame).is_none() {
                return Ok(None);
            }
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
            match time::resolve_in(timing, &exposures, &frames, pattern, frame)? {
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
