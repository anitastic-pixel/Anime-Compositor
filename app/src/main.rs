#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! The desktop shell: a window, a canvas, and the transport that puts frames in it.
//!
//! Everything about how a frame is *made* lives in `anime_compositor`. What lives here is the
//! part that could not be checked by a fixture — a window, a wall clock, and a request. The
//! division is document 06's ("the rendering core stays independent of the interface") and cargo
//! enforces it: the arrow points this way and there is no way back.
//!
//! Two decisions are implemented rather than described:
//!
//! - **D-36.** Frames reach the webview over a custom URI scheme, not over IPC. A frame is an
//!   ordinary resource with a content type; the IPC channel stays free for commands, so a frame
//!   in flight cannot delay a stop or a scrub.
//! - **D-32.** The wall clock is in the page and the *decision* is in the core. The page asks
//!   "what belongs on screen at this instant", `Playback::at` answers, and the frames between
//!   that answer and the last one are counted as skipped rather than shown late. The page never
//!   computes a frame number.
//!
//! And D-33's indicator is a header on every response, derived from the quality that rendered
//! the frame, so it cannot describe a resolution other than the one on screen.
//!
//! A project reaches the window four ways: named on the command line, dropped on it, chosen in
//! the Open dialog, or picked out of the recent list. All four go through [`open`], which is
//! `persist::load` and nothing else, so what the viewer says about a project is what the core
//! said about it — including everything document 28 asks to be told.
//!
//! It goes back out through [`write`], which is `persist::save` and nothing else, so the atomic
//! replacement SP-01 measured and the unknown-data preservation document 28 requires are the
//! core's, not a second implementation of them living in a window. The shell holds the
//! [`Document`] and its [`Preserved`] rather than a copy of the project, for one reason: a
//! window that saved a project parsed into the parts this build understands would quietly drop
//! the parts it does not, and the person would find out when their masks were gone.
//!
//! The commands are a second custom scheme, `http://project.localhost`, beside the frame one.
//! Not IPC: the frame scheme is already proven in this window and needs no permission surface,
//! and a command here is a request with an answer, which is the shape a fetch already has. IPC
//! stays free, which is what D-36 wanted it for.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anime_compositor::audio;
use anime_compositor::cache::{CelCache, Finished, FrameCache, Sight};
use anime_compositor::command::{Command, Document, Target, TimeRemap};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{Diagnostic, DiagnosticId, FrameLog, Severity};
use anime_compositor::effects::{
    Effect, EffectInstance, EffectKey, ADD_GRAIN, ADVANCED_SPILL_SUPPRESSOR, AERIAL_HAZE,
    ARBITRARY_MAP, AUDIO_SPECTRUM, AUDIO_WAVEFORM, BASIC_TEXT, BEAM, BENDER, BEND_IT, BEVEL_ALPHA,
    BEVEL_EDGES, BEZIER_WARP, BILATERAL_BLUR, BLACK_WHITE, BLOBBYLIZE, BLOCK_DISSOLVE, BLOCK_LOAD,
    BLOOM, BRIGHTNESS_CONTRAST, BROADCAST_SAFE, BRUSH_STROKES, BUBBLES, BUBBLES_SETTINGS,
    BUBBLES_WORDS, BULGE, BURN_FILM, CAMERA_SHAKE, CARD_DANCE, CARD_DANCE_TRANSFORMS, CARD_WIPE,
    CARTOON, CAUSTICS, CC_BALL_ACTION, CC_CYLINDER, CC_ENVIRONMENT, CC_RAINFALL, CC_SPHERE,
    CC_SPOTLIGHT, CC_VIGNETTE, CC_WIDE_TIME, CDK_NAMES, CELL_PATTERN, CHANGE_COLOR,
    CHANGE_TO_COLOR, CHANNEL_BLUR, CHANNEL_MIXER, CHANNEL_THRESHOLD, CHANNEL_WIPE, CHECKERBOARD,
    CHROMATIC_ABERRATION, CIRCLE, COLORAMA, COLOR_BALANCE, COLOR_BALANCE_HLS, COLOR_DIFFERENCE_KEY,
    COLOR_GRADE, COLOR_GRADE_SETTINGS, COLOR_KEY, COLOR_LINK, COLOR_LOOKUP, COLOR_NEUTRALIZER,
    COLOR_OFFSET, COLOR_RANGE, COLOR_STABILIZER, COMPOUND_BLUR, COMPRESSOR, CONTOUR_PAINT,
    CORNER_PIN, CRANGE_NAMES, CROSS_BLUR, CROSS_GLARE, CRYPTOMATTE, CURL_NOISE, CURVES, DEBAND,
    DEPTH_KEY, DEPTH_OF_FIELD, DETAIL_UPSCALE, DIFFERENCE_MATTE, DIFFUSION, DIRECTIONAL_BLUR,
    DISPLACEMENT_MAP, DISTANCE_GRADATION, DISTORTION, DRIZZLE, DRIZZLE_SETTINGS, DRIZZLE_WORDS,
    DROP_SHADOW, DUST_SCRATCHES, ECHO, ELLIPSE, EMBOSS, ENERGY_STROKE, EXPOSURE, EXPOSURE_FLICKER,
    EXTRACT, EYEDROPPER_FILL, FAST_BLUR_LEGACY, FAST_BOX_BLUR, FAST_ZOOM_BLUR, FILL, FIND_EDGES,
    FISHEYE, FLOW_MOTION, FOAM, FOAM_SETTINGS, FOAM_WORDS, FOG_3D, FORCE_MOTION_BLUR,
    FOUR_COLOR_GRADIENT, FRACTAL, FRACTAL_NOISE, GAMMA_PEDESTAL_GAIN, GAUSSIAN_BLUR,
    GAUSSIAN_BLUR_LEGACY, GLASS, GLASS_MELT, GLOW, GLOW_WIPE, GLUE_GUN, GRADIENT, GRADIENT_MAP,
    GRADIENT_WIPE, GRID, GRIDDLER, HAIR, HAIR_SETTINGS, HAIR_WORDS, HALATION, HALFTONE, HEX_TILE,
    HSV_KEY, HUE_RANGES, HUE_SATURATION, IDENTIFIER, ID_KEY, INNER_OUTER_KEY, INVERT, IRIS_WIPE,
    KALEIDOSCOPE, KERNEL, KEY_CLEANER, KIRA_KIRA, LEAVE_COLOR, LENS_BLUR, LENS_FLARE, LEVELS,
    LEVELS_INDIVIDUAL, LEVELS_NAMES, LEVELS_PLAIN, LIGHTNING_BOLT, LIGHT_BURST, LIGHT_RAYS,
    LIGHT_SWEEP, LIGHT_WRAP, LINEAR_COLOR_KEY, LINEAR_WIPE, LINE_BLUR, LINE_RECOLOR, LINE_SMOOTH,
    LINE_WIDTH, LIQUIFY, MAGNIFY, MASK_SMEAR, MATCH_GRAIN, MATTE_CHOKER, MEDIAN, MERCURY,
    MERCURY_SETTINGS, MERCURY_WORDS, MESH_WARP, MIRROR, MOMENT_MAP, MOSAIC, MOTION_TILE,
    MR_SMOOTHIE, NOISE, NOISE_ALPHA, NOISE_GATE, NOISE_HLS, NOISE_HLS_AUTO, NUMBERS, OFFSET,
    OPTICS_COMPENSATION, OUTLINE, PAGE_TURN, PAINT_BUCKET, PARAFFIN, PARTICLE_EMITTER,
    PARTICLE_EMITTER_COLORS, PARTICLE_EMITTER_SETTINGS, PARTICLE_EMITTER_WORDS, PARTICLE_GRID,
    PARTICLE_GRID_COLORS, PARTICLE_GRID_LAYERS, PARTICLE_GRID_SETTINGS, PARTICLE_GRID_WORDS,
    PARTICLE_PLANE, PARTICLE_PLANE_COLORS, PARTICLE_PLANE_SETTINGS, PARTICLE_PLANE_WORDS,
    PARTICLE_PLAYGROUND, PARTICLE_PLAYGROUND_COLORS, PARTICLE_PLAYGROUND_LAYERS,
    PARTICLE_PLAYGROUND_SETTINGS, PARTICLE_PLAYGROUND_WORDS, PARTICLE_SPACE, PARTICLE_SPACE_COLORS,
    PARTICLE_SPACE_SETTINGS, PARTICLE_SPACE_WORDS, PASS_EXTRACT, PHOTO_FILTER, PIXEL_MOTION_BLUR,
    PLASTIC, POLAR_COORDINATES, POLYGON_SHARDS, POLYGON_SHARDS_SETTINGS, POLYGON_SHARDS_WORDS,
    POSTERIZE, POSTERIZE_TIME, POWER_PIN, PUSH_IRIS, RADIAL_BLUR, RADIAL_SHADOW, RADIAL_WIPE,
    RADIO_WAVES, RAIN, REDUCE_INTERLACE_FLICKER, REFINE_HARD_MATTE, REFINE_SOFT_MATTE,
    REMOVE_GRAIN, REPEAT_TILE, RESHAPE, RIM_LIGHT, RIPPLE, RIPPLE_PULSE, ROLLING_SHUTTER_REPAIR,
    ROUGHEN_EDGES, SCATTER, SCATTERIZE, SCATTERIZE_SETTINGS, SCATTERIZE_WORDS, SCREEN_KEY,
    SCRIBBLE, SELECTIVE_COLOR, SELECTIVE_COLOR_BLUR, SELECT_COLOR, SHADOW_HIGHLIGHT, SHARPEN,
    SHATTER, SHIFT_CHANNELS, SIMPLE_CHOKER, SIMPLE_WIRE_REMOVAL, SLANT, SMART_BLUR, SMEAR,
    SNOWFALL, SOLARIZE, SOLID_COMPOSITE, SPEED_LINES, SPHERIZE, SPILL_SUPPRESSOR, SPIN_ZOOM_BLUR,
    SPLIT, SPLIT_2, SPREAD_TONES, STAR_BURST, STAR_BURST_SETTINGS, STRETCH_COLOR, STRETCH_CONTRAST,
    STRETCH_LEVELS, STRETCH_WIPE, STRIPE_SWEEP, STROBE_LIGHT, STROKE, TDIFF_ALPHAS, TEETH_WIPE,
    TEXTURIZE, TEXT_ANIMATOR, THREADS, THREE_D_GLASSES, THRESHOLD, THRESHOLD_RGB, TILES, TILE_WIPE,
    TIMECODE, TIMEWARP, TIME_DIFFERENCE, TINT, TONER, TRANSFORM, TRITONE, TURBULENT_DISPLACE,
    TURBULENT_NOISE, TWIRL, TWISTER, UNMULT, VECTOR_BLUR, VEGAS, VENETIAN_BLINDS, VIBRANCE,
    VIGNETTE, WARP, WARP_BLEND, WARP_STABILIZER, WAVE_WARP, WAVE_WORLD, WRITE_ON,
};
use anime_compositor::export::{
    self, ExportChoices, ExportReport, ExportRequest, ExportStatus, MissingSource, OutputFormat,
};
use anime_compositor::exr_io::{self, ExrSamples};
use anime_compositor::gpu::{Gpu, Paint};
use anime_compositor::lut;
use anime_compositor::media;
use anime_compositor::model::{
    Asset, AssetKind, BlendMode, Composition, Expression, Id, Interp, Interpretation, Keyframe,
    Layer, LayerKind, Marker, MatteMode, Project, Prop, Property, Solid, Value,
};
use anime_compositor::mp4_out::Mp4Quality;
use anime_compositor::package::{self, Answer};
use anime_compositor::persist::{self, Preserved};
use anime_compositor::preview::{self, Playback, PreviewQuality};
use anime_compositor::session_log::{self, SessionLog};
use anime_compositor::time::{ExposureSpan, FrameRate};
use anime_compositor::timesheet;
use anime_compositor::{AlphaMode, ColorSpace};
use anime_compositor::{OutputAlpha, OutputDepth};
use tauri::http::{Request, Response};
use tauri::{AppHandle, DragDropEvent, Manager, WindowEvent};
use tauri_plugin_dialog::DialogExt;

/// What the shell is looking at.
///
/// One project, one composition, one clock, and whatever the core had to say when it was
/// opened.
struct Viewer {
    /// The project and its undo history, as the core models it. Held whole rather than as a
    /// `Project` so that saving is `persist::save` of the thing that was loaded.
    document: Document,
    /// Everything in the file this build does not model — masks, effects, anything a later
    /// version writes. Opaque here on purpose; the shell's only dealing with it is handing it
    /// back to `persist::save` unread.
    preserved: Preserved,
    /// Where Save writes. `None` until the project has a file of its own, which is the state
    /// [`demo`] is in and the reason Save falls through to Save As rather than overwriting
    /// something in `verification/`.
    path: Option<PathBuf>,
    /// The directory asset paths resolve against. Normally the project file's own directory,
    /// which is the rule `persist::load` checks media against — see [`demo`] for the one
    /// exception and why it exists.
    root: PathBuf,
    composition: Id,
    quality: PreviewQuality,
    playback: Playback,
    /// What to call the open project on screen.
    name: String,
    /// Document 28's warnings from opening it, verbatim, or empty when there were none.
    /// Never summarised and never dropped: a project that opened with missing media is not the
    /// same project as one that opened cleanly, and the window has to say which one is on
    /// screen.
    notes: Vec<String>,
    /// What the last command did, in one sentence, or empty. Separate from [`notes`](Self::notes)
    /// because a warning is about the project and this is about the last thing the person asked
    /// for: "Saved to shot.json" is not something wrong with the project, and a save that failed
    /// must not be filed away among warnings that were already there.
    status: String,
    /// The recovery snapshots found beside this project when it was opened, newest first, or
    /// empty. Read once at open rather than on every frame: a header is written for every frame
    /// and asking the file system five times a frame for files that change every two minutes
    /// would be a cost paid sixty times a second for nothing.
    recovery: Vec<PathBuf>,
    /// What the autosave timer last did, in one sentence, or empty. Its own line rather than
    /// [`status`](Self::status), which belongs to the last thing the *person* asked for: an
    /// autosave happens on a clock while somebody is doing something else, and overwriting
    /// "Saved to shot.json" with it would take away the answer they were waiting for.
    autosaved: String,
    /// When this document first became dirty, as far as the timer has seen. `None` while it is
    /// clean. Document 07 asks for a snapshot "after two minutes of dirty activity", so the two
    /// minutes are measured from here.
    dirty_since: Option<Instant>,
    /// Document 05 line 31's alpha-only view and transparency grid, and document 21 line 97's
    /// rule about both: they are presentation. Neither is in the project, neither is undoable
    /// (document 24 says so in its own column), and neither may change a cached pixel or
    /// anything exported. The grid is drawn by the page behind a canvas that already has
    /// alpha, so it touches no pixel at all; the alpha view is applied to the copy of the
    /// frame that is about to leave, after the cache has been given the frame it keeps.
    alpha_only: bool,
    checkerboard: bool,
    /// W-24: the layers soloed, which the preview draws alone. Presentation like the alpha view:
    /// not in the file, not undoable, and never exported.
    solo: Vec<Id>,
    /// W-25: the layers Ctrl+C took, as they were, for Ctrl+V. The window's, like `solo`.
    clipboard: Vec<Layer>,
    /// W-02's relink, worked out and not yet agreed to. See [`PendingRelink`]. At most one at a
    /// time: a person is answering one question, and a second proposal replaces the first rather
    /// than queueing behind it.
    relink: Option<PendingRelink>,
    /// B-08b: the decoded cels this preview has already paid for. Belongs to the viewer rather
    /// than to a frame because its whole purpose is to outlive one, and it is replaced along with
    /// everything else when a different project is opened, so nothing from the old one survives.
    /// Behind a lock of its own since P-04, and held by the viewer only so that opening a
    /// different project still replaces it along with everything else. A frame is rendered
    /// outside the viewer lock, so the one thing a render holds for its whole length has to
    /// be something a command never needs, and no command touches this.
    cache: Arc<Mutex<CelCache>>,
    /// B-154: finished frames, and where playback is, for the render-ahead. Replaced with the
    /// project, as the cels are.
    ram: Arc<Mutex<Ram>>,
}

/// B-154 (G3): the viewer's RAM preview.
struct Ram {
    frames: FrameCache,
    /// The frame playback last asked for, or `None` when it is not playing. The render-ahead
    /// works outward from here and stops when this is `None`.
    playing: Option<i32>,
    /// What the card is showing in the window, so a frame made ahead on the card can put it back.
    shown: Option<Finished>,
}

/// What the page is asking for.
///
/// Split out from the request handling so it can be tested without a window, which is the only
/// part of the transport a headless test can reach.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Ask {
    /// The frame the playback clock says belongs on screen this many milliseconds after
    /// playback began. The page supplies the instant; the core decides the frame.
    At(u64),
    /// One named frame, for stepping. The clock is not consulted and nothing is skipped.
    Frame(i32),
    /// Start the clock over from this frame and answer it (W-09). What the page asks for when
    /// Play is pressed, so that playback begins under the playhead.
    Play(i32),
}

/// Read `/at/<milliseconds>`, `/frame/<n>` or `/play/<n>`, with an optional `?q=draft|full`.
///
/// Returns `None` for anything else, which the handler answers with a 404 rather than guessing.
/// An unreadable quality is `None` in the second slot, meaning "leave it as it is": a typo in a
/// query string should not silently switch the preview to a resolution nobody asked for.
fn parse(path: &str, query: Option<&str>) -> Option<(Ask, Option<PreviewQuality>)> {
    let mut parts = path.trim_matches('/').split('/');
    let ask = match (parts.next()?, parts.next()?, parts.next()) {
        ("at", ms, None) => Ask::At(ms.parse().ok()?),
        ("frame", n, None) => Ask::Frame(n.parse().ok()?),
        ("play", n, None) => Ask::Play(n.parse().ok()?),
        _ => return None,
    };
    Some((ask, quality_asked(query)))
}

/// The `?q=draft|half|full` half of a frame request, shared by `/frame`, `/at` and `/boxes`.
///
/// `None` means "leave it as it is": a typo in a query string should not silently switch the
/// preview to a resolution nobody asked for, and a selection outline asking for the wrong one
/// would draw its boxes at four times the size of the picture under them.
fn quality_asked(query: Option<&str>) -> Option<PreviewQuality> {
    query
        .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix("q=")))
        .and_then(|value| match value {
            "draft" => Some(PreviewQuality::Draft),
            "half" => Some(PreviewQuality::Half),
            "full" => Some(PreviewQuality::Full),
            _ => None,
        })
}

/// Where each layer's drawing lands on the frame that is on screen, as four corners.
///
/// The window is the only thing that can answer this. A layer's box is its source rectangle
/// carried through its transform, and the page has neither: document 07's project format records
/// no pixel size for a drawing, so the page cannot know how big a cel is, and the transform it
/// does hold is a set of numbers rather than a matrix. Asking the renderer is not a shortcut
/// around that — it is the only place the answer exists.
///
/// The corners are in the frame's own pixels, which is what the canvas is drawn in, so a draft
/// preview's boxes are a quarter the size of a full one's and land on the same drawing either
/// way. That is why the plan is [`anime_compositor::compose::plan_preview`]'s here, exactly as
/// it is when the frame is rendered.
///
/// A layer with no corners is a layer the plan does not contain — hidden, out of its own
/// `[in_frame, out_frame)`, or missing its drawing at this frame — and having no box is the
/// truthful answer for it rather than an empty one.
///
/// This is the preview path and holds the preview's cache, so it costs a plan the frame beside
/// it has usually already paid for. ADR-015 is untouched: nothing here is reachable from export.
/// One property of one layer, evaluated at every frame of a range, for the graph editor.
///
/// The graph editor draws a curve, and it has to know where the curve goes. It could work that
/// out from the keyframes the page already holds, which is what a graph editor usually does -
/// but then the shape on screen would be a second implementation of document 20, free to
/// disagree with the one that renders the frame, and a disagreement would look exactly like a
/// correct curve. So the page asks, and what it draws is what `Property::value_at` says.
///
/// This rides on the frame scheme beside `boxes` and for the same reason: it is a question about
/// what is on screen, whose answer changes nothing and needs no undo record. Unlike `boxes` it
/// costs no render - a property is evaluated without planning a frame.
fn curve(viewer: &Mutex<Viewer>, query: Option<&str>) -> Response<Vec<u8>> {
    let viewer = viewer.lock().expect("the viewer lock was poisoned");
    let number = |name: &str, fallback: i32| {
        parameter(query, name)
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(fallback)
    };
    let answer = (|| {
        let comp = viewer.document.project().composition(&viewer.composition)?;
        let name = parameter(query, "prop")?;
        // B-19d: an effect's setting, as the frame is rendered with it: D-68's value held
        // inside the setting's range.
        if let Some((instance, setting)) = effect_setting(&name) {
            let layer = comp.layer(&Id::new(&parameter(query, "layer")?))?;
            let fx = layer.effects.iter().find(|e| e.instance_id == instance)?;
            let (from, to) = (number("from", 0), number("to", 0));
            let to = to.clamp(from, from.saturating_add(10_000));
            let samples: Vec<serde_json::Value> = (from..=to)
                .map(|f| serde_json::json!(fx.at_time(f, layer.key_time(f as f64)).get(&setting)))
                .collect();
            return Some(serde_json::json!({ "from": from, "to": to, "samples": samples }));
        }
        // D-298: a mask's number, as the frame is drawn with it.
        if let Some((at, which)) = mask_prop(&name) {
            let layer = comp.layer(&Id::new(&parameter(query, "layer")?))?;
            let m = layer.masks.get(at)?;
            let (from, to) = (number("from", 0), number("to", 0));
            let to = to.clamp(from, from.saturating_add(10_000));
            let samples: Vec<serde_json::Value> = (from..=to)
                .map(|f| {
                    let v = m.at_time(layer.key_time(f as f64)).channel(which)?;
                    Some(serde_json::json!([v.base().as_scalar()?]))
                })
                .collect::<Option<_>>()?;
            return Some(serde_json::json!({ "from": from, "to": to, "samples": samples }));
        }
        // B-24f: a path is not one number, so its graph is how far its points have gone: the
        // mean distance between two keys' points, added up key after key, and part of the way
        // along a segment by where `points_at` puts the path. Its speed graph is then pixels a
        // second, which is what After Effects' graph shows of a path.
        // ponytail: an ease that overshoots reads as distance on the way back; signed travel if
        // that ever misleads.
        if let Some((shape, at)) = path_prop(&name) {
            let layer = comp.layer(&Id::new(&parameter(query, "layer")?))?;
            let (base, keys) = if shape {
                let s = layer.shapes.get(at)?;
                (&s.points, &s.keys)
            } else {
                let m = layer.masks.get(at)?;
                (&m.points, &m.keys)
            };
            use anime_compositor::mask::MaskPoint;
            let apart = |p: &[MaskPoint], q: &[MaskPoint]| {
                p.iter()
                    .zip(q)
                    .map(|(a, b)| (a.point.0 - b.point.0).hypot(a.point.1 - b.point.1))
                    .sum::<f64>()
                    / p.len().max(1) as f64
            };
            // B-150c: read at the frame's key time, as the picture is.
            let travelled = |f: i32| {
                let u = layer.key_time(f as f64);
                let i = keys.partition_point(|k| k.frame as f64 <= u);
                if i == 0 {
                    return 0.0;
                }
                let before: f64 = keys[..i]
                    .windows(2)
                    .map(|w| apart(&w[0].points, &w[1].points))
                    .sum();
                before
                    + apart(
                        &keys[i - 1].points,
                        &anime_compositor::mask::points_at_time(base, keys, u),
                    )
            };
            let (from, to) = (number("from", 0), number("to", 0));
            let to = to.clamp(from, from.saturating_add(10_000));
            let samples: Vec<serde_json::Value> = (from..=to)
                .map(|f| serde_json::json!([travelled(f)]))
                .collect();
            return Some(serde_json::json!({ "from": from, "to": to, "samples": samples }));
        }
        // D-22: a scale is a percentage in the panels and a lens is millimetres, so the graph is
        // drawn in the numbers the inspector beside it shows.
        // B-150c: a layer's keys are read at its key time, the camera's at the frame itself.
        let mut when: Option<&Layer> = None;
        let (held, factor) = if parameter(query, "target").as_deref() == Some("camera") {
            // B-13e's camera, at the default the renderer uses when the file has none.
            use anime_compositor::model::{Camera, CameraProp};
            let which = CameraProp::from_str(&name)?;
            let camera = comp
                .camera
                .clone()
                .unwrap_or_else(|| Camera::default_for(comp.width, comp.height));
            let factor = if which == CameraProp::Zoom {
                36.0 / comp.width as f64
            } else {
                1.0
            };
            (camera.get(which).clone(), factor)
        } else {
            let prop = property(&name)?;
            let layer = comp.layer(&Id::new(&parameter(query, "layer")?))?;
            when = Some(layer);
            // B-13d: looked up once rather than per frame, because the lookup that reaches a
            // depth as well as the five hands back something owned.
            (
                property_of(layer, prop),
                if prop == Prop::Scale { 100.0 } else { 1.0 },
            )
        };
        let (from, to) = (number("from", 0), number("to", 0));
        // A range wider than the longest composition the envelope allows is a page with a bug,
        // not a person with a problem, so it is cut short rather than refused.
        let to = to.clamp(from, from.saturating_add(10_000));
        let samples: Vec<serde_json::Value> = (from..=to)
            .map(
                |f| match held.value_at_time(when.map_or(f as f64, |l| l.key_time(f as f64))) {
                    Value::Scalar(v) => serde_json::json!([v * factor]),
                    Value::Vec2(x, y) => serde_json::json!([x * factor, y * factor]),
                },
            )
            .collect();
        Some(serde_json::json!({ "from": from, "to": to, "samples": samples }))
    })();
    // A layer or property that is not there is a page asking about something the person has just
    // deleted. No samples is the truthful answer to that, and the graph draws nothing.
    let body = answer.unwrap_or_else(|| serde_json::json!({ "from": 0, "to": 0, "samples": [] }));
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "application/json; charset=utf-8")
        .body(body.to_string().into_bytes())
        .expect("build the curve response")
}

/// D-84a: the composition on screen as a paper timesheet, for the Sheet tab.
///
/// A column for each layer that shows an image sequence, the bottom of the stack first because a
/// paper sheet has it on the left, and a cell for each frame holding what the paper would: the
/// drawing's number where it is first exposed, `|` where it holds, `x` where a blank run starts
/// and `""` while it lasts, and `null` outside the layer's in and out. Every cell is the core's
/// own `drawing_at` after `local_frame`, the two the picture is drawn from, so the sheet cannot
/// disagree with the picture. Like `curve` it is a question whose answer changes nothing.
///
/// D-84c: `text` holds the dialogue and camera columns the cut was imported with, in the order
/// kept, a cell for each frame of the sheet: the entry's strings joined by a space where it
/// starts, `|` while it lasts, and `""` where there is none. D-84g: the Action column is always
/// among them, and each drawing column says its `keys`.
fn sheet(viewer: &Mutex<Viewer>) -> Response<Vec<u8>> {
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "application/json; charset=utf-8")
        .body(sheet_grid(viewer).to_string().into_bytes())
        .expect("build the sheet response")
}

/// W-40c (D-249): `/ready`, which frames of the composition on screen the frame memory
/// (B-154) holds for exactly what is on screen now: the project as shown, the quality and the
/// processor a frame asked for now would be made with. The timeline draws them as a green line.
/// `held` is the bytes the memory holds in all, `setting` the Preferences memory setting. A
/// question whose answer changes nothing, like `sheet`.
fn ready_frames(viewer: &Mutex<Viewer>, card: Option<&Mutex<Card>>) -> serde_json::Value {
    let (mut sight, ram) = {
        let v = viewer.lock().expect("the viewer lock was poisoned");
        let sight = Sight {
            project: shown(&v),
            composition: v.composition.clone(),
            root: v.root.clone(),
            quality: v.quality,
            card: None,
        };
        (sight, Arc::clone(&v.ram))
    };
    sight.card = card.and_then(|c| drawn_by(Some(&*c.lock().expect("the card lock was poisoned"))));
    let ram = ram.lock().expect("the frame memory lock was poisoned");
    serde_json::json!({ "frames": ram.frames.kept(&sight), "held": ram.frames.held_bytes(), "setting": ram_ceiling() })
}

/// D-84d: `/sheet/print?seconds=&colour=`, the Sheet as paper for the system's print dialog.
/// The window writes the whole page, so what prints is the Sheet's own cells and a table can read
/// the pages. D-84e: 6 seconds a page unless asked for 3, black unless asked for red.
fn sheet_print(viewer: &Mutex<Viewer>, query: Option<&str>) -> Response<Vec<u8>> {
    let seconds = parameter(query, "seconds").map_or(6, |s| s.parse().unwrap_or(0));
    let red = parameter(query, "colour").as_deref() == Some("red");
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "text/html; charset=utf-8")
        .body(sheet_paper(&sheet_grid(viewer), seconds, red).into_bytes())
        .expect("build the printable sheet")
}

/// The printable page for a grid from `sheet_grid`. A page holds six seconds as two halves of
/// three, left then right, as a Japanese A4 timesheet does, or three seconds in one; rows past
/// the cut's end are empty and a heavy line goes under its last frame. D-84e: frames are counted
/// from 1 on paper, whatever the Sheet on screen starts at, with each second's number beside its
/// last row. A grid with no drawing columns, or a page of other than 6 or 3 seconds, has no pages.
fn sheet_paper(grid: &serde_json::Value, seconds: usize, red: bool) -> String {
    let escape = |text: &str| {
        text.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
    };
    let empty = Vec::new();
    let list = |key: &str| grid[key].as_array().unwrap_or(&empty).clone();
    let drawings = list("columns");
    let name = escape(grid["name"].as_str().unwrap_or(""));
    let mut html = format!(
        "<!DOCTYPE html>\n<html><head><meta charset=\"utf-8\"><title>{name}</title><style>\n{}\n</style></head><body><div id=\"paper\"{}>\n",
        sheet_paper_style(),
        if red { " class=\"red\"" } else { "" }
    );
    if drawings.is_empty() {
        html.push_str("<p>There is nothing to print: no layer here shows drawings.</p>\n</div></body></html>\n");
        return html;
    }
    if seconds != 6 && seconds != 3 {
        html.push_str("<p>A page holds 6 or 3 seconds.</p>\n</div></body></html>\n");
        return html;
    }
    let text = list("text");
    // D-84e: as a studio's sheet: Action, then dialogue, the drawings in at least six cel
    // columns, then camera and anything else. D-84g: the grid always has an Action column.
    let action = serde_json::json!({ "name": "Action" });
    let cel = serde_json::json!({ "name": "" });
    let mut columns: Vec<(&serde_json::Value, bool)> = vec![(
        text.iter()
            .find(|c| c["kind"] == "action")
            .unwrap_or(&action),
        true,
    )];
    columns.extend(
        text.iter()
            .filter(|c| c["kind"] == "dialogue")
            .map(|c| (c, true)),
    );
    columns.extend(drawings.iter().map(|c| (c, false)));
    columns.extend((drawings.len()..6).map(|_| (&cel, false)));
    columns.extend(
        text.iter()
            .filter(|c| c["kind"] != "dialogue" && c["kind"] != "action")
            .map(|c| (c, true)),
    );
    let rate = &grid["frame_rate"];
    let fps = (rate["numerator"].as_f64().unwrap_or(24.0)
        / rate["denominator"].as_f64().unwrap_or(1.0))
    .round()
    .max(1.0) as usize;
    let duration = grid["duration"].as_u64().unwrap_or(0) as usize;
    let half = 3 * fps;
    let halves = seconds / 3;
    let pages = duration.div_ceil(halves * half).max(1);
    let length = format!("{} + {}", duration / fps, duration % fps);
    // D-84f: the title block's top row, the name and then the four Composition Settings writes.
    let detail = |key: &str| escape(grid["details"][key].as_str().unwrap_or(""));
    let top = format!(
        "<b class=\"name\">{name}</b><span class=\"episode\">{}</span><span class=\"scene\">{}</span><span class=\"cut\">{}</span><span class=\"animator\">{}</span>",
        detail("episode"),
        detail("scene"),
        detail("cut"),
        detail("animator")
    );
    for page in 0..pages {
        html.push_str(&format!(
            "<section class=\"page\"><header>{top}<span class=\"time\">{length}</span><span class=\"rate\">{fps} fps</span><span class=\"sheet\">Sheet {} of {pages}</span><span class=\"memo\"></span></header><div class=\"halves\">\n",
            page + 1
        ));
        for side in 0..halves {
            let first = (page * halves + side) * half;
            html.push_str(&format!(
                "<table data-first=\"{}\" data-last=\"{}\"><tr><th class=\"sec\">sec</th><th class=\"frame\">frame</th>",
                first + 1,
                first + half
            ));
            for (c, words) in &columns {
                let class = if *words { " class=\"words\"" } else { "" };
                html.push_str(&format!(
                    "<th{class}>{}</th>",
                    escape(c["name"].as_str().unwrap_or(""))
                ));
            }
            html.push_str("</tr>\n");
            for i in first..first + half {
                let mut class = Vec::new();
                if (i + 1) % fps == 0 {
                    class.push("second");
                } else if fps % 2 == 0 && (i + 1) % (fps / 2) == 0 {
                    class.push("half");
                }
                if i + 1 == duration {
                    class.push("end");
                }
                if i >= duration {
                    class.push("past");
                }
                let second = if (i + 1) % fps == 0 {
                    ((i + 1) / fps).to_string()
                } else {
                    String::new()
                };
                html.push_str(&format!(
                    "<tr data-row=\"{}\" class=\"{}\"><td class=\"sec\">{second}</td><td class=\"frame\">{}</td>",
                    i + 1,
                    class.join(" "),
                    i + 1
                ));
                for (c, words) in &columns {
                    let cell = if i < duration {
                        c["cells"].get(i)
                    } else {
                        None
                    };
                    html.push_str(&match (cell.map(|v| v.as_str()), words) {
                        (None, _) | (Some(Some("")), _) => "<td></td>".to_string(),
                        (Some(None), _) => "<td class=\"out\"></td>".to_string(),
                        (Some(Some("|")), _) => "<td class=\"hold\"></td>".to_string(),
                        (Some(Some("x")), false) => "<td>\u{d7}</td>".to_string(),
                        (Some(Some(mark)), true) => {
                            format!("<td class=\"words\">{}</td>", escape(mark))
                        }
                        // D-84g: a key drawing's number is circled wherever it is written.
                        (Some(Some(mark)), false) => {
                            let key = c["keys"].as_array().is_some_and(|keys| {
                                keys.iter().any(|n| {
                                    n.as_u64().map(|n| n.to_string()).as_deref() == Some(mark)
                                })
                            });
                            format!(
                                "<td{}>{}</td>",
                                if key { " class=\"key\"" } else { "" },
                                escape(mark)
                            )
                        }
                    });
                }
                html.push_str("</tr>\n");
            }
            html.push_str("</table>\n");
        }
        html.push_str("</div></section>\n");
    }
    html.push_str("</div></body></html>\n");
    html
}

/// The paper's look, copied from the page's own style between its `sheet paper` marks. The
/// window's content security policy refuses a style written into a page at run time, so the
/// window prints from its own page; the copy is for the printable page opened on its own.
fn sheet_paper_style() -> &'static str {
    let page = include_str!("../ui/index.html");
    let from = page
        .find("/* sheet paper: begin")
        .expect("the page has the sheet paper style");
    let to = page
        .find("/* sheet paper: end */")
        .expect("the sheet paper style ends");
    &page[from..to]
}

/// The Sheet's grid: `sheet` answers it to the page and `sheet_paper` prints it.
fn sheet_grid(viewer: &Mutex<Viewer>) -> serde_json::Value {
    use anime_compositor::time::ExposureMap;
    let viewer = viewer.lock().expect("the viewer lock was poisoned");
    let project = viewer.document.project();
    match project.composition(&viewer.composition) {
        None => serde_json::json!({ "from": 0, "columns": [], "text": [] }),
        Some(comp) => {
            let frames = comp.start_frame..comp.start_frame + comp.duration_frames as i32;
            let columns: Vec<serde_json::Value> = comp
                .layers_in_order()
                .filter(|l| {
                    project
                        .assets
                        .iter()
                        .any(|a| a.id == l.asset_id && a.kind == AssetKind::ImageSequence)
                })
                .map(|l| {
                    let timing = l.timing();
                    // Spans the core refuses are refused when the project is opened, so a map
                    // that cannot be made here is one no open project has.
                    let map = ExposureMap::new(l.exposure_spans.clone()).ok();
                    // The cell before: `None` outside the layer, `Some(None)` blank.
                    let mut before: Option<Option<u32>> = None;
                    let cells: Vec<serde_json::Value> = frames
                        .clone()
                        .map(|f| {
                            let now = timing
                                .local_frame(f)
                                .map(|local| map.as_ref().and_then(|m| m.drawing_at(local)));
                            let cell = match (now, before) {
                                (None, _) => serde_json::Value::Null,
                                (Some(n), Some(b)) if n == b => {
                                    serde_json::json!(if n.is_some() { "|" } else { "" })
                                }
                                (Some(Some(n)), _) => serde_json::json!(n.to_string()),
                                (Some(None), _) => serde_json::json!("x"),
                            };
                            before = now;
                            cell
                        })
                        .collect();
                    serde_json::json!({ "layer": l.id.as_str(), "name": l.name, "cells": cells,
                        "keys": l.key_drawings })
                })
                .collect();
            // D-84g: the Action column is on every Sheet with drawings, empty until written in.
            let action = anime_compositor::model::SheetText {
                kind: "action".into(),
                name: "Action".into(),
                track: 0,
                entries: Vec::new(),
            };
            let unwritten =
                !columns.is_empty() && !comp.sheet_text.iter().any(|c| c.kind == "action");
            let text: Vec<serde_json::Value> = unwritten
                .then_some(&action)
                .into_iter()
                .chain(&comp.sheet_text)
                .map(|column| {
                    let mut cells = vec![String::new(); comp.duration_frames as usize];
                    for e in &column.entries {
                        for f in e.start_frame.max(0)..e.end_frame_exclusive.min(cells.len() as i32)
                        {
                            cells[f as usize] = if f == e.start_frame {
                                e.text.join(" ")
                            } else {
                                "|".into()
                            };
                        }
                    }
                    serde_json::json!({ "kind": column.kind, "name": column.name, "cells": cells })
                })
                .collect();
            serde_json::json!({ "from": comp.start_frame, "columns": columns, "text": text,
                "name": comp.name, "details": { "episode": comp.sheet_details.episode,
                    "scene": comp.sheet_details.scene, "cut": comp.sheet_details.cut,
                    "animator": comp.sheet_details.animator },
                "duration": comp.duration_frames, "frame_rate": {
                    "numerator": comp.frame_rate.numerator(), "denominator": comp.frame_rate.denominator() } })
        }
    }
}

fn boxes(viewer: &Mutex<Viewer>, frame: i32, quality: Option<PreviewQuality>) -> Response<Vec<u8>> {
    let taken = {
        let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
        if let Some(quality) = quality {
            viewer.quality = quality;
        }
        Snapshot {
            project: shown(viewer),
            composition: viewer.composition.clone(),
            root: viewer.root.clone(),
            quality: viewer.quality,
            alpha_only: viewer.alpha_only,
            cache: Arc::clone(&viewer.cache),
            ram: Arc::clone(&viewer.ram),
            frame,
            skipped: 0,
            ahead: None,
            dragging: false,
            reply: Response::builder(),
        }
    };
    let mut log = FrameLog::new(0);
    // B-18b: planned at the viewer's quality, so that a composition layer's picture (D-67) is
    // not rendered at full size to outline a draft frame.
    // P-25: and without the masks that cannot change an outline, which with a feather were a
    // whole mask drawn again for each outline asked for in a mask's drag.
    let plan = match anime_compositor::compose::plan_preview(
        &taken.project.for_outlines(&taken.composition),
        &taken.composition,
        taken.frame,
        &taken.root,
        taken.quality,
        &mut log,
        &mut taken.cache.lock().expect("the cel cache lock was poisoned"),
        false,
    ) {
        Ok(plan) => plan,
        // The frame's own request will have said why in words. A selection outline is not the
        // place to say it a second time, so this answers with no boxes rather than an error.
        Err(_) => anime_compositor::render::FramePlan {
            float: false,
            width: 0,
            height: 0,
            layers: Vec::new(),
        },
    };
    let boxed = |id: &str, [l, t, r, b]: [f64; 4], map: &dyn Fn(f64, f64) -> (f64, f64)| {
        let corners: Vec<f64> = [(l, t), (r, t), (r, b), (l, b)]
            .iter()
            .flat_map(|&(x, y)| {
                let (x, y) = map(x, y);
                [x, y]
            })
            .collect();
        serde_json::json!({ "layer": id, "corners": corners })
    };
    let outline = |id: &str, w: f64, h: f64, map: &dyn Fn(f64, f64) -> (f64, f64)| {
        boxed(id, [0.0, 0.0, w, h], map)
    };
    let comp_now = taken.project.composition(&taken.composition);
    let mut found: Vec<serde_json::Value> = plan
        .layers
        .iter()
        .map(|layer| {
            let (w, h) = (layer.source.width() as f64, layer.source.height() as f64);
            // D-264: a text layer's source is the whole composition; its outline, and so what a
            // press on the picture picks, is its words' box. An effect that grew the source
            // (a blur's margin) keeps the whole source, whose corners are then not the words'.
            // The source is drawn at the composition's size at every quality; a draft's smaller
            // frame is in the layer's transform, so the words' box goes through it as it is.
            let words = comp_now.and_then(|c| {
                let text = c.layer(&layer.id)?.text.as_ref()?;
                let grown = w != c.width as f64 || h != c.height as f64;
                // Empty words have no width, and are outlined and picked by the whole layer.
                anime_compositor::text::bounds(text)
                    .filter(|[l, t, r, b]| !grown && r - l >= 1.0 && b - t >= 1.0)
            });
            // `words` is the same box in the layer's own (composition) pixels, so that the page
            // can still carry a mask point or a free transform between the layer and the screen.
            match words {
                Some(own) => {
                    let mut found =
                        boxed(layer.id.as_str(), own, &|x, y| layer.transform.apply(x, y));
                    found["words"] = serde_json::json!(own);
                    found
                }
                None => outline(layer.id.as_str(), w, h, &|x, y| layer.transform.apply(x, y)),
            }
        })
        .collect();
    // B-26c: a null (D-82) is in no plan, because it is never drawn. Its outline is the 100 by
    // 100 square in its own space, carried to the screen as a drawn layer's would be, and it is
    // put among the others in the composition's order so a press on the picture finds the one
    // on top. A null that is switched off or outside its frames has no outline, as a drawing
    // would have none.
    if let Some(comp) = taken.project.composition(&taken.composition) {
        let s = 1.0 / taken.quality.divisor() as f64;
        for (at, layer) in comp.layers_in_order().enumerate() {
            let on = layer.enabled && layer.in_frame <= frame && frame < layer.out_frame;
            if layer.kind != LayerKind::Null || !on {
                continue;
            }
            let Some(map) = anime_compositor::compose::screen_transform(comp, &layer.id, frame)
            else {
                continue;
            };
            let below: std::collections::HashSet<&str> = comp
                .layers_in_order()
                .take(at)
                .map(|l| l.id.as_str())
                .collect();
            let index = found
                .iter()
                .rposition(|b| b["layer"].as_str().is_some_and(|id| below.contains(id)))
                .map_or(0, |i| i + 1);
            let null = outline(layer.id.as_str(), 100.0, 100.0, &|x, y| {
                let (x, y) = map.apply(x, y);
                (x * s, y * s)
            });
            found.insert(index, null);
        }
    }
    // W-10: what each layer's five properties are on this frame, in the units the file holds,
    // so that the inspector can show a keyframed property's value under the playhead rather
    // than a base value nothing is drawn from. Every layer of the composition is here, not only
    // the ones the plan drew: a layer outside its own life still has a panel.
    //
    // B-14c: a property with an expression is its value after the expression, which is what is
    // drawn, and one whose expression fails is its keyed value with the reason in `errors`,
    // under the same two names, so the panel can say it beside the property.
    let mut errors = serde_json::Map::new();
    let values: serde_json::Map<String, serde_json::Value> = taken
        .project
        .composition(&taken.composition)
        .map(|comp| {
            use anime_compositor::expr;
            let mut failed = |row: &str, prop: &str, e: Option<expr::ExprError>| {
                if let Some(e) = e {
                    errors
                        .entry(row.to_string())
                        .or_insert_with(|| serde_json::json!({}))[prop] =
                        serde_json::json!(format!("{}: {}.", e.id.as_str(), e.message));
                }
            };
            let mut values: serde_json::Map<String, serde_json::Value> = comp
                .layers_in_order()
                .map(|layer| {
                    let target = || expr::Target::Layer(layer.id.clone());
                    // D-216: a stretched layer's keys play at its key time, not at the frame.
                    let t = layer.key_time(frame as f64);
                    let at: serde_json::Map<String, serde_json::Value> = layer
                        .transform
                        .value_at(frame)
                        .into_iter()
                        .map(|(prop, _)| {
                            let property = layer.transform.get(prop).expect("one of the five");
                            // B-19g: `resolve` costs a read of the keys where there is no
                            // expression, and knows a separated position's halves may have one.
                            // D-742: at the key time, as the picture reads them.
                            let (value, e) =
                                expr::resolve_at(comp, property, target, prop, frame, t);
                            failed(layer.id.as_str(), prop.as_str(), e);
                            // D-22: the file holds a scale as a percentage, and a whole
                            // number as a whole number, so the panel reads 100 and not 100.0.
                            let factor = if prop == Prop::Scale { 100.0 } else { 1.0 };
                            let num = |v: f64| match v * factor {
                                w if w.fract() == 0.0 && w.abs() < 1e15 => {
                                    serde_json::json!(w as i64)
                                }
                                w => serde_json::json!(w),
                            };
                            let value = match value {
                                Value::Scalar(v) => num(v),
                                Value::Vec2(x, y) => serde_json::json!([num(x), num(y)]),
                            };
                            (prop.as_str().to_string(), value)
                        })
                        .collect();
                    // B-13d: and the depth, for the reason the five are here. A depth that is
                    // keyed is not its base on any frame but the first, and the inspector would
                    // otherwise have gone on showing the number the animation started from.
                    let mut at = at;
                    // B-19g: D-69's two halves under their own names, for their own rows.
                    if layer.transform.position.split().is_some() {
                        if let Some(pair) = at.get("position").and_then(|p| p.as_array()).cloned() {
                            at.insert("position_x".to_string(), pair[0].clone());
                            at.insert("position_y".to_string(), pair[1].clone());
                        }
                    }
                    let plane = layer.depth.as_ref().map_or(0.0, |d| {
                        let (v, e) = expr::resolve_at(comp, d, target, Prop::Depth, frame, t);
                        failed(layer.id.as_str(), Prop::Depth.as_str(), e);
                        v.as_scalar().unwrap_or(0.0)
                    });
                    at.insert(
                        "depth".to_string(),
                        match plane {
                            w if w.fract() == 0.0 && w.abs() < 1e15 => {
                                serde_json::json!(w as i64)
                            }
                            w => serde_json::json!(w),
                        },
                    );
                    // D-323: and the Time Remap, read at the key time as it is drawn.
                    if let Some(remap) = &layer.time_remap {
                        let v = remap.value_at_time(t);
                        at.insert("time_remap".to_string(), serde_json::json!(v.as_scalar()));
                    }
                    // B-19d: and every keyed setting of its effects, under the name the page
                    // gives it, for the reason the depth is here. D-291: and every one with an
                    // expression, after it, with the reason beside one that fails.
                    for fx in layer.effects.iter().filter(|fx| !fx.tracks.is_empty()) {
                        let (now, failures) = expr::effect_at(comp, &layer.id, fx, frame, t);
                        for (name, e) in failures {
                            let prop = format!("fx:{}:{name}", fx.instance_id);
                            failed(layer.id.as_str(), &prop, Some(e));
                        }
                        for name in fx.tracks.keys() {
                            if let Some(v) = now.get(name) {
                                at.insert(
                                    format!("fx:{}:{name}", fx.instance_id),
                                    serde_json::json!(v),
                                );
                            }
                        }
                    }
                    // D-298: and every keyed number of a mask, for the same reason.
                    for (i, m) in layer
                        .masks
                        .iter()
                        .enumerate()
                        .filter(|(_, m)| !m.tracks.is_empty())
                    {
                        let now = m.at_time(t);
                        for name in m.tracks.keys() {
                            let v = now.channel(name).and_then(|p| p.base().as_scalar());
                            at.insert(format!("mask:{i}:{name}"), serde_json::json!(v));
                        }
                    }
                    // B-24d: and the shape of every mask on this frame, six numbers a point in
                    // the layer's own pixels, in the order the masks were drawn. A keyed path is
                    // not its base on any frame but a key's, and the page has no business
                    // solving document 20's curves a second time: the picture it draws points
                    // on, and the points a drag starts from, are the ones the renderer used.
                    // B-25c: and a shape layer's shapes, for the same reason and in the same six.
                    let six = |points: Vec<anime_compositor::mask::MaskPoint>| {
                        points
                            .iter()
                            .map(|p| {
                                [
                                    p.point.0,
                                    p.point.1,
                                    p.in_handle.0,
                                    p.in_handle.1,
                                    p.out_handle.0,
                                    p.out_handle.1,
                                ]
                            })
                            .collect::<Vec<_>>()
                    };
                    if !layer.masks.is_empty() {
                        at.insert(
                            "masks".to_string(),
                            serde_json::json!(layer
                                .masks
                                .iter()
                                .map(|m| {
                                    six(anime_compositor::mask::points_at_time(
                                        &m.points, &m.keys, t,
                                    ))
                                })
                                .collect::<Vec<_>>()),
                        );
                    }
                    if !layer.shapes.is_empty() {
                        at.insert(
                            "shapes".to_string(),
                            serde_json::json!(layer
                                .shapes
                                .iter()
                                .map(|s| six(s.placed(&layer.shape_groups, frame).points))
                                .collect::<Vec<_>>()),
                        );
                    }
                    // B-108b, B-109b: and every gradient point and trim number, under the name the
                    // page gives it, for the reason the depth is here.
                    for (i, shape) in layer.shapes.iter().enumerate() {
                        for name in anime_compositor::shape::SHAPE_PROPERTIES {
                            let Some(channels) = shape.channels(name) else {
                                continue;
                            };
                            let now: Vec<serde_json::Value> = channels
                                .iter()
                                .map(|p| match p.value_at(frame) {
                                    Value::Vec2(x, y) => serde_json::json!([x, y]),
                                    Value::Scalar(n) => serde_json::json!(n),
                                })
                                .collect();
                            // D-170: a colour is its three numbers, anything else its one.
                            let now = match now.len() {
                                1 => now[0].clone(),
                                _ => serde_json::Value::Array(now),
                            };
                            at.insert(format!("shape:{i}:{name}"), now);
                        }
                    }
                    // D-738 (b): and every group's seven, as the file holds them.
                    for (i, group) in layer.shape_groups.iter().enumerate() {
                        for (name, p) in &group.transform {
                            let now = match p.value_at(frame) {
                                Value::Vec2(x, y) => serde_json::json!([x, y]),
                                Value::Scalar(n) => serde_json::json!(n),
                            };
                            at.insert(format!("group:{i}:{name}"), now);
                        }
                        // D-738 (c): and its repeater's eight, `group:<n>:repeater_copies` and on.
                        let numbers = group.repeater.iter().flat_map(|r| &r.numbers);
                        for (name, p) in numbers {
                            let now = match p.value_at(frame) {
                                Value::Vec2(x, y) => serde_json::json!([x, y]),
                                Value::Scalar(n) => serde_json::json!(n),
                            };
                            at.insert(format!("group:{i}:repeater_{name}"), now);
                        }
                        // D-738 (d): and each operation's, `group:<n>:op<k>_radius` and on.
                        for (k, o) in group.operations.iter().enumerate() {
                            for (name, p) in &o.numbers {
                                let now = match p.value_at(frame) {
                                    Value::Vec2(x, y) => serde_json::json!([x, y]),
                                    Value::Scalar(n) => serde_json::json!(n),
                                };
                                at.insert(format!("group:{i}:op{k}_{name}"), now);
                            }
                        }
                    }
                    (layer.id.as_str().to_string(), serde_json::Value::Object(at))
                })
                .collect();
            // B-13e: and the camera's three, under the window's own name for it, because the
            // rows that draw them are the rows that draw a layer's and read this same answer.
            // W-10's rule is what makes it necessary: a keyed camera is not its base on any
            // frame but the first, and the boxes would have gone on showing the number the
            // move started from.
            //
            // In the units the file holds, as every other value here is. The lens is the one
            // a person reads in millimetres, and the page turns it over at the place it draws
            // it, exactly as it already does for an opacity the file keeps as nought to one.
            let camera = comp.camera.clone().unwrap_or_else(|| {
                anime_compositor::model::Camera::default_for(comp.width, comp.height)
            });
            let nice = |v: f64| match v {
                w if w.fract() == 0.0 && w.abs() < 1e15 => serde_json::json!(w as i64),
                w => serde_json::json!(w),
            };
            let at: serde_json::Map<String, serde_json::Value> = [
                anime_compositor::model::CameraProp::Position,
                anime_compositor::model::CameraProp::Depth,
                anime_compositor::model::CameraProp::Zoom,
            ]
            .into_iter()
            .map(|which| {
                let prop = prop_of_camera(which);
                let (value, e) = expr::resolve(
                    comp,
                    camera.get(which),
                    || expr::Target::Camera,
                    prop,
                    frame,
                );
                failed(CAMERA_ROW, prop.as_str(), e);
                let value = match value {
                    Value::Scalar(v) => nice(v),
                    Value::Vec2(x, y) => serde_json::json!([nice(x), nice(y)]),
                };
                (which.as_str().to_string(), value)
            })
            .collect();
            values.insert(CAMERA_ROW.to_string(), serde_json::Value::Object(at));
            values
        })
        .unwrap_or_default();
    allow_the_page_to_read_this(taken.reply)
        .header("content-type", "application/json; charset=utf-8")
        .body(
            serde_json::json!({
                "frame": frame,
                "layers": found,
                "values": values,
                "errors": errors,
            })
            .to_string()
            .into_bytes(),
        )
        .expect("build the boxes response")
}

/// Percent-encode a string so it can travel in an HTTP header and arrive unharmed.
///
/// Header values are bytes, and a project called `背景_日本語` or a diagnostic quoting a path
/// with a Japanese directory in it is not ASCII. Sending those bytes raw would either be
/// rejected or arrive as mojibake, and mojibake in a *diagnostic* is worse than no diagnostic:
/// the person is told the wrong filename. The page reads these with `decodeURIComponent`.
///
/// Everything outside printable ASCII is encoded, and so is `%` itself, so decoding is exact.
fn for_a_header(text: &str) -> String {
    let mut encoded = String::with_capacity(text.len());
    for byte in text.bytes() {
        match byte {
            b'%' | 0x00..=0x1f | 0x7f..=0xff => {
                encoded.push_str(&format!("%{byte:02X}"));
            }
            _ => encoded.push(byte as char),
        }
    }
    encoded
}

/// Let the page read the headers, not only receive them.
///
/// The webview treats `http://frame.localhost` as a different origin from the page, so a
/// response without both of these is fetched successfully and then withheld: the body fails
/// CORS and every `x-` header reads as absent. Both are needed; either alone is silence.
fn allow_the_page_to_read_this(
    response: tauri::http::response::Builder,
) -> tauri::http::response::Builder {
    response
        .header("access-control-allow-origin", "*")
        .header("access-control-expose-headers", "*")
}

/// Replace a frame's colour with its own alpha, in place, so the page draws the alpha channel.
///
/// Document 05 line 31 asks for an alpha-only view and document 21 line 97 requires it to be
/// presentation: this runs on the bytes that are about to be sent, which
/// `WorkingBuffer::to_srgb8_straight` has just allocated, so the cached frame and the export path
/// never see it. Grey rather than false colour, and opaque rather than transparent, because the
/// question the view answers is "how transparent is this pixel" and an answer that is itself
/// transparent cannot be looked at.
fn as_alpha_only(pixels: &mut [u8]) {
    for pixel in pixels.chunks_exact_mut(4) {
        pixel[0] = pixel[3];
        pixel[1] = pixel[3];
        pixel[2] = pixel[3];
        pixel[3] = 255;
    }
}

/// B-48, D-105: what the viewer's picture is drawn on. The page keeps the choice between launches
/// and says it when it opens; until it has, the CPU.
#[derive(Clone, Copy, PartialEq, Default, Debug)]
enum DrawOn {
    /// The card whenever it can: a frame it refuses goes to the CPU on its own, and a card that
    /// fails is given up for the rest of the session, once, with a sentence saying so.
    Auto,
    /// The card, tried on every frame even after it fails.
    Gpu,
    #[default]
    Cpu,
}

/// B-44, D-100 (a): whether the viewer's picture is drawn on the graphics card, and the card.
///
/// The window's, not the project's, like the alpha view, and exports never read it. The card is
/// opened the first time something other than CPU is chosen, so a person who never chooses it
/// pays nothing for it.
#[derive(Default)]
struct Card {
    mode: DrawOn,
    /// Drawing on the card now: the mode is Auto or GPU and the card opened, and Auto has not
    /// given it up.
    on: bool,
    /// Auto gave the card up after it failed (B-48).
    given_up: bool,
    /// D-105: the Custom ceiling for the card's memory, or `None` for Automatic.
    ceiling: Option<usize>,
    gpu: Option<Result<Gpu, String>>,
    /// B-45, D-102: what the page has around and under the picture, in window pixels, and the
    /// window's size, once the page has said. Empty until then, and the picture goes to the page.
    paints: Vec<Paint>,
    size: (u32, u32),
    /// Why the card stopped painting the window, once it has. It does not try again this session.
    off_screen: Option<String>,
}

impl Card {
    /// Choose what the preview is drawn on, and say what happened.
    fn set(&mut self, mode: DrawOn) -> String {
        self.mode = mode;
        self.given_up = false;
        if mode == DrawOn::Cpu {
            if self.on {
                self.on = false;
                // The page lays out again before the card paints the window again.
                self.paints.clear();
            }
            return "The preview is drawn on the CPU.".to_string();
        }
        match self.gpu.get_or_insert_with(Gpu::new) {
            Ok(gpu) => {
                gpu.budget = self
                    .ceiling
                    .map_or(gpu.automatic_budget(), |c| c.min(gpu.largest_budget()));
                self.on = true;
                format!(
                    "The preview is drawn on the graphics card{}: {}. Exports are still drawn on \
                        the CPU. A frame the card cannot draw is drawn on the CPU, and the picture's \
                        label says so.",
                    if mode == DrawOn::Auto { " whenever it can be" } else { "" },
                    gpu.about()
                )
            }
            Err(why) => {
                self.on = false;
                format!("The preview is drawn on the CPU: no usable graphics card, {why}.")
            }
        }
    }

    /// How many frames the card has failed so far.
    fn failures(&self) -> u64 {
        match &self.gpu {
            Some(Ok(gpu)) => gpu.failures(),
            _ => 0,
        }
    }

    /// B-48: Auto gives the card up for the session when it fails a frame, and says so once.
    /// The frame it failed was already drawn on the CPU and labelled so.
    fn give_up_if_failed(&mut self, failures_before: u64) -> Option<String> {
        if self.mode != DrawOn::Auto || !self.on || self.failures() == failures_before {
            return None;
        }
        self.on = false;
        self.given_up = true;
        self.paints.clear();
        Some(
            "The graphics card failed, so Auto draws the preview on the CPU for the rest of this \
                session. The session log says what failed. Choose GPU to try the card again."
                .to_string(),
        )
    }

    /// What the switch's tooltip says: the card, or why there is none, or nothing yet.
    fn about(&self) -> String {
        match &self.gpu {
            None => String::new(),
            Some(Ok(gpu)) if self.given_up => {
                format!("{}; Auto gave it up after it failed", gpu.about())
            }
            Some(Ok(gpu)) => match &self.off_screen {
                None => gpu.about().to_string(),
                Some(why) => format!("{}; not painting the window: {why}", gpu.about()),
            },
            Some(Err(why)) => format!("no usable card: {why}"),
        }
    }
}

/// One frame's worth of viewer, copied out under the lock so that the render can run without it.
///
/// P-04. Everything here is either cheap to copy - a project is layer and asset records, not
/// pixels - or already shared, and the whole point of it is that none of it is borrowed from the
/// viewer: the lock is released the instant this exists.
struct Snapshot {
    project: Project,
    composition: Id,
    root: PathBuf,
    quality: PreviewQuality,
    alpha_only: bool,
    /// The decoded cels, which a render holds for its whole length and no command touches.
    cache: Arc<Mutex<CelCache>>,
    /// B-154: the finished frames.
    ram: Arc<Mutex<Ram>>,
    frame: i32,
    /// P-19: frames the clock passed over to reach this one, for the session log's row.
    skipped: u32,
    /// P-18: the frame playback will ask for next, when this one came from the clock. Its
    /// drawings are read while the page is still busy with this one.
    ahead: Option<i32>,
    /// P-25: a drag is under way. Each of its frames is of a project that lasts one move of the
    /// hand, so none is looked for in the frame memory, read back from the card or kept there.
    dragging: bool,
    /// Everything the window has to say about this frame, already written into the response.
    /// Built under the lock rather than after the render, so that it describes the document the
    /// pixels were made from even if an edit lands while they are being made.
    reply: tauri::http::response::Builder,
}

/// Everything the window says about a frame, as headers, read under the lock that took the
/// snapshot beside it.
fn said_about(
    viewer: &Viewer,
    ask: Ask,
    frame: i32,
    skipped: u32,
    exporting: bool,
    exported: &str,
) -> tauri::http::response::Builder {
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "application/octet-stream")
        .header("x-frame", frame.to_string())
        .header("x-skipped", skipped.to_string())
        .header("x-quality", viewer.quality.label())
        // What the person is looking through. The page needs both: one to draw the grid behind
        // the canvas, and one so the buttons show which view is on.
        // W-02's proposal, if one is waiting to be agreed to. It travels with the frame like
        // everything else the window has to say, so the panel offering it cannot outlive it.
        .header(
            "x-relink",
            for_a_header(match &viewer.relink {
                Some(pending) => &pending.said,
                None => "",
            }),
        )
        .header(
            "x-relink-asset",
            for_a_header(match &viewer.relink {
                Some(pending) => pending.asset.as_str(),
                None => "",
            }),
        )
        .header("x-alpha", viewer.alpha_only.to_string())
        .header("x-checkerboard", viewer.checkerboard.to_string())
        .header(
            "x-differs",
            viewer.quality.differs_from_export().to_string(),
        )
        .header("x-project", for_a_header(&viewer.name))
        .header("x-notes", for_a_header(&viewer.notes.join("\n")))
        .header("x-status", for_a_header(&viewer.status))
        // What the timer last wrote, and what there is to recover. Both here rather than in
        // `x-status` so that neither can take the status line away from a command's answer.
        .header("x-autosaved", for_a_header(&viewer.autosaved))
        // The export, which belongs to the window rather than to the project on screen: it is
        // still running, and still cancellable, after another project has been opened.
        .header("x-exporting", exporting.to_string())
        .header("x-export", for_a_header(exported))
        .header(
            "x-recovery",
            for_a_header(
                &viewer
                    .recovery
                    .iter()
                    .map(|p| p.display().to_string())
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        )
        // Document 26's dirty flag, as the person sees it. It is the core's answer, not a guess
        // by the window: `Document::is_dirty` is false again when the state matches the last
        // successful save, which is the rule a window cannot reimplement correctly.
        .header("x-dirty", viewer.document.is_dirty().to_string())
        .header(
            "x-path",
            for_a_header(
                &viewer
                    .path
                    .as_ref()
                    .map_or(String::new(), |p| p.display().to_string()),
            ),
        )
        // The playback report belongs to playback. A stepped frame did not come from the clock
        // and saying "played 0 frames" beside it would be a sentence about nothing.
        .header(
            "x-report",
            match ask {
                Ask::At(_) => viewer.playback.report(),
                Ask::Frame(_) | Ask::Play(_) => String::new(),
            },
        )
}

/// Render what was asked for and hand it back as raw display-ready pixels.
///
/// The body is `WorkingBuffer::to_srgb8_straight` — the same bytes an 8-bit export writes, minus
/// the PNG container. Encoding a PNG here would cost ten to thirty milliseconds against a frame
/// budget the latency measurement put at eighty-two, to be immediately undone by the browser.
/// The page draws these straight into an `ImageData`.
///
/// Everything the page needs to *say* about the frame travels in headers beside it, so the
/// number on screen and the pixels on screen always came from the same render.
#[cfg(test)]
fn serve(
    viewer: &Mutex<Viewer>,
    export: &Mutex<Export>,
    ask: Ask,
    quality: Option<PreviewQuality>,
) -> Response<Vec<u8>> {
    serve_logged(viewer, export, None, None, ask, quality)
}

/// [`serve`], with the frame written into P-19's session log when its switch is on. Tests only:
/// the window's frames come through [`serve_seen`], which can draw just the part on screen.
#[cfg(test)]
fn serve_logged(
    viewer: &Mutex<Viewer>,
    export: &Mutex<Export>,
    session: Option<&Mutex<SessionLog>>,
    card: Option<&Mutex<Card>>,
    ask: Ask,
    quality: Option<PreviewQuality>,
) -> Response<Vec<u8>> {
    serve_seen(viewer, export, session, card, ask, quality, None)
}

/// B-158 (G8): [`serve_logged`] for the part of the picture on screen. `seen` is the page's
/// `v=left,top,right,bottom`, fractions of the picture across and down; `None`, or all of it,
/// is the whole frame.
fn serve_seen(
    viewer: &Mutex<Viewer>,
    export: &Mutex<Export>,
    session: Option<&Mutex<SessionLog>>,
    card: Option<&Mutex<Card>>,
    ask: Ask,
    quality: Option<PreviewQuality>,
    seen: Option<&str>,
) -> Response<Vec<u8>> {
    // B-158: four finite numbers, and only for a frame asked for by number; anything else is the
    // whole frame. Playback is sent whole.
    let seen: Option<[f64; 4]> = seen.filter(|_| matches!(ask, Ask::Frame(_))).and_then(|v| {
        let n: Vec<f64> = v
            .split(',')
            .map(|x| x.trim().parse().ok().filter(|f: &f64| f.is_finite()))
            .collect::<Option<_>>()?;
        <[f64; 4]>::try_from(n).ok()
    });
    // P-15: how long this whole answer took inside the window, sent back with it. The page
    // subtracts it from its own round trip, and what is left is the transport - the one part of
    // the path from a project file to a picture that no measurement in `verification/` covers,
    // as P-01 and B-08 both say of themselves. One `Instant` on a path that is about to render a
    // frame is not a cost worth a switch.
    let began = std::time::Instant::now();
    // P-19: the switch is read once, so a frame is logged whole or not at all. The stopwatch
    // collects this thread's stages from here, which is everything this frame's own work does.
    let logging =
        session.is_some_and(|s| s.lock().expect("the session log lock was poisoned").is_on());
    if logging {
        anime_compositor::perf::begin_capture();
    }
    let (exporting, exported, progress) = {
        let export = export.lock().expect("the export lock was poisoned");
        // Frames written, frames asked for, and milliseconds since the export began.
        let progress = format!(
            "{} {} {}",
            export.done.load(Ordering::SeqCst),
            export.total,
            export.started.map_or(0, |t| t.elapsed().as_millis())
        );
        (
            export.cancel.is_some(),
            export.said.clone(),
            (progress, export.changes),
        )
    };
    // P-01: how long the page waited for the frame already in flight. The guard outlives this
    // statement, so the duration is recorded rather than wrapped around a closure.
    let waited = std::time::Instant::now();
    // P-04, and document 06's snapshot contract applied to the viewer: everything this frame
    // needs is copied out under the lock, and the lock is given back before any of the work
    // begins. What the window has to say about the frame is written here as well, into the
    // response itself, so the sentence on screen and the pixels on screen still came from one
    // instant of one document - which is what holding the lock across the render used to buy,
    // and is the only thing it bought.
    let taken = {
        let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
        anime_compositor::perf::record(
            anime_compositor::perf::Stage::LockWait,
            waited.elapsed().as_nanos() as u64,
        );
        if let Some(quality) = quality {
            viewer.quality = quality;
        }
        // W-24: the clock plays the work area, so it is rebuilt when the work area has changed.
        if let Some(comp) = viewer.document.project().composition(&viewer.composition) {
            let (first, last) = comp.work_frames();
            if viewer.playback.at_rest() != first
                || viewer.playback.length() as i32 != last - first + 1
            {
                viewer.playback = Playback::new(first, last, comp.frame_rate);
            }
        }
        let (frame, skipped) = match ask {
            Ask::At(ms) => {
                let shown = viewer.playback.at(Duration::from_millis(ms));
                (shown.frame, shown.skipped)
            }
            // Stepping stops at the ends of the composition rather than running off them: a frame
            // outside the composition is not a frame, and the viewer has nowhere to go from
            // there. The composition's ends and not the work area's since W-24, which only
            // bounds playback, as After Effects' does.
            Ask::Frame(n) => {
                let (first, last) = viewer
                    .document
                    .project()
                    .composition(&viewer.composition)
                    .map(|c| (c.start_frame, c.start_frame + c.duration_frames as i32 - 1))
                    .unwrap_or((n, n));
                (n.clamp(first, last), 0)
            }
            // W-09: playback begins at the playhead. The clock is started over from this frame
            // and answers it at once, so the first frame played is the one that was on screen.
            Ask::Play(n) => {
                viewer.playback.start_from(n);
                let shown = viewer.playback.at(Duration::ZERO);
                (shown.frame, shown.skipped)
            }
        };
        let ahead = match ask {
            Ask::At(_) | Ask::Play(_) => Some(viewer.playback.after(frame)),
            Ask::Frame(_) => None,
        };
        Snapshot {
            project: shown(viewer),
            composition: viewer.composition.clone(),
            root: viewer.root.clone(),
            quality: viewer.quality,
            alpha_only: viewer.alpha_only,
            cache: Arc::clone(&viewer.cache),
            ram: Arc::clone(&viewer.ram),
            frame,
            skipped,
            ahead,
            dragging: viewer.document.drag_in_progress(),
            reply: said_about(viewer, ask, frame, skipped, exporting, &exported)
                .header("x-export-progress", progress.0)
                // D-251: the queue's changes and the document's, so an edit marks rows stale.
                .header(
                    "x-queue",
                    format!("{} {}", progress.1, viewer.document.revision()),
                ),
        }
    };

    // B-154: playback's place, for the render-ahead; a frame asked for by number stops it.
    taken
        .ram
        .lock()
        .expect("the frame memory lock was poisoned")
        .playing = matches!(ask, Ask::At(_) | Ask::Play(_)).then_some(taken.frame);
    let mut sight = Sight {
        project: taken.project,
        composition: taken.composition,
        root: taken.root,
        quality: taken.quality,
        card: card.and_then(|c| drawn_by(Some(&*c.lock().expect("the card lock was poisoned")))),
    };
    // A frame already made for this same sight is sent from memory, without waiting for the cels.
    let kept = if taken.dragging {
        None
    } else {
        taken
            .ram
            .lock()
            .expect("the frame memory lock was poisoned")
            .frames
            .get(&sight, taken.frame)
    };

    let mut log = FrameLog::new(3);
    let mut cache = kept
        .is_none()
        .then(|| taken.cache.lock().expect("the cel cache lock was poisoned"));
    if let Some(cels) = cache.as_deref_mut() {
        fit_cels(cels, &taken.ram);
    }
    // P-19: the counters are the cache's own running totals. The lock is held from here until
    // they are read again, so the difference is this frame's and no read-ahead's.
    let counted = cache
        .as_deref()
        .map_or((0, 0, 0), |c| (c.hits(), c.misses(), c.effect_hits()));
    // B-44: on the card when the switch is on. The CPU path below is unchanged, so with the
    // switch off every frame is exactly what it was.
    enum Made {
        Cpu(anime_compositor::WorkingBuffer),
        Gpu(Vec<u8>, usize, usize),
        // B-45: already painted into the window by the card; only its size goes to the page. The
        // pixels are read back for B-154's memory, and are empty when they could not be.
        Shown(Vec<u8>, usize, usize),
        // B-154: from memory, and whether the card has painted it into the window.
        Kept(Finished, bool),
        // B-158: only the part on screen, with the whole frame's width and height.
        Part(
            anime_compositor::WorkingBuffer,
            usize,
            usize,
            anime_compositor::render::Tile,
        ),
    }
    let mut card = card.map(|c| c.lock().expect("the card lock was poisoned"));
    if kept.is_none() {
        sight.card = drawn_by(card.as_deref());
    }
    let on_gpu = sight.card.is_some();
    let failures = card.as_deref().map_or(0, Card::failures);
    let (made, files) = match (kept, cache.as_deref_mut()) {
        (Some(kept), _) => (
            Ok(match card.as_deref_mut() {
                Some(Card {
                    on: true,
                    gpu: Some(Ok(gpu)),
                    paints,
                    size,
                    off_screen,
                    ..
                }) if gpu.on_screen() && !paints.is_empty() => {
                    gpu.hold(&kept.pixels, kept.width, kept.height);
                    match gpu.show(*size, paints, taken.alpha_only) {
                        Ok(()) => Made::Kept(kept, true),
                        Err(why) => {
                            stop_painting(viewer, gpu, off_screen, why);
                            Made::Kept(kept, false)
                        }
                    }
                }
                _ => Made::Kept(kept, false),
            }),
            Vec::new(),
        ),
        (None, Some(cache)) => anime_compositor::cache::files_read(|| match card.as_deref_mut() {
            Some(Card {
                on: true,
                gpu: Some(Ok(gpu)),
                paints,
                size,
                off_screen,
                ..
            }) if gpu.on_screen() && !paints.is_empty() => {
                preview::preview_frame_held(
                    &sight.project,
                    &sight.composition,
                    taken.frame,
                    &sight.root,
                    sight.quality,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    cache,
                    gpu,
                )
                .and_then(|(width, height)| {
                    // P-25: in a drag the picture is only painted; it is read back once the
                    // hand lets go, for the frame that is kept.
                    let pixels = if taken.dragging {
                        Vec::new()
                    } else {
                        gpu.picture().unwrap_or_default()
                    };
                    match gpu.show(*size, paints, taken.alpha_only) {
                        Ok(()) => Ok(Made::Shown(pixels, width, height)),
                        // No silent fallback: the card stops painting the window, the status line
                        // and the switch's tooltip say why, and the picture goes to the page as in
                        // B-44.
                        Err(why) => {
                            stop_painting(viewer, gpu, off_screen, why);
                            gpu.picture().map(|pixels| Made::Gpu(pixels, width, height))
                        }
                    }
                })
            }
            Some(Card {
                on: true,
                gpu: Some(Ok(gpu)),
                ..
            }) => preview::preview_frame_srgb8(
                &sight.project,
                &sight.composition,
                taken.frame,
                &sight.root,
                sight.quality,
                DEFAULT_TILE_SIZE,
                &mut log,
                cache,
                gpu,
            )
            .map(|(pixels, width, height)| Made::Gpu(pixels, width, height)),
            _ if seen.is_some() => preview::preview_part(
                &sight.project,
                &sight.composition,
                taken.frame,
                &sight.root,
                sight.quality,
                DEFAULT_TILE_SIZE,
                &mut log,
                cache,
                seen.expect("matched above"),
            )
            .map(|(buffer, width, height, part)| match part {
                Some(part) => Made::Part(buffer, width, height, part),
                None => Made::Cpu(buffer),
            }),
            _ => preview::preview_frame_cached(
                &sight.project,
                &sight.composition,
                taken.frame,
                &sight.root,
                sight.quality,
                DEFAULT_TILE_SIZE,
                &mut log,
                cache,
            )
            .map(Made::Cpu),
        }),
        (None, None) => unreachable!("the cels are locked for every frame not in memory"),
    };
    // A frame the card failed on is not kept: made again, the card may well draw it.
    let card_failed = card.as_deref().map_or(0, Card::failures) != failures;
    if let Some(said) = card
        .as_deref_mut()
        .and_then(|c| c.give_up_if_failed(failures))
    {
        announce(viewer, said);
    }
    let card_about = card.as_deref().map_or(String::new(), Card::about);
    drop(card);
    let counted = cache.as_deref().map_or((0, 0, 0), |cache| {
        (
            cache.hits() - counted.0,
            cache.misses() - counted.1,
            cache.effect_hits() - counted.2,
        )
    });
    // B-161: what happened to a disk copy on the way, this frame's or a read-ahead's.
    let disk_notes = cache
        .as_deref_mut()
        .map_or_else(Vec::new, |c| c.take_disk_notes());
    drop(cache);
    let made = match made {
        Ok(made) => made,
        // Document 28: a frame that cannot be made is reported, never replaced by something
        // that looks like a frame. The page shows this sentence instead of a picture.
        Err(diagnostic) => {
            anime_compositor::perf::end_capture();
            return allow_the_page_to_read_this(Response::builder().status(500))
                .header("content-type", "text/plain; charset=utf-8")
                .body(diagnostic.sentence().into_bytes())
                .expect("build the diagnostic response");
        }
    };

    let fresh = !matches!(made, Made::Kept(..));
    let on_screen = matches!(made, Made::Shown(..) | Made::Kept(_, true));
    // B-158: a part made alone is sent, never remembered as a frame.
    let mut drawn_part = None;
    let finished = if let Made::Kept(kept, _) = made {
        kept
    } else {
        let (pixels, width, height) = match made {
            Made::Cpu(buffer) => (buffer.to_srgb8_straight(), buffer.width(), buffer.height()),
            Made::Part(buffer, width, height, part) => {
                drawn_part = Some(part);
                (buffer.to_srgb8_straight(), width, height)
            }
            Made::Gpu(pixels, width, height) | Made::Shown(pixels, width, height) => {
                (pixels, width, height)
            }
            Made::Kept(..) => unreachable!("taken above"),
        };
        finished(pixels, width, height, taken.frame, log)
    };
    {
        let mut ram = taken
            .ram
            .lock()
            .expect("the frame memory lock was poisoned");
        if on_screen && !finished.pixels.is_empty() {
            ram.shown = Some(finished.clone());
        }
        if fresh
            && !taken.dragging
            && !card_failed
            && drawn_part.is_none()
            && finished.pixels.len() == finished.width * finished.height * 4
        {
            ram.frames
                .store(sight.clone(), taken.frame, files, finished.clone());
        }
    }

    // P-18: the next frame's drawings are read while this one is encoded, sent and drawn.
    let read_ahead = taken.ahead.is_some();
    if let Some(next) = taken.ahead {
        let cache = Arc::clone(&taken.cache);
        preview::read_ahead(sight.project, sight.composition, next, sight.root, cache);
    }

    let (width, height) = (finished.width, finished.height);
    // B-158: the part on screen, cut from a frame made whole or remembered when not made alone.
    // The card painting the window (B-45) paints all of it.
    let part = if on_screen {
        None
    } else {
        drawn_part.or_else(|| seen.and_then(|s| preview::part_in(s, width, height)))
    };
    let mut pixels = if on_screen {
        Vec::new()
    } else if let Some(p) = part.filter(|_| drawn_part.is_none()) {
        (p.y..p.y + p.height)
            .flat_map(|y| {
                finished.pixels[(y * width + p.x) * 4..(y * width + p.x + p.width) * 4]
                    .iter()
                    .copied()
            })
            .collect()
    } else {
        Arc::try_unwrap(finished.pixels).unwrap_or_else(|kept| kept.to_vec())
    };
    // No silent fallback: a frame the switch sent to the card and the CPU drew says so.
    let fell_back = finished.fell_back;
    let drawn_on = match (on_gpu, fell_back) {
        (false, _) => "CPU",
        (true, false) => "GPU",
        (true, true) => "CPU, not GPU",
    };
    if taken.alpha_only {
        as_alpha_only(&mut pixels);
    }
    let ms = began.elapsed().as_secs_f64() * 1000.0;
    if let Some(session) = session.filter(|_| logging) {
        session
            .lock()
            .expect("the session log lock was poisoned")
            .push(session_log::Row {
                since_on_ms: 0,
                frame: taken.frame,
                quality: taken.quality,
                playing: matches!(ask, Ask::At(_) | Ask::Play(_)),
                ms,
                stages: anime_compositor::perf::end_capture().unwrap_or_default(),
                dropped: taken.skipped,
                read_ahead,
                exporting,
                cel_hits: counted.0,
                cel_misses: counted.1,
                effect_hits: counted.2,
                warnings: finished
                    .warnings
                    .iter()
                    .cloned()
                    .chain(disk_notes)
                    .collect(),
            });
    }
    let reply = match part {
        Some(p) => taken.reply.header(
            "x-part",
            format!("{},{},{},{}", p.x, p.y, p.width, p.height),
        ),
        None => taken.reply,
    };
    reply
        // The only two things about a frame the window cannot say until it has been made.
        .header("x-width", width.to_string())
        .header("x-height", height.to_string())
        // P-15. Everything this window did for this frame, including the wait for the lock and
        // the encode, and stopping where the window's own work stops: the bytes are made, and
        // handing them to the web view is the next thing to happen and is not in here.
        .header("x-ms", format!("{ms:.1}"))
        // B-44: which processor drew these pixels, and the card, for the switch's label.
        .header("x-drawn-on", drawn_on)
        .header("x-card", for_a_header(&card_about))
        // Audit W8, W9: on a frame the card drew, the effects the processor ran for it, which
        // the viewer names beside the frame ("partly on the processor").
        .header(
            "x-partly",
            for_a_header(if drawn_on == "GPU" {
                &finished.on_processor
            } else {
                ""
            }),
        )
        // B-45: the pixels are already in the window, under the page; the body is empty.
        .header("x-on-screen", if on_screen { "1" } else { "0" })
        // B-154: 1 when the frame was sent from memory rather than made.
        .header("x-cached", if fresh { "0" } else { "1" })
        // D-263: a font this frame could not find, said beside the window's notes, as a note
        // is: its sentence, a tab, then its code and sentence for the error details. Missing
        // drawings have their own Relink line and are not repeated here.
        .header(
            "x-frame-notes",
            for_a_header(
                &finished
                    .warnings
                    .iter()
                    .filter_map(|w| {
                        w.strip_prefix("TEXT_FONT_MISSING: ")
                            .map(|said| format!("{said}\t{w}"))
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
        )
        .body(pixels)
        .expect("build the frame response")
}

/// B-154: which processor a frame is made on, as the frame memory tells frames apart: the card's
/// budget when the card is drawing, `None` for the CPU.
fn drawn_by(card: Option<&Card>) -> Option<usize> {
    match card {
        Some(Card {
            on: true,
            gpu: Some(Ok(gpu)),
            ..
        }) => Some(gpu.budget),
        _ => None,
    }
}

/// No silent fallback: the card stops painting the window, and the status line and the switch's
/// tooltip say why.
fn stop_painting(
    viewer: &Mutex<Viewer>,
    gpu: &mut Gpu,
    off_screen: &mut Option<String>,
    why: String,
) {
    gpu.let_go();
    announce(
        viewer,
        format!("The graphics card stopped painting the viewer: {why}. The page shows the picture instead."),
    );
    *off_screen = Some(why);
}

/// A frame just made, as the frame memory keeps it: with what was said making it.
fn finished(pixels: Vec<u8>, width: usize, height: usize, frame: i32, log: FrameLog) -> Finished {
    Finished {
        pixels: Arc::new(pixels),
        width,
        height,
        fell_back: log.ids_at(frame).contains(&DiagnosticId::GpuPreviewOnCpu),
        on_processor: log.on_processor().join(", "),
        warnings: log
            .finish()
            .iter()
            .map(|d| format!("{}: {}", d.id.as_str(), d.sentence()))
            .collect(),
    }
}

/// B-154 (G3): make the frames playback will ask for next, from where it is round the loop, until
/// every frame is made, playback stops or memory is full; and say how many were made. Each is made
/// as `serve_seen` makes it, under the same locks, one at a time, so a frame asked for waits at
/// most for one frame being made ahead.
fn render_ahead(viewer: &Mutex<Viewer>, card: Option<&Mutex<Card>>) -> usize {
    let mut made = 0;
    loop {
        let (mut sight, order, cels, ram) = {
            let v = viewer.lock().expect("the viewer lock was poisoned");
            let ram = Arc::clone(&v.ram);
            let Some(from) = ram
                .lock()
                .expect("the frame memory lock was poisoned")
                .playing
            else {
                return made;
            };
            let order: Vec<i32> = std::iter::successors(Some(from), |&n| Some(v.playback.after(n)))
                .take(v.playback.length() as usize)
                .collect();
            let sight = Sight {
                project: shown(&v),
                composition: v.composition.clone(),
                root: v.root.clone(),
                quality: v.quality,
                card: None,
            };
            (sight, order, Arc::clone(&v.cache), ram)
        };
        let mut cels = cels.lock().expect("the cel cache lock was poisoned");
        fit_cels(&mut cels, &ram);
        let mut card = card.map(|c| c.lock().expect("the card lock was poisoned"));
        sight.card = drawn_by(card.as_deref());
        let next = {
            let ram = ram.lock().expect("the frame memory lock was poisoned");
            order.into_iter().find(|&n| !ram.frames.holds(&sight, n))
        };
        let Some(frame) = next else { return made };
        let failures = card.as_deref().map_or(0, Card::failures);
        let mut log = FrameLog::new(3);
        let (result, files) = anime_compositor::cache::files_read(|| match card.as_deref_mut() {
            Some(Card {
                on: true,
                gpu: Some(Ok(gpu)),
                ..
            }) => {
                let result = preview::preview_frame_srgb8(
                    &sight.project,
                    &sight.composition,
                    frame,
                    &sight.root,
                    sight.quality,
                    DEFAULT_TILE_SIZE,
                    &mut log,
                    &mut cels,
                    gpu,
                );
                // The card's picture is also what the window shows: put back what was there.
                if gpu.on_screen() {
                    if let Some(shown) = &ram
                        .lock()
                        .expect("the frame memory lock was poisoned")
                        .shown
                    {
                        gpu.hold(&shown.pixels, shown.width, shown.height);
                    }
                }
                result
            }
            _ => preview::preview_frame_cached(
                &sight.project,
                &sight.composition,
                frame,
                &sight.root,
                sight.quality,
                DEFAULT_TILE_SIZE,
                &mut log,
                &mut cels,
            )
            .map(|b| (b.to_srgb8_straight(), b.width(), b.height())),
        });
        let card_failed = card.as_deref().map_or(0, Card::failures) != failures;
        if let Some(said) = card
            .as_deref_mut()
            .and_then(|c| c.give_up_if_failed(failures))
        {
            announce(viewer, said);
        }
        drop(card);
        drop(cels);
        let Ok((pixels, width, height)) = result else {
            return made;
        };
        if card_failed {
            return made;
        }
        let finished = finished(pixels, width, height, frame, log);
        if !ram
            .lock()
            .expect("the frame memory lock was poisoned")
            .frames
            .store(sight, frame, files, finished)
        {
            return made;
        }
        made += 1;
    }
}

/// Open a project file.
///
/// This is `persist::load` and a composition to look at. Media resolves against the project
/// file's own directory, which is the rule the core checks media against, so what the viewer
/// renders and what the core warned about are the same set of files.
fn open(path: &Path) -> Result<Viewer, Diagnostic> {
    let loaded = persist::load(path)?;
    // The first composition that has anything in it, and the first composition otherwise.
    //
    // Until B-12d a project this window edited had exactly one composition and `first` was the
    // whole rule. `composition.create` makes a second one reachable, and `verification/
    // B-12b_w01_walkthrough.md` immediately found what that costs: the walk made a composition,
    // filled it, saved, closed and reopened, and came back looking at the empty one the file had
    // started with. Nothing was lost, but a person cannot tell that from a blank viewer.
    //
    // This is a rule of thumb and not the answer. Document 07's project format has no field for
    // which composition was open, so there is nothing to restore; adding one is a schema change
    // and the owner's decision, and it is written up in
    // `verification/B-12d_new_composition_table.md`. Until then, an empty composition is never
    // the one somebody was working in.
    let composition = loaded
        .document
        .project()
        .compositions
        .iter()
        .find(|c| !c.is_empty())
        .or_else(|| loaded.document.project().compositions.first())
        .ok_or_else(|| {
            Diagnostic::new(
                DiagnosticId::ProjectSchemaInvalid,
                Severity::Error,
                "This project has no composition to show.",
                format!("{} contains an empty compositions array.", path.display()),
            )
            .with_remediation("The project that was open is still open. Nothing was changed.")
        })?;
    let (first, last) = composition.work_frames();
    let playback = Playback::new(first, last, composition.frame_rate);
    let composition = composition.id.clone();

    // Document 28's PROJECT_RECOVERY_AVAILABLE, at the one moment it can be acted on. A person
    // who is told about unsaved work an hour after opening the project has already redone it.
    let candidates = persist::recovery_candidates(path);
    let mut notes: Vec<String> = loaded.warnings.iter().map(Diagnostic::note).collect();
    if let Some(diagnostic) = persist::recovery_diagnostic(&candidates) {
        notes.push(diagnostic.note());
    }

    Ok(Viewer {
        document: loaded.document,
        preserved: loaded.preserved,
        path: Some(path.to_path_buf()),
        root: path.parent().unwrap_or(Path::new(".")).to_path_buf(),
        composition,
        quality: PreviewQuality::default(),
        playback,
        name: path
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_else(|| path.display().to_string()),
        notes,
        status: String::new(),
        recovery: candidates.into_iter().map(|c| c.path).collect(),
        autosaved: String::new(),
        dirty_since: None,
        alpha_only: false,
        // On, because a transparent frame that reads as black is a frame a person
        // misjudges, and every photograph of this window so far was taken with the grid there.
        checkerboard: true,
        solo: Vec::new(),
        clipboard: Vec::new(),
        relink: None,
        cache: Arc::new(Mutex::new(viewer_cache())),
        ram: Arc::new(Mutex::new(Ram {
            frames: FrameCache::with_budget(frame_share()),
            playing: None,
            shown: None,
        })),
    })
}

/// B-48, D-105: the Custom ceiling for the viewer's cache in bytes, or 0 for Automatic. The
/// window's, not a project's: a project opened later is given the same.
static RAM_CEILING: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

fn ram_ceiling() -> usize {
    match RAM_CEILING.load(std::sync::atomic::Ordering::Relaxed) {
        0 => anime_compositor::cache::automatic_budget(),
        custom => custom,
    }
}

/// B-161, D-232: where decoded drawings are kept on disk and the most they may take, or `None`
/// when the setting is 0 GB. The window's, as the memory setting is.
static DISK: Mutex<Option<anime_compositor::cache::DiskCache>> = Mutex::new(None);

fn disk_setting() -> Option<anime_compositor::cache::DiskCache> {
    DISK.lock()
        .expect("the disk setting lock was poisoned")
        .clone()
}

/// B-154: the most the finished frames may hold of the memory setting, in bytes: half, and never
/// so much that the cels have less than D-40's gibibyte, so at the smallest setting the viewer is
/// what it was before there were finished frames to keep.
fn frame_share() -> usize {
    let total = ram_ceiling();
    (total / 2).min(total.saturating_sub(anime_compositor::cache::DEFAULT_BUDGET_BYTES))
}

/// B-154: the cels have what the finished frames do not hold, so a first pass works with the whole
/// setting and the cels give way only as the loop is remembered. A fixed half starved a heavy
/// shot's first pass: document 08's ten layers dropped 85 frames of 240, not 24.
fn fit_cels(cels: &mut CelCache, ram: &Mutex<Ram>) {
    let frames = ram
        .lock()
        .expect("the frame memory lock was poisoned")
        .frames
        .held_bytes();
    cels.resize(ram_ceiling().saturating_sub(frames));
}

/// The viewer's cache as the memory and disk settings say, for a window or a project opened later.
fn viewer_cache() -> CelCache {
    let mut cache = CelCache::viewer_sized(ram_ceiling());
    cache.set_disk(disk_setting());
    cache
}

/// B-48, D-105: take the memory setting in `query`, apply it, and say what is in use, in bytes.
/// B-161: and the disk setting, `disk` in gigabytes (0 is off, anything unreadable the default)
/// and `diskfolder`, empty for the application's own data folder.
fn memory(app: &AppHandle, viewer: &Mutex<Viewer>, query: Option<&str>) -> String {
    use anime_compositor::cache;
    // Gigabytes as a person writes them, kept between 1 GiB and `most`; anything else is Automatic.
    let custom = |name: &str, most: usize| {
        parameter(query, name)
            .and_then(|v| v.parse::<f64>().ok())
            .filter(|gb| gb.is_finite() && *gb > 0.0)
            .map(|gb| ((gb * 1e9) as usize).clamp(cache::DEFAULT_BUDGET_BYTES, most))
    };
    if parameter(query, "ram").is_some() {
        let ram = custom("ram", cache::largest_budget());
        RAM_CEILING.store(ram.unwrap_or(0), std::sync::atomic::Ordering::Relaxed);
    }
    if let Some(gb) = parameter(query, "disk") {
        let cap = gb
            .parse::<f64>()
            .ok()
            .filter(|gb| gb.is_finite() && *gb >= 0.0)
            .map_or(cache::DEFAULT_DISK_CAP_BYTES, |gb| (gb * 1e9) as u64);
        // Always a folder of its own inside the one chosen, so the copies never mix with
        // anything a person keeps there.
        let folder = parameter(query, "diskfolder")
            .map(|f| f.trim().to_string())
            .filter(|f| !f.is_empty())
            .map(PathBuf::from)
            .or_else(|| app.path().app_local_data_dir().ok())
            .map(|dir| dir.join("decoded drawings"));
        *DISK.lock().expect("the disk setting lock was poisoned") = folder
            .filter(|_| cap > 0)
            .map(|folder| cache::DiskCache { folder, cap });
    }
    let held = {
        let (cache, ram) = {
            let viewer = viewer.lock().expect("the viewer lock was poisoned");
            (Arc::clone(&viewer.cache), Arc::clone(&viewer.ram))
        };
        let mut cache = cache.lock().expect("the cel cache lock was poisoned");
        cache.set_disk(disk_setting());
        let mut ram = ram.lock().expect("the frame memory lock was poisoned");
        ram.frames.resize(frame_share());
        cache.resize(ram_ceiling().saturating_sub(ram.frames.held_bytes()));
        cache.held_bytes() + cache.effect_held_bytes() + ram.frames.held_bytes()
    };
    let state = app.state::<Mutex<Card>>();
    let card = &mut *state.lock().expect("the card lock was poisoned");
    if parameter(query, "card").is_some() {
        // The Custom number is kept even while the card is closed, and applied when it opens.
        let most = match &card.gpu {
            Some(Ok(gpu)) => gpu.largest_budget(),
            _ => usize::MAX,
        };
        card.ceiling = custom("card", most);
    }
    let on_card = match &mut card.gpu {
        Some(Ok(gpu)) => {
            gpu.budget = card
                .ceiling
                .map_or(gpu.automatic_budget(), |c| c.min(gpu.largest_budget()));
            serde_json::json!({
                "memory": gpu.memory(),
                "now": gpu.budget,
                "automatic": gpu.automatic_budget(),
                "most": gpu.largest_budget(),
                "held": gpu.held(),
            })
        }
        _ => serde_json::Value::Null,
    };
    serde_json::json!({
        "ram": {
            "memory": cache::installed_memory(),
            "now": ram_ceiling(),
            "automatic": cache::automatic_budget(),
            "most": cache::largest_budget(),
            "held": held,
        },
        "card": on_card,
        "disk": disk_setting().map(|disk| serde_json::json!({
            "folder": disk.folder.to_string_lossy(),
            "now": disk.cap,
            "held": disk.held(),
        })),
    })
    .to_string()
}

/// The project the window opens on when it was not given one.
///
/// The reference shot, resolved against this crate's source directory, so it only works from a
/// checkout. Its media root is overridden because this one project is not where a project
/// normally is: `verification/B-08a_project.json` is written by `cargo test` into `verification`
/// while the cels it names live under `Fixtures/reference_shot`. The load warnings are dropped
/// with it, because they describe the directory this project is *not* rendered against and
/// would name files that are present. That is a development convenience and the only one; a
/// project the person actually opens goes through [`open`] unaltered.
fn demo() -> Viewer {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .to_path_buf();
    let path = repo.join("verification/B-08a_project.json");
    let mut viewer =
        open(&path).unwrap_or_else(|d| panic!("open {}: {}", path.display(), d.message));
    viewer.root = repo.join("Fixtures/reference_shot");
    viewer.name = "the reference shot".to_string();
    viewer.notes.clear();
    // With no save path there is nothing to recover *into*, so offering a snapshot would offer
    // something that could not be finished.
    viewer.recovery.clear();
    // No save path, deliberately. This project's file is written into `verification/` by
    // `cargo test` and CI checks that directory has not changed; a Save that landed there would
    // fail the build for a reason nobody would connect to a button. Save therefore asks where.
    viewer.path = None;
    viewer
}

/// W-24: document 24's `project.new`. An empty project with one composition to put layers in, the
/// size `composition.create` makes by default, and no file until Save As gives it one. Media
/// resolves against the directory the last project did until then.
fn blank(root: PathBuf) -> Viewer {
    let mut project = Project::new(Id::new("project-1"));
    let rate = FrameRate::new(24, 1).expect("twenty-four frames a second is a frame rate");
    let composition = Composition::new(Id::new("comp-1"), "Main", 1920, 1080, rate, 0, 240);
    let (first, last) = composition.work_frames();
    let id = composition.id.clone();
    project.compositions.push(composition);
    Viewer {
        document: Document::new(project),
        preserved: Preserved::none(),
        path: None,
        root,
        composition: id,
        quality: PreviewQuality::default(),
        playback: Playback::new(first, last, rate),
        name: "Untitled project".to_string(),
        notes: Vec::new(),
        status: String::new(),
        recovery: Vec::new(),
        autosaved: String::new(),
        dirty_since: None,
        alpha_only: false,
        checkerboard: true,
        solo: Vec::new(),
        clipboard: Vec::new(),
        relink: None,
        cache: Arc::new(Mutex::new(viewer_cache())),
        ram: Arc::new(Mutex::new(Ram {
            frames: FrameCache::with_budget(frame_share()),
            playing: None,
            shown: None,
        })),
    }
}

/// W-24: the project as the preview draws it. With layers soloed, every other layer of the
/// composition on screen is off; a soloed layer since deleted, or in another composition, counts
/// for nothing, so a stale solo can never blank the picture.
fn shown(viewer: &Viewer) -> Project {
    let mut project = viewer.document.project().clone();
    if let Some(comp) = project
        .compositions
        .iter_mut()
        .find(|c| c.id == viewer.composition)
    {
        let soloed: Vec<Id> = viewer
            .solo
            .iter()
            .filter(|id| comp.layer(id).is_some())
            .cloned()
            .collect();
        if !soloed.is_empty() {
            comp.solo(&soloed);
        }
    }
    project
}

/// Save the open project to `path`, and say what happened in one sentence.
///
/// This is `persist::save` with the preserved data handed back to it, and nothing else. On
/// failure the file at `path` is untouched and the document stays dirty, which is the core's
/// guarantee rather than this function's.
fn write(viewer: &mut Viewer, path: &Path) -> Result<(), Diagnostic> {
    persist::save(path, &mut viewer.document, &viewer.preserved)
}

/// Save to the file the project came from, or say why there is not one.
fn save(viewer: &Mutex<Viewer>) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    let Some(path) = viewer.path.clone() else {
        return "This project has no file of its own yet. Use Save As.".to_string();
    };
    match write(viewer, &path) {
        Ok(()) => format!("Saved to {}", path.display()),
        Err(diagnostic) => diagnostic.sentence(),
    }
}

/// Save to a file the person chose, then open that file.
///
/// Reopening is the point. After Save As the window is showing a project that lives somewhere
/// else, and media paths in a project file are relative to it, so a project saved into another
/// directory may no longer find its cels. Loading the file back means the window shows what
/// somebody opening it tomorrow would see, warnings and all, instead of a picture that only
/// works because the old media root is still in memory.
///
/// The paths are rewritten for the new directory first, on a copy, so the reopened project finds
/// the same drawings; a failed write leaves the open project exactly as it was.
fn save_as(viewer: &Mutex<Viewer>, path: &Path) -> String {
    {
        let viewer = &*viewer.lock().expect("the viewer lock was poisoned");
        let to = path.parent().unwrap_or(Path::new("."));
        let mut copy = Document::new(persist::rebased(
            viewer.document.project(),
            &viewer.root,
            to,
        ));
        if let Err(diagnostic) = persist::save(path, &mut copy, &viewer.preserved) {
            return diagnostic.sentence();
        }
        if let (None, Some(folder)) = (&viewer.path, unsaved_folder()) {
            forget_unsaved(&folder);
        }
    }
    let said = format!("Saved to {}", path.display());
    take(viewer, path);
    said
}

/// D-732: the file Increment and Save writes, as After Effects' Ctrl+Alt+Shift+S does: the name's
/// trailing `_v<number>` goes up by one (`shot_v002.json` to `shot_v003.json`), a name without
/// one gains `_v002`, and a number already taken on disk is skipped so nothing is overwritten.
fn incremented(path: &Path) -> PathBuf {
    let stem = path
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let ext = path
        .extension()
        .map(|e| format!(".{}", e.to_string_lossy()))
        .unwrap_or_default();
    let (base, mut n) = match stem.rfind("_v") {
        Some(at) if at + 2 < stem.len() && stem[at + 2..].bytes().all(|b| b.is_ascii_digit()) => (
            stem[..at].to_string(),
            stem[at + 2..].parse::<u64>().unwrap_or(1) + 1,
        ),
        _ => (stem.clone(), 2),
    };
    loop {
        let next = path.with_file_name(format!("{base}_v{n:03}{ext}"));
        if !next.exists() {
            return next;
        }
        n += 1;
    }
}

/// D-732: Save As to the next version's name, so the window works on the copy and the file it
/// came from is left as it was. `None` when the project has no file to count up from yet.
fn increment_and_save(viewer: &Mutex<Viewer>) -> Option<String> {
    let path = viewer
        .lock()
        .expect("the viewer lock was poisoned")
        .path
        .clone()?;
    Some(save_as(viewer, &incremented(&path)))
}

/// A file the importer reads: PNG, EXR by D-62, or one of D-72's formats.
fn is_drawing(path: &Path) -> bool {
    path.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("png"))
        || exr_io::is_exr(&path.to_string_lossy())
        || media::is_other_format(path)
}

/// Load a dropped or named file into the viewer, or report why it could not be.
///
/// A file that cannot be opened leaves the project that was open exactly as it was and adds the
/// reason to what the window is saying. Closing a working project because the next one was
/// unreadable would lose the person their place to punish them for a bad drop.
/// True if it opened; only then does it belong on the recent list.
fn take(viewer: &Mutex<Viewer>, path: &Path) -> bool {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match open(path) {
        Ok(opened) => {
            *viewer = opened;
            viewer.status = format!("Opened {}", path.display());
            true
        }
        Err(diagnostic) => {
            viewer.notes = vec![diagnostic.note()];
            viewer.status = format!("{} could not be opened.", path.display());
            false
        }
    }
}

// -------------------------------------------------------------------------------------------
// Autosave and recovery
// -------------------------------------------------------------------------------------------

/// Document 07: "save a recovery snapshot after two minutes of dirty activity".
const AUTOSAVE_AFTER: Duration = Duration::from_secs(120);

/// How often the timer looks. Short enough that the two minutes above are two minutes and not
/// two and a half, long enough that a window nobody is touching costs nothing.
const AUTOSAVE_TICK: Duration = Duration::from_secs(10);

/// One look at the clock. Returns what to say if a snapshot was written, and nothing otherwise.
///
/// `now` is a parameter so this can be checked without waiting two minutes; the window passes
/// `Instant::now()`.
///
/// Document 26: "Autosave does not clear user-facing dirty state and does not replace the
/// canonical manual-save path." Nothing here calls `mark_saved` and nothing here can — the core's
/// `autosave` takes the document by shared reference for exactly that reason.
fn autosave_tick(viewer: &mut Viewer, now: Instant) -> Option<String> {
    autosave_tick_in(viewer, now, unsaved_folder().as_deref())
}

/// D-256: the app's own folder for copies of a project that was never saved, set once the window
/// knows where `%LOCALAPPDATA%` is. Unset, such a project gets no copies, as before D-256.
static UNSAVED: Mutex<Option<PathBuf>> = Mutex::new(None);

fn unsaved_folder() -> Option<PathBuf> {
    UNSAVED
        .lock()
        .expect("the unsaved folder lock was poisoned")
        .clone()
}

/// The file a never-saved project's copies are named after. It is never written itself; its
/// copies sit beside it as `Unsaved project.autosave-N.json`, as a saved project's do.
fn unsaved_copy(folder: &Path) -> PathBuf {
    folder.join("Unsaved project.json")
}

/// [`autosave_tick`] with the folder for a never-saved project's copies handed in.
fn autosave_tick_in(viewer: &mut Viewer, now: Instant, unsaved: Option<&Path>) -> Option<String> {
    // D-256: a project with no file of its own puts its copies in the app's own folder, its
    // drawings' paths rewritten for there as Save As would. With no such folder there is nowhere
    // to put one.
    let path = match (viewer.path.clone(), unsaved) {
        (Some(path), _) => path,
        (None, Some(folder)) => unsaved_copy(folder),
        (None, None) => {
            viewer.dirty_since = None;
            return None;
        }
    };
    if !viewer.document.is_dirty() {
        viewer.dirty_since = None;
        return None;
    }
    let since = *viewer.dirty_since.get_or_insert(now);
    if now.duration_since(since) < AUTOSAVE_AFTER {
        return None;
    }
    // Measured from this snapshot, not from when the work began, so a document left dirty writes
    // one snapshot every two minutes rather than one on every tick after the first two.
    viewer.dirty_since = Some(now);
    let rebased = unsaved.filter(|_| viewer.path.is_none()).map(|folder| {
        let _ = std::fs::create_dir_all(folder);
        Document::new(persist::rebased(
            viewer.document.project(),
            &viewer.root,
            folder,
        ))
    });
    Some(
        match persist::autosave(
            &path,
            rebased.as_ref().unwrap_or(&viewer.document),
            &viewer.preserved,
        ) {
            Ok(written) => {
                viewer.recovery = persist::recovery_candidates(&path)
                    .into_iter()
                    .map(|c| c.path)
                    .collect();
                format!("Recovery snapshot written to {}", written.display())
            }
            // A failed autosave is said out loud rather than swallowed. It is the one moment the
            // person can still do something about it — the work is in memory and nowhere else.
            Err(diagnostic) => diagnostic.sentence(),
        },
    )
}

/// Open a recovery snapshot as the project it belongs to.
///
/// The snapshot's *contents* are loaded and the project's *identity* is kept: the window is left
/// pointing at the project file, so Save writes the recovered work into the project rather than
/// back into the snapshot. Document 07 requires that recovering not overwrite the last manual
/// save, and it does not: nothing is written here at all. The project file is still on disk
/// exactly as it was, which is why the document opens dirty — the difference between what is on
/// screen and what is in the file is the work being recovered.
fn recover(viewer: &Mutex<Viewer>, snapshot: &Path) -> String {
    recover_in(viewer, snapshot, unsaved_folder().as_deref())
}

/// [`recover`] with the folder for a never-saved project's copies handed in.
fn recover_in(viewer: &Mutex<Viewer>, snapshot: &Path, unsaved: Option<&Path>) -> String {
    // Bound first: a lock taken in the `match` itself would be held through the arms below.
    let path = viewer
        .lock()
        .expect("the viewer lock was poisoned")
        .path
        .clone();
    let project = match path {
        Some(path) => path,
        // D-256: a copy of a project that was never saved opens as one, still unsaved. Its
        // drawings' paths were written for the folder it is in, which `open` makes its root.
        None => match unsaved.filter(|folder| snapshot.parent() == Some(folder)) {
            Some(folder) => {
                let mut taken = match open(snapshot) {
                    Ok(taken) => taken,
                    Err(diagnostic) => return diagnostic.sentence(),
                };
                let project = taken.document.project().clone();
                taken.document = Document::recovered(project.clone(), Project::new(project.id));
                taken.path = None;
                taken.name = "Untitled project (recovered)".to_string();
                taken.recovery = persist::recovery_candidates(&unsaved_copy(folder))
                    .into_iter()
                    .map(|c| c.path)
                    .collect();
                taken.notes.push(format!(
                    "This is the recovery copy {} of a project that was never saved. Save As gives it a file.",
                    snapshot.display()
                ));
                *viewer.lock().expect("the viewer lock was poisoned") = taken;
                return format!("Recovered {}", snapshot.display());
            }
            None => return "There is no project to recover into.".to_string(),
        },
    };
    let mut taken = match open(snapshot) {
        Ok(taken) => taken,
        Err(diagnostic) => return diagnostic.sentence(),
    };
    // The project as the file has it, which is the baseline the recovered document is dirty
    // against. If it cannot be read, the snapshot is not opened either: a window that could not
    // say what the file holds cannot say what is outstanding.
    let saved = match persist::load(&project) {
        Ok(saved) => saved,
        Err(diagnostic) => return diagnostic.sentence(),
    };
    taken.document = Document::recovered(
        taken.document.project().clone(),
        saved.document.project().clone(),
    );
    taken.name = project
        .file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_else(|| project.display().to_string());
    taken.recovery = persist::recovery_candidates(&project)
        .into_iter()
        .map(|c| c.path)
        .collect();
    taken.notes.push(format!(
        "This is the recovery snapshot {}, not the saved project. Nothing has been written to \
         {} yet; Save writes this into it.",
        snapshot.display(),
        project.display()
    ));
    taken.path = Some(project);
    let said = format!("Recovered {}", snapshot.display());
    *viewer.lock().expect("the viewer lock was poisoned") = taken;
    said
}

/// D-256: at the start, a window with no project file of its own offers the copies of the last
/// project that was never saved, as an opened project offers its own.
fn offer_unsaved(viewer: &mut Viewer, folder: &Path) {
    if viewer.path.is_some() {
        return;
    }
    viewer.recovery = persist::recovery_candidates(&unsaved_copy(folder))
        .into_iter()
        .map(|c| c.path)
        .collect();
    // Its own sentence: document 28's speaks of a saved project still on disk, and there is none.
    match viewer.recovery.len() {
        0 => {}
        n => viewer.notes.push(format!(
            "There {} {n} recovery {} of a project that was never saved. File > Recover unsaved work opens one.",
            if n == 1 { "is" } else { "are" },
            if n == 1 { "snapshot" } else { "snapshots" },
        )),
    }
}

/// D-256: once a never-saved project has a file, its copies go beside it and the old ones go.
fn forget_unsaved(folder: &Path) {
    for slot in 0..persist::AUTOSAVE_SLOTS {
        let _ = std::fs::remove_file(persist::autosave_path(&unsaved_copy(folder), slot));
    }
}

/// Say something in the window's status line, replacing whatever it said before.
fn announce(viewer: &Mutex<Viewer>, said: String) {
    viewer.lock().expect("the viewer lock was poisoned").status = said;
}

// -------------------------------------------------------------------------------------------
// Editing
// -------------------------------------------------------------------------------------------

/// Everything the editing panels draw, as one JSON answer.
///
/// The project half of it is `persist::to_json` - the same text a save writes, preserved records
/// and all - rather than a summary shaped for the page. Two reasons. What a panel shows is then
/// what the file would hold, so a panel cannot quietly disagree with a save. And a record this
/// build does not model still reaches the page, so an effect from a later version is listed as
/// present in the stack instead of being absent from the only view the person has of it.
///
/// Selection is not here. Which layer is being worked on is not a fact about the project and does
/// not belong in the file; it lives in the page, and the page is the only thing that needs it.
fn state(viewer: &Mutex<Viewer>) -> String {
    let viewer = &*viewer.lock().expect("the viewer lock was poisoned");
    let project: serde_json::Value = serde_json::from_str(&persist::to_json(
        viewer.document.project(),
        &viewer.preserved,
    ))
    .expect("the project text the core just wrote is JSON");
    serde_json::json!({
        "project": project,
        "composition": viewer.composition.as_str(),
        "revision": viewer.document.revision(),
        // W-24: which layers are soloed. The window's, not the file's, like the alpha view.
        "solo": viewer.solo.iter().map(|id| id.as_str()).collect::<Vec<_>>(),
        // What Undo and Redo would do next, in the words document 26 requires each record to
        // carry. The buttons say it rather than saying "Undo", because a person who has been
        // away from the window for a minute cannot otherwise know what is about to be taken back.
        "undo": viewer.document.undo_labels(),
        "redo": viewer.document.redo_labels(),
    })
    .to_string()
}

/// Apply one command and say what happened, in the history record's own words.
///
/// Document 26: a rejected command changes nothing at all. That is the core's guarantee and not
/// this function's - what happens here on a refusal is that the diagnostic becomes the sentence
/// on the status line, so the person is told which rule stopped them rather than watching a
/// control spring back with no explanation.
fn edit(viewer: &Mutex<Viewer>, command: Command) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match viewer.document.apply(command) {
        Ok(record) => record.label.clone(),
        Err(diagnostic) => diagnostic.sentence(),
    }
}

/// Put a composition of the open project on screen, with a playback clock to match it.
///
/// The clock is rebuilt rather than kept, because it carries the composition's first and last
/// frame and its rate: a viewer that switched composition without it would play the new shot to
/// the old one's length.
fn show(viewer: &Mutex<Viewer>, id: &Id) {
    let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
    let Some(comp) = held.document.project().composition(id) else {
        return;
    };
    let (first, last) = comp.work_frames();
    held.playback = Playback::new(first, last, comp.frame_rate);
    held.composition = id.clone();
}

/// Put the window on the composition an undo or a redo moved, and never on one that is gone.
///
/// `touched` is the record's own affected list. Three rules in order of preference, and they are
/// one rule: show the person the change they just asked for.
///
/// If the record names a composition that is in the project, that is where the change was, and
/// that is where the window goes -- which for a redone `composition.create` is the composition
/// that has just come back, and for any layer edit is the composition it happened in. If it
/// names none that exist, the window stays where it is. If where it is has itself been taken
/// away -- which is what undoing `composition.create` does -- it falls back to the first
/// composition, which is where [`open`] starts. Without that last rule the window keeps an ID
/// nothing answers, every frame request finds nothing, and the person is left looking at a
/// viewer that says there is no composition on screen with no way back to one there is.
fn settle(viewer: &Mutex<Viewer>, touched: &[Id]) {
    let go = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let project = held.document.project();
        // B-18c: a record that touched the composition on screen stays there. Pre-compose
        // names the new composition first, and redoing it should not walk the person into it.
        touched
            .iter()
            .find(|id| **id == held.composition && project.composition(id).is_some())
            .or_else(|| touched.iter().find(|id| project.composition(id).is_some()))
            .or_else(|| project.composition(&held.composition).map(|c| &c.id))
            .or_else(|| project.compositions.first().map(|c| &c.id))
            .cloned()
    };
    if let Some(id) = go {
        show(viewer, &id);
    }
}

/// `edit.undo` and `edit.redo` from document 24.
///
/// Both name what they moved. "Undone" alone would be true and useless: the whole reason undo is
/// trusted is that the person can see it took back the thing they meant.
fn undo(viewer: &Mutex<Viewer>) -> String {
    let (said, touched) = {
        let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
        let dragging = held.document.drag_in_progress();
        match held.document.undo() {
            Some(record) => (format!("Undone: {}", record.label), record.affected.clone()),
            None if dragging => ("Let go of the drag first.".to_string(), Vec::new()),
            None => ("There is nothing to undo.".to_string(), Vec::new()),
        }
    };
    settle(viewer, &touched);
    said
}

fn redo(viewer: &Mutex<Viewer>) -> String {
    let (said, touched) = {
        let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
        let dragging = held.document.drag_in_progress();
        match held.document.redo() {
            Some(record) => (format!("Redone: {}", record.label), record.affected.clone()),
            None if dragging => ("Let go of the drag first.".to_string(), Vec::new()),
            None => ("There is nothing to redo.".to_string(), Vec::new()),
        }
    };
    settle(viewer, &touched);
    said
}

/// Which transform property a request names, or `None` for a word that is not one of them.
///
/// The five names are document 19's own, which are also the keys `persist` writes, so the page
/// sends back the word it was given rather than a number this file would have to keep in step.
fn property(name: &str) -> Option<Prop> {
    [
        Prop::Anchor,
        Prop::Position,
        Prop::Scale,
        Prop::Rotation,
        Prop::Opacity,
        // B-13d: document 24 line 91. A depth is named here with the five, and every route
        // below that takes a property reaches it without another word being written.
        Prop::Depth,
        // B-19g: D-69's two halves of a separated position. A layer whose position is whole
        // has neither, and the core says so.
        Prop::PositionX,
        Prop::PositionY,
        // D-323: a Time Remap, keyed through the same routes; the core refuses it while it is off.
        Prop::TimeRemap,
    ]
    .into_iter()
    .find(|p| p.as_str() == name)
}

/// The property a request names, as something owned.
///
/// Owned rather than borrowed because a depth may not be there to borrow: a layer that has
/// never been given one sits on plane 0, and that is the property to read rather than a
/// refusal. The five are cloned for the sake of one answer covering both, which costs a
/// keyframe list per request at the rate a window asks questions.
fn property_of(layer: &Layer, prop: Prop) -> anime_compositor::model::Property {
    let plane_zero = || anime_compositor::model::Property::constant(Value::Scalar(0.0));
    match prop {
        Prop::Depth => layer.depth.clone().unwrap_or_else(plane_zero),
        Prop::TimeRemap => layer.time_remap.clone().unwrap_or_else(plane_zero),
        _ => layer
            .transform
            .get(prop)
            .cloned()
            .unwrap_or_else(plane_zero),
    }
}

/// A property value from the page: `x,y` for the three vec2 properties, one number for the two
/// scalar ones.
///
/// Shaped by the property rather than guessed from the text, so that sending one number for
/// position is refused by the core's own rule about kinds instead of being quietly read as a
/// scalar. Whether the number is in range is not decided here either - opacity clamps, and that
/// is document 19's decision and is made in one place.
/// B-19d: `fx:<instance>:<setting>` names an effect's setting wherever a request names a
/// property, so that the diamond, the rows of the timeline, the graph, F9 and the key clipboard
/// reach D-68's keys through the requests they already send.
fn effect_setting(prop: &str) -> Option<(Id, String)> {
    let (instance, setting) = prop.strip_prefix("fx:")?.rsplit_once(':')?;
    Some((Id::new(instance), setting.to_string()))
}

/// The numbers of a setting as a request writes them, one or a colour's three.
fn setting_value(setting: &str, count: usize, text: &str) -> Result<Vec<f64>, String> {
    let value: Vec<f64> = text
        .split(',')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    if value.len() != count || text.split(',').count() != count {
        return Err(format!(
            "{setting} needs {}. Not \"{text}\".",
            if count == 3 {
                "three numbers, like 1, 0.5, 0"
            } else {
                "a number"
            }
        ));
    }
    Ok(value)
}

/// B-19d: what one keyframe request, or one value, does to an effect's setting. The core's
/// command replaces every key of the setting as one, so the keys are read, changed and sent
/// back; a value on a setting with no keys is `SET_EFFECT_PARAMETERS` as it always was.
fn effect_key_command(
    id: &str,
    query: Option<&str>,
    composition: Id,
    layer: &Layer,
    instance_id: Id,
    setting: String,
) -> Result<Command, String> {
    let Some(existing) = layer.effects.iter().find(|e| e.instance_id == instance_id) else {
        return Err(format!("{instance_id} is not an effect on this layer."));
    };
    let Some(count) = existing.arity(&setting) else {
        return Err(format!(
            "A {} has no setting called {setting}.",
            existing.type_id()
        ));
    };
    let mut keys = existing.keys(&setting);
    let value = match id {
        "property.set_base" | "property.drag_update" => {
            let Some(text) = parameter(query, "value") else {
                return Err(format!("What should {setting} be set to?"));
            };
            let value = setting_value(&setting, count, &text)?;
            // D-202: the Mix is the effect's own, not one of its settings.
            if keys.is_empty() && setting == "mix" {
                return Ok(Command::SetEffectMix {
                    composition,
                    layer_id: layer.id.clone(),
                    instance_id,
                    mix: value[0],
                });
            }
            if keys.is_empty() {
                let mut effect = existing.effect.clone();
                effect.set(&setting, &value);
                return Ok(Command::SetEffectParameters {
                    composition,
                    layer_id: layer.id.clone(),
                    instance_id,
                    effect,
                });
            }
            Some(value)
        }
        _ => None,
    };
    let frame = frame_parameter(
        query,
        if id == "keyframe.move" {
            "from"
        } else {
            "frame"
        },
    )?;
    let at = keys.iter().position(|k| k.frame == frame);
    let missing = || format!("{setting} has no keyframe at frame {frame}.");
    match (id, value) {
        // W-10's rule for a layer's property: a value on a keyed setting is a key on that frame,
        // keeping the ease a key already there was given.
        (_, Some(value)) => match at {
            Some(i) => keys[i].value = value,
            None => keys.push(EffectKey {
                frame,
                value,
                interp: Interp::Linear,
            }),
        },
        // The diamond: a key holding what the setting already is on that frame, or the key gone.
        ("keyframe.add_remove", _) => match at {
            Some(i) => {
                keys.remove(i);
            }
            None => keys.push(EffectKey {
                frame,
                value: existing.at(frame).get(&setting).unwrap_or_default(),
                interp: Interp::Linear,
            }),
        },
        ("keyframe.move", _) => keys[at.ok_or_else(missing)?].frame = frame_parameter(query, "to")?,
        _ => keys[at.ok_or_else(missing)?].interp = interp_parameter(query)?,
    }
    keys.sort_by_key(|k| k.frame);
    Ok(Command::SetEffectKeys {
        composition,
        layer_id: layer.id.clone(),
        instance_id,
        setting,
        keys,
    })
}

fn property_value(prop: Prop, text: &str) -> Option<Value> {
    let number = |t: &str| t.trim().parse::<f64>().ok();
    // D-22: the file holds a scale as a percentage and the model holds the factor document 21
    // composes with, and `persist` divides on the way in and multiplies on the way out. The panel
    // is given the file's numbers and sends them back in the same units, so the same division
    // belongs here. Without it, somebody who typed the 100 the field was already showing got a
    // layer a hundred times its size, and the walkthrough in `acceptance` is what found that:
    // every earlier test of this function set a position or checked a scale being refused.
    let percent = if prop == Prop::Scale { 100.0 } else { 1.0 };
    match prop.kind() {
        "vec2" => {
            let (x, y) = text.split_once(',')?;
            Some(Value::Vec2(number(x)? / percent, number(y)? / percent))
        }
        _ => Some(Value::Scalar(number(text)? / percent)),
    }
}

/// The interpolation a request names, for the two routes that name one.
///
/// One reader rather than two since B-13e: a layer's keys and the camera's are eased by the same
/// request, and a second copy of this match is a second place for `curve` to be forgotten.
/// The window's name for the camera where a request wants a layer id.
///
/// Not a name a file can hold: document 19's ids come out of the project and this one comes out
/// of the page, which is what lets a key on a camera row travel through the requests that name
/// their keys `layer|property|frame` without a second format being invented for it.
const CAMERA_ROW: &str = "::camera";

/// A camera property as one of `Prop`'s.
///
/// B-13e: the camera's properties go through the same commands as a layer's and so arrive as the
/// same `Prop`. Which of the two a command means is the target beside it, never the name.
fn prop_of_camera(which: anime_compositor::model::CameraProp) -> Prop {
    match which {
        anime_compositor::model::CameraProp::Position => Prop::Position,
        anime_compositor::model::CameraProp::Depth => Prop::Depth,
        anime_compositor::model::CameraProp::Zoom => Prop::Zoom,
    }
}

fn interp_parameter(query: Option<&str>) -> Result<Interp, String> {
    interp_named(query, "mode")
}

/// `interp_parameter` under another name, for B-24e's path keys: `mask.set` already reads
/// `mode` as the mask's own mode, so a path key's curve arrives as `interp`.
fn interp_named(query: Option<&str>, name: &str) -> Result<Interp, String> {
    match parameter(query, name).as_deref() {
        Some("hold") => Ok(Interp::Hold),
        Some("linear") => Ok(Interp::Linear),
        // Without `curve`, this is the preset button beside the diamond and the curve is easy
        // ease. With it, it is the graph editor sending the four numbers a handle was just
        // dragged to.
        Some("ease") => match parameter(query, "curve") {
            None => Ok(Interp::EASY),
            Some(text) => handles(&text),
        },
        other => Err(format!(
            "An interpolation is hold, linear or ease. Not \"{}\".",
            other.unwrap_or("")
        )),
    }
}

/// Release a drag: one history entry covering everything the scrub passed through.
///
/// A drag that ended where it started is not an edit. The core returns `None` for it and puts
/// the value back, and this says so rather than reporting a change nobody made.
fn end_drag(viewer: &Mutex<Viewer>) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    match viewer.document.end_drag() {
        Some(record) => record.label.clone(),
        None => "Nothing moved.".to_string(),
    }
}

/// Escape during a drag. Document 24: "Escape restores the pre-drag value."
fn cancel_drag(viewer: &Mutex<Viewer>) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    let was = viewer.document.drag_in_progress();
    viewer.document.cancel_drag();
    if was {
        "Cancelled; the value is back where the drag started.".to_string()
    } else {
        "Nothing is being dragged.".to_string()
    }
}

/// One intermediate value of a scrub, starting the transaction if this is the first one.
///
/// Document 26 wants a drag to be one history entry, which means something has to open the
/// transaction. Doing it here rather than on a separate request from the page removes the state
/// the page would otherwise have to keep in step with the document - a page that forgot to open
/// one would write a history entry per pixel of mouse movement, and nothing would look wrong
/// until somebody pressed Ctrl+Z.
fn drag_update(viewer: &Mutex<Viewer>, command: Command) -> String {
    let viewer = &mut *viewer.lock().expect("the viewer lock was poisoned");
    if !viewer.document.drag_in_progress() {
        if let Err(diagnostic) = viewer.document.begin_drag() {
            return diagnostic.sentence();
        }
    }
    match viewer.document.update_drag(command) {
        // Not the label: a scrub sends these several times a second and the status line is for
        // reading. What it moved to is the useful half.
        Ok(()) => "Dragging.".to_string(),
        Err(diagnostic) => diagnostic.sentence(),
    }
}

/// A layer ID nothing in this project is using.
///
/// Counted rather than random, so that running the same steps twice writes the same file and a
/// person reading one can tell which layer is which. It takes one past the highest number in
/// use rather than the first gap: a gap means a layer was deleted, and giving its number to
/// something else makes two different layers indistinguishable in a diff of two saves.
fn unused_layer_id(project: &Project) -> Id {
    let highest = project
        .compositions
        .iter()
        .flat_map(|c| c.layer_order())
        .filter_map(|id| id.as_str().strip_prefix("layer-"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max();
    Id::new(format!("layer-{}", highest.map_or(1, |n| n + 1)))
}

/// W-22: a layer as it is, under an identifier nothing in the project is using. Its effects keep
/// their instance IDs: a command names an effect by its layer as well, so two layers may hold the
/// same one, and `unused_effect_id` counts past both.
fn copy_of(project: &Project, layer: &Layer) -> Layer {
    let mut copy = Layer {
        id: unused_layer_id(project),
        ..layer.clone()
    };
    // D-191: a layer setting naming the layer itself names the copy; the others are kept.
    for instance in &mut copy.effects {
        for (named, _) in instance.effect.layer_settings_mut() {
            if named.as_str() == Some(layer.id.as_str()) {
                *named = copy.id.as_str().into();
            }
        }
    }
    copy
}

/// An effect instance ID nothing in this project is using, counted the way layer IDs are.
///
/// Counted across the whole project rather than the one layer, so that an effect keeps its
/// identity if it is ever moved between layers and two saves of the same steps match.
fn unused_effect_id(project: &Project) -> Id {
    let highest = project
        .compositions
        .iter()
        .flat_map(|c| c.layers_in_order())
        .flat_map(|l| l.effects.iter())
        .filter_map(|e| e.instance_id.as_str().strip_prefix("fx-"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max();
    Id::new(format!("fx-{}", highest.map_or(1, |n| n + 1)))
}

/// A composition ID nothing in this project is using, counted the way layer IDs are.
fn unused_composition_id(project: &Project) -> Id {
    let highest = project
        .compositions
        .iter()
        .filter_map(|c| c.id.as_str().strip_prefix("comp-"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max();
    Id::new(format!("comp-{}", highest.map_or(1, |n| n + 1)))
}

/// An asset ID nothing in this project is using, counted the way layer IDs are.
fn unused_asset_id(project: &Project) -> Id {
    let highest = project
        .assets
        .iter()
        .filter_map(|a| a.id.as_str().strip_prefix("asset-"))
        .filter_map(|n| n.parse::<u64>().ok())
        .max();
    Id::new(format!("asset-{}", highest.map_or(1, |n| n + 1)))
}

/// A list of numbers as somebody would read it out: `7`, `7 and 9`, `7, 9 and 11`.
fn spoken(numbers: &[u32]) -> String {
    let text: Vec<String> = numbers.iter().map(u32::to_string).collect();
    match text.split_last() {
        None => String::new(),
        Some((last, [])) => last.clone(),
        Some((last, rest)) => format!("{} and {last}", rest.join(", ")),
    }
}

/// D-84's `timesheet.import`: the cut in the chosen folder, added as one entry to undo and put on
/// screen. What the reading noticed goes to the notes list, as an import's does, and a folder
/// nothing is imported from says why on the status line and in the notes.
fn import_cut(viewer: &Mutex<Viewer>, folder: &Path) -> String {
    let cut = match timesheet::read_cut(folder) {
        Ok(cut) => cut,
        Err(refused) => {
            let refusal = refused.refusal.diagnostic();
            let mut held = viewer.lock().expect("the viewer lock was poisoned");
            held.notes.push(refusal.note());
            held.notes
                .extend(refused.notes.iter().map(|n| n.diagnostic().note()));
            return format!("Nothing was imported. {}", refusal.sentence());
        }
    };
    let id = {
        let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
        let (id, commands) = timesheet::commands(&cut, held.document.project(), &held.root);
        if let Err(diagnostic) = held.document.apply_all(commands) {
            return diagnostic.sentence();
        }
        held.notes
            .extend(cut.notes.iter().map(|n| n.diagnostic().note()));
        held.notes.extend(cut.media.iter().map(Diagnostic::note));
        id
    };
    show(viewer, &id);
    let told = cut.notes.len() + cut.media.len();
    format!(
        "{} is imported and open: {} {} from its timesheet, {} frames at {} frames a second, \
         because a timesheet does not say its rate. {}Ctrl+Z takes it all back.",
        cut.name,
        cut.columns.len(),
        if cut.columns.len() == 1 {
            "layer"
        } else {
            "layers"
        },
        cut.duration,
        timesheet::FRAME_RATE,
        match told {
            0 => String::new(),
            1 => "1 note is below. ".to_string(),
            n => format!("{n} notes are below. "),
        }
    )
}

/// Document 24's `media.import`: group the chosen files into one sequence and add it.
///
/// The selection is the person's and is never widened here. Document 07: "Search only
/// user-selected locations" — B-03's importer takes the files it is given and does not look in
/// the folder around them, and this passes on what it was handed.
///
/// Everything the importer said travels to the notes list unchanged, because a gap, a duplicate
/// number or a file that was left out is a fact about the project now on screen and not about
/// the button that was just pressed. The status line gets the summary a person checks a scan
/// against: how many drawings arrived, which numbers they run between, and what is missing.
fn import(viewer: &Mutex<Viewer>, files: &[PathBuf]) -> String {
    // D-71: sound files, each an asset of its own. A choice that mixes sound and drawings
    // falls through to the importer, which says what it could not use.
    if !files.is_empty() && files.iter().all(|f| is_sound(f)) {
        return import_sounds(viewer, files);
    }
    // B-12 fix: one file chosen on its own is a still, as After Effects imports one, shown on
    // every frame of a layer made from it. Grouped as a sequence it was drawing N of a sequence
    // of one, and a layer showed it on no frame or one.
    let exr = |f: &PathBuf| exr_io::is_exr(&f.to_string_lossy());
    if let [file] = files {
        if is_drawing(file) {
            // D-62: an EXR still is read before it is added, so a file this build refuses is
            // not added at all, and what was adjusted is told once, here, as for a sequence.
            let mut told = Vec::new();
            if exr(file) {
                match exr_io::read(file) {
                    Ok(picture) => told.extend(picture.diagnostic(file)),
                    Err(refused) => {
                        return format!("Nothing was imported. {}", refused.sentence());
                    }
                }
            }
            let asset = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                let name = file.file_name().map_or_else(
                    || file.display().to_string(),
                    |n| n.to_string_lossy().into_owned(),
                );
                Asset::still(
                    unused_asset_id(held.document.project()),
                    name,
                    persist::stored_path(&held.root, file),
                )
            };
            let id = asset.id.clone();
            let said = edit(viewer, Command::AddAsset { asset });
            {
                let mut held = viewer.lock().expect("the viewer lock was poisoned");
                if held.document.project().assets.iter().any(|a| a.id == id) {
                    held.notes.extend(told.iter().map(Diagnostic::note));
                }
            }
            return format!(
                "{said}: a still picture, shown on every frame of a layer made from it. To import \
             a sequence, choose all of its files."
            );
        }
    }
    let result = media::import_sequence(files);
    let told: Vec<String> = result
        .diagnostics
        .iter()
        .map(Diagnostic::sentence)
        .collect();
    let Some(sequence) = result.asset else {
        let mut said = match files.len() {
            1 => "The chosen file does not form an image sequence, so nothing was imported."
                .to_string(),
            n => format!(
                "None of the {n} chosen files form an image sequence, so nothing was imported."
            ),
        };
        for note in told {
            said.push(' ');
            said.push_str(&note);
        }
        return said;
    };

    let asset = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let frames = sequence
            .frames()
            .iter()
            .map(|(n, p)| (*n, persist::stored_path(&held.root, p)))
            .collect();
        // The pattern is the name, because it is what the folder shows and what a person
        // recognises. It is a description of the naming, never the authority on which files
        // exist; the frame list is that, and it is what the record carries.
        Asset::sequence(
            unused_asset_id(held.document.project()),
            sequence.pattern(),
            sequence.pattern(),
        )
        .with_frames(frames)
    };
    let id = asset.id.clone();
    let said = edit(viewer, Command::AddAsset { asset });

    {
        let mut held = viewer.lock().expect("the viewer lock was poisoned");
        if !held.document.project().assets.iter().any(|a| a.id == id) {
            return said;
        }
        held.notes
            .extend(result.diagnostics.iter().map(Diagnostic::note));
    }
    let (lo, hi) = sequence
        .range()
        .expect("an imported sequence has at least one drawing");
    let count = sequence.frames().len();
    match sequence.missing().as_slice() {
        [] => format!("{said}: {count} drawings, numbered {lo} to {hi}."),
        [one] => format!(
            "{said}: {count} drawings, numbered {lo} to {hi}, and drawing {one} is missing."
        ),
        many => format!(
            "{said}: {count} drawings, numbered {lo} to {hi}, and {} drawings are missing: {}.",
            many.len(),
            spoken(many)
        ),
    }
}

/// D-71: the sound files the window offers, played by the page's own decoder, so the list is what
/// WebView2 decodes. D-639: the core reads them all but Opus; a video file (`mp4`, `mov`) brings
/// in its first sound track only.
const SOUNDS: [&str; 9] = [
    "wav", "mp3", "ogg", "opus", "flac", "m4a", "aac", "mp4", "mov",
];
const DRAWINGS_AND_SOUND: [&str; 18] = [
    "png", "exr", "bmp", "tga", "tif", "tiff", "webp", "jpg", "jpeg", "wav", "mp3", "ogg", "opus",
    "flac", "m4a", "aac", "mp4", "mov",
];

fn is_sound(file: &Path) -> bool {
    file.extension()
        .is_some_and(|e| SOUNDS.iter().any(|s| e.eq_ignore_ascii_case(s)))
}

/// D-71: one audio asset per chosen file. A WAV the core cannot measure is still imported, with
/// the reason told once, because document 28 keeps the reference and makes the layer silent.
/// D-639: every other format is read here too (and kept for the effects that listen), so an
/// unreadable file or an Opus one is told the same way; a video file says its picture stays out.
fn import_sounds(viewer: &Mutex<Viewer>, files: &[PathBuf]) -> String {
    let mut said = Vec::new();
    let mut warned = String::new();
    let mut video = false;
    for file in files {
        let name = file.file_name().map_or_else(
            || file.display().to_string(),
            |n| n.to_string_lossy().into_owned(),
        );
        let wav = file
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("wav"));
        video |= file
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("mp4") || e.eq_ignore_ascii_case("mov"));
        let told = match wav.then(|| std::fs::read(file).map(|raw| audio::read_wav(&raw))) {
            Some(Ok(Ok(audio::Wav::Read {
                cut_short: true, ..
            }))) => Some(audio::cut_short(&name)),
            Some(Ok(Err(unreadable))) => Some(unreadable),
            Some(_) => None,
            None => audio::load(file).err(),
        };
        let asset = {
            let held = viewer.lock().expect("the viewer lock was poisoned");
            let mut asset = Asset::still(
                unused_asset_id(held.document.project()),
                name,
                persist::stored_path(&held.root, file),
            );
            asset.kind = AssetKind::Audio;
            asset
        };
        let id = asset.id.clone();
        said.push(edit(viewer, Command::AddAsset { asset }));
        let mut held = viewer.lock().expect("the viewer lock was poisoned");
        if held.document.project().assets.iter().any(|a| a.id == id) {
            if let Some(told) = &told {
                warned = format!("{warned} {}", told.sentence());
            }
            held.notes.extend(told.iter().map(Diagnostic::note));
        }
    }
    let picture = if video {
        " Only its sound is brought in; the picture is not."
    } else {
        ""
    };
    format!(
        "{}: sound. Drag it to the layer list, or press Make a layer, to hear it.{picture}{warned}",
        said.join(" ")
    )
}

/// D-71: the bytes of one sound file, for the page's own decoder. It rides on the frame scheme
/// beside `curve`, because it is a question whose answer changes nothing.
fn sound(viewer: &Mutex<Viewer>, query: Option<&str>) -> Response<Vec<u8>> {
    let bytes = (|| {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let id = Id::new(parameter(query, "asset")?);
        let asset = held
            .document
            .project()
            .assets
            .iter()
            .find(|a| a.id == id && a.kind == AssetKind::Audio)?;
        std::fs::read(held.root.join(asset.path.as_ref()?)).ok()
    })();
    match bytes {
        Some(bytes) => allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/octet-stream")
            .body(bytes)
            .expect("build the sound response"),
        None => allow_the_page_to_read_this(Response::builder().status(404))
            .header("content-type", "text/plain; charset=utf-8")
            .body(b"that sound file is not there to read".to_vec())
            .expect("build the missing sound response"),
    }
}

/// D-640: `/mix?from=&frames=`, the sound of the composition on screen for those frames as the
/// core mixes it (solo applied, as the picture is): 32-bit floats, the left channel then the
/// right, at 48,000 a second. `x-left-out` names the layers left out with MEDIA_AUDIO_PREVIEW_ONLY
/// (Opus), which the page still plays from the file itself.
fn mix(viewer: &Mutex<Viewer>, query: Option<&str>) -> Response<Vec<u8>> {
    let from = parameter(query, "from")
        .and_then(|v| v.parse::<i32>().ok())
        .unwrap_or(0);
    // ponytail: at most 10 seconds at 24 a second a call; the page asks for about one.
    let frames = parameter(query, "frames")
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(1)
        .min(240);
    let (project, root, id) = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        (shown(&held), held.root.clone(), held.composition.clone())
    };
    let Some(comp) = project.composition(&id) else {
        return allow_the_page_to_read_this(Response::builder().status(404))
            .header("content-type", "text/plain; charset=utf-8")
            .body(b"no composition is on screen".to_vec())
            .expect("build the missing composition response");
    };
    let (sound, left_out) = anime_compositor::audio::mix(&project, &root, comp, from, frames);
    let opus: Vec<String> = left_out
        .into_iter()
        .filter(|(_, d)| d.id.as_str() == "MEDIA_AUDIO_PREVIEW_ONLY")
        .map(|(layer, _)| layer)
        .collect();
    let body: Vec<u8> = sound
        .iter()
        .flatten()
        .flat_map(|v| (*v as f32).to_le_bytes())
        .collect();
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "application/octet-stream")
        .header("x-left-out", opus.join(","))
        .body(body)
        .expect("build the mix response")
}

/// A relink that has been worked out and is waiting to be agreed to.
///
/// W-02 requires the window to "present changed dimensions, frame range or alpha interpretation
/// before applying the change", so choosing the replacement drawings and relinking to them cannot
/// be one action: the first works out what would happen and leaves it here, and the second either
/// applies it or throws it away. Nothing about the project changes while one of these exists.
struct PendingRelink {
    asset: Id,
    /// The description the person is being asked to agree to, in the words the panel shows.
    said: String,
    candidate: persist::RelinkCandidate,
}

/// D-257: every file the project names that is not on the disk now, with the footage item to
/// relink. Changes nothing. A file replaced under the same name is not gone.
fn files_gone(viewer: &Mutex<Viewer>) -> serde_json::Value {
    let held = viewer.lock().expect("the viewer lock was poisoned");
    let root = held.root.clone();
    held.document
        .project()
        .assets
        .iter()
        .flat_map(|asset| {
            asset
                .files()
                .into_iter()
                .filter(|relative| !root.join(relative).exists())
                .map(|file| serde_json::json!({ "asset": asset.id.as_str(), "name": asset.name, "file": file }))
                .collect::<Vec<_>>()
        })
        .collect()
}

/// D-260: a small picture of a composition's `frame`, or of a footage item's first drawing, each
/// side a whole number of times smaller so it is at most 160 pixels across. Every block of pixels
/// is averaged in linear light, premultiplied, then made 8-bit sRGB as frames are. Drawn on a
/// copy, outside the lock, so the viewer is not held and nothing changes. Sound and lookup files
/// have no picture and are refused.
fn thumbnail(
    viewer: &Mutex<Viewer>,
    item: &Id,
    frame: i32,
) -> Result<(usize, usize, Vec<u8>), String> {
    let (project, root) = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        (held.document.project().clone(), held.root.clone())
    };
    let full = if project.composition(item).is_some() {
        anime_compositor::compose::render_frame(
            &project,
            item,
            frame,
            &root,
            DEFAULT_TILE_SIZE,
            &mut FrameLog::new(0),
        )
        .map_err(|d| d.to_string())?
    } else {
        let asset = project
            .assets
            .iter()
            .find(|a| &a.id == item)
            .ok_or_else(|| format!("no item {}", item.as_str()))?;
        if !matches!(asset.kind, AssetKind::Still | AssetKind::ImageSequence) {
            return Err(format!("{} has no picture", asset.name));
        }
        let first = asset
            .files()
            .first()
            .map(|f| root.join(f))
            .ok_or_else(|| format!("{} has no files", asset.name))?;
        media::decode(&first)
            .map_err(|d| d.to_string())?
            .into_working()
    };
    Ok(export::small(&full, 160))
}

/// D-258: one layer of the composition on screen at `frame`, drawn by itself as W-24's solo would
/// draw it, for the onion skin. Solo is applied to a copy for this one picture: the window's own
/// solo, the screen and the history are untouched, and nothing is cached or exported.
fn alone(
    viewer: &Mutex<Viewer>,
    layer: &Id,
    frame: i32,
    quality: PreviewQuality,
) -> Result<(usize, usize, Vec<u8>), String> {
    let (mut project, composition, root) = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        (
            held.document.project().clone(),
            held.composition.clone(),
            held.root.clone(),
        )
    };
    let comp = project
        .compositions
        .iter_mut()
        .find(|c| c.id == composition)
        .filter(|c| c.layer(layer).is_some())
        .ok_or_else(|| format!("no layer {} in the composition on screen", layer.as_str()))?;
    comp.solo(std::slice::from_ref(layer));
    let picture = preview::preview_frame(
        &project,
        &composition,
        frame,
        &root,
        quality,
        DEFAULT_TILE_SIZE,
        &mut FrameLog::new(0),
    )
    .map_err(|d| d.to_string())?;
    Ok((
        picture.width(),
        picture.height(),
        picture.to_srgb8_straight(),
    ))
}

/// The size of the drawings an asset points at now, or `None` when they cannot be read.
///
/// Relinking is what a person does when the media has gone, so the usual case is that there is
/// nothing to compare against. Saying that is the point: an unanswerable comparison presented as
/// "unchanged" would be a lie in the one place W-02 asks for the truth.
fn size_now(root: &Path, asset: &Asset) -> Option<(usize, usize)> {
    asset
        .frames
        .values()
        .find_map(|relative| media::decode(&root.join(relative)).ok())
        .map(|image| (image.width(), image.height()))
}

/// Work out what relinking `asset` to `files` would do, and hold it until it is agreed to.
fn propose_relink(viewer: &Mutex<Viewer>, asset: &Id, files: &[PathBuf]) -> String {
    // D-71: a sound is one file with no size, range or alpha to compare, so there is nothing to
    // agree to first. It is relinked at once, and Ctrl+Z takes it back. D-182: so is a lookup
    // file.
    let sound = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let found = held
            .document
            .project()
            .assets
            .iter()
            .find(|a| &a.id == asset && matches!(a.kind, AssetKind::Audio | AssetKind::Lut))
            .cloned();
        found.map(|a| (a, held.root.clone()))
    };
    if let Some((mut record, root)) = sound {
        let lookup = record.kind == AssetKind::Lut;
        let [file] = files else {
            let what = if lookup { "lookup file" } else { "sound" };
            return format!("A {what} is one file, so choose one. Nothing was changed.");
        };
        // D-182: a lookup file the reading rule refuses is not linked to.
        if lookup {
            if let Err(why) = read_lookup_file(file, is_amp(file)) {
                return format!(
                    "{} was not used: {why}. Nothing was changed.",
                    file.display()
                );
            }
        } else if !is_sound(file) {
            return "That is not a sound file this window plays. Nothing was changed.".to_string();
        }
        record.path = Some(persist::stored_path(&root, file));
        return edit(
            viewer,
            Command::RelinkAsset {
                asset: Box::new(record),
            },
        );
    }
    let mut held = viewer.lock().expect("the viewer lock was poisoned");
    let candidate =
        match persist::relink_candidate(held.document.project(), asset, files, &held.root) {
            Ok(candidate) => candidate,
            Err(diagnostic) => return diagnostic.sentence(),
        };
    let existing = held
        .document
        .project()
        .assets
        .iter()
        .find(|a| &a.id == asset)
        .cloned()
        .expect("relink_candidate refuses an asset that is not there");
    let was = size_now(&held.root, &existing);

    let count = candidate.asset.frames.len();
    let numbered = match candidate.range {
        // Audit B2: a still stays one picture.
        _ if candidate.asset.kind == AssetKind::Still => "this one picture".to_string(),
        Some((lo, hi)) => format!("{count} drawings, numbered {lo} to {hi}"),
        None => format!("{count} drawings"),
    };
    let gaps = match candidate.missing.as_slice() {
        [] => String::new(),
        [one] => format!(", and drawing {one} would be missing"),
        many => format!(
            ", and {} drawings would be missing: {}",
            many.len(),
            spoken(many)
        ),
    };
    // The three things W-02 names, each stated whether or not it changed. "Unchanged" is as much
    // of an answer as "1920 by 1080 becomes 1280 by 720", and a person about to replace the
    // artwork in a shot needs to be told both.
    let size = match was {
        Some((w, h)) if (w, h) == (candidate.width as usize, candidate.height as usize) => format!(
            "The drawings are {}x{}, the same size as the ones it points at now.",
            candidate.width, candidate.height
        ),
        Some((w, h)) => format!(
            "The drawings are {}x{}, where the ones it points at now are {w}x{h}.",
            candidate.width, candidate.height
        ),
        None => format!(
            "The drawings are {}x{}. What it points at now cannot be read, so there is nothing \
             to compare that with.",
            candidate.width, candidate.height
        ),
    };
    let read_as = |i: &Interpretation| {
        format!(
            "{} with {} alpha",
            match i.color_space {
                ColorSpace::Srgb => "sRGB",
                ColorSpace::LinearLight => "linear light",
            },
            match i.alpha {
                AlphaMode::Straight => "straight",
                AlphaMode::Premultiplied => "premultiplied",
            }
        )
    };
    // D-62: an EXR is always linear light with premultiplied alpha, so relinking between PNG and
    // EXR changes how the pixels are read, and W-02 asks for that to be said first.
    let now = read_as(&candidate.interpretation);
    let interpretation = if candidate.interpretation != existing.interpretation {
        format!(
            "The colour will be read as {now}, where it is read as {} now: an EXR file is \
             always linear light with premultiplied alpha, and a PNG is not.",
            read_as(&existing.interpretation)
        )
    } else if exr_io::is_exr(&candidate.pattern) {
        format!("The colour is read as {now}, as it is now: an EXR file is always read that way.")
    } else {
        format!(
            "The colour is read as {now}, which is what this project already recorded: a PNG \
             does not state either, so relinking does not change how the pixels are read."
        )
    };
    let mut said = format!(
        "Relinking \"{}\" to {} would give it {numbered}{gaps}. {size} {interpretation} The \
         layers using it keep their stacking, transforms, exposures, masks and effects. Nothing \
         has changed yet.",
        existing.name, candidate.pattern,
    );
    for note in candidate.diagnostics.iter().map(Diagnostic::sentence) {
        said.push(' ');
        said.push_str(&note);
    }
    held.relink = Some(PendingRelink {
        asset: asset.clone(),
        said: said.clone(),
        candidate,
    });
    said
}

/// The five effects of document 21, at the settings that change no pixels.
///
/// Adding an effect and setting it are two commands rather than one, so that a stack can be
/// built before any of it is tuned; starting each one at its identity means the picture does
/// not jump the instant an effect is added, and the change a person then sees is the one they
/// typed. Line smoothing is the exception D-86 makes: it starts at softness 50 and threshold 10,
/// because a line with its steps smoothed is the only reason to add it. Selective colour blur
/// starts at blur 12 with no colour chosen (D-87), which changes nothing until one is. Glow
/// starts at After Effects' own defaults (D-89), because a glow nobody can see is no start.
/// Line recolour starts with no colour chosen and red as the new one (D-91). Directional blur
/// starts at a streak of 10 pixels up and down (D-92), as a glow does, so adding it shows it.
/// D-353: a Soft Physical Glow as added.
fn soft_glow_added() -> Effect {
    Effect::SoftGlow {
        falloff: "physical".to_string(),
        threshold_mode: "chroma".to_string(),
        threshold: 0.0,
        threshold_smooth: 0.0,
        saturation_bias: 0.0,
        radius: 500.0,
        exposure: 1.0,
        aspect_ratio: 1.0,
        aspect_angle: 0.0,
        operation: "screen".to_string(),
        source_opacity: 100.0,
        unmult: "on".to_string(),
        // D-542: no tint, the colours ready for one.
        tint: "off".to_string(),
        inner_color: "#ffffff".to_string(),
        outer_color: "#4080ff".to_string(),
        tint_amount: 100.0,
        // D-543: Spectral off, at Deep Glow's sizes.
        spectral: "off".to_string(),
        spectral_red: 80.0,
        spectral_green: 100.0,
        spectral_blue: 120.0,
        // D-545: no dirt.
        dirt: Default::default(),
    }
}

fn new_effect(type_id: &str) -> Option<Effect> {
    match type_id {
        EXPOSURE => Some(Effect::exposure(0.0)),
        GAUSSIAN_BLUR => Some(Effect::GaussianBlur {
            sigma_px: 0.0,
            edges: "transparent".to_string(),
            dimensions: "both".to_string(),
            // D-321: one added from now on takes After Effects' Blurriness.
            units: "blurriness".to_string(),
        }),
        // D-401: After Effects' Tint, black to black and white to white, which turns it grey.
        TINT => Some(Effect::TintMap {
            map_black_to: "#000000".to_string(),
            map_white_to: "#ffffff".to_string(),
            amount_to_tint: 100.0,
        }),
        // D-402: white, a sepia brown of our own and black, nothing blended back.
        TRITONE => Some(Effect::Tritone {
            highlights: "#ffffff".to_string(),
            midtones: "#8c7355".to_string(),
            shadows: "#000000".to_string(),
            blend_with_original: 0.0,
        }),
        // D-403: a pale sky blue of our own at 30, even until a matte layer is chosen.
        AERIAL_HAZE => Some(Effect::AerialHaze {
            haze_color: "#b4c8dc".to_string(),
            amount: 30.0,
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            map: None,
        }),
        // D-408: After Effects' own, which leave the layer as it is until one is changed.
        TRANSFORM => Some(Effect::Transform {
            anchor_point: [50.0, 50.0],
            position: [50.0, 50.0],
            uniform_scale: "on".to_string(),
            scale_height: 100.0,
            scale_width: 100.0,
            skew: 0.0,
            skew_axis: 0.0,
            rotation: 0.0,
            opacity: 100.0,
            use_composition_shutter_angle: "on".to_string(),
            shutter_angle: 0.0,
            sampling: "bilinear".to_string(),
            moments: Vec::new(),
        }),
        LINE_SMOOTH => Some(Effect::LineSmooth {
            softness: 50.0,
            threshold: 10.0,
        }),
        SELECTIVE_COLOR_BLUR => Some(Effect::SelectiveColorBlur {
            blur: 12.0,
            colors: Vec::new(),
            tolerance: 0.0,
        }),
        GLOW => Some(Effect::Glow {
            based_on: "bright".to_string(),
            threshold: 60.0,
            colors: Vec::new(),
            tolerance: 0.0,
            radius: 10.0,
            intensity: 1.0,
            operation: "add".to_string(),
            tint: String::new(),
            // D-322: one added from now on takes After Effects' radius and strength.
            units: "after_effects".to_string(),
            // D-747: and After Effects' starting values for its further controls.
            look: Default::default(),
        }),
        LINE_RECOLOR => Some(Effect::LineRecolor {
            colors: Vec::new(),
            tolerance: 0.0,
            new_color: "#ff0000".to_string(),
        }),
        DIRECTIONAL_BLUR => Some(Effect::DirectionalBlur {
            direction: 0.0,
            length: 10.0,
            edges: "transparent".to_string(),
        }),
        // D-93: no colour chosen, so adding it changes nothing until one is.
        SELECT_COLOR => Some(Effect::SelectColor {
            colors: Vec::new(),
            tolerance: 0.0,
            keep: "chosen".to_string(),
        }),
        // D-94: one pixel thicker, on the whole shape, so adding it shows it.
        LINE_WIDTH => Some(Effect::LineWidth {
            width: 1.0,
            based_on: "shape".to_string(),
            colors: Vec::new(),
            tolerance: 0.0,
        }),
        // D-95: a small spin about the middle, so adding it shows it.
        RADIAL_BLUR => Some(Effect::RadialBlur {
            kind: "spin".to_string(),
            amount: 10.0,
            center: [50.0, 50.0],
            edges: "transparent".to_string(),
        }),
        // D-96: its own starting settings, no streaks.
        BLOOM => Some(Effect::Bloom {
            threshold: 80.0,
            radius: 20.0,
            intensity: 1.0,
            streaks: "none".to_string(),
            length: 60.0,
            angle: 0.0,
        }),
        // D-97: no colour chosen, so adding it changes nothing until one is.
        COLOR_KEY => Some(Effect::ColorKey {
            colors: Vec::new(),
            tolerance: 20.0,
            softness: 20.0,
            match_by: "rgb".to_string(),
        }),
        // D-111: every curve the straight line, which changes nothing.
        CURVES => {
            let straight = || vec![vec![0.0, 0.0], vec![255.0, 255.0]];
            Some(Effect::Curves {
                master: straight(),
                red: straight(),
                green: straight(),
                blue: straight(),
                alpha: straight(),
            })
        }
        // D-112: the full ranges and gamma 1, which change nothing.
        LEVELS => Some(Effect::Levels {
            input_black: 0.0,
            input_white: 255.0,
            gamma: 1.0,
            output_black: 0.0,
            output_white: 255.0,
        }),
        // D-383: every set at its start, which changes nothing.
        LEVELS_INDIVIDUAL => Some(Effect::ChannelLevels {
            individual: true,
            channel: "rgb".into(),
            sets: [LEVELS_PLAIN; 5],
        }),
        // D-113: all three 0, which changes nothing.
        HUE_SATURATION => Some(Effect::HueSaturation {
            hue: 0.0,
            saturation: 0.0,
            lightness: 0.0,
            ranges: [[0.0; 3]; 6],
        }),
        // D-114: its starting settings, white to violet, top to bottom, multiplied at up to 50.
        GRADIENT => Some(Effect::Gradient {
            shape: "linear".to_string(),
            start: [50.0, 0.0],
            end: [50.0, 100.0],
            start_color: "#ffffff".to_string(),
            end_color: "#6450a0".to_string(),
            start_opacity: 0.0,
            end_opacity: 50.0,
            blend: "multiply".to_string(),
        }),
        // D-115: black at half strength, five pixels down and to the right, hard-edged.
        DROP_SHADOW => Some(Effect::DropShadow {
            color: "#000000".to_string(),
            opacity: 50.0,
            direction: 135.0,
            distance: 5.0,
            softness: 0.0,
        }),
        // D-116: a radius of 10, fading at the edges; D-121: a round iris, no highlights.
        LENS_BLUR => Some(Effect::LensBlur {
            radius: 10.0,
            edges: "transparent".to_string(),
            iris: "circle".to_string(),
            roundness: 0.0,
            rotation: 0.0,
            aspect: 1.0,
            highlight_gain: 0.0,
            highlight_threshold: 100.0,
            // D-359: no blur map.
            layer: serde_json::Value::from(""),
            fit: "center".to_string(),
            channel: "luminance".to_string(),
            focal_distance: 0.0,
            invert: "off".to_string(),
            map: None,
        }),
        // D-117: white light from the upper right, three pixels deep.
        RIM_LIGHT => Some(Effect::RimLight {
            color: "#ffffff".to_string(),
            direction: 45.0,
            width: 3.0,
            softness: 1.0,
            intensity: 100.0,
            blend: "normal".to_string(),
        }),
        // D-118: a hard white band three pixels wide.
        OUTLINE => Some(Effect::Outline {
            color: "#ffffff".to_string(),
            width: 3.0,
            softness: 0.0,
            opacity: 100.0,
        }),
        // D-119: a light grey grain that changes every frame.
        NOISE => Some(Effect::Noise {
            amount: 10.0,
            mode: "mono".to_string(),
            seed: 0.0,
            animate: "on".to_string(),
            frame: 0,
        }),
        // D-120: a slight split about the middle; D-409's form, which draws D-120's split
        // until a new setting is moved.
        CHROMATIC_ABERRATION => Some(Effect::LensChromaticAberration {
            mode: "radial".to_string(),
            amount: 3.0,
            center: [50.0, 50.0],
            angle: 90.0,
            falloff: 0.0,
            red_scale: 100.0,
            green_scale: 0.0,
            blue_scale: -100.0,
            fringe_blur: 0.0,
        }),
        // D-123: violet multiplied in at up to 50, ten pixels in from the edge.
        DISTANCE_GRADATION => Some(Effect::DistanceGradation {
            color: "#6450a0".to_string(),
            width: 10.0,
            opacity: 50.0,
            invert: "off".to_string(),
            blend: "multiply".to_string(),
        }),
        // D-423: CC Light Rays' controls, the source a round 50 pixels about the middle, the
        // layer's own colours laid over it; D-124's form only opens from an older project.
        LIGHT_RAYS => Some(Effect::CcLightRays {
            intensity: 100.0,
            center: [50.0, 50.0],
            radius: 50.0,
            warp_softness: 50.0,
            shape: "round".to_string(),
            direction: 0.0,
            color_from_source: "on".to_string(),
            allow_brightening: "on".to_string(),
            color: "#ffffff".to_string(),
            transfer_mode: "none".to_string(),
        }),
        // D-125: a quarter of a stop either way, a new brightness every frame.
        EXPOSURE_FLICKER => Some(Effect::ExposureFlicker {
            amount: 0.25,
            hold: 1.0,
            seed: 0.0,
            frame: 0,
        }),
        // D-126: black, half strength, its outer edge at the corners.
        VIGNETTE => Some(Effect::Vignette {
            amount: 50.0,
            color: "#000000".to_string(),
            size: 100.0,
            roundness: 0.0,
            softness: 50.0,
            center: [50.0, 50.0],
        }),
        // D-562: a full lens falloff about the middle, nothing pinned; CC Vignette's starting
        // values are not published, so these are ours.
        CC_VIGNETTE => Some(Effect::CcVignette {
            amount: 100.0,
            angle_of_view: 90.0,
            center: [50.0, 50.0],
            pin_highlights: 0.0,
        }),
        // D-127: a gentle wobble that moves.
        TURBULENT_DISPLACE => Some(Effect::TurbulentDisplace {
            amount: 10.0,
            size: 60.0,
            complexity: 2.0,
            evolution: 0.0,
            speed: 20.0,
            seed: 0.0,
            edges: "transparent".to_string(),
            frame: 0,
            displacement: "turbulent".to_string(),
            pinning: "none".to_string(),
            // D-328: one added from now on takes After Effects' push, smaller at small sizes.
            units: "after_effects".to_string(),
            new_seed_every: 0.0,
            drift_direction: 0.0,
            drift_speed: 0.0,
        }),
        // D-128: soft grey clouds, still. D-743: Sub Influence 70 and Sub Scaling 56 as After
        // Effects starts them; a file without them still reads 50 and 50.
        FRACTAL_NOISE => Some(Effect::FractalNoise {
            size: 100.0,
            complexity: 4.0,
            contrast: 100.0,
            brightness: 0.0,
            evolution: 0.0,
            speed: 0.0,
            seed: 0.0,
            dark_color: "#000000".to_string(),
            light_color: "#ffffff".to_string(),
            opacity: 100.0,
            blend: "normal".to_string(),
            fractal_type: "basic".to_string(),
            noise_type: "smooth".to_string(),
            invert: "off".to_string(),
            offset: [0.0, 0.0],
            scale_width: 100.0,
            scale_height: 100.0,
            cycle: 0.0,
            frame: 0,
            float: false,
            more: anime_compositor::effects::NoiseMore {
                sub_influence: 70.0,
                sub_scaling: 56.0,
                ..Default::default()
            },
        }),
        // D-129: black, grey and white, which turns the picture grey.
        GRADIENT_MAP => Some(Effect::GradientMap {
            shadow_color: "#000000".to_string(),
            midtone_color: "#808080".to_string(),
            highlight_color: "#ffffff".to_string(),
            midpoint: 50.0,
            amount: 100.0,
        }),
        // D-130: all nine at 0, which changes nothing. D-295: Preserve Luminosity on, as After
        // Effects starts it.
        COLOR_BALANCE => Some(Effect::ColorBalance {
            shadows: vec![0.0; 3],
            midtones: vec![0.0; 3],
            highlights: vec![0.0; 3],
            preserve_luminosity: "on".to_string(),
        }),
        // D-131: no shift, which changes nothing.
        OFFSET => Some(Effect::Offset { shift: [0.0, 0.0] }),
        // D-132: a light reaching about three pixels in, at full strength, screened.
        LIGHT_WRAP => Some(Effect::LightWrap {
            width: 10.0,
            intensity: 100.0,
            blend: "screen".to_string(),
        }),
        // D-134: the colours turned fully to their opposites, a negative.
        INVERT => Some(Effect::Invert {
            channel: "rgb".to_string(),
            amount: 100.0,
        }),
        // D-135: nothing changed until a setting is moved.
        BRIGHTNESS_CONTRAST => Some(Effect::BrightnessContrast {
            brightness: 0.0,
            contrast: 0.0,
        }),
        // D-136: the ranges as a paint program starts them.
        BLACK_WHITE => Some(Effect::BlackWhite {
            reds: 40.0,
            yellows: 60.0,
            greens: 40.0,
            cyans: 60.0,
            blues: 20.0,
            magentas: 80.0,
        }),
        // D-137: six steps per channel, a clear poster look.
        POSTERIZE => Some(Effect::Posterize { levels: 6.0 }),
        // D-138: the middle, 128.
        THRESHOLD => Some(Effect::Threshold { level: 128.0 }),
        // D-139: each channel from itself alone, which changes nothing.
        CHANNEL_MIXER => Some(Effect::ChannelMixer {
            red: vec![100.0, 0.0, 0.0, 0.0],
            green: vec![0.0, 100.0, 0.0, 0.0],
            blue: vec![0.0, 0.0, 100.0, 0.0],
            monochrome: "off".to_string(),
        }),
        // D-140: nothing changed until a setting is moved.
        VIBRANCE => Some(Effect::Vibrance {
            vibrance: 0.0,
            saturation: 0.0,
        }),
        // D-141: the reds kept, everything else grey.
        LEAVE_COLOR => Some(Effect::LeaveColor {
            color: "#ff0000".to_string(),
            tolerance: 15.0,
            softness: 10.0,
            amount: 100.0,
        }),
        // D-142: the middle, 128.
        SOLARIZE => Some(Effect::Solarize { threshold: 128.0 }),
        // D-143: black dots on white, eight pixels apart, turned 45 degrees.
        HALFTONE => Some(Effect::Halftone {
            size: 8.0,
            angle: 45.0,
            ink: "#000000".to_string(),
            paper: "#ffffff".to_string(),
            amount: 100.0,
        }),
        // D-144: blocks ten pixels wide. D-754: as After Effects' starts, 10 by 10 blocks, Sharp
        // Colors off; the size kept for size mode.
        MOSAIC => Some(Effect::Mosaic {
            size: 10.0,
            mode: "count".to_string(),
            horizontal: 10.0,
            vertical: 10.0,
            sharp_colors: "off".to_string(),
        }),
        // D-145: grey relief, lit from the top left.
        EMBOSS => Some(Effect::Emboss {
            direction: 135.0,
            relief: 1.0,
            contrast: 100.0,
            mode: "grey".to_string(),
        }),
        // D-146: dark lines on white, all the way.
        FIND_EDGES => Some(Effect::FindEdges {
            invert: "off".to_string(),
            amount: 100.0,
        }),
        // D-147: a plain crisping.
        SHARPEN => Some(Effect::Sharpen {
            amount: 100.0,
            radius: 1.0,
            threshold: 0.0,
        }),
        // D-148: a soft screen glow, half on.
        DIFFUSION => Some(Effect::Diffusion {
            radius: 10.0,
            amount: 50.0,
            blend: "screen".to_string(),
            second_amount: 0.0,
            second_blend: "soft_light".to_string(),
        }),
        // D-149: a still sine wave, pushing up and down.
        WAVE_WARP => Some(Effect::WaveWarp {
            shape: "sine".to_string(),
            height: 10.0,
            width: 40.0,
            direction: 90.0,
            speed: 0.0,
            phase: 0.0,
            edges: "transparent".to_string(),
            pinning: "none".to_string(),
            frame: 0,
        }),
        // D-150: rings spreading from the middle.
        RIPPLE => Some(Effect::Ripple {
            center: [50.0, 50.0],
            amplitude: 5.0,
            wavelength: 30.0,
            speed: 20.0,
            phase: 0.0,
            fade: 0.0,
            frame: 0,
        }),
        // D-151: a quarter turn at the middle; D-744: out to 30 per cent of the drawing's larger
        // side, After Effects' start, the radius in pixels left at 0.
        TWIRL => Some(Effect::Twirl {
            angle: 90.0,
            radius: 0.0,
            center: [50.0, 50.0],
            twirl_radius: 30.0,
        }),
        // D-152: a gentle swell in the middle, fifty pixels out.
        BULGE => Some(Effect::Bulge {
            center: [50.0, 50.0],
            radius: 50.0,
            height: 1.0,
            vertical_radius: 0.0,
            taper_radius: 0.0,
        }),
        // D-153: the line straight down the middle, the right half kept.
        MIRROR => Some(Effect::Mirror {
            center: [50.0, 50.0],
            angle: 0.0,
        }),
        // D-154: one tile, the drawing itself, until a size is raised.
        MOTION_TILE => Some(Effect::MotionTile {
            output_width: 100.0,
            output_height: 100.0,
            mirror: "off".to_string(),
            tile_center: [50.0, 50.0],
            tile_width: 100.0,
            tile_height: 100.0,
        }),
        // D-155: nothing wiped yet, the edge to come from the left, hard.
        LINEAR_WIPE => Some(Effect::LinearWipe {
            completion: 0.0,
            angle: 90.0,
            feather: 0.0,
        }),
        // D-156: nothing swept yet, from straight up round the middle, clockwise, hard.
        RADIAL_WIPE => Some(Effect::RadialWipe {
            completion: 0.0,
            start_angle: 0.0,
            center: [50.0, 50.0],
            wipe: "clockwise".to_string(),
            feather: 0.0,
        }),
        // D-157: nothing cleared yet, level slats twenty pixels tall, hard.
        VENETIAN_BLINDS => Some(Effect::VenetianBlinds {
            completion: 0.0,
            angle: 0.0,
            width: 20.0,
            feather: 0.0,
        }),
        // D-158: not closed yet, on the middle, hard, the circle closing in.
        IRIS_WIPE => Some(Effect::IrisWipe {
            completion: 0.0,
            center: [50.0, 50.0],
            feather: 0.0,
            invert: "off".to_string(),
        }),
        // D-159: no change yet.
        SIMPLE_CHOKER => Some(Effect::SimpleChoker { choke: 0.0 }),
        // D-160: a hundred and twenty thin black lines on twos, the middle clear.
        SPEED_LINES => Some(Effect::SpeedLines {
            center: [50.0, 50.0],
            color: "#000000".to_string(),
            count: 120.0,
            thickness: 1.5,
            inner: 150.0,
            inner_jitter: 40.0,
            angle_jitter: 50.0,
            seed: 0.0,
            hold: 2.0,
            opacity: 100.0,
            frame: 0,
        }),
        // D-161: a white four-pointed star on the brightest points, turned 45 degrees.
        CROSS_GLARE => Some(Effect::CrossGlare {
            threshold: 80.0,
            length: 40.0,
            points: 4.0,
            angle: 45.0,
            intensity: 1.0,
            color: "#ffffff".to_string(),
            // D-546: one colour, no shimmer.
            glint: Default::default(),
        }),
        // D-162: a ten-pixel jolt every frame, untipped.
        CAMERA_SHAKE => Some(Effect::CameraShake {
            amount: 10.0,
            rotation: 0.0,
            hold: 1.0,
            seed: 0.0,
            frame: 0,
        }),
        // D-163: pale blue streaks falling a little to the right.
        RAIN => Some(Effect::Rain {
            color: "#c8d8ff".to_string(),
            density: 30.0,
            spacing: 24.0,
            length: 20.0,
            width: 1.0,
            direction: 170.0,
            speed: 30.0,
            seed: 0.0,
            opacity: 60.0,
            frame: 0,
        }),
        // D-182: no file chosen, which changes nothing until one is.
        COLOR_LOOKUP => Some(Effect::ColorLookup {
            lut: String::new(),
            table: None,
        }),
        // D-183: a short pass along every line at full strength, over the whole layer.
        LINE_BLUR => Some(Effect::LineBlur {
            length: 4.0,
            strength: 100.0,
            lines_only: "off".to_string(),
        }),
        // D-184: a green screen, its shadow and its spill, where OpenToonz starts at 0.
        HSV_KEY => Some(Effect::HsvKey {
            hue: 120.0,
            saturation: 60.0,
            value: 60.0,
            hue_range: 40.0,
            saturation_range: 40.0,
            value_range: 40.0,
            invert: "off".to_string(),
        }),
        // D-185: a violet shadow multiplied up the lower 70 per cent of the figure.
        PARAFFIN => Some(Effect::Paraffin {
            color: "#6450a0".to_string(),
            direction: 180.0,
            spread: 70.0,
            opacity: 50.0,
            blend: "multiply".to_string(),
        }),
        // D-186: a star on some of the near-white highlights, twinkling once a second.
        KIRA_KIRA => Some(Effect::KiraKira {
            threshold: 95.0,
            spacing: 64.0,
            density: 60.0,
            size: 40.0,
            shape: "star".to_string(),
            angle: 0.0,
            twinkle: 100.0,
            period: 24.0,
            seed: 0.0,
            opacity: 100.0,
            color: "#ffffff".to_string(),
            frame: 0,
        }),
        // D-190: a bolt down the middle of the layer, white in a blue glow, new every two frames.
        LIGHTNING_BOLT => Some(Effect::LightningBolt {
            start: [40.0, 0.0],
            end: [60.0, 100.0],
            jagged: 40.0,
            detail: 6.0,
            branches: 30.0,
            width: 3.0,
            glow: 24.0,
            opacity: 100.0,
            hold: 2.0,
            seed: 0.0,
            color: "#ffffff".to_string(),
            glow_color: "#6e8cff".to_string(),
            // D-300: After Effects' Composite on Original is off when added.
            composite: "off".to_string(),
            kind: "direction".to_string(),
            turbulence: 0.0,
            decay: 0.0,
            conductivity: 0.0,
            obstacle: 0.0,
            path: "split".to_string(),
            core: "hard".to_string(),
            forks: "short".to_string(),
            frame: 0,
        }),
        // D-191: After Effects' Stretch Map to Fit is on when added.
        COMPOUND_BLUR => Some(Effect::CompoundBlur {
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            max_blur: 20.0,
            invert: "off".to_string(),
            edges: "transparent".to_string(),
            map: None,
        }),
        // D-193: After Effects' own: red across, green down, 5 pixels each, stretched.
        DISPLACEMENT_MAP => Some(Effect::DisplacementMap {
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            horizontal: "red".to_string(),
            max_horizontal: 5.0,
            vertical: "green".to_string(),
            max_vertical: 5.0,
            wrap: "off".to_string(),
            expand: "off".to_string(),
            red_amount: 100.0,
            green_amount: 100.0,
            blue_amount: 100.0,
            spectrum: 3.0,
            map: None,
        }),
        // D-194: After Effects' own: nothing wiped yet, a hard edge, stretched.
        GRADIENT_WIPE => Some(Effect::GradientWipe {
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            completion: 0.0,
            softness: 0.0,
            invert: "off".to_string(),
            map: None,
        }),
        // D-195: After Effects' own: one echo a frame back, whole, added.
        ECHO => Some(Effect::Echo {
            echo_time: -1.0,
            echoes: 1.0,
            intensity: 1.0,
            decay: 1.0,
            operator: "add".to_string(),
            picture: None,
        }),
        // D-520: ours (CycoreFX publishes no starting values): three frames each way, the
        // compositions inside drawn without their own motion blur.
        CC_WIDE_TIME => Some(Effect::CcWideTime {
            forward_steps: 3.0,
            backward_steps: 3.0,
            native_motion_blur: "off".to_string(),
            picture: None,
        }),
        // D-581: ours (CycoreFX publishes no starting values): eight moments in the
        // composition's own shutter, the compositions inside keeping their own blur.
        FORCE_MOTION_BLUR => Some(Effect::ForceMotionBlur {
            motion_blur_samples: 8.0,
            override_native_shutter: "off".to_string(),
            shutter_angle: 180.0,
            native_motion_blur: "on".to_string(),
            picture: None,
        }),
        // D-650: After Effects' own as added: the composition's shutter, Vector Detail 20.
        PIXEL_MOTION_BLUR => Some(Effect::PixelMotionBlur {
            shutter_control: "automatic".to_string(),
            shutter_angle: 180.0,
            shutter_samples: 16.0,
            vector_detail: 20.0,
            frames: None,
        }),
        // D-652: the settings when added, as tools/timewarp_reference.py names them: Pixel
        // Motion at half speed, the estimate tuned as Pixel Motion Blur's, no blur.
        TIMEWARP => Some(Effect::Timewarp {
            method: "pixel_motion".to_string(),
            adjust_time_by: "speed".to_string(),
            speed: 50.0,
            source_frame: 0.0,
            vector_detail: 20.0,
            global_smoothness: 0.5,
            local_smoothness: 0.5,
            smoothing_iterations: 4.0,
            build_from_one_image: "off".to_string(),
            correct_luminance_changes: "off".to_string(),
            filtering: "normal".to_string(),
            error_threshold: 1.0,
            block_size: 6.0,
            red_weight: 0.3,
            green_weight: 0.6,
            blue_weight: 0.1,
            enable_motion_blur: "off".to_string(),
            shutter_control: "automatic".to_string(),
            shutter_angle: 180.0,
            shutter_samples: 5.0,
            picture: None,
        }),
        // D-653: the settings when added, as tools/warp_stabilizer_reference.py names them:
        // Smooth Motion at 50, Position, Scale, Rotation, cropped and scaled up to 150 per cent.
        WARP_STABILIZER => Some(Effect::WarpStabilizer {
            result: "smooth_motion".to_string(),
            smoothness: 50.0,
            method: "position_scale_rotation".to_string(),
            preserve_scale: "off".to_string(),
            framing: "stabilize_crop_auto_scale".to_string(),
            maximum_scale: 150.0,
            action_safe_margin: 0.0,
            additional_scale: 100.0,
            detailed_analysis: "off".to_string(),
            picture: None,
        }),
        // D-654: the settings when added, as tools/rolling_shutter_repair_reference.py names them:
        // 50 per cent, Top to Bottom, Warp, Detailed Analysis off, Pixel Motion Detail 20.
        ROLLING_SHUTTER_REPAIR => Some(Effect::RollingShutterRepair {
            rolling_shutter_rate: 50.0,
            scan_direction: "top_to_bottom".to_string(),
            method: "warp".to_string(),
            detailed_analysis: "off".to_string(),
            pixel_motion_detail: 20.0,
            picture: None,
        }),
        // D-582: ours, Drops 5000 after Adobe's rain tutorial and the rest ours: fine pale
        // drops through a deep volume, laid over the layer with Lighten.
        CC_RAINFALL => Some(Effect::CcRainfall {
            drops: 5000.0,
            size: 3.0,
            scene_depth: 5000.0,
            speed: 4000.0,
            wind: 0.0,
            wind_variation: 0.0,
            spread: 15.0,
            color: "#b4bec8".to_string(),
            opacity: 25.0,
            transfer_mode: "lighten".to_string(),
            composite_with_original: "on".to_string(),
            ground_level: 100.0,
            random_seed: 0.0,
            clock: [0.0, 0.0],
        }),
        // D-630: ours (Adobe gives no values we found): a 60-cell grid, one small ring at the
        // middle beating once a second, no ground layer, so open water everywhere.
        WAVE_WORLD => Some(Effect::WaveWorld {
            brightness: 0.5,
            contrast: 0.5,
            gamma: 1.0,
            render_dry_areas: "level".to_string(),
            transparency: 0.0,
            grid_resolution: 60.0,
            grid_res_downsamples: "off".to_string(),
            wave_speed: 0.25,
            damping: 0.05,
            reflect_edges: "none".to_string(),
            pre_roll: 0.0,
            ground_layer: serde_json::Value::from(""),
            ground_map: None,
            steepness: 0.25,
            height: 0.5,
            wave_strength: 0.0,
            producer_1_type: "ring".to_string(),
            producer_1_position: vec![50.0, 50.0],
            producer_1_height: 0.1,
            producer_1_width: 0.1,
            producer_1_angle: 0.0,
            producer_1_amplitude: 0.5,
            producer_1_frequency: 1.0,
            producer_1_phase: 0.0,
            producer_2_type: "ring".to_string(),
            producer_2_position: vec![75.0, 25.0],
            producer_2_height: 0.1,
            producer_2_width: 0.1,
            producer_2_angle: 0.0,
            producer_2_amplitude: 0.0,
            producer_2_frequency: 1.0,
            producer_2_phase: 0.0,
            view: "height_map".to_string(),
            horizontal_rotation: 0.0,
            vertical_rotation: 30.0,
            vertical_scale: 1.0,
            history: Vec::new(),
            fps: 0.0,
            grid_scale: 1.0,
        }),
        // D-633: ours (Adobe gives few values): ten bricks across, broken by one force at the centre
        // and thrown down by gravity, seen by its own camera.
        SHATTER => Some(Effect::Shatter {
            view: "rendered".to_string(),
            render: "all".to_string(),
            pattern: "bricks".to_string(),
            custom_shatter_map: serde_json::Value::from(""),
            custom_map: None,
            white_tiles_fixed: "off".to_string(),
            repetitions: 10.0,
            direction: 0.0,
            origin: [50.0, 50.0],
            extrusion_depth: 0.2,
            force_1_position: [50.0, 50.0],
            force_1_depth: 0.1,
            force_1_radius: 0.4,
            force_1_strength: 5.0,
            force_2_position: [50.0, 50.0],
            force_2_depth: 0.1,
            force_2_radius: 0.0,
            force_2_strength: 5.0,
            gradient_layer: serde_json::Value::from(""),
            gradient_map: None,
            shatter_threshold: 0.0,
            invert_gradient: "off".to_string(),
            rotation_speed: 0.2,
            tumble_axis: "free".to_string(),
            randomness: 0.1,
            viscosity: 0.1,
            mass_variance: 30.0,
            gravity: 3.0,
            gravity_direction: 180.0,
            gravity_inclination: 0.0,
            color: "#ffffff".to_string(),
            opacity: 1.0,
            front_mode: "layer".to_string(),
            side_mode: "tinted_layer".to_string(),
            back_mode: "layer".to_string(),
            front_layer: serde_json::Value::from(""),
            front_map: None,
            side_layer: serde_json::Value::from(""),
            side_map: None,
            back_layer: serde_json::Value::from(""),
            back_map: None,
            camera_system: "camera_position".to_string(),
            camera_x_rotation: 0.0,
            camera_y_rotation: 0.0,
            camera_z_rotation: 0.0,
            camera_xy_position: [50.0, 50.0],
            camera_z_position: 1.0,
            focal_length: 50.0,
            light_type: "distant".to_string(),
            light_intensity: 1.0,
            light_color: "#ffffff".to_string(),
            light_position: [50.0, 50.0],
            light_depth: 1.0,
            ambient_light: 0.25,
            diffuse_reflection: 0.75,
            specular_reflection: 0.0,
            upper_left_corner: [0.0, 0.0],
            upper_right_corner: [100.0, 0.0],
            lower_left_corner: [0.0, 100.0],
            lower_right_corner: [100.0, 100.0],
            auto_focal_length: "off".to_string(),
            highlight_sharpness: 5.0,
            seen: [0.0; 3],
            history: Vec::new(),
            fps: 0.0,
        }),
        // D-632: ours (Adobe gives few values): eight by eight cards, each moved toward the viewer
        // by the brightness of the layer's own drawing under it, seen by its own camera.
        CARD_DANCE => Some(Effect::CardDance {
            rows_columns: "independent".to_string(),
            rows: 8.0,
            columns: 8.0,
            back_layer: serde_json::Value::from(""),
            back_map: None,
            gradient_layer_1: serde_json::Value::from(""),
            gradient_map_1: None,
            gradient_layer_2: serde_json::Value::from(""),
            gradient_map_2: None,
            rotation_order: "xyz".to_string(),
            transformation_order: "scale_rotate_position".to_string(),
            sources: CARD_DANCE_TRANSFORMS.map(|t| {
                if t == "z_position" {
                    "intensity_1"
                } else {
                    "none"
                }
                .to_string()
            }),
            multipliers: [1.0, 1.0, 1.0, 90.0, 90.0, 90.0, 0.5, 0.5],
            offsets: [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 1.0, 1.0],
            camera_system: "camera_position".to_string(),
            camera_x_rotation: 0.0,
            camera_y_rotation: 0.0,
            camera_z_rotation: 0.0,
            camera_xy_position: [50.0, 50.0],
            camera_z_position: 1.0,
            focal_length: 50.0,
            light_type: "distant".to_string(),
            light_intensity: 1.0,
            light_color: "#ffffff".to_string(),
            light_position: [50.0, 50.0],
            light_depth: 1.0,
            ambient_light: 0.25,
            diffuse_reflection: 0.75,
            specular_reflection: 0.0,
            highlight_sharpness: 5.0,
            upper_left_corner: [0.0, 0.0],
            upper_right_corner: [100.0, 0.0],
            lower_left_corner: [0.0, 100.0],
            lower_right_corner: [100.0, 100.0],
            auto_focal_length: "off".to_string(),
            view: [0.0; 3],
        }),
        // D-631: ours (Adobe gives few values): the layer's own drawing as the bottom, no water
        // surface yet (flat water until one is chosen), a light blue surface at 30%, no caustics.
        CAUSTICS => Some(Effect::Caustics {
            bottom_layer: serde_json::Value::from(""),
            bottom_map: None,
            bottom_scaling: 1.0,
            bottom_repeat_mode: "tiled".to_string(),
            bottom_fit: "stretch".to_string(),
            bottom_blur: 0.0,
            water_surface: serde_json::Value::from(""),
            surface_map: None,
            wave_height: 0.2,
            smoothing: 5.0,
            water_depth: 0.1,
            refractive_index: 1.2,
            surface_color: "#2a7fd4".to_string(),
            surface_opacity: 0.3,
            caustics_strength: 0.0,
            sky_layer: serde_json::Value::from(""),
            sky_map: None,
            sky_scaling: 1.0,
            sky_repeat_mode: "tiled".to_string(),
            sky_fit: "stretch".to_string(),
            sky_intensity: 0.3,
            sky_convergence: 0.2,
            light_type: "distant".to_string(),
            light_intensity: 1.0,
            light_color: "#ffffff".to_string(),
            light_position: vec![50.0, 50.0],
            light_height: 1.0,
            ambient_light: 0.25,
            diffuse_reflection: 0.75,
            specular_reflection: 0.0,
            highlight_sharpness: 5.0,
        }),
        // D-521: ours (Adobe gives no values we found): the layer against itself now, which is
        // black until Time Offset is set; Absolute Difference on, the covering the layer's own.
        TIME_DIFFERENCE => Some(Effect::TimeDifference {
            layer: serde_json::Value::from(""),
            time_offset: 0.0,
            contrast: 0.0,
            absolute_difference: "on".to_string(),
            alpha_channel: TDIFF_ALPHAS[0].to_string(),
            map: None,
        }),
        // D-196: on twos in a composition of 24.
        POSTERIZE_TIME => Some(Effect::PosterizeTime { frame_rate: 12.0 }),
        // D-197: After Effects' own start, pure red to a sky blue, the hue alone.
        CHANGE_TO_COLOR => Some(Effect::ChangeToColor {
            from: "#ff0000".to_string(),
            to: "#0080ff".to_string(),
            change: "hue".to_string(),
            change_by: "setting".to_string(),
            hue_tolerance: 5.0,
            lightness_tolerance: 50.0,
            saturation_tolerance: 50.0,
            softness: 50.0,
            view_matte: "off".to_string(),
        }),
        // D-198: every corner where it is, so the drawing is unchanged until one is moved.
        CORNER_PIN => Some(Effect::CornerPin {
            upper_left: [0.0, 0.0],
            upper_right: [100.0, 0.0],
            lower_left: [0.0, 100.0],
            lower_right: [100.0, 100.0],
        }),
        // D-199: After Effects' own: a soft white band fifty pixels wide, leaning a little left.
        LIGHT_SWEEP => Some(Effect::LightSweep {
            center: [50.0, 50.0],
            direction: -30.0,
            shape: "smooth".to_string(),
            width: 50.0,
            sweep_intensity: 50.0,
            edge_intensity: 100.0,
            edge_thickness: 1.0,
            light_color: "#ffffff".to_string(),
            light_reception: "add".to_string(),
        }),
        // D-200: After Effects' own: a white circle from the middle every 24 frames, growing 5
        // pixels a frame for 96 frames and fading over the last 48.
        RADIO_WAVES => Some(Effect::RadioWaves {
            producer_point: [50.0, 50.0],
            sides: 64.0,
            interval: 24.0,
            expansion: 5.0,
            orientation: 0.0,
            direction: 90.0,
            velocity: 0.0,
            spin: 0.0,
            lifespan: 96.0,
            opacity: 100.0,
            fade_in_time: 0.0,
            fade_out_time: 48.0,
            start_width: 5.0,
            end_width: 5.0,
            profile: "square".to_string(),
            color: "#ffffff".to_string(),
            frame: 0,
        }),
        // D-201: all the way round, so the effect shows as soon as it is added; After Effects
        // starts at 0, which shows nothing. D-320: a circle, After Effects' look.
        POLAR_COORDINATES => Some(Effect::PolarCoordinates {
            interpolation: 100.0,
            conversion: "rect_to_polar".to_string(),
            shape: "circle".to_string(),
        }),
        // D-203: After Effects' Smart Blur starts at a radius of 3 and a threshold of 25 in 100,
        // about 64 in 255.
        MEDIAN => Some(Effect::Median {
            radius: 2.0,
            operate_on_alpha: "off".to_string(),
        }),
        // D-453: the smallest disc, and every speck that differs at all goes.
        DUST_SCRATCHES => Some(Effect::DustScratches {
            radius: 1.0,
            threshold: 0.0,
            operate_on_alpha: "off".to_string(),
        }),
        // D-642: the gate's starting settings, document 21's.
        NOISE_GATE => Some(Effect::NoiseGate {
            threshold: -60.0,
            attack: 10.0,
            hold: 40.0,
            release: 220.0,
        }),
        // D-643: the compressor's starting settings, After Effects 26.0's.
        // D-644: the distortion's starting settings, After Effects 26.0's.
        DISTORTION => Some(Effect::Distortion {
            kind: "soft_clip".to_string(),
            drive: 25.0,
            gain: 25.0,
            wet: 100.0,
            volume: 11.0,
            resolution: 24.0,
            downsample: 1.0,
        }),
        COMPRESSOR => Some(Effect::Compressor {
            threshold: -16.0,
            ratio: 3.0,
            knee: 15.0,
            attack: 6.0,
            release: 440.0,
            auto_release: "on".to_string(),
            makeup: 0.0,
            output_limit: 0.0,
        }),
        // D-447: After Effects' Brush Strokes as it is added; the seed and Animate are ours.
        BRUSH_STROKES => Some(Effect::BrushStrokes {
            stroke_angle: 135.0,
            brush_size: 2.0,
            stroke_length: 8.0,
            stroke_density: 1.0,
            stroke_randomness: 1.0,
            paint_surface: "original".to_string(),
            blend_with_original: 0.0,
            random_seed: 0.0,
            animate: "on".to_string(),
            frame: 0,
        }),
        // D-448: Cartoon as it is added; After Effects' starting values are not published, so
        // these are ours.
        // D-449: Block Load as it is added: Start Cleared on, as After Effects starts it; Scans
        // and Bilinear are ours.
        BLOCK_LOAD => Some(Effect::BlockLoad {
            completion: 0.0,
            scans: 4.0,
            start_cleared: "on".to_string(),
            bilinear: "off".to_string(),
            pixel: 1.0,
        }),
        // D-460: Burn Film as it is added, nothing burned yet, from the middle.
        BURN_FILM => Some(Effect::BurnFilm {
            burn: 0.0,
            center: [50.0, 50.0],
            seed: 0.0,
        }),
        // D-461: Hex Tile as it is added: Tile, radius 50, from the middle, unlocked; After
        // Effects' starting values are not published, so these are ours.
        HEX_TILE => Some(Effect::HexTile {
            render: "tile".to_string(),
            radius: 50.0,
            center: [50.0, 50.0],
            lock_center: "off".to_string(),
            rotate: 0.0,
            smearing: 0.0,
        }),
        // D-462: Mr. Smoothie as it is added: the layer itself, Luminance (After Effects' own
        // start), softened by 5, from a quarter to three quarters across the middle, ABA; the rest
        // of After Effects' starting values are not published, so these are ours.
        MR_SMOOTHIE => Some(Effect::MrSmoothie {
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            property: "luminance".to_string(),
            smoothness: 5.0,
            sample_a: [25.0, 50.0],
            sample_b: [75.0, 50.0],
            phase: 0.0,
            color_loop: "aba".to_string(),
            map: None,
        }),
        // D-463: Plastic as it is added: the layer's own covering as the bump, softened by 10,
        // Height 50, cut 0 to 100, and Blobbylize's light and shading but Metal 0, for plastic;
        // After Effects' starting values are not published, so these are ours.
        PLASTIC => Some(Effect::Plastic {
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            property: "alpha".to_string(),
            softness: 10.0,
            height: 50.0,
            cut_min: 0.0,
            cut_max: 100.0,
            light_intensity: 100.0,
            light_color: "#ffffff".to_string(),
            light_type: "distant".to_string(),
            light_height: 100.0,
            light_position: [30.0, 30.0],
            light_direction: -45.0,
            ambient_light_color: "#ffffff".to_string(),
            ambient: 25.0,
            diffuse: 75.0,
            dust: 0.0,
            specular: 50.0,
            roughness: 0.05,
            metal: 0.0,
            map: None,
        }),
        // D-464: Repeat Tile as it is added: nothing grown, Unfold, no blending; After Effects'
        // starting values are not published, so these are ours.
        REPEAT_TILE => Some(Effect::RepeatTile {
            expand_right: 0.0,
            expand_left: 0.0,
            expand_down: 0.0,
            expand_up: 0.0,
            tiling: "unfold".to_string(),
            blend_borders: 0.0,
        }),
        // D-465: Scatter as it is added: amount 10 as Qt's port of After Effects' Scatter starts;
        // both ways and the same jumble every frame are this program's own choices.
        SCATTER => Some(Effect::Scatter {
            scatter_amount: 10.0,
            grain: "both".to_string(),
            randomize: "off".to_string(),
            frame: 0,
        }),
        // D-466: Strobe Light as it is added: Copy as After Effects starts; white, a tenth of a
        // second every second, at full strength and no random strobes are this program's own.
        STROBE_LIGHT => Some(Effect::StrobeLight {
            strobe_color: "#ffffff".to_string(),
            blend_with_original: 0.0,
            strobe_duration: 0.1,
            strobe_period: 1.0,
            random_strobe_probability: 0.0,
            strobe: "color_only".to_string(),
            strobe_operator: "copy".to_string(),
            random_seed: 0.0,
            time: 0.0,
            count: 0,
        }),
        // D-468: Threshold's 128 on luminance, so it starts as Threshold does; CC Threshold's
        // starting values are not published, so these are ours.
        CHANNEL_THRESHOLD => Some(Effect::ChannelThreshold {
            threshold: 128.0,
            channel: "luminance".to_string(),
            invert: "off".to_string(),
            blend_with_original: 0.0,
        }),
        // D-469: every level 128 and nothing turned over, as Channel Threshold's RGB; CC Threshold
        // RGB's starting values are not published, so these are ours.
        THRESHOLD_RGB => Some(Effect::ThresholdRgb {
            red_threshold: 128.0,
            green_threshold: 128.0,
            blue_threshold: 128.0,
            invert_red: "off".to_string(),
            invert_green: "off".to_string(),
            invert_blue: "off".to_string(),
            blend_with_original: 0.0,
        }),
        // D-470: Contour Paint as it is added: the final result, Stylization and Cleanliness 50,
        // three smears, no fills (K-Means, 8 colours, the whole strength range ready), no turn,
        // no snapping and no edge map; After Effects' starting values are not published, so
        // these are ours.
        CONTOUR_PAINT => Some(Effect::ContourPaint {
            view: "final_result".to_string(),
            stylization: 50.0,
            cleanliness: 50.0,
            smear_iterations: 3.0,
            apply_to_alpha: "off".to_string(),
            draw_color_fills: "none".to_string(),
            reduction_method: "k_means".to_string(),
            posterization_level: 8.0,
            low_strength: 0.0,
            high_strength: 100.0,
            offset_flow_angles: 0.0,
            quantize_flow_angles: 0.0,
            edge_map_layer: serde_json::Value::from(""),
            colors: Vec::new(),
            // D-585: the arrows' heads on as it is added, as After Effects' Edge Map shows them.
            arrowheads: "on".to_string(),
            px: 1.0,
            palette: None,
            map: None,
        }),
        // D-467: Texturize as it is added: no texture until one is picked, Tile Texture, the light
        // from the top left and Texture Contrast 1; After Effects' starting values are not
        // published, so these are ours.
        TEXTURIZE => Some(Effect::Texturize {
            layer: serde_json::Value::from(""),
            fit: "tile".to_string(),
            light_direction: 135.0,
            texture_contrast: 1.0,
            map: None,
        }),
        CARTOON => Some(Effect::Cartoon {
            render: "fill_and_edges".to_string(),
            detail_radius: 8.0,
            detail_threshold: 10.0,
            shading_steps: 8.0,
            shading_smoothness: 40.0,
            edge_threshold: 1.6,
            edge_width: 1.5,
            edge_softness: 60.0,
            edge_opacity: 100.0,
            edge_enhancement: 0.0,
            edge_black_level: 0.0,
            edge_contrast: 0.5,
        }),
        SMART_BLUR => Some(Effect::SmartBlur {
            radius: 3.0,
            threshold: 64.0,
        }),
        // D-358: our own starting values, a light smoothing of skin and grain that keeps lines.
        BILATERAL_BLUR => Some(Effect::BilateralBlur {
            radius: 5.0,
            threshold: 20.0,
            colorize: "on".to_string(),
        }),
        // D-360..D-362: our own starting values, a soft cross, a light zoom and a fading streak.
        CROSS_BLUR => Some(Effect::CrossBlur {
            radius_x: 10.0,
            radius_y: 10.0,
            mode: "blend".to_string(),
            edges: "transparent".to_string(),
        }),
        SPIN_ZOOM_BLUR => Some(Effect::SpinZoomBlur {
            kind: "straight_zoom".to_string(),
            amount: 10.0,
            quality: 50.0,
            center: [50.0, 50.0],
        }),
        FAST_ZOOM_BLUR => Some(Effect::FastZoomBlur {
            amount: 50.0,
            center: [50.0, 50.0],
            zoom: "standard".to_string(),
        }),
        // D-365: as After Effects starts Broadcast Colors, NTSC, darkened, 110 IRE.
        BROADCAST_SAFE => Some(Effect::BroadcastSafe {
            locale: "ntsc".to_string(),
            method: "reduce_luminance".to_string(),
            max_amplitude: 110.0,
        }),
        // D-366: each unbalance colour already grey and nothing added, which changes nothing.
        COLOR_NEUTRALIZER => Some(Effect::ColorNeutralizer {
            shadows_unbalance: "#000000".to_string(),
            midtones_unbalance: "#808080".to_string(),
            highlights_unbalance: "#ffffff".to_string(),
            shadows: vec![0.0; 3],
            midtones: vec![0.0; 3],
            highlights: vec![0.0; 3],
            pinning: 0.0,
            black_point: 0.0,
            white_point: 255.0,
        }),
        // D-367: no phase, wrapping, which changes nothing.
        COLOR_OFFSET => Some(Effect::ColorOffset {
            red_phase: 0.0,
            green_phase: 0.0,
            blue_phase: 0.0,
            overflow: "wrap".to_string(),
        }),
        // D-368: the middle pixel alone, which changes nothing.
        KERNEL => Some(Effect::Kernel {
            line_1: vec![0.0; 3],
            line_2: vec![0.0, 1.0, 0.0],
            line_3: vec![0.0; 3],
            divider: 1.0,
            absolute_values: "off".to_string(),
        }),
        // D-369: a warm sepia tritone, our own starting colours.
        TONER => Some(Effect::Toner {
            tones: "tritone".to_string(),
            highlights: "#ffffff".to_string(),
            brights: "#e0cfb0".to_string(),
            midtones: "#8c7355".to_string(),
            darktones: "#46382a".to_string(),
            shadows: "#000000".to_string(),
        }),
        // D-370: red matched by hue at 15, every transform 0, which changes nothing.
        CHANGE_COLOR => Some(Effect::ChangeColor {
            view: "corrected".to_string(),
            hue_transform: 0.0,
            lightness_transform: 0.0,
            saturation_transform: 0.0,
            color_to_change: "#ff0000".to_string(),
            tolerance: 15.0,
            softness: 0.0,
            match_colors: "hue".to_string(),
            invert_mask: "off".to_string(),
        }),
        // D-374: every setting 0, which changes nothing.
        COLOR_BALANCE_HLS => Some(Effect::ColorBalanceHls {
            hue: 0.0,
            lightness: 0.0,
            saturation: 0.0,
        }),
        // D-382: no stretch, every gamma and gain 1, every pedestal 0, which changes nothing.
        GAMMA_PEDESTAL_GAIN => Some(Effect::GammaPedestalGain {
            black_stretch: 1.0,
            gamma: [1.0; 3],
            pedestal: [0.0; 3],
            gain: [1.0; 3],
        }),
        // D-397: every setting at its start and no look, which changes nothing.
        COLOR_GRADE => Some(Effect::ColorGrade {
            values: COLOR_GRADE_SETTINGS.map(|s| s.3),
            look: String::new(),
            curves: std::array::from_fn(|_| vec![vec![0.0, 0.0], vec![255.0, 255.0]]),
            hue_saturation: Vec::new(),
            more_curves: Default::default(),
            secondary_wheels: "one".to_string(),
            key_invert: "off".to_string(),
            key_view: "off".to_string(),
            table: None,
        }),
        // D-384: After Effects' own start, Warming Filter (85) at density 25, luminosity kept.
        PHOTO_FILTER => Some(Effect::PhotoFilter {
            filter: "warming_85".to_string(),
            color: "#ec8a00".to_string(),
            density: 25.0,
            preserve_luminosity: "on".to_string(),
        }),
        // D-375: the layer's own average, clipped 5 per cent each end, at full opacity.
        COLOR_LINK => Some(Effect::ColorLink {
            layer: serde_json::json!(""),
            sample: "average".to_string(),
            clip: 5.0,
            stencil: "off".to_string(),
            opacity: 100.0,
            blending_mode: "normal".to_string(),
            map: None,
        }),
        // D-376: brightness held to frame 0, the points a quarter, half and three quarters across.
        COLOR_STABILIZER => Some(Effect::ColorStabilizer {
            stabilize: "brightness".to_string(),
            reference_frame: 0.0,
            black_point: [25.0, 50.0],
            mid_point: [50.0, 50.0],
            white_point: [75.0, 50.0],
            sample_size: 5.0,
            reference: None,
        }),
        // D-204: white flakes, about After Effects' CC Snowfall's look as it starts.
        SNOWFALL => Some(Effect::Snowfall {
            color: "#ffffff".to_string(),
            density: 50.0,
            spacing: 32.0,
            size: 6.0,
            depth: 50.0,
            speed: 2.0,
            wind: 0.5,
            wiggle: 3.0,
            period: 48.0,
            seed: 0.0,
            opacity: 100.0,
            frame: 0,
        }),
        // D-205: six mirrored wedges round the middle, as After Effects' CC Kaleida starts.
        KALEIDOSCOPE => Some(Effect::Kaleidoscope {
            segments: 6.0,
            rotation: 0.0,
            size: 100.0,
            center: [50.0, 50.0],
            mode: "mirror".to_string(),
        }),
        // D-206: small rough bites up to eight pixels deep, the band a rust brown.
        ROUGHEN_EDGES => Some(Effect::RoughenEdges {
            edge_type: "roughen".to_string(),
            edge_color: "#8a3c14".to_string(),
            border: 8.0,
            size: 10.0,
            complexity: 3.0,
            evolution: 0.0,
            speed: 0.0,
            seed: 0.0,
            frame: 0,
        }),
        // D-207: a short white-and-blue shot at the left of a line across the middle.
        BEAM => Some(Effect::Beam {
            start: [10.0, 50.0],
            end: [90.0, 50.0],
            length: 25.0,
            time: 0.0,
            start_thickness: 8.0,
            end_thickness: 8.0,
            softness: 50.0,
            inside_color: "#ffffff".to_string(),
            outside_color: "#3c8cff".to_string(),
            composite: "on".to_string(),
        }),
        // D-208: After Effects' four colours at the four corners.
        FOUR_COLOR_GRADIENT => Some(Effect::FourColorGradient {
            point_1: [10.0, 10.0],
            point_2: [90.0, 10.0],
            point_3: [10.0, 90.0],
            point_4: [90.0, 90.0],
            color_1: "#ffff00".to_string(),
            color_2: "#00ff00".to_string(),
            color_3: "#ff00ff".to_string(),
            color_4: "#0000ff".to_string(),
            blend: 100.0,
            opacity: 100.0,
            blending_mode: "normal".to_string(),
        }),
        // D-209: After Effects' own start, bubbles sixty pixels a cell, black to white.
        CELL_PATTERN => Some(Effect::CellPattern {
            pattern: "bubbles".to_string(),
            invert: "off".to_string(),
            contrast: 100.0,
            disperse: 1.0,
            size: 60.0,
            evolution: 0.0,
            seed: 0.0,
            dark_color: "#000000".to_string(),
            light_color: "#ffffff".to_string(),
            opacity: 100.0,
            blend: "normal".to_string(),
        }),
        // D-210: After Effects' own start, a field of view of 0, which leaves the layer as it is.
        OPTICS_COMPENSATION => Some(Effect::OpticsCompensation {
            field_of_view: 0.0,
            reverse: "off".to_string(),
            orientation: "horizontal".to_string(),
            center: [50.0, 50.0],
        }),
        // D-211: black at half, the light at the top of the middle, a wall a tenth away.
        RADIAL_SHADOW => Some(Effect::RadialShadow {
            color: "#000000".to_string(),
            opacity: 50.0,
            light: [50.0, 0.0],
            distance: 10.0,
            softness: 0.0,
            render: "regular".to_string(),
            color_influence: 100.0,
            shadow_only: "off".to_string(),
        }),
        // D-212: everything kept, so adding it changes nothing until a point moves.
        EXTRACT => Some(Effect::Extract {
            channel: "luminance".to_string(),
            black_point: 0.0,
            white_point: 255.0,
            black_softness: 0.0,
            white_softness: 0.0,
            invert: "off".to_string(),
        }),
        // D-213: After Effects' own defaults, a white light at the upper left.
        BEVEL_ALPHA => Some(Effect::BevelAlpha {
            edge_thickness: 2.0,
            light_angle: -60.0,
            light_color: "#ffffff".to_string(),
            light_intensity: 0.4,
        }),
        BEVEL_EDGES => Some(Effect::BevelEdges {
            edge_thickness: 0.1,
            light_angle: -60.0,
            light_color: "#ffffff".to_string(),
            light_intensity: 0.4,
        }),
        // D-214: nothing gone yet, pixel-sized blocks, as After Effects starts.
        BLOCK_DISSOLVE => Some(Effect::BlockDissolve {
            completion: 0.0,
            block_width: 1.0,
            block_height: 1.0,
            feather: 0.0,
        }),
        // D-305: each channel from itself, which changes nothing.
        SHIFT_CHANNELS => Some(Effect::ShiftChannels {
            take_alpha: "alpha".to_string(),
            take_red: "red".to_string(),
            take_green: "green".to_string(),
            take_blue: "blue".to_string(),
        }),
        // D-312: After Effects' start, the layer over white.
        SOLID_COMPOSITE => Some(Effect::SolidComposite {
            source_opacity: 100.0,
            color: "#ffffff".to_string(),
            opacity: 100.0,
            blend: "normal".to_string(),
        }),
        // D-313: no blur, which changes nothing.
        CHANNEL_BLUR => Some(Effect::ChannelBlur {
            red_blurriness: 0.0,
            green_blurriness: 0.0,
            blue_blurriness: 0.0,
            alpha_blurriness: 0.0,
            edges: "transparent".to_string(),
            dimensions: "both".to_string(),
            // D-380: one added from now on takes After Effects' Blurriness, as D-321's.
            units: "blurriness".to_string(),
        }),
        // D-327: radius 0, which changes nothing, and After Effects' 3 iterations.
        FAST_BOX_BLUR => Some(Effect::FastBoxBlur {
            radius: 0.0,
            iterations: 3.0,
            edges: "transparent".to_string(),
            dimensions: "both".to_string(),
        }),
        // D-471: Blurriness 0, which changes nothing, After Effects' start.
        FAST_BLUR_LEGACY => Some(Effect::FastBlurLegacy {
            blurriness: 0.0,
            edges: "transparent".to_string(),
            dimensions: "both".to_string(),
        }),
        // D-472: Blurriness 0, which changes nothing.
        GAUSSIAN_BLUR_LEGACY => Some(Effect::GaussianBlurLegacy {
            blurriness: 0.0,
            dimensions: "both".to_string(),
        }),
        // D-316: After Effects' start, phase from intensity round a hue cycle, here five hues
        // evenly spaced; no layer added.
        COLORAMA => Some(Effect::Colorama {
            get_phase: "intensity".to_string(),
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            phase_shift: 0.0,
            cycle_repetitions: 1.0,
            stops: 5.0,
            color_1: "#ff0000".to_string(),
            color_2: "#ccff00".to_string(),
            color_3: "#00ff66".to_string(),
            color_4: "#0066ff".to_string(),
            color_5: "#cc00ff".to_string(),
            blend_with_original: 0.0,
            map: None,
            // D-381: the rest at their start, which changes nothing D-316 drew.
            add_phase_from: "intensity".to_string(),
            add_mode: "wrap".to_string(),
            interpolate: "on".to_string(),
            opacity_1: 100.0,
            opacity_2: 100.0,
            opacity_3: 100.0,
            opacity_4: 100.0,
            opacity_5: 100.0,
            modify: "all".to_string(),
            modify_alpha: "off".to_string(),
            change_empty: "off".to_string(),
            matching_mode: "off".to_string(),
            matching_color: "#ffffff".to_string(),
            matching_tolerance: 15.0,
            matching_softness: 0.0,
            mask_layer: serde_json::Value::from(""),
            masking_mode: "luminance".to_string(),
            composite_over: "on".to_string(),
            mask_map: None,
        }),
        // D-377..D-379: CycoreFX's starting values, a bar bent 45 degrees up the middle, a
        // 20-pixel bend up the middle, and the layer's own alpha softened by 10, lit from the top left.
        BEND_IT => Some(Effect::BendIt {
            bend: 45.0,
            start: [50.0, 100.0],
            end: [50.0, 0.0],
            render_prestart: "none".to_string(),
            distort: "legal".to_string(),
        }),
        BENDER => Some(Effect::Bender {
            amount: 20.0,
            style: "bend".to_string(),
            adjust_to_distance: "off".to_string(),
            top: [50.0, 0.0],
            base: [50.0, 100.0],
        }),
        // D-385..D-387: this program's own starting values, a pull in at the left and a push out
        // at the right, tiles a tenth of the width at 80 per cent, and a lens half the diagonal.
        FLOW_MOTION => Some(Effect::FlowMotion {
            knot_1: [25.0, 50.0],
            amount_1: 10.0,
            knot_2: [75.0, 50.0],
            amount_2: -10.0,
            falloff: 5.0,
            tile_edges: "on".to_string(),
            finer_controls: "off".to_string(),
            antialiasing: "low".to_string(),
        }),
        GRIDDLER => Some(Effect::Griddler {
            horizontal_scale: 80.0,
            vertical_scale: 80.0,
            tile_size: 10.0,
            rotation: 0.0,
            cut_tiles: "on".to_string(),
        }),
        FISHEYE => Some(Effect::Fisheye {
            center: [50.0, 50.0],
            size: 50.0,
            convergence: 50.0,
        }),
        // D-388..D-390: this program's own starting values: the bottom right corner turned up to
        // three quarters of the way, each pin where it starts, and no pulse until the level moves.
        PAGE_TURN => Some(Effect::PageTurn {
            controls: "bottom_right".to_string(),
            fold_position: [75.0, 75.0],
            fold_direction: -60.0,
            fold_radius: 30.0,
            light_direction: -45.0,
            render: "full".to_string(),
            back_page: serde_json::Value::from(""),
            back_opacity: 100.0,
            paper_color: "#f0f0f0".to_string(),
            map: None,
        }),
        // D-607: no mask chosen yet, unmoved, all of the way, Normal, as After Effects adds it.
        MASK_SMEAR => Some(Effect::MaskSmear {
            source_mask: 0.0,
            boundary_mask: 0.0,
            mask_offset: [0.0, 0.0],
            mask_rotation: 0.0,
            mask_scale: 100.0,
            percent: 100.0,
            elasticity: "normal".to_string(),
            paths: None,
        }),
        // D-606: no masks chosen yet, all of the way, Normal, as After Effects adds it.
        RESHAPE => Some(Effect::Reshape {
            source_mask: 0.0,
            destination_mask: 0.0,
            boundary_mask: 0.0,
            percent: 100.0,
            elasticity: "normal".to_string(),
            correspondence_points: Vec::new(),
            interpolation: "discrete".to_string(),
            between: Vec::new(),
            paths: None,
        }),
        // D-605: the Warp brush at 64 and 50 with nothing painted yet, as After Effects adds it.
        LIQUIFY => Some(Effect::Liquify {
            tool: "warp".to_string(),
            brush_size: 64.0,
            brush_pressure: 50.0,
            stroke: Vec::new(),
            distortion_percentage: 100.0,
            distortion_mesh_offset: [0.0, 0.0],
            // D-700: jitter 70, no clone offset, Revert, no freeze mask, no earlier stroke.
            turbulent_jitter: 70.0,
            clone_offset: [0.0, 0.0],
            reconstruction_mode: "revert".to_string(),
            freeze_area_mask: 0.0,
            strokes: Vec::new(),
            freeze_path: None,
        }),
        // D-604: Arc, horizontal, Bend 50, no distortion, as After Effects adds it.
        WARP => Some(Effect::Warp {
            warp_style: "arc".to_string(),
            warp_axis: "horizontal".to_string(),
            bend: 50.0,
            horizontal_distortion: 0.0,
            vertical_distortion: 0.0,
        }),
        // D-603: seven by seven at rest, Quality 8, as After Effects adds it.
        MESH_WARP => Some(Effect::MeshWarp {
            rows: 7.0,
            columns: 7.0,
            quality: 8.0,
            distortion_mesh: Vec::new(),
            vertex_handles: Vec::new(),
        }),
        // D-602: every point where it starts, Quality 8, as After Effects adds it.
        BEZIER_WARP => Some(Effect::BezierWarp {
            points: anime_compositor::effects::BEZIER_START,
            quality: 8.0,
        }),
        POWER_PIN => Some(Effect::PowerPin {
            top_left: [0.0, 0.0],
            top_right: [100.0, 0.0],
            bottom_left: [0.0, 100.0],
            bottom_right: [100.0, 100.0],
            perspective: 100.0,
            unstretch: "off".to_string(),
            expansion_top: 0.0,
            expansion_left: 0.0,
            expansion_right: 0.0,
            expansion_bottom: 0.0,
        }),
        // D-391..D-393: this program's own starting values: Slant upright with its floor at the
        // bottom, a short smear across the middle, and a split as wide as half the drawing.
        SLANT => Some(Effect::Slant {
            slant: 0.0,
            stretching: "off".to_string(),
            height: 100.0,
            floor: [50.0, 100.0],
            set_color: "off".to_string(),
            color: "#000000".to_string(),
        }),
        SMEAR => Some(Effect::Smear {
            from: [40.0, 50.0],
            to: [60.0, 50.0],
            reach: 100.0,
            radius: 70.0,
        }),
        SPLIT => Some(Effect::Split {
            point_a: [25.0, 50.0],
            point_b: [75.0, 50.0],
            split: 50.0,
        }),
        SPLIT_2 => Some(Effect::Split2 {
            point_a: [25.0, 50.0],
            point_b: [75.0, 50.0],
            split_1: 50.0,
            split_2: 50.0,
        }),
        TILES => Some(Effect::Tiles {
            scale: 50.0,
            center: [50.0, 50.0],
            blend: 0.0,
        }),
        // D-405: Adobe gives no starting values but Blending Mode's Normal; the rest are ours.
        MAGNIFY => Some(Effect::Magnify {
            shape: "circle".to_string(),
            center: [50.0, 50.0],
            magnification: 200.0,
            link: "none".to_string(),
            size: 100.0,
            feather: 0.0,
            opacity: 100.0,
            scaling: "standard".to_string(),
            blending_mode: "normal".to_string(),
            resize_layer: "off".to_string(),
        }),
        // D-406: Adobe gives no starting radius; 100 is ours.
        SPHERIZE => Some(Effect::Spherize {
            radius: 100.0,
            center: [50.0, 50.0],
        }),
        // D-413: the anchor in the middle, squares 64 pixels across in white, laid in place of
        // the layer, as After Effects adds it.
        CHECKERBOARD => Some(Effect::Checkerboard {
            anchor: [50.0, 50.0],
            size_from: "width_slider".to_string(),
            corner: [60.0, 60.0],
            width: 64.0,
            height: 64.0,
            feather_width: 0.0,
            feather_height: 0.0,
            color: "#ffffff".to_string(),
            opacity: 100.0,
            blending_mode: "none".to_string(),
        }),
        // D-414: a white disk of radius 75 in the middle, in place of the layer, as After
        // Effects adds it.
        CIRCLE => Some(Effect::Circle {
            center: [50.0, 50.0],
            radius: 75.0,
            edge: "none".to_string(),
            edge_thickness: 10.0,
            feather_outer: 0.0,
            feather_inner: 0.0,
            invert: "off".to_string(),
            color: "#ffffff".to_string(),
            opacity: 100.0,
            blending_mode: "none".to_string(),
        }),
        // D-415: an outline 200 across in the middle over the layer, white along its middle and
        // blue at its edges, as Beam's colours.
        ELLIPSE => Some(Effect::Ellipse {
            center: [50.0, 50.0],
            width: 200.0,
            height: 200.0,
            thickness: 8.0,
            softness: 50.0,
            inside_color: "#ffffff".to_string(),
            outside_color: "#3c8cff".to_string(),
            composite: "on".to_string(),
        }),
        // D-407: After Effects' own starting settings.
        DETAIL_UPSCALE => Some(Effect::DetailUpscale {
            scale: 100.0,
            reduce_noise: 0.0,
            detail: 20.0,
        }),
        // D-420: After Effects' own starting settings; no Audio Layer, so silence until one is
        // chosen.
        AUDIO_SPECTRUM => Some(Effect::AudioSpectrum {
            audio_layer: serde_json::json!(""),
            start_point: [10.0, 50.0],
            end_point: [90.0, 50.0],
            path: 0.0,
            use_polar_path: "off".to_string(),
            start_frequency: 20.0,
            end_frequency: 2000.0,
            frequency_bands: 64.0,
            maximum_height: 200.0,
            audio_duration: 90.0,
            audio_offset: 0.0,
            thickness: 3.0,
            softness: 50.0,
            inside_color: "#ffffff".to_string(),
            outside_color: "#3c8cff".to_string(),
            blend_overlapping_colors: "off".to_string(),
            hue_interpolation: 0.0,
            dynamic_hue_phase: "off".to_string(),
            color_symmetry: "off".to_string(),
            display_options: "digital".to_string(),
            side_options: "side_a_b".to_string(),
            duration_averaging: "off".to_string(),
            composite: "off".to_string(),
            levels: None,
            outline: None,
        }),
        // D-421: After Effects' own control list; the numbers are ours (Adobe gives none).
        AUDIO_WAVEFORM => Some(Effect::AudioWaveform {
            audio_layer: serde_json::json!(""),
            start_point: [10.0, 50.0],
            end_point: [90.0, 50.0],
            path: 0.0,
            displayed_samples: 32.0,
            maximum_height: 300.0,
            audio_duration: 90.0,
            audio_offset: 0.0,
            thickness: 2.0,
            softness: 50.0,
            random_seed: 1.0,
            inside_color: "#ffffff".to_string(),
            outside_color: "#3c8cff".to_string(),
            waveform_options: "mono".to_string(),
            display_options: "analog_lines".to_string(),
            composite: "off".to_string(),
            levels: None,
            outline: None,
        }),
        // D-416: After Effects' own starting settings.
        FRACTAL => Some(Effect::Fractal {
            set_choice: "mandelbrot".to_string(),
            equation: "z2".to_string(),
            mandelbrot_center: [-0.75, 0.0],
            mandelbrot_magnification: 0.0,
            mandelbrot_escape_limit: 100.0,
            julia_center: [0.0, 0.0],
            julia_magnification: 0.0,
            julia_escape_limit: 100.0,
            overlay: "off".to_string(),
            transparency: "off".to_string(),
            palette: "lightness_gradient".to_string(),
            hue: 0.0,
            cycle_steps: 10.0,
            cycle_offset: 0.0,
            edge_highlight: "off".to_string(),
            oversample_method: "edge_detect".to_string(),
            oversample_factor: 2.0,
        }),
        // D-417: white lines 2 pixels thick, cells from the middle to (60, 60) per cent, in place
        // of the layer.
        // D-418: as added, After Effects' Fill: no mask (the whole layer), red, unfeathered.
        // D-419: as added: the middle's one pixel, Skip Empty, nothing blended back.
        EYEDROPPER_FILL => Some(Effect::EyedropperFill {
            sample_point: [50.0, 50.0],
            sample_radius: 0.0,
            average_pixel_colors: "skip_empty".to_string(),
            maintain_original_alpha: "off".to_string(),
            blend_with_original: 0.0,
        }),
        // D-440: as added, After Effects' Paint Bucket: the middle, Color & Alpha within 10, a
        // pixel's antialiasing, red, laid on normally.
        PAINT_BUCKET => Some(Effect::PaintBucket {
            fill_point: [50.0, 50.0],
            fill_selector: "color_and_alpha".to_string(),
            tolerance: 10.0,
            view_threshold: "off".to_string(),
            stroke: "antialias".to_string(),
            invert_fill: "off".to_string(),
            spread_radius: 3.0,
            stroke_width: 3.0,
            feather_softness: 10.0,
            color: "#ff0000".to_string(),
            opacity: 100.0,
            blending_mode: "normal".to_string(),
        }),
        // D-443: as added (After Effects gives no values): a fine coloured grain at intensity 1,
        // laid on by Film, the same strength in every tone, new each frame.
        // D-450: as added (After Effects gives no values): still random speckles taking up to a
        // fifth of the covering away inside the drawing only.
        NOISE_ALPHA => Some(Effect::NoiseAlpha {
            noise: "uniform_random".to_string(),
            amount: 20.0,
            original_alpha: "clamp".to_string(),
            overflow: "clip".to_string(),
            random_seed: 0.0,
            noise_phase: 0.0,
            cycle_noise: "off".to_string(),
            cycle: 1.0,
        }),
        // D-451: as added (After Effects gives no values): still, pixel-sized flecks of lightness
        // up to a tenth either way, the hue and saturation untouched.
        NOISE_HLS => Some(Effect::NoiseHls {
            noise: "uniform".to_string(),
            hue: 0.0,
            lightness: 10.0,
            saturation: 0.0,
            grain_size: 1.0,
            noise_phase: 0.0,
        }),
        // D-452: as Noise HLS is added, moving to a new noise every frame.
        NOISE_HLS_AUTO => Some(Effect::NoiseHlsAuto {
            noise: "uniform".to_string(),
            hue: 0.0,
            lightness: 10.0,
            saturation: 0.0,
            grain_size: 1.0,
            animation_speed: 1.0,
            frame: 0,
        }),
        ADD_GRAIN => Some(Effect::AddGrain {
            intensity: 1.0,
            size: 1.0,
            softness: 0.0,
            aspect_ratio: 1.0,
            red_intensity: 1.0,
            green_intensity: 1.0,
            blue_intensity: 1.0,
            monochromatic: "off".to_string(),
            saturation: 1.0,
            blending_mode: "film".to_string(),
            shadows: 1.0,
            midtones: 1.0,
            highlights: 1.0,
            midpoint: 0.5,
            animation_speed: 1.0,
            animate_smoothly: "on".to_string(),
            random_seed: 0.0,
            frame: 0,
        }),
        // D-454: Add Grain's starting values, and no noise source until one is chosen.
        MATCH_GRAIN => Some(Effect::MatchGrain {
            layer: serde_json::json!(""),
            grain: Box::new(new_effect(ADD_GRAIN)?),
            map: None,
        }),
        // D-455: After Effects' starting values for the controls built: Noise Reduction 1,
        // one pass, Multichannel, no Unsharp Mask (amount 0, radius 1, threshold 0).
        REMOVE_GRAIN => Some(Effect::RemoveGrain {
            noise_reduction: 1.0,
            passes: 1.0,
            mode: "multichannel".to_string(),
            unsharp_amount: 0.0,
            unsharp_radius: 1.0,
            unsharp_threshold: 0.0,
            noise: None,
        }),
        // D-456: this program's own starting values (CelFX gives none): threshold 4 and
        // softness 2 levels, blur 8, diffusion 1, noise 1, every reduction 100, reseeding every
        // frame, view Normal.
        DEBAND => Some(Effect::Deband {
            view: "normal".to_string(),
            edge_threshold: 4.0,
            edge_softness: 2.0,
            blur_amount: 8.0,
            edge_blur_reduction: 100.0,
            diffusion_amount: 1.0,
            edge_diffusion_reduction: 100.0,
            diffusion_reseed_interval: 1.0,
            noise_amount: 1.0,
            edge_noise_reduction: 100.0,
            noise_reseed_interval: 1.0,
            frame: 0,
        }),
        // D-457: as added (Adobe names the controls, not every value): Standard at 100, and for Ultra a pure green key,
        // tolerance 50, desaturate 25, spill range 50, colour correction 50, luma correction 50.
        ADVANCED_SPILL_SUPPRESSOR => Some(Effect::AdvancedSpillSuppressor {
            method: "standard".to_string(),
            suppression: 100.0,
            key_color: "#00ff00".to_string(),
            tolerance: 50.0,
            desaturate: 25.0,
            spill_range: 50.0,
            spill_color_correction: 50.0,
            luma_correction: 50.0,
        }),
        // D-459: as added (Adobe gives no values): a blue key, the final view, every levels plain.
        COLOR_DIFFERENCE_KEY => Some(Effect::ColorDifferenceKey {
            view: "final".to_string(),
            key_color: "#0000ff".to_string(),
            partial_a: LEVELS_PLAIN,
            partial_b: LEVELS_PLAIN,
            matte: [0.0, 255.0, 1.0],
        }),
        // D-500: as added (Adobe gives no values we found): Lab, every end of the box 0, which
        // keys nothing until set.
        COLOR_RANGE => Some(Effect::ColorRange {
            color_space: "lab".to_string(),
            fuzziness: 0.0,
            range: [0.0; 6],
        }),
        // D-501: as added (Adobe gives no values we found): the final picture, no plate until one
        // is chosen (the layer is left as it is), centred, tolerance 15, no softness, no blur.
        DIFFERENCE_MATTE => Some(Effect::DifferenceMatte {
            view: "final".to_string(),
            layer: serde_json::Value::from(""),
            fit: "center".to_string(),
            matching_tolerance: 15.0,
            matching_softness: 0.0,
            blur_before_difference: 0.0,
            map: None,
        }),
        // D-502: as added (Adobe gives no values we found): no foreground (nothing keyed until a
        // mask is chosen), no background, radius 10, no thin, feather or threshold, not inverted,
        // none of the original blended back.
        INNER_OUTER_KEY => Some(Effect::InnerOuterKey {
            foreground: 0.0,
            background: 0.0,
            single_mask_highlight_radius: 10.0,
            edge_thin: 0.0,
            edge_feather: 0.0,
            edge_threshold: 0.0,
            invert_extraction: "off".to_string(),
            blend_with_original: 0.0,
            paths: None,
        }),
        // D-503: as added: Adobe's radius 10; no contrast added; the whole of the cleaning.
        KEY_CLEANER => Some(Effect::KeyCleaner {
            additional_edge_radius: 10.0,
            alpha_contrast: 0.0,
            strength: 100.0,
        }),
        // D-504: as added (Adobe gives no values we found): final, pure blue, RGB as Adobe
        // advises, tolerance and softness 10 (softness under Adobe's best 20).
        LINEAR_COLOR_KEY => Some(Effect::LinearColorKey {
            view: "final".to_string(),
            key_color: "#0000ff".to_string(),
            match_colors: "rgb".to_string(),
            key_operation: "key".to_string(),
            matching_tolerance: 10.0,
            matching_softness: 10.0,
        }),
        // D-540: as added: final, a pure green screen, gain 100, balance half way, no clip,
        // despill bias half way at full strength, no shrink, softness or edge refine.
        SCREEN_KEY => Some(Effect::ScreenKey {
            view: "final".to_string(),
            screen_color: "#00ff00".to_string(),
            screen_gain: 100.0,
            screen_balance: 50.0,
            clip_black: 0.0,
            clip_white: 100.0,
            despill_bias: 50.0,
            despill: 100.0,
            shrink_grow: 0.0,
            softness: 0.0,
            edge_refine: 0.0,
        }),
        // D-548: as added: a blue screen's spill, as Color Difference Key and Linear Color Key
        // start from blue, held down fully; After Effects' starting values are not published.
        SPILL_SUPPRESSOR => Some(Effect::SpillSuppressor {
            color_to_suppress: "#0000ff".to_string(),
            suppression: 100.0,
        }),
        // D-549: as added, a slight softening, sigma 1 pixel; After Effects' is not published.
        REDUCE_INTERLACE_FLICKER => Some(Effect::ReduceInterlaceFlicker { softness: 1.0 }),
        // D-505: as added (Adobe gives no values we found): black, level 0, no softness, so
        // the covering is the brightest channel; colour matting removed; HDR not clipped.
        UNMULT => Some(Effect::Unmult {
            background_color: "black".to_string(),
            black_level: 0.0,
            white_level: 100.0,
            softness: 0.0,
            remove_color_matting: "on".to_string(),
            clip_hdr_results: "off".to_string(),
        }),
        // D-506: as added (Adobe gives no values we found): the layer itself as both views, no
        // convergence or alignment, so nothing changes; pixels; not swapped; balanced colored
        // red blue; balance 0.75.
        // D-507: as added (CycoreFX gives no values we found): a white light a quarter across and
        // down, half the width above, at the middle, opening 40 degrees, softness 20, full
        // intensity, render light shadow.
        // D-508: as added (Video Copilot gives no values we found): a straight beam from a quarter
        // to three quarters across, half way down, core 8 across, softness 30, a blue glow spread
        // 40 at intensity 100, no flicker or distortion, added to the layer.
        ENERGY_STROKE => Some(Effect::EnergyStroke {
            source: "points".to_string(),
            core_start: [25.0, 50.0],
            core_end: [75.0, 50.0],
            evolution: 0.0,
            start_offset: 0.0,
            end_offset: 100.0,
            start_size: 100.0,
            end_size: 100.0,
            core_size: 8.0,
            core_softness: 30.0,
            glow_color: "#3080ff".to_string(),
            glow_intensity: 100.0,
            glow_spread: 40.0,
            flicker_amount: 0.0,
            flicker_speed: 20.0,
            distortion_amount: 0.0,
            distortion_scale: 60.0,
            distortion_speed: 20.0,
            random_seed: 0.0,
            composite: "add".to_string(),
            // D-565: D-508's beam: round ends, the noise along the path, nothing cut.
            start_roundness: 100.0,
            end_roundness: 100.0,
            noise_along_path: "on".to_string(),
            alpha_mode: "off".to_string(),
            invert_masks: "off".to_string(),
            frame: 0,
            paths: None,
            matte: None,
        }),
        // D-530: as added (CycoreFX gives no values we found): an upright cylinder half the
        // drawing's half width in radius, in the middle, unturned, both sides drawn, lit white
        // from the upper left half way towards the viewer, ambient 40, diffuse 60, specular 50,
        // roughness 10, metal 100.
        // D-531: as added (CycoreFX gives no values we found): no environment, a spherical map
        // looked at straight ahead with a 40 degree view (about a 50 mm lens), filtered.
        CC_ENVIRONMENT => Some(Effect::CcEnvironment {
            environment: serde_json::Value::from(""),
            mapping: "spherical".to_string(),
            horizontal_pan: 0.0,
            tilt: 0.0,
            roll: 0.0,
            field_of_view: 40.0,
            filter_environment: "on".to_string(),
            map: None,
        }),
        // D-580: as added (Adobe gives no values we found): nothing flipped yet, a band half the
        // way across, eight rows and eight columns flipping left to right about x, the cards
        // their own size, no back or gradient layer, in order; the camera straight on from a
        // 50 mm lens one focal length away, so the picture is unchanged; a distant white light
        // from in front, ambient 0.25, diffuse 0.75, no highlight.
        CARD_WIPE => Some(Effect::CardWipe {
            transition_completion: 0.0,
            transition_width: 50.0,
            back_layer: serde_json::Value::from(""),
            back_map: None,
            rows_columns: "independent".to_string(),
            rows: 8.0,
            columns: 8.0,
            card_scale: 1.0,
            flip_axis: "x".to_string(),
            flip_direction: "positive".to_string(),
            flip_order: "left_to_right".to_string(),
            gradient_layer: serde_json::Value::from(""),
            gradient_map: None,
            timing_randomness: 0.0,
            random_seed: 0.0,
            x_rotation: 0.0,
            y_rotation: 0.0,
            z_rotation: 0.0,
            xy_position: [50.0, 50.0],
            z_position: 1.0,
            focal_length: 50.0,
            light_type: "distant".to_string(),
            light_intensity: 1.0,
            light_color: "#ffffff".to_string(),
            light_position: [50.0, 50.0],
            light_depth: 1.0,
            ambient_light: 0.25,
            diffuse_reflection: 0.75,
            specular_reflection: 0.0,
            highlight_sharpness: 5.0,
            camera_system: "camera_position".to_string(),
            upper_left_corner: [0.0, 0.0],
            upper_right_corner: [100.0, 0.0],
            lower_left_corner: [0.0, 100.0],
            lower_right_corner: [100.0, 100.0],
            auto_focal_length: "off".to_string(),
        }),
        // D-532: as added (CycoreFX gives no values we found): a sphere half the drawing's half
        // width in radius, in the middle, unturned, both sides drawn, lit as CC Cylinder is.
        CC_SPHERE => Some(Effect::CcSphere {
            radius: 50.0,
            offset: [50.0, 50.0],
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            render: "full".to_string(),
            light_intensity: 100.0,
            light_color: "#ffffff".to_string(),
            light_height: 50.0,
            light_direction: -45.0,
            ambient: 40.0,
            diffuse: 60.0,
            specular: 50.0,
            roughness: 10.0,
            metal: 100.0,
        }),
        CC_CYLINDER => Some(Effect::CcCylinder {
            radius: 50.0,
            position: [50.0, 50.0],
            position_z: 0.0,
            rotation_x: 0.0,
            rotation_y: 0.0,
            rotation_z: 0.0,
            render: "full".to_string(),
            light_intensity: 100.0,
            light_color: "#ffffff".to_string(),
            light_height: 50.0,
            light_direction: -45.0,
            ambient: 40.0,
            diffuse: 60.0,
            specular: 50.0,
            roughness: 10.0,
            metal: 100.0,
        }),
        CC_SPOTLIGHT => Some(Effect::CcSpotlight {
            from: [25.0, 25.0],
            to: [50.0, 50.0],
            height: 50.0,
            cone_angle: 40.0,
            edge_softness: 20.0,
            color: "#ffffff".to_string(),
            intensity: 100.0,
            render: "light_shadow".to_string(),
            gel_layer: serde_json::Value::from(""),
            gel_map: None,
        }),
        // D-551: as added (CycoreFX gives no values we found): no scatter, no turn, no twist, a
        // ball every 8 pixels as wide as its cell.
        CC_BALL_ACTION => Some(Effect::CcBallAction {
            scatter: 0.0,
            rotation_axis: "x".to_string(),
            rotation: 0.0,
            twist_property: "x_axis".to_string(),
            twist_angle: 0.0,
            grid_spacing: 8.0,
            ball_size: 100.0,
            instability_state: 0.0,
        }),
        // D-610: as added (ours, CycoreFX gives no values we found): 200 particles a second
        // living a second, thrown out every way from the middle and falling, drawn as lines in yellow to
        // red through its own camera.
        PARTICLE_SPACE => Some(Effect::ParticleSpace {
            values: PARTICLE_SPACE_SETTINGS.map(|s| s.3),
            words: PARTICLE_SPACE_WORDS.map(|w| w.2.to_string()),
            colors: PARTICLE_SPACE_COLORS.map(|c| c.1.to_string()),
            clock: [0.0; 2],
            view: [0.0; 4],
            history: None,
            texture_layer: serde_json::Value::from(""),
            texture: None,
        }),
        // D-672: as added (ours, CycoreFX gives no values we found): ten drops a second, each
        // ripple two rings spreading for a second, lit white from the top left.
        DRIZZLE => Some(Effect::Drizzle {
            values: DRIZZLE_SETTINGS.map(|s| s.3),
            words: DRIZZLE_WORDS.map(|w| w.2.to_string()),
            light_color: "#ffffff".into(),
            clock: [0.0; 2],
            history: None,
        }),
        // D-673: as added (ours, CycoreFX gives no values we found): a short brown fuzz, half
        // the picture's colour, hanging by half its length, lit from the top left.
        HAIR => Some(Effect::Hair {
            values: HAIR_SETTINGS.map(|s| s.3),
            words: HAIR_WORDS.map(|w| w.2.to_string()),
            hair_color: "#7a4e2d".into(),
        }),
        // D-674: as added (ours, CycoreFX gives no values we found): twenty drops a second
        // thrown every way from the middle and falling, swelling in and shrinking out over two
        // seconds, lit white from the top left.
        MERCURY => Some(Effect::Mercury {
            values: MERCURY_SETTINGS.map(|s| s.3),
            words: MERCURY_WORDS.map(|w| w.2.to_string()),
            light_color: "#ffffff".into(),
            clock: [0.0; 2],
            history: None,
        }),
        // D-675: as added (ours, CycoreFX gives no values we found): a grid of 20 pixels cut
        // into textured triangles, thrown from the middle at once and falling slowly, depth
        // sorted.
        POLYGON_SHARDS => Some(Effect::PolygonShards {
            values: POLYGON_SHARDS_SETTINGS.map(|s| s.3),
            words: POLYGON_SHARDS_WORDS.map(|w| w.2.to_string()),
            clock: [0.0; 2],
            view: [0.0; 4],
        }),
        // D-678: as added (ours, after what we recall of After Effects' and not checked): the
        // Draft view, thirty tiny bubbles a second from a small producer in the middle, drifting
        // right on a light wind, living about ten seconds.
        FOAM => Some(Effect::Foam {
            values: FOAM_SETTINGS.map(|s| s.3),
            words: FOAM_WORDS.map(|w| w.2.to_string()),
            clock: [0.0; 2],
            history: None,
        }),
        // D-676: as added (ours, CycoreFX gives no values we found): a fine dusting, untwisted,
        // laid over.
        SCATTERIZE => Some(Effect::Scatterize {
            values: SCATTERIZE_SETTINGS.map(|s| s.3),
            words: SCATTERIZE_WORDS.map(|w| w.2.to_string()),
        }),
        // D-677: as added (ours, CycoreFX gives no values we found): a fine field, fully
        // scattered, flying towards the eye at a picture height a second.
        STAR_BURST => Some(Effect::StarBurst {
            values: STAR_BURST_SETTINGS.map(|s| s.3),
            clock: [0.0; 2],
            view: [0.0; 4],
            history: None,
        }),
        // D-671: as added (ours, CycoreFX gives no values we found): a hundred small bubbles
        // rising slowly and wobbling once a second, turned over, faded towards the middle.
        BUBBLES => Some(Effect::Bubbles {
            values: BUBBLES_SETTINGS.map(|s| s.3),
            words: BUBBLES_WORDS.map(|w| w.2.to_string()),
            clock: [0.0; 2],
            history: None,
        }),
        // D-612: as added (ours, CycoreFX gives no values we found): 200 particles a second
        // living a second, thrown out every way from the middle and falling, drawn as lines in
        // yellow to red, flat on the layer.
        PARTICLE_PLANE => Some(Effect::ParticlePlane {
            values: PARTICLE_PLANE_SETTINGS.map(|s| s.3),
            words: PARTICLE_PLANE_WORDS.map(|w| w.2.to_string()),
            colors: PARTICLE_PLANE_COLORS.map(|c| c.1.to_string()),
            clock: [0.0; 2],
            history: None,
            texture_layer: serde_json::Value::from(""),
            texture: None,
        }),
        // D-613: as added (ours, Adobe's page gives no values we found): 20 red dots a second
        // shot upwards from the middle at 130 pixels a second, spread a little and falling, no
        // grid, nothing exploded, no wall.
        PARTICLE_PLAYGROUND => Some(Effect::ParticlePlayground {
            values: PARTICLE_PLAYGROUND_SETTINGS.map(|s| s.3),
            cannon_position: vec![50.0, 50.0],
            grid_position: vec![50.0, 50.0],
            words: PARTICLE_PLAYGROUND_WORDS.map(|w| w.2.to_string()),
            layers: std::array::from_fn(|_| serde_json::Value::from("")),
            colors: PARTICLE_PLAYGROUND_COLORS.map(|c| c.1.to_string()),
            maps: Default::default(),
            wall: None,
            clock: [0.0; 2],
            history: None,
        }),
        // D-615: as added (ours, Red Giant's page gives no values we found): 100 white soft
        // spheres a second, 4 pixels across, thrown every way at 100 pixels a second from the
        // middle, living 3 seconds, no gravity, no floor.
        PARTICLE_EMITTER => Some(Effect::ParticleEmitter {
            values: PARTICLE_EMITTER_SETTINGS.map(|s| s.3),
            emitter_position: vec![50.0, 50.0],
            words: PARTICLE_EMITTER_WORDS.map(|w| w.2.to_string()),
            colors: PARTICLE_EMITTER_COLORS.map(|c| c.1.to_string()),
            clock: [0.0; 2],
            view: [0.0; 4],
            history: None,
        }),
        // D-616: as added (ours, Red Giant's page gives no values we found): a still grid 820 by
        // 620 pixels of 41 by 31 white soft spheres 4 pixels across, round the middle, no maps.
        PARTICLE_GRID => Some(Effect::ParticleGrid {
            values: PARTICLE_GRID_SETTINGS.map(|s| s.3),
            form_center: vec![50.0, 50.0],
            words: PARTICLE_GRID_WORDS.map(|w| w.2.to_string()),
            layers: std::array::from_fn(|_| serde_json::Value::from("")),
            colors: PARTICLE_GRID_COLORS.map(|c| c.1.to_string()),
            maps: Default::default(),
            clock: [0.0; 2],
            view: [0.0; 4],
            history: None,
        }),
        // D-510: as added (CycoreFX gives no values we found): nothing wiped, from the middle,
        // ten tiles across, round, no border.
        // D-544: as added (ours): the brightest light, spreading 25 pixels, red-orange at full
        // strength, so adding it shows it.
        HALATION => Some(Effect::Halation {
            threshold: 60.0,
            threshold_smooth: 50.0,
            radius: 25.0,
            amount: 100.0,
            color: "#ff4020".to_string(),
        }),
        // D-511: as added (CycoreFX gives no values we found): nothing parted, across the
        // middle, spikes 40 pixels high and wide.
        // D-512: as added (CycoreFX gives no values we found): nothing wiped, the layer itself
        // as the gradient read by its luminance, stretched, softness 10, no blur.
        CHANNEL_WIPE => Some(Effect::ChannelWipe {
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            completion: 0.0,
            softness: 10.0,
            property: "luminance".to_string(),
            blur: 0.0,
            invert: "off".to_string(),
            map: None,
        }),
        // D-513: as added (CycoreFX gives only Color from Source off): nothing wiped, a white
        // circle of light from the middle at full intensity.
        // D-514: as added (CycoreFX gives none): nothing wiped, lines 20 pixels thick across a
        // left-to-right sweep, each starting halfway through the one before.
        // D-518: as added (CycoreFX gives none): not begun, nothing to reveal, by brightness,
        // smoothness 10 and amount 50 as Glass Melt's, the two pulled the same way, blending over
        // the middle half.
        WARP_BLEND => Some(Effect::WarpBlend {
            completion: 0.0,
            layer_to_reveal: serde_json::Value::from(""),
            reactor: "brightness".to_string(),
            smoothness: 10.0,
            warp_amount: 50.0,
            warp_direction: "joint".to_string(),
            blend_span: 50.0,
            map: None,
        }),
        // D-517: as added (CycoreFX gives none): nothing turned, nothing on the back, shaded,
        // from the middle, the twist running to the right.
        TWISTER => Some(Effect::Twister {
            completion: 0.0,
            backside: serde_json::Value::from(""),
            shading: "on".to_string(),
            center: [50.0, 50.0],
            axis: 0.0,
            map: None,
        }),
        // D-516: as added (CycoreFX gives none): nothing stretched, from the middle, toward the
        // right.
        STRETCH_WIPE => Some(Effect::StretchWipe {
            scale: 100.0,
            center: [50.0, 50.0],
            direction: 0.0,
        }),
        // D-515: as added (CycoreFX gives none): nothing opened, from the middle.
        PUSH_IRIS => Some(Effect::PushIris {
            completion: 0.0,
            center: [50.0, 50.0],
            reverse: "off".to_string(),
        }),
        STRIPE_SWEEP => Some(Effect::StripeSweep {
            completion: 0.0,
            direction: 0.0,
            thickness: 20.0,
            slant: 50.0,
            flip: "off".to_string(),
        }),
        GLOW_WIPE => Some(Effect::GlowWipe {
            completion: 0.0,
            center: [50.0, 50.0],
            intensity: 100.0,
            direction: 0.0,
            shape: "radial".to_string(),
            color_from_source: "off".to_string(),
            color: "#ffffff".to_string(),
            reverse: "off".to_string(),
        }),
        TEETH_WIPE => Some(Effect::TeethWipe {
            completion: 0.0,
            center: [50.0, 50.0],
            direction: 0.0,
            height: 40.0,
            width: 40.0,
            shape: "spikes".to_string(),
        }),
        TILE_WIPE => Some(Effect::TileWipe {
            completion: 0.0,
            center: [50.0, 50.0],
            rotation: 0.0,
            border: 0.0,
            tiles: 10.0,
            shape: "radial".to_string(),
            reverse: "off".to_string(),
        }),
        // D-509: as added (CycoreFX gives no values we found): nothing melted, nothing to reveal,
        // the layer's own brightness as the gradient, softness 10, displacement 50.
        GLASS_MELT => Some(Effect::GlassMelt {
            completion: 0.0,
            layer_to_reveal: serde_json::Value::from(""),
            gradient_layer: serde_json::Value::from(""),
            softness: 10.0,
            displacement_amount: 50.0,
            reveal_map: None,
            gradient_map: None,
        }),
        THREE_D_GLASSES => Some(Effect::ThreeDGlasses {
            left_view: serde_json::Value::from(""),
            right_view: serde_json::Value::from(""),
            scene_convergence: 0.0,
            vertical_alignment: 0.0,
            units: "pixels".to_string(),
            swap_left_right: "off".to_string(),
            three_d_view: "balanced_colored_red_blue".to_string(),
            balance: 0.75,
            left_map: None,
            right_map: None,
        }),
        // D-458: as added (CycoreFX gives no values): a level wire from a quarter to three quarters
        // across, half way down, 5 pixels thick, displaced, slope 50, mirror blend 25, a frame back.
        SIMPLE_WIRE_REMOVAL => Some(Effect::SimpleWireRemoval {
            point_a: [25.0, 50.0],
            point_b: [75.0, 50.0],
            removal_style: "displace".to_string(),
            thickness: 5.0,
            slope: 50.0,
            mirror_blend: 25.0,
            frame_offset: -1.0,
            picture: None,
        }),
        // D-441: as added (After Effects gives no values): the middle, a white brush 6 across at
        // hardness 75, a mark every 0.01 seconds kept for ever, laid on the layer.
        // D-442: as added (After Effects gives no values): the first mask filled inside with
        // white lines 5 across at 45 degrees, 5 apart, a little curved, wiggling smoothly.
        SCRIBBLE => Some(Effect::Scribble {
            scribble: "single_mask".to_string(),
            mask: 1.0,
            fill_type: "inside".to_string(),
            edge_width: 10.0,
            color: "#ffffff".to_string(),
            opacity: 100.0,
            angle: 45.0,
            stroke_width: 5.0,
            curviness: 5.0,
            curviness_variation: 1.0,
            spacing: 5.0,
            spacing_variation: 0.5,
            path_overlap: 0.0,
            path_overlap_variation: 2.0,
            start: 0.0,
            end: 100.0,
            fill_paths_sequentially: "on".to_string(),
            wiggle_type: "smooth".to_string(),
            wiggles_per_second: 5.0,
            random_seed: 1.0,
            composite: "on_original".to_string(),
            masks: None,
            time: 0.0,
        }),
        WRITE_ON => Some(Effect::WriteOn {
            brush_position: [50.0, 50.0],
            color: "#ffffff".to_string(),
            brush_size: 6.0,
            brush_hardness: 75.0,
            brush_opacity: 100.0,
            stroke_length: 0.0,
            brush_spacing: 0.01,
            paint_time_properties: "none".to_string(),
            brush_time_properties: "none".to_string(),
            paint_style: "on_original".to_string(),
            marks: Vec::new(),
        }),
        FILL => Some(Effect::Fill {
            mask: 0.0,
            all_masks: "off".to_string(),
            color: "#ff0000".to_string(),
            invert: "off".to_string(),
            horizontal_feather: 0.0,
            vertical_feather: 0.0,
            opacity: 100.0,
            paths: None,
        }),
        GRID => Some(Effect::Grid {
            anchor: [50.0, 50.0],
            size_from: "corner_point".to_string(),
            corner: [60.0, 60.0],
            width: 64.0,
            height: 64.0,
            border: 2.0,
            feather_width: 0.0,
            feather_height: 0.0,
            invert: "off".to_string(),
            color: "#ffffff".to_string(),
            opacity: 100.0,
            blending_mode: "none".to_string(),
        }),
        // D-422: CC Light Burst 2.5's controls; the numbers are ours (the manual gives none).
        LIGHT_BURST => Some(Effect::LightBurst {
            center: [50.0, 50.0],
            intensity: 100.0,
            ray_length: 50.0,
            burst: "straight".to_string(),
            set_color: "off".to_string(),
            color: "#ffffff".to_string(),
        }),
        ARBITRARY_MAP => Some(Effect::ArbitraryMap {
            map: String::new(),
            phase: 0.0,
            apply_to_alpha: "off".to_string(),
            table: None,
        }),
        // D-396: Photoshop's own start, every amount 0, which changes nothing.
        SELECTIVE_COLOR => Some(Effect::SelectiveColor {
            method: "relative".to_string(),
            families: std::array::from_fn(|_| vec![0.0; 4]),
        }),
        // D-400: After Effects' panel as it starts.
        SHADOW_HIGHLIGHT => Some(Effect::ShadowHighlight {
            shadow_amount: 50.0,
            highlight_amount: 0.0,
            shadow_tonal_width: 50.0,
            shadow_radius: 30.0,
            highlight_tonal_width: 50.0,
            highlight_radius: 30.0,
            color_correction: 20.0,
            // D-563: 0 each; After Effects is recalled to start the clips at 0.01 per cent.
            midtone_contrast: 0.0,
            black_clip: 0.0,
            white_clip: 0.0,
            // D-564: off; After Effects is recalled to start with Auto Amounts on.
            auto_amounts: "off".to_string(),
            temporal_smoothing: 0.0,
            scene_detect: "off".to_string(),
            stats: None,
        }),
        RIPPLE_PULSE => Some(Effect::RipplePulse {
            center: [50.0, 50.0],
            pulse_level: 0.0,
            time_span: 1.0,
            amplitude: 10.0,
            render_bump_map: "off".to_string(),
            levels: Vec::new(),
        }),
        BLOBBYLIZE => Some(Effect::Blobbylize {
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            property: "alpha".to_string(),
            softness: 10.0,
            cut_away: 0.0,
            light_intensity: 100.0,
            light_color: "#ffffff".to_string(),
            light_type: "distant".to_string(),
            light_height: 100.0,
            light_position: [30.0, 30.0],
            light_direction: -45.0,
            ambient: 25.0,
            diffuse: 75.0,
            specular: 50.0,
            roughness: 0.05,
            metal: 100.0,
            map: None,
        }),
        // D-424: CC Glue Gun's controls; the numbers are ours (the manual gives none), the light
        // Blobbylize's.
        GLUE_GUN => Some(Effect::GlueGun {
            brush_position: [50.0, 50.0],
            stroke_width: 20.0,
            density: 5.0,
            time_span: 1.0,
            reflection: 50.0,
            strength: 50.0,
            paint_style: "plain".to_string(),
            wobble_width: 10.0,
            wobble_height: 10.0,
            wobble_speed: 1.0,
            light_intensity: 100.0,
            light_color: "#ffffff".to_string(),
            light_type: "distant".to_string(),
            light_height: 100.0,
            light_position: [30.0, 30.0],
            light_direction: -45.0,
            ambient: 25.0,
            diffuse: 75.0,
            specular: 50.0,
            roughness: 0.05,
            metal: 100.0,
            trail: Vec::new(),
            clock: [0.0, 0.0],
        }),
        // D-426: After Effects' Lens Flare controls; the starting values are ours.
        LENS_FLARE => Some(Effect::LensFlare {
            flare_center: [30.0, 30.0],
            flare_brightness: 100.0,
            lens_type: "zoom".to_string(),
            blend_with_original: 0.0,
            // D-545: no dirt.
            dirt: Default::default(),
        }),
        // D-445: After Effects' Turbulent Noise controls over Fractal Noise's engine; Complexity
        // 6 as After Effects', the rest as Fractal Noise starts.
        TURBULENT_NOISE => Some(Effect::TurbulentNoise {
            fractal_type: "basic".to_string(),
            noise_type: "smooth".to_string(),
            invert: "off".to_string(),
            contrast: 100.0,
            brightness: 0.0,
            size: 100.0,
            scale_width: 100.0,
            scale_height: 100.0,
            offset: [0.0, 0.0],
            complexity: 6.0,
            evolution: 0.0,
            seed: 0.0,
            opacity: 100.0,
            blend: "normal".to_string(),
            float: false,
            more: Default::default(),
        }),
        // D-446: After Effects' Curl Noise controls (26.3 beta); the starting values are ours.
        CURL_NOISE => Some(Effect::CurlNoise {
            source: "internal".to_string(),
            source_layer: serde_json::Value::from(""),
            softness: 0.0,
            source_mapping: "fill".to_string(),
            mirror_at_edge: "off".to_string(),
            map: None,
            speed: 10.0,
            direction: 0.0,
            size: 100.0,
            offset: [0.0, 0.0],
            rotation: 0.0,
            scale: 100.0,
            perspective_offset: "off".to_string(),
            evolution: 0.0,
            turbulence_speed: 20.0,
            swirl: 45.0,
            density: 0.0,
            smoothness: 50.0,
            vertical_bias: 50.0,
            sample_count: 12.0,
            sample_radius: 30.0,
            flow_softness: 20.0,
            edge_definition: 50.0,
            flow_falloff: 0.0,
            view: "final_render".to_string(),
            contrast: 100.0,
            brightness: 0.0,
            clip_hdr_results: "on".to_string(),
            channel: "rgb".to_string(),
            frame: 0,
            float: false,
        }),
        // D-444: After Effects' Vegas controls on masks; the starting values are ours, after
        // After Effects' (Image Contours, its default, is not built).
        VEGAS => Some(Effect::Vegas {
            stroke: "masks".to_string(),
            mask: 1.0,
            all_masks: "off".to_string(),
            segments: 32.0,
            length: 1.0,
            segment_distribution: "bunched".to_string(),
            rotation: 0.0,
            random_phase: "off".to_string(),
            random_seed: 1.0,
            blend_mode: "over".to_string(),
            color: "#ffffff".to_string(),
            width: 2.0,
            hardness: 0.0,
            start_opacity: 1.0,
            mid_point_opacity: 0.0,
            mid_point_position: 0.5,
            end_opacity: 0.0,
            paths: None,
        }),
        // D-425: CC Threads' controls; the numbers are ours (the manual gives none).
        THREADS => Some(Effect::Threads {
            width: 50.0,
            height: 50.0,
            overlaps: 1.0,
            direction: 0.0,
            center: [50.0, 50.0],
            coverage: 90.0,
            shadowing: 50.0,
            texture: 0.0,
        }),
        // D-317: After Effects' start, the layer's own alpha as the bump, lit from the top left.
        GLASS => Some(Effect::Glass {
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            property: "alpha".to_string(),
            softness: 10.0,
            height: 50.0,
            displacement: 100.0,
            light_angle: -45.0,
            light_color: "#ffffff".to_string(),
            light_intensity: 100.0,
            // D-561: the full light when added; a file without it keeps the classic one.
            light_type: "distant".to_string(),
            light_height: 25.0,
            light_position: [50.0, 50.0],
            ambient: 75.0,
            diffuse: 90.0,
            specular: 25.0,
            roughness: 0.1,
            metal: 100.0,
            map: None,
        }),
        // D-336: After Effects' start, the layer's own lightness, softened by 30.
        VECTOR_BLUR => Some(Effect::VectorBlur {
            kind: "natural".to_string(),
            amount: 10.0,
            angle_offset: 0.0,
            ridge_smoothness: 10.0,
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            property: "lightness".to_string(),
            map_softness: 30.0,
            map: None,
        }),
        // D-347: After Effects' start, one second each way, 60 steps a second, the layer itself.
        MOMENT_MAP => Some(Effect::MomentMap {
            max_time: 1.0,
            resolution: 60.0,
            layer: serde_json::Value::from(""),
            fit: "stretch".to_string(),
            map: None,
            picture: None,
        }),
        // D-348: After Effects' 3D Channel Extract shows the depth, white from 1 on; Depth
        // Matte starts at 0, keeping everything.
        PASS_EXTRACT => Some(Effect::PassExtract {
            pass: "depth".to_string(),
            black_point: 0.0,
            white_point: 1.0,
            invert: "off".to_string(),
            clamp: "on".to_string(),
            channel: String::new(),
            // D-552: EXtractoR's UnMult, off so a render's channels are taken as written.
            unmult: "off".to_string(),
            channels: None,
        }),
        DEPTH_KEY => Some(Effect::DepthKey {
            depth: 0.0,
            feather: 0.0,
            invert: "off".to_string(),
            channels: None,
        }),
        // D-349: After Effects' ID Matte starts on the object ids, id 0, no feather.
        ID_KEY => Some(Effect::IdKey {
            aux_channel: "object_id".to_string(),
            id: 0.0,
            feather: 0.0,
            invert: "off".to_string(),
            channels: None,
        }),
        // D-527: the layer's own time in white on a black box low in the middle, as After Effects
        // adds Timecode.
        TIMECODE => Some(Effect::Timecode {
            display_format: "smpte".to_string(),
            time_source: "layer_source".to_string(),
            time_units: 30.0,
            drop_frame: "off".to_string(),
            starting_frame: 0.0,
            text_position: [50.0, 85.0],
            text_size: 36.0,
            text_color: "#ffffff".to_string(),
            show_box: "on".to_string(),
            box_color: "#000000".to_string(),
            opacity: 100.0,
            composite_on_original: "on".to_string(),
            count: [0.0; 3],
        }),
        // D-526: a white 0 in the middle of the layer, over it, as After Effects adds Numbers.
        NUMBERS => Some(Effect::Numbers {
            kind: "number".to_string(),
            random_values: "off".to_string(),
            value: 0.0,
            decimal_places: 0.0,
            position: [50.0, 50.0],
            display: "fill_only".to_string(),
            fill_color: "#ffffff".to_string(),
            stroke_color: "#ff3300".to_string(),
            stroke_width: 3.0,
            size: 64.0,
            tracking: 0.0,
            proportional_spacing: "on".to_string(),
            composite_on_original: "on".to_string(),
            font: anime_compositor::text::Text::BUNDLED_FONT.to_string(),
            alignment: "center".to_string(),
            clock: [0.0; 2],
        }),
        // D-473: the word Text, white, in the middle of the layer, over it, as Numbers adds.
        BASIC_TEXT => Some(Effect::BasicText {
            text: "Text".to_string(),
            position: [50.0, 50.0],
            display: "fill_only".to_string(),
            fill_color: "#ffffff".to_string(),
            stroke_color: "#ff3300".to_string(),
            stroke_width: 3.0,
            size: 64.0,
            tracking: 0.0,
            composite_on_original: "on".to_string(),
            font: anime_compositor::text::Text::BUNDLED_FONT.to_string(),
            alignment: "center".to_string(),
        }),
        // D-525: fnord's IDentifier starts on the colours, so every id shows.
        IDENTIFIER => Some(Effect::Identifier {
            aux_channel: "object_id".to_string(),
            id: 0.0,
            output: "colors".to_string(),
            channels: None,
        }),
        // D-522: the first layer, nothing picked, the colours, so every object shows until one
        // is named.
        CRYPTOMATTE => Some(Effect::Cryptomatte {
            layer: String::new(),
            selection: String::new(),
            output: "colors".to_string(),
            channels: None,
        }),
        // D-524: white fog from depth 0 to 10, half density, the back fogged, no gradient.
        FOG_3D => Some(Effect::Fog3d {
            fog_color: "#ffffff".to_string(),
            fog_start_depth: 0.0,
            fog_end_depth: 10.0,
            fog_opacity: 100.0,
            scattering_density: 50.0,
            foggy_background: "on".to_string(),
            gradient_layer: serde_json::Value::from(""),
            layer_contribution: 100.0,
            map: None,
            channels: None,
        }),
        // D-523: focused at 0, a 10-pixel blur at most, no slab, the thin lens's bias.
        DEPTH_OF_FIELD => Some(Effect::DepthOfField {
            focal_plane: 0.0,
            maximum_radius: 10.0,
            focal_plane_thickness: 0.0,
            focal_bias: 50.0,
            channels: None,
        }),
        // D-350: After Effects' new animator and Range Selector: nothing moved, the whole line
        // picked, square, so a property changed acts on every letter until Start or End moves.
        TEXT_ANIMATOR => Some(Effect::TextAnimator {
            position: [0.0, 0.0],
            scale: [100.0, 100.0],
            rotation: 0.0,
            opacity: 100.0,
            fill: "off".to_string(),
            color: [1.0, 0.0, 0.0],
            tracking: 0.0,
            start: 0.0,
            end: 100.0,
            offset: 0.0,
            amount: 100.0,
            based_on: "characters".to_string(),
            shape: "square".to_string(),
            smoothness: 100.0,
            ease_high: 0.0,
            ease_low: 0.0,
        }),
        // D-351: After Effects' Auto Levels, Auto Contrast and Auto Color as added: clips 0.1 per
        // cent, no smoothing, no scene detect, no snap.
        STRETCH_LEVELS | STRETCH_CONTRAST | STRETCH_COLOR => Some(Effect::AutoTone {
            kind: match type_id {
                STRETCH_LEVELS => "levels",
                STRETCH_CONTRAST => "contrast",
                _ => "color",
            },
            temporal_smoothing: 0.0,
            scene_detect: "off".to_string(),
            black_clip: 0.1,
            white_clip: 0.1,
            snap_neutral_midtones: "off".to_string(),
            stats: None,
        }),
        SPREAD_TONES => Some(Effect::SpreadTones {
            equalize: "rgb".to_string(),
            amount: 100.0,
        }),
        // D-352: as added, a choke that takes in the edge a little and rounds it.
        MATTE_CHOKER => Some(Effect::MatteChoker {
            geometric_softness_1: 4.0,
            choke_1: 75.0,
            gray_level_softness_1: 10.0,
            geometric_softness_2: 0.0,
            choke_2: 0.0,
            gray_level_softness_2: 100.0,
            iterations: 1.0,
        }),
        // D-352: Refine Hard Matte as added softens and steepens a little; Refine Soft Matte
        // fits a 10-pixel band to the colours and decontaminates.
        REFINE_HARD_MATTE | REFINE_SOFT_MATTE => {
            let soft = type_id == REFINE_SOFT_MATTE;
            Some(Effect::RefineMatte {
                kind: if soft { "soft" } else { "hard" },
                edge_radius: if soft { 10.0 } else { 0.0 },
                view_edge_region: "off".to_string(),
                feather: if soft { 0.0 } else { 2.0 },
                contrast: if soft { 0.0 } else { 50.0 },
                shift_edge: 0.0,
                decontaminate: if soft { "on" } else { "off" }.to_string(),
                decontamination_amount: 100.0,
                decontamination_radius: 0.0,
                view_decontamination_map: "off".to_string(),
            })
        }
        // D-356: as added, After Effects' Stroke: mask 1, a white brush 2 pixels across, the
        // whole path, drawn on the layer.
        STROKE => Some(Effect::Stroke {
            mask: 1.0,
            all_masks: "off".to_string(),
            stroke_sequentially: "off".to_string(),
            color: "#ffffff".to_string(),
            brush_size: 2.0,
            brush_hardness: 75.0,
            opacity: 100.0,
            start: 0.0,
            end: 100.0,
            spacing: 15.0,
            paint_style: "on_original".to_string(),
            source: "masks".to_string(),
            paths: None,
        }),
        _ => None,
    }
}

/// The settings for an effect of `type_id`, read from the request.
///
/// Every parameter of the type has to be present. The alternative -- filling a missing one in
/// from a default -- would let a request that named one parameter quietly reset the others, and
/// this is the request a panel sends after somebody types in one field. Whether the numbers are
/// in range is document 21's decision and is made in the core, so a negative sigma is refused
/// there, in its words, rather than judged twice.
fn effect_parameters(type_id: &str, query: Option<&str>) -> Result<Effect, String> {
    let number = |name: &str| -> Result<f64, String> {
        let Some(text) = parameter(query, name) else {
            return Err(format!("What should {name} be set to?"));
        };
        text.trim()
            .parse::<f64>()
            .map_err(|_| format!("{name} needs a number. Not \"{text}\"."))
    };
    // D-89 on: a word or a colour as written, in small letters; whether it is right is the
    // core's check, in its words.
    let word = |name: &str| -> Result<String, String> {
        parameter(query, name)
            .map(|t| t.trim().to_ascii_lowercase())
            .ok_or_else(|| format!("What should {name} be set to?"))
    };
    // D-109: a blur's edges, transparent when the command does not say.
    let edges = || word("edges").unwrap_or_else(|_| "transparent".to_string());
    // Chosen colours as one comma-separated list, empty for none.
    let colors = || -> Result<Vec<String>, String> {
        Ok(word("colors")?
            .split(',')
            .map(|c| c.trim().to_string())
            .filter(|c| !c.is_empty())
            .collect())
    };
    // D-114: a point as the page sends it, two numbers, x then y, as a radial blur's centre.
    let pair = |name: &str| -> Result<[f64; 2], String> {
        let text = word(name)?;
        let parts: Vec<f64> = text
            .split(',')
            .map(|p| p.trim().parse::<f64>())
            .collect::<Result<_, _>>()
            .unwrap_or_default();
        match parts[..] {
            [x, y] => Ok([x, y]),
            _ => Err(format!(
                "{name} needs two numbers, like 50, 50. Not \"{text}\"."
            )),
        }
    };
    // D-647: Turbulent Noise's further settings; D-743: Fractal Noise's too. Those sent by a
    // window before them read as added.
    let noise_more = || -> Result<anime_compositor::effects::NoiseMore, String> {
        let start = anime_compositor::effects::NoiseMore::default();
        let num = |key: &str, start: f64| {
            if parameter(query, key).is_some() {
                number(key)
            } else {
                Ok(start)
            }
        };
        let choice = |key: &str, start: &str| {
            if parameter(query, key).is_some() {
                word(key)
            } else {
                Ok(start.to_string())
            }
        };
        Ok(anime_compositor::effects::NoiseMore {
            overflow: choice("overflow", &start.overflow)?,
            rotation: num("rotation", start.rotation)?,
            uniform_scaling: choice("uniform_scaling", &start.uniform_scaling)?,
            perspective_offset: choice("perspective_offset", &start.perspective_offset)?,
            sub_influence: num("sub_influence", start.sub_influence)?,
            sub_scaling: num("sub_scaling", start.sub_scaling)?,
            sub_rotation: num("sub_rotation", start.sub_rotation)?,
            sub_offset: if parameter(query, "sub_offset").is_some() {
                pair("sub_offset")?
            } else {
                start.sub_offset
            },
            center_subscale: choice("center_subscale", &start.center_subscale)?,
        })
    };
    // D-130, D-366: a tone as the page sends it, red, green and blue with commas. How many
    // there are is the core's check, in its words.
    let tone = |name: &str| -> Result<Vec<f64>, String> {
        let text = word(name)?;
        text.split(',')
            .map(|p| p.trim().parse::<f64>())
            .collect::<Result<_, _>>()
            .map_err(|_| format!("{name} needs three numbers, like 20, 0, -10. Not \"{text}\"."))
    };
    // D-111: a curve's points as the page sends them, in then out, a comma between points.
    // How many there are and whether they are in range is the core's check, in its words.
    let points = |name: &str| -> Result<Vec<Vec<f64>>, String> {
        let text = word(name)?;
        text.split(',')
            .map(|p| {
                p.split_whitespace()
                    .map(|n| n.parse::<f64>())
                    .collect::<Result<Vec<f64>, _>>()
                    .map_err(|_| {
                        format!(
                            "{name} needs points of two numbers, like 0 0, 128 180, 255 255. \
                             Not \"{text}\"."
                        )
                    })
            })
            .collect()
    };
    // D-545: Lens Dirt's settings, as added when the command leaves them out; the layer travels
    // as `dirt_layer`, as written, "" for none.
    let dirt = || -> Result<anime_compositor::effects::LensDirt, String> {
        let or = |name: &str, start: f64| {
            if parameter(query, name).is_some() {
                number(name)
            } else {
                Ok(start)
            }
        };
        Ok(anime_compositor::effects::LensDirt {
            layer: serde_json::Value::from(
                parameter(query, "dirt_layer").unwrap_or_default().trim(),
            ),
            opacity: or("dirt_opacity", 100.0)?,
            inherit: or("dirt_inherit", 0.0)?,
            exposure: or("dirt_exposure", 0.0)?,
            gamma: or("dirt_gamma", 1.0)?,
            repeat: if parameter(query, "dirt_repeat").is_some() {
                word("dirt_repeat")?
            } else {
                "on".to_string()
            },
            map: None,
        })
    };
    match type_id {
        EXPOSURE => Ok(Effect::Exposure {
            stops: number("stops")?,
            // D-335: none, 1 and off when the command does not say.
            offset: if parameter(query, "offset").is_some() { number("offset")? } else { 0.0 },
            gamma: if parameter(query, "gamma").is_some() { number("gamma")? } else { 1.0 },
            bypass: word("bypass").unwrap_or_else(|_| "off".to_string()),
        }),
        GAUSSIAN_BLUR => Ok(Effect::GaussianBlur {
            sigma_px: number("sigma_px")?,
            edges: edges(),
            // D-303: both when the command does not say.
            dimensions: word("dimensions").unwrap_or_else(|_| "both".to_string()),
            // D-321: sigma when the command does not say, as a file without it.
            units: word("units").unwrap_or_else(|_| "sigma".to_string()),
        }),
        // D-401: without the older `color`, After Effects' settings.
        TINT if parameter(query, "color").is_none() => Ok(Effect::TintMap {
            map_black_to: word("map_black_to")?,
            map_white_to: word("map_white_to")?,
            amount_to_tint: number("amount_to_tint")?,
        }),
        TRITONE => Ok(Effect::Tritone {
            highlights: word("highlights")?,
            midtones: word("midtones")?,
            shadows: word("shadows")?,
            blend_with_original: number("blend_with_original")?,
        }),
        AERIAL_HAZE => Ok(Effect::AerialHaze {
            haze_color: word("haze_color")?,
            amount: number("amount")?,
            layer: serde_json::Value::from(
                parameter(query, "map_layer")
                    .ok_or_else(|| "What should map_layer be set to?".to_string())?
                    .trim(),
            ),
            fit: word("fit")?,
            map: None,
        }),
        TRANSFORM => Ok(Effect::Transform {
            anchor_point: pair("anchor_point")?,
            position: pair("position")?,
            uniform_scale: word("uniform_scale")?,
            scale_height: number("scale_height")?,
            scale_width: number("scale_width")?,
            skew: number("skew")?,
            skew_axis: number("skew_axis")?,
            rotation: number("rotation")?,
            opacity: number("opacity")?,
            use_composition_shutter_angle: word("use_composition_shutter_angle")?,
            shutter_angle: number("shutter_angle")?,
            sampling: word("sampling")?,
            moments: Vec::new(),
        }),
        TINT => {
            let Some(text) = parameter(query, "color") else {
                return Err("What colour should the tint be?".to_string());
            };
            let parts: Vec<f64> = text
                .split(',')
                .map(|p| p.trim().parse::<f64>())
                .collect::<Result<_, _>>()
                .unwrap_or_default();
            let [r, g, b] = parts[..] else {
                return Err(format!(
                    "color needs three numbers, like 1, 0.5, 0. Not \"{text}\"."
                ));
            };
            Ok(Effect::Tint {
                color: [r, g, b],
                amount: number("amount")?,
            })
        }
        LINE_SMOOTH => Ok(Effect::LineSmooth {
            softness: number("softness")?,
            threshold: number("threshold")?,
        }),
        // D-87: the chosen colours as one comma-separated list, empty for none. Whether each
        // is a colour is the core's check, in its words.
        SELECTIVE_COLOR_BLUR => {
            let Some(text) = parameter(query, "colors") else {
                return Err("Which colours should be blurred?".to_string());
            };
            Ok(Effect::SelectiveColorBlur {
                blur: number("blur")?,
                colors: text
                    .split(',')
                    .map(|c| c.trim().to_ascii_lowercase())
                    .filter(|c| !c.is_empty())
                    .collect(),
                tolerance: number("tolerance")?,
            })
        }
        // D-353: a falloff other than classic is Soft Physical Glow, kept as written so a wrong
        // one is the core's to refuse in its words.
        GLOW if parameter(query, "falloff").is_some_and(|f| f != "classic") => Ok(Effect::SoftGlow {
            falloff: word("falloff")?,
            threshold_mode: word("threshold_mode")?,
            threshold: number("threshold")?,
            threshold_smooth: number("threshold_smooth")?,
            saturation_bias: number("saturation_bias")?,
            radius: number("radius")?,
            exposure: number("exposure")?,
            aspect_ratio: number("aspect_ratio")?,
            aspect_angle: number("aspect_angle")?,
            operation: word("operation")?,
            source_opacity: number("source_opacity")?,
            unmult: word("unmult")?,
            // D-542: left out, as before it.
            tint: if parameter(query, "tint").is_some() { word("tint")? } else { "off".to_string() },
            inner_color: if parameter(query, "inner_color").is_some() { word("inner_color")? } else { "#ffffff".to_string() },
            outer_color: if parameter(query, "outer_color").is_some() { word("outer_color")? } else { "#4080ff".to_string() },
            tint_amount: if parameter(query, "tint_amount").is_some() { number("tint_amount")? } else { 100.0 },
            // D-543: the same.
            spectral: if parameter(query, "spectral").is_some() { word("spectral")? } else { "off".to_string() },
            spectral_red: if parameter(query, "spectral_red").is_some() { number("spectral_red")? } else { 80.0 },
            spectral_green: if parameter(query, "spectral_green").is_some() { number("spectral_green")? } else { 100.0 },
            spectral_blue: if parameter(query, "spectral_blue").is_some() { number("spectral_blue")? } else { 120.0 },
            dirt: dirt()?,
        }),
        // D-89: the words and colours as written, the colours as one comma-separated list and
        // the tint empty for none. Whether each is right is the core's check, in its words.
        GLOW => Ok(Effect::Glow {
            based_on: word("based_on")?,
            threshold: number("threshold")?,
            colors: colors()?,
            tolerance: number("tolerance")?,
            radius: number("radius")?,
            intensity: number("intensity")?,
            operation: word("operation")?,
            tint: word("tint")?,
            // D-322: classic when the command does not say, as a file without it.
            units: word("units").unwrap_or_else(|_| "classic".to_string()),
            // D-747: each at After Effects' start when the command does not say, as a file without it.
            look: {
                let start = anime_compositor::effects::GlowLook::default();
                let w = |k: &str, s: String| if parameter(query, k).is_some() { word(k) } else { Ok(s) };
                let n = |k: &str, s: f64| if parameter(query, k).is_some() { number(k) } else { Ok(s) };
                anime_compositor::effects::GlowLook {
                    composite: w("composite", start.composite)?,
                    dimensions: w("dimensions", start.dimensions)?,
                    glow_colors: w("glow_colors", start.glow_colors)?,
                    color_looping: w("color_looping", start.color_looping)?,
                    color_loops: n("color_loops", start.color_loops)?,
                    color_phase: n("color_phase", start.color_phase)?,
                    midpoint: n("midpoint", start.midpoint)?,
                    color_a: w("color_a", start.color_a)?,
                    color_b: w("color_b", start.color_b)?,
                }
            },
        }),
        LINE_RECOLOR => Ok(Effect::LineRecolor {
            colors: colors()?,
            tolerance: number("tolerance")?,
            new_color: word("new_color")?,
        }),
        DIRECTIONAL_BLUR => Ok(Effect::DirectionalBlur {
            direction: number("direction")?,
            length: number("length")?,
            edges: edges(),
        }),
        SELECT_COLOR => Ok(Effect::SelectColor {
            colors: colors()?,
            tolerance: number("tolerance")?,
            keep: word("keep")?,
        }),
        LINE_WIDTH => Ok(Effect::LineWidth {
            width: number("width")?,
            based_on: word("based_on")?,
            colors: colors()?,
            tolerance: number("tolerance")?,
        }),
        // D-95: the centre as two numbers, x then y, as a tint's colour is three.
        RADIAL_BLUR => {
            let Some(text) = parameter(query, "center") else {
                return Err("Where should the centre be?".to_string());
            };
            let parts: Vec<f64> = text
                .split(',')
                .map(|p| p.trim().parse::<f64>())
                .collect::<Result<_, _>>()
                .unwrap_or_default();
            let [x, y] = parts[..] else {
                return Err(format!(
                    "center needs two numbers, like 50, 50. Not \"{text}\"."
                ));
            };
            Ok(Effect::RadialBlur {
                kind: word("type")?,
                amount: number("amount")?,
                center: [x, y],
                edges: edges(),
            })
        }
        BLOOM => Ok(Effect::Bloom {
            threshold: number("threshold")?,
            radius: number("radius")?,
            intensity: number("intensity")?,
            streaks: word("streaks")?,
            length: number("length")?,
            angle: number("angle")?,
        }),
        COLOR_KEY => Ok(Effect::ColorKey {
            colors: colors()?,
            tolerance: number("tolerance")?,
            softness: number("softness")?,
            match_by: word("match")?,
        }),
        CURVES => Ok(Effect::Curves {
            master: points("master")?,
            red: points("red")?,
            green: points("green")?,
            blue: points("blue")?,
            // D-302: straight when the command does not say.
            alpha: if parameter(query, "alpha").is_some() { points("alpha")? } else { vec![vec![0.0, 0.0], vec![255.0, 255.0]] },
        }),
        // D-383: with the Channel menu, or as Levels (Individual Controls), a set for each
        // channel, one the command does not name at its start.
        LEVELS if parameter(query, "channel").is_none() => Ok(Effect::Levels {
            input_black: number("input_black")?,
            input_white: number("input_white")?,
            gamma: number("gamma")?,
            output_black: number("output_black")?,
            output_white: number("output_white")?,
        }),
        LEVELS | LEVELS_INDIVIDUAL => {
            let mut sets = [LEVELS_PLAIN; 5];
            for (i, (set, names)) in sets.iter_mut().zip(LEVELS_NAMES).enumerate() {
                for (v, name) in set.iter_mut().zip(names) {
                    if i == 0 || parameter(query, name).is_some() {
                        *v = number(name)?;
                    }
                }
            }
            let individual = type_id == LEVELS_INDIVIDUAL;
            let channel = if individual { "rgb".to_string() } else { word("channel")? };
            Ok(Effect::ChannelLevels { individual, channel, sets })
        }
        HUE_SATURATION => Ok(Effect::HueSaturation {
            hue: number("hue")?,
            saturation: number("saturation")?,
            lightness: number("lightness")?,
            // D-307: each range as the page sends it, hue, saturation and lightness with commas;
            // one the command does not name stays at 0.
            ranges: {
                let mut ranges = [[0.0; 3]; 6];
                for (r, name) in ranges.iter_mut().zip(HUE_RANGES) {
                    if parameter(query, name).is_some() {
                        let text = word(name)?;
                        *r = text
                            .split(',')
                            .map(|p| p.trim().parse::<f64>())
                            .collect::<Result<Vec<_>, _>>()
                            .ok()
                            .and_then(|v| <[f64; 3]>::try_from(v).ok())
                            .ok_or_else(|| format!("{name} needs three numbers, like 14, 0, 0. Not \"{text}\"."))?;
                    }
                }
                ranges
            },
        }),
        GRADIENT => Ok(Effect::Gradient {
            shape: word("shape")?,
            start: pair("start")?,
            end: pair("end")?,
            start_color: word("start_color")?,
            end_color: word("end_color")?,
            start_opacity: number("start_opacity")?,
            end_opacity: number("end_opacity")?,
            blend: word("blend")?,
        }),
        DROP_SHADOW => Ok(Effect::DropShadow {
            color: word("color")?,
            opacity: number("opacity")?,
            direction: number("direction")?,
            distance: number("distance")?,
            softness: number("softness")?,
        }),
        LENS_BLUR => Ok(Effect::LensBlur {
            radius: number("radius")?,
            edges: edges(),
            iris: word("iris")?,
            roundness: number("roundness")?,
            rotation: number("rotation")?,
            aspect: number("aspect")?,
            highlight_gain: number("highlight_gain")?,
            highlight_threshold: number("highlight_threshold")?,
            // D-359: the blur map, travelling as `blur_layer` as Compound Blur's does; none, and
            // AE's start values, when the command does not say.
            layer: serde_json::Value::from(parameter(query, "blur_layer").unwrap_or_default().trim()),
            fit: word("fit").unwrap_or_else(|_| "center".to_string()),
            channel: word("channel").unwrap_or_else(|_| "luminance".to_string()),
            focal_distance: if parameter(query, "focal_distance").is_some() { number("focal_distance")? } else { 0.0 },
            invert: word("invert").unwrap_or_else(|_| "off".to_string()),
            map: None,
        }),
        RIM_LIGHT => Ok(Effect::RimLight {
            color: word("color")?,
            direction: number("direction")?,
            width: number("width")?,
            softness: number("softness")?,
            intensity: number("intensity")?,
            blend: word("blend")?,
        }),
        OUTLINE => Ok(Effect::Outline {
            color: word("color")?,
            width: number("width")?,
            softness: number("softness")?,
            opacity: number("opacity")?,
        }),
        NOISE => Ok(Effect::Noise {
            amount: number("amount")?,
            mode: word("mode")?,
            seed: number("seed")?,
            animate: word("animate")?,
            frame: 0,
        }),
        // D-409: without a mode, D-120's form, as an older project holds it.
        CHROMATIC_ABERRATION if parameter(query, "mode").is_none() => Ok(Effect::ChromaticAberration {
            amount: number("amount")?,
            center: pair("center")?,
        }),
        CHROMATIC_ABERRATION => Ok(Effect::LensChromaticAberration {
            mode: word("mode")?,
            amount: number("amount")?,
            center: pair("center")?,
            angle: number("angle")?,
            falloff: number("falloff")?,
            red_scale: number("red_scale")?,
            green_scale: number("green_scale")?,
            blue_scale: number("blue_scale")?,
            fringe_blur: number("fringe_blur")?,
        }),
        DISTANCE_GRADATION => Ok(Effect::DistanceGradation {
            color: word("color")?,
            width: number("width")?,
            opacity: number("opacity")?,
            invert: word("invert")?,
            blend: word("blend")?,
        }),
        // D-423: with a length, D-124's form, as an older project holds it.
        LIGHT_RAYS if parameter(query, "length").is_some() => Ok(Effect::LightRays {
            center: pair("center")?,
            length: number("length")?,
            threshold: number("threshold")?,
            intensity: number("intensity")?,
            color: word("color")?,
        }),
        LIGHT_RAYS => Ok(Effect::CcLightRays {
            intensity: number("intensity")?,
            center: pair("center")?,
            radius: number("radius")?,
            warp_softness: number("warp_softness")?,
            shape: word("shape")?,
            direction: number("direction")?,
            color_from_source: word("color_from_source")?,
            allow_brightening: word("allow_brightening")?,
            color: word("color")?,
            transfer_mode: word("transfer_mode")?,
        }),
        EXPOSURE_FLICKER => Ok(Effect::ExposureFlicker {
            amount: number("amount")?,
            hold: number("hold")?,
            seed: number("seed")?,
            frame: 0,
        }),
        VIGNETTE => Ok(Effect::Vignette {
            amount: number("amount")?,
            color: word("color")?,
            size: number("size")?,
            roundness: number("roundness")?,
            softness: number("softness")?,
            center: pair("center")?,
        }),
        CC_VIGNETTE => Ok(Effect::CcVignette {
            amount: number("amount")?,
            angle_of_view: number("angle_of_view")?,
            center: pair("center")?,
            pin_highlights: number("pin_highlights")?,
        }),
        TURBULENT_DISPLACE => Ok(Effect::TurbulentDisplace {
            amount: number("amount")?,
            size: number("size")?,
            complexity: number("complexity")?,
            evolution: number("evolution")?,
            speed: number("speed")?,
            seed: number("seed")?,
            edges: word("edges")?,
            frame: 0,
            // D-306: each where it starts when the command does not say.
            displacement: word("displacement").unwrap_or_else(|_| "turbulent".to_string()),
            pinning: word("pinning").unwrap_or_else(|_| "none".to_string()),
            units: word("units").unwrap_or_else(|_| "classic".to_string()),
            // D-410: one seed when the command does not say.
            new_seed_every: if parameter(query, "new_seed_every").is_some() { number("new_seed_every")? } else { 0.0 },
            // D-411: no drift when the command does not say.
            drift_direction: if parameter(query, "drift_direction").is_some() { number("drift_direction")? } else { 0.0 },
            drift_speed: if parameter(query, "drift_speed").is_some() { number("drift_speed")? } else { 0.0 },
        }),
        FRACTAL_NOISE => Ok(Effect::FractalNoise {
            size: number("size")?,
            complexity: number("complexity")?,
            contrast: number("contrast")?,
            brightness: number("brightness")?,
            evolution: number("evolution")?,
            speed: number("speed")?,
            seed: number("seed")?,
            dark_color: word("dark_color")?,
            light_color: word("light_color")?,
            opacity: number("opacity")?,
            blend: word("blend")?,
            // D-299: each where it starts when the command does not say, as edges are.
            fractal_type: word("fractal_type").unwrap_or_else(|_| "basic".to_string()),
            noise_type: word("noise_type").unwrap_or_else(|_| "smooth".to_string()),
            invert: word("invert").unwrap_or_else(|_| "off".to_string()),
            offset: if parameter(query, "offset").is_some() { pair("offset")? } else { [0.0, 0.0] },
            scale_width: if parameter(query, "scale_width").is_some() { number("scale_width")? } else { 100.0 },
            scale_height: if parameter(query, "scale_height").is_some() { number("scale_height")? } else { 100.0 },
            cycle: if parameter(query, "cycle").is_some() { number("cycle")? } else { 0.0 },
            frame: 0,
            float: false,
            // D-743: Uniform Scaling stays off; Fractal Noise has Scale Width and Height.
            more: anime_compositor::effects::NoiseMore {
                uniform_scaling: "off".into(),
                ..noise_more()?
            },
        }),
        GRADIENT_MAP => Ok(Effect::GradientMap {
            shadow_color: word("shadow_color")?,
            midtone_color: word("midtone_color")?,
            highlight_color: word("highlight_color")?,
            midpoint: number("midpoint")?,
            amount: number("amount")?,
        }),
        COLOR_BALANCE => Ok(Effect::ColorBalance {
            shadows: tone("shadows")?,
            midtones: tone("midtones")?,
            highlights: tone("highlights")?,
            preserve_luminosity: word("preserve_luminosity")?,
        }),
        OFFSET => Ok(Effect::Offset {
            shift: pair("shift")?,
        }),
        LIGHT_WRAP => Ok(Effect::LightWrap {
            width: number("width")?,
            intensity: number("intensity")?,
            blend: word("blend")?,
        }),
        INVERT => Ok(Effect::Invert {
            channel: word("channel")?,
            amount: number("amount")?,
        }),
        BRIGHTNESS_CONTRAST => Ok(Effect::BrightnessContrast {
            brightness: number("brightness")?,
            contrast: number("contrast")?,
        }),
        BLACK_WHITE => Ok(Effect::BlackWhite {
            reds: number("reds")?,
            yellows: number("yellows")?,
            greens: number("greens")?,
            cyans: number("cyans")?,
            blues: number("blues")?,
            magentas: number("magentas")?,
        }),
        POSTERIZE => Ok(Effect::Posterize {
            levels: number("levels")?,
        }),
        THRESHOLD => Ok(Effect::Threshold {
            level: number("level")?,
        }),
        // D-139: each row as the page sends it, four numbers with commas. How many there are is
        // the core's check, in its words.
        CHANNEL_MIXER => {
            let row = |name: &str| -> Result<Vec<f64>, String> {
                let text = word(name)?;
                text.split(',')
                    .map(|p| p.trim().parse::<f64>())
                    .collect::<Result<_, _>>()
                    .map_err(|_| format!("{name} needs four numbers, like 100, 0, 0, 0. Not \"{text}\"."))
            };
            Ok(Effect::ChannelMixer {
                red: row("red")?,
                green: row("green")?,
                blue: row("blue")?,
                monochrome: word("monochrome")?,
            })
        }
        VIBRANCE => Ok(Effect::Vibrance {
            vibrance: number("vibrance")?,
            saturation: number("saturation")?,
        }),
        LEAVE_COLOR => Ok(Effect::LeaveColor {
            color: word("color")?,
            tolerance: number("tolerance")?,
            softness: number("softness")?,
            amount: number("amount")?,
        }),
        SOLARIZE => Ok(Effect::Solarize {
            threshold: number("threshold")?,
        }),
        HALFTONE => Ok(Effect::Halftone {
            size: number("size")?,
            angle: number("angle")?,
            ink: word("ink")?,
            paper: word("paper")?,
            amount: number("amount")?,
        }),
        MOSAIC => Ok(Effect::Mosaic {
            size: number("size")?,
            // D-754: a command without them means what a file without them means.
            mode: if parameter(query, "mode").is_some() { word("mode")? } else { "size".to_string() },
            horizontal: if parameter(query, "horizontal").is_some() { number("horizontal")? } else { 10.0 },
            vertical: if parameter(query, "vertical").is_some() { number("vertical")? } else { 10.0 },
            sharp_colors: if parameter(query, "sharp_colors").is_some() {
                word("sharp_colors")?
            } else {
                "off".to_string()
            },
        }),
        EMBOSS => Ok(Effect::Emboss {
            direction: number("direction")?,
            relief: number("relief")?,
            contrast: number("contrast")?,
            mode: word("mode")?,
        }),
        FIND_EDGES => Ok(Effect::FindEdges {
            invert: word("invert")?,
            amount: number("amount")?,
        }),
        SHARPEN => Ok(Effect::Sharpen {
            amount: number("amount")?,
            radius: number("radius")?,
            threshold: if parameter(query, "threshold").is_some() { number("threshold")? } else { 0.0 },
        }),
        DIFFUSION => Ok(Effect::Diffusion {
            radius: number("radius")?,
            amount: number("amount")?,
            blend: word("blend")?,
            second_amount: if parameter(query, "second_amount").is_some() { number("second_amount")? } else { 0.0 },
            second_blend: word("second_blend").unwrap_or_else(|_| "soft_light".to_string()),
        }),
        WAVE_WARP => Ok(Effect::WaveWarp {
            shape: word("shape")?,
            height: number("height")?,
            width: number("width")?,
            direction: number("direction")?,
            speed: number("speed")?,
            phase: number("phase")?,
            edges: word("edges")?,
            pinning: if parameter(query, "pinning").is_some() { word("pinning")? } else { "none".to_string() },
            frame: 0,
        }),
        RIPPLE => Ok(Effect::Ripple {
            center: pair("center")?,
            amplitude: number("amplitude")?,
            wavelength: number("wavelength")?,
            speed: number("speed")?,
            phase: number("phase")?,
            fade: number("fade")?,
            frame: 0,
        }),
        TWIRL => Ok(Effect::Twirl {
            angle: number("angle")?,
            radius: number("radius")?,
            center: pair("center")?,
            twirl_radius: if parameter(query, "twirl_radius").is_some() { number("twirl_radius")? } else { 0.0 },
        }),
        BULGE => Ok(Effect::Bulge {
            center: pair("center")?,
            radius: number("radius")?,
            height: number("height")?,
            vertical_radius: if parameter(query, "vertical_radius").is_some() { number("vertical_radius")? } else { 0.0 },
            taper_radius: if parameter(query, "taper_radius").is_some() { number("taper_radius")? } else { 0.0 },
        }),
        MIRROR => Ok(Effect::Mirror {
            center: pair("center")?,
            angle: number("angle")?,
        }),
        MOTION_TILE => Ok(Effect::MotionTile {
            output_width: number("output_width")?,
            output_height: number("output_height")?,
            mirror: word("mirror")?,
            // D-304: each where it starts when the command does not say.
            tile_center: if parameter(query, "tile_center").is_some() { pair("tile_center")? } else { [50.0, 50.0] },
            tile_width: if parameter(query, "tile_width").is_some() { number("tile_width")? } else { 100.0 },
            tile_height: if parameter(query, "tile_height").is_some() { number("tile_height")? } else { 100.0 },
        }),
        LINEAR_WIPE => Ok(Effect::LinearWipe {
            completion: number("completion")?,
            angle: number("angle")?,
            feather: number("feather")?,
        }),
        RADIAL_WIPE => Ok(Effect::RadialWipe {
            completion: number("completion")?,
            start_angle: number("start_angle")?,
            center: pair("center")?,
            wipe: word("wipe")?,
            feather: number("feather")?,
        }),
        VENETIAN_BLINDS => Ok(Effect::VenetianBlinds {
            completion: number("completion")?,
            angle: number("angle")?,
            width: number("width")?,
            feather: number("feather")?,
        }),
        IRIS_WIPE => Ok(Effect::IrisWipe {
            completion: number("completion")?,
            center: pair("center")?,
            feather: number("feather")?,
            invert: word("invert")?,
        }),
        SIMPLE_CHOKER => Ok(Effect::SimpleChoker {
            choke: number("choke")?,
        }),
        SPEED_LINES => Ok(Effect::SpeedLines {
            center: pair("center")?,
            color: word("color")?,
            count: number("count")?,
            thickness: number("thickness")?,
            inner: number("inner")?,
            inner_jitter: number("inner_jitter")?,
            angle_jitter: number("angle_jitter")?,
            seed: number("seed")?,
            hold: number("hold")?,
            opacity: number("opacity")?,
            frame: 0,
        }),
        CROSS_GLARE => Ok(Effect::CrossGlare {
            threshold: number("threshold")?,
            length: number("length")?,
            points: number("points")?,
            angle: number("angle")?,
            intensity: number("intensity")?,
            color: word("color")?,
            // D-546: Glint Streaks' settings, as added when the command leaves them out.
            glint: anime_compositor::effects::Glint {
                ray_colors: if parameter(query, "ray_colors").is_some() { word("ray_colors")? } else { "one".to_string() },
                color_2: if parameter(query, "color_2").is_some() { word("color_2")? } else { "#80c0ff".to_string() },
                shimmer: if parameter(query, "shimmer").is_some() { number("shimmer")? } else { 0.0 },
                shimmer_period: if parameter(query, "shimmer_period").is_some() { number("shimmer_period")? } else { 24.0 },
                frame: 0,
            },
        }),
        CAMERA_SHAKE => Ok(Effect::CameraShake {
            amount: number("amount")?,
            rotation: number("rotation")?,
            hold: number("hold")?,
            seed: number("seed")?,
            frame: 0,
        }),
        RAIN => Ok(Effect::Rain {
            color: word("color")?,
            density: number("density")?,
            spacing: number("spacing")?,
            length: number("length")?,
            width: number("width")?,
            direction: number("direction")?,
            speed: number("speed")?,
            seed: number("seed")?,
            opacity: number("opacity")?,
            frame: 0,
        }),
        // D-182: an asset's id as written, not in small letters.
        COLOR_LOOKUP => Ok(Effect::ColorLookup {
            lut: parameter(query, "lut")
                .map(|t| t.trim().to_string())
                .ok_or_else(|| "What should lut be set to?".to_string())?,
            table: None,
        }),
        LINE_BLUR => Ok(Effect::LineBlur {
            length: number("length")?,
            strength: number("strength")?,
            lines_only: word("lines_only")?,
        }),
        HSV_KEY => Ok(Effect::HsvKey {
            hue: number("hue")?,
            saturation: number("saturation")?,
            value: number("value")?,
            hue_range: number("hue_range")?,
            saturation_range: number("saturation_range")?,
            value_range: number("value_range")?,
            invert: word("invert")?,
        }),
        PARAFFIN => Ok(Effect::Paraffin {
            color: word("color")?,
            direction: number("direction")?,
            spread: number("spread")?,
            opacity: number("opacity")?,
            blend: word("blend")?,
        }),
        KIRA_KIRA => Ok(Effect::KiraKira {
            threshold: number("threshold")?,
            spacing: number("spacing")?,
            density: number("density")?,
            size: number("size")?,
            shape: word("shape")?,
            angle: number("angle")?,
            twinkle: number("twinkle")?,
            period: number("period")?,
            seed: number("seed")?,
            opacity: number("opacity")?,
            color: word("color")?,
            frame: 0,
        }),
        LIGHTNING_BOLT => Ok(Effect::LightningBolt {
            start: pair("start")?,
            end: pair("end")?,
            jagged: number("jagged")?,
            detail: number("detail")?,
            branches: number("branches")?,
            width: number("width")?,
            glow: number("glow")?,
            opacity: number("opacity")?,
            hold: number("hold")?,
            seed: number("seed")?,
            color: word("color")?,
            glow_color: word("glow_color")?,
            // D-300: on, as before, when the command does not say.
            composite: word("composite").unwrap_or_else(|_| "on".to_string()),
            // D-324: D-190's bolt when the command does not say.
            kind: word("kind").unwrap_or_else(|_| "direction".to_string()),
            turbulence: if parameter(query, "turbulence").is_some() { number("turbulence")? } else { 0.0 },
            decay: if parameter(query, "decay").is_some() { number("decay")? } else { 0.0 },
            conductivity: if parameter(query, "conductivity").is_some() { number("conductivity")? } else { 0.0 },
            obstacle: if parameter(query, "obstacle").is_some() { number("obstacle")? } else { 0.0 },
            // D-329: stopping at an obstacle, as D-324's, when the command does not say.
            path: word("path").unwrap_or_else(|_| "split".to_string()),
            // D-334: the hard core, as D-190's, when the command does not say.
            core: word("core").unwrap_or_else(|_| "hard".to_string()),
            // D-338: the short forks, as D-190's, when the command does not say.
            forks: word("forks").unwrap_or_else(|_| "short".to_string()),
            frame: 0,
        }),
        // D-189: the request's own `layer` is the layer holding the effect, so the setting
        // travels as `blur_layer`, and as written: an identifier is not put in small letters.
        COMPOUND_BLUR => Ok(Effect::CompoundBlur {
            layer: serde_json::Value::from(
                parameter(query, "blur_layer")
                    .ok_or_else(|| "What should blur_layer be set to?".to_string())?
                    .trim(),
            ),
            fit: word("fit")?,
            max_blur: number("max_blur")?,
            invert: word("invert")?,
            edges: edges(),
            map: None,
        }),
        // D-189: as Compound Blur's, the setting travels as `map_layer`.
        DISPLACEMENT_MAP => Ok(Effect::DisplacementMap {
            layer: serde_json::Value::from(
                parameter(query, "map_layer")
                    .ok_or_else(|| "What should map_layer be set to?".to_string())?
                    .trim(),
            ),
            fit: word("fit")?,
            horizontal: word("horizontal")?,
            max_horizontal: number("max_horizontal")?,
            vertical: word("vertical")?,
            max_vertical: number("max_vertical")?,
            wrap: word("wrap")?,
            // D-315: off when the command does not say.
            expand: word("expand").unwrap_or_else(|_| "off".to_string()),
            // D-412: every colour moving alike, three samples, when the command does not say.
            red_amount: if parameter(query, "red_amount").is_some() { number("red_amount")? } else { 100.0 },
            green_amount: if parameter(query, "green_amount").is_some() { number("green_amount")? } else { 100.0 },
            blue_amount: if parameter(query, "blue_amount").is_some() { number("blue_amount")? } else { 100.0 },
            spectrum: if parameter(query, "spectrum").is_some() { number("spectrum")? } else { 3.0 },
            map: None,
        }),
        // D-189: as Compound Blur's, the setting travels as `map_layer`.
        GRADIENT_WIPE => Ok(Effect::GradientWipe {
            layer: serde_json::Value::from(
                parameter(query, "map_layer")
                    .ok_or_else(|| "What should map_layer be set to?".to_string())?
                    .trim(),
            ),
            fit: word("fit")?,
            completion: number("completion")?,
            softness: number("softness")?,
            invert: word("invert")?,
            map: None,
        }),
        ECHO => Ok(Effect::Echo {
            echo_time: number("echo_time")?,
            echoes: number("echoes")?,
            intensity: number("intensity")?,
            decay: number("decay")?,
            operator: word("operator")?,
            picture: None,
        }),
        CC_WIDE_TIME => Ok(Effect::CcWideTime {
            forward_steps: number("forward_steps")?,
            backward_steps: number("backward_steps")?,
            native_motion_blur: word("native_motion_blur")?,
            picture: None,
        }),
        FORCE_MOTION_BLUR => Ok(Effect::ForceMotionBlur {
            motion_blur_samples: number("motion_blur_samples")?,
            override_native_shutter: word("override_native_shutter")?,
            shutter_angle: number("shutter_angle")?,
            native_motion_blur: word("native_motion_blur")?,
            picture: None,
        }),
        PIXEL_MOTION_BLUR => Ok(Effect::PixelMotionBlur {
            shutter_control: word("shutter_control")?,
            shutter_angle: number("shutter_angle")?,
            shutter_samples: number("shutter_samples")?,
            vector_detail: number("vector_detail")?,
            frames: None,
        }),
        TIMEWARP => Ok(Effect::Timewarp {
            method: word("method")?,
            adjust_time_by: word("adjust_time_by")?,
            speed: number("speed")?,
            source_frame: number("source_frame")?,
            vector_detail: number("vector_detail")?,
            global_smoothness: number("global_smoothness")?,
            local_smoothness: number("local_smoothness")?,
            smoothing_iterations: number("smoothing_iterations")?,
            build_from_one_image: word("build_from_one_image")?,
            correct_luminance_changes: word("correct_luminance_changes")?,
            filtering: word("filtering")?,
            error_threshold: number("error_threshold")?,
            block_size: number("block_size")?,
            red_weight: number("red_weight")?,
            green_weight: number("green_weight")?,
            blue_weight: number("blue_weight")?,
            enable_motion_blur: word("enable_motion_blur")?,
            shutter_control: word("shutter_control")?,
            shutter_angle: number("shutter_angle")?,
            shutter_samples: number("shutter_samples")?,
            picture: None,
        }),
        WARP_STABILIZER => Ok(Effect::WarpStabilizer {
            result: word("result")?,
            smoothness: number("smoothness")?,
            method: word("method")?,
            preserve_scale: word("preserve_scale")?,
            framing: word("framing")?,
            maximum_scale: number("maximum_scale")?,
            action_safe_margin: number("action_safe_margin")?,
            additional_scale: number("additional_scale")?,
            detailed_analysis: word("detailed_analysis")?,
            picture: None,
        }),
        ROLLING_SHUTTER_REPAIR => Ok(Effect::RollingShutterRepair {
            rolling_shutter_rate: number("rolling_shutter_rate")?,
            scan_direction: word("scan_direction")?,
            method: word("method")?,
            detailed_analysis: word("detailed_analysis")?,
            pixel_motion_detail: number("pixel_motion_detail")?,
            picture: None,
        }),
        CC_RAINFALL => Ok(Effect::CcRainfall {
            drops: number("drops")?,
            size: number("size")?,
            scene_depth: number("scene_depth")?,
            speed: number("speed")?,
            wind: number("wind")?,
            wind_variation: number("wind_variation")?,
            spread: number("spread")?,
            opacity: number("opacity")?,
            ground_level: number("ground_level")?,
            random_seed: number("random_seed")?,
            color: word("color")?,
            transfer_mode: word("transfer_mode")?,
            composite_with_original: word("composite_with_original")?,
            clock: [0.0, 0.0],
        }),
        WAVE_WORLD => Ok(Effect::WaveWorld {
            brightness: number("brightness")?,
            contrast: number("contrast")?,
            gamma: number("gamma")?,
            transparency: number("transparency")?,
            grid_resolution: number("grid_resolution")?,
            wave_speed: number("wave_speed")?,
            damping: number("damping")?,
            pre_roll: number("pre_roll")?,
            steepness: number("steepness")?,
            height: number("height")?,
            wave_strength: number("wave_strength")?,
            render_dry_areas: word("render_dry_areas")?,
            grid_res_downsamples: word("grid_res_downsamples")?,
            reflect_edges: word("reflect_edges")?,
            ground_layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            ground_map: None,
            producer_1_type: word("producer_1_type")?,
            producer_1_position: pair("producer_1_position")?.to_vec(),
            producer_1_height: number("producer_1_height")?,
            producer_1_width: number("producer_1_width")?,
            producer_1_angle: number("producer_1_angle")?,
            producer_1_amplitude: number("producer_1_amplitude")?,
            producer_1_frequency: number("producer_1_frequency")?,
            producer_1_phase: number("producer_1_phase")?,
            producer_2_type: word("producer_2_type")?,
            producer_2_position: pair("producer_2_position")?.to_vec(),
            producer_2_height: number("producer_2_height")?,
            producer_2_width: number("producer_2_width")?,
            producer_2_angle: number("producer_2_angle")?,
            producer_2_amplitude: number("producer_2_amplitude")?,
            producer_2_frequency: number("producer_2_frequency")?,
            producer_2_phase: number("producer_2_phase")?,
            // D-635: each as added when the command does not say.
            view: if parameter(query, "view").is_some() { word("view")? } else { "height_map".to_string() },
            horizontal_rotation: if parameter(query, "horizontal_rotation").is_some() { number("horizontal_rotation")? } else { 0.0 },
            vertical_rotation: if parameter(query, "vertical_rotation").is_some() { number("vertical_rotation")? } else { 30.0 },
            vertical_scale: if parameter(query, "vertical_scale").is_some() { number("vertical_scale")? } else { 1.0 },
            history: Vec::new(),
            fps: 0.0,
            grid_scale: 1.0,
        }),
        SHATTER => Ok(Effect::Shatter {
            view: word("view")?,
            render: word("render")?,
            pattern: word("pattern")?,
            white_tiles_fixed: word("white_tiles_fixed")?,
            invert_gradient: word("invert_gradient")?,
            tumble_axis: word("tumble_axis")?,
            color: word("color")?,
            front_mode: word("front_mode")?,
            side_mode: word("side_mode")?,
            back_mode: word("back_mode")?,
            camera_system: word("camera_system")?,
            light_type: word("light_type")?,
            light_color: word("light_color")?,
            repetitions: number("repetitions")?,
            direction: number("direction")?,
            extrusion_depth: number("extrusion_depth")?,
            force_1_depth: number("force_1_depth")?,
            force_1_radius: number("force_1_radius")?,
            force_1_strength: number("force_1_strength")?,
            force_2_depth: number("force_2_depth")?,
            force_2_radius: number("force_2_radius")?,
            force_2_strength: number("force_2_strength")?,
            shatter_threshold: number("shatter_threshold")?,
            rotation_speed: number("rotation_speed")?,
            randomness: number("randomness")?,
            viscosity: number("viscosity")?,
            mass_variance: number("mass_variance")?,
            gravity: number("gravity")?,
            gravity_direction: number("gravity_direction")?,
            gravity_inclination: number("gravity_inclination")?,
            opacity: number("opacity")?,
            camera_x_rotation: number("camera_x_rotation")?,
            camera_y_rotation: number("camera_y_rotation")?,
            camera_z_rotation: number("camera_z_rotation")?,
            camera_z_position: number("camera_z_position")?,
            focal_length: number("focal_length")?,
            light_intensity: number("light_intensity")?,
            light_depth: number("light_depth")?,
            ambient_light: number("ambient_light")?,
            diffuse_reflection: number("diffuse_reflection")?,
            specular_reflection: number("specular_reflection")?,
            highlight_sharpness: number("highlight_sharpness")?,
            origin: pair("origin")?,
            // D-634: each as added when the command does not say.
            upper_left_corner: if parameter(query, "upper_left_corner").is_some() { pair("upper_left_corner")? } else { [0.0, 0.0] },
            upper_right_corner: if parameter(query, "upper_right_corner").is_some() { pair("upper_right_corner")? } else { [100.0, 0.0] },
            lower_left_corner: if parameter(query, "lower_left_corner").is_some() { pair("lower_left_corner")? } else { [0.0, 100.0] },
            lower_right_corner: if parameter(query, "lower_right_corner").is_some() { pair("lower_right_corner")? } else { [100.0, 100.0] },
            auto_focal_length: if parameter(query, "auto_focal_length").is_some() { word("auto_focal_length")? } else { "off".to_string() },
            force_1_position: pair("force_1_position")?,
            force_2_position: pair("force_2_position")?,
            camera_xy_position: pair("camera_xy_position")?,
            light_position: pair("light_position")?,
            custom_shatter_map: serde_json::Value::from(parameter(query, "custom_shatter_map").unwrap_or_default().trim()),
            custom_map: None,
            gradient_layer: serde_json::Value::from(parameter(query, "gradient_layer").unwrap_or_default().trim()),
            gradient_map: None,
            front_layer: serde_json::Value::from(parameter(query, "front_layer").unwrap_or_default().trim()),
            front_map: None,
            side_layer: serde_json::Value::from(parameter(query, "side_layer").unwrap_or_default().trim()),
            side_map: None,
            back_layer: serde_json::Value::from(parameter(query, "back_layer").unwrap_or_default().trim()),
            back_map: None,
            seen: [0.0; 3],
            history: Vec::new(),
            fps: 0.0,
        }),
        CARD_DANCE => Ok(Effect::CardDance {
            rows_columns: word("rows_columns")?,
            rows: number("rows")?,
            columns: number("columns")?,
            back_layer: serde_json::Value::from(parameter(query, "back_layer").unwrap_or_default().trim()),
            back_map: None,
            gradient_layer_1: serde_json::Value::from(parameter(query, "gradient_layer_1").unwrap_or_default().trim()),
            gradient_map_1: None,
            gradient_layer_2: serde_json::Value::from(parameter(query, "gradient_layer_2").unwrap_or_default().trim()),
            gradient_map_2: None,
            rotation_order: word("rotation_order")?,
            transformation_order: word("transformation_order")?,
            sources: CARD_DANCE_TRANSFORMS.map(|t| word(&format!("{t}_source"))).into_iter().collect::<Result<Vec<_>, _>>()?.try_into().unwrap(),
            multipliers: CARD_DANCE_TRANSFORMS.map(|t| number(&format!("{t}_multiplier"))).into_iter().collect::<Result<Vec<_>, _>>()?.try_into().unwrap(),
            offsets: CARD_DANCE_TRANSFORMS.map(|t| number(&format!("{t}_offset"))).into_iter().collect::<Result<Vec<_>, _>>()?.try_into().unwrap(),
            camera_system: word("camera_system")?,
            camera_x_rotation: number("camera_x_rotation")?,
            camera_y_rotation: number("camera_y_rotation")?,
            camera_z_rotation: number("camera_z_rotation")?,
            camera_xy_position: pair("camera_xy_position")?,
            camera_z_position: number("camera_z_position")?,
            focal_length: number("focal_length")?,
            light_type: word("light_type")?,
            light_intensity: number("light_intensity")?,
            light_color: word("light_color")?,
            light_position: pair("light_position")?,
            light_depth: number("light_depth")?,
            ambient_light: number("ambient_light")?,
            diffuse_reflection: number("diffuse_reflection")?,
            specular_reflection: number("specular_reflection")?,
            highlight_sharpness: number("highlight_sharpness")?,
            // D-634: each as added when the command does not say.
            upper_left_corner: if parameter(query, "upper_left_corner").is_some() { pair("upper_left_corner")? } else { [0.0, 0.0] },
            upper_right_corner: if parameter(query, "upper_right_corner").is_some() { pair("upper_right_corner")? } else { [100.0, 0.0] },
            lower_left_corner: if parameter(query, "lower_left_corner").is_some() { pair("lower_left_corner")? } else { [0.0, 100.0] },
            lower_right_corner: if parameter(query, "lower_right_corner").is_some() { pair("lower_right_corner")? } else { [100.0, 100.0] },
            auto_focal_length: if parameter(query, "auto_focal_length").is_some() { word("auto_focal_length")? } else { "off".to_string() },
            view: [0.0; 3],
        }),
        CAUSTICS => Ok(Effect::Caustics {
            bottom_layer: serde_json::Value::from(parameter(query, "bottom_layer").unwrap_or_default().trim()),
            bottom_map: None,
            bottom_scaling: number("bottom_scaling")?,
            bottom_repeat_mode: word("bottom_repeat_mode")?,
            bottom_fit: word("bottom_fit")?,
            bottom_blur: number("bottom_blur")?,
            water_surface: serde_json::Value::from(parameter(query, "water_surface").unwrap_or_default().trim()),
            surface_map: None,
            wave_height: number("wave_height")?,
            smoothing: number("smoothing")?,
            water_depth: number("water_depth")?,
            refractive_index: number("refractive_index")?,
            surface_color: word("surface_color")?,
            surface_opacity: number("surface_opacity")?,
            caustics_strength: number("caustics_strength")?,
            sky_layer: serde_json::Value::from(parameter(query, "sky_layer").unwrap_or_default().trim()),
            sky_map: None,
            sky_scaling: number("sky_scaling")?,
            sky_repeat_mode: word("sky_repeat_mode")?,
            sky_fit: word("sky_fit")?,
            sky_intensity: number("sky_intensity")?,
            sky_convergence: number("sky_convergence")?,
            light_type: word("light_type")?,
            light_intensity: number("light_intensity")?,
            light_color: word("light_color")?,
            light_position: pair("light_position")?.to_vec(),
            light_height: number("light_height")?,
            ambient_light: number("ambient_light")?,
            diffuse_reflection: number("diffuse_reflection")?,
            specular_reflection: number("specular_reflection")?,
            highlight_sharpness: number("highlight_sharpness")?,
        }),
        TIME_DIFFERENCE => Ok(Effect::TimeDifference {
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            time_offset: number("time_offset")?,
            contrast: number("contrast")?,
            absolute_difference: word("absolute_difference")?,
            alpha_channel: word("alpha_channel")?,
            map: None,
        }),
        POSTERIZE_TIME => Ok(Effect::PosterizeTime { frame_rate: number("frame_rate")? }),
        CHANGE_TO_COLOR => Ok(Effect::ChangeToColor {
            from: word("from")?,
            to: word("to")?,
            change: word("change")?,
            change_by: word("change_by")?,
            hue_tolerance: number("hue_tolerance")?,
            lightness_tolerance: number("lightness_tolerance")?,
            saturation_tolerance: number("saturation_tolerance")?,
            softness: number("softness")?,
            view_matte: word("view_matte")?,
        }),
        CORNER_PIN => Ok(Effect::CornerPin {
            upper_left: pair("upper_left")?,
            upper_right: pair("upper_right")?,
            lower_left: pair("lower_left")?,
            lower_right: pair("lower_right")?,
        }),
        LIGHT_SWEEP => Ok(Effect::LightSweep {
            center: pair("center")?,
            direction: number("direction")?,
            shape: word("shape")?,
            width: number("width")?,
            sweep_intensity: number("sweep_intensity")?,
            edge_intensity: number("edge_intensity")?,
            edge_thickness: number("edge_thickness")?,
            light_color: word("light_color")?,
            light_reception: word("light_reception")?,
        }),
        RADIO_WAVES => Ok(Effect::RadioWaves {
            producer_point: pair("producer_point")?,
            sides: number("sides")?,
            interval: number("interval")?,
            expansion: number("expansion")?,
            orientation: number("orientation")?,
            direction: number("direction")?,
            velocity: number("velocity")?,
            spin: number("spin")?,
            lifespan: number("lifespan")?,
            opacity: number("opacity")?,
            fade_in_time: number("fade_in_time")?,
            fade_out_time: number("fade_out_time")?,
            start_width: number("start_width")?,
            end_width: number("end_width")?,
            profile: word("profile")?,
            color: word("color")?,
            frame: 0,
        }),
        // D-320: a command without a shape keeps a file's own rule, the ellipse.
        POLAR_COORDINATES => Ok(Effect::PolarCoordinates {
            interpolation: number("interpolation")?,
            conversion: word("conversion")?,
            shape: word("shape").unwrap_or_else(|_| "ellipse".to_string()),
        }),
        MEDIAN => Ok(Effect::Median {
            radius: number("radius")?,
            operate_on_alpha: word("operate_on_alpha")?,
        }),
        DUST_SCRATCHES => Ok(Effect::DustScratches {
            radius: number("radius")?,
            threshold: number("threshold")?,
            operate_on_alpha: word("operate_on_alpha")?,
        }),
        // D-642.
        NOISE_GATE => Ok(Effect::NoiseGate {
            threshold: number("threshold")?,
            attack: number("attack")?,
            hold: number("hold")?,
            release: number("release")?,
        }),
        // D-643.
        // D-644.
        DISTORTION => Ok(Effect::Distortion {
            kind: word("type")?,
            drive: number("drive")?,
            gain: number("gain")?,
            wet: number("wet")?,
            volume: number("volume")?,
            resolution: number("resolution")?,
            downsample: number("downsample")?,
        }),
        COMPRESSOR => Ok(Effect::Compressor {
            threshold: number("threshold")?,
            ratio: number("ratio")?,
            knee: number("knee")?,
            attack: number("attack")?,
            release: number("release")?,
            auto_release: word("auto_release")?,
            makeup: number("makeup")?,
            output_limit: number("output_limit")?,
        }),
        // D-447: the frame is set by compose each frame.
        BRUSH_STROKES => Ok(Effect::BrushStrokes {
            stroke_angle: number("stroke_angle")?,
            brush_size: number("brush_size")?,
            stroke_length: number("stroke_length")?,
            stroke_density: number("stroke_density")?,
            stroke_randomness: number("stroke_randomness")?,
            paint_surface: word("paint_surface")?,
            blend_with_original: number("blend_with_original")?,
            random_seed: number("random_seed")?,
            animate: word("animate")?,
            frame: 0,
        }),
        BLOCK_LOAD => Ok(Effect::BlockLoad {
            completion: number("completion")?,
            scans: number("scans")?,
            start_cleared: word("start_cleared")?,
            bilinear: word("bilinear")?,
            pixel: 1.0,
        }),
        BURN_FILM => Ok(Effect::BurnFilm { burn: number("burn")?, center: pair("center")?, seed: number("seed")? }),
        HEX_TILE => Ok(Effect::HexTile {
            render: word("render")?,
            radius: number("radius")?,
            center: pair("center")?,
            lock_center: word("lock_center")?,
            rotate: number("rotate")?,
            smearing: number("smearing")?,
        }),
        // D-462: as CC Vector Blur's, the layer travels as `map_layer`.
        MR_SMOOTHIE => Ok(Effect::MrSmoothie {
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            fit: word("fit")?,
            property: word("property")?,
            smoothness: number("smoothness")?,
            sample_a: pair("sample_a")?,
            sample_b: pair("sample_b")?,
            phase: number("phase")?,
            color_loop: word("color_loop")?,
            map: None,
        }),
        // D-463: as Blobbylize's, the layer travels as `map_layer`.
        PLASTIC => Ok(Effect::Plastic {
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            fit: word("fit")?,
            property: word("property")?,
            softness: number("softness")?,
            height: number("height")?,
            cut_min: number("cut_min")?,
            cut_max: number("cut_max")?,
            light_intensity: number("light_intensity")?,
            light_color: word("light_color")?,
            light_type: word("light_type")?,
            light_height: number("light_height")?,
            light_position: pair("light_position")?,
            light_direction: number("light_direction")?,
            ambient_light_color: word("ambient_light_color")?,
            ambient: number("ambient")?,
            diffuse: number("diffuse")?,
            dust: number("dust")?,
            specular: number("specular")?,
            roughness: number("roughness")?,
            metal: number("metal")?,
            map: None,
        }),
        REPEAT_TILE => Ok(Effect::RepeatTile {
            expand_right: number("expand_right")?,
            expand_left: number("expand_left")?,
            expand_down: number("expand_down")?,
            expand_up: number("expand_up")?,
            tiling: word("tiling")?,
            blend_borders: number("blend_borders")?,
        }),
        SCATTER => Ok(Effect::Scatter {
            scatter_amount: number("scatter_amount")?,
            grain: word("grain")?,
            randomize: word("randomize")?,
            frame: 0,
        }),
        STROBE_LIGHT => Ok(Effect::StrobeLight {
            strobe_color: word("strobe_color")?,
            blend_with_original: number("blend_with_original")?,
            strobe_duration: number("strobe_duration")?,
            strobe_period: number("strobe_period")?,
            random_strobe_probability: number("random_strobe_probability")?,
            strobe: word("strobe")?,
            strobe_operator: word("strobe_operator")?,
            random_seed: number("random_seed")?,
            time: 0.0,
            count: 0,
        }),
        CHANNEL_THRESHOLD => Ok(Effect::ChannelThreshold {
            threshold: number("threshold")?,
            channel: word("channel")?,
            invert: word("invert")?,
            blend_with_original: number("blend_with_original")?,
        }),
        THRESHOLD_RGB => Ok(Effect::ThresholdRgb {
            red_threshold: number("red_threshold")?,
            green_threshold: number("green_threshold")?,
            blue_threshold: number("blue_threshold")?,
            invert_red: word("invert_red")?,
            invert_green: word("invert_green")?,
            invert_blue: word("invert_blue")?,
            blend_with_original: number("blend_with_original")?,
        }),
        // D-470: as Texturize's, the edge map travels as `map_layer`.
        CONTOUR_PAINT => Ok(Effect::ContourPaint {
            view: word("view")?,
            stylization: number("stylization")?,
            cleanliness: number("cleanliness")?,
            smear_iterations: number("smear_iterations")?,
            apply_to_alpha: word("apply_to_alpha")?,
            draw_color_fills: word("draw_color_fills")?,
            reduction_method: word("reduction_method")?,
            posterization_level: number("posterization_level")?,
            low_strength: number("low_strength")?,
            high_strength: number("high_strength")?,
            offset_flow_angles: number("offset_flow_angles")?,
            quantize_flow_angles: number("quantize_flow_angles")?,
            edge_map_layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            // D-584: the custom colours as D-87's chosen ones; none when the command does not say.
            colors: if parameter(query, "colors").is_some() { colors()? } else { Vec::new() },
            // D-585: off when the command does not say, as a file without it.
            arrowheads: if parameter(query, "arrowheads").is_some() { word("arrowheads")? } else { "off".to_string() },
            px: 1.0,
            palette: None,
            map: None,
        }),
        // D-467: as Plastic's, the layer travels as `map_layer`.
        TEXTURIZE => Ok(Effect::Texturize {
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            fit: word("fit")?,
            light_direction: number("light_direction")?,
            texture_contrast: number("texture_contrast")?,
            map: None,
        }),
        CARTOON => Ok(Effect::Cartoon {
            render: word("render")?,
            detail_radius: number("detail_radius")?,
            detail_threshold: number("detail_threshold")?,
            shading_steps: number("shading_steps")?,
            shading_smoothness: number("shading_smoothness")?,
            edge_threshold: number("edge_threshold")?,
            edge_width: number("edge_width")?,
            edge_softness: number("edge_softness")?,
            edge_opacity: number("edge_opacity")?,
            edge_enhancement: number("edge_enhancement")?,
            edge_black_level: number("edge_black_level")?,
            edge_contrast: number("edge_contrast")?,
        }),
        SMART_BLUR => Ok(Effect::SmartBlur {
            radius: number("radius")?,
            threshold: number("threshold")?,
        }),
        BILATERAL_BLUR => Ok(Effect::BilateralBlur {
            radius: number("radius")?,
            threshold: number("threshold")?,
            colorize: word("colorize")?,
        }),
        CROSS_BLUR => Ok(Effect::CrossBlur {
            radius_x: number("radius_x")?,
            radius_y: number("radius_y")?,
            mode: word("mode")?,
            edges: edges(),
        }),
        SPIN_ZOOM_BLUR => Ok(Effect::SpinZoomBlur {
            kind: word("type")?,
            amount: number("amount")?,
            quality: number("quality")?,
            center: pair("center")?,
        }),
        FAST_ZOOM_BLUR => Ok(Effect::FastZoomBlur {
            amount: number("amount")?,
            center: pair("center")?,
            zoom: word("zoom")?,
        }),
        BROADCAST_SAFE => Ok(Effect::BroadcastSafe {
            locale: word("locale")?,
            method: word("method")?,
            max_amplitude: number("max_amplitude")?,
        }),
        COLOR_NEUTRALIZER => Ok(Effect::ColorNeutralizer {
            shadows_unbalance: word("shadows_unbalance")?,
            midtones_unbalance: word("midtones_unbalance")?,
            highlights_unbalance: word("highlights_unbalance")?,
            shadows: tone("shadows")?,
            midtones: tone("midtones")?,
            highlights: tone("highlights")?,
            pinning: number("pinning")?,
            black_point: number("black_point")?,
            white_point: number("white_point")?,
        }),
        COLOR_OFFSET => Ok(Effect::ColorOffset {
            red_phase: number("red_phase")?,
            green_phase: number("green_phase")?,
            blue_phase: number("blue_phase")?,
            overflow: word("overflow")?,
        }),
        KERNEL => Ok(Effect::Kernel {
            line_1: tone("line_1")?,
            line_2: tone("line_2")?,
            line_3: tone("line_3")?,
            divider: number("divider")?,
            absolute_values: word("absolute_values")?,
        }),
        TONER => Ok(Effect::Toner {
            tones: word("tones")?,
            highlights: word("highlights")?,
            brights: word("brights")?,
            midtones: word("midtones")?,
            darktones: word("darktones")?,
            shadows: word("shadows")?,
        }),
        CHANGE_COLOR => Ok(Effect::ChangeColor {
            view: word("view")?,
            hue_transform: number("hue_transform")?,
            lightness_transform: number("lightness_transform")?,
            saturation_transform: number("saturation_transform")?,
            color_to_change: word("color_to_change")?,
            tolerance: number("tolerance")?,
            softness: number("softness")?,
            match_colors: word("match_colors")?,
            invert_mask: word("invert_mask")?,
        }),
        // D-374.
        COLOR_BALANCE_HLS => Ok(Effect::ColorBalanceHls {
            hue: number("hue")?,
            lightness: number("lightness")?,
            saturation: number("saturation")?,
        }),
        // D-382.
        GAMMA_PEDESTAL_GAIN => Ok(Effect::GammaPedestalGain {
            black_stretch: number("black_stretch")?,
            gamma: [number("red_gamma")?, number("green_gamma")?, number("blue_gamma")?],
            pedestal: [number("red_pedestal")?, number("green_pedestal")?, number("blue_pedestal")?],
            gain: [number("red_gain")?, number("green_gain")?, number("blue_gain")?],
        }),
        // D-397: the look, an asset's id as written, and the numbers. D-398: the curves as
        // Curves' points, the hue curve likewise (empty for none) and the two words; any later
        // setting the command does not name at its start. D-571: the other curves as the hue
        // curve, and the wheels' word.
        COLOR_GRADE => {
            let mut values = [0.0; 55];
            for (i, ((name, _, _, start), v)) in COLOR_GRADE_SETTINGS.iter().zip(values.iter_mut()).enumerate() {
                *v = if i < 22 || parameter(query, name).is_some() { number(name)? } else { *start };
            }
            let curve = |name: &str| if parameter(query, name).is_some() { points(name) } else { Ok(vec![vec![0.0, 0.0], vec![255.0, 255.0]]) };
            let hues = parameter(query, "hue_saturation").unwrap_or_default();
            let word_or = |name: &str| if parameter(query, name).is_some() { word(name) } else { Ok("off".to_string()) };
            let list = |name: &str| if parameter(query, name).unwrap_or_default().trim().is_empty() { Ok(Vec::new()) } else { points(name) };
            Ok(Effect::ColorGrade {
                values,
                look: parameter(query, "look").unwrap_or_default().trim().to_string(),
                curves: [curve("curve_master")?, curve("curve_red")?, curve("curve_green")?, curve("curve_blue")?],
                hue_saturation: if hues.trim().is_empty() { Vec::new() } else { points("hue_saturation")? },
                more_curves: [list("hue_hue")?, list("hue_luma")?, list("luma_saturation")?, list("saturation_saturation")?],
                secondary_wheels: if parameter(query, "secondary_wheels").is_some() { word("secondary_wheels")? } else { "one".to_string() },
                key_invert: word_or("key_invert")?,
                key_view: word_or("key_view")?,
                table: None,
            })
        }
        // D-384.
        PHOTO_FILTER => Ok(Effect::PhotoFilter {
            filter: word("filter")?,
            color: word("color")?,
            density: number("density")?,
            preserve_luminosity: word("preserve_luminosity")?,
        }),
        // D-375: the layer travels as `map_layer`; none named is "", the layer itself.
        COLOR_LINK => Ok(Effect::ColorLink {
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            sample: word("sample")?,
            clip: number("clip")?,
            stencil: word("stencil")?,
            opacity: number("opacity")?,
            blending_mode: word("blending_mode")?,
            map: None,
        }),
        // D-376.
        COLOR_STABILIZER => Ok(Effect::ColorStabilizer {
            stabilize: word("stabilize")?,
            reference_frame: number("reference_frame")?,
            black_point: pair("black_point")?,
            mid_point: pair("mid_point")?,
            white_point: pair("white_point")?,
            sample_size: number("sample_size")?,
            reference: None,
        }),
        SNOWFALL => Ok(Effect::Snowfall {
            color: word("color")?,
            density: number("density")?,
            spacing: number("spacing")?,
            size: number("size")?,
            depth: number("depth")?,
            speed: number("speed")?,
            wind: number("wind")?,
            wiggle: number("wiggle")?,
            period: number("period")?,
            seed: number("seed")?,
            opacity: number("opacity")?,
            frame: 0,
        }),
        KALEIDOSCOPE => Ok(Effect::Kaleidoscope {
            segments: number("segments")?,
            rotation: number("rotation")?,
            size: number("size")?,
            center: pair("center")?,
            mode: word("mode")?,
        }),
        ROUGHEN_EDGES => Ok(Effect::RoughenEdges {
            edge_type: word("edge_type")?,
            edge_color: word("edge_color")?,
            border: number("border")?,
            size: number("size")?,
            complexity: number("complexity")?,
            evolution: number("evolution")?,
            speed: number("speed")?,
            seed: number("seed")?,
            frame: 0,
        }),
        BEAM => Ok(Effect::Beam {
            start: pair("start")?,
            end: pair("end")?,
            length: number("length")?,
            time: number("time")?,
            start_thickness: number("start_thickness")?,
            end_thickness: number("end_thickness")?,
            softness: number("softness")?,
            inside_color: word("inside_color")?,
            outside_color: word("outside_color")?,
            composite: word("composite")?,
        }),
        FOUR_COLOR_GRADIENT => Ok(Effect::FourColorGradient {
            point_1: pair("point_1")?,
            point_2: pair("point_2")?,
            point_3: pair("point_3")?,
            point_4: pair("point_4")?,
            color_1: word("color_1")?,
            color_2: word("color_2")?,
            color_3: word("color_3")?,
            color_4: word("color_4")?,
            blend: number("blend")?,
            opacity: number("opacity")?,
            blending_mode: word("blending_mode")?,
        }),
        CELL_PATTERN => Ok(Effect::CellPattern {
            pattern: word("pattern")?,
            invert: word("invert")?,
            contrast: number("contrast")?,
            disperse: number("disperse")?,
            size: number("size")?,
            evolution: number("evolution")?,
            seed: number("seed")?,
            dark_color: word("dark_color")?,
            light_color: word("light_color")?,
            opacity: number("opacity")?,
            blend: word("blend")?,
        }),
        OPTICS_COMPENSATION => Ok(Effect::OpticsCompensation {
            field_of_view: number("field_of_view")?,
            reverse: word("reverse")?,
            orientation: word("orientation")?,
            center: pair("center")?,
        }),
        RADIAL_SHADOW => Ok(Effect::RadialShadow {
            color: word("color")?,
            opacity: number("opacity")?,
            light: pair("light")?,
            distance: number("distance")?,
            softness: number("softness")?,
            render: word("render")?,
            color_influence: number("color_influence")?,
            shadow_only: word("shadow_only")?,
        }),
        EXTRACT => Ok(Effect::Extract {
            channel: word("channel")?,
            black_point: number("black_point")?,
            white_point: number("white_point")?,
            black_softness: number("black_softness")?,
            white_softness: number("white_softness")?,
            invert: word("invert")?,
        }),
        BEVEL_ALPHA => Ok(Effect::BevelAlpha {
            edge_thickness: number("edge_thickness")?,
            light_angle: number("light_angle")?,
            light_color: word("light_color")?,
            light_intensity: number("light_intensity")?,
        }),
        BEVEL_EDGES => Ok(Effect::BevelEdges {
            edge_thickness: number("edge_thickness")?,
            light_angle: number("light_angle")?,
            light_color: word("light_color")?,
            light_intensity: number("light_intensity")?,
        }),
        BLOCK_DISSOLVE => Ok(Effect::BlockDissolve {
            completion: number("completion")?,
            block_width: number("block_width")?,
            block_height: number("block_height")?,
            feather: number("feather")?,
        }),
        SHIFT_CHANNELS => Ok(Effect::ShiftChannels {
            take_alpha: word("take_alpha")?,
            take_red: word("take_red")?,
            take_green: word("take_green")?,
            take_blue: word("take_blue")?,
        }),
        SOLID_COMPOSITE => Ok(Effect::SolidComposite {
            source_opacity: number("source_opacity")?,
            color: word("color")?,
            opacity: number("opacity")?,
            blend: word("blend")?,
        }),
        CHANNEL_BLUR => Ok(Effect::ChannelBlur {
            red_blurriness: number("red_blurriness")?,
            green_blurriness: number("green_blurriness")?,
            blue_blurriness: number("blue_blurriness")?,
            alpha_blurriness: number("alpha_blurriness")?,
            edges: edges(),
            dimensions: word("dimensions").unwrap_or_else(|_| "both".to_string()),
            // D-380: sigma when the command does not say, as a file without it.
            units: word("units").unwrap_or_else(|_| "sigma".to_string()),
        }),
        FAST_BOX_BLUR => Ok(Effect::FastBoxBlur {
            radius: number("radius")?,
            iterations: number("iterations")?,
            edges: edges(),
            dimensions: word("dimensions").unwrap_or_else(|_| "both".to_string()),
        }),
        FAST_BLUR_LEGACY => Ok(Effect::FastBlurLegacy {
            blurriness: number("blurriness")?,
            edges: edges(),
            dimensions: word("dimensions").unwrap_or_else(|_| "both".to_string()),
        }),
        GAUSSIAN_BLUR_LEGACY => Ok(Effect::GaussianBlurLegacy {
            blurriness: number("blurriness")?,
            dimensions: word("dimensions").unwrap_or_else(|_| "both".to_string()),
        }),
        // D-189: as Compound Blur's, the layer travels as `map_layer`; none named is "".
        // D-381: the rest at their start when the command does not say, the layer read as the
        // pixel is; the mask layer travels as `mask_layer`.
        COLORAMA => Ok(Effect::Colorama {
            add_phase_from: word("add_phase_from").or_else(|_| word("get_phase"))?,
            add_mode: word("add_mode").unwrap_or_else(|_| "wrap".to_string()),
            interpolate: word("interpolate").unwrap_or_else(|_| "on".to_string()),
            opacity_1: if parameter(query, "opacity_1").is_some() { number("opacity_1")? } else { 100.0 },
            opacity_2: if parameter(query, "opacity_2").is_some() { number("opacity_2")? } else { 100.0 },
            opacity_3: if parameter(query, "opacity_3").is_some() { number("opacity_3")? } else { 100.0 },
            opacity_4: if parameter(query, "opacity_4").is_some() { number("opacity_4")? } else { 100.0 },
            opacity_5: if parameter(query, "opacity_5").is_some() { number("opacity_5")? } else { 100.0 },
            modify: word("modify").unwrap_or_else(|_| "all".to_string()),
            modify_alpha: word("modify_alpha").unwrap_or_else(|_| "off".to_string()),
            change_empty: word("change_empty").unwrap_or_else(|_| "off".to_string()),
            matching_mode: word("matching_mode").unwrap_or_else(|_| "off".to_string()),
            matching_color: word("matching_color").unwrap_or_else(|_| "#ffffff".to_string()),
            matching_tolerance: if parameter(query, "matching_tolerance").is_some() { number("matching_tolerance")? } else { 15.0 },
            matching_softness: if parameter(query, "matching_softness").is_some() { number("matching_softness")? } else { 0.0 },
            mask_layer: serde_json::Value::from(parameter(query, "mask_layer").unwrap_or_default().trim()),
            masking_mode: word("masking_mode").unwrap_or_else(|_| "luminance".to_string()),
            composite_over: word("composite_over").unwrap_or_else(|_| "on".to_string()),
            mask_map: None,
            get_phase: word("get_phase")?,
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            fit: word("fit")?,
            phase_shift: number("phase_shift")?,
            cycle_repetitions: number("cycle_repetitions")?,
            stops: number("stops")?,
            color_1: word("color_1")?,
            color_2: word("color_2")?,
            color_3: word("color_3")?,
            color_4: word("color_4")?,
            color_5: word("color_5")?,
            blend_with_original: number("blend_with_original")?,
            map: None,
        }),
        BEND_IT => Ok(Effect::BendIt {
            bend: number("bend")?,
            start: pair("start")?,
            end: pair("end")?,
            render_prestart: word("render_prestart")?,
            distort: word("distort")?,
        }),
        BENDER => Ok(Effect::Bender {
            amount: number("amount")?,
            style: word("style")?,
            adjust_to_distance: word("adjust_to_distance")?,
            top: pair("top")?,
            base: pair("base")?,
        }),
        FLOW_MOTION => Ok(Effect::FlowMotion {
            knot_1: pair("knot_1")?,
            amount_1: number("amount_1")?,
            knot_2: pair("knot_2")?,
            amount_2: number("amount_2")?,
            falloff: number("falloff")?,
            tile_edges: word("tile_edges")?,
            finer_controls: word("finer_controls")?,
            antialiasing: word("antialiasing")?,
        }),
        GRIDDLER => Ok(Effect::Griddler {
            horizontal_scale: number("horizontal_scale")?,
            vertical_scale: number("vertical_scale")?,
            tile_size: number("tile_size")?,
            rotation: number("rotation")?,
            cut_tiles: word("cut_tiles")?,
        }),
        FISHEYE => Ok(Effect::Fisheye { center: pair("center")?, size: number("size")?, convergence: number("convergence")? }),
        // D-388: the back page travels as `map_layer`, as CC Glass's layer does; none is "".
        PAGE_TURN => Ok(Effect::PageTurn {
            controls: word("controls")?,
            fold_position: pair("fold_position")?,
            fold_direction: number("fold_direction")?,
            fold_radius: number("fold_radius")?,
            light_direction: number("light_direction")?,
            render: word("render")?,
            back_page: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            back_opacity: number("back_opacity")?,
            paper_color: word("paper_color")?,
            map: None,
        }),
        // D-607.
        MASK_SMEAR => Ok(Effect::MaskSmear {
            source_mask: number("source_mask")?,
            boundary_mask: number("boundary_mask")?,
            mask_offset: pair("mask_offset")?,
            mask_rotation: number("mask_rotation")?,
            mask_scale: number("mask_scale")?,
            percent: number("percent")?,
            elasticity: word("elasticity")?,
            paths: None,
        }),
        // D-606. D-702: the correspondence points as one list, commas or spaces between, none
        // for none; pairs are the core's check, in its words; Discrete when not said.
        RESHAPE => Ok(Effect::Reshape {
            source_mask: number("source_mask")?,
            destination_mask: number("destination_mask")?,
            boundary_mask: number("boundary_mask")?,
            percent: number("percent")?,
            elasticity: word("elasticity")?,
            correspondence_points: parameter(query, "correspondence_points")
                .unwrap_or_default()
                .split(|ch: char| ch == ',' || ch.is_whitespace())
                .filter(|p| !p.is_empty())
                .map(|p| p.parse::<f64>().map_err(|_| format!("Correspondence points need numbers, source then destination for each pair. Not \"{p}\".")))
                .collect::<Result<Vec<_>, _>>()?,
            interpolation: parameter(query, "interpolation").map_or_else(|| "discrete".to_string(), |t| t.trim().to_ascii_lowercase()),
            between: Vec::new(),
            paths: None,
        }),
        // D-605: the stroke as one list, commas or spaces between, none for no stroke. Whole
        // points are the core's check, in its words.
        LIQUIFY => {
            let text = parameter(query, "stroke").unwrap_or_default();
            let stroke = text
                .split(|ch: char| ch == ',' || ch.is_whitespace())
                .filter(|p| !p.is_empty())
                .map(|p| p.parse::<f64>().map_err(|_| format!("Stroke needs numbers, x then y for each point. Not \"{p}\".")))
                .collect::<Result<Vec<_>, _>>()?;
            // D-700: the new settings as added when the command leaves them out; the earlier
            // strokes as the page sends them, a comma between strokes, none for none.
            let or = |name: &str, start: f64| if parameter(query, name).is_some() { number(name) } else { Ok(start) };
            let strokes = match parameter(query, "strokes") {
                Some(t) if !t.trim().is_empty() => points("strokes")?,
                _ => Vec::new(),
            };
            Ok(Effect::Liquify {
                tool: word("tool")?,
                brush_size: number("brush_size")?,
                brush_pressure: number("brush_pressure")?,
                stroke,
                distortion_percentage: number("distortion_percentage")?,
                distortion_mesh_offset: pair("distortion_mesh_offset")?,
                turbulent_jitter: or("turbulent_jitter", 70.0)?,
                clone_offset: if parameter(query, "clone_offset").is_some() { pair("clone_offset")? } else { [0.0, 0.0] },
                reconstruction_mode: if parameter(query, "reconstruction_mode").is_some() { word("reconstruction_mode")? } else { "revert".to_string() },
                freeze_area_mask: or("freeze_area_mask", 0.0)?,
                strokes,
                freeze_path: None,
            })
        }
        // D-604.
        WARP => Ok(Effect::Warp {
            warp_style: word("warp_style")?,
            warp_axis: word("warp_axis")?,
            bend: number("bend")?,
            horizontal_distortion: number("horizontal_distortion")?,
            vertical_distortion: number("vertical_distortion")?,
        }),
        // D-603: the vertices as one list, commas or spaces between, none for the mesh at rest.
        // How many there should be is the core's check, in its words. D-701: the handles the same.
        MESH_WARP => {
            let list = |name: &str, says: &str| {
                parameter(query, name)
                    .unwrap_or_default()
                    .split(|ch: char| ch == ',' || ch.is_whitespace())
                    .filter(|p| !p.is_empty())
                    .map(|p| p.parse::<f64>().map_err(|_| format!("{says} Not \"{p}\".")))
                    .collect::<Result<Vec<_>, _>>()
            };
            let distortion_mesh = list("distortion_mesh", "Distortion mesh needs numbers, x then y for each vertex.")?;
            let vertex_handles = list("vertex_handles", "Vertex handles need numbers, eight for each vertex.")?;
            Ok(Effect::MeshWarp { rows: number("rows")?, columns: number("columns")?, quality: number("quality")?, distortion_mesh, vertex_handles })
        }
        // D-602.
        BEZIER_WARP => {
            let mut points = anime_compositor::effects::BEZIER_START;
            for (p, name) in points.iter_mut().zip(anime_compositor::effects::BEZIER_POINTS) {
                *p = pair(name)?;
            }
            Ok(Effect::BezierWarp { points, quality: number("quality")? })
        }
        POWER_PIN => Ok(Effect::PowerPin {
            top_left: pair("top_left")?,
            top_right: pair("top_right")?,
            bottom_left: pair("bottom_left")?,
            bottom_right: pair("bottom_right")?,
            perspective: number("perspective")?,
            unstretch: word("unstretch")?,
            expansion_top: number("expansion_top")?,
            expansion_left: number("expansion_left")?,
            expansion_right: number("expansion_right")?,
            expansion_bottom: number("expansion_bottom")?,
        }),
        // D-391..D-393.
        SLANT => Ok(Effect::Slant {
            slant: number("slant")?,
            stretching: word("stretching")?,
            height: number("height")?,
            floor: pair("floor")?,
            set_color: word("set_color")?,
            color: word("color")?,
        }),
        SMEAR => Ok(Effect::Smear { from: pair("from")?, to: pair("to")?, reach: number("reach")?, radius: number("radius")? }),
        SPLIT => Ok(Effect::Split { point_a: pair("point_a")?, point_b: pair("point_b")?, split: number("split")? }),
        SPLIT_2 => Ok(Effect::Split2 { point_a: pair("point_a")?, point_b: pair("point_b")?, split_1: number("split_1")?, split_2: number("split_2")? }),
        TILES => Ok(Effect::Tiles { scale: number("scale")?, center: pair("center")?, blend: number("blend")? }),
        MAGNIFY => Ok(Effect::Magnify {
            shape: word("shape")?,
            center: pair("center")?,
            magnification: number("magnification")?,
            link: word("link")?,
            size: number("size")?,
            feather: number("feather")?,
            opacity: number("opacity")?,
            scaling: word("scaling")?,
            blending_mode: word("blending_mode")?,
            resize_layer: word("resize_layer")?,
        }),
        SPHERIZE => Ok(Effect::Spherize { radius: number("radius")?, center: pair("center")? }),
        CHECKERBOARD => Ok(Effect::Checkerboard {
            anchor: pair("anchor")?,
            size_from: word("size_from")?,
            corner: pair("corner")?,
            width: number("width")?,
            height: number("height")?,
            feather_width: number("feather_width")?,
            feather_height: number("feather_height")?,
            color: word("color")?,
            opacity: number("opacity")?,
            blending_mode: word("blending_mode")?,
        }),
        CIRCLE => Ok(Effect::Circle {
            center: pair("center")?,
            radius: number("radius")?,
            edge: word("edge")?,
            edge_thickness: number("edge_thickness")?,
            feather_outer: number("feather_outer")?,
            feather_inner: number("feather_inner")?,
            invert: word("invert")?,
            color: word("color")?,
            opacity: number("opacity")?,
            blending_mode: word("blending_mode")?,
        }),
        ELLIPSE => Ok(Effect::Ellipse {
            center: pair("center")?,
            width: number("width")?,
            height: number("height")?,
            thickness: number("thickness")?,
            softness: number("softness")?,
            inside_color: word("inside_color")?,
            outside_color: word("outside_color")?,
            composite: word("composite")?,
        }),
        AUDIO_SPECTRUM => Ok(Effect::AudioSpectrum {
            audio_layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            start_point: pair("start_point")?,
            end_point: pair("end_point")?,
            path: number("path")?,
            use_polar_path: word("use_polar_path")?,
            start_frequency: number("start_frequency")?,
            end_frequency: number("end_frequency")?,
            frequency_bands: number("frequency_bands")?,
            maximum_height: number("maximum_height")?,
            audio_duration: number("audio_duration")?,
            audio_offset: number("audio_offset")?,
            thickness: number("thickness")?,
            softness: number("softness")?,
            inside_color: word("inside_color")?,
            outside_color: word("outside_color")?,
            blend_overlapping_colors: word("blend_overlapping_colors")?,
            hue_interpolation: number("hue_interpolation")?,
            dynamic_hue_phase: word("dynamic_hue_phase")?,
            color_symmetry: word("color_symmetry")?,
            display_options: word("display_options")?,
            side_options: word("side_options")?,
            duration_averaging: word("duration_averaging")?,
            composite: word("composite")?,
            levels: None,
            outline: None,
        }),
        AUDIO_WAVEFORM => Ok(Effect::AudioWaveform {
            audio_layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            start_point: pair("start_point")?,
            end_point: pair("end_point")?,
            path: number("path")?,
            displayed_samples: number("displayed_samples")?,
            maximum_height: number("maximum_height")?,
            audio_duration: number("audio_duration")?,
            audio_offset: number("audio_offset")?,
            thickness: number("thickness")?,
            softness: number("softness")?,
            random_seed: number("random_seed")?,
            inside_color: word("inside_color")?,
            outside_color: word("outside_color")?,
            waveform_options: word("waveform_options")?,
            display_options: word("display_options")?,
            composite: word("composite")?,
            levels: None,
            outline: None,
        }),
        LIGHT_BURST => Ok(Effect::LightBurst {
            center: pair("center")?,
            intensity: number("intensity")?,
            ray_length: number("ray_length")?,
            burst: word("burst")?,
            set_color: word("set_color")?,
            color: word("color")?,
        }),
        DETAIL_UPSCALE => Ok(Effect::DetailUpscale {
            scale: number("scale")?,
            reduce_noise: number("reduce_noise")?,
            detail: number("detail")?,
        }),
        FRACTAL => Ok(Effect::Fractal {
            set_choice: word("set_choice")?,
            equation: word("equation")?,
            mandelbrot_center: pair("mandelbrot_center")?,
            mandelbrot_magnification: number("mandelbrot_magnification")?,
            mandelbrot_escape_limit: number("mandelbrot_escape_limit")?,
            julia_center: pair("julia_center")?,
            julia_magnification: number("julia_magnification")?,
            julia_escape_limit: number("julia_escape_limit")?,
            overlay: word("overlay")?,
            transparency: word("transparency")?,
            palette: word("palette")?,
            hue: number("hue")?,
            cycle_steps: number("cycle_steps")?,
            cycle_offset: number("cycle_offset")?,
            edge_highlight: word("edge_highlight")?,
            oversample_method: word("oversample_method")?,
            oversample_factor: number("oversample_factor")?,
        }),
        // D-418: the paths are found by compose each frame.
        EYEDROPPER_FILL => Ok(Effect::EyedropperFill {
            sample_point: pair("sample_point")?,
            sample_radius: number("sample_radius")?,
            average_pixel_colors: word("average_pixel_colors")?,
            maintain_original_alpha: word("maintain_original_alpha")?,
            blend_with_original: number("blend_with_original")?,
        }),
        PAINT_BUCKET => Ok(Effect::PaintBucket {
            fill_point: pair("fill_point")?,
            fill_selector: word("fill_selector")?,
            tolerance: number("tolerance")?,
            view_threshold: word("view_threshold")?,
            stroke: word("stroke")?,
            invert_fill: word("invert_fill")?,
            spread_radius: number("spread_radius")?,
            stroke_width: number("stroke_width")?,
            feather_softness: number("feather_softness")?,
            color: word("color")?,
            opacity: number("opacity")?,
            blending_mode: word("blending_mode")?,
        }),
        NOISE_ALPHA => Ok(Effect::NoiseAlpha {
            noise: word("noise")?,
            amount: number("amount")?,
            original_alpha: word("original_alpha")?,
            overflow: word("overflow")?,
            random_seed: number("random_seed")?,
            noise_phase: number("noise_phase")?,
            cycle_noise: word("cycle_noise")?,
            cycle: number("cycle")?,
        }),
        NOISE_HLS => Ok(Effect::NoiseHls {
            noise: word("noise")?,
            hue: number("hue")?,
            lightness: number("lightness")?,
            saturation: number("saturation")?,
            grain_size: number("grain_size")?,
            noise_phase: number("noise_phase")?,
        }),
        NOISE_HLS_AUTO => Ok(Effect::NoiseHlsAuto {
            noise: word("noise")?,
            hue: number("hue")?,
            lightness: number("lightness")?,
            saturation: number("saturation")?,
            grain_size: number("grain_size")?,
            animation_speed: number("animation_speed")?,
            frame: 0,
        }),
        ADD_GRAIN => Ok(Effect::AddGrain {
            intensity: number("intensity")?,
            size: number("size")?,
            softness: number("softness")?,
            aspect_ratio: number("aspect_ratio")?,
            red_intensity: number("red_intensity")?,
            green_intensity: number("green_intensity")?,
            blue_intensity: number("blue_intensity")?,
            monochromatic: word("monochromatic")?,
            saturation: number("saturation")?,
            blending_mode: word("blending_mode")?,
            shadows: number("shadows")?,
            midtones: number("midtones")?,
            highlights: number("highlights")?,
            midpoint: number("midpoint")?,
            animation_speed: number("animation_speed")?,
            animate_smoothly: word("animate_smoothly")?,
            random_seed: number("random_seed")?,
            frame: 0,
        }),
        // D-454: the noise source travels as `map_layer`, the rest as Add Grain's.
        MATCH_GRAIN => Ok(Effect::MatchGrain {
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            grain: Box::new(effect_parameters(ADD_GRAIN, query)?),
            map: None,
        }),
        // D-455: the noise level is measured by compose each frame.
        REMOVE_GRAIN => Ok(Effect::RemoveGrain {
            noise_reduction: number("noise_reduction")?,
            passes: number("passes")?,
            mode: word("mode")?,
            unsharp_amount: number("unsharp_amount")?,
            unsharp_radius: number("unsharp_radius")?,
            unsharp_threshold: number("unsharp_threshold")?,
            noise: None,
        }),
        // D-456: the frame is set by compose each frame.
        DEBAND => Ok(Effect::Deband {
            view: word("view")?,
            edge_threshold: number("edge_threshold")?,
            edge_softness: number("edge_softness")?,
            blur_amount: number("blur_amount")?,
            edge_blur_reduction: number("edge_blur_reduction")?,
            diffusion_amount: number("diffusion_amount")?,
            edge_diffusion_reduction: number("edge_diffusion_reduction")?,
            diffusion_reseed_interval: number("diffusion_reseed_interval")?,
            noise_amount: number("noise_amount")?,
            edge_noise_reduction: number("edge_noise_reduction")?,
            noise_reseed_interval: number("noise_reseed_interval")?,
            frame: 0,
        }),
        ADVANCED_SPILL_SUPPRESSOR => Ok(Effect::AdvancedSpillSuppressor {
            method: word("method")?,
            suppression: number("suppression")?,
            key_color: word("key_color")?,
            tolerance: number("tolerance")?,
            desaturate: number("desaturate")?,
            spill_range: number("spill_range")?,
            spill_color_correction: number("spill_color_correction")?,
            luma_correction: number("luma_correction")?,
        }),
        COLOR_DIFFERENCE_KEY => {
            let mut v = [0.0; 13];
            for (v, name) in v.iter_mut().zip(CDK_NAMES) {
                *v = number(name)?;
            }
            Ok(Effect::ColorDifferenceKey {
                view: word("view")?,
                key_color: word("key_color")?,
                partial_a: [v[0], v[1], v[2], v[3], v[4]],
                partial_b: [v[5], v[6], v[7], v[8], v[9]],
                matte: [v[10], v[11], v[12]],
            })
        }
        COLOR_RANGE => {
            let mut range = [0.0; 6];
            for (v, name) in range.iter_mut().zip(CRANGE_NAMES) {
                *v = number(name)?;
            }
            Ok(Effect::ColorRange { color_space: word("color_space")?, fuzziness: number("fuzziness")?, range })
        }
        DIFFERENCE_MATTE => Ok(Effect::DifferenceMatte {
            view: word("view")?,
            layer: serde_json::Value::from(
                parameter(query, "map_layer")
                    .ok_or_else(|| "What should map_layer be set to?".to_string())?
                    .trim(),
            ),
            fit: word("fit")?,
            matching_tolerance: number("matching_tolerance")?,
            matching_softness: number("matching_softness")?,
            blur_before_difference: number("blur_before_difference")?,
            map: None,
        }),
        // D-502: the masks are found by compose each frame.
        INNER_OUTER_KEY => Ok(Effect::InnerOuterKey {
            foreground: number("foreground")?,
            background: number("background")?,
            single_mask_highlight_radius: number("single_mask_highlight_radius")?,
            edge_thin: number("edge_thin")?,
            edge_feather: number("edge_feather")?,
            edge_threshold: number("edge_threshold")?,
            invert_extraction: word("invert_extraction")?,
            blend_with_original: number("blend_with_original")?,
            paths: None,
        }),
        KEY_CLEANER => Ok(Effect::KeyCleaner {
            additional_edge_radius: number("additional_edge_radius")?,
            alpha_contrast: number("alpha_contrast")?,
            strength: number("strength")?,
        }),
        LINEAR_COLOR_KEY => Ok(Effect::LinearColorKey {
            view: word("view")?,
            key_color: word("key_color")?,
            match_colors: word("match_colors")?,
            // D-541.
            key_operation: if parameter(query, "key_operation").is_some() { word("key_operation")? } else { "key".to_string() },
            matching_tolerance: number("matching_tolerance")?,
            matching_softness: number("matching_softness")?,
        }),
        SCREEN_KEY => Ok(Effect::ScreenKey {
            view: word("view")?,
            screen_color: word("screen_color")?,
            screen_gain: number("screen_gain")?,
            screen_balance: number("screen_balance")?,
            clip_black: number("clip_black")?,
            clip_white: number("clip_white")?,
            despill_bias: number("despill_bias")?,
            despill: number("despill")?,
            shrink_grow: number("shrink_grow")?,
            softness: number("softness")?,
            edge_refine: number("edge_refine")?,
        }),
        SPILL_SUPPRESSOR => Ok(Effect::SpillSuppressor {
            color_to_suppress: word("color_to_suppress")?,
            suppression: number("suppression")?,
        }),
        REDUCE_INTERLACE_FLICKER => Ok(Effect::ReduceInterlaceFlicker { softness: number("softness")? }),
        UNMULT => Ok(Effect::Unmult {
            background_color: word("background_color")?,
            black_level: number("black_level")?,
            white_level: number("white_level")?,
            softness: number("softness")?,
            remove_color_matting: word("remove_color_matting")?,
            clip_hdr_results: word("clip_hdr_results")?,
        }),
        // D-506: the views are read by compose each frame.
        // D-508: the frame is set and the paths found by compose each frame.
        ENERGY_STROKE => Ok(Effect::EnergyStroke {
            source: word("source")?,
            core_start: pair("core_start")?,
            core_end: pair("core_end")?,
            evolution: number("evolution")?,
            start_offset: number("start_offset")?,
            end_offset: number("end_offset")?,
            start_size: number("start_size")?,
            end_size: number("end_size")?,
            core_size: number("core_size")?,
            core_softness: number("core_softness")?,
            glow_color: word("glow_color")?,
            glow_intensity: number("glow_intensity")?,
            glow_spread: number("glow_spread")?,
            flicker_amount: number("flicker_amount")?,
            flicker_speed: number("flicker_speed")?,
            distortion_amount: number("distortion_amount")?,
            distortion_scale: number("distortion_scale")?,
            distortion_speed: number("distortion_speed")?,
            random_seed: number("random_seed")?,
            composite: word("composite")?,
            start_roundness: number("start_roundness")?,
            end_roundness: number("end_roundness")?,
            noise_along_path: word("noise_along_path")?,
            alpha_mode: word("alpha_mode")?,
            invert_masks: word("invert_masks")?,
            frame: 0,
            paths: None,
            matte: None,
        }),
        CC_ENVIRONMENT => Ok(Effect::CcEnvironment {
            environment: serde_json::Value::from(
                parameter(query, "map_layer")
                    .ok_or_else(|| "What should map_layer be set to?".to_string())?
                    .trim(),
            ),
            mapping: word("mapping")?,
            horizontal_pan: number("horizontal_pan")?,
            tilt: number("tilt")?,
            roll: number("roll")?,
            field_of_view: number("field_of_view")?,
            filter_environment: word("filter_environment")?,
            map: None,
        }),
        CARD_WIPE => Ok(Effect::CardWipe {
            transition_completion: number("transition_completion")?,
            transition_width: number("transition_width")?,
            back_layer: serde_json::Value::from(
                parameter(query, "back_layer").ok_or_else(|| "What should back_layer be set to?".to_string())?.trim(),
            ),
            back_map: None,
            rows_columns: word("rows_columns")?,
            rows: number("rows")?,
            columns: number("columns")?,
            card_scale: number("card_scale")?,
            flip_axis: word("flip_axis")?,
            flip_direction: word("flip_direction")?,
            flip_order: word("flip_order")?,
            gradient_layer: serde_json::Value::from(
                parameter(query, "gradient_layer").ok_or_else(|| "What should gradient_layer be set to?".to_string())?.trim(),
            ),
            gradient_map: None,
            timing_randomness: number("timing_randomness")?,
            random_seed: number("random_seed")?,
            x_rotation: number("x_rotation")?,
            y_rotation: number("y_rotation")?,
            z_rotation: number("z_rotation")?,
            xy_position: pair("xy_position")?,
            z_position: number("z_position")?,
            focal_length: number("focal_length")?,
            light_type: word("light_type")?,
            light_intensity: number("light_intensity")?,
            light_color: word("light_color")?,
            light_position: pair("light_position")?,
            light_depth: number("light_depth")?,
            ambient_light: number("ambient_light")?,
            diffuse_reflection: number("diffuse_reflection")?,
            specular_reflection: number("specular_reflection")?,
            highlight_sharpness: number("highlight_sharpness")?,
            // D-634: each as added when the command does not say.
            camera_system: if parameter(query, "camera_system").is_some() { word("camera_system")? } else { "camera_position".to_string() },
            upper_left_corner: if parameter(query, "upper_left_corner").is_some() { pair("upper_left_corner")? } else { [0.0, 0.0] },
            upper_right_corner: if parameter(query, "upper_right_corner").is_some() { pair("upper_right_corner")? } else { [100.0, 0.0] },
            lower_left_corner: if parameter(query, "lower_left_corner").is_some() { pair("lower_left_corner")? } else { [0.0, 100.0] },
            lower_right_corner: if parameter(query, "lower_right_corner").is_some() { pair("lower_right_corner")? } else { [100.0, 100.0] },
            auto_focal_length: if parameter(query, "auto_focal_length").is_some() { word("auto_focal_length")? } else { "off".to_string() },
        }),
        CC_SPHERE => Ok(Effect::CcSphere {
            radius: number("radius")?,
            offset: pair("offset")?,
            rotation_x: number("rotation_x")?,
            rotation_y: number("rotation_y")?,
            rotation_z: number("rotation_z")?,
            render: word("render")?,
            light_intensity: number("light_intensity")?,
            light_color: word("light_color")?,
            light_height: number("light_height")?,
            light_direction: number("light_direction")?,
            ambient: number("ambient")?,
            diffuse: number("diffuse")?,
            specular: number("specular")?,
            roughness: number("roughness")?,
            metal: number("metal")?,
        }),
        CC_CYLINDER => Ok(Effect::CcCylinder {
            radius: number("radius")?,
            position: pair("position")?,
            position_z: number("position_z")?,
            rotation_x: number("rotation_x")?,
            rotation_y: number("rotation_y")?,
            rotation_z: number("rotation_z")?,
            render: word("render")?,
            light_intensity: number("light_intensity")?,
            light_color: word("light_color")?,
            light_height: number("light_height")?,
            light_direction: number("light_direction")?,
            ambient: number("ambient")?,
            diffuse: number("diffuse")?,
            specular: number("specular")?,
            roughness: number("roughness")?,
            metal: number("metal")?,
        }),
        CC_SPOTLIGHT => Ok(Effect::CcSpotlight {
            from: pair("from")?,
            to: pair("to")?,
            height: number("height")?,
            cone_angle: number("cone_angle")?,
            edge_softness: number("edge_softness")?,
            color: word("color")?,
            intensity: number("intensity")?,
            render: word("render")?,
            // D-519: the gel is read by compose each frame; none sent is none.
            gel_layer: serde_json::Value::from(parameter(query, "gel_layer").unwrap_or_default().trim()),
            gel_map: None,
        }),
        CC_BALL_ACTION => Ok(Effect::CcBallAction {
            scatter: number("scatter")?,
            rotation_axis: word("rotation_axis")?,
            rotation: number("rotation")?,
            twist_property: word("twist_property")?,
            twist_angle: number("twist_angle")?,
            grid_spacing: number("grid_spacing")?,
            ball_size: number("ball_size")?,
            instability_state: number("instability_state")?,
        }),
        // D-610: every setting by its name; compose fills the clock, view and history. D-611: the
        // texture layer travels as `map_layer`.
        PARTICLE_SPACE => {
            let mut values = [0.0; 46];
            for (v, s) in values.iter_mut().zip(PARTICLE_SPACE_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 7] = Default::default();
            for (w, s) in words.iter_mut().zip(PARTICLE_SPACE_WORDS) {
                *w = word(s.0)?;
            }
            let mut colors: [String; 8] = Default::default();
            for (c, s) in colors.iter_mut().zip(PARTICLE_SPACE_COLORS) {
                *c = word(s.0)?;
            }
            Ok(Effect::ParticleSpace {
                values,
                words,
                colors,
                clock: [0.0; 2],
                view: [0.0; 4],
                history: None,
                texture_layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
                texture: None,
            })
        }
        // D-672: every setting by its name and the light's colour; compose fills the clock and
        // history.
        DRIZZLE => {
            let mut values = [0.0; 16];
            for (v, s) in values.iter_mut().zip(DRIZZLE_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 1] = Default::default();
            for (w, s) in words.iter_mut().zip(DRIZZLE_WORDS) {
                *w = word(s.0)?;
            }
            Ok(Effect::Drizzle { values, words, light_color: word("light_color")?, clock: [0.0; 2], history: None })
        }
        // D-673: every setting by its name and the hair's colour.
        HAIR => {
            let mut values = [0.0; 13];
            for (v, s) in values.iter_mut().zip(HAIR_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 1] = Default::default();
            for (w, s) in words.iter_mut().zip(HAIR_WORDS) {
                *w = word(s.0)?;
            }
            Ok(Effect::Hair { values, words, hair_color: word("hair_color")? })
        }
        // D-674: every setting by its name and the light's colour; compose fills the clock and
        // history.
        MERCURY => {
            let mut values = [0.0; 24];
            for (v, s) in values.iter_mut().zip(MERCURY_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 3] = Default::default();
            for (w, s) in words.iter_mut().zip(MERCURY_WORDS) {
                *w = word(s.0)?;
            }
            Ok(Effect::Mercury { values, words, light_color: word("light_color")?, clock: [0.0; 2], history: None })
        }
        // D-675: every setting by its name; compose fills the clock and the camera.
        POLYGON_SHARDS => {
            let mut values = [0.0; 9];
            for (v, s) in values.iter_mut().zip(POLYGON_SHARDS_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 2] = Default::default();
            for (w, s) in words.iter_mut().zip(POLYGON_SHARDS_WORDS) {
                *w = word(s.0)?;
            }
            Ok(Effect::PolygonShards { values, words, clock: [0.0; 2], view: [0.0; 4] })
        }
        // D-678: every setting by its name; compose fills the clock and history.
        FOAM => {
            let mut values = [0.0; 26];
            for (v, s) in values.iter_mut().zip(FOAM_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 6] = Default::default();
            for (w, s) in words.iter_mut().zip(FOAM_WORDS) {
                *w = word(s.0)?;
            }
            Ok(Effect::Foam { values, words, clock: [0.0; 2], history: None })
        }
        // D-676: every setting by its name.
        SCATTERIZE => {
            let mut values = [0.0; 3];
            for (v, s) in values.iter_mut().zip(SCATTERIZE_SETTINGS) {
                *v = number(s.0)?;
            }
            Ok(Effect::Scatterize { values, words: [word(SCATTERIZE_WORDS[0].0)?] })
        }
        // D-677: every setting by its name; compose fills the clock, view and history.
        STAR_BURST => {
            let mut values = [0.0; 6];
            for (v, s) in values.iter_mut().zip(STAR_BURST_SETTINGS) {
                *v = number(s.0)?;
            }
            Ok(Effect::StarBurst { values, clock: [0.0; 2], view: [0.0; 4], history: None })
        }
        // D-671: every setting by its name; compose fills the clock and history.
        BUBBLES => {
            let mut values = [0.0; 6];
            for (v, s) in values.iter_mut().zip(BUBBLES_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 2] = Default::default();
            for (w, s) in words.iter_mut().zip(BUBBLES_WORDS) {
                *w = word(s.0)?;
            }
            Ok(Effect::Bubbles { values, words, clock: [0.0; 2], history: None })
        }
        // D-612: every setting by its name; compose fills the clock and history.
        PARTICLE_PLANE => {
            let mut values = [0.0; 25];
            for (v, s) in values.iter_mut().zip(PARTICLE_PLANE_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 6] = Default::default();
            for (w, s) in words.iter_mut().zip(PARTICLE_PLANE_WORDS) {
                *w = word(s.0)?;
            }
            let mut colors: [String; 7] = Default::default();
            for (c, s) in colors.iter_mut().zip(PARTICLE_PLANE_COLORS) {
                *c = word(s.0)?;
            }
            Ok(Effect::ParticlePlane {
                values,
                words,
                colors,
                clock: [0.0; 2],
                history: None,
                texture_layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
                texture: None,
            })
        }
        // D-613: every setting by its name, the layers as written; compose fills the rest.
        PARTICLE_PLAYGROUND => {
            let mut values = [0.0; 45];
            for (v, s) in values.iter_mut().zip(PARTICLE_PLAYGROUND_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 14] = Default::default();
            for (w, s) in words.iter_mut().zip(PARTICLE_PLAYGROUND_WORDS) {
                *w = word(s.0)?;
            }
            let mut layers: [serde_json::Value; 10] = Default::default();
            for (l, s) in layers.iter_mut().zip(PARTICLE_PLAYGROUND_LAYERS) {
                *l = serde_json::Value::from(parameter(query, s).ok_or_else(|| format!("What should {s} be set to?"))?.trim());
            }
            let mut colors: [String; 2] = Default::default();
            for (c, s) in colors.iter_mut().zip(PARTICLE_PLAYGROUND_COLORS) {
                *c = word(s.0)?;
            }
            Ok(Effect::ParticlePlayground {
                values,
                cannon_position: pair("cannon_position")?.to_vec(),
                grid_position: pair("grid_position")?.to_vec(),
                words,
                layers,
                colors,
                maps: Default::default(),
                wall: None,
                clock: [0.0; 2],
                history: None,
            })
        }
        // D-615: every setting by its name; compose fills the clock, camera and history.
        PARTICLE_EMITTER => {
            let mut values = [0.0; 54];
            for (v, s) in values.iter_mut().zip(PARTICLE_EMITTER_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 9] = Default::default();
            for (w, s) in words.iter_mut().zip(PARTICLE_EMITTER_WORDS) {
                *w = word(s.0)?;
            }
            let mut colors: [String; 7] = Default::default();
            for (c, s) in colors.iter_mut().zip(PARTICLE_EMITTER_COLORS) {
                *c = word(s.0)?;
            }
            Ok(Effect::ParticleEmitter {
                values,
                emitter_position: pair("emitter_position")?.to_vec(),
                words,
                colors,
                clock: [0.0; 2],
                view: [0.0; 4],
                history: None,
            })
        }
        // D-616: every setting by its name, the layers as written; compose fills the rest.
        PARTICLE_GRID => {
            let mut values = [0.0; 28];
            for (v, s) in values.iter_mut().zip(PARTICLE_GRID_SETTINGS) {
                *v = number(s.0)?;
            }
            let mut words: [String; 3] = Default::default();
            for (w, s) in words.iter_mut().zip(PARTICLE_GRID_WORDS) {
                *w = word(s.0)?;
            }
            let mut layers: [serde_json::Value; 4] = Default::default();
            for (l, s) in layers.iter_mut().zip(PARTICLE_GRID_LAYERS) {
                *l = serde_json::Value::from(parameter(query, s).ok_or_else(|| format!("What should {s} be set to?"))?.trim());
            }
            let mut colors: [String; 1] = Default::default();
            for (c, s) in colors.iter_mut().zip(PARTICLE_GRID_COLORS) {
                *c = word(s.0)?;
            }
            Ok(Effect::ParticleGrid {
                values,
                form_center: pair("form_center")?.to_vec(),
                words,
                layers,
                colors,
                maps: Default::default(),
                clock: [0.0; 2],
                view: [0.0; 4],
                history: None,
            })
        }
        // D-512: as Gradient Wipe's, the layer travels as `map_layer`.
        CHANNEL_WIPE => Ok(Effect::ChannelWipe {
            layer: serde_json::Value::from(
                parameter(query, "map_layer")
                    .ok_or_else(|| "What should map_layer be set to?".to_string())?
                    .trim(),
            ),
            fit: word("fit")?,
            completion: number("completion")?,
            softness: number("softness")?,
            property: word("property")?,
            blur: number("blur")?,
            invert: word("invert")?,
            map: None,
        }),
        // D-518: the layer is read by compose each frame.
        WARP_BLEND => Ok(Effect::WarpBlend {
            completion: number("completion")?,
            layer_to_reveal: serde_json::Value::from(
                parameter(query, "layer_to_reveal").ok_or_else(|| "What should layer_to_reveal be set to?".to_string())?.trim(),
            ),
            reactor: word("reactor")?,
            smoothness: number("smoothness")?,
            warp_amount: number("warp_amount")?,
            warp_direction: word("warp_direction")?,
            blend_span: number("blend_span")?,
            map: None,
        }),
        // D-517: the back is read by compose each frame.
        TWISTER => Ok(Effect::Twister {
            completion: number("completion")?,
            backside: serde_json::Value::from(
                parameter(query, "backside").ok_or_else(|| "What should backside be set to?".to_string())?.trim(),
            ),
            shading: word("shading")?,
            center: pair("center")?,
            axis: number("axis")?,
            map: None,
        }),
        STRETCH_WIPE => Ok(Effect::StretchWipe {
            scale: number("scale")?,
            center: pair("center")?,
            direction: number("direction")?,
        }),
        PUSH_IRIS => Ok(Effect::PushIris {
            completion: number("completion")?,
            center: pair("center")?,
            reverse: word("reverse")?,
        }),
        STRIPE_SWEEP => Ok(Effect::StripeSweep {
            completion: number("completion")?,
            direction: number("direction")?,
            thickness: number("thickness")?,
            slant: number("slant")?,
            flip: word("flip")?,
        }),
        GLOW_WIPE => Ok(Effect::GlowWipe {
            completion: number("completion")?,
            center: pair("center")?,
            intensity: number("intensity")?,
            direction: number("direction")?,
            shape: word("shape")?,
            color_from_source: word("color_from_source")?,
            color: word("color")?,
            reverse: word("reverse")?,
        }),
        HALATION => Ok(Effect::Halation {
            threshold: number("threshold")?,
            threshold_smooth: number("threshold_smooth")?,
            radius: number("radius")?,
            amount: number("amount")?,
            color: word("color")?,
        }),
        TEETH_WIPE => Ok(Effect::TeethWipe {
            completion: number("completion")?,
            center: pair("center")?,
            direction: number("direction")?,
            height: number("height")?,
            width: number("width")?,
            shape: word("shape")?,
        }),
        TILE_WIPE => Ok(Effect::TileWipe {
            completion: number("completion")?,
            center: pair("center")?,
            rotation: number("rotation")?,
            border: number("border")?,
            tiles: number("tiles")?,
            shape: word("shape")?,
            reverse: word("reverse")?,
        }),
        // D-509: the layers are read by compose each frame.
        GLASS_MELT => Ok(Effect::GlassMelt {
            completion: number("completion")?,
            layer_to_reveal: serde_json::Value::from(
                parameter(query, "layer_to_reveal").ok_or_else(|| "What should layer_to_reveal be set to?".to_string())?.trim(),
            ),
            gradient_layer: serde_json::Value::from(
                parameter(query, "gradient_layer").ok_or_else(|| "What should gradient_layer be set to?".to_string())?.trim(),
            ),
            softness: number("softness")?,
            displacement_amount: number("displacement_amount")?,
            reveal_map: None,
            gradient_map: None,
        }),
        THREE_D_GLASSES => Ok(Effect::ThreeDGlasses {
            left_view: serde_json::Value::from(
                parameter(query, "left_view").ok_or_else(|| "What should left_view be set to?".to_string())?.trim(),
            ),
            right_view: serde_json::Value::from(
                parameter(query, "right_view").ok_or_else(|| "What should right_view be set to?".to_string())?.trim(),
            ),
            scene_convergence: number("scene_convergence")?,
            vertical_alignment: number("vertical_alignment")?,
            units: word("units")?,
            swap_left_right: word("swap_left_right")?,
            three_d_view: word("three_d_view")?,
            balance: number("balance")?,
            left_map: None,
            right_map: None,
        }),
        // D-458: the other frame is read by compose each frame.
        SIMPLE_WIRE_REMOVAL => Ok(Effect::SimpleWireRemoval {
            point_a: pair("point_a")?,
            point_b: pair("point_b")?,
            removal_style: word("removal_style")?,
            thickness: number("thickness")?,
            slope: number("slope")?,
            mirror_blend: number("mirror_blend")?,
            frame_offset: number("frame_offset")?,
            picture: None,
        }),
        // D-441: the marks are found by compose each frame.
        // D-442: the masks and the time are found by compose each frame.
        SCRIBBLE => Ok(Effect::Scribble {
            scribble: word("scribble")?,
            mask: number("mask")?,
            fill_type: word("fill_type")?,
            edge_width: number("edge_width")?,
            color: word("color")?,
            opacity: number("opacity")?,
            angle: number("angle")?,
            stroke_width: number("stroke_width")?,
            curviness: number("curviness")?,
            curviness_variation: number("curviness_variation")?,
            spacing: number("spacing")?,
            spacing_variation: number("spacing_variation")?,
            path_overlap: number("path_overlap")?,
            path_overlap_variation: number("path_overlap_variation")?,
            start: number("start")?,
            end: number("end")?,
            fill_paths_sequentially: word("fill_paths_sequentially")?,
            wiggle_type: word("wiggle_type")?,
            wiggles_per_second: number("wiggles_per_second")?,
            random_seed: number("random_seed")?,
            composite: word("composite")?,
            masks: None,
            time: 0.0,
        }),
        WRITE_ON => Ok(Effect::WriteOn {
            brush_position: pair("brush_position")?,
            color: word("color")?,
            brush_size: number("brush_size")?,
            brush_hardness: number("brush_hardness")?,
            brush_opacity: number("brush_opacity")?,
            stroke_length: number("stroke_length")?,
            brush_spacing: number("brush_spacing")?,
            paint_time_properties: word("paint_time_properties")?,
            brush_time_properties: word("brush_time_properties")?,
            paint_style: word("paint_style")?,
            marks: Vec::new(),
        }),
        FILL => Ok(Effect::Fill {
            mask: number("mask")?,
            all_masks: word("all_masks")?,
            color: word("color")?,
            invert: word("invert")?,
            horizontal_feather: number("horizontal_feather")?,
            vertical_feather: number("vertical_feather")?,
            opacity: number("opacity")?,
            paths: None,
        }),
        GRID => Ok(Effect::Grid {
            anchor: pair("anchor")?,
            size_from: word("size_from")?,
            corner: pair("corner")?,
            width: number("width")?,
            height: number("height")?,
            border: number("border")?,
            feather_width: number("feather_width")?,
            feather_height: number("feather_height")?,
            invert: word("invert")?,
            color: word("color")?,
            opacity: number("opacity")?,
            blending_mode: word("blending_mode")?,
        }),
        // D-395: the map's asset id as written, as Color Lookup's.
        ARBITRARY_MAP => Ok(Effect::ArbitraryMap {
            map: parameter(query, "map")
                .map(|t| t.trim().to_string())
                .ok_or_else(|| "What should map be set to?".to_string())?,
            phase: number("phase")?,
            apply_to_alpha: word("apply_to_alpha")?,
            table: None,
        }),
        // D-396: each family as the page sends it, four numbers with commas, as Channel Mixer's
        // rows. How many there are is the core's check, in its words.
        SELECTIVE_COLOR => {
            let mut families: [Vec<f64>; 9] = Default::default();
            for (row, name) in families.iter_mut().zip(anime_compositor::effects::SELECTIVE_COLOR_FAMILIES) {
                let text = word(name)?;
                *row = text
                    .split(',')
                    .map(|p| p.trim().parse::<f64>())
                    .collect::<Result<_, _>>()
                    .map_err(|_| format!("{name} needs four numbers, like 0, 0, 0, 0. Not \"{text}\"."))?;
            }
            Ok(Effect::SelectiveColor { method: word("method")?, families })
        }
        SHADOW_HIGHLIGHT => Ok(Effect::ShadowHighlight {
            shadow_amount: number("shadow_amount")?,
            highlight_amount: number("highlight_amount")?,
            shadow_tonal_width: number("shadow_tonal_width")?,
            shadow_radius: number("shadow_radius")?,
            highlight_tonal_width: number("highlight_tonal_width")?,
            highlight_radius: number("highlight_radius")?,
            color_correction: number("color_correction")?,
            midtone_contrast: number("midtone_contrast")?,
            black_clip: number("black_clip")?,
            white_clip: number("white_clip")?,
            auto_amounts: word("auto_amounts")?,
            temporal_smoothing: number("temporal_smoothing")?,
            scene_detect: word("scene_detect")?,
            stats: None,
        }),
        RIPPLE_PULSE => Ok(Effect::RipplePulse {
            center: pair("center")?,
            pulse_level: number("pulse_level")?,
            time_span: number("time_span")?,
            amplitude: number("amplitude")?,
            render_bump_map: word("render_bump_map")?,
            levels: Vec::new(),
        }),
        // D-379: as CC Glass's, the layer travels as `map_layer`.
        BLOBBYLIZE => Ok(Effect::Blobbylize {
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            fit: word("fit")?,
            property: word("property")?,
            softness: number("softness")?,
            cut_away: number("cut_away")?,
            light_intensity: number("light_intensity")?,
            light_color: word("light_color")?,
            light_type: word("light_type")?,
            light_height: number("light_height")?,
            light_position: pair("light_position")?,
            light_direction: number("light_direction")?,
            ambient: number("ambient")?,
            diffuse: number("diffuse")?,
            specular: number("specular")?,
            roughness: number("roughness")?,
            metal: number("metal")?,
            map: None,
        }),
        GLUE_GUN => Ok(Effect::GlueGun {
            brush_position: pair("brush_position")?,
            stroke_width: number("stroke_width")?,
            density: number("density")?,
            time_span: number("time_span")?,
            reflection: number("reflection")?,
            strength: number("strength")?,
            paint_style: word("paint_style")?,
            wobble_width: number("wobble_width")?,
            wobble_height: number("wobble_height")?,
            wobble_speed: number("wobble_speed")?,
            light_intensity: number("light_intensity")?,
            light_color: word("light_color")?,
            light_type: word("light_type")?,
            light_height: number("light_height")?,
            light_position: pair("light_position")?,
            light_direction: number("light_direction")?,
            ambient: number("ambient")?,
            diffuse: number("diffuse")?,
            specular: number("specular")?,
            roughness: number("roughness")?,
            metal: number("metal")?,
            trail: Vec::new(),
            clock: [0.0, 0.0],
        }),
        LENS_FLARE => Ok(Effect::LensFlare {
            flare_center: pair("flare_center")?,
            flare_brightness: number("flare_brightness")?,
            lens_type: word("lens_type")?,
            blend_with_original: number("blend_with_original")?,
            dirt: dirt()?,
        }),
        // D-445: the depth is set by compose each frame.
        TURBULENT_NOISE => Ok(Effect::TurbulentNoise {
            fractal_type: word("fractal_type")?,
            noise_type: word("noise_type")?,
            invert: word("invert")?,
            contrast: number("contrast")?,
            brightness: number("brightness")?,
            size: number("size")?,
            scale_width: number("scale_width")?,
            scale_height: number("scale_height")?,
            offset: pair("offset")?,
            complexity: number("complexity")?,
            evolution: number("evolution")?,
            seed: number("seed")?,
            opacity: number("opacity")?,
            blend: word("blend")?,
            float: false,
            more: noise_more()?,
        }),
        // D-446: the frame and the depth are set by compose each frame.
        CURL_NOISE => Ok(Effect::CurlNoise {
            source: word("source")?,
            // D-583: the four sent by a window before them read as added.
            source_layer: serde_json::Value::from(parameter(query, "source_layer").unwrap_or_default().trim()),
            softness: if parameter(query, "softness").is_some() { number("softness")? } else { 0.0 },
            source_mapping: if parameter(query, "source_mapping").is_some() { word("source_mapping")? } else { "fill".to_string() },
            mirror_at_edge: if parameter(query, "mirror_at_edge").is_some() { word("mirror_at_edge")? } else { "off".to_string() },
            map: None,
            speed: number("speed")?,
            direction: number("direction")?,
            size: number("size")?,
            offset: pair("offset")?,
            // D-646: the three sent by a window before them read as added.
            rotation: if parameter(query, "rotation").is_some() { number("rotation")? } else { 0.0 },
            scale: if parameter(query, "scale").is_some() { number("scale")? } else { 100.0 },
            perspective_offset: if parameter(query, "perspective_offset").is_some() { word("perspective_offset")? } else { "off".to_string() },
            evolution: number("evolution")?,
            turbulence_speed: number("turbulence_speed")?,
            swirl: number("swirl")?,
            density: number("density")?,
            smoothness: number("smoothness")?,
            vertical_bias: number("vertical_bias")?,
            sample_count: number("sample_count")?,
            sample_radius: number("sample_radius")?,
            flow_softness: number("flow_softness")?,
            edge_definition: number("edge_definition")?,
            flow_falloff: number("flow_falloff")?,
            view: word("view")?,
            contrast: number("contrast")?,
            brightness: number("brightness")?,
            clip_hdr_results: word("clip_hdr_results")?,
            channel: word("channel")?,
            frame: 0,
            float: false,
        }),
        // D-444: the paths are found by compose each frame.
        VEGAS => Ok(Effect::Vegas {
            stroke: word("stroke")?,
            mask: number("mask")?,
            all_masks: word("all_masks")?,
            segments: number("segments")?,
            length: number("length")?,
            segment_distribution: word("segment_distribution")?,
            rotation: number("rotation")?,
            random_phase: word("random_phase")?,
            random_seed: number("random_seed")?,
            blend_mode: word("blend_mode")?,
            color: word("color")?,
            width: number("width")?,
            hardness: number("hardness")?,
            start_opacity: number("start_opacity")?,
            mid_point_opacity: number("mid_point_opacity")?,
            mid_point_position: number("mid_point_position")?,
            end_opacity: number("end_opacity")?,
            paths: None,
        }),
        THREADS => Ok(Effect::Threads {
            width: number("width")?,
            height: number("height")?,
            overlaps: number("overlaps")?,
            direction: number("direction")?,
            center: pair("center")?,
            coverage: number("coverage")?,
            shadowing: number("shadowing")?,
            texture: number("texture")?,
        }),
        // D-189: as Compound Blur's, the layer travels as `map_layer`; none named is "".
        GLASS => Ok(Effect::Glass {
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            fit: word("fit")?,
            property: word("property")?,
            softness: number("softness")?,
            height: number("height")?,
            displacement: number("displacement")?,
            light_angle: number("light_angle")?,
            light_color: word("light_color")?,
            light_intensity: number("light_intensity")?,
            light_type: word("light_type")?,
            light_height: number("light_height")?,
            light_position: pair("light_position")?,
            ambient: number("ambient")?,
            diffuse: number("diffuse")?,
            specular: number("specular")?,
            roughness: number("roughness")?,
            metal: number("metal")?,
            map: None,
        }),
        // D-336: as CC Glass's, the layer travels as `map_layer`.
        VECTOR_BLUR => Ok(Effect::VectorBlur {
            kind: word("type")?,
            amount: number("amount")?,
            angle_offset: number("angle_offset")?,
            ridge_smoothness: number("ridge_smoothness")?,
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            fit: word("fit")?,
            property: word("property")?,
            map_softness: number("map_softness")?,
            map: None,
        }),
        // D-347: the layer travels as `map_layer`, as Vector Blur's.
        MOMENT_MAP => Ok(Effect::MomentMap {
            max_time: number("max_time")?,
            resolution: number("resolution")?,
            layer: serde_json::Value::from(parameter(query, "map_layer").unwrap_or_default().trim()),
            fit: word("fit")?,
            map: None,
            picture: None,
        }),
        // D-348.
        PASS_EXTRACT => Ok(Effect::PassExtract {
            pass: word("pass")?,
            black_point: number("black_point")?,
            white_point: number("white_point")?,
            invert: word("invert")?,
            clamp: word("clamp")?,
            // D-349: the channel names are free text, so they travel as typed.
            channel: parameter(query, "channel").unwrap_or_default().trim().to_string(),
            // D-552.
            unmult: word("unmult")?,
            channels: None,
        }),
        DEPTH_KEY => Ok(Effect::DepthKey {
            depth: number("depth")?,
            feather: number("feather")?,
            invert: word("invert")?,
            channels: None,
        }),
        TIMECODE => Ok(Effect::Timecode {
            display_format: word("display_format")?,
            time_source: word("time_source")?,
            time_units: number("time_units")?,
            drop_frame: word("drop_frame")?,
            starting_frame: number("starting_frame")?,
            text_position: pair("text_position")?,
            text_size: number("text_size")?,
            text_color: word("text_color")?,
            show_box: word("show_box")?,
            box_color: word("box_color")?,
            opacity: number("opacity")?,
            composite_on_original: word("composite_on_original")?,
            count: [0.0; 3],
        }),
        // D-526: the type travels as number_type, since type names the effect; the font as
        // typed, since a font's file name keeps its capitals.
        NUMBERS => Ok(Effect::Numbers {
            kind: word("number_type")?,
            random_values: word("random_values")?,
            value: number("value")?,
            decimal_places: number("decimal_places")?,
            position: pair("position")?,
            display: word("display")?,
            fill_color: word("fill_color")?,
            stroke_color: word("stroke_color")?,
            stroke_width: number("stroke_width")?,
            size: number("size")?,
            tracking: number("tracking")?,
            proportional_spacing: word("proportional_spacing")?,
            composite_on_original: word("composite_on_original")?,
            font: parameter(query, "font").map(|f| f.trim().to_string()).ok_or_else(|| "What should font be set to?".to_string())?,
            alignment: word("alignment")?,
            clock: [0.0; 2],
        }),
        // D-473: the text and the font as typed.
        BASIC_TEXT => Ok(Effect::BasicText {
            text: parameter(query, "text").map(|t| t.to_string()).ok_or_else(|| "What should text be set to?".to_string())?,
            position: pair("position")?,
            display: word("display")?,
            fill_color: word("fill_color")?,
            stroke_color: word("stroke_color")?,
            stroke_width: number("stroke_width")?,
            size: number("size")?,
            tracking: number("tracking")?,
            composite_on_original: word("composite_on_original")?,
            font: parameter(query, "font").map(|f| f.trim().to_string()).ok_or_else(|| "What should font be set to?".to_string())?,
            alignment: word("alignment")?,
        }),
        IDENTIFIER => Ok(Effect::Identifier {
            aux_channel: word("aux_channel")?,
            id: number("id")?,
            output: word("output")?,
            channels: None,
        }),
        ID_KEY => Ok(Effect::IdKey {
            aux_channel: word("aux_channel")?,
            id: number("id")?,
            feather: number("feather")?,
            invert: word("invert")?,
            channels: None,
        }),
        // D-522: the layer and the names are free text, so they travel as typed; the layer as
        // `crypto_layer`, since `layer` names the layer the effect is on.
        CRYPTOMATTE => Ok(Effect::Cryptomatte {
            layer: parameter(query, "crypto_layer").unwrap_or_default().trim().to_string(),
            selection: parameter(query, "selection").unwrap_or_default().trim().to_string(),
            output: word("output")?,
            channels: None,
        }),
        FOG_3D => Ok(Effect::Fog3d {
            fog_color: word("fog_color")?,
            fog_start_depth: number("fog_start_depth")?,
            fog_end_depth: number("fog_end_depth")?,
            fog_opacity: number("fog_opacity")?,
            scattering_density: number("scattering_density")?,
            foggy_background: word("foggy_background")?,
            gradient_layer: serde_json::Value::from(
                parameter(query, "map_layer")
                    .ok_or_else(|| "What should map_layer be set to?".to_string())?
                    .trim(),
            ),
            layer_contribution: number("layer_contribution")?,
            map: None,
            channels: None,
        }),
        DEPTH_OF_FIELD => Ok(Effect::DepthOfField {
            focal_plane: number("focal_plane")?,
            maximum_radius: number("maximum_radius")?,
            focal_plane_thickness: number("focal_plane_thickness")?,
            focal_bias: number("focal_bias")?,
            channels: None,
        }),
        TEXT_ANIMATOR => Ok(Effect::TextAnimator {
            position: pair("position")?,
            scale: pair("scale")?,
            rotation: number("rotation")?,
            opacity: number("opacity")?,
            fill: word("fill")?,
            color: {
                let text = word("color")?;
                text.split(',')
                    .map(|p| p.trim().parse::<f64>())
                    .collect::<Result<Vec<_>, _>>()
                    .ok()
                    .and_then(|v| <[f64; 3]>::try_from(v).ok())
                    .ok_or_else(|| format!("color needs three numbers, like 1, 0.5, 0. Not \"{text}\"."))?
            },
            tracking: number("tracking")?,
            start: number("start")?,
            end: number("end")?,
            offset: number("offset")?,
            amount: number("amount")?,
            based_on: word("based_on")?,
            shape: word("shape")?,
            smoothness: number("smoothness")?,
            ease_high: number("ease_high")?,
            ease_low: number("ease_low")?,
        }),
        // D-351: Stretch Color's snap is its own; the other two have none.
        STRETCH_LEVELS | STRETCH_CONTRAST | STRETCH_COLOR => Ok(Effect::AutoTone {
            kind: match type_id {
                STRETCH_LEVELS => "levels",
                STRETCH_CONTRAST => "contrast",
                _ => "color",
            },
            temporal_smoothing: number("temporal_smoothing")?,
            scene_detect: word("scene_detect")?,
            black_clip: number("black_clip")?,
            white_clip: number("white_clip")?,
            snap_neutral_midtones: if type_id == STRETCH_COLOR { word("snap_neutral_midtones")? } else { "off".to_string() },
            stats: None,
        }),
        SPREAD_TONES => Ok(Effect::SpreadTones { equalize: word("equalize")?, amount: number("amount")? }),
        MATTE_CHOKER => Ok(Effect::MatteChoker {
            geometric_softness_1: number("geometric_softness_1")?,
            choke_1: number("choke_1")?,
            gray_level_softness_1: number("gray_level_softness_1")?,
            geometric_softness_2: number("geometric_softness_2")?,
            choke_2: number("choke_2")?,
            gray_level_softness_2: number("gray_level_softness_2")?,
            iterations: number("iterations")?,
        }),
        // D-352: the edge radius and its view are Refine Soft Matte's own.
        REFINE_HARD_MATTE | REFINE_SOFT_MATTE => {
            let soft = type_id == REFINE_SOFT_MATTE;
            Ok(Effect::RefineMatte {
                kind: if soft { "soft" } else { "hard" },
                edge_radius: if soft { number("edge_radius")? } else { 0.0 },
                view_edge_region: if soft { word("view_edge_region")? } else { "off".to_string() },
                feather: number("feather")?,
                contrast: number("contrast")?,
                shift_edge: number("shift_edge")?,
                decontaminate: word("decontaminate")?,
                decontamination_amount: number("decontamination_amount")?,
                decontamination_radius: number("decontamination_radius")?,
                view_decontamination_map: word("view_decontamination_map")?,
            })
        }
        // D-356: the paths are found by compose each frame.
        STROKE => Ok(Effect::Stroke {
            mask: number("mask")?,
            all_masks: word("all_masks")?,
            stroke_sequentially: word("stroke_sequentially")?,
            color: word("color")?,
            brush_size: number("brush_size")?,
            brush_hardness: number("brush_hardness")?,
            opacity: number("opacity")?,
            start: number("start")?,
            end: number("end")?,
            spacing: number("spacing")?,
            paint_style: word("paint_style")?,
            // D-357: as D-315's expand, Masks when the page does not say.
            source: word("source").unwrap_or_else(|_| "masks".to_string()),
            paths: None,
        }),
        // Document 19 keeps an effect this build does not have rather than dropping it, and
        // keeping it means keeping its settings as they were written. There is no schema here
        // to read them against, so they are left alone and said to be left alone.
        _ => Err(format!(
            "{type_id} is not an effect this build has, so its settings are kept as the file \
             wrote them and cannot be changed here."
        )),
    }
}

/// Everything the page can do to the open document, keyed by document 24's own command IDs.
///
/// The IDs are the URL paths, so what the page asks for and what the map lists are the same
/// string, and a command that is not in the map cannot be reached by spelling one. `None` means
/// this path was not one of them, which the caller answers with its 404.
///
/// Every answer, including a refusal, is a sentence for the status line. The list a button was
/// clicked in can be older than the document, so a layer that is no longer there is told about
/// rather than silently doing nothing.
/// The identifiers `edit_command` answers, and the only ones.
///
/// `None` is how this function says it has never heard of an identifier, and `fn command` reads
/// that as permission to try the routes it answers itself -- opening, saving, exporting. Without
/// this list an unknown identifier did not reach that `None`: it fell through to the layer
/// lookup at the bottom of the function and was answered "Which layer? Choose one in the layer
/// list.", which is a sentence, which is `Some`. Every route the shell answers was therefore
/// swallowed on its way past, and Open, Save, Save As, Export, the recent list and recovery all
/// stopped working in the built window while every test kept passing, because the tests call
/// those functions directly and only the window goes through here.
///
/// `verification/B-12b_page_table.md` checks that everything the page sends is in this list, and
/// `verification/B-12b_command_map_table.md` checks that nothing outside it is answered.
const ANSWERS: &[&str] = &[
    "asset.set_redistribute",
    "camera.set_property",
    "composition.create",
    "composition.delete",
    "composition.duplicate",
    "composition.open",
    "composition.set_frame_blending",
    "composition.set_motion_blur",
    "composition.set_settings",
    "edit.redo",
    "edit.undo",
    "effect.add",
    "effect.delete",
    "effect.fit_scale",
    "effect.move",
    "effect.move_down",
    "effect.move_up",
    "effect.paste",
    "effect.set_parameters",
    "effect.toggle_bypass",
    "exposure.set_span",
    "exposure.toggle_key",
    "exposure.write",
    "item.set_label",
    "keyframe.add_remove",
    "keyframe.move",
    "keyframe.reverse",
    "keyframe.set_interp",
    "keyframe.set_kind",
    "keyframe.set_path",
    "keyframe.set_roving",
    "layer.add_adjustment",
    "layer.add_composition",
    "layer.add_null",
    "layer.add_shape",
    "layer.add_solid",
    "layer.add_text",
    "layer.copy",
    "layer.create",
    "layer.delete",
    "layer.duplicate",
    "layer.freeze_frame",
    "layer.move",
    "layer.move_down",
    "layer.move_up",
    "layer.paste",
    "layer.precompose",
    "layer.rename",
    "layer.replace_source",
    "layer.set_blend_mode",
    "layer.set_depth",
    "layer.set_drawing_dissolve",
    "layer.set_gain",
    "layer.set_label",
    "layer.set_matte",
    "layer.set_matte_mode",
    "layer.set_parent",
    "layer.set_time_remap",
    "layer.set_time_stretch",
    "layer.shift",
    "layer.split",
    "layer.toggle_frame_blend",
    "layer.toggle_lock",
    "layer.toggle_motion_blur",
    "layer.toggle_shy",
    "layer.toggle_solo",
    "layer.toggle_visibility",
    "layer.trim",
    "mask.add",
    "mask.add_remove_key",
    "mask.delete",
    "mask.set",
    "mask.set_path",
    "media.import",
    "media.relink",
    "path.add_point",
    "path.remove_point",
    "preset.apply",
    "property.drag_cancel",
    "property.drag_end",
    "property.drag_update",
    "property.link",
    "property.separate",
    "property.set_base",
    "property.set_expression",
    "shape.add",
    "shape.add_remove_key",
    "shape.delete",
    "shape.group",
    "shape.operation",
    "shape.repeater",
    "shape.set",
    "shape.set_path",
    "shape.ungroup",
    "sheet.write_action",
    "sketch.add_stroke",
    "sketch.clear",
    "sketch.move_layer",
    "sketch.move_strokes",
    "sketch.remove_layer",
    "sketch.remove_strokes",
    "sketch.set_layer",
    "sketch.set_look",
    "solid.set",
    "text.set",
    "timeline.set_markers",
    "timeline.set_work_end",
    "timeline.set_work_start",
    "timesheet.import",
    "viewer.toggle_alpha",
    "viewer.toggle_checkerboard",
];

/// B-19g: whether the layer a request names has its position apart as X and Y (D-69).
fn separated(viewer: &Mutex<Viewer>, query: Option<&str>) -> bool {
    let held = viewer.lock().expect("the viewer lock was poisoned");
    held.document
        .project()
        .composition(&held.composition)
        .zip(parameter(query, "layer"))
        .and_then(|(comp, layer)| comp.layer(&Id::new(layer)))
        .is_some_and(|layer| layer.transform.position.split().is_some())
}

fn edit_command(viewer: &Mutex<Viewer>, id: &str, query: Option<&str>) -> Option<String> {
    if !ANSWERS.contains(&id) {
        return None;
    }
    // B-19g: a separated position (D-69) is still one thing to drag in the viewer and one pair
    // to type, so `position` sent to a layer whose position is apart is sent on as its X and
    // its Y, inside one drag, which makes it one entry to undo.
    if matches!(id, "property.set_base" | "property.drag_update")
        && parameter(query, "prop").as_deref() == Some("position")
        && separated(viewer, query)
    {
        if let Some((x, y)) = parameter(query, "value")
            .as_deref()
            .and_then(|v| v.split_once(','))
        {
            let rest: Vec<&str> = query
                .unwrap_or("")
                .split('&')
                .filter(|piece| !piece.starts_with("prop=") && !piece.starts_with("value="))
                .collect();
            let half = |prop: &str, value: &str| {
                let query = format!(
                    "{}&prop={prop}&value={}&drag=1",
                    rest.join("&"),
                    value.trim()
                );
                edit_command(viewer, id, Some(&query)).unwrap_or_default()
            };
            let said = half("position_x", x);
            let said = if said == "Dragging." {
                half("position_y", y)
            } else {
                said
            };
            let dragging = id == "property.drag_update" || parameter(query, "drag").is_some();
            return Some(match (dragging, said == "Dragging.") {
                (false, true) => end_drag(viewer),
                (false, false) => {
                    cancel_drag(viewer);
                    said
                }
                _ => said,
            });
        }
    }
    match id {
        // D-407: Fit to Comp Width and Fit to Comp Height, the owner's one-time buttons
        // (2026-10-10): Scale worked out once from the layer as it reaches the Upscale on `frame`
        // and written as any value is, a key on that frame when Scale has keys.
        "effect.fit_scale" => {
            let (Some(layer_id), Some(fx), Ok(frame)) = (
                parameter(query, "layer").map(Id::new),
                parameter(query, "effect").map(Id::new),
                frame_parameter(query, "frame"),
            ) else {
                return Some(
                    "Fit which effect, on which frame? Say layer, effect and frame.".to_string(),
                );
            };
            let across = match parameter(query, "to").as_deref() {
                Some("width") => true,
                Some("height") => false,
                _ => {
                    return Some(
                        "Fit to the width or the height? Say to=width or to=height.".to_string(),
                    )
                }
            };
            let (project, composition, root, cache) = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                (
                    held.document.project().clone(),
                    held.composition.clone(),
                    held.root.clone(),
                    Arc::clone(&held.cache),
                )
            };
            let Some(comp) = project.compositions.iter().find(|c| c.id == composition) else {
                return Some("There is no composition open.".to_string());
            };
            let (cw, ch) = (comp.width as f64, comp.height as f64);
            let Some(layer) = comp.layer(&layer_id) else {
                return Some(format!("{layer_id} is not a layer in this composition."));
            };
            let Some(at) = layer.effects.iter().position(|e| e.instance_id == fx) else {
                return Some(format!("{fx} is not an effect on this layer."));
            };
            if !matches!(layer.effects[at].effect, Effect::DetailUpscale { .. }) {
                return Some(format!("{fx} is not a Detail-preserving Upscale."));
            }
            // The layer as the Upscale is given it: without the Upscale or what follows, switched
            // off on a copy that has no history.
            let off: Vec<Command> = layer.effects[at..]
                .iter()
                .filter(|e| e.enabled)
                .map(|e| Command::SetEffectEnabled {
                    composition: composition.clone(),
                    layer_id: layer_id.clone(),
                    instance_id: e.instance_id.clone(),
                    enabled: false,
                })
                .collect();
            let mut copy = anime_compositor::command::Document::new(project.clone());
            if copy.apply_all(off).is_err() {
                return Some(format!("{fx} could not be looked at without itself."));
            }
            let plan = anime_compositor::compose::plan_frame_at(
                copy.project(),
                &composition,
                frame,
                &root,
                PreviewQuality::Full,
                &mut FrameLog::new(0),
                &mut cache.lock().expect("the cel cache lock was poisoned"),
            );
            let size = plan
                .ok()
                .and_then(|p| {
                    p.layers
                        .iter()
                        .find(|d| d.id == layer_id)
                        .map(|d| (d.source.width(), d.source.height()))
                })
                .filter(|&(w, h)| w > 0 && h > 0);
            let Some((w, h)) = size else {
                return Some(format!(
                    "{layer_id} is not drawn on frame {frame}, so there is nothing to fit."
                ));
            };
            let scale =
                (if across { cw / w as f64 } else { ch / h as f64 } * 100.0).clamp(100.0, 1000.0);
            return edit_command(
                viewer,
                "property.set_base",
                Some(&format!(
                    "layer={layer_id}&prop=fx:{fx}:scale&value={scale}&frame={frame}"
                )),
            );
        }
        // B-19g: D-69's Separate Dimensions, which is a toggle like the others: which way it
        // goes is read from the document rather than sent by the page.
        "property.separate" => {
            let Some(layer_id) = parameter(query, "layer").map(Id::new) else {
                return Some("Which layer? Choose one in the layer list.".to_string());
            };
            let separate = !separated(viewer, query);
            let composition = viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .composition
                .clone();
            return Some(edit(
                viewer,
                Command::SeparatePosition {
                    composition,
                    layer_id,
                    separate,
                },
            ));
        }
        // B-19g: D-69's key kinds and roving, on every chosen key as one entry to undo. The
        // keys are named `layer|prop|frame` as `keyframe.move` names them.
        "keyframe.set_kind" | "keyframe.set_roving" => {
            let kind = parameter(query, "kind").unwrap_or_default();
            let kind = match kind.as_str() {
                "bezier" => Some(anime_compositor::model::Kind::Bezier),
                other => anime_compositor::model::Kind::named(other),
            };
            if id == "keyframe.set_kind" && kind.is_none() {
                return Some("Which kind? Say bezier, continuous or auto.".to_string());
            }
            let roving = match parameter(query, "roving").as_deref() {
                Some("true") => true,
                Some("false") => false,
                _ if id == "keyframe.set_roving" => {
                    return Some("Roving is true or false.".to_string())
                }
                _ => false,
            };
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            let composition = held.composition.clone();
            let comp = held.document.project().composition(&composition);
            let mut chosen: std::collections::BTreeMap<(String, String), Vec<i32>> =
                Default::default();
            for named in parameters(query, "key") {
                let mut parts = named.rsplitn(3, '|');
                let (at, prop, layer) = (parts.next(), parts.next(), parts.next());
                let on = layer.and_then(|l| comp.and_then(|c| c.layer(&Id::new(l))));
                match (at.and_then(|at| stored_frames(on, at, 0)), prop, layer) {
                    (Some((at, _)), Some(prop), Some(layer)) => chosen
                        .entry((layer.to_string(), prop.to_string()))
                        .or_default()
                        .push(at),
                    _ => return Some(format!("A key is layer|property|frame. Not \"{named}\".")),
                }
            }
            if chosen.is_empty() {
                return Some("Which keys? Choose some on the timeline.".to_string());
            }
            let mut commands = Vec::new();
            for ((layer, prop), frames) in chosen {
                // ponytail: a layer's keys only. The camera's and an effect setting's keys keep
                // plain bezier handles until somebody asks for the kinds there; D-69 names
                // neither.
                let Some(prop) = property(&prop).filter(|_| layer != CAMERA_ROW) else {
                    return Some(
                        "Only a layer's own properties have these kinds of key.".to_string(),
                    );
                };
                let target = Target::Layer(Id::new(layer));
                let composition = composition.clone();
                commands.push(match kind {
                    Some(kind) if id == "keyframe.set_kind" => Command::SetKeyKind {
                        composition,
                        target,
                        prop,
                        frames,
                        kind,
                    },
                    _ => Command::SetKeyRoving {
                        composition,
                        target,
                        prop,
                        frames,
                        roving,
                    },
                });
            }
            return Some(match held.document.apply_all(commands) {
                Ok(record) => record.label.clone(),
                Err(diagnostic) => diagnostic.sentence(),
            });
        }
        "edit.undo" => return Some(undo(viewer)),
        "edit.redo" => return Some(redo(viewer)),
        // The two ends of an interaction transaction. Neither names a layer: the drag already
        // knows which value it is holding, and asking the page to name it again would be asking
        // it to be right about something it has no way to check.
        // The two ways of looking at a frame rather than changing it. Document 24's table gives
        // both "no" under undoable, and document 21 line 97 calls them presentation, so neither
        // touches the document: they set a flag the next frame is drawn through. That also means
        // neither marks the project dirty, which is the part a person would notice if it were
        // got wrong - looking at the alpha channel is not unsaved work.
        // W-24: After Effects' solo switch. Presentation, like the alpha view below: the preview
        // draws the soloed layers alone, and the file, the history and an export are untouched.
        "layer.toggle_solo" => {
            let Some(layer) = parameter(query, "layer").map(Id::new) else {
                return Some("Which layer? Choose one in the layer list.".to_string());
            };
            let mut held = viewer.lock().expect("the viewer lock was poisoned");
            let name = held
                .document
                .project()
                .composition(&held.composition)
                .and_then(|c| c.layer(&layer))
                .map(|l| l.name.clone())
                .unwrap_or_else(|| layer.to_string());
            let said = match held.solo.iter().position(|id| *id == layer) {
                Some(at) => {
                    held.solo.remove(at);
                    format!("{name} is no longer soloed.")
                }
                None => {
                    held.solo.push(layer);
                    format!("{name} is soloed: the preview shows the soloed layers alone.")
                }
            };
            return Some(said);
        }
        "viewer.toggle_alpha" => {
            let mut held = viewer.lock().expect("the viewer lock was poisoned");
            held.alpha_only = !held.alpha_only;
            return Some(match held.alpha_only {
                true => "Alpha-only inspection is on. The picture is the alpha channel, white \
                         where the frame is opaque and black where it is empty; what is exported \
                         is unchanged."
                    .to_string(),
                false => "Alpha-only inspection is off.".to_string(),
            });
        }
        "viewer.toggle_checkerboard" => {
            let mut held = viewer.lock().expect("the viewer lock was poisoned");
            held.checkerboard = !held.checkerboard;
            return Some(match held.checkerboard {
                true => "The transparency grid is on. It is drawn behind the frame and is not \
                         part of it."
                    .to_string(),
                false => "The transparency grid is off.".to_string(),
            });
        }
        "property.drag_end" => return Some(end_drag(viewer)),
        "property.drag_cancel" => return Some(cancel_drag(viewer)),
        // The one command that names files rather than anything in the project. Without any,
        // the request never reaches here: the window answers it with the operating system's
        // file dialog, which only the app handle can open.
        // W-02, in three requests rather than one: choose the drawings, read what would change,
        // then apply it or drop it. The middle step is the requirement - nothing about the
        // project moves until `apply` arrives, and `cancel` leaves it pointing where it did.
        "media.relink" => {
            let Some(asset) = parameter(query, "asset") else {
                return Some(
                    "Which sequence should be relinked? Choose one in the media bin.".to_string(),
                );
            };
            let asset = Id::new(&asset);
            if parameter(query, "cancel").is_some() {
                let waiting = viewer
                    .lock()
                    .expect("the viewer lock was poisoned")
                    .relink
                    .take();
                return Some(match waiting {
                    Some(_) => "The relink was dropped. The sequence still points at the \
                                drawings it did."
                        .to_string(),
                    None => "There is no relink waiting to be dropped.".to_string(),
                });
            }
            if parameter(query, "apply").is_some() {
                let waiting = {
                    let mut held = viewer.lock().expect("the viewer lock was poisoned");
                    match held.relink.as_ref().is_some_and(|p| p.asset == asset) {
                        true => held.relink.take(),
                        false => None,
                    }
                };
                let Some(waiting) = waiting else {
                    return Some(
                        "There is no relink waiting for that sequence. Choose the replacement \
                         drawings first, so that what would change can be read before it happens."
                            .to_string(),
                    );
                };
                return Some(edit(viewer, persist::relink_command(&waiting.candidate)));
            }
            let files: Vec<PathBuf> = parameters(query, "file")
                .into_iter()
                .map(PathBuf::from)
                .collect();
            return Some(match files.is_empty() {
                true => "Which drawings should the sequence point at? Choose the files \
                         themselves, not the folder they are in."
                    .to_string(),
                false => propose_relink(viewer, &asset, &files),
            });
        }
        // D-61's "Can be passed on" tick, for the drawing chosen in the Project panel.
        "asset.set_redistribute" => {
            let Some(asset) = parameter(query, "asset") else {
                return Some("Which drawings? Choose them in the Project panel.".to_string());
            };
            let redistribute = parameter(query, "redistribute").as_deref() != Some("false");
            return Some(edit(
                viewer,
                Command::SetAssetRedistribute {
                    asset: Id::new(&asset),
                    redistribute,
                },
            ));
        }
        // D-254: a composition's or a footage item's label, from the Project panel's right-click.
        "item.set_label" => {
            let Some(item) = parameter(query, "item") else {
                return Some("Which item? Right-click it in the Project panel.".to_string());
            };
            let Some(label) = parameter(query, "label").and_then(|n| n.parse::<u8>().ok()) else {
                return Some("Which colour? Send a label from 0, none, to 8.".to_string());
            };
            return Some(edit(
                viewer,
                Command::SetItemLabel {
                    item: Id::new(&item),
                    label,
                },
            ));
        }
        // W-01 step 3, which had no command until B-12d. Answered here rather than below
        // because everything below reads the composition on screen first, and this is the one
        // command whose whole purpose is that there may not be one worth working in yet.
        //
        // The five fields are document 19 line 15's own: name, width, height, frame rate, and a
        // length in frames. A start frame is not asked for. Document 19 has one and every
        // fixture in this build starts at 0; a field nobody in W-01 is told what to put in is a
        // field somebody puts the wrong thing in, and `SetExposureSpans` can move the work
        // afterwards if a shot ever needs it.
        // W-25: After Effects' Composition Settings, Ctrl+K, for the composition on screen. A
        // field that is not sent keeps what the composition has, so a rate of 30000/1001 is not
        // rounded to 30 by a form that only asked for the name.
        // D-188: the composition's shutter. A setting that is not sent keeps what it has, so the
        // timeline's switch sends `enabled` alone and Composition Settings the other three.
        "composition.set_motion_blur" => {
            let (id, mut shutter) = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                match held.document.project().composition(&held.composition) {
                    Some(comp) => (comp.id.clone(), comp.motion_blur),
                    None => {
                        return Some("There is no composition on screen to change.".to_string())
                    }
                }
            };
            match parameter(query, "enabled").as_deref() {
                None => {}
                Some("true") => shutter.enabled = true,
                Some("false") => shutter.enabled = false,
                Some(other) => return Some(format!("\"{other}\" is not on or off.")),
            }
            for (field, slot) in [
                ("angle", &mut shutter.shutter_angle),
                ("phase", &mut shutter.shutter_phase),
            ] {
                if let Some(text) = parameter(query, field) {
                    match text.trim().parse::<f64>() {
                        Ok(degrees) => *slot = degrees,
                        Err(_) => {
                            return Some(format!("\"{}\" is not a number of degrees.", text.trim()))
                        }
                    }
                }
            }
            if let Some(text) = parameter(query, "samples") {
                match text.trim().parse::<u32>() {
                    Ok(samples) => shutter.samples = samples,
                    Err(_) => {
                        return Some(format!(
                            "\"{}\" is not a whole number of samples.",
                            text.trim()
                        ))
                    }
                }
            }
            return Some(edit(
                viewer,
                Command::SetMotionBlur {
                    composition: id,
                    motion_blur: shutter,
                },
            ));
        }
        // D-216: the composition's frame-blending switch, off until it is turned on.
        "composition.set_frame_blending" => {
            let id = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                match held.document.project().composition(&held.composition) {
                    Some(comp) => comp.id.clone(),
                    None => {
                        return Some("There is no composition on screen to change.".to_string())
                    }
                }
            };
            let value = match parameter(query, "enabled").as_deref() {
                Some("true") => true,
                Some("false") => false,
                Some(other) => return Some(format!("\"{other}\" is not on or off.")),
                None => return Some("On or off? Send enabled=true or false.".to_string()),
            };
            return Some(edit(
                viewer,
                Command::SetFrameBlending {
                    composition: id,
                    value,
                },
            ));
        }
        // D-261: the Sketch workspace's layers and strokes, on the composition on screen. Never
        // drawn into a frame or an export; the page draws them over the picture for the eye.
        "sketch.set_layer"
        | "sketch.remove_layer"
        | "sketch.add_stroke"
        | "sketch.clear"
        | "sketch.set_look"
        | "sketch.move_layer"
        | "sketch.remove_strokes"
        | "sketch.move_strokes" => {
            let composition = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                match held.document.project().composition(&held.composition) {
                    Some(comp) => comp.id.clone(),
                    None => {
                        return Some("There is no composition on screen to change.".to_string())
                    }
                }
            };
            let Some(sketch) = parameter(query, "sketch").filter(|s| !s.is_empty()) else {
                return Some("Which sketch layer? Send sketch=<its id>.".to_string());
            };
            let sketch = anime_compositor::model::Id::new(sketch);
            let frame = match parameter(query, "frame") {
                None => None,
                Some(text) => match text.parse::<i32>() {
                    Ok(f) => Some(f),
                    Err(_) => return Some(format!("\"{text}\" is not a frame number.")),
                },
            };
            // D-282, D-284: strokes by their place in the layer's list, as 0,3,4.
            let strokes = || -> Result<Vec<usize>, String> {
                parameter(query, "strokes")
                    .unwrap_or_default()
                    .split(',')
                    .map(|i| {
                        i.trim()
                            .parse()
                            .map_err(|_| "Which strokes? Send strokes=0,3,4.".to_string())
                    })
                    .collect()
            };
            let number = |key: &str| -> Result<f64, String> {
                let text = parameter(query, key).unwrap_or_default();
                text.parse()
                    .map_err(|_| format!("\"{text}\" is not a number for {key}."))
            };
            let command = match id {
                "sketch.set_look" => match number("opacity") {
                    Ok(opacity) => Command::SetSketchLook {
                        composition,
                        sketch,
                        opacity,
                        locked: parameter(query, "locked").as_deref() == Some("true"),
                    },
                    Err(said) => return Some(said),
                },
                "sketch.move_layer" => match parameter(query, "to").and_then(|t| t.parse().ok()) {
                    Some(to) => Command::MoveSketchLayer {
                        composition,
                        sketch,
                        to,
                    },
                    None => {
                        return Some("Where to? Send to=<its place>, 0 the bottom.".to_string())
                    }
                },
                "sketch.remove_strokes" => match strokes() {
                    Ok(strokes) => Command::RemoveSketchStrokes {
                        composition,
                        sketch,
                        strokes,
                    },
                    Err(said) => return Some(said),
                },
                "sketch.move_strokes" => match (strokes(), number("dx"), number("dy")) {
                    (Ok(strokes), Ok(dx), Ok(dy)) => Command::MoveSketchStrokes {
                        composition,
                        sketch,
                        strokes,
                        by: [dx, dy],
                    },
                    (Err(said), _, _) | (_, Err(said), _) | (_, _, Err(said)) => return Some(said),
                },
                "sketch.set_layer" => Command::SetSketchLayer {
                    composition,
                    sketch,
                    name: parameter(query, "name").unwrap_or_default(),
                    visible: parameter(query, "visible").as_deref() != Some("false"),
                    whole_cut: parameter(query, "whole_cut").as_deref() == Some("true"),
                },
                "sketch.remove_layer" => Command::RemoveSketchLayer {
                    composition,
                    sketch,
                },
                "sketch.clear" => Command::ClearSketch {
                    composition,
                    sketch,
                    frame,
                },
                _ => {
                    // The page sends a stroke's points as x,y;x,y in composition pixels.
                    let points: Option<Vec<[f64; 2]>> = parameter(query, "points")
                        .unwrap_or_default()
                        .split(';')
                        .map(|pair| {
                            let (x, y) = pair.split_once(',')?;
                            Some([x.trim().parse().ok()?, y.trim().parse().ok()?])
                        })
                        .collect();
                    let Some(points) = points else {
                        return Some("The stroke's points are written x,y;x,y.".to_string());
                    };
                    // D-271: a pen's pressure at each point as p;p, none for a mouse.
                    let pressure: Option<Vec<f64>> = match parameter(query, "pressure") {
                        None => Some(Vec::new()),
                        Some(text) => text.split(';').map(|p| p.trim().parse().ok()).collect(),
                    };
                    let Some(pressure) = pressure else {
                        return Some(
                            "The stroke's pressure is written p;p, one per point.".to_string(),
                        );
                    };
                    Command::AddSketchStroke {
                        composition,
                        sketch,
                        stroke: anime_compositor::model::Stroke {
                            frame,
                            tool: parameter(query, "tool").unwrap_or_default(),
                            size: parameter(query, "size")
                                .and_then(|s| s.parse().ok())
                                .unwrap_or(f64::NAN),
                            colour: parameter(query, "colour").unwrap_or_default(),
                            points,
                            pressure,
                            // D-286, D-287.
                            pressure_opacity: parameter(query, "pressure_opacity").as_deref()
                                == Some("true"),
                            filled: parameter(query, "filled").as_deref() == Some("true"),
                            ..Default::default()
                        },
                    }
                }
            };
            return Some(edit(viewer, command));
        }
        "composition.set_settings" => {
            // D-253: By cut sets a status on any cut, so it may name one; otherwise the one on screen.
            let (comp, on_screen) = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                let wanted = parameter(query, "composition")
                    .map(|c| Id::new(&c))
                    .unwrap_or(held.composition.clone());
                match held.document.project().composition(&wanted) {
                    Some(comp) => (comp.clone(), wanted == held.composition),
                    None => {
                        return Some("There is no composition on screen to change.".to_string())
                    }
                }
            };
            let number = |field: &str, fallback: u32| -> Result<u32, String> {
                match parameter(query, field) {
                    None => Ok(fallback),
                    Some(text) => text.trim().parse::<u32>().map_err(|_| {
                        format!(
                            "\"{}\" is not a number of {field}. A composition needs whole \
                             numbers for its width, its height, its frame rate and its length.",
                            text.trim()
                        )
                    }),
                }
            };
            let (width, height, frames) = match (
                number("width", comp.width),
                number("height", comp.height),
                number("frames", comp.duration_frames),
            ) {
                (Ok(w), Ok(h), Ok(f)) => (w, h, f),
                (Err(said), _, _) | (_, Err(said), _) | (_, _, Err(said)) => return Some(said),
            };
            let frame_rate = match parameter(query, "fps") {
                None => comp.frame_rate,
                // D-290: 23.976 and the other broadcast rates are their exact 1001 fractions.
                Some(text) => match FrameRate::parse(&text) {
                    Some(rate) => rate,
                    None => {
                        return Some(format!(
                            "\"{}\" is not a frame rate. Send a number above nought, such as \
                             24, 23.976 or 29.97.",
                            text.trim()
                        ))
                    }
                },
            };
            // D-253: one of the five, or what the composition says now.
            let status = match parameter(query, "status") {
                None => comp.sheet_details.status.clone(),
                Some(s) if anime_compositor::model::CUT_STATUSES.contains(&s.as_str()) => s,
                Some(s) => {
                    return Some(format!(
                        "\"{s}\" is not a cut status. Send not_started, in_progress, check, retake or done."
                    ))
                }
            };
            // D-311: `none`, or three numbers from 0 to 1, linear; a page that does not send it
            // keeps what the composition has.
            let background_color = match parameter(query, "background_color")
                .as_deref()
                .map(str::trim)
            {
                None => comp.background_color,
                Some("none") => None,
                Some(text) => {
                    let n: Vec<f64> = text
                        .split(',')
                        .filter_map(|v| v.trim().parse().ok())
                        .collect();
                    match n[..] {
                        [r, g, b] => Some([r, g, b]),
                        _ => {
                            return Some(format!(
                                "\"{text}\" is not a background colour. Send three numbers from 0 to 1, or none."
                            ))
                        }
                    }
                }
            };
            // D-319: `true` for Float, `false` for Display; a page that does not send it keeps
            // what the composition has. D-330: `8bpc` for 8 bpc (After Effects). D-333: `ae32` for
            // 32 bpc (After Effects).
            let (float_depth, eight_bpc, ae_32bpc) = match parameter(query, "float_depth").as_deref().map(str::trim) {
                None => (comp.float_depth, comp.eight_bpc, comp.ae_32bpc),
                Some("true") => (true, false, false),
                Some("false") => (false, false, false),
                Some("8bpc") => (false, true, false),
                Some("ae32") => (true, false, true),
                Some(text) => return Some(format!("\"{text}\" is not a working depth. Send true for Float, false for Display, 8bpc for 8 bpc or ae32 for 32 bpc (After Effects).")),
            };
            let said = edit(
                viewer,
                Command::SetCompositionSettings {
                    composition: comp.id.clone(),
                    background_color,
                    float_depth,
                    eight_bpc,
                    ae_32bpc,
                    name: parameter(query, "name").unwrap_or(comp.name),
                    width,
                    height,
                    frame_rate,
                    duration_frames: frames,
                    // D-84f: a box the page does not send keeps what it says.
                    sheet_details: anime_compositor::model::SheetDetails {
                        episode: parameter(query, "episode").unwrap_or(comp.sheet_details.episode),
                        scene: parameter(query, "scene").unwrap_or(comp.sheet_details.scene),
                        cut: parameter(query, "cut").unwrap_or(comp.sheet_details.cut),
                        animator: parameter(query, "animator")
                            .unwrap_or(comp.sheet_details.animator),
                        status,
                    },
                },
            );
            // The clock is rebuilt from the new rate and length, as opening the composition does.
            if on_screen {
                show(viewer, &comp.id);
            }
            return Some(said);
        }
        // D-58's camera. One command for three properties, as document 24 has it: the
        // place is a pair of numbers and the depth and the zoom are one each.
        //
        // The zoom is sent as a lens in millimetres and stored in pixels. That is D-22's
        // rule about where a unit is converted, applied to a second number: a person says
        // 50 mm because that is what a lens is called, and the file keeps the pixels the
        // arithmetic actually uses, converted once, here.
        "camera.set_property" => {
            let comp = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                match held.document.project().composition(&held.composition) {
                    Some(comp) => comp.clone(),
                    None => {
                        return Some("There is no composition on screen to change.".to_string())
                    }
                }
            };
            let Some(name) = parameter(query, "property") else {
                return Some(
                    "Which of the camera's properties? Its position, its depth or its zoom."
                        .to_string(),
                );
            };
            let Some(prop) = anime_compositor::model::CameraProp::from_str(&name) else {
                return Some(format!(
                    "\"{name}\" is not one of the camera's properties. They are position, \
                     depth and zoom."
                ));
            };
            let mut numbers: Vec<f64> = Vec::new();
            if let Some(text) = parameter(query, "value") {
                for part in text.split(',') {
                    match part.trim().parse::<f64>() {
                        Ok(number) => numbers.push(number),
                        Err(_) => return Some(format!("\"{}\" is not a number.", part.trim())),
                    }
                }
            }
            use anime_compositor::model::{CameraProp, Value};
            let value = match (prop, numbers.as_slice()) {
                (CameraProp::Position, [x, y]) => Value::Vec2(*x, *y),
                (CameraProp::Depth, [d]) => Value::Scalar(*d),
                (CameraProp::Zoom, [mm]) => Value::Scalar(comp.width as f64 * mm / 36.0),
                (CameraProp::Position, _) => {
                    return Some(
                        "The camera's position is two numbers, across and down: \
                         value=960,540."
                            .to_string(),
                    )
                }
                (CameraProp::Depth, _) => {
                    return Some(
                        "The camera's depth is one number, in pixels: value=-2666.67.".to_string(),
                    )
                }
                (CameraProp::Zoom, _) => {
                    return Some(
                        "The camera's zoom is one number, a lens in millimetres on a 36 mm \
                         back: value=50."
                            .to_string(),
                    )
                }
            };
            return Some(edit(
                viewer,
                Command::SetCameraProperty {
                    composition: comp.id.clone(),
                    prop,
                    value,
                },
            ));
        }
        // W-25: Ctrl+C on layers. The layers are kept as they are, keys, effects and all, in the
        // window rather than the file, and nothing is added to the history.
        "layer.copy" => {
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            let ids = parameters(query, "layer");
            let taken: Vec<Layer> = held
                .document
                .project()
                .composition(&held.composition)
                .map(|comp| {
                    comp.layers_in_order()
                        .filter(|l| ids.iter().any(|id| id == l.id.as_str()))
                        .cloned()
                        .collect()
                })
                .unwrap_or_default();
            let said = match taken.len() {
                0 => return Some("Which layers? Choose them in the layer list.".to_string()),
                1 => format!("Copied {}.", taken[0].name),
                n => format!("Copied {n} layers."),
            };
            held.clipboard = taken;
            return Some(said);
        }
        // W-25: Ctrl+V on layers. The copies go in front, in the order they were in, under new
        // identifiers, as one entry to undo. A matte on a layer copied with them follows the
        // copy; a matte on a layer this composition does not have is let go. The mattes are set
        // after every copy is in, because a copy's matte may be a copy further up.
        "layer.paste" => {
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            if held.clipboard.is_empty() {
                return Some("Nothing is copied. Choose layers and press Ctrl+C.".to_string());
            }
            let composition = held.composition.clone();
            let project = held.document.project();
            let Some(comp) = project.composition(&composition) else {
                return Some("There is no composition on screen to paste into.".to_string());
            };
            let first = unused_layer_id(project).as_str()["layer-".len()..]
                .parse::<u64>()
                .unwrap_or(1);
            let new_id = |at: usize| Id::new(format!("layer-{}", first + at as u64));
            let mut commands = Vec::new();
            let mut mattes = Vec::new();
            // D-57: a parent travels with a copy exactly as a matte does, and for the same
            // reason -- a copy of a chain is a chain. `keep_place` is false here alone: the
            // copy is made in the space it is being put into, so converting its values would
            // move it away from the layer it was copied from. The frame is not read.
            let mut parents = Vec::new();
            for (at, layer) in held.clipboard.iter().enumerate() {
                if let Some(parent) = &layer.parent {
                    let copied = held.clipboard.iter().position(|l| l.id == *parent);
                    let target = match copied {
                        Some(other) => Some(new_id(other)),
                        None => comp.layer(parent).map(|l| l.id.clone()),
                    };
                    if let Some(target) = target {
                        parents.push(Command::SetParent {
                            composition: composition.clone(),
                            layer_id: new_id(at),
                            parent: Some(target),
                            frame: 0,
                            keep_place: false,
                        });
                    }
                }
                if let Some(matte) = &layer.matte {
                    let copied = held.clipboard.iter().position(|l| l.id == matte.layer_id);
                    let target = match copied {
                        Some(other) => Some(new_id(other)),
                        None => comp.layer(&matte.layer_id).map(|l| l.id.clone()),
                    };
                    if let Some(target) = target {
                        mattes.push(Command::SetMatte {
                            composition: composition.clone(),
                            layer_id: new_id(at),
                            matte: Some(target),
                            matte_only: matte.matte_only,
                        });
                        // D-293: and its mode, which a new matte starts without.
                        if matte.mode != MatteMode::Alpha {
                            mattes.push(Command::SetMatteMode {
                                composition: composition.clone(),
                                layer_id: new_id(at),
                                mode: matte.mode,
                            });
                        }
                    }
                }
                commands.push(Command::AddLayer {
                    composition: composition.clone(),
                    layer: Box::new(Layer {
                        id: new_id(at),
                        matte: None,
                        parent: None,
                        ..layer.clone()
                    }),
                    index: comp.len() + at,
                });
            }
            commands.extend(mattes);
            commands.extend(parents);
            return Some(match held.document.apply_all(commands) {
                Ok(record) => record.label.clone(),
                Err(diagnostic) => diagnostic.sentence(),
            });
        }
        // W-26: a composition deleted from the project panel, one entry to undo. The core keeps
        // the last one. Deleting the one on screen moves the window to the first that is left.
        "composition.delete" => {
            let Some(asked) = parameter(query, "composition") else {
                return Some(
                    "Which composition should be deleted? Choose one in the project panel."
                        .to_string(),
                );
            };
            let id = Id::new(&asked);
            let name = viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .document
                .project()
                .composition(&id)
                .map(|comp| comp.name.clone());
            let Some(name) = name else {
                return Some(format!("There is no composition {asked} in this project."));
            };
            let said = edit(
                viewer,
                Command::RemoveComposition {
                    composition: id.clone(),
                },
            );
            let gone = viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .document
                .project()
                .composition(&id)
                .is_none();
            settle(viewer, &[]);
            return Some(match gone {
                true => format!("{name} is deleted. Ctrl+Z brings it back."),
                false => said,
            });
        }
        // W-26: a copy of a composition, as one entry to undo, and the window moves into it. Its
        // layers get new identifiers and a matte follows the copy of its layer. A lock goes back on
        // last, because a locked layer refuses its matte.
        "composition.duplicate" => {
            let Some(asked) = parameter(query, "composition") else {
                return Some(
                    "Which composition should be duplicated? Choose one in the project panel."
                        .to_string(),
                );
            };
            let made = {
                let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
                let project = held.document.project();
                let Some(comp) = project.composition(&Id::new(&asked)) else {
                    return Some(format!("There is no composition {asked} in this project."));
                };
                let copy = unused_composition_id(project);
                let name = format!("{} copy", comp.name);
                let mut empty = Composition::new(
                    copy.clone(),
                    name.clone(),
                    comp.width,
                    comp.height,
                    comp.frame_rate,
                    comp.start_frame,
                    comp.duration_frames,
                );
                empty.work_area = comp.work_area;
                empty.markers = comp.markers.clone();
                let first = unused_layer_id(project).as_str()["layer-".len()..]
                    .parse::<u64>()
                    .unwrap_or(1);
                let new_id = |at: usize| Id::new(format!("layer-{}", first + at as u64));
                let layers: Vec<&Layer> = comp.layers_in_order().collect();
                let mut commands = vec![Command::AddComposition {
                    composition: Box::new(empty),
                }];
                let mut mattes = Vec::new();
                // D-57: as with a matte, a parent is remapped onto the copy of the layer it
                // named. A parent outside this composition cannot exist, so there is no second
                // case to handle here.
                let mut parents = Vec::new();
                let mut locks = Vec::new();
                for (at, layer) in layers.iter().enumerate() {
                    let mut copied = Layer {
                        id: new_id(at),
                        matte: None,
                        parent: None,
                        locked: false,
                        ..(*layer).clone()
                    };
                    // D-191: a layer setting names the copy of the layer it named, as a matte
                    // does; one naming no layer here is kept as written.
                    for instance in &mut copied.effects {
                        for (named, _) in instance.effect.layer_settings_mut() {
                            if let Some(other) = layers
                                .iter()
                                .position(|l| named.as_str() == Some(l.id.as_str()))
                            {
                                *named = new_id(other).as_str().into();
                            }
                        }
                    }
                    commands.push(Command::AddLayer {
                        composition: copy.clone(),
                        layer: Box::new(copied),
                        index: at,
                    });
                    if let Some(matte) = &layer.matte {
                        if let Some(other) = layers.iter().position(|l| l.id == matte.layer_id) {
                            mattes.push(Command::SetMatte {
                                composition: copy.clone(),
                                layer_id: new_id(at),
                                matte: Some(new_id(other)),
                                matte_only: matte.matte_only,
                            });
                            if matte.mode != MatteMode::Alpha {
                                mattes.push(Command::SetMatteMode {
                                    composition: copy.clone(),
                                    layer_id: new_id(at),
                                    mode: matte.mode,
                                });
                            }
                        }
                    }
                    if let Some(parent) = &layer.parent {
                        if let Some(other) = layers.iter().position(|l| l.id == *parent) {
                            parents.push(Command::SetParent {
                                composition: copy.clone(),
                                layer_id: new_id(at),
                                parent: Some(new_id(other)),
                                frame: 0,
                                keep_place: false,
                            });
                        }
                    }
                    if layer.locked {
                        locks.push(Command::SetLayerLocked {
                            composition: copy.clone(),
                            layer_id: new_id(at),
                            value: true,
                        });
                    }
                }
                commands.extend(mattes);
                commands.extend(parents);
                commands.extend(locks);
                match held.document.apply_all(commands) {
                    Ok(_) => Ok((copy, name)),
                    Err(diagnostic) => Err(diagnostic.sentence()),
                }
            };
            return Some(match made {
                Ok((copy, name)) => {
                    show(viewer, &copy);
                    format!("{name} is on screen. Ctrl+Z takes it away again.")
                }
                Err(said) => said,
            });
        }
        "composition.create" => {
            let name = parameter(query, "name").unwrap_or_else(|| "New composition".to_string());
            let number = |field: &str, fallback: u32| -> Result<u32, String> {
                match parameter(query, field) {
                    None => Ok(fallback),
                    Some(text) => text.trim().parse::<u32>().map_err(|_| {
                        format!(
                            "\"{}\" is not a number of {field}. A composition needs whole \
                             numbers for its width, its height, its frame rate and its length.",
                            text.trim()
                        )
                    }),
                }
            };
            let (width, height, frames) = match (
                number("width", 1920),
                number("height", 1080),
                number("frames", 240),
            ) {
                (Ok(w), Ok(h), Ok(f)) => (w, h, f),
                (Err(said), _, _) | (_, Err(said), _) | (_, _, Err(said)) => return Some(said),
            };
            // The rate goes through the core's own reader, so a zero is refused by the rule that
            // owns it rather than by a second copy of that rule written here. D-290: 23.976 and
            // the other broadcast rates are their exact 1001 fractions.
            let fps = parameter(query, "fps").unwrap_or_else(|| "24".to_string());
            let Some(frame_rate) = FrameRate::parse(&fps) else {
                return Some(if fps.trim().parse::<f64>() == Ok(0.0) {
                    "A frame rate of zero is not a frame rate. Twenty-four is the one the \
                     reference shot uses."
                        .to_string()
                } else {
                    format!(
                        "\"{}\" is not a frame rate. Send a number above nought, such as 24, \
                         23.976 or 29.97.",
                        fps.trim()
                    )
                });
            };
            let composition = {
                let held = viewer.lock().expect("the viewer lock was poisoned");
                Composition::new(
                    unused_composition_id(held.document.project()),
                    name,
                    width,
                    height,
                    frame_rate,
                    0,
                    frames,
                )
            };
            let id = composition.id.clone();
            let said = edit(
                viewer,
                Command::AddComposition {
                    composition: Box::new(composition),
                },
            );
            // Only if the core took it. A refused command must leave the window looking at what
            // it was looking at, and `show` would otherwise move it to a composition that is
            // not in the project.
            let made = viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .document
                .project()
                .composition(&id)
                .is_some();
            if made {
                show(viewer, &id);
                return Some(format!(
                    "{said}, {width}x{height} at {} fps, {frames} frames. It is empty; \
                     import drawings and add layers to fill it.",
                    frame_rate.label()
                ));
            }
            return Some(said);
        }
        // Which composition the window is looking at. Not an edit, for the same reason the two
        // inspection toggles above are not: nothing about the project changes, the window looks
        // somewhere else. It is also the way back out of a composition that has just been made,
        // and without it `composition.create` is a one-way door - the owner found that the first
        // time they used it, with no way to return to the shot they had been working on.
        "composition.open" => {
            let Some(asked) = parameter(query, "composition") else {
                return Some(
                    "Which composition should be opened? Choose one in the project panel."
                        .to_string(),
                );
            };
            let id = Id::new(&asked);
            let named = viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .document
                .project()
                .composition(&id)
                .map(|comp| comp.name.clone());
            return Some(match named {
                None => format!("There is no composition {asked} in this project."),
                Some(name) => {
                    show(viewer, &id);
                    format!("{name} is on screen.")
                }
            });
        }
        "media.import" => {
            let files: Vec<PathBuf> = parameters(query, "file")
                .into_iter()
                .map(PathBuf::from)
                .collect();
            return Some(match files.is_empty() {
                true => "Which drawings should be imported? Choose the files themselves, not \
                         the folder they are in."
                    .to_string(),
                false => import(viewer, &files),
            });
        }
        "timesheet.import" => {
            return Some(match parameter(query, "folder") {
                None => "Which cut? Choose the folder that holds its .xdts timesheet.".to_string(),
                Some(folder) => import_cut(viewer, Path::new(&folder)),
            });
        }
        // W-22: After Effects' Ctrl+Shift+D. The layer ends on the frame before the one asked for
        // and a copy of it in front begins there, each keeping the drawing and the keys under its
        // frames, as one entry to undo.
        "layer.split" => {
            let Some(layer_id) = parameter(query, "layer").map(Id::new) else {
                return Some("Which layer? Choose one in the layer list.".to_string());
            };
            let frame = match frame_parameter(query, "frame") {
                Ok(frame) => frame,
                Err(said) => return Some(said),
            };
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            let composition = held.composition.clone();
            let project = held.document.project();
            let Some(comp) = project.composition(&composition) else {
                return Some("There is no composition on screen to edit.".to_string());
            };
            let Some(layer) = comp.layer(&layer_id) else {
                return Some(format!("{layer_id} is not a layer in this composition."));
            };
            if frame <= layer.in_frame || frame >= layer.out_frame {
                return Some(format!(
                    "{} runs from frame {} to {}, so it can only be split on a frame after its \
                     first and before its end.",
                    layer.name,
                    layer.in_frame,
                    layer.out_frame - 1
                ));
            }
            let at = comp
                .index_of(&layer_id)
                .expect("a layer that is here has a place");
            let copy = copy_of(project, layer);
            let (copy_id, in_frame, out_frame) = (copy.id.clone(), layer.in_frame, layer.out_frame);
            let commands = vec![
                Command::AddLayer {
                    composition: composition.clone(),
                    layer: Box::new(copy),
                    index: at + 1,
                },
                Command::TrimLayer {
                    composition: composition.clone(),
                    layer_id,
                    in_frame,
                    out_frame: frame,
                },
                Command::TrimLayer {
                    composition,
                    layer_id: copy_id,
                    in_frame: frame,
                    out_frame,
                },
            ];
            return Some(match held.document.apply_all(commands) {
                Ok(record) => record.label.clone(),
                Err(diagnostic) => diagnostic.sentence(),
            });
        }
        // D-314: After Effects' Time-Reverse Keyframes, on the chosen keys, as one entry to undo.
        // Inside the span from the first chosen key to the last, a key shown on frame f goes to
        // first + last - f with its value, and each stretch between two of a row's chosen keys
        // is the stretch it mirrors played backwards, so it takes that one's ease turned round
        // and a position key's two path handles change places. A key not chosen never moves.
        // ponytail: D-69's kind and roving are not carried over; a reversed key is a plain one.
        "keyframe.reverse" => {
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            let composition = held.composition.clone();
            let Some(comp) = held.document.project().composition(&composition) else {
                return Some("There is no composition on screen to edit.".to_string());
            };
            let mut rows: std::collections::BTreeMap<(String, String), Vec<f64>> =
                Default::default();
            for named in parameters(query, "key") {
                let mut parts = named.rsplitn(3, '|');
                let (Some(Ok(at)), Some(prop), Some(layer)) = (
                    parts.next().map(str::parse::<f64>),
                    parts.next(),
                    parts.next(),
                ) else {
                    return Some(format!("A key is layer|property|frame. Not \"{named}\"."));
                };
                rows.entry((layer.to_string(), prop.to_string()))
                    .or_default()
                    .push(at);
            }
            let shown: Vec<f64> = rows.values().flatten().copied().collect();
            let lo = shown.iter().copied().fold(f64::MAX, f64::min);
            let hi = shown.iter().copied().fold(f64::MIN, f64::max);
            if shown.len() < 2 || lo == hi {
                return Some("Choose two or more keys on different frames to reverse.".to_string());
            }
            // A chosen key's stored frame and the one it goes to, by B-150c's rule.
            let place =
                |layer: Option<&Layer>, f: f64| match layer.filter(|l| l.time_stretch != 100.0) {
                    Some(l) => (
                        l.key_time(f).round() as i32,
                        l.key_time(lo + hi - f).round() as i32,
                    ),
                    None => (f.round() as i32, (lo + hi - f).round() as i32),
                };
            let (mut removes, mut sets) = (Vec::new(), Vec::new());
            // D-741: a layer's paths' and masks', shapes' and groups' numbers' keys, by layer.
            let mut owned: std::collections::BTreeMap<
                String,
                std::collections::BTreeMap<String, Vec<(i32, i32)>>,
            > = Default::default();
            for ((named, prop), frames) in rows {
                let layer = comp.layer(&Id::new(&named));
                if layer.is_none() && named != CAMERA_ROW {
                    return Some(format!("There is no layer {named} here."));
                }
                let mut moves: Vec<(i32, i32)> = frames.iter().map(|&f| place(layer, f)).collect();
                moves.sort();
                moves.dedup();
                if layer.is_some() && layer_keyed(&prop) {
                    owned.entry(named).or_default().insert(prop, moves);
                    continue;
                }
                if let (Some((instance, setting)), Some(layer)) = (effect_setting(&prop), layer) {
                    let Some(existing) = layer.effects.iter().find(|e| e.instance_id == instance)
                    else {
                        return Some(format!("{instance} is not an effect on {named}."));
                    };
                    let mut keys = existing.keys(&setting);
                    let new = match reversed(
                        &keys.iter().map(|k| (k.frame, k.interp)).collect::<Vec<_>>(),
                        &moves,
                    ) {
                        Ok(new) => new,
                        Err(said) => return Some(said),
                    };
                    for (k, (frame, interp)) in keys.iter_mut().zip(new) {
                        (k.frame, k.interp) = (frame, interp);
                    }
                    keys.sort_by_key(|k| k.frame);
                    sets.push(Command::SetEffectKeys {
                        composition: composition.clone(),
                        layer_id: layer.id.clone(),
                        instance_id: instance,
                        setting,
                        keys,
                    });
                    continue;
                }
                let (target, chosen, property) = match layer {
                    None => {
                        let Some(which) = anime_compositor::model::CameraProp::from_str(&prop)
                        else {
                            return Some(format!("The camera has no {prop}."));
                        };
                        let camera = comp.camera.clone().unwrap_or_else(|| {
                            anime_compositor::model::Camera::default_for(comp.width, comp.height)
                        });
                        (
                            Target::Camera,
                            prop_of_camera(which),
                            camera.get(which).clone(),
                        )
                    }
                    Some(layer) => match property(&prop) {
                        Some(chosen) => (
                            Target::Layer(layer.id.clone()),
                            chosen,
                            property_of(layer, chosen),
                        ),
                        None => return Some(format!("{} has no {prop}.", layer.name)),
                    },
                };
                let keys = property.keyframes();
                let new = match reversed(
                    &keys.iter().map(|k| (k.frame, k.interp)).collect::<Vec<_>>(),
                    &moves,
                ) {
                    Ok(new) => new,
                    Err(said) => return Some(said),
                };
                for (key, (frame, interp)) in keys.iter().zip(new) {
                    if !moves.iter().any(|(at, _)| *at == key.frame) {
                        continue;
                    }
                    removes.push(Command::RemoveKeyframe {
                        composition: composition.clone(),
                        target: target.clone(),
                        prop: chosen,
                        frame: key.frame,
                    });
                    sets.push(Command::SetKeyframe {
                        composition: composition.clone(),
                        target: target.clone(),
                        prop: chosen,
                        frame,
                        value: key.value,
                        interp,
                        spatial: key.spatial.map(|[ix, iy, ox, oy]| [ox, oy, ix, iy]),
                    });
                }
            }
            for (named, chosen) in owned {
                let layer = comp.layer(&Id::new(&named)).expect("found above");
                match layer_keys_commands(id, &composition, layer, &chosen) {
                    Ok(commands) => sets.extend(commands),
                    Err(said) => return Some(said),
                }
            }
            return Some(
                match held
                    .document
                    .apply_all(removes.into_iter().chain(sets).collect())
                {
                    Ok(record) => record.label.clone(),
                    Err(diagnostic) => diagnostic.sentence(),
                },
            );
        }
        // W-17: every chosen key moved by the same distance, as one entry to undo. Each is named
        // `layer|prop|frame`, split from the right because a frame and a property name cannot
        // hold a `|` and a layer id might. The keys furthest along the way they travel go first,
        // so no key is put down on a frame a neighbour moving with it has not left yet; a key
        // landing on one that is not moving is refused by the core, and then none of them move.
        "keyframe.move" | "keyframe.add_remove" if !parameters(query, "key").is_empty() => {
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            let composition = held.composition.clone();
            let by = if id == "keyframe.move" {
                match frame_parameter(query, "by") {
                    Ok(by) => by,
                    Err(said) => return Some(said),
                }
            } else {
                0
            };
            // B-150c: each key is named by the frame it is shown on, and on a stretched layer
            // it is stored, and lands, where document 20 puts it: `(at, to)` below.
            let comp = held.document.project().composition(&composition);
            let stored = |layer: &str, at: &str| {
                stored_frames(comp.and_then(|c| c.layer(&Id::new(layer))), at, by)
            };
            let mut keys = Vec::new();
            // B-19d: the chosen keys that are an effect setting's, by layer, effect and setting,
            // each with the frame it is on and the value sent with it. `key_values` is the
            // values of the others, which `value=` no longer lines up with once these are out.
            let mut fx: std::collections::BTreeMap<
                (String, String, String),
                Vec<((i32, i32), Option<String>)>,
            > = Default::default();
            // D-741: a layer's paths' and masks', shapes' and groups' numbers' keys, by layer.
            let mut owned: std::collections::BTreeMap<
                String,
                std::collections::BTreeMap<String, Vec<(i32, i32)>>,
            > = Default::default();
            let all_values = parameters(query, "value");
            let mut key_values = Vec::new();
            let named_count = parameters(query, "key").len();
            for (i, named) in parameters(query, "key").into_iter().enumerate() {
                let mut parts = named.rsplitn(3, '|');
                let (Some(at), Some(prop), Some(layer)) =
                    (parts.next(), parts.next(), parts.next())
                else {
                    return Some(format!("A key is layer|property|frame. Not \"{named}\"."));
                };
                // B-24f: a path's keys, by layer and path, and since D-741 a mask's, shape's and
                // group's numbers'. A value sent with one is not read: the graph draws no curve
                // for them, and a path's is its travelled distance, not something it can hold.
                if let (true, Some(at)) = (layer_keyed(prop), stored(layer, at)) {
                    owned
                        .entry(layer.to_string())
                        .or_default()
                        .entry(prop.to_string())
                        .or_default()
                        .push(at);
                    continue;
                }
                if let (Some((instance, setting)), Some(at)) =
                    (effect_setting(prop), stored(layer, at))
                {
                    fx.entry((layer.to_string(), instance.as_str().to_string(), setting))
                        .or_default()
                        .push((at, all_values.get(i).cloned()));
                    continue;
                }
                key_values.extend(all_values.get(i).cloned());
                // B-13e: the camera's three rows carry keys on the timeline like a layer's,
                // and the page names them with the same three parts. Only the first part tells
                // them apart, and a camera has its own three property names rather than the six
                // a layer answers to.
                let chosen = if layer == CAMERA_ROW {
                    anime_compositor::model::CameraProp::from_str(prop).map(prop_of_camera)
                } else {
                    property(prop)
                };
                let (Some(prop), Some((at, to))) = (chosen, stored(layer, at)) else {
                    return Some(format!("A key is layer|property|frame. Not \"{named}\"."));
                };
                let target = if layer == CAMERA_ROW {
                    Target::Camera
                } else {
                    Target::Layer(Id::new(layer))
                };
                keys.push((target, prop, at, to));
            }
            // B-19d: each setting's keys, with the chosen ones moved, given their values or
            // taken out, as the one command the core has for them.
            let mut fx_commands = Vec::new();
            for ((layer, instance, setting), chosen) in fx {
                let existing = held
                    .document
                    .project()
                    .composition(&composition)
                    .and_then(|c| c.layer(&Id::new(&layer)))
                    .and_then(|l| {
                        l.effects
                            .iter()
                            .find(|e| e.instance_id.as_str() == instance)
                    });
                let Some(existing) = existing else {
                    return Some(format!("{instance} is not an effect on {layer}."));
                };
                let original = existing.keys(&setting);
                let mut list = original.clone();
                for ((at, to), text) in &chosen {
                    let Some(i) = original.iter().position(|k| k.frame == *at) else {
                        return Some(format!("{setting} has no key at frame {at}."));
                    };
                    list[i].frame = *to;
                    if let Some(text) = text {
                        match setting_value(&setting, list[i].value.len(), text) {
                            Ok(value) => list[i].value = value,
                            Err(said) => return Some(said),
                        }
                    }
                }
                if id == "keyframe.add_remove" {
                    list.retain(|k| !chosen.iter().any(|((at, _), _)| *at == k.frame));
                } else if chosen
                    .iter()
                    .all(|((at, to), text)| at == to && text.is_none())
                {
                    continue;
                }
                list.sort_by_key(|k| k.frame);
                fx_commands.push(Command::SetEffectKeys {
                    composition: composition.clone(),
                    layer_id: Id::new(&layer),
                    instance_id: Id::new(&instance),
                    setting,
                    keys: list,
                });
            }
            // B-24f, D-741: each layer's chosen keys of its paths and numbers moved by the same
            // distance or taken out, as one command for its masks and one for its shapes.
            for (named, chosen) in owned {
                let removing = id == "keyframe.add_remove";
                if !removing && chosen.values().flatten().all(|(at, to)| at == to) {
                    continue;
                }
                let Some(layer) = held
                    .document
                    .project()
                    .composition(&composition)
                    .and_then(|c| c.layer(&Id::new(&named)))
                else {
                    return Some(format!("There is no layer {named} here."));
                };
                match layer_keys_commands(id, &composition, layer, &chosen) {
                    Ok(commands) => fx_commands.extend(commands),
                    Err(said) => return Some(said),
                }
            }
            // W-19: the chosen keys named are removed, as one entry to undo. Only removed: a key
            // somebody chose is a key that is there, and one that has gone since is refused by
            // the core rather than put back.
            if id == "keyframe.add_remove" {
                let commands = keys
                    .into_iter()
                    .map(|(target, prop, frame, _)| Command::RemoveKeyframe {
                        composition: composition.clone(),
                        target,
                        prop,
                        frame,
                    })
                    .chain(fx_commands)
                    .collect();
                return Some(match held.document.apply_all(commands) {
                    Ok(record) => record.label.clone(),
                    Err(diagnostic) => diagnostic.sentence(),
                });
            }
            // B-19a: a key dragged in the graph goes up and down as well as along, and one drag
            // is one entry to undo. `value=` is repeated once for each `key=`, in the same order,
            // and is what that key holds where it lands; its ease and its path handles go with
            // it. `by=0` is a key whose value changed and whose frame did not.
            let values = key_values;
            if !all_values.is_empty() && all_values.len() != named_count {
                return Some(format!(
                    "{} keys were named and {} values: a value is given for every key or for none.",
                    named_count,
                    all_values.len()
                ));
            }
            let mut sets = Vec::new();
            if let Some(comp) = held.document.project().composition(&composition) {
                for ((target, prop, at, to), text) in keys.iter().zip(&values) {
                    let property = match target {
                        Target::Camera => {
                            let which = match prop {
                                Prop::Depth => anime_compositor::model::CameraProp::Depth,
                                Prop::Zoom => anime_compositor::model::CameraProp::Zoom,
                                _ => anime_compositor::model::CameraProp::Position,
                            };
                            comp.camera
                                .clone()
                                .unwrap_or_else(|| {
                                    anime_compositor::model::Camera::default_for(
                                        comp.width,
                                        comp.height,
                                    )
                                })
                                .get(which)
                                .clone()
                        }
                        Target::Layer(id) => match comp.layer(id) {
                            Some(layer) => property_of(layer, *prop),
                            None => return Some(format!("There is no layer {id} here.")),
                        },
                    };
                    let (Some(key), Some(value)) =
                        (property.keyframe_at(*at), property_value(*prop, text))
                    else {
                        return Some(format!(
                            "{prop} has no key at frame {at}, or \"{text}\" is not a value for it."
                        ));
                    };
                    sets.push(Command::SetKeyframe {
                        composition: composition.clone(),
                        target: target.clone(),
                        prop: *prop,
                        frame: *to,
                        value,
                        interp: key.interp,
                        spatial: key.spatial,
                    });
                }
            }
            keys.sort_by_key(|key| if by > 0 { -key.2 } else { key.2 });
            let commands: Vec<Command> = keys
                .into_iter()
                .filter(|(_, _, at, to)| at != to)
                .map(
                    |(target, prop, from_frame, to_frame)| Command::MoveKeyframe {
                        composition: composition.clone(),
                        target,
                        prop,
                        from_frame,
                        to_frame,
                    },
                )
                .chain(sets)
                .chain(fx_commands)
                .collect();
            if commands.is_empty() {
                return Some("Nothing moved.".to_string());
            }
            return Some(match held.document.apply_all(commands) {
                Ok(record) => record.label.clone(),
                Err(diagnostic) => diagnostic.sentence(),
            });
        }
        // B-18c: D-67's Pre-compose. The core says what it is made of and refuses a choice that
        // would cut a parent or a matte in two; the window picks the identifiers and the name,
        // Precomp 1, Precomp 2 and so on, the first no composition has.
        "layer.precompose" => {
            let chosen: Vec<Id> = parameters(query, "layer")
                .into_iter()
                .map(Id::new)
                .collect();
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            let project = held.document.project();
            // D-273: the name the Pre-compose window asked for, or the next "Precomp N".
            let name = parameter(query, "name")
                .map(|n| n.trim().to_string())
                .filter(|n| !n.is_empty())
                .unwrap_or_else(|| {
                    (1..)
                        .map(|n| format!("Precomp {n}"))
                        .find(|name| project.compositions.iter().all(|c| &c.name != name))
                        .expect("the numbers do not run out")
                });
            let commands = match anime_compositor::command::precompose(
                project,
                &held.composition,
                &chosen,
                unused_composition_id(project),
                unused_layer_id(project),
                &name,
            ) {
                Ok(commands) => commands,
                Err(diagnostic) => return Some(diagnostic.sentence()),
            };
            return Some(match held.document.apply_all(commands) {
                Ok(_) => format!(
                    "{name} holds the chosen layers now, and one layer shows it here. \
                     Double-click that layer to open it. Ctrl+Z puts them back."
                ),
                Err(diagnostic) => diagnostic.sentence(),
            });
        }
        _ => {}
    }
    // A drawing number an exposure names that its sequence has not got, filled in by the arm
    // that can see both and said at the end, once the core has accepted the change.
    let mut absent: Option<u32> = None;
    // D-57: whether the parent just chosen could keep the layer exactly where it was. Read
    // before the change, because it is a question about the chain the layer is in now.
    let mut inexact = false;
    // D-57: a layer being deleted lets its children go where they stand. Filled in by the
    // delete arm, which can see the composition, and applied with the deletion as one entry.
    let mut unparent: Vec<Command> = Vec::new();
    let command = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let composition = held.composition.clone();
        let project = held.document.project();
        let Some(comp) = project.composition(&composition) else {
            return Some("There is no composition on screen to edit.".to_string());
        };
        // W-24: the composition's work area and markers, which name no layer. B sets the start at
        // a frame and keeps the end, unless the end would then come first, when the end goes back
        // to the composition's; N is the same the other way round. Markers are the whole list,
        // `marker=frame|name` repeated, so a drag sends where every marker now is.
        if id.starts_with("timeline.") {
            let past = comp.start_frame + comp.duration_frames as i32;
            let (start, end) = comp.work_area.unwrap_or((comp.start_frame, past));
            match id {
                "timeline.set_work_start" => {
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    Command::SetWorkArea {
                        composition,
                        start_frame: frame,
                        end_frame_exclusive: if frame >= end { past } else { end },
                    }
                }
                "timeline.set_work_end" => {
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    Command::SetWorkArea {
                        composition,
                        start_frame: if start > frame {
                            comp.start_frame
                        } else {
                            start
                        },
                        end_frame_exclusive: frame + 1,
                    }
                }
                _ => {
                    let mut markers = Vec::new();
                    for text in parameters(query, "marker") {
                        let (at, name) = text.split_once('|').unwrap_or((text.as_str(), ""));
                        let Ok(frame) = at.trim().parse::<i32>() else {
                            return Some(format!("A marker is frame|name. Not \"{text}\"."));
                        };
                        markers.push(Marker {
                            frame,
                            name: name.to_string(),
                        });
                    }
                    Command::SetMarkers {
                        composition,
                        markers,
                    }
                }
            }
        } else if id == "sheet.write_action" {
            // D-84g: a note in the Action column sits on its one frame. Written on a note, it
            // replaces it; erased (no `text`), it goes, and the column with it when it was the
            // last. The whole list of text columns is one `SetSheetText`, one entry to undo.
            let frame = match frame_parameter(query, "frame") {
                Ok(frame) => frame,
                Err(said) => return Some(said),
            };
            let at = frame - comp.start_frame;
            if at < 0 || at >= comp.duration_frames as i32 {
                return Some(format!("Frame {frame} is outside {}.", comp.name));
            }
            let mut columns = comp.sheet_text.clone();
            let place = match columns.iter().position(|c| c.kind == "action") {
                Some(place) => place,
                None => {
                    columns.insert(
                        0,
                        anime_compositor::model::SheetText {
                            kind: "action".into(),
                            name: "Action".into(),
                            track: 0,
                            entries: Vec::new(),
                        },
                    );
                    0
                }
            };
            let entries = &mut columns[place].entries;
            let was = entries
                .iter()
                .position(|e| e.start_frame <= at && at < e.end_frame_exclusive);
            match parameter(query, "text") {
                None => {
                    let Some(was) = was else {
                        return Some(format!(
                            "Nothing is written in Action on frame {frame} to erase."
                        ));
                    };
                    entries.remove(was);
                }
                Some(text) => {
                    let text = text.trim().to_string();
                    if text.is_empty() {
                        return Some(
                            "An Action note needs words: spaces alone write nothing.".to_string(),
                        );
                    }
                    if let Some(was) = was {
                        if entries[was].text.join(" ") == text {
                            return Some(format!("Action on frame {frame} already says {text}."));
                        }
                        entries.remove(was);
                    }
                    entries.push(anime_compositor::model::SheetTextEntry {
                        start_frame: at,
                        end_frame_exclusive: at + 1,
                        text: vec![text],
                    });
                    entries.sort_by_key(|e| e.start_frame);
                }
            }
            if columns[place].entries.is_empty() {
                columns.remove(place);
            }
            Command::SetSheetText {
                composition,
                columns,
            }
        } else if id == "property.set_expression" {
            // B-14c: D-59's expression, on a layer's property or the camera's. `text` absent
            // removes it; `on=false` keeps it and switches it off. A text the language cannot
            // read is stored as typed, because document 24 says a half-typed expression is not
            // lost: its error is shown beside the property until it is fixed or switched off.
            let camera = parameter(query, "target").as_deref() == Some("camera");
            let name = parameter(query, "prop").unwrap_or_default();
            // D-291: an effect's number setting, named `fx:<instance>:<setting>` as for its keys.
            if let Some((instance_id, setting)) = effect_setting(&name) {
                let Some(layer_id) = parameter(query, "layer").map(Id::new) else {
                    return Some("Which layer? Choose one in the layer list.".to_string());
                };
                let Some(layer) = comp.layer(&layer_id) else {
                    return Some(format!("{layer_id} is not a layer in this composition."));
                };
                if !layer.effects.iter().any(|e| e.instance_id == instance_id) {
                    return Some(format!("{instance_id} is not an effect on this layer."));
                }
                Command::SetEffectExpression {
                    composition,
                    layer_id,
                    instance_id,
                    setting,
                    expression: parameter(query, "text").map(|text| Expression {
                        text,
                        enabled: parameter(query, "on").as_deref() != Some("false"),
                    }),
                }
            } else {
                let chosen = if camera {
                    anime_compositor::model::CameraProp::from_str(&name).map(prop_of_camera)
                } else {
                    property(&name)
                };
                let Some(prop) = chosen else {
                    return Some(format!(
                        "\"{name}\" is not a property that can carry an expression."
                    ));
                };
                let target = if camera {
                    Target::Camera
                } else {
                    let Some(layer_id) = parameter(query, "layer").map(Id::new) else {
                        return Some("Which layer? Choose one in the layer list.".to_string());
                    };
                    if comp.layer(&layer_id).is_none() {
                        return Some(format!("{layer_id} is not a layer in this composition."));
                    }
                    Target::Layer(layer_id)
                };
                Command::SetExpression {
                    composition,
                    target,
                    prop,
                    expression: parameter(query, "text").map(|text| Expression {
                        text,
                        enabled: parameter(query, "on").as_deref() != Some("false"),
                    }),
                }
            }
        } else if id == "property.link" {
            // B-27c: D-83's value whip. `layer` or `target=camera` and `prop` name the property
            // the spiral was dragged from, which gets the expression; `from_layer` or
            // `from_target=camera` and `from_prop` name the one it was dropped on. The core
            // writes the text or says why not, and the step is `property.set_expression`'s.
            use anime_compositor::expr;
            let side = |prefix: &str| {
                let name = parameter(query, &format!("{prefix}prop")).unwrap_or_default();
                if parameter(query, &format!("{prefix}target")).as_deref() == Some("camera") {
                    let prop =
                        anime_compositor::model::CameraProp::from_str(&name).map(prop_of_camera);
                    return prop.map(|p| (expr::Target::Camera, p)).ok_or(name);
                }
                let layer = parameter(query, &format!("{prefix}layer")).map(Id::new);
                match (layer, property(&name)) {
                    (Some(layer), Some(p)) => Ok((expr::Target::Layer(layer), p)),
                    _ => Err(name),
                }
            };
            let ((to, to_prop), (from, from_prop)) = match (side(""), side("from_")) {
                (Ok(to), Ok(from)) => (to, from),
                (Err(name), _) | (_, Err(name)) => {
                    return Some(format!("\"{name}\" is not a property that can be linked."))
                }
            };
            let text = match expr::link(comp, &from, from_prop, &to, to_prop) {
                Ok(text) => text,
                Err(said) => return Some(said),
            };
            Command::SetExpression {
                composition,
                target: match to {
                    expr::Target::Camera => Target::Camera,
                    expr::Target::Layer(id) => Target::Layer(id),
                },
                prop: to_prop,
                expression: Some(Expression {
                    text,
                    enabled: true,
                }),
            }
        } else if id == "layer.create" {
            // The one layer command that names no existing layer. It names a drawing instead,
            // because document 19's layer has an `asset_id` and no state in which it has none.
            let Some(asset) = parameter(query, "asset").map(Id::new) else {
                return Some("Which drawing should the new layer show?".to_string());
            };
            if !project.assets.iter().any(|a| a.id == asset) {
                return Some(format!("There is no imported drawing called {asset}."));
            }
            // D-71: a layer made from a sound file is an audio layer.
            let sound = project
                .assets
                .iter()
                .any(|a| a.id == asset && a.kind == AssetKind::Audio);
            let mut layer = Layer::new(
                unused_layer_id(project),
                parameter(query, "name").unwrap_or_else(|| "New layer".to_string()),
                asset,
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            if sound {
                layer.kind = LayerKind::Audio;
            }
            // D-292: a sequence's drawings are exposed on ones in number order from the layer's
            // first frame, as After Effects lays footage in; past the last one it is transparent.
            // Before, its sheet was empty and the layer drew nothing until it was filled.
            if let Some(numbers) = project
                .assets
                .iter()
                .find(|a| a.id == layer.asset_id && a.kind == AssetKind::ImageSequence)
                .map(|a| a.frames.keys().copied().collect::<Vec<u32>>())
                .filter(|n| !n.is_empty())
            {
                let spans = anime_compositor::time::ExposureMap::on_twos_style(&numbers, 1)
                    .expect("one-frame spans in a row never overlap");
                layer.exposure_spans = spans.spans().to_vec();
            }
            // D-266: its anchor is the middle of its drawing and it stands at the composition's
            // centre, as After Effects places a new layer. A drawing that cannot be read is left
            // at the top left as before, rather than given a size it might not have.
            let size = project
                .assets
                .iter()
                .find(|a| {
                    a.id == layer.asset_id
                        && matches!(a.kind, AssetKind::Still | AssetKind::ImageSequence)
                })
                .and_then(|a| {
                    a.files()
                        .first()
                        .and_then(|f| media::decode(&held.root.join(f)).ok())
                })
                .map(|image| (image.width() as f64, image.height() as f64));
            if let Some((w, h)) = size {
                layer.transform.anchor = Property::constant(Value::Vec2(w / 2.0, h / 2.0));
                layer.transform.position = Property::constant(Value::Vec2(
                    comp.width as f64 / 2.0,
                    comp.height as f64 / 2.0,
                ));
            }
            // The end of the order is the front of the picture -- `layers_in_order` is bottom
            // first -- and the front is where somebody who has just added a layer looks for it.
            // W-22: a drawing dropped on the layer list says where in the stack it landed.
            Command::AddLayer {
                composition,
                layer: Box::new(layer),
                index: parameter(query, "to")
                    .and_then(|to| to.parse::<usize>().ok())
                    .unwrap_or(comp.len()),
            }
        } else if id == "layer.add_adjustment" {
            // B-17c: D-66's layer with no drawing, so it names none. It lands where the page
            // says (`to`, above the chosen layer) or else at the front, where a new layer goes.
            let layer = Layer::adjustment(
                unused_layer_id(project),
                parameter(query, "name").unwrap_or_else(|| "Adjustment layer".to_string()),
                comp.width,
                comp.height,
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            Command::AddLayer {
                composition,
                layer: Box::new(layer),
                index: parameter(query, "to")
                    .and_then(|to| to.parse::<usize>().ok())
                    .unwrap_or(comp.len()),
            }
        } else if id == "layer.add_solid" {
            // B-23c: D-74's solid, the composition's size and covering it, named with the
            // smallest `Solid N` this composition does not have yet. Above the chosen layer
            // (`to`) or else at the front. The core refuses a value outside D-74's ranges.
            let grey = anime_compositor::color::srgb_to_linear(128.0 / 255.0) as f64;
            let solid = match solid_from(
                query,
                Solid {
                    color: [grey; 3],
                    width: comp.width,
                    height: comp.height,
                },
            ) {
                Ok(solid) => solid,
                Err(sentence) => return Some(sentence),
            };
            let name = (1..)
                .map(|n| format!("Solid {n}"))
                .find(|name| comp.layers_in_order().all(|l| &l.name != name))
                .expect("a name not yet taken");
            let layer = Layer::solid(
                unused_layer_id(project),
                parameter(query, "name").unwrap_or(name),
                solid,
                comp.width,
                comp.height,
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            Command::AddLayer {
                composition,
                layer: Box::new(layer),
                index: parameter(query, "to")
                    .and_then(|to| to.parse::<usize>().ok())
                    .unwrap_or(comp.len()),
            }
        } else if id == "layer.add_text" {
            // D-263: a text layer reading "Text" in the font that comes with the program, its
            // first line starting at `x`, `y` in composition pixels (the composition's centre,
            // centred, when not sent), named with the smallest `Text N` not yet taken. Above the
            // chosen layer (`to`) or else at the front, as a new solid lands.
            let centre = parameter(query, "x").is_none() && parameter(query, "y").is_none();
            // D-264: kerning on, as the editors start a new layer; `box` makes it paragraph text.
            let start = anime_compositor::text::Text {
                text: "Text".to_string(),
                font: anime_compositor::text::Text::BUNDLED_FONT.to_string(),
                size: (comp.height as f64 / 9.0).round().max(1.0),
                color: [1.0; 3],
                at: [comp.width as f64 / 2.0, comp.height as f64 / 2.0],
                align: if centre {
                    anime_compositor::text::Align::Center
                } else {
                    anime_compositor::text::Align::Left
                },
                kerning: true,
                ..Default::default()
            };
            let words = match text_from(query, start) {
                Ok(words) => words,
                Err(sentence) => return Some(sentence),
            };
            let name = (1..)
                .map(|n| format!("Text {n}"))
                .find(|name| comp.layers_in_order().all(|l| &l.name != name))
                .expect("a name not yet taken");
            let layer = Layer::text(
                unused_layer_id(project),
                parameter(query, "name").unwrap_or(name),
                words,
                comp.width,
                comp.height,
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            Command::AddLayer {
                composition,
                layer: Box::new(layer),
                index: parameter(query, "to")
                    .and_then(|to| to.parse::<usize>().ok())
                    .unwrap_or(comp.len()),
            }
        } else if id == "layer.add_null" {
            // B-26c: D-82's null, named with the smallest `Null N` this composition does not
            // have yet, lasting the whole composition, at its centre. Above the chosen layer
            // (`to`) or else at the front, as a new solid lands.
            let name = (1..)
                .map(|n| format!("Null {n}"))
                .find(|name| comp.layers_in_order().all(|l| &l.name != name))
                .expect("a name not yet taken");
            let layer = Layer::null(
                unused_layer_id(project),
                parameter(query, "name").unwrap_or(name),
                comp.width,
                comp.height,
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            Command::AddLayer {
                composition,
                layer: Box::new(layer),
                index: parameter(query, "to")
                    .and_then(|to| to.parse::<usize>().ok())
                    .unwrap_or(comp.len()),
            }
        } else if id == "layer.add_shape" {
            // B-25c: D-78's shape layer, empty, over the frame exactly and named with the
            // smallest `Shape Layer N` this composition does not have yet. Above the chosen layer
            // (`to`) or else at the front, as a new solid lands.
            let name = (1..)
                .map(|n| format!("Shape Layer {n}"))
                .find(|name| comp.layers_in_order().all(|l| &l.name != name))
                .expect("a name not yet taken");
            let layer = Layer::shape(
                unused_layer_id(project),
                parameter(query, "name").unwrap_or(name),
                Vec::new(),
                comp.width,
                comp.height,
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            Command::AddLayer {
                composition,
                layer: Box::new(layer),
                index: parameter(query, "to")
                    .and_then(|to| to.parse::<usize>().ok())
                    .unwrap_or(comp.len()),
            }
        } else if id == "layer.add_composition" {
            // B-18c: D-67's layer of another composition, centred, above the chosen layer
            // (`to`) or else at the front. A cycle is the core's to refuse.
            let asked = parameter(query, "of").unwrap_or_default();
            let Some(inner) = project.composition(&Id::new(&asked)) else {
                return Some(format!("There is no composition {asked} in this project."));
            };
            let layer = Layer::composition(
                unused_layer_id(project),
                inner.name.clone(),
                inner,
                comp.width,
                comp.height,
                comp.start_frame,
                comp.start_frame + comp.duration_frames as i32,
            );
            Command::AddLayer {
                composition,
                layer: Box::new(layer),
                index: parameter(query, "to")
                    .and_then(|to| to.parse::<usize>().ok())
                    .unwrap_or(comp.len()),
            }
        } else if parameter(query, "target").as_deref() == Some("camera")
            && id == "layer.set_parent"
        {
            // D-171: the layer the camera rides, keeping where it stands on the frame on screen,
            // as a layer's parent does.
            let frame = match frame_parameter(query, "frame") {
                Ok(frame) => frame,
                Err(said) => return Some(said),
            };
            Command::SetCameraParent {
                composition,
                parent: parameter(query, "parent")
                    .filter(|p| !p.is_empty())
                    .map(Id::new),
                frame,
                keep_place: true,
            }
        } else if parameter(query, "target").as_deref() == Some("camera") {
            // B-13e: the camera, named as a target instead of a layer.
            //
            // Document 24 line 91: "once a command can name the camera as its target instead of
            // a layer, every property command that already exists - set a base value, add a key,
            // delete a key, set an ease, set a motion path - reaches the camera unchanged". These
            // arms are that sentence. Each reads the camera's property and builds one of the same
            // four commands the layer arms below build, and then falls through to the same tail:
            // the drag transaction, the history entry, and the refusal turned into a sentence.
            //
            // The camera is not a layer, so this arm is beside the layer block rather than inside
            // it. Nothing here looks a layer up, and a lock on a layer does not reach it.
            use anime_compositor::model::{Camera, CameraProp};
            let name =
                match parameter(query, "prop") {
                    Some(name) => name,
                    // D-53: a path is position only, so that route names no property.
                    None if id == "keyframe.set_path" => "position".to_string(),
                    None => return Some(
                        "Which of the camera's properties? Its position, its depth or its zoom."
                            .to_string(),
                    ),
                };
            let Some(which) = CameraProp::from_str(&name) else {
                return Some(format!(
                    "\"{name}\" is not one of the camera's properties. They are position, \
                     depth and zoom."
                ));
            };
            let prop = prop_of_camera(which);
            // A composition nobody has pointed a camera at is shown, and keyed, at the camera the
            // renderer would have used. Pressing the diamond on a fresh composition holds that
            // default rather than a blank, which is what the three boxes already showed.
            let camera = comp
                .camera
                .clone()
                .unwrap_or_else(|| Camera::default_for(comp.width, comp.height));
            let property = camera.get(which).clone();
            match id {
                "keyframe.add_remove" => {
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    // Which of the two, read from the document as the layer arm reads it: a page
                    // can be describing a key that has since been undone.
                    match property.keyframe_at(frame) {
                        Some(_) => Command::RemoveKeyframe {
                            composition,
                            target: Target::Camera,
                            prop,
                            frame,
                        },
                        None => Command::SetKeyframe {
                            composition,
                            target: Target::Camera,
                            prop,
                            frame,
                            value: property.value_at(frame),
                            interp: Interp::Linear,
                            spatial: None,
                        },
                    }
                }
                "keyframe.move" => {
                    let from_frame = match frame_parameter(query, "from") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let to_frame = match frame_parameter(query, "to") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    Command::MoveKeyframe {
                        composition,
                        target: Target::Camera,
                        prop,
                        from_frame,
                        to_frame,
                    }
                }
                "keyframe.set_interp" => {
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let interp = match interp_parameter(query) {
                        Ok(interp) => interp,
                        Err(said) => return Some(said),
                    };
                    let Some(key) = property.keyframe_at(frame) else {
                        return Some(format!(
                            "The camera's {name} has no keyframe at frame {frame}, so there is \
                             no segment to ease. Add a keyframe first."
                        ));
                    };
                    Command::SetKeyframe {
                        composition,
                        target: Target::Camera,
                        prop,
                        frame,
                        value: key.value,
                        interp,
                        spatial: key.spatial,
                    }
                }
                // D-53's handles. The camera's place is a point that travels, so it can carry
                // them; its depth and its zoom are lengths and cannot, which is the same rule
                // the core states and is said here in the words of the thing being asked for.
                "keyframe.set_path" => {
                    if which != CameraProp::Position {
                        return Some(format!(
                            "The camera's {name} is a number, not a path. Only its place can \
                             carry handles."
                        ));
                    }
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let text = parameter(query, "handles").unwrap_or_default();
                    let numbers: Vec<f64> = text
                        .split(',')
                        .filter_map(|n| n.trim().parse().ok())
                        .collect();
                    let [in_x, in_y, out_x, out_y] = numbers[..] else {
                        return Some(format!(
                            "A path is four numbers, in_x,in_y,out_x,out_y. Not \"{text}\"."
                        ));
                    };
                    let Some(key) = property.keyframe_at(frame) else {
                        return Some(format!(
                            "The camera's place has no keyframe at frame {frame}, so there is \
                             no handle to move. Add a keyframe first."
                        ));
                    };
                    Command::SetKeyframe {
                        composition,
                        target: Target::Camera,
                        prop,
                        frame,
                        value: key.value,
                        interp: key.interp,
                        spatial: Some([in_x, in_y, out_x, out_y]),
                    }
                }
                "property.set_base" | "property.drag_update" => {
                    let Some(text) = parameter(query, "value") else {
                        return Some(format!("What should the camera's {name} be set to?"));
                    };
                    let Some(value) = property_value(prop, &text) else {
                        return Some(match which {
                            CameraProp::Position => format!(
                                "The camera's place needs two numbers, like 960, 540. Not \
                                 \"{text}\"."
                            ),
                            _ => format!("The camera's {name} needs a number. Not \"{text}\"."),
                        });
                    };
                    // No unit conversion here, deliberately. This road carries what the file
                    // holds, as it does for every property: an opacity crosses it as nought to
                    // one and the panel is what shows a percentage, so a lens crosses it as the
                    // pixels D-58's arithmetic uses and the control is what shows millimetres.
                    // `camera.set_property` converts because its boxes are the boundary; this
                    // command's boundary is the blue number the person is dragging.
                    // W-10's rule, which is why the page sends the frame with every value: on a
                    // property that has keys the base is not what is drawn, so the same request
                    // sets a key at the frame on screen instead.
                    if !property.is_animated() {
                        Command::SetPropertyBase {
                            composition,
                            target: Target::Camera,
                            prop,
                            value,
                        }
                    } else {
                        let frame = match frame_parameter(query, "frame") {
                            Ok(frame) => frame,
                            Err(_) => {
                                return Some(format!(
                                    "The camera's {name} is keyframed, so a value belongs to a \
                                     frame. Say frame=<frame>."
                                ))
                            }
                        };
                        Command::SetKeyframe {
                            composition,
                            target: Target::Camera,
                            prop,
                            frame,
                            value,
                            interp: property
                                .keyframe_at(frame)
                                .map_or(Interp::Linear, |k| k.interp),
                            spatial: property.keyframe_at(frame).and_then(|k| k.spatial),
                        }
                    }
                }
                // Every other command names a layer, and saying so is better than the "Which
                // layer?" a camera request would otherwise have fallen through to.
                _ => {
                    return Some(format!(
                        "{id} changes a layer, not the camera. The camera takes a value, a \
                         keyframe, a move, an ease and a path."
                    ))
                }
            }
        } else {
            let Some(layer_id) = parameter(query, "layer").map(Id::new) else {
                return Some("Which layer? Choose one in the layer list.".to_string());
            };
            let Some(layer) = comp.layer(&layer_id) else {
                return Some(format!("{layer_id} is not a layer in this composition."));
            };
            // B-150c: a stretched layer's keys are named by the frame they play on.
            let stored = stored_key_query(id, query, layer);
            let query = stored.as_deref().or(query);
            let at = comp
                .index_of(&layer_id)
                .expect("a layer that is here has a place");
            match id {
                // D-57: the children of a layer being deleted let go where they stand,
                // in the same entry as the deletion, so that one undo puts both back. This is
                // the one place parenting does not follow the matte, whose reference is left
                // dangling: a dangling parent would move the layer, and a layer deleted by
                // mistake would take everything riding on it across the screen.
                "layer.delete" => {
                    let frame = frame_parameter(query, "frame").unwrap_or(0);
                    unparent = comp
                        .children_of(&layer_id)
                        .into_iter()
                        .map(|child| Command::SetParent {
                            composition: composition.clone(),
                            layer_id: child,
                            parent: None,
                            frame,
                            keep_place: true,
                        })
                        .collect();
                    // D-171: and the camera, if it rides the layer.
                    if comp.camera.as_ref().and_then(|c| c.parent.as_ref()) == Some(&layer_id) {
                        unparent.push(Command::SetCameraParent {
                            composition: composition.clone(),
                            parent: None,
                            frame,
                            keep_place: true,
                        });
                    }
                    Command::RemoveLayer {
                        composition,
                        layer_id,
                    }
                }
                // Not refused for being the name it already has. Document 26 makes that a
                // history entry that changes nothing, which is honest: somebody pressed F2 and
                // pressed return, and undo should take them back to before they did.
                "layer.rename" => match parameter(query, "name") {
                    Some(name) => Command::RenameLayer {
                        composition,
                        layer_id,
                        name,
                    },
                    None => return Some("A layer needs a name.".to_string()),
                },
                // Document 24 calls both of these a toggle, so the new value is read from the
                // document rather than sent by the page. A page that sent it would be deciding
                // what the layer currently is from a list that may be stale.
                "layer.toggle_visibility" => Command::SetLayerEnabled {
                    composition,
                    layer_id,
                    value: !layer.enabled,
                },
                "layer.toggle_lock" => Command::SetLayerLocked {
                    composition,
                    layer_id,
                    value: !layer.locked,
                },
                // D-71: an audio layer's level, in decibels. The core keeps it between -96 and 12.
                "layer.set_gain" => {
                    let text = parameter(query, "value").unwrap_or_default();
                    let Ok(value) = text.trim().parse::<f64>() else {
                        return Some(format!(
                            "\"{}\" is not a level. A level is a number of decibels, from -96 to 12.",
                            text.trim()
                        ));
                    };
                    Command::SetAudioGain {
                        composition,
                        layer_id,
                        value,
                    }
                }
                // B-23c: D-74's colour and size from the layer's panel. What is not sent stays as
                // it is; a layer that is not a solid is the core's to refuse.
                "solid.set" => {
                    let now = layer.solid.unwrap_or(Solid {
                        color: [0.0; 3],
                        width: 1,
                        height: 1,
                    });
                    match solid_from(query, now) {
                        Ok(solid) => Command::SetSolid {
                            composition,
                            layer_id,
                            solid,
                        },
                        Err(sentence) => return Some(sentence),
                    }
                }
                // D-263: the words and how they look, from Effect controls. What is not sent stays
                // as it is; a layer that is not a text layer is the core's to refuse.
                "text.set" => {
                    let Some(now) = layer.text.clone() else {
                        return Some(format!("\"{}\" is not a text layer, so it has no words.", layer.name));
                    };
                    match text_from(query, now) {
                        Ok(text) => Command::SetText {
                            composition,
                            layer_id,
                            text,
                        },
                        Err(sentence) => return Some(sentence),
                    }
                }
                // B-24c: D-77's masks from the picture and from the layer's panel. The core
                // takes the whole list in one command, so each of these reads the list the
                // layer has, changes one thing in it and sends it back: a mask drawn, a point or
                // a handle moved, one setting, or one mask taken out. Whether the shape is legal
                // stays in the core, which is the reader that knows document 19's rules.
                //
                // D-71: a sound layer has no picture to cut, so it has no masks. The core
                // refuses it too - `sets_a_picture` names SET_MASKS - and this says it in the
                // words of the thing the person was doing, before a shape is even read.
                "mask.add" | "mask.set_path" | "mask.set" | "mask.delete"
                | "mask.add_remove_key" => {
                    if layer.kind == LayerKind::Audio {
                        return Some(format!(
                            "\"{}\" is a sound layer, so it has no masks.",
                            layer.name
                        ));
                    }
                    let mut masks = layer.masks.clone();
                    // Which mask, for the three that name one. `mask.add` makes the next one.
                    let at = match id {
                        "mask.add" => masks.len(),
                        _ => match parameter(query, "mask").and_then(|m| m.parse::<usize>().ok()) {
                            Some(at) if at < masks.len() => at,
                            _ => {
                                return Some(format!(
                                    "Which mask? \"{}\" has {}.",
                                    layer.name,
                                    match masks.len() {
                                        0 => "none".to_string(),
                                        1 => "one".to_string(),
                                        n => format!("{n}"),
                                    }
                                ))
                            }
                        },
                    };
                    match id {
                        "mask.delete" => {
                            masks.remove(at);
                        }
                        // B-24d: the stopwatch. No keys at all and the path starts moving, from
                        // the shape it has now; a key under the playhead comes off, and the last
                        // one to come off leaves its own shape behind as the base, so that
                        // turning the stopwatch off does not move the mask. Otherwise the shape
                        // on this frame becomes a key of its own.
                        "mask.add_remove_key" => {
                            let frame = match frame_parameter(query, "frame") {
                                Ok(frame) => frame,
                                Err(said) => return Some(said),
                            };
                            let mask = &mut masks[at];
                            match mask.keys.iter().position(|k| k.frame == frame) {
                                Some(i) => {
                                    let gone = mask.keys.remove(i);
                                    if mask.keys.is_empty() {
                                        mask.points = gone.points;
                                    }
                                }
                                None => {
                                    let points = mask.points_at(frame);
                                    mask.keys.push(anime_compositor::mask::MaskKey {
                                        frame,
                                        points,
                                        interp: anime_compositor::model::Interp::Linear,
                                    });
                                }
                            }
                        }
                        "mask.add" | "mask.set_path" => {
                            let points = match mask_points(query) {
                                Ok(points) => points,
                                Err(sentence) => return Some(sentence),
                            };
                            if id == "mask.set_path" && !masks[at].keys.is_empty() {
                                // B-24d: a path that moves is edited at the key under the
                                // playhead, not at its base, and a drag between two keys makes
                                // one there rather than moving the whole path under them.
                                let frame = match frame_parameter(query, "frame") {
                                    Ok(frame) => frame,
                                    Err(said) => return Some(said),
                                };
                                let mask = &mut masks[at];
                                match mask.keys.iter_mut().find(|k| k.frame == frame) {
                                    Some(key) => key.points = points,
                                    None => mask.keys.push(anime_compositor::mask::MaskKey {
                                        frame,
                                        points,
                                        interp: anime_compositor::model::Interp::Linear,
                                    }),
                                }
                            } else if id == "mask.add" {
                                // Numbered past every name already in use, the way layer ids are,
                                // so that drawing, undoing and drawing again does not make two
                                // masks called the same thing.
                                let name = (1..)
                                    .map(|n| format!("Mask {n}"))
                                    .find(|name| masks.iter().all(|m| &m.name != name))
                                    .expect("the numbers do not run out");
                                // D-737: `closed=0` leaves the pen's path open, a line of mode
                                // None that cuts nothing.
                                let closed = parameter(query, "closed").as_deref() != Some("0");
                                masks.push(anime_compositor::mask::Mask {
                                    name,
                                    points,
                                    closed,
                                    mode: if closed {
                                        anime_compositor::mask::MaskMode::Add
                                    } else {
                                        anime_compositor::mask::MaskMode::None
                                    },
                                    ..Default::default()
                                });
                            } else {
                                masks[at].points = points;
                            }
                        }
                        _ => {
                            if let Err(sentence) = path_key(query, &mut masks[at].keys) {
                                return Some(sentence);
                            }
                            if let Some(text) = parameter(query, "mode") {
                                match anime_compositor::mask::MaskMode::from_str(&text) {
                                    Some(mode) => masks[at].mode = mode,
                                    None => {
                                        return Some(format!(
                                            "A mask mode is add, subtract, intersect, difference \
                                             or none. Not \"{text}\"."
                                        ))
                                    }
                                }
                            }
                            for (key, what) in [
                                ("opacity", "an opacity"),
                                ("feather", "a feather"),
                                ("expansion", "an expansion"),
                            ] {
                                let Some(text) = parameter(query, key) else {
                                    continue;
                                };
                                let Ok(value) = text.trim().parse::<f64>() else {
                                    return Some(format!(
                                        "\"{}\" is not {what}. It is a number.",
                                        text.trim()
                                    ));
                                };
                                // D-298: a keyed number is set on a frame, through its key.
                                if masks[at].tracks.contains_key(key) {
                                    return Some(format!(
                                        "{}'s {key} has keys. Set it on a frame with \
                                         property.set_base and prop=mask:{at}:{key}.",
                                        masks[at].name
                                    ));
                                }
                                match key {
                                    // The panel counts opacity in percent, as it does a layer's.
                                    "opacity" => masks[at].opacity = value / 100.0,
                                    "feather" => masks[at].feather_px = value,
                                    _ => masks[at].expansion_px = value,
                                }
                            }
                            // The two switches are read from the document, as every other toggle
                            // in this window is, so a stale list in the page cannot flip them.
                            if parameter(query, "inverted").is_some() {
                                masks[at].inverted = !masks[at].inverted;
                            }
                            if parameter(query, "enabled").is_some() {
                                masks[at].enabled = !masks[at].enabled;
                            }
                            if let Some(name) = parameter(query, "name") {
                                masks[at].name = name;
                            }
                            // D-737: a switch like the two above, as a shape's `closed` is. Opening
                            // a mask makes it a line, mode None; closing it gives it Add, the mode
                            // a new mask has.
                            if parameter(query, "closed").is_some() {
                                let mask = &mut masks[at];
                                mask.closed = !mask.closed;
                                mask.mode = if mask.closed {
                                    anime_compositor::mask::MaskMode::Add
                                } else {
                                    anime_compositor::mask::MaskMode::None
                                };
                            }
                        }
                    }
                    Command::SetMasks {
                        composition,
                        layer_id,
                        masks,
                    }
                }
                // B-25c: D-78's shapes, the mask commands' twin. The same read-change-send of
                // the whole list, because SET_SHAPES takes the whole list, and the same pen: which
                // of the two it draws is decided by the kind of layer chosen.
                // D-738 (b): Group puts the chosen shapes, all in one group or in none, in a new
                // group inside that one; Ungroup gives a group's shapes and groups to the group it
                // is in and takes it away, its transform with it, as After Effects' does.
                "shape.group" | "shape.ungroup" => {
                    if layer.kind != LayerKind::Shape {
                        return Some(format!(
                            "\"{}\" is not a shape layer, so it has no shapes to group.",
                            layer.name
                        ));
                    }
                    let mut shapes = layer.shapes.clone();
                    let mut groups = layer.shape_groups.clone();
                    if id == "shape.group" {
                        let chosen: Option<Vec<usize>> = parameter(query, "shapes").and_then(|v| {
                            v.split(',')
                                .map(|n| n.trim().parse().ok().filter(|n| *n < shapes.len()))
                                .collect()
                        });
                        let Some(chosen) = chosen.filter(|c| !c.is_empty()) else {
                            return Some(format!(
                                "Which shapes? Say shapes=0,2; \"{}\" has {}.",
                                layer.name,
                                shapes.len()
                            ));
                        };
                        let parent = shapes[chosen[0]].group;
                        if chosen.iter().any(|&s| shapes[s].group != parent) {
                            return Some(
                                "Those shapes are in different groups. Choose shapes that sit \
                                 together."
                                    .to_string(),
                            );
                        }
                        let name = (1..)
                            .map(|n| format!("Group {n}"))
                            .find(|name| groups.iter().all(|g| &g.name != name))
                            .expect("the numbers do not run out");
                        groups.push(anime_compositor::shape::Group::new(name, parent));
                        for &s in &chosen {
                            shapes[s].group = Some(groups.len() - 1);
                        }
                    } else {
                        let gone = match parameter(query, "group").and_then(|g| g.parse().ok()) {
                            Some(g) if g < groups.len() => g,
                            _ => {
                                return Some(format!(
                                    "Which group? \"{}\" has {}.",
                                    layer.name,
                                    groups.len()
                                ))
                            }
                        };
                        let up = groups.remove(gone).parent;
                        // Into the group above, then every later place one lower.
                        let renumber = |g: Option<usize>| match g {
                            Some(g) if g == gone => up.map(|u| u - usize::from(u > gone)),
                            Some(g) => Some(g - usize::from(g > gone)),
                            None => None,
                        };
                        for s in &mut shapes {
                            s.group = renumber(s.group);
                        }
                        for g in &mut groups {
                            g.parent = renumber(g.parent);
                        }
                    }
                    Command::SetShapes {
                        composition,
                        layer_id: layer.id.clone(),
                        shapes,
                        groups: Some(groups),
                    }
                }
                // D-738 (c): a group's repeater added (After Effects' three copies, 100 apart),
                // taken away, or its composite set: on=1, on=0, composite=below|above.
                "shape.repeater" => {
                    let mut groups = layer.shape_groups.clone();
                    let at = parameter(query, "group").and_then(|g| g.parse::<usize>().ok());
                    let Some(group) = at.and_then(|g| groups.get_mut(g)) else {
                        return Some(format!(
                            "Which group? \"{}\" has {}.",
                            layer.name,
                            groups.len()
                        ));
                    };
                    match parameter(query, "on").as_deref() {
                        Some("1") => {
                            group.repeater.get_or_insert_with(Default::default);
                        }
                        Some("0") => group.repeater = None,
                        Some(other) => return Some(format!("on={other}? Say on=1 or on=0.")),
                        None => {}
                    }
                    if let Some(composite) = parameter(query, "composite") {
                        let Some(r) = group.repeater.as_mut() else {
                            return Some(format!("\"{}\" has no repeater.", group.name));
                        };
                        r.above = match composite.as_str() {
                            "above" => true,
                            "below" => false,
                            other => {
                                return Some(format!(
                                    "composite={other}? Say composite=below or composite=above."
                                ))
                            }
                        };
                    }
                    Command::SetShapes {
                        composition,
                        layer_id: layer.id.clone(),
                        shapes: layer.shapes.clone(),
                        groups: Some(groups),
                    }
                }
                // D-738 (d): a group's path operation added at the foot of its list, After
                // Effects' new one of a type, or the `op`th removed or its choice set:
                // add=<type>, remove=<k>, op=<k>&choice=<word>.
                "shape.operation" => {
                    use anime_compositor::shape::{Operation, OPERATIONS};
                    let mut groups = layer.shape_groups.clone();
                    let at = parameter(query, "group").and_then(|g| g.parse::<usize>().ok());
                    let Some(group) = at.and_then(|g| groups.get_mut(g)) else {
                        return Some(format!(
                            "Which group? \"{}\" has {}.",
                            layer.name,
                            groups.len()
                        ));
                    };
                    let names = || OPERATIONS.map(|o| o.0).join(", ");
                    let op = |key: &str| parameter(query, key).and_then(|k| k.parse::<usize>().ok());
                    if let Some(kind) = parameter(query, "add") {
                        let Some(o) = Operation::new(&kind) else {
                            return Some(format!("add={kind}? Say one of {}.", names()));
                        };
                        group.operations.push(o);
                    } else if let Some(k) = op("remove") {
                        if k >= group.operations.len() {
                            return Some(format!("\"{}\" has no operation {k}.", group.name));
                        }
                        group.operations.remove(k);
                    } else if let (Some(k), Some(word)) = (op("op"), parameter(query, "choice")) {
                        let Some(o) = group.operations.get_mut(k) else {
                            return Some(format!("\"{}\" has no operation {k}.", group.name));
                        };
                        o.choice = Some(word);
                    } else {
                        return Some(
                            "Say add=<type>, remove=<k>, or op=<k>&choice=<word>.".to_string(),
                        );
                    }
                    Command::SetShapes {
                        composition,
                        layer_id: layer.id.clone(),
                        shapes: layer.shapes.clone(),
                        groups: Some(groups),
                    }
                }
                "shape.add" | "shape.set_path" | "shape.set" | "shape.delete"
                | "shape.add_remove_key" => {
                    if layer.kind != LayerKind::Shape {
                        return Some(format!(
                            "\"{}\" is not a shape layer, so it has no shapes.",
                            layer.name
                        ));
                    }
                    let mut shapes = layer.shapes.clone();
                    let at = match id {
                        "shape.add" => shapes.len(),
                        _ => match parameter(query, "shape").and_then(|s| s.parse::<usize>().ok()) {
                            Some(at) if at < shapes.len() => at,
                            _ => {
                                return Some(format!(
                                    "Which shape? \"{}\" has {}.",
                                    layer.name,
                                    match shapes.len() {
                                        0 => "none".to_string(),
                                        1 => "one".to_string(),
                                        n => format!("{n}"),
                                    }
                                ))
                            }
                        },
                    };
                    // D-738 (a): a rectangle, ellipse, polygon or star is drawn from its settings.
                    if matches!(id, "shape.set_path" | "shape.add_remove_key") {
                        if let Some(p) = &shapes[at].parametric {
                            return Some(format!(
                                "\"{}\" is a {}, drawn from its settings, so its points cannot be moved one by one. Key its settings, or Convert to Bezier Path first.",
                                shapes[at].name,
                                p.kind.as_str()
                            ));
                        }
                    }
                    let key = |frame, points| anime_compositor::mask::MaskKey {
                        frame,
                        points,
                        interp: anime_compositor::model::Interp::Linear,
                    };
                    match id {
                        "shape.delete" => {
                            shapes.remove(at);
                        }
                        // The stopwatch, exactly as `mask.add_remove_key` works it.
                        "shape.add_remove_key" => {
                            let frame = match frame_parameter(query, "frame") {
                                Ok(frame) => frame,
                                Err(said) => return Some(said),
                            };
                            let shape = &mut shapes[at];
                            match shape.keys.iter().position(|k| k.frame == frame) {
                                Some(i) => {
                                    let gone = shape.keys.remove(i);
                                    if shape.keys.is_empty() {
                                        shape.points = gone.points;
                                    }
                                }
                                None => {
                                    let points = shape.points_at(frame);
                                    shape.keys.push(key(frame, points));
                                }
                            }
                        }
                        "shape.add" | "shape.set_path" => {
                            let parametric = match parameter(query, "parametric") {
                                Some(kind) if id == "shape.add" => {
                                    match parametric_shape(query, &kind) {
                                        Ok(p) => Some(p),
                                        Err(sentence) => return Some(sentence),
                                    }
                                }
                                _ => None,
                            };
                            let points = match &parametric {
                                Some(p) => Ok(p.outline(None)),
                                None => mask_points(query),
                            };
                            let mut points = match points {
                                Ok(points) => points,
                                Err(sentence) => return Some(sentence),
                            };
                            // D-738 (b): the page sends a grouped shape's points where they are
                            // drawn, and its path holds them in the group's own space.
                            if let Some(g) = shapes.get(at).and_then(|s| s.group) {
                                use anime_compositor::shape::{apply, invert, placing};
                                let frame = frame_parameter(query, "frame").unwrap_or(0);
                                let (m, _) = placing(&layer.shape_groups, Some(g), frame);
                                let Some(back) = invert(&m) else {
                                    return Some(format!(
                                        "\"{}\"'s group is scaled to nothing here, so its points cannot be placed.",
                                        shapes[at].name
                                    ));
                                };
                                for p in &mut points {
                                    p.point = apply(&back, p.point, false);
                                    p.in_handle = apply(&back, p.in_handle, true);
                                    p.out_handle = apply(&back, p.out_handle, true);
                                }
                            }
                            if id == "shape.add" {
                                // A closed shape comes with a fill and an open one with a stroke,
                                // so that what was just drawn can be seen: an open path has
                                // nothing to fill that a person meant, and a closed one drawn
                                // with a line only looks like a mask. Grey, as a new solid is.
                                let grey = anime_compositor::color::srgb_to_linear(128.0 / 255.0)
                                    as f64;
                                let closed = parametric.is_some()
                                    || parameter(query, "closed").as_deref() != Some("0");
                                let name = (1..)
                                    .map(|n| format!("Shape {n}"))
                                    .find(|name| shapes.iter().all(|s| &s.name != name))
                                    .expect("the numbers do not run out");
                                shapes.push(anime_compositor::shape::Shape {
                                    name,
                                    closed,
                                    points,
                                    fill: closed.then_some(anime_compositor::shape::Fill {
                                        color: [grey; 3],
                                        opacity: 1.0,
                                        gradient: None,
                                    }),
                                    stroke: (!closed)
                                        .then(|| anime_compositor::shape::Stroke::new([1.0; 3], 4.0)),
                                    parametric,
                                    ..Default::default()
                                });
                            } else if shapes[at].keys.is_empty() {
                                shapes[at].points = points;
                            } else {
                                let frame = match frame_parameter(query, "frame") {
                                    Ok(frame) => frame,
                                    Err(said) => return Some(said),
                                };
                                let shape = &mut shapes[at];
                                match shape.keys.iter_mut().find(|k| k.frame == frame) {
                                    Some(k) => k.points = points,
                                    None => shape.keys.push(key(frame, points)),
                                }
                            }
                        }
                        _ => {
                            if let Err(sentence) = path_key(query, &mut shapes[at].keys)
                                .and_then(|()| shape_settings(query, &mut shapes[at]))
                            {
                                return Some(sentence);
                            }
                        }
                    }
                    Command::SetShapes {
                        composition,
                        layer_id,
                        shapes,
                        groups: None,
                    }
                }
                // W-26: the shy switch, read from the document as the other toggles are.
                "layer.toggle_shy" => Command::SetLayerShy {
                    composition,
                    layer_id,
                    value: !layer.shy,
                },
                // D-188: After Effects' motion-blur switch, read from the document the same way.
                "layer.toggle_motion_blur" => Command::SetLayerMotionBlur {
                    composition,
                    layer_id,
                    value: !layer.motion_blur,
                },
                // D-216: Frame Mix, Time Stretch and Drawing Dissolve. Which layers take
                // them and what range is allowed is the core's to refuse.
                "layer.toggle_frame_blend" => Command::SetLayerFrameBlend {
                    composition,
                    layer_id,
                    value: !layer.frame_blend,
                },
                "layer.set_time_stretch" => {
                    let text = parameter(query, "value").unwrap_or_default();
                    let Ok(value) = text.trim().parse::<f64>() else {
                        return Some(format!(
                            "\"{}\" is not a stretch. A stretch is a percent, from 1 to 10000.",
                            text.trim()
                        ));
                    };
                    Command::SetLayerTimeStretch {
                        composition,
                        layer_id,
                        value,
                    }
                }
                // D-323: After Effects' Enable Time Remapping, on (`on=1`) or off (`on=0`).
                "layer.set_time_remap" => Command::SetTimeRemap {
                    composition,
                    layer_id,
                    value: match parameter(query, "on").as_deref() {
                        Some("1") => TimeRemap::On,
                        Some("0") => TimeRemap::Off,
                        _ => return Some("Say on=1 to turn Time Remapping on or on=0 to turn it off.".to_string()),
                    },
                },
                // D-314: After Effects' Freeze Frame, for a drawn layer: the drawing on the
                // playhead held for the whole layer, as one entry to undo. D-323: a composition
                // layer, or one with a Time Remap, is frozen by one hold key on its Time Remap.
                "layer.freeze_frame" if layer.composition_id.is_some() || layer.time_remap.is_some() => {
                    match frame_parameter(query, "frame") {
                        Ok(frame) => Command::SetTimeRemap {
                            composition,
                            layer_id,
                            value: TimeRemap::Freeze(frame),
                        },
                        Err(said) => return Some(said),
                    }
                }
                "layer.freeze_frame" => {
                    use anime_compositor::time::ExposureMap;
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    if layer.exposure_spans.is_empty() {
                        return Some(format!(
                            "{} has no drawings to hold. Freeze Frame holds a drawn or \
                             composition layer's picture.",
                            layer.name
                        ));
                    }
                    let timing = layer.timing();
                    let Some(here) = timing.local_frame(frame) else {
                        return Some(format!("Frame {frame} is outside {}.", layer.name));
                    };
                    let map = ExposureMap::new(layer.exposure_spans.clone()).ok();
                    let Some(drawing_number) = map.and_then(|m| m.drawing_at(here)) else {
                        return Some(format!(
                            "{} shows no drawing on frame {frame} to hold.",
                            layer.name
                        ));
                    };
                    // From the first exposure, or the layer's first frame, to past its last.
                    let first = timing.source_offset_frames;
                    let start_frame = layer.exposure_spans.iter().map(|s| s.start_frame).fold(first, i32::min);
                    let end_frame_exclusive = layer
                        .exposure_spans
                        .iter()
                        .map(|s| s.end_frame_exclusive)
                        .fold(timing.out_frame - timing.in_frame + first, i32::max);
                    Command::SetExposureSpans {
                        composition,
                        layer_id,
                        spans: vec![ExposureSpan {
                            start_frame,
                            end_frame_exclusive,
                            drawing_number,
                        }],
                    }
                }
                "layer.set_drawing_dissolve" => {
                    let text = parameter(query, "frames").unwrap_or_default();
                    let Ok(frames) = text.trim().parse::<u32>() else {
                        return Some(format!(
                            "\"{}\" is not a number of frames, from 0 to 100.",
                            text.trim()
                        ));
                    };
                    Command::SetDrawingDissolve {
                        composition,
                        layer_id,
                        frames,
                    }
                }
                // W-25: the blend mode, from the inspector's list or the layer's menu.
                "layer.set_blend_mode" => Command::SetBlendMode {
                    composition,
                    layer_id,
                    mode: match parameter(query, "mode").as_deref().and_then(BlendMode::from_str) {
                        Some(mode) => mode,
                        _ => {
                            return Some(format!(
                                "Which blend mode? Send mode={}.",
                                BlendMode::ALL.map(BlendMode::as_str).join(", ")
                            ))
                        }
                    },
                },
                // D-734: After Effects' Replace Layer Source (Ctrl+Alt+/): the item picked in
                // the Project panel takes the layer's place as its source; the core refuses a
                // kind that does not fit.
                "layer.replace_source" => match parameter(query, "asset") {
                    Some(asset) if !asset.is_empty() => Command::SetLayerSource {
                        composition,
                        layer_id,
                        asset: Id::new(&asset),
                    },
                    _ => {
                        return Some(
                            "Pick the picture, sequence or sound in the Project panel first, then press Ctrl+Alt+/."
                                .to_string(),
                        )
                    }
                },
                // W-24: After Effects' label colours, 0 for none and 1 to 8.
                "layer.set_label" => {
                    match parameter(query, "label").and_then(|l| l.parse::<u8>().ok()) {
                        Some(label) => Command::SetLayerLabel {
                            composition,
                            layer_id,
                            label,
                        },
                        None => {
                            return Some(
                                "Which colour? Send label=0 for none, or 1 to 8.".to_string(),
                            )
                        }
                    }
                }
                // Toward the front is later in the order. Asking to move the front layer further
                // forward is not an error and not a history entry: it is somebody pressing the
                // key one more time, and it is told so rather than given an undo item that
                // undoes nothing.
                "layer.move_up" if at + 1 < comp.len() => Command::ReorderLayer {
                    composition,
                    layer_id,
                    to_index: at + 1,
                },
                "layer.move_down" if at > 0 => Command::ReorderLayer {
                    composition,
                    layer_id,
                    to_index: at - 1,
                },
                "layer.move_up" => return Some(format!("{} is already at the front.", layer.name)),
                "layer.move_down" => {
                    return Some(format!("{} is already at the back.", layer.name))
                }
                // W-22: a row dragged up or down the list lands at one position, sent once at the
                // drop, as `effect.move` is for the effects. Counted from the back, as the order is.
                "layer.move" => {
                    let Some(to_index) =
                        parameter(query, "to").and_then(|to| to.parse::<usize>().ok())
                    else {
                        return Some(
                            "Where to? Send the position in the stack, counted from nought at \
                             the back."
                                .to_string(),
                        );
                    };
                    if to_index == at {
                        return Some(format!("{} is already at position {to_index}.", layer.name));
                    }
                    Command::ReorderLayer {
                        composition,
                        layer_id,
                        to_index,
                    }
                }
                // W-22: After Effects' Ctrl+D. The copy goes in front of the layer it copies.
                "layer.duplicate" => Command::AddLayer {
                    composition,
                    layer: Box::new(copy_of(project, layer)),
                    index: at + 1,
                },
                // W-05: the two halves of document 20's sentence, as the bar on the timeline
                // sends them. Both take frames rather than a distance, so that a drag which
                // sends the same numbers twice has asked for the same thing twice.
                "layer.shift" => Command::ShiftLayer {
                    composition,
                    layer_id,
                    in_frame: match frame_parameter(query, "in") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    },
                },
                "layer.trim" => Command::TrimLayer {
                    composition,
                    layer_id,
                    in_frame: match frame_parameter(query, "in") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    },
                    out_frame: match frame_parameter(query, "out") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    },
                },
                // W-10: document 24's toggle. Where the property has no key on the frame, one
                // is set holding the value the property already has there, so pressing the
                // diamond changes nothing on screen and only begins to hold it; where it has
                // one, that key is removed. Which of the two is read from the document, as the
                // other toggles are, rather than sent by a page that may be describing a key
                // that has since been undone.
                // B-19d: the same five requests about an effect's setting, which the page names
                // where it names a property.
                "keyframe.add_remove"
                | "keyframe.move"
                | "keyframe.set_interp"
                | "property.set_base"
                | "property.drag_update"
                    if parameter(query, "prop")
                        .as_deref()
                        .and_then(effect_setting)
                        .is_some() =>
                {
                    let (instance_id, setting) = parameter(query, "prop")
                        .as_deref()
                        .and_then(effect_setting)
                        .expect("the guard above");
                    match effect_key_command(id, query, composition, layer, instance_id, setting)
                    {
                        Ok(command) => command,
                        Err(said) => return Some(said),
                    }
                }
                // B-108b: and all five about a shape's gradient point or, since B-109b, trim.
                "keyframe.add_remove"
                | "keyframe.move"
                | "keyframe.set_interp"
                | "property.set_base"
                | "property.drag_update"
                    if parameter(query, "prop").as_deref().and_then(shape_prop).is_some() =>
                {
                    let which = parameter(query, "prop")
                        .as_deref()
                        .and_then(shape_prop)
                        .expect("the guard above");
                    match shape_key_command(id, query, composition, layer, which) {
                        Ok(command) => command,
                        Err(said) => return Some(said),
                    }
                }
                // D-298: and all five about a mask's feather, opacity or expansion.
                "keyframe.add_remove"
                | "keyframe.move"
                | "keyframe.set_interp"
                | "property.set_base"
                | "property.drag_update"
                    if parameter(query, "prop").as_deref().and_then(mask_prop).is_some() =>
                {
                    let which = parameter(query, "prop")
                        .as_deref()
                        .and_then(mask_prop)
                        .expect("the guard above");
                    match mask_key_command(id, query, composition, layer, which) {
                        Ok(command) => command,
                        Err(said) => return Some(said),
                    }
                }
                // D-738 (b): and all five about a shape group's transform.
                "keyframe.add_remove"
                | "keyframe.move"
                | "keyframe.set_interp"
                | "property.set_base"
                | "property.drag_update"
                    if parameter(query, "prop").as_deref().and_then(group_prop).is_some() =>
                {
                    let which = parameter(query, "prop")
                        .as_deref()
                        .and_then(group_prop)
                        .expect("the guard above");
                    match group_key_command(id, query, composition, layer, which) {
                        Ok(command) => command,
                        Err(said) => return Some(said),
                    }
                }
                // B-24f: and three of them about a mask's or a shape's path.
                "keyframe.add_remove" | "keyframe.move" | "keyframe.set_interp"
                    if parameter(query, "prop").as_deref().and_then(path_prop).is_some() =>
                {
                    let which = parameter(query, "prop")
                        .as_deref()
                        .and_then(path_prop)
                        .expect("the guard above");
                    match path_key_command(id, query, composition, layer, which) {
                        Ok(command) => command,
                        Err(said) => return Some(said),
                    }
                }
                // B-24h, D-79: a point added to a path, or taken off it. Not a path edit like
                // the others: it changes how many points the path has, so it changes the base
                // and every key on it at once rather than the key under the playhead.
                "path.add_point" | "path.remove_point" => {
                    let Some(which) = parameter(query, "prop").as_deref().and_then(path_prop)
                    else {
                        return Some(
                            "Which path? Say prop=mask:0 for the layer's first mask, or \
                             prop=shape:0 for its first shape."
                                .to_string(),
                        );
                    };
                    match path_point_command(id, query, composition, layer, which) {
                        Ok(command) => command,
                        Err(said) => return Some(said),
                    }
                }
                "keyframe.add_remove" => {
                    let Some(prop) = parameter(query, "prop").as_deref().and_then(property) else {
                        return Some(
                            "Which property? Say anchor, position, scale, rotation, \
                             opacity or depth."
                                .to_string(),
                        );
                    };
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let property = property_of(layer, prop);
                    match property.keyframe_at(frame) {
                        Some(_) => Command::RemoveKeyframe {
                            composition,
                            target: Target::Layer(layer_id.clone()),
                            prop,
                            frame,
                        },
                        None => Command::SetKeyframe {
                            composition,
                            target: Target::Layer(layer_id.clone()),
                            prop,
                            frame,
                            value: property.value_at(frame),
                            interp: Interp::Linear,
                            spatial: None,
                        },
                    }
                }
                // W-11: a key taken hold of on its property's row and put down on another frame.
                // The page names both frames rather than a distance, as `layer.shift` does, so
                // that what arrives is the whole move however many pointer moves drew it.
                //
                // No `drag=1` on this one, and that is deliberate: a drag record is replayed by
                // redo against the state the drag began in, and a move names the frame it starts
                // from, so an intermediate one would name a frame the key had already left. The
                // page follows the pointer with a mark of its own and sends this once, on
                // release, which is one history entry either way.
                "keyframe.move" => {
                    let Some(prop) = parameter(query, "prop").as_deref().and_then(property) else {
                        return Some(
                            "Which property? Say anchor, position, scale, rotation, \
                             opacity or depth."
                                .to_string(),
                        );
                    };
                    let from_frame = match frame_parameter(query, "from") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let to_frame = match frame_parameter(query, "to") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    Command::MoveKeyframe {
                        composition,
                        target: Target::Layer(layer_id.clone()),
                        prop,
                        from_frame,
                        to_frame,
                    }
                }
                // D-52's preset, which is the third thing a keyframe can be set to. Document 20
                // says the mode belongs to the segment that *starts* at the keyframe, so this
                // changes what happens between this key and the next one and nothing else - it
                // is not After Effects' F9, which eases both sides of the key it is pressed on.
                // The page's label says so.
                //
                // The value is read out of the document rather than sent, for the same reason
                // `keyframe.add_remove` reads which of its two halves to do: a page can be
                // describing a key that has since been undone, and the value is not the thing
                // being changed here.
                "keyframe.set_interp" => {
                    let Some(prop) = parameter(query, "prop").as_deref().and_then(property) else {
                        return Some(
                            "Which property? Say anchor, position, scale, rotation, \
                             opacity or depth."
                                .to_string(),
                        );
                    };
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let interp = match interp_parameter(query) {
                        Ok(interp) => interp,
                        Err(said) => return Some(said),
                    };
                    let property = property_of(layer, prop);
                    let Some(key) = property.keyframe_at(frame) else {
                        return Some(format!(
                            "{prop} has no keyframe at frame {frame}, so there is no segment to \
                             ease. Add a keyframe first."
                        ));
                    };
                    Command::SetKeyframe {
                        composition,
                        target: Target::Layer(layer_id.clone()),
                        prop,
                        frame,
                        value: key.value,
                        interp,
                        spatial: key.spatial,
                    }
                }
                // D-53's handles, pulled on the canvas. The four numbers are the whole of the
                // key's `spatial` as document 19 writes it, sent once at release; the value and
                // the interpolation are read out of the document, as `keyframe.set_interp` reads
                // the value, because a handle is the only thing this changes. Position only,
                // because a path is position only, so there is no property to name.
                "keyframe.set_path" => {
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let text = parameter(query, "handles").unwrap_or_default();
                    let numbers: Vec<f64> = text
                        .split(',')
                        .filter_map(|n| n.trim().parse().ok())
                        .collect();
                    let [in_x, in_y, out_x, out_y] = numbers[..] else {
                        return Some(format!(
                            "A path is four numbers, in_x,in_y,out_x,out_y. Not \"{text}\"."
                        ));
                    };
                    let path = property_of(layer, Prop::Position);
                    let Some(key) = path.keyframe_at(frame) else {
                        return Some(format!(
                            "position has no keyframe at frame {frame}, so there is no handle \
                             to move. Add a keyframe first."
                        ));
                    };
                    Command::SetKeyframe {
                        composition,
                        target: Target::Layer(layer_id.clone()),
                        prop: Prop::Position,
                        frame,
                        value: key.value,
                        interp: key.interp,
                        spatial: Some([in_x, in_y, out_x, out_y]),
                    }
                }
                // Typed into a field, or scrubbed on its label. The same command either way;
                // what differs is whether it becomes a history entry on its own or joins the one
                // the drag will commit at release.
                //
                // W-10: on a property that has keyframes the base is not what is drawn, so the
                // same request sets a key at the frame the page names instead, keeping the
                // interpolation a key already there was given. This is what After Effects does
                // when a value is changed on an animated property, and it is why the page sends
                // the frame with every value: the window decides which of the two it means.
                "property.set_base" | "property.drag_update" => {
                    let Some(prop) = parameter(query, "prop").as_deref().and_then(property) else {
                        return Some(
                            "Which property? Say anchor, position, scale, rotation, \
                             opacity or depth."
                                .to_string(),
                        );
                    };
                    let Some(text) = parameter(query, "value") else {
                        return Some(format!("What should {prop} be set to?"));
                    };
                    let Some(value) = property_value(prop, &text) else {
                        // Named before the core sees it, because the core's refusal is about
                        // kinds and this one is about the text not being numbers at all.
                        return Some(match prop.kind() {
                            "vec2" => {
                                format!("{prop} needs two numbers, like 12, -4. Not \"{text}\".")
                            }
                            _ => format!("{prop} needs a number. Not \"{text}\"."),
                        });
                    };
                    let property = property_of(layer, prop);
                    if !property.is_animated() {
                        Command::SetPropertyBase {
                            composition,
                            target: Target::Layer(layer_id.clone()),
                            prop,
                            value,
                        }
                    } else {
                        let frame = match frame_parameter(query, "frame") {
                            Ok(frame) => frame,
                            Err(_) => {
                                return Some(format!(
                                    "{prop} is keyframed, so a value belongs to a frame. Say \
                                     frame=<frame>."
                                ))
                            }
                        };
                        Command::SetKeyframe {
                            composition,
                            target: Target::Layer(layer_id.clone()),
                            prop,
                            frame,
                            value,
                            interp: property
                                .keyframe_at(frame)
                                .map_or(Interp::Linear, |k| k.interp),
                            spatial: property.keyframe_at(frame).and_then(|k| k.spatial),
                        }
                    }
                }
                // One command for all three of choosing a matte, changing whether the matte
                // layer is still drawn in its own right, and clearing it. Document 19 holds
                // them in one record and the core takes them in one command, so splitting them
                // here would only give the page a way to send half of one.
                //
                // No layer named means no matte, which is what the "none" entry in the list
                // sends. A layer that has gone is not the same thing and is refused by the
                // core, which is the reader that knows what is in the composition.
                "layer.set_matte" => Command::SetMatte {
                    composition,
                    layer_id,
                    matte: parameter(query, "matte")
                        .filter(|m| !m.is_empty())
                        .map(Id::new),
                    matte_only: parameter(query, "only").as_deref() == Some("true"),
                },
                // D-293: After Effects' Track Matte mode, kept apart from the matte layer as its
                // switch is. A word that is not one of the four is refused rather than read as alpha.
                "layer.set_matte_mode" => {
                    let word = parameter(query, "mode").unwrap_or_default();
                    let Some(mode) = MatteMode::parse(&word) else {
                        return Some(format!(
                            "\"{word}\" is not a matte mode. Choose alpha, alpha_inverted, luma or luma_inverted."
                        ));
                    };
                    Command::SetMatteMode { composition, layer_id, mode }
                }
                // D-57's parenting. No layer named means no parent, which is what the "none"
                // entry in the list sends; a layer that is not here is refused by the core.
                //
                // The frame comes from the page because document 21 works the keep-place
                // conversion at one frame, and on an animated parent the answer differs from
                // frame to frame. The layer keeps where it is on screen, which is what a person
                // choosing a parent means, and whether that could be done exactly is a sentence
                // said afterwards rather than a refusal.
                "layer.set_parent" => {
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let parent = parameter(query, "parent")
                        .filter(|p| !p.is_empty())
                        .map(Id::new);
                    inexact = parent.is_some()
                        && !anime_compositor::command::parent_keep_place_is_exact(
                            comp,
                            &layer_id,
                            parent.as_ref(),
                            frame,
                        );
                    Command::SetParent {
                        composition,
                        layer_id,
                        parent,
                        frame,
                        keep_place: true,
                    }
                }
                // D-58's depth: which plane the layer sits on, in pixels from its
                // parent's plane, or from the depth-0 plane when it has no parent. A
                // number of pixels and not a lens: this is the distance the parallax is
                // worked out from, and 0 is the plane the composition is drawn at.
                "layer.set_depth" => {
                    let Some(text) = parameter(query, "depth") else {
                        return Some(
                            "How far back? A depth is a number of pixels, and 0 is the \
                             plane the composition is drawn at."
                                .to_string(),
                        );
                    };
                    let Ok(value) = text.trim().parse::<f64>() else {
                        return Some(format!(
                            "\"{}\" is not a depth. A depth is a number of pixels.",
                            text.trim()
                        ));
                    };
                    Command::SetDepth {
                        composition,
                        layer_id,
                        value,
                    }
                }
                // Document 24's `exposure.set_span`, which assigns one span. The core takes the
                // whole ordered list, so the list is built here out of the one the layer has:
                // a span starting where an existing one starts replaces it, which is what
                // editing a row does, and anything else is inserted in frame order. Whether the
                // result is legal is document 20's rule and stays in the core.
                //
                // No drawing number means the row is being cleared, which is the only way to
                // remove an exposure and is not the same as setting it to drawing zero.
                "exposure.set_span" => {
                    let Some(start) = parameter(query, "start") else {
                        return Some("Which frame does the exposure start at?".to_string());
                    };
                    let Ok(start) = start.parse::<i32>() else {
                        return Some(format!(
                            "An exposure starts at a whole frame number. Not \"{start}\"."
                        ));
                    };
                    // Which row this is, when it is not the frame the row is moving to. An
                    // exposure moved to another frame is one row edited, not one added and one
                    // left behind, and the page is the only thing that knows which row was
                    // being typed in.
                    let at = match parameter(query, "was") {
                        None => start,
                        Some(was) => match was.parse::<i32>() {
                            Ok(was) => was,
                            Err(_) => {
                                return Some(format!(
                                    "An exposure starts at a whole frame number. Not \"{was}\"."
                                ))
                            }
                        },
                    };
                    let mut spans = layer.exposure_spans.clone();
                    let held = spans.iter().position(|s| s.start_frame == at);
                    if let Some(index) = held {
                        spans.remove(index);
                    }
                    match parameter(query, "drawing").filter(|d| !d.is_empty()) {
                        None if held.is_none() => {
                            return Some(format!(
                                "There is no exposure starting at frame {at} to clear."
                            ))
                        }
                        None => {}
                        Some(drawing) => {
                            let Ok(drawing_number) = drawing.parse::<u32>() else {
                                return Some(format!(
                                    "A drawing number is a whole number, counting from zero. \
                                     Not \"{drawing}\"."
                                ));
                            };
                            let Some(end) = parameter(query, "end") else {
                                return Some(
                                    "Which frame does the exposure end before?".to_string(),
                                );
                            };
                            let Ok(end_frame_exclusive) = end.parse::<i32>() else {
                                return Some(format!(
                                    "An exposure ends before a whole frame number. Not \"{end}\"."
                                ));
                            };
                            let span = ExposureSpan {
                                start_frame: start,
                                end_frame_exclusive,
                                drawing_number,
                            };
                            spans.retain(|s| s.start_frame != start);
                            spans.push(span);
                            spans.sort_by_key(|s| s.start_frame);
                            // Document 28 renders a frame whose drawing is absent as nothing
                            // and substitutes no neighbour. That is a decision somebody should
                            // meet while typing the number, not three hundred frames into an
                            // export, so the answer says it.
                            if !project
                                .assets
                                .iter()
                                .find(|a| a.id == layer.asset_id)
                                .is_some_and(|a| a.frames.contains_key(&drawing_number))
                            {
                                absent = Some(drawing_number);
                            }
                        }
                    }
                    Command::SetExposureSpans {
                        composition,
                        layer_id,
                        spans,
                    }
                }
                // D-84b: a cell of the Sheet written as on paper. A number or a cross lasts
                // from its frame until the next thing written below it, so it replaces what the
                // frames showed up to where that changes; erasing one lets what is written above
                // it run on through the same frames. The rest of the column is left as it was,
                // and all of it is one `SetExposureSpans`, so one cell is one entry to undo.
                "exposure.write" => {
                    use anime_compositor::time::ExposureMap;
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let timing = layer.timing();
                    let Some(here) = timing.local_frame(frame) else {
                        return Some(format!(
                            "Frame {frame} is outside {}, where its cells are shaded.",
                            layer.name
                        ));
                    };
                    // The layer's frames as the sheet shows them, and past its out point as
                    // far as its exposures go, so a drawing held off the end is replaced whole.
                    let first = timing.source_offset_frames;
                    let limit = layer
                        .exposure_spans
                        .iter()
                        .map(|s| s.end_frame_exclusive)
                        .fold(timing.out_frame - timing.in_frame + first, i32::max);
                    let map = ExposureMap::new(layer.exposure_spans.clone()).ok();
                    let shows = |f: i32| map.as_ref().and_then(|m| m.drawing_at(f));
                    let now = shows(here);
                    // The sheet's first cell always has something written in it; above it,
                    // on paper, there is nothing.
                    let above = if here == first { None } else { shows(here - 1) };
                    let written = here == first || above != now;
                    let mark = parameter(query, "mark").unwrap_or_default();
                    let new = match mark.as_str() {
                        "erase" if !written => {
                            return Some(format!(
                                "Nothing is written on frame {frame} of {} to erase: it {}.",
                                layer.name,
                                if now.is_some() {
                                    "holds the drawing above it"
                                } else {
                                    "is part of the blank above it"
                                }
                            ))
                        }
                        "erase" => above,
                        "x" => None,
                        number => match number.parse::<u32>() {
                            Ok(n) => Some(n),
                            Err(_) => {
                                return Some(format!(
                                    "A cell takes a drawing number, x or an erase. Not \
                                     \"{number}\"."
                                ))
                            }
                        },
                    };
                    if new == now {
                        return Some(format!(
                            "Frame {frame} of {} already shows {}.",
                            layer.name,
                            now.map_or("nothing".to_string(), |n| format!("drawing {n}"))
                        ));
                    }
                    let mut end = here + 1;
                    while end < limit && shows(end) == now {
                        end += 1;
                    }
                    // What lies either side of `here..end` is kept, cut where it overlaps.
                    let mut spans = Vec::new();
                    for s in &layer.exposure_spans {
                        if s.start_frame < here {
                            spans.push(ExposureSpan {
                                end_frame_exclusive: s.end_frame_exclusive.min(here),
                                ..*s
                            });
                        }
                        if s.end_frame_exclusive > end {
                            spans.push(ExposureSpan {
                                start_frame: s.start_frame.max(end),
                                ..*s
                            });
                        }
                    }
                    if let Some(drawing_number) = new {
                        // The same drawing straight above runs on, rather than being exposed
                        // again, which is what an erase means and what writing it again reads as.
                        match spans.iter_mut().find(|s| {
                            s.end_frame_exclusive == here && s.drawing_number == drawing_number
                        }) {
                            Some(s) => s.end_frame_exclusive = end,
                            None => spans.push(ExposureSpan {
                                start_frame: here,
                                end_frame_exclusive: end,
                                drawing_number,
                            }),
                        }
                        spans.sort_by_key(|s| s.start_frame);
                        if !project
                            .assets
                            .iter()
                            .find(|a| a.id == layer.asset_id)
                            .is_some_and(|a| a.frames.contains_key(&drawing_number))
                        {
                            absent = Some(drawing_number);
                        }
                    }
                    Command::SetExposureSpans {
                        composition,
                        layer_id,
                        spans,
                    }
                }
                // D-84g: K on a written number marks its drawing a key of the layer, or unmarks
                // it, and every cell where that number is written is circled. A line, a cross
                // or an empty cell has no number of its own to mark.
                "exposure.toggle_key" => {
                    use anime_compositor::time::ExposureMap;
                    let frame = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let timing = layer.timing();
                    let Some(here) = timing.local_frame(frame) else {
                        return Some(format!(
                            "Frame {frame} is outside {}, where its cells are shaded.",
                            layer.name
                        ));
                    };
                    let map = ExposureMap::new(layer.exposure_spans.clone()).ok();
                    let shows = |f: i32| map.as_ref().and_then(|m| m.drawing_at(f));
                    let now = shows(here);
                    let written = here == timing.source_offset_frames || shows(here - 1) != now;
                    let drawing = match (now, written) {
                        (Some(n), true) => n,
                        (Some(_), false) => {
                            return Some(format!(
                                "Frame {frame} of {} is a line holding the drawing above it. \
                                 Press K on the number itself.",
                                layer.name
                            ))
                        }
                        (None, true) => {
                            return Some(format!(
                                "Frame {frame} of {} is a cross, which is no drawing to mark as \
                                 a key.",
                                layer.name
                            ))
                        }
                        (None, false) => {
                            return Some(format!(
                                "Frame {frame} of {} is empty, under a cross, so it has no \
                                 drawing to mark as a key.",
                                layer.name
                            ))
                        }
                    };
                    let mut drawings = layer.key_drawings.clone();
                    match drawings.iter().position(|&d| d == drawing) {
                        Some(i) => {
                            drawings.remove(i);
                        }
                        None => {
                            drawings.push(drawing);
                            drawings.sort_unstable();
                        }
                    }
                    Command::SetKeyDrawings {
                        composition,
                        layer_id,
                        drawings,
                    }
                }
                // A new effect goes on the end of the stack, which document 21 evaluates last,
                // because that is where somebody who has just added one looks for it.
                "effect.add" => {
                    let Some(type_id) = parameter(query, "type") else {
                        return Some(
                            "Which effect? Say core.gaussian_blur, core.exposure, core.tint, \
                             core.line_smooth, core.selective_color_blur, core.glow, \
                             core.line_recolor, core.directional_blur, core.select_color, \
                             core.line_width, core.radial_blur, core.bloom, core.color_key, \
                             core.curves, core.levels, core.hue_saturation, core.gradient, \
                             core.drop_shadow, core.lens_blur, core.rim_light, core.outline, \
                             core.noise, core.chromatic_aberration, core.distance_gradation, \
                             core.light_rays, core.exposure_flicker, core.vignette, \
                             core.turbulent_displace, core.fractal_noise, core.gradient_map, \
                             core.color_balance, core.offset, core.light_wrap, core.invert, \
                             core.brightness_contrast, core.black_white, core.posterize, \
                             core.threshold, core.channel_mixer, core.vibrance, \
                             core.leave_color, core.solarize, core.halftone, core.mosaic, \
                             core.emboss, core.find_edges, core.sharpen, core.diffusion, \
                             core.wave_warp, core.ripple, core.twirl, core.bulge, core.mirror, \
                             core.motion_tile, core.linear_wipe, core.radial_wipe, \
                             core.venetian_blinds, core.iris_wipe, core.simple_choker, \
                             core.speed_lines, core.cross_glare, core.camera_shake, core.rain, \
                             core.color_lookup, core.line_blur, core.hsv_key, \
                             core.paraffin, core.kira_kira, core.lightning_bolt, \
                             core.compound_blur, core.displacement_map, core.gradient_wipe, core.glass_melt, core.tile_wipe, core.teeth_wipe, core.glow_wipe, core.stripe_sweep, core.push_iris, core.stretch_wipe, core.twister, core.warp_blend, core.channel_wipe, core.halation, core.echo, core.cc_wide_time, core.force_motion_blur, core.cc_rainfall, core.wave_world, core.caustics, core.card_dance, core.shatter, core.time_difference, core.posterize_time, core.pixel_motion_blur, core.timewarp, core.warp_stabilizer, core.rolling_shutter_repair, \
                             core.change_to_color, core.corner_pin, core.light_sweep, core.radio_waves, core.polar_coordinates, core.median, core.smart_blur, core.snowfall, core.kaleidoscope, core.roughen_edges, core.beam, core.four_color_gradient, core.cell_pattern, core.optics_compensation, core.radial_shadow, core.extract, core.bevel_alpha, core.bevel_edges, core.block_dissolve, core.shift_channels, core.solid_composite, core.channel_blur, core.fast_box_blur, core.fast_blur_legacy, core.gaussian_blur_legacy, core.colorama, core.glass, core.vector_blur, core.moment_map, core.pass_extract, core.depth_key, core.id_key, core.cryptomatte, core.depth_of_field, core.fog_3d, core.identifier, core.numbers, core.timecode, core.basic_text, core.text_animator, core.stretch_levels, core.stretch_contrast, core.stretch_color, core.spread_tones, core.matte_choker, core.refine_hard_matte, core.refine_soft_matte, core.stroke, core.bilateral_blur, core.cross_blur, core.spin_zoom_blur, core.fast_zoom_blur, core.broadcast_safe, core.color_neutralizer, core.color_offset, core.kernel, core.toner, core.change_color, core.bend_it, core.bender, core.blobbylize, core.glue_gun, core.threads, core.lens_flare, core.color_balance_hls, core.color_link, core.color_stabilizer, core.gamma_pedestal_gain, core.flow_motion, core.griddler, core.fisheye, core.levels_individual, core.photo_filter, core.color_grade, core.page_turn, core.power_pin, core.bezier_warp, core.mesh_warp, core.warp, core.liquify, core.reshape, core.mask_smear, core.ripple_pulse, core.slant, core.smear, core.split, core.split_2, core.tiles, core.magnify, core.spherize, core.checkerboard, core.circle, core.ellipse, core.detail_upscale, core.audio_spectrum, core.audio_waveform, core.fractal, core.grid, core.light_burst, core.fill, core.eyedropper_fill, core.paint_bucket, core.write_on, core.scribble, core.energy_stroke, core.add_grain, core.vegas, core.curl_noise, core.turbulent_noise, core.noise_alpha, core.noise_hls, core.noise_hls_auto, core.dust_scratches, core.match_grain, core.remove_grain, core.brush_strokes, core.cartoon, core.deband, core.advanced_spill_suppressor, core.color_difference_key, core.color_range, core.difference_matte, core.inner_outer_key, core.key_cleaner, core.linear_color_key, core.screen_key, core.spill_suppressor, core.reduce_interlace_flicker, core.unmult, core.three_d_glasses, core.cc_spotlight, core.cc_cylinder, core.cc_ball_action, core.particle_space, core.bubbles, core.drizzle, core.particle_plane, core.particle_playground, core.hair, core.particle_emitter, core.particle_grid, core.mercury, core.polygon_shards, core.foam, core.scatterize, core.star_burst, core.cc_environment, core.cc_sphere, core.card_wipe, core.simple_wire_removal, core.block_load, core.burn_film, core.hex_tile, core.mr_smoothie, core.plastic, core.repeat_tile, core.scatter, core.strobe_light, core.texturize, core.channel_threshold, core.threshold_rgb, core.contour_paint, core.cc_vignette, core.arbitrary_map, core.selective_color, core.shadow_highlight, core.tritone, core.aerial_haze, core.transform, core.noise_gate, core.compressor or core.distortion."
                                .to_string(),
                        );
                    };
                    // D-353: Soft Physical Glow is Glow's "physical" falloff, added as
                    // PLUGINS.md 2.3 adds it: a wide haze laid on with screen.
                    let physical = type_id == GLOW && parameter(query, "falloff").as_deref() == Some("physical");
                    let added = if physical { Some(soft_glow_added()) } else { new_effect(&type_id) };
                    let Some(effect) = added else {
                        return Some(format!(
                            "This build has no effect called {type_id}. It has \
                             core.gaussian_blur, core.exposure, core.tint, core.line_smooth, \
                             core.selective_color_blur, core.glow, core.line_recolor, \
                             core.directional_blur, core.select_color, core.line_width, \
                             core.radial_blur, core.bloom, core.color_key, core.curves, \
                             core.levels, core.hue_saturation, core.gradient, core.drop_shadow, \
                             core.lens_blur, core.rim_light, core.outline, core.noise, \
                             core.chromatic_aberration, core.distance_gradation, \
                             core.light_rays, core.exposure_flicker, core.vignette, \
                             core.turbulent_displace, core.fractal_noise, core.gradient_map, \
                             core.color_balance, core.offset, core.light_wrap, core.invert, \
                             core.brightness_contrast, core.black_white, core.posterize, \
                             core.threshold, core.channel_mixer, core.vibrance, \
                             core.leave_color, core.solarize, core.halftone, core.mosaic, \
                             core.emboss, core.find_edges, core.sharpen, core.diffusion, \
                             core.wave_warp, core.ripple, core.twirl, core.bulge, core.mirror, \
                             core.motion_tile, core.linear_wipe, core.radial_wipe, \
                             core.venetian_blinds, core.iris_wipe, core.simple_choker, \
                             core.speed_lines, core.cross_glare, core.camera_shake, core.rain, \
                             core.color_lookup, core.line_blur, core.hsv_key, \
                             core.paraffin, core.kira_kira, core.lightning_bolt, \
                             core.compound_blur, core.displacement_map, core.gradient_wipe, core.glass_melt, core.tile_wipe, core.teeth_wipe, core.glow_wipe, core.stripe_sweep, core.push_iris, core.stretch_wipe, core.twister, core.warp_blend, core.channel_wipe, core.halation, core.echo, core.cc_wide_time, core.force_motion_blur, core.cc_rainfall, core.wave_world, core.caustics, core.card_dance, core.shatter, core.time_difference, core.posterize_time, core.pixel_motion_blur, core.timewarp, core.warp_stabilizer, core.rolling_shutter_repair, \
                             core.change_to_color, core.corner_pin, core.light_sweep, core.radio_waves, core.polar_coordinates, core.median, core.smart_blur, core.snowfall, core.kaleidoscope, core.roughen_edges, core.beam, core.four_color_gradient, core.cell_pattern, core.optics_compensation, core.radial_shadow, core.extract, core.bevel_alpha, core.bevel_edges, core.block_dissolve, core.shift_channels, core.solid_composite, core.channel_blur, core.fast_box_blur, core.fast_blur_legacy, core.gaussian_blur_legacy, core.colorama, core.glass, core.vector_blur, core.moment_map, core.pass_extract, core.depth_key, core.id_key, core.cryptomatte, core.depth_of_field, core.fog_3d, core.identifier, core.numbers, core.timecode, core.basic_text, core.text_animator, core.stretch_levels, core.stretch_contrast, core.stretch_color, core.spread_tones, core.matte_choker, core.refine_hard_matte, core.refine_soft_matte, core.stroke, core.bilateral_blur, core.cross_blur, core.spin_zoom_blur, core.fast_zoom_blur, core.broadcast_safe, core.color_neutralizer, core.color_offset, core.kernel, core.toner, core.change_color, core.bend_it, core.bender, core.blobbylize, core.glue_gun, core.threads, core.lens_flare, core.color_balance_hls, core.color_link, core.color_stabilizer, core.gamma_pedestal_gain, core.flow_motion, core.griddler, core.fisheye, core.levels_individual, core.photo_filter, core.color_grade, core.page_turn, core.power_pin, core.bezier_warp, core.mesh_warp, core.warp, core.liquify, core.reshape, core.mask_smear, core.ripple_pulse, core.slant, core.smear, core.split, core.split_2, core.tiles, core.magnify, core.spherize, core.checkerboard, core.circle, core.ellipse, core.detail_upscale, core.audio_spectrum, core.audio_waveform, core.fractal, core.grid, core.light_burst, core.fill, core.eyedropper_fill, core.paint_bucket, core.write_on, core.scribble, core.energy_stroke, core.add_grain, core.vegas, core.curl_noise, core.turbulent_noise, core.noise_alpha, core.noise_hls, core.noise_hls_auto, core.dust_scratches, core.match_grain, core.remove_grain, core.brush_strokes, core.cartoon, core.deband, core.advanced_spill_suppressor, core.color_difference_key, core.color_range, core.difference_matte, core.inner_outer_key, core.key_cleaner, core.linear_color_key, core.screen_key, core.spill_suppressor, core.reduce_interlace_flicker, core.unmult, core.three_d_glasses, core.cc_spotlight, core.cc_cylinder, core.cc_ball_action, core.particle_space, core.bubbles, core.drizzle, core.particle_plane, core.particle_playground, core.hair, core.particle_emitter, core.particle_grid, core.mercury, core.polygon_shards, core.foam, core.scatterize, core.star_burst, core.cc_environment, core.cc_sphere, core.card_wipe, core.simple_wire_removal, core.block_load, core.burn_film, core.hex_tile, core.mr_smoothie, core.plastic, core.repeat_tile, core.scatter, core.strobe_light, core.texturize, core.channel_threshold, core.threshold_rgb, core.contour_paint, core.cc_vignette, core.arbitrary_map, core.selective_color, core.shadow_highlight, core.tritone, core.aerial_haze, core.transform, core.noise_gate, core.compressor and core.distortion."
                        ));
                    };
                    // D-87: selective colour blur matches exact colours, which anything before
                    // it would change, so it goes to the top of the stack. D-86 as amended:
                    // smoothing finds steps a blur or tint before it would hide, so it goes to
                    // the top too, but below the selective colour blurs already there. D-91:
                    // line recolour matches exact colours as well, and goes there too, and
                    // so do D-93's select colour, D-94's line width and D-97's colour key.
                    let selective = |e: &EffectInstance| {
                        matches!(e.effect, Effect::SelectiveColorBlur { .. })
                    };
                    let index = match effect {
                        Effect::SelectiveColorBlur { .. } => Some(0),
                        Effect::LineSmooth { .. }
                        | Effect::LineRecolor { .. }
                        | Effect::SelectColor { .. }
                        | Effect::LineWidth { .. }
                        | Effect::ColorKey { .. } => {
                            Some(layer.effects.iter().take_while(|e| selective(e)).count())
                        }
                        _ => None,
                    };
                    Command::AddEffect {
                        composition,
                        layer_id,
                        effect: EffectInstance::new(unused_effect_id(project), effect),
                        index,
                    }
                }
                // W-31: effects copied off a layer or kept as a preset, in the file's own words,
                // put on the end of the stack in the order they came, under identifiers nothing
                // is using, as one entry to undo (the list D-57's deletion fills is applied as
                // one). The file's rules read them, and refuse what this build would drop.
                "effect.paste" => {
                    let Some(text) = parameter(query, "effects") else {
                        return Some("Which effects? Copy some first, or choose a preset.".to_string());
                    };
                    let pasted = match persist::read_effects(&text) {
                        Ok(pasted) => pasted,
                        Err(diagnostic) => return Some(diagnostic.sentence()),
                    };
                    let next = unused_effect_id(project)
                        .as_str()
                        .strip_prefix("fx-")
                        .and_then(|n| n.parse::<u64>().ok())
                        .unwrap_or(1);
                    let mut adds: Vec<Command> = pasted
                        .into_iter()
                        .enumerate()
                        .map(|(k, effect)| Command::AddEffect {
                            composition: composition.clone(),
                            layer_id: layer_id.clone(),
                            effect: EffectInstance {
                                instance_id: Id::new(format!("fx-{}", next + k as u64)),
                                ..effect
                            },
                            index: None,
                        })
                        .collect();
                    let Some(last) = adds.pop() else {
                        return Some("There were no effects to paste.".to_string());
                    };
                    unparent = adds;
                    last
                }
                // D-739: an animation preset, its keys starting at `frame`, as one step to undo.
                "preset.apply" => {
                    let Some(text) = parameter(query, "preset") else {
                        return Some("Which preset? Choose one in the presets list.".to_string());
                    };
                    let preset = match persist::read_preset(&text) {
                        Ok(preset) => preset,
                        Err(diagnostic) => return Some(diagnostic.sentence()),
                    };
                    let at = match frame_parameter(query, "frame") {
                        Ok(frame) => frame,
                        Err(said) => return Some(said),
                    };
                    let next = unused_effect_id(project)
                        .as_str()
                        .strip_prefix("fx-")
                        .and_then(|n| n.parse::<u64>().ok())
                        .unwrap_or(1);
                    let mut all = match preset.commands(&composition, layer, at, next) {
                        Ok(all) => all,
                        Err(said) => return Some(said),
                    };
                    let Some(last) = all.pop() else {
                        return Some(format!("\"{}\" has nothing in it to apply.", preset.name));
                    };
                    unparent = all;
                    last
                }
                // The other six all name an instance that is already on the layer, so the
                // lookup and its refusal are written once.
                "effect.delete"
                | "effect.toggle_bypass"
                | "effect.set_parameters"
                | "effect.move"
                | "effect.move_up"
                | "effect.move_down" => {
                    let Some(instance_id) = parameter(query, "effect").map(Id::new) else {
                        return Some("Which effect? Choose one in the effects list.".to_string());
                    };
                    let Some(existing) =
                        layer.effects.iter().find(|e| e.instance_id == instance_id)
                    else {
                        return Some(format!("{instance_id} is not an effect on this layer."));
                    };
                    // Where in the stack it is now. The list in the window is drawn in
                    // evaluation order, first at the top, so up is one step earlier.
                    let at = layer
                        .effects
                        .iter()
                        .position(|e| e.instance_id == instance_id)
                        .unwrap_or(0);
                    match id {
                        "effect.delete" => Command::RemoveEffect {
                            composition,
                            layer_id,
                            instance_id,
                        },
                        // Refused in words at either end rather than clamped: an arrow that
                        // silently did nothing would read as a window that had stopped
                        // listening, which is the thing the first sitting with this build
                        // complained of.
                        "effect.move_up" if at == 0 => {
                            return Some(format!("{instance_id} is already first."));
                        }
                        "effect.move_down" if at + 1 >= layer.effects.len() => {
                            return Some(format!("{instance_id} is already last."));
                        }
                        "effect.move_up" | "effect.move_down" => Command::ReorderEffect {
                            composition,
                            layer_id,
                            instance_id,
                            to_index: if id == "effect.move_up" {
                                at - 1
                            } else {
                                at + 1
                            },
                        },
                        // W-07: a card dragged up or down the stack lands at one position,
                        // sent once at the drop. Past the end is the core's refusal; the place
                        // it already has is refused here, because the core would write it as
                        // a change and it would be an undo step that undoes nothing.
                        "effect.move" => {
                            let Some(to_index) =
                                parameter(query, "to").and_then(|to| to.parse::<usize>().ok())
                            else {
                                return Some(
                                    "Where to? Send the position in the stack, counted from \
                                     nought."
                                        .to_string(),
                                );
                            };
                            if to_index == at {
                                return Some(format!(
                                    "{instance_id} is already at position {to_index}."
                                ));
                            }
                            Command::ReorderEffect {
                                composition,
                                layer_id,
                                instance_id,
                                to_index,
                            }
                        }
                        "effect.toggle_bypass" => Command::SetEffectEnabled {
                            composition,
                            layer_id,
                            instance_id,
                            enabled: !existing.enabled,
                        },
                        // The type is read off the effect rather than sent by the page. The
                        // core refuses settings of the wrong kind for an instance, and this
                        // makes that mistake unreachable from here instead of catchable.
                        _ => match effect_parameters(existing.type_id(), query) {
                            Ok(effect) => Command::SetEffectParameters {
                                composition,
                                layer_id,
                                instance_id,
                                effect,
                            },
                            Err(said) => return Some(said),
                        },
                    }
                }
                _ => return None,
            }
        }
    };
    let before = viewer
        .lock()
        .expect("the viewer lock was poisoned")
        .document
        .undo_depth();
    // W-05: any edit joins the running drag when the page says so. A bar pulled along the
    // timeline sends `layer.shift` or `exposure.set_span` many times a second, and one press
    // has to be one thing to undo whichever command is inside it, which is the rule
    // `property.drag_update` already has for the transform fields.
    let said = if !unparent.is_empty() {
        // D-57: the children first, so that each is converted while the parent it is letting
        // go of is still there, and all of it as one entry to undo.
        let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
        unparent.push(command);
        match held.document.apply_all(unparent) {
            Ok(record) => record.label.clone(),
            Err(diagnostic) => diagnostic.sentence(),
        }
    } else if id == "property.drag_update" || parameter(query, "drag").is_some() {
        drag_update(viewer, command)
    } else {
        edit(viewer, command)
    };
    // Only when the change landed. A command the core refused changed nothing, and a warning
    // about what it would have done reads as though it had happened.
    let landed = viewer
        .lock()
        .expect("the viewer lock was poisoned")
        .document
        .undo_depth()
        > before;
    if let (Some(drawing), true) = (absent, landed) {
        return Some(format!(
            "{said} Drawing {drawing} is not in this sequence, so the frames exposing it stay \
             empty; no neighbouring drawing is put there instead."
        ));
    }
    // Document 21: where the chain scales differently in x and y and something in it is
    // turned, the exact answer is a skew a layer cannot hold. The anchor lands where it was and
    // the rest of the layer may shift. That is a thing to meet at the moment it happens rather
    // than to find later by looking at the picture, so the answer says it.
    if inexact && landed {
        return Some(format!(
            "{said} It could not keep its shape exactly: the parent is stretched more one way \
             than the other and something in the chain is turned, so the anchor is where it \
             was and the rest of the layer has moved. Ctrl+Z undoes it."
        ));
    }
    // Deleting a layer that another layer was using as its matte leaves that reference behind.
    // The project loader treats it as a warning and keeps it rather than clearing it, so this
    // is a legal state and not a fault -- but nothing else would tell the person at the moment
    // it happened, and they would meet it as a warning the next time the file was opened.
    Some(match dangling_mattes(viewer) {
        names if id == "layer.delete" && !names.is_empty() => format!(
            "{said} {} now shaped by a layer that is not here; undo puts it back.",
            match names.as_slice() {
                [one] => format!("\"{one}\" is"),
                many => format!("{} layers are", many.len()),
            }
        ),
        _ => said,
    })
}

/// The names of layers whose matte points at a layer the composition does not have.
fn dangling_mattes(viewer: &Mutex<Viewer>) -> Vec<String> {
    let held = viewer.lock().expect("the viewer lock was poisoned");
    let Some(comp) = held.document.project().composition(&held.composition) else {
        return Vec::new();
    };
    comp.layers_in_order()
        .filter(|l| {
            l.matte
                .as_ref()
                .is_some_and(|m| comp.layer(&m.layer_id).is_none())
        })
        .map(|l| l.name.clone())
        .collect()
}

// -------------------------------------------------------------------------------------------
// Export
// -------------------------------------------------------------------------------------------

/// The export the window has running, and what the last one did.
///
/// Beside the viewer rather than inside it, and for one reason: an export outlives the project it
/// came from. It works on a snapshot taken when the person asked for it, so opening another
/// project or recovering a snapshot while it runs replaces the [`Viewer`] and must not take away
/// the Cancel button or the report of a job still writing files.
#[derive(Default)]
struct Export {
    /// Set while a job is running. Setting it true is the whole of Cancel — `export_sequence`
    /// reads it between frames, which is what R-09's "cancellation between frames" means and why
    /// a cancelled job's files are whole ones.
    cancel: Option<Arc<AtomicBool>>,
    /// What the running job is doing, or what the last one did, in the core's words.
    said: String,
    /// B-12 fix: frames written so far, of how many, since when. The page draws a bar from these.
    done: Arc<AtomicUsize>,
    total: usize,
    started: Option<Instant>,
    /// D-251: the render queue's rows, the row being written, and a count that goes up with every
    /// change to them, so the page asks for the list again only when it has changed.
    queue: Vec<QueueRow>,
    row: Option<u64>,
    changes: u64,
    /// D-252: what the page shows of the job being written, and its Pause; a new one for every
    /// job, so the last job's frames stay to be looked at until the next begins. `first` is the
    /// job's first frame, where the page's bar begins.
    watch: Arc<export::Watch>,
    first: i32,
}

/// PNG depth and alpha for an export from the window.
///
/// R-09 makes both a choice and `ExportRequest` carries them; the window does not offer the
/// choice yet, so it states the default it is taking rather than leaving the reader to guess.
/// Eight bits and straight alpha is what document 21 line 31 asks for and what `T-08_frames/`
/// was written with, so what this window exports is comparable with what the fixtures committed.
const WINDOW_DEPTH: OutputDepth = OutputDepth::Eight;
const WINDOW_ALPHA: OutputAlpha = OutputAlpha::Straight;

/// The file each exported frame becomes, as the page's format list names it (D-62). No choice
/// is PNG, which is what the window wrote before it had the list.
fn output_format(query: Option<&str>) -> Result<OutputFormat, String> {
    match parameter(query, "format").as_deref() {
        None | Some("png") => Ok(OutputFormat::Png),
        Some("exr-half") => Ok(OutputFormat::Exr(ExrSamples::Half)),
        Some("exr-float") => Ok(OutputFormat::Exr(ExrSamples::Float)),
        Some("gif") => Ok(OutputFormat::Gif),
        Some("apng") => Ok(OutputFormat::Apng),
        Some("mp4") => Ok(OutputFormat::Mp4),
        Some(other) => Err(format!(
            "Nothing was exported: \"{other}\" is not a format this window writes. Choose PNG, \
             EXR half, EXR float, GIF, animated PNG or MP4."
        )),
    }
}

/// D-73's choices, as the page sends them: `quality=` beside an MP4, `dither=on` beside a GIF.
/// Neither said is Standard and no dithering, which is what B-21c and B-21d wrote.
fn export_choices(query: Option<&str>) -> Result<ExportChoices, String> {
    Ok(ExportChoices {
        mp4_quality: match parameter(query, "quality").as_deref() {
            None | Some("standard") => Mp4Quality::Standard,
            Some("preview") => Mp4Quality::Preview,
            Some("high") => Mp4Quality::High,
            Some(other) => {
                return Err(format!(
                    "Nothing was exported: \"{other}\" is not an MP4 quality. Choose Preview, \
                     Standard or High."
                ))
            }
        },
        gif_dither: parameter(query, "dither").as_deref() == Some("on"),
        // D-641: the WAV beside a sequence or an animated PNG.
        wav: parameter(query, "wav").as_deref() == Some("on"),
        ..ExportChoices::default()
    })
}

/// How the status line says what the job is written as, with the choice that was made.
fn written_as(format: OutputFormat, choices: ExportChoices) -> String {
    let beside = match format {
        OutputFormat::Gif | OutputFormat::Mp4 => false,
        _ => choices.wav,
    };
    let said = match format {
        OutputFormat::Png => String::new(),
        OutputFormat::Exr(ExrSamples::Half) => " as EXR, half float,".into(),
        OutputFormat::Exr(ExrSamples::Float) => " as EXR, full float,".into(),
        OutputFormat::Gif if choices.gif_dither => {
            " as one GIF, a preview of 256 colours a frame, dithered,".into()
        }
        OutputFormat::Gif => " as one GIF, a preview of 256 colours a frame,".into(),
        OutputFormat::Apng => " as one animated PNG,".into(),
        OutputFormat::Mp4 => format!(
            " as one MP4 at {} quality, over black, with the composition's sound,",
            match choices.mp4_quality {
                Mp4Quality::Preview => "Preview",
                Mp4Quality::Standard => "Standard",
                Mp4Quality::High => "High",
            }
        ),
    };
    if beside {
        format!("{said} with its sound as a WAV beside it,")
    } else {
        said
    }
}

/// Everything a job needs, taken from the viewer under one lock and owned from then on.
///
/// This is B-10's immutable export snapshot, and it is a `clone` rather than a lock held for four
/// minutes: what gets written is the shot as it was at the instant the person asked, whatever
/// happens to the open document while it is being written.
///
/// The range is the work area, which is the same first and last frame the transport steps between,
/// so what is exported is what the viewer plays.
fn export_job(
    viewer: &Viewer,
    into: &Path,
    missing: MissingSource,
    format: OutputFormat,
) -> (Project, PathBuf, ExportRequest) {
    // W-24: the work area, as After Effects renders it; the whole composition when there is none.
    let (first, last) = viewer
        .document
        .project()
        .composition(&viewer.composition)
        .map(|c| c.work_frames())
        .unwrap_or((0, -1));
    // The project's own name, so two shots exported into one folder do not overwrite each other.
    let stem = Path::new(&viewer.name)
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .filter(|s| !s.trim().is_empty())
        .unwrap_or_else(|| "shot".to_string());
    (
        viewer.document.project().clone(),
        viewer.root.clone(),
        ExportRequest {
            composition: viewer.composition.clone(),
            first_frame: first,
            last_frame: last,
            output_dir: into.to_path_buf(),
            naming: match format {
                OutputFormat::Png => format!("{stem}_%04d.png"),
                OutputFormat::Exr(_) => format!("{stem}_%04d.exr"),
                // D-72: one file for the whole job. `_animated` keeps it from looking like a
                // still, and from landing on a frame of a PNG sequence in the same folder.
                OutputFormat::Gif => format!("{stem}.gif"),
                OutputFormat::Apng => format!("{stem}_animated.png"),
                OutputFormat::Mp4 => format!("{stem}.mp4"),
            },
            depth: WINDOW_DEPTH,
            alpha: WINDOW_ALPHA,
            tile_size: DEFAULT_TILE_SIZE,
            missing,
            format,
            choices: Default::default(),
        },
    )
}

/// Run a job to the end and say what it did in the window's one line.
#[cfg(test)]
fn run_export(
    project: &Project,
    root: &Path,
    request: &ExportRequest,
    cancel: &AtomicBool,
) -> String {
    what_the_export_did(
        &export::export_sequence(project, root, request, cancel),
        &request.output_dir,
    )
}

/// An export report as a sentence, with every diagnostic the core produced kept.
///
/// Nothing here summarises: the core's `FrameLog` has already capped repeated warnings at three
/// and a count (D-25), so what arrives is bounded, and folding it further would be this window
/// deciding what the person is allowed to know about their own render.
fn what_the_export_did(report: &ExportReport, into: &Path) -> String {
    let mut lines = vec![match report.status {
        // D-72: a GIF or an animated PNG is one file however many frames it holds.
        ExportStatus::Completed if report.files_expected != report.frames_requested => format!(
            "Exported {} frames as one file, {}.",
            report.frames_requested,
            report
                .written
                .first()
                .unwrap_or(&into.to_path_buf())
                .display()
        ),
        ExportStatus::Completed => format!(
            "Exported {} frames into {}.",
            report.written.len(),
            into.display()
        ),
        ExportStatus::Blocked => "Nothing was exported.".to_string(),
        ExportStatus::Cancelled if report.files_expected != report.frames_requested => {
            "Nothing was exported.".to_string()
        }
        ExportStatus::Cancelled => format!(
            "The {} frames that finished are in {}.",
            report.written.len(),
            into.display()
        ),
        ExportStatus::Failed => format!(
            "The export stopped on a problem after {} of {} frames, in {}.",
            report.written.len(),
            report.frames_requested,
            into.display()
        ),
    }];
    // Document 28: output produced with something left out says so, in the report as well as in
    // the file's own tag.
    if report.fidelity_incomplete {
        lines.push(
            "These frames are not a faithful render: something this build does not support was \
             left out of them."
                .to_string(),
        );
    }
    lines.extend(report.diagnostics.iter().map(Diagnostic::sentence));
    lines.join(" ")
}

/// Start an export into `into`, or say why not. Returns what the status line should say now.
fn start_export(
    app: &AppHandle,
    into: &Path,
    missing: MissingSource,
    format: OutputFormat,
    choices: ExportChoices,
) -> String {
    let state = app.state::<Mutex<Export>>();
    if state
        .lock()
        .expect("the export lock was poisoned")
        .cancel
        .is_some()
    {
        return "An export is already running. Cancel it, or wait for it to finish.".to_string();
    }
    let (project, root, mut request) = {
        let viewer = app.state::<Mutex<Viewer>>();
        let viewer = viewer.lock().expect("the viewer lock was poisoned");
        export_job(&viewer, into, missing, format)
    };
    request.choices = choices;
    let said = format!(
        "Exporting {} frames{} into {}. The window stays usable while it writes.",
        request.last_frame - request.first_frame + 1,
        written_as(format, choices),
        into.display()
    );
    let cancel = Arc::new(AtomicBool::new(false));
    {
        let mut export = state.lock().expect("the export lock was poisoned");
        export.cancel = Some(Arc::clone(&cancel));
        export.said = said.clone();
        export.done = Arc::new(AtomicUsize::new(0));
        export.total = (request.last_frame - request.first_frame + 1) as usize;
        export.started = Some(Instant::now());
        export.watch = Arc::new(export::Watch::default());
        export.first = request.first_frame;
    }
    let (done, watch) = {
        let export = state.lock().expect("the export lock was poisoned");
        (Arc::clone(&export.done), Arc::clone(&export.watch))
    };
    // A thread, so the window keeps answering for frames while a shot is being written: an
    // export of the reference shot takes minutes, and a viewer frozen for minutes is a viewer
    // that looks broken.
    // Said here, before the thread starts, so an export that ends at once cannot finish before
    // the status line says it began.
    announce(&app.state::<Mutex<Viewer>>(), said.clone());
    let handle = app.clone();
    let started = said.clone();
    std::thread::spawn(move || {
        let done = contained(&handle, "exporting", || {
            what_the_export_did(
                &export::export_sequence_watched(&project, &root, &request, &cancel, &done, &watch),
                &request.output_dir,
            )
        })
        .unwrap_or_else(|said| said);
        {
            let state = handle.state::<Mutex<Export>>();
            let mut export = state.lock().expect("the export lock was poisoned");
            export.cancel = None;
            export.said = done;
        }
        // The owner's B-16c playtest: "Exporting..." stayed on the status line after the export
        // had ended. Replaced only if nothing has been said since, so a later command's answer
        // is never taken away.
        let viewer = handle.state::<Mutex<Viewer>>();
        let mut viewer = viewer.lock().expect("the viewer lock was poisoned");
        if viewer.status == started {
            viewer.status = EXPORT_ENDED.to_string();
        }
    });
    said
}

/// The status line once an export has ended, whatever happened: the line below it says what.
const EXPORT_ENDED: &str = "The export has ended. What it wrote, or why it did not, is below.";

/// Ask the operating system which folder the frames go in, then start writing them there.
fn ask_where_to_export(
    app: &AppHandle,
    missing: MissingSource,
    format: OutputFormat,
    choices: ExportChoices,
) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Export the frames into a folder")
        .pick_folder(move |chosen| {
            let Some(into) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            // Audit B8: files already there are replaced only once the person says so.
            let in_the_way = {
                let viewer = handle.state::<Mutex<Viewer>>();
                let viewer = viewer.lock().expect("the viewer lock was poisoned");
                let (_, _, mut request) = export_job(&viewer, &into, missing, format);
                request.choices = choices;
                export::files_in_the_way(&request)
            };
            let go = move |handle: AppHandle| {
                let said = start_export(&handle, &into, missing, format, choices);
                announce(&handle.state::<Mutex<Viewer>>(), said);
                refresh(&handle);
            };
            let Some(first) = in_the_way.first() else {
                return go(handle);
            };
            let asked = handle.clone();
            handle
                .dialog()
                .message(replace_question(first, in_the_way.len()))
                .title("Replace files?")
                .kind(tauri_plugin_dialog::MessageDialogKind::Warning)
                .buttons(tauri_plugin_dialog::MessageDialogButtons::OkCancelCustom(
                    "Replace".into(),
                    "Cancel".into(),
                ))
                .show(move |replace| {
                    if replace {
                        go(asked);
                    } else {
                        announce(
                            &asked.state::<Mutex<Viewer>>(),
                            "Nothing was exported: the files already there were kept.".into(),
                        );
                        refresh(&asked);
                    }
                });
        });
}

/// Audit B8: what the export asks before writing over files already in the folder.
fn replace_question(first: &Path, count: usize) -> String {
    let name = first
        .file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned());
    match count {
        1 => format!("{name} is already in this folder. Replace it?"),
        n => format!(
            "{name} and {} more files this export writes are already in this folder. Replace them?",
            n - 1
        ),
    }
}

/// The file the open project is, or would be: stored drawing paths are relative to the media
/// root, and D-61 names a never-saved project `project.json`.
fn project_file(viewer: &Viewer) -> PathBuf {
    let name = viewer
        .path
        .as_deref()
        .and_then(Path::file_name)
        .map_or("project.json".into(), |n| n.to_os_string());
    viewer.root.join(name)
}

/// D-61's `project.collect`: write the package into `into` and say what happened. Nothing in the
/// window changes, so there is nothing to refresh but the status line.
fn collect_into(viewer: &Mutex<Viewer>, into: &Path) -> String {
    let held = viewer.lock().expect("the viewer lock was poisoned");
    let written = package::collect(
        held.document.project(),
        &held.preserved,
        Some(&project_file(&held)),
        into,
    );
    drop(held);
    if let Err(diagnostic) = written {
        return diagnostic.sentence();
    }
    // The counts are read back from the manifest just written, so the sentence cannot disagree
    // with it.
    let manifest: serde_json::Value = std::fs::read_to_string(into.join(package::MANIFEST))
        .ok()
        .and_then(|text| serde_json::from_str(&text).ok())
        .unwrap_or_default();
    let count = |status: &str| {
        manifest["assets"]
            .as_array()
            .into_iter()
            .flatten()
            .flat_map(|a| a["files"].as_array().into_iter().flatten())
            .filter(|f| f["status"] == status)
            .count()
    };
    let (copied, missing, excluded) = (count("copied"), count("missing"), count("excluded"));
    let mut said = format!(
        "Collected into {}: {} copied.",
        into.display(),
        drawings(copied)
    );
    if missing > 0 {
        said += &format!(" {} missing, listed without a copy.", were(missing));
    }
    if excluded > 0 {
        said += &format!(" {} left out, marked not to be passed on.", were(excluded));
    }
    said + " The open project is unchanged."
}

fn drawings(n: usize) -> String {
    match n {
        1 => "1 drawing".to_string(),
        n => format!("{n} drawings"),
    }
}

fn were(n: usize) -> String {
    match n {
        1 => "1 drawing was".to_string(),
        n => format!("{n} drawings were"),
    }
}

/// D-61's `package.check`: every listed file that is not as packaged goes to the notes, and the
/// status line gives the count of each answer.
fn check_package(viewer: &Mutex<Viewer>) -> String {
    let mut held = viewer.lock().expect("the viewer lock was poisoned");
    // The saved file itself, not `project_file`: after Save As the media root stays behind in the
    // old folder, and the manifest is beside the file.
    let Some(path) = held.path.clone() else {
        return "This project has no file yet, so there is no package beside it to check. \
                Open the project inside a collected folder first."
            .to_string();
    };
    let rows = match package::check(&path) {
        Ok(rows) => rows,
        Err(diagnostic) => {
            held.notes = vec![diagnostic.note()];
            return diagnostic.sentence();
        }
    };
    held.notes = rows
        .iter()
        .filter_map(|row| {
            row.diagnostic
                .as_ref()
                .map(|d| format!("{}: {}", row.path, d.note()))
        })
        .collect();
    let counts: Vec<String> = [
        Answer::Ok,
        Answer::Changed,
        Answer::Missing,
        Answer::Excluded,
        Answer::Unverified,
    ]
    .into_iter()
    .filter_map(|answer| {
        let n = rows.iter().filter(|r| r.answer == answer).count();
        (n > 0).then(|| format!("{n} {}", answer.as_str()))
    })
    .collect();
    format!(
        "Checked {} {} against the package manifest: {}.",
        rows.len(),
        if rows.len() == 1 { "file" } else { "files" },
        counts.join(", ")
    )
}

/// Ask which folder to collect into. After Effects asks for a new folder; so does this, and a
/// folder with anything in it is refused rather than mixed into.
fn ask_where_to_collect(app: &AppHandle) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Collect the project and its drawings into an empty folder")
        .pick_folder(move |chosen| {
            let Some(into) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let viewer = handle.state::<Mutex<Viewer>>();
            let said = collect_into(&viewer, &into);
            announce(&viewer, said);
            refresh(&handle);
        });
}

/// Ask a running export to stop. It stops between frames, so the file being written finishes.
fn cancel_export(app: &AppHandle) -> String {
    let state = app.state::<Mutex<Export>>();
    let export = state.lock().expect("the export lock was poisoned");
    match &export.cancel {
        Some(flag) => {
            flag.store(true, Ordering::SeqCst);
            "Stopping the export. The frame being written will be finished first.".to_string()
        }
        None => "No export is running.".to_string(),
    }
}

/// D-252: Pause, and Pause again to go on. The export waits before its next frame is written, so
/// what it writes is unchanged; Cancel still stops it while it waits.
fn pause_export(export: &Mutex<Export>) -> String {
    let held = locked(export);
    if held.cancel.is_none() {
        return "No export is running.".into();
    }
    if held.watch.pause.fetch_xor(true, Ordering::SeqCst) {
        "Going on.".into()
    } else {
        "Pausing after the frame being written. Press Pause again to go on.".into()
    }
}

/// D-252: what the page draws of the job being written: the frames it last wrote, the frames so
/// far whose drawing was missing, and whether it is paused.
fn watching(export: &Mutex<Export>) -> serde_json::Value {
    let held = locked(export);
    let frames: Vec<i32> = held
        .watch
        .written
        .lock()
        .expect("the watch lock was poisoned")
        .iter()
        .map(|w| w.0)
        .collect();
    let missing = held
        .watch
        .missing
        .lock()
        .expect("the watch lock was poisoned")
        .clone();
    serde_json::json!({
        "running": held.cancel.is_some(),
        "paused": held.cancel.is_some() && held.watch.pause.load(Ordering::SeqCst),
        "first": held.first,
        "total": held.total,
        "frames": frames,
        "missing": missing,
    })
}

/// D-252: one of the frames the job last wrote, small, as 8-bit sRGB with straight alpha.
fn watched_frame(export: &Mutex<Export>, frame: i32) -> Option<(usize, usize, Vec<u8>)> {
    let watch = Arc::clone(&locked(export).watch);
    let written = watch.written.lock().expect("the watch lock was poisoned");
    written
        .iter()
        .find(|w| w.0 == frame)
        .map(|w| (w.1, w.2, w.3.clone()))
}

// -------------------------------------------------------------------------------------------
// D-251: the render queue
// -------------------------------------------------------------------------------------------

/// One row of the render queue: a job exactly as Export would start it, kept until ▷ Render.
///
/// The job is B-10's snapshot, taken when the row was added, so later edits do not reach it until
/// the person asks for Refresh; the list says when the project has changed since.
struct QueueRow {
    id: u64,
    ticked: bool,
    /// The composition's name, and when the row was added in milliseconds since 1970, which the
    /// page shows as a time of day.
    name: String,
    added: u128,
    job: Arc<(Project, PathBuf, ExportRequest)>,
    /// Queued, Rendering, Done, Stopped, Not started or Failed.
    status: &'static str,
    /// What the row's export did, in the core's words.
    said: String,
    took: u128,
}

/// The row's format as the panel names it, with the choice that was made.
fn format_name(format: OutputFormat, choices: ExportChoices) -> String {
    (match format {
        OutputFormat::Png => "PNG sequence".into(),
        OutputFormat::Exr(ExrSamples::Half) => "EXR sequence, half float".into(),
        OutputFormat::Exr(ExrSamples::Float) => "EXR sequence, full float".into(),
        OutputFormat::Gif if choices.gif_dither => "GIF, dithered".into(),
        OutputFormat::Gif => "GIF".into(),
        OutputFormat::Apng => "Animated PNG".into(),
        OutputFormat::Mp4 => format!("MP4, {:?}", choices.mp4_quality),
    }) + if choices.wav && !matches!(format, OutputFormat::Gif | OutputFormat::Mp4) {
        ", with a WAV"
    } else {
        ""
    }
}

fn locked(export: &Mutex<Export>) -> std::sync::MutexGuard<'_, Export> {
    export.lock().expect("the export lock was poisoned")
}

/// Add the open composition to the queue as Export would write it now. Returns what the status
/// line says.
fn queue_add(
    export: &Mutex<Export>,
    viewer: &Viewer,
    into: &Path,
    missing: MissingSource,
    format: OutputFormat,
    choices: ExportChoices,
) -> String {
    let (project, root, mut request) = export_job(viewer, into, missing, format);
    request.choices = choices;
    let name = project
        .composition(&request.composition)
        .map_or_else(String::new, |c| c.name.clone());
    let said = format!(
        "Added {name} to the render queue{}, into {}. \u{25b7} Render writes it.",
        written_as(format, choices),
        into.display()
    );
    let mut held = locked(export);
    held.changes += 1;
    let id = held.changes;
    held.queue.push(QueueRow {
        id,
        ticked: true,
        name,
        added: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_or(0, |d| d.as_millis()),
        job: Arc::new((project, root, request)),
        status: "Queued",
        said: String::new(),
        took: 0,
    });
    said
}

/// The queue as the Render panel draws it. `stale` is true when the open project is no longer
/// the one the row copied.
fn queue_list(export: &Mutex<Export>, viewer: &Mutex<Viewer>) -> serde_json::Value {
    let viewer = viewer.lock().expect("the viewer lock was poisoned");
    let held = locked(export);
    held.queue
        .iter()
        .map(|row| {
            let (project, _, request) = &*row.job;
            serde_json::json!({
                "id": row.id,
                "ticked": row.ticked,
                "name": row.name,
                "format": format_name(request.format, request.choices),
                "frames": request.last_frame - request.first_frame + 1,
                "output": request.output_dir.display().to_string(),
                "added": row.added as u64,
                "status": row.status,
                "said": row.said,
                "took": row.took as u64,
                "running": held.row == Some(row.id),
                "stale": project != viewer.document.project(),
            })
        })
        .collect()
}

/// Tick, remove, move or refresh one row, `?do=&row=`. Returns what the status line says, or
/// nothing for a tick.
fn queue_change(export: &Mutex<Export>, viewer: &Mutex<Viewer>, query: Option<&str>) -> String {
    let id = parameter(query, "row")
        .and_then(|r| r.parse::<u64>().ok())
        .unwrap_or(0);
    let what = parameter(query, "do");
    // Taken before the queue's lock, in the order queue_list takes them.
    let project = (what.as_deref() == Some("refresh")).then(|| {
        let viewer = viewer.lock().expect("the viewer lock was poisoned");
        (viewer.document.project().clone(), viewer.root.clone())
    });
    let mut held = locked(export);
    let running = held.cancel.is_some();
    let Some(at) = held.queue.iter().position(|r| r.id == id) else {
        return "That row is no longer in the render queue.".into();
    };
    let said = match what.as_deref() {
        Some("tick") => {
            held.queue[at].ticked = parameter(query, "on").as_deref() != Some("0");
            String::new()
        }
        Some("remove") if running => {
            "The render queue is rendering. Stop it before removing a row.".into()
        }
        Some("remove") => {
            let row = held.queue.remove(at);
            format!("Removed {} from the render queue.", row.name)
        }
        Some("move") => {
            let row = held.queue.remove(at);
            let to = parameter(query, "to")
                .and_then(|t| t.parse::<usize>().ok())
                .unwrap_or(at)
                .min(held.queue.len());
            held.queue.insert(to, row);
            String::new()
        }
        Some("refresh") if running => {
            "The render queue is rendering. Stop it before refreshing a row.".into()
        }
        Some("refresh") => {
            let (project, root) = project.expect("taken above for a refresh");
            let row = &mut held.queue[at];
            let old = &row.job.2;
            let Some((first, last)) = project
                .composition(&old.composition)
                .map(|c| c.work_frames())
            else {
                return format!(
                    "{} is no longer in the project, so its row cannot be refreshed.",
                    row.name
                );
            };
            let request = ExportRequest {
                composition: old.composition.clone(),
                first_frame: first,
                last_frame: last,
                output_dir: old.output_dir.clone(),
                naming: old.naming.clone(),
                depth: old.depth,
                alpha: old.alpha,
                tile_size: old.tile_size,
                missing: old.missing,
                format: old.format,
                choices: old.choices,
            };
            row.name = project
                .composition(&request.composition)
                .map_or_else(String::new, |c| c.name.clone());
            row.job = Arc::new((project, root, request));
            format!(
                "{} in the render queue now has the project as it is.",
                row.name
            )
        }
        _ => "The render queue can tick, remove, move or refresh a row.".into(),
    };
    held.changes += 1;
    said
}

/// Write every ticked row in turn, as Export would write each, and say how it went.
///
/// Stop is `cancel`, read between frames as Export reads it: the row being written keeps the
/// frames that finished, and the rows after it are not started.
fn render_queue(export: &Mutex<Export>, cancel: &AtomicBool) -> String {
    let ids: Vec<u64> = {
        let mut held = locked(export);
        held.changes += 1;
        for row in held.queue.iter_mut().filter(|r| r.ticked) {
            row.status = "Queued";
            row.said.clear();
            row.took = 0;
        }
        held.queue
            .iter()
            .filter(|r| r.ticked)
            .map(|r| r.id)
            .collect()
    };
    if ids.is_empty() {
        return "Nothing in the render queue is ticked.".into();
    }
    let mut rendered = 0;
    let mut stopped = None;
    for (n, id) in ids.iter().enumerate() {
        let (job, done, watch) = {
            let mut held = locked(export);
            held.changes += 1;
            let Some(row) = held.queue.iter_mut().find(|r| r.id == *id) else {
                continue;
            };
            if stopped.is_some() || cancel.load(Ordering::SeqCst) {
                row.status = "Not started";
                stopped.get_or_insert(n);
                continue;
            }
            row.status = "Rendering";
            let job = Arc::clone(&row.job);
            let said = format!(
                "Rendering row {} of {}, {}{}.",
                n + 1,
                ids.len(),
                row.name,
                written_as(job.2.format, job.2.choices).trim_end_matches(',')
            );
            held.row = Some(*id);
            held.done = Arc::new(AtomicUsize::new(0));
            held.total = (job.2.last_frame - job.2.first_frame + 1).max(0) as usize;
            held.started = Some(Instant::now());
            held.said = said;
            held.watch = Arc::new(export::Watch::default());
            held.first = job.2.first_frame;
            (job, Arc::clone(&held.done), Arc::clone(&held.watch))
        };
        let (project, root, request) = &*job;
        let began = Instant::now();
        let report = export::export_sequence_watched(project, root, request, cancel, &done, &watch);
        let mut held = locked(export);
        held.changes += 1;
        held.row = None;
        if let Some(row) = held.queue.iter_mut().find(|r| r.id == *id) {
            row.said = what_the_export_did(&report, &request.output_dir);
            row.took = began.elapsed().as_millis();
            row.status = match report.status {
                _ if report.succeeded() => {
                    row.ticked = false;
                    rendered += 1;
                    "Done"
                }
                ExportStatus::Cancelled => "Stopped",
                _ => "Failed",
            };
        }
        if report.status == ExportStatus::Cancelled {
            stopped = Some(n);
        }
    }
    match stopped {
        Some(n) => format!("Stopped during row {} of {}.", n + 1, ids.len()),
        None => format!("Rendered {rendered} of {} rows.", ids.len()),
    }
}

/// ▷ Render with rows in the queue: write them on a thread, as Export does, or say why not.
fn start_queue(app: &AppHandle) -> String {
    let state = app.state::<Mutex<Export>>();
    let cancel = Arc::new(AtomicBool::new(false));
    let said = {
        let mut export = locked(&state);
        if export.cancel.is_some() {
            return "An export is already running. Cancel it, or wait for it to finish."
                .to_string();
        }
        let ticked = export.queue.iter().filter(|r| r.ticked).count();
        if ticked == 0 {
            return "Nothing in the render queue is ticked.".to_string();
        }
        export.cancel = Some(Arc::clone(&cancel));
        export.said = format!(
            "Rendering the {ticked} ticked {} of the render queue. The window stays usable while it writes.",
            if ticked == 1 { "row" } else { "rows" }
        );
        export.said.clone()
    };
    announce(&app.state::<Mutex<Viewer>>(), said.clone());
    let handle = app.clone();
    let started = said.clone();
    std::thread::spawn(move || {
        let state = handle.state::<Mutex<Export>>();
        let done = contained(&handle, "rendering the queue", || {
            render_queue(&state, &cancel)
        })
        .unwrap_or_else(|said| said);
        {
            let mut export = locked(&state);
            export.cancel = None;
            export.said = done;
            export.changes += 1;
        }
        let viewer = handle.state::<Mutex<Viewer>>();
        let mut viewer = viewer.lock().expect("the viewer lock was poisoned");
        if viewer.status == started {
            viewer.status = EXPORT_ENDED.to_string();
        }
    });
    said
}

/// `/queue`: the list, after `?do=` if one is asked. `add` asks for a folder first, as Export does.
fn queue(app: &AppHandle, query: Option<&str>) -> serde_json::Value {
    let viewer = app.state::<Mutex<Viewer>>();
    let export = app.state::<Mutex<Export>>();
    let said = match parameter(query, "do").as_deref() {
        None => String::new(),
        Some("add") => match output_format(query).and_then(|f| Ok((f, export_choices(query)?))) {
            Ok((format, choices)) => {
                let missing = missing_source(query);
                let handle = app.clone();
                app.dialog()
                    .file()
                    .set_title("Render this row into a folder")
                    .pick_folder(move |chosen| {
                        let Some(into) = chosen.and_then(|c| c.into_path().ok()) else {
                            return;
                        };
                        let said = {
                            let viewer = handle.state::<Mutex<Viewer>>();
                            let held = viewer.lock().expect("the viewer lock was poisoned");
                            queue_add(
                                &handle.state::<Mutex<Export>>(),
                                &held,
                                &into,
                                missing,
                                format,
                                choices,
                            )
                        };
                        announce(&handle.state::<Mutex<Viewer>>(), said);
                        refresh(&handle);
                    });
                String::new()
            }
            Err(refused) => refused.replacen("Nothing was exported", "Nothing was added", 1),
        },
        Some("render") => start_queue(app),
        Some(_) => queue_change(&export, &viewer, query),
    };
    if !said.is_empty() {
        announce(&viewer, said);
    }
    queue_list(&export, &viewer)
}

// -------------------------------------------------------------------------------------------
// The recent list
// -------------------------------------------------------------------------------------------

/// How many projects the recent list remembers. Long enough to cover a working day's shots,
/// short enough to read without scrolling.
const RECENT_LIMIT: usize = 8;

/// The recent list after opening `opened`: most recent first, no duplicates, capped.
///
/// Pure, so it can be checked without a window. Comparison is case-insensitive because this is
/// a Windows-only product per ADR-001 and `C:\Shots\a.json` and `c:\shots\A.JSON` are one file;
/// two entries for it would be two ways to open the same project and one of them would look
/// like a different one.
fn remembered(existing: &[String], opened: &str) -> Vec<String> {
    let mut list = vec![opened.to_string()];
    for entry in existing {
        if !entry.eq_ignore_ascii_case(opened) && !entry.trim().is_empty() {
            list.push(entry.clone());
        }
    }
    list.truncate(RECENT_LIMIT);
    list
}

/// Where the recent list is kept: one absolute path per line, in the application's own
/// configuration directory.
///
/// Beside the application rather than beside a project, because it is about the person and not
/// about any one shot. Plain text rather than JSON because a path cannot contain a newline on
/// this platform, so lines are enough, and because a person who wants to clear the list should
/// be able to look at the file and see what it is.
fn recent_file(app: &AppHandle) -> Option<PathBuf> {
    let directory = app.path().app_config_dir().ok()?;
    std::fs::create_dir_all(&directory).ok()?;
    Some(directory.join("recent.txt"))
}

/// The recent list, newest first, with everything that is no longer on disk left out.
///
/// Offering a file that has been moved or deleted would turn the list into a source of failed
/// opens; a project that comes back is simply offered again the next time it is opened.
fn recent(app: &AppHandle) -> Vec<String> {
    let Some(file) = recent_file(app) else {
        return Vec::new();
    };
    std::fs::read_to_string(file)
        .unwrap_or_default()
        .lines()
        .filter(|line| !line.trim().is_empty() && Path::new(line).is_file())
        .map(str::to_string)
        .collect()
}

/// Put `path` at the top of the recent list. A failure to write it is not worth telling anyone
/// about: the list is a convenience and nothing depends on it.
fn remember(app: &AppHandle, path: &Path) {
    let Some(file) = recent_file(app) else { return };
    let absolute = std::fs::canonicalize(path)
        .unwrap_or_else(|_| path.to_path_buf())
        .display()
        .to_string();
    // Windows canonicalisation returns the \\?\ form, which is correct and unreadable. The list
    // is shown to a person, so the prefix comes off; it is not part of the path's identity.
    let absolute = absolute
        .strip_prefix(r"\\?\")
        .unwrap_or(&absolute)
        .to_string();
    let list = remembered(&recent(app), &absolute);
    let _ = std::fs::write(file, list.join("\n"));
}

// -------------------------------------------------------------------------------------------
// The window's place (B-174, D-247)
// -------------------------------------------------------------------------------------------

/// Where the window was when it last closed: its outer corner and inner size in pixels, and
/// whether it was maximized. While it is maximized, the corner and size are where Restore goes.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Placement {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
    maximized: bool,
}

/// A display's work area (the screen less the taskbar): left, top, width, height in pixels.
type Area = (i32, i32, u32, u32);

/// Remembered sizes outside this, in pixels, are taken for a damaged file: a window too small to
/// see, or bigger than any display made.
const WINDOW_SIZES: std::ops::RangeInclusive<u32> = 200..=100_000;

/// Beside the recent list, for the same reason: it is about the person, not about any one shot.
fn window_file(app: &AppHandle) -> Option<PathBuf> {
    Some(recent_file(app)?.with_file_name("window.txt"))
}

/// `window.txt` read: one line, `x y width height normal` or `... maximized`. Anything else is
/// `None`, so a damaged file opens the window as a first launch does rather than somewhere odd.
fn placement_from_text(text: &str) -> Option<Placement> {
    let words: Vec<&str> = text.split_whitespace().collect();
    let [x, y, width, height, state] = words[..] else {
        return None;
    };
    let (width, height) = (width.parse().ok()?, height.parse().ok()?);
    if !WINDOW_SIZES.contains(&width) || !WINDOW_SIZES.contains(&height) {
        return None;
    }
    let maximized = match state {
        "maximized" => true,
        "normal" => false,
        _ => return None,
    };
    Some(Placement {
        x: x.parse().ok()?,
        y: y.parse().ok()?,
        width,
        height,
        maximized,
    })
}

fn placement_text(p: Placement) -> String {
    format!(
        "{} {} {} {} {}\n",
        p.x,
        p.y,
        p.width,
        p.height,
        if p.maximized { "maximized" } else { "normal" }
    )
}

/// Where the window opens. A remembered place is kept while the middle of its title bar is on a
/// display that is still attached, no bigger than that display; anything else (the very first
/// launch, a damaged file, a display since unplugged) opens maximized on the main display, with
/// Restore going to three quarters of it, centred.
fn placement_on_screen(saved: Option<Placement>, areas: &[Area], main: Area) -> Placement {
    if let Some(p) = saved {
        if let Some(&(_, _, w, h)) = areas.iter().find(|a| holds_title_bar(**a, p)) {
            return Placement {
                width: p.width.min(w),
                height: p.height.min(h),
                ..p
            };
        }
    }
    let (x, y, w, h) = main;
    let (width, height) = (w / 4 * 3, h / 4 * 3);
    Placement {
        x: x + ((w - width) / 2) as i32,
        y: y + ((h - height) / 2) as i32,
        width,
        height,
        maximized: true,
    }
}

/// Whether the middle of the window's title bar is in the display area.
fn holds_title_bar((x, y, w, h): Area, p: Placement) -> bool {
    let (px, py) = (p.x as i64 + p.width as i64 / 2, p.y as i64 + 16);
    (x as i64..x as i64 + w as i64).contains(&px) && (y as i64..y as i64 + h as i64).contains(&py)
}

/// Makes the window where it was last closed. Made here rather than in tauri.conf.json so that it
/// is made at its place and size and shown at once, as before, rather than made at a default size
/// and moved once the page has loaded.
fn open_window(app: &AppHandle) -> tauri::Result<()> {
    let area = |m: &tauri::Monitor| {
        let a = m.work_area();
        (a.position.x, a.position.y, a.size.width, a.size.height)
    };
    let monitors = app.available_monitors().unwrap_or_default();
    let areas: Vec<Area> = monitors.iter().map(area).collect();
    let primary = app.primary_monitor().ok().flatten();
    let mut window = tauri::WebviewWindowBuilder::new(app, "main", tauri::WebviewUrl::default())
        .title("Anime Compositor")
        .inner_size(1000.0, 640.0);
    if let Some(main) = primary.as_ref().map(area).or(areas.first().copied()) {
        let saved = window_file(app)
            .and_then(|f| std::fs::read_to_string(f).ok())
            .and_then(|t| placement_from_text(&t));
        let p = placement_on_screen(saved, &areas, main);
        // The builder takes points: the display the window opens on says how many pixels one is.
        let scale = monitors
            .iter()
            .find(|m| holds_title_bar(area(m), p))
            .or(primary.as_ref())
            .map_or(1.0, |m| m.scale_factor());
        window = window
            .position(p.x as f64 / scale, p.y as f64 / scale)
            .inner_size(p.width as f64 / scale, p.height as f64 / scale)
            .maximized(p.maximized);
        let normal = Some(Placement {
            maximized: false,
            ..p
        });
        *app.state::<Mutex<[Option<Placement>; 2]>>()
            .lock()
            .expect("the placement lock was poisoned") = [normal, normal];
    }
    window.build()?;
    Ok(())
}

/// Called as the window moves, changes size and closes: keeps the last place it had while
/// neither maximized nor minimized, and writes it out with the maximized state on closing.
///
/// Windows reports the move into the maximized place before it says the window is maximized, so
/// that one move is taken as a normal place; the place before it is kept, and put back when the
/// size change that follows says maximized or minimized.
fn follow_window(window: &tauri::Window, event: &WindowEvent) {
    let places = window.state::<Mutex<[Option<Placement>; 2]>>();
    let mut places = places.lock().expect("the placement lock was poisoned");
    let [last, before] = &mut *places;
    match event {
        WindowEvent::Moved(_) | WindowEvent::Resized(_) => {
            if window.is_maximized().unwrap_or(true) || window.is_minimized().unwrap_or(true) {
                if matches!(event, WindowEvent::Resized(_)) {
                    *last = *before;
                }
                return;
            }
            if let (Ok(at), Ok(size)) = (window.outer_position(), window.inner_size()) {
                *before = *last;
                *last = Some(Placement {
                    x: at.x,
                    y: at.y,
                    width: size.width,
                    height: size.height,
                    maximized: false,
                });
            }
        }
        WindowEvent::CloseRequested { .. } => {
            let Some(p) = *last else { return };
            let Some(file) = window_file(window.app_handle()) else {
                return;
            };
            // Like the recent list, a convenience: failing to write it is not worth a word.
            let _ = std::fs::write(
                file,
                placement_text(Placement {
                    maximized: window.is_maximized().unwrap_or(false),
                    ..p
                }),
            );
        }
        _ => {}
    }
}

// -------------------------------------------------------------------------------------------
// The commands
// -------------------------------------------------------------------------------------------

/// The value of `name` in a query string, percent-decoded.
/// Every value given for one parameter, in the order they appear.
///
/// [`parameter`] takes the first, which is what a setting with one value wants. An import names
/// one file per value: a selection is a list, and joining it into a single string would need a
/// separator, which is a character a Japanese file name is entitled to contain.
fn parameters(query: Option<&str>, name: &str) -> Vec<String> {
    let prefix = format!("{name}=");
    query
        .into_iter()
        .flat_map(|q| q.split('&'))
        .filter_map(|pair| pair.strip_prefix(&prefix))
        .map(from_a_query)
        .collect()
}

/// Whether this export writes the frames whose drawings are missing.
///
/// Document 07's default is that a missing drawing blocks a final export. `?missing=write` is the
/// person overriding it in front of the checkbox that says what it does, which is document 28's
/// recorded override rather than a silent fallback. Written down as its own function because it
/// is the one place in this file where a query string decides what reaches the disk: everything
/// that is not exactly `write` blocks, and that has a table row of its own.
fn missing_source(query: Option<&str>) -> MissingSource {
    match parameter(query, "missing").as_deref() {
        Some("write") => MissingSource::RenderTransparent,
        _ => MissingSource::Block,
    }
}

/// B-23c: a solid's `color` (three linear numbers, as `r,g,b`), `width` and `height` from the
/// query, each kept from `solid` when it is not sent. Only what is not a number at all is
/// refused here; a number outside D-74's ranges is the core's to refuse, in its own sentence.
fn solid_from(query: Option<&str>, mut solid: Solid) -> Result<Solid, String> {
    if let Some(text) = parameter(query, "color") {
        let numbers: Vec<f64> = text
            .split(',')
            .filter_map(|n| n.trim().parse::<f64>().ok())
            .collect();
        solid.color = <[f64; 3]>::try_from(numbers).map_err(|_| {
            format!("\"{text}\" is not a colour. A colour is three numbers from 0 to 1, red, green and blue.")
        })?;
    }
    for (name, side) in [("width", &mut solid.width), ("height", &mut solid.height)] {
        if let Some(text) = parameter(query, name) {
            *side = text.trim().parse::<u32>().map_err(|_| {
                format!(
                    "\"{}\" is not a {name}. A {name} is a whole number of pixels, from 1 to {}.",
                    text.trim(),
                    Solid::MAX_SIDE
                )
            })?;
        }
    }
    Ok(solid)
}

/// D-263: a text layer's `text`, `font`, `size`, `color` (`r,g,b`, linear), `x`, `y` and `align`
/// from the query, each kept from `words` when it is not sent. Only what is not a number at all
/// is refused here; a number outside D-263's ranges is the core's to refuse.
///
/// D-264: `tracking`, `leading`, `box`, `kerning`, `caps`, `bold` and `italic` (`on` or `off`),
/// and the stroke, background and shadow: `stroke=off` takes the stroke away and `stroke=on`
/// or any of `stroke_color` and `stroke_width` gives one, starting from black a twentieth of the
/// size wide; `bg` with `bg_color`, `bg_opacity`, `bg_padding` and `bg_round`, and `shadow` with
/// `shadow_color`, `shadow_opacity`, `shadow_angle`, `shadow_distance` and `shadow_softness`, the
/// same way.
fn text_from(
    query: Option<&str>,
    mut words: anime_compositor::text::Text,
) -> Result<anime_compositor::text::Text, String> {
    use anime_compositor::text::{Background, Shadow, Stroke};
    if let Some(text) = parameter(query, "text") {
        words.text = text;
    }
    if let Some(font) = parameter(query, "font") {
        words.font = font.trim().to_string();
    }
    let colour = |name: &str, into: &mut [f64; 3]| -> Result<(), String> {
        if let Some(text) = parameter(query, name) {
            let numbers: Vec<f64> = text
                .split(',')
                .filter_map(|n| n.trim().parse::<f64>().ok())
                .collect();
            *into = <[f64; 3]>::try_from(numbers).map_err(|_| {
                format!("\"{text}\" is not a colour. A colour is three numbers from 0 to 1, red, green and blue.")
            })?;
        }
        Ok(())
    };
    let number = |name: &str, into: &mut f64| -> Result<(), String> {
        if let Some(text) = parameter(query, name) {
            *into = text
                .trim()
                .parse::<f64>()
                .ok()
                .filter(|v| v.is_finite())
                .ok_or_else(|| format!("\"{}\" is not a number.", text.trim()))?;
        }
        Ok(())
    };
    let switch = |name: &str, into: &mut bool| -> Result<(), String> {
        if let Some(text) = parameter(query, name) {
            *into = match text.trim() {
                "on" => true,
                "off" => false,
                other => return Err(format!("{name} is on or off, not \"{other}\".")),
            };
        }
        Ok(())
    };
    colour("color", &mut words.color)?;
    let [x, y] = &mut words.at;
    for (name, value) in [("size", &mut words.size), ("x", x), ("y", y)] {
        if let Some(text) = parameter(query, name) {
            *value = text
                .trim()
                .parse::<f64>()
                .ok()
                .filter(|v| v.is_finite())
                .ok_or_else(|| format!("\"{}\" is not a number of pixels.", text.trim()))?;
        }
    }
    if let Some(text) = parameter(query, "align") {
        words.align = anime_compositor::text::Align::parse(text.trim()).ok_or_else(|| {
            format!("\"{text}\" is not an alignment: left, center, right or justify.")
        })?;
    }
    number("tracking", &mut words.tracking)?;
    number("leading", &mut words.leading)?;
    number("box", &mut words.box_width)?;
    switch("kerning", &mut words.kerning)?;
    switch("caps", &mut words.all_caps)?;
    switch("bold", &mut words.faux_bold)?;
    switch("italic", &mut words.faux_italic)?;
    // A record is asked for by its switch or by any of its settings, and starts from the
    // editors' usual look at this size.
    let size = words.size;
    let wanted = |name: &str, parts: &[&str]| -> Result<Option<bool>, String> {
        match parameter(query, name).as_deref().map(str::trim) {
            Some("on") => Ok(Some(true)),
            Some("off") => Ok(Some(false)),
            Some(other) => Err(format!("{name} is on or off, not \"{other}\".")),
            None => Ok(parts
                .iter()
                .any(|p| parameter(query, &format!("{name}_{p}")).is_some())
                .then_some(true)),
        }
    };
    match wanted("stroke", &["color", "width"])? {
        Some(false) => words.stroke = None,
        Some(true) => {
            let mut s = words.stroke.take().unwrap_or(Stroke {
                color: [0.0; 3],
                width: (size * 0.05).round().max(1.0),
            });
            colour("stroke_color", &mut s.color)?;
            number("stroke_width", &mut s.width)?;
            words.stroke = Some(s);
        }
        None => {}
    }
    match wanted("bg", &["color", "opacity", "padding", "round"])? {
        Some(false) => words.background = None,
        Some(true) => {
            let mut b = words.background.take().unwrap_or(Background {
                color: [0.0; 3],
                opacity: 0.75,
                padding: (size * 0.2).round(),
                roundness: 0.0,
            });
            colour("bg_color", &mut b.color)?;
            number("bg_opacity", &mut b.opacity)?;
            number("bg_padding", &mut b.padding)?;
            number("bg_round", &mut b.roundness)?;
            words.background = Some(b);
        }
        None => {}
    }
    match wanted(
        "shadow",
        &["color", "opacity", "angle", "distance", "softness"],
    )? {
        Some(false) => words.shadow = None,
        Some(true) => {
            let mut s = words.shadow.take().unwrap_or(Shadow {
                color: [0.0; 3],
                opacity: 0.75,
                angle: 135.0,
                distance: (size * 0.05).round().max(1.0),
                softness: (size * 0.05).round(),
            });
            colour("shadow_color", &mut s.color)?;
            number("shadow_opacity", &mut s.opacity)?;
            number("shadow_angle", &mut s.angle)?;
            number("shadow_distance", &mut s.distance)?;
            number("shadow_softness", &mut s.softness)?;
            words.shadow = Some(s);
        }
        None => {}
    }
    Ok(words)
}

/// B-24e: one key on a mask's or a shape's path, named by its frame in `key`, moved along its
/// row to `to`, given `interp` - hold, linear or ease, with `curve`'s four numbers - or eased by
/// F9's rule in `ease`: `both` sides, the side it is reached `in` by, or the side it leaves `out`
/// by, each side keeping the other half of its segment's curve and a hold left a hold, exactly as
/// the page's F9 eases a property's keys. A key moved onto another takes its place, as After
/// Effects' does; the core puts the keys back in frame order. Nothing named, nothing done.
fn path_key(
    query: Option<&str>,
    keys: &mut Vec<anime_compositor::mask::MaskKey>,
) -> Result<(), String> {
    if parameter(query, "key").is_none() {
        return Ok(());
    }
    let frame = frame_parameter(query, "key")?;
    let Some(i) = keys.iter().position(|k| k.frame == frame) else {
        return Err(format!("This path has no key at frame {frame}."));
    };
    if parameter(query, "interp").is_some() {
        keys[i].interp = interp_named(query, "interp")?;
    }
    if let Some(sides) = parameter(query, "ease") {
        let Interp::Ease {
            x1: ex,
            y1: ey,
            x2: fx,
            y2: fy,
        } = Interp::EASY
        else {
            unreachable!("easy ease is an ease")
        };
        // The curve a segment has now: its own, or the one that is exactly straight.
        let curve = |k: &anime_compositor::mask::MaskKey| match k.interp {
            Interp::Ease { x1, y1, x2, y2 } => [x1, y1, x2, y2],
            _ => [1.0 / 3.0, 1.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0],
        };
        let (out, into) = match sides.as_str() {
            "both" => (true, true),
            "out" => (true, false),
            "in" => (false, true),
            other => return Err(format!("F9 eases both, in or out. Not \"{other}\".")),
        };
        if out && i + 1 < keys.len() && keys[i].interp != Interp::Hold {
            let [_, _, x2, y2] = curve(&keys[i]);
            keys[i].interp = Interp::Ease {
                x1: ex,
                y1: ey,
                x2,
                y2,
            };
        }
        if into && i > 0 && keys[i - 1].interp != Interp::Hold {
            let [x1, y1, _, _] = curve(&keys[i - 1]);
            keys[i - 1].interp = Interp::Ease {
                x1,
                y1,
                x2: fx,
                y2: fy,
            };
        }
    }
    if parameter(query, "to").is_some() {
        let mut moved = keys.remove(i);
        moved.frame = frame_parameter(query, "to")?;
        keys.retain(|k| k.frame != moved.frame);
        keys.push(moved);
    }
    Ok(())
}

/// B-24f: `mask:<n>` and `shape:<n>` name a path's keys wherever a request names a property, as
/// `fx:` names an effect's setting, so that the timeline's choice, the arrow keys, Delete, the
/// key menu, F9, copy and paste and the graph reach a path's keys through the requests they
/// already send. `true` is a shape.
fn path_prop(prop: &str) -> Option<(bool, usize)> {
    let (kind, at) = prop.split_once(':')?;
    let shape = match kind {
        "mask" => false,
        "shape" => true,
        _ => return None,
    };
    Some((shape, at.parse().ok()?))
}

/// B-24f: the path `which` names on a layer handed to `change`, base and keys, and the layer's
/// masks or shapes sent back whole, as the one command the core has for them.
fn with_path(
    composition: Id,
    layer: &Layer,
    (shape, at): (bool, usize),
    change: impl FnOnce(
        &mut Vec<anime_compositor::mask::MaskPoint>,
        &mut Vec<anime_compositor::mask::MaskKey>,
    ) -> Result<(), String>,
) -> Result<Command, String> {
    let (mut masks, mut shapes) = (layer.masks.clone(), layer.shapes.clone());
    let path = if shape {
        shapes.get_mut(at).map(|s| (&mut s.points, &mut s.keys))
    } else {
        masks.get_mut(at).map(|m| (&mut m.points, &mut m.keys))
    };
    let Some((base, keys)) = path else {
        return Err(format!(
            "{} has no {} {at}.",
            layer.name,
            if shape { "shape" } else { "mask" }
        ));
    };
    change(base, keys)?;
    let layer_id = layer.id.clone();
    Ok(if shape {
        Command::SetShapes {
            composition,
            layer_id,
            shapes,
            groups: None,
        }
    } else {
        Command::SetMasks {
            composition,
            layer_id,
            masks,
        }
    })
}

/// D-741: whether a property name is one of a layer's masks', shapes' or groups', whose keys go
/// back as the layer's masks or shapes whole rather than one key at a time.
fn layer_keyed(prop: &str) -> bool {
    path_prop(prop).is_some()
        || shape_prop(prop).is_some()
        || mask_prop(prop).is_some()
        || group_prop(prop).is_some()
}

/// D-741: the chosen keys of a layer's paths and of its masks', shapes' and groups' numbers, by
/// property, each as the frame it is on and the one it goes to, moved (`keyframe.move`), taken
/// out (`keyframe.add_remove`) or reversed (`keyframe.reverse`) in one copy of the layer's masks,
/// shapes and groups, and sent back as one command for each, so that keys of several of them on
/// one layer go together. The last key of a path or number taken out leaves it as the first
/// chosen key of it held.
fn layer_keys_commands(
    id: &str,
    composition: &Id,
    layer: &Layer,
    chosen: &std::collections::BTreeMap<String, Vec<(i32, i32)>>,
) -> Result<Vec<Command>, String> {
    let (mut masks, mut shapes, mut groups) = (
        layer.masks.clone(),
        layer.shapes.clone(),
        layer.shape_groups.clone(),
    );
    let (mut masks_changed, mut shapes_changed, mut groups_changed) = (false, false, false);
    let no = |prop: &str| format!("{} has no {prop}.", layer.name);
    for (prop, moves) in chosen {
        if let Some((shape, at)) = path_prop(prop) {
            let path = if shape {
                shapes.get_mut(at).map(|s| (&mut s.points, &mut s.keys))
            } else {
                masks.get_mut(at).map(|m| (&mut m.points, &mut m.keys))
            };
            let (base, keys) = path.ok_or_else(|| no(prop))?;
            let new = retimed(
                id,
                &keys.iter().map(|k| (k.frame, k.interp)).collect::<Vec<_>>(),
                moves,
                "This path",
            )?;
            let first = keys
                .iter()
                .find(|k| moves.iter().any(|m| m.0 == k.frame))
                .cloned();
            *keys = std::mem::take(keys)
                .into_iter()
                .zip(new)
                .filter_map(|(k, new)| {
                    new.map(|(frame, interp)| anime_compositor::mask::MaskKey {
                        frame,
                        interp,
                        ..k
                    })
                })
                .collect();
            keys.sort_by_key(|k| k.frame);
            if let (true, Some(first)) = (keys.is_empty(), first) {
                *base = first.points;
            }
            if shape {
                shapes_changed = true;
            } else {
                masks_changed = true;
            }
        } else if let Some((at, name)) = shape_prop(prop) {
            let shape = shapes.get_mut(at).ok_or_else(|| no(prop))?;
            let mut channels = shape.channels(name).ok_or_else(|| no(prop))?;
            retime_channels(id, &mut channels, moves)?;
            shape.set_channels(name, channels);
            shapes_changed = true;
        } else if let Some((at, name)) = mask_prop(prop) {
            let mask = masks.get_mut(at).ok_or_else(|| no(prop))?;
            let mut channels = vec![mask.channel(name).ok_or_else(|| no(prop))?];
            retime_channels(id, &mut channels, moves)?;
            mask.set_channel(name, channels.remove(0));
            masks_changed = true;
        } else if let Some((at, name, part)) = group_prop(prop) {
            let numbers = groups.get_mut(at).and_then(|g| match part {
                GroupPart::Transform => Some(&mut g.transform),
                GroupPart::Repeater => g.repeater.as_mut().map(|r| &mut r.numbers),
                GroupPart::Operation(k) => g.operations.get_mut(k).map(|o| &mut o.numbers),
            });
            let number = numbers
                .and_then(|n| n.get_mut(name))
                .ok_or_else(|| no(prop))?;
            let mut channels = vec![number.clone()];
            retime_channels(id, &mut channels, moves)?;
            *number = channels.remove(0);
            groups_changed = true;
        } else {
            return Err(no(prop));
        }
    }
    let mut commands = Vec::new();
    if masks_changed {
        commands.push(Command::SetMasks {
            composition: composition.clone(),
            layer_id: layer.id.clone(),
            masks,
        });
    }
    if shapes_changed || groups_changed {
        commands.push(Command::SetShapes {
            composition: composition.clone(),
            layer_id: layer.id.clone(),
            shapes,
            groups: groups_changed.then_some(groups),
        });
    }
    Ok(commands)
}

/// D-741: [`retimed`] on a number's channels, which are keyed together. A reversed key's path
/// handles swap ends, as a property's do.
fn retime_channels(
    id: &str,
    channels: &mut [Property],
    moves: &[(i32, i32)],
) -> Result<(), String> {
    let old = channels[0].keyframes().to_vec();
    let new = retimed(
        id,
        &old.iter().map(|k| (k.frame, k.interp)).collect::<Vec<_>>(),
        moves,
        "This number",
    )?;
    let first = old
        .iter()
        .position(|k| moves.iter().any(|m| m.0 == k.frame));
    for channel in channels.iter_mut() {
        let keys = channel.keyframes().to_vec();
        for k in &keys {
            channel.remove_keyframe(k.frame);
        }
        for (k, new) in keys.iter().zip(&new) {
            let Some((frame, interp)) = *new else {
                continue;
            };
            let turn = id == "keyframe.reverse" && moves.iter().any(|m| m.0 == k.frame);
            let spatial = match turn {
                true => k.spatial.map(|[ix, iy, ox, oy]| [ox, oy, ix, iy]),
                false => k.spatial,
            };
            channel.set_keyframe(Keyframe {
                frame,
                interp,
                spatial,
                ..*k
            });
        }
        if let (true, Some(i)) = (channel.keyframes().is_empty(), first) {
            channel.set_base(keys[i].value);
        }
    }
    Ok(())
}

/// D-741: each of a list's keys, as (frame, ease) in order, after the chosen ones, `(on, to)`
/// sorted by `on`, are moved, reversed by [`reversed`], or taken out, `None`. A key put onto one
/// that is not chosen is refused. `this` is what the keys are on, said in a refusal.
fn retimed(
    id: &str,
    keys: &[(i32, Interp)],
    moves: &[(i32, i32)],
    this: &str,
) -> Result<Vec<Option<(i32, Interp)>>, String> {
    if let Some((at, _)) = moves.iter().find(|(at, _)| keys.iter().all(|k| k.0 != *at)) {
        return Err(format!("{this} has no key at frame {at}."));
    }
    let to = |frame: i32| moves.iter().find(|m| m.0 == frame).map(|m| m.1);
    let new: Vec<Option<(i32, Interp)>> = match id {
        "keyframe.reverse" => reversed(keys, moves)?.into_iter().map(Some).collect(),
        "keyframe.add_remove" => keys
            .iter()
            .map(|k| to(k.0).is_none().then_some(*k))
            .collect(),
        _ => keys
            .iter()
            .map(|k| Some((to(k.0).unwrap_or(k.0), k.1)))
            .collect(),
    };
    let mut landed: Vec<i32> = new.iter().flatten().map(|k| k.0).collect();
    landed.sort();
    if let Some(on) = landed.windows(2).find(|w| w[0] == w[1]) {
        return Err(format!(
            "{this} would have a second key at frame {}, where a key that is not chosen is. \
             Choose it too, or move it first.",
            on[0]
        ));
    }
    Ok(new)
}

/// B-24f: the three single-key requests about a path's key - the diamond, a move and an ease -
/// by the rules `mask.add_remove_key` and `effect_key_command` already keep. A key moved onto
/// another is refused by the core, as a property's is.
fn path_key_command(
    id: &str,
    query: Option<&str>,
    composition: Id,
    layer: &Layer,
    which: (bool, usize),
) -> Result<Command, String> {
    let frame = frame_parameter(
        query,
        if id == "keyframe.move" {
            "from"
        } else {
            "frame"
        },
    )?;
    with_path(composition, layer, which, |base, keys| {
        let at = keys.iter().position(|k| k.frame == frame);
        let missing = || format!("This path has no key at frame {frame}.");
        match id {
            "keyframe.add_remove" => match at {
                Some(i) => {
                    let gone = keys.remove(i);
                    if keys.is_empty() {
                        *base = gone.points;
                    }
                }
                None => {
                    let points = anime_compositor::mask::points_at(base, keys, frame);
                    keys.push(anime_compositor::mask::MaskKey {
                        frame,
                        points,
                        interp: Interp::Linear,
                    });
                }
            },
            "keyframe.move" => keys[at.ok_or_else(missing)?].frame = frame_parameter(query, "to")?,
            _ => keys[at.ok_or_else(missing)?].interp = interp_parameter(query)?,
        }
        Ok(())
    })
}

/// B-108b: `shape:<n>:fill_start` and its siblings name a gradient's points wherever a request
/// names a property, as `shape:<n>` names the path; B-109b adds `trim_start`, `trim_end` and
/// `trim_offset`. The names are the core's `SHAPE_PROPERTIES`.
fn shape_prop(prop: &str) -> Option<(usize, &'static str)> {
    let (at, name) = prop.strip_prefix("shape:")?.split_once(':')?;
    let name = anime_compositor::shape::SHAPE_PROPERTIES
        .into_iter()
        .find(|n| *n == name)?;
    Some((at.parse().ok()?, name))
}

/// B-108b: the five requests about a gradient's point, or since B-109b a trim's number, or since
/// B-110b a fill's or stroke's colour, opacity or width, by the rules a layer's position keeps:
/// a value on a point with no keys moves it, on a keyed one it is a key on that frame keeping
/// the ease a key there had; the diamond adds a key holding where the point already is, or takes
/// one off, the last one leaving the point where it was. A colour is three channels that are
/// keyed together. The layer's shapes go back whole, as the one command the core has for them,
/// and `Shape::problem` has the last word.
fn shape_key_command(
    id: &str,
    query: Option<&str>,
    composition: Id,
    layer: &Layer,
    (at, name): (usize, &str),
) -> Result<Command, String> {
    let mut shapes = layer.shapes.clone();
    let Some(shape) = shapes.get_mut(at) else {
        return Err(format!("{} has no shape {at}.", layer.name));
    };
    let called = shape.name.clone();
    let (group, which) = name.split_once('_').expect("every name has one");
    let point = group != "trim" && matches!(which, "start" | "end")
        || group == "path" && anime_compositor::shape::is_pair(which);
    let colour = which == "color";
    let Some(mut channels) = shape.channels(name) else {
        return Err(match group {
            "trim" => format!("\"{called}\" has no trim."),
            // D-738 (a): a drawn path, or a setting this kind has not.
            "path" => format!("\"{called}\" has no {} setting.", which.replace('_', " ")),
            _ if point => format!("\"{called}\" has no {group} gradient."),
            _ => format!("\"{called}\" has no {group}."),
        });
    };
    let this = match () {
        _ if point => "This point",
        _ if colour => "This colour",
        _ => "This number",
    };
    let values = match id {
        "property.set_base" | "property.drag_update" => parameter(query, "value")
            .and_then(|text| match () {
                _ if point => property_value(Prop::Position, &text).map(|v| vec![v]),
                _ => text
                    .split(',')
                    .map(|n| n.trim().parse().ok().map(Value::Scalar))
                    .collect::<Option<Vec<Value>>>(),
            })
            .filter(|values| values.len() == channels.len())
            .ok_or_else(|| match () {
                _ if point => "Where should the point go? Say value=x,y.".to_string(),
                _ if colour => "What colour? Say value=r,g,b, each from 0 to 1.".to_string(),
                _ => "What number? Say value=50.".to_string(),
            })?,
        _ => Vec::new(),
    };
    edit_channels(id, query, &mut channels, values, this)?;
    shape.set_channels(name, channels);
    Ok(Command::SetShapes {
        composition,
        layer_id: layer.id.clone(),
        shapes,
        groups: None,
    })
}

/// Which of a group's sets of numbers a `group:` property names.
#[derive(Clone, Copy)]
enum GroupPart {
    Transform,
    Repeater,
    Operation(usize),
}

/// D-738 (b): `group:<n>:position` and its six siblings name a shape group's transform wherever
/// a request names a property. The names are the core's `GROUP_PROPERTIES`; D-738 (c)'s
/// `group:<n>:repeater_copies` and its seven siblings, `REPEATER_PROPERTIES`, name its repeater's;
/// D-738 (d)'s `group:<n>:op<k>_radius` and the like, `OPERATIONS`' numbers, its `k`th operation's.
fn group_prop(prop: &str) -> Option<(usize, &'static str, GroupPart)> {
    use anime_compositor::shape::{GROUP_PROPERTIES, OPERATIONS, REPEATER_PROPERTIES};
    let (at, name) = prop.strip_prefix("group:")?.split_once(':')?;
    let op = name
        .strip_prefix("op")
        .and_then(|rest| rest.split_once('_'));
    let found = if let Some((k, name)) = op.filter(|(k, _)| k.parse::<usize>().is_ok()) {
        let mut all = OPERATIONS.iter().flat_map(|o| o.1.iter().map(|n| n.0));
        (
            all.find(|n| *n == name)?,
            GroupPart::Operation(k.parse().ok()?),
        )
    } else if let Some(name) = name.strip_prefix("repeater_") {
        let name = REPEATER_PROPERTIES.into_iter().find(|n| *n == name)?;
        (name, GroupPart::Repeater)
    } else {
        let name = GROUP_PROPERTIES.into_iter().find(|n| *n == name)?;
        (name, GroupPart::Transform)
    };
    Some((at.parse().ok()?, found.0, found.1))
}

/// D-738 (b): the five requests about a group's number, by the rules `shape_key_command` keeps.
/// Anchor, position and scale are two numbers, said as a point is; scale in per cent and opacity
/// from 0 to 1, as the file holds them. `groups_problem` has the last word on the ranges.
fn group_key_command(
    id: &str,
    query: Option<&str>,
    composition: Id,
    layer: &Layer,
    (at, name, part): (usize, &str, GroupPart),
) -> Result<Command, String> {
    let mut groups = layer.shape_groups.clone();
    let Some(group) = groups.get_mut(at) else {
        return Err(format!("{} has no group {at}.", layer.name));
    };
    let gname = group.name.clone();
    let numbers = match part {
        GroupPart::Transform => &mut group.transform,
        GroupPart::Repeater => match group.repeater.as_mut() {
            Some(r) => &mut r.numbers,
            None => return Err(format!("\"{gname}\" has no repeater.")),
        },
        GroupPart::Operation(k) => match group.operations.get_mut(k) {
            Some(o) => &mut o.numbers,
            None => return Err(format!("\"{gname}\" has no operation {k}.")),
        },
    };
    let Some(now) = numbers.get(name).cloned() else {
        return Err(format!("\"{gname}\" has no {name}."));
    };
    let pair = matches!(name, "anchor" | "position" | "scale" | "center");
    let values = match id {
        "property.set_base" | "property.drag_update" => vec![parameter(query, "value")
            .and_then(|text| match pair {
                true => property_value(Prop::Position, &text),
                false => text.trim().parse().ok().map(Value::Scalar),
            })
            .ok_or_else(|| match pair {
                true => "What two numbers? Say value=x,y.".to_string(),
                false => "What number? Say value=50.".to_string(),
            })?],
        _ => Vec::new(),
    };
    let mut channels = vec![now];
    edit_channels(id, query, &mut channels, values, "This number")?;
    numbers.insert(name.to_string(), channels.remove(0));
    Ok(Command::SetShapes {
        composition,
        layer_id: layer.id.clone(),
        shapes: layer.shapes.clone(),
        groups: Some(groups),
    })
}

/// D-298: `mask:<n>:feather`, `opacity` and `expansion` name a mask's numbers wherever a request
/// names a property, as `shape:<n>:fill_opacity` names a shape's. The names are the core's
/// `MASK_PROPERTIES`; the values are the file's, an opacity from 0 to 1.
fn mask_prop(prop: &str) -> Option<(usize, &'static str)> {
    let (at, name) = prop.strip_prefix("mask:")?.split_once(':')?;
    let name = anime_compositor::mask::MASK_PROPERTIES
        .into_iter()
        .find(|n| *n == name)?;
    Some((at.parse().ok()?, name))
}

/// D-298: the five requests about a mask's number, by the rules [`shape_key_command`] keeps for
/// a shape's. The layer's masks go back whole and the core holds every key to D-77's ranges.
fn mask_key_command(
    id: &str,
    query: Option<&str>,
    composition: Id,
    layer: &Layer,
    (at, name): (usize, &str),
) -> Result<Command, String> {
    let mut masks = layer.masks.clone();
    let Some(mask) = masks.get_mut(at) else {
        return Err(format!("{} has no mask {at}.", layer.name));
    };
    let mut channels = vec![mask.channel(name).expect("a name from MASK_PROPERTIES")];
    let values = match id {
        "property.set_base" | "property.drag_update" => vec![parameter(query, "value")
            .and_then(|text| text.trim().parse().ok())
            .map(Value::Scalar)
            .ok_or_else(|| "What number? Say value=50.".to_string())?],
        _ => Vec::new(),
    };
    edit_channels(id, query, &mut channels, values, "This number")?;
    mask.set_channel(name, channels.remove(0));
    Ok(Command::SetMasks {
        composition,
        layer_id: layer.id.clone(),
        masks,
    })
}

/// B-110b: what the five requests do to a keyed number's channels, which are keyed together: a
/// value on a number with no keys sets it, on a keyed one it is a key on that frame keeping the
/// ease a key there had; the diamond adds a key holding the number as it is or takes one off, the
/// last one leaving the number where it was. `values` is one a channel for the two that set one.
fn edit_channels(
    id: &str,
    query: Option<&str>,
    channels: &mut [Property],
    values: Vec<Value>,
    this: &str,
) -> Result<(), String> {
    let frame = frame_parameter(
        query,
        if id == "keyframe.move" {
            "from"
        } else {
            "frame"
        },
    )?;
    let key = |frame, value| Keyframe {
        frame,
        value,
        interp: Interp::Linear,
        spatial: None,
        kind: Default::default(),
        roving: false,
    };
    let missing = || format!("{this} has no key at frame {frame}.");
    match id {
        "property.set_base" | "property.drag_update" => {
            for (channel, value) in channels.iter_mut().zip(values) {
                match channel.keyframe_at(frame).cloned() {
                    _ if channel.keyframes().is_empty() => channel.set_base(value),
                    Some(was) => channel.set_keyframe(Keyframe { value, ..was }),
                    None => channel.set_keyframe(key(frame, value)),
                }
            }
        }
        "keyframe.add_remove" => {
            for channel in channels.iter_mut() {
                match channel.remove_keyframe(frame) {
                    Some(gone) if channel.keyframes().is_empty() => channel.set_base(gone.value),
                    Some(_) => {}
                    None => channel.set_keyframe(key(frame, channel.value_at(frame))),
                }
            }
        }
        "keyframe.move" => {
            let to = frame_parameter(query, "to")?;
            if to != frame && channels[0].keyframe_at(to).is_some() {
                return Err(format!("{this} already has a key at frame {to}."));
            }
            for channel in channels.iter_mut() {
                let was = channel.remove_keyframe(frame).ok_or_else(missing)?;
                channel.set_keyframe(Keyframe { frame: to, ..was });
            }
        }
        _ => {
            let interp = interp_parameter(query)?;
            for channel in channels.iter_mut() {
                let was = channel.keyframe_at(frame).cloned().ok_or_else(missing)?;
                channel.set_keyframe(Keyframe { interp, ..was });
            }
        }
    }
    Ok(())
}

/// B-24h: D-79's two requests about a path's points, one added and one taken off.
///
/// `at` names a point of the path. For `path.add_point` it is the point the new one comes after,
/// and `t` is the fraction along the segment that starts there, more than 0 and less than 1;
/// `mask::insert_point` cuts that segment where D-79 says, on the base and on every key. For
/// `path.remove_point` it is the point to take off, and it comes off the base and off every key.
///
/// Both change how many points the path holds, which is why neither can go the way a pulled
/// point goes -- into the key under the playhead. A key holding a different number of points
/// from the base is the one thing D-77 cannot interpolate, and the core refuses it.
///
/// An open shape has no segment after its last point, so `at` may not name it.
///
/// B-24i, D-80: `path.remove_point` takes several points separated by commas, because several
/// points can be chosen at once and taking four off has to be one thing to undo rather than
/// four. They come off from the highest index down, so the lower ones go on meaning what they
/// meant while the higher ones are going. `path.add_point` stays one point: there is no gesture
/// that asks for several, and a segment cut twice is two different fractions.
fn path_point_command(
    id: &str,
    query: Option<&str>,
    composition: Id,
    layer: &Layer,
    which: (bool, usize),
) -> Result<Command, String> {
    let (shape, at_path) = which;
    let ats: Vec<usize> = parameter(query, "at")
        .and_then(|v| {
            v.split(',')
                .map(|one| one.trim().parse::<usize>().ok())
                .collect::<Option<Vec<usize>>>()
        })
        .filter(|ats| !ats.is_empty())
        .ok_or_else(|| "Which point? Say at=0 for the first.".to_string())?;
    let at = ats[0];
    let t = match parameter(query, "t") {
        None => 0.5,
        Some(text) => text
            .trim()
            .parse::<f64>()
            .ok()
            .filter(|t| *t > 0.0 && *t < 1.0)
            .ok_or_else(|| {
                format!(
                    "\"{}\" is not a place along a segment. It is a fraction, more than 0 and \
                     less than 1.",
                    text.trim()
                )
            })?,
    };
    let open = shape && layer.shapes.get(at_path).is_some_and(|s| !s.closed);
    with_path(composition, layer, which, |base, keys| {
        let last = base.len().saturating_sub(1 + open as usize);
        match id {
            "path.add_point" if ats.len() > 1 => Err("A point is added one at a time, because \
                 each one is a place along its own segment. Say at= one point."
                .to_string()),
            "path.add_point" if at > last => Err(match open {
                true => format!("This path ends at point {last}, so nothing starts after {at}."),
                false => format!(
                    "This path has {} points, so it has no point {at}.",
                    base.len()
                ),
            }),
            "path.add_point" => {
                anime_compositor::mask::insert_point(base, keys, at, t);
                Ok(())
            }
            _ => {
                let mut taken: Vec<usize> = ats.clone();
                taken.sort_unstable();
                taken.dedup();
                if let Some(&past) = taken.iter().find(|&&a| a >= base.len()) {
                    return Err(format!(
                        "This path has {} points, so it has no point {past}.",
                        base.len()
                    ));
                }
                for at in taken.iter().rev() {
                    anime_compositor::mask::remove_point(base, keys, *at);
                }
                Ok(())
            }
        }
    })
}

/// B-25c: `shape.set`'s settings onto one shape. `fill` and `stroke` are three linear numbers,
/// or `none` to take it off; the opacities come in percent, as the panel counts them; `enabled`
/// and `closed` are switches read from the document, as every toggle here is. Only text that is
/// not a number at all is refused here; D-78's ranges are the core's to refuse.
fn shape_settings(
    query: Option<&str>,
    shape: &mut anime_compositor::shape::Shape,
) -> Result<(), String> {
    use anime_compositor::shape::{Cap, Fill, Gradient, GradientKind, Join, Stop, Stroke};
    let color = |text: &str| -> Result<[f64; 3], String> {
        let numbers: Vec<f64> = text
            .split(',')
            .filter_map(|n| n.trim().parse::<f64>().ok())
            .collect();
        <[f64; 3]>::try_from(numbers).map_err(|_| {
            format!("\"{text}\" is not a colour. A colour is three numbers from 0 to 1, red, green and blue.")
        })
    };
    let number = |key: &str, what: &str| -> Result<Option<f64>, String> {
        parameter(query, key)
            .map(|text| {
                text.trim()
                    .parse::<f64>()
                    .map_err(|_| format!("\"{}\" is not {what}. It is a number.", text.trim()))
            })
            .transpose()
    };
    match parameter(query, "fill").as_deref() {
        None => {}
        Some("none") => {
            shape.fill = None;
            shape.tracks.retain(|n, _| !n.starts_with("fill_"));
        }
        Some(text) => {
            let color = color(text)?;
            let (opacity, gradient) = shape
                .fill
                .take()
                .map_or((1.0, None), |f| (f.opacity, f.gradient));
            shape.fill = Some(Fill {
                color,
                opacity,
                gradient,
            });
        }
    }
    match parameter(query, "stroke").as_deref() {
        None => {}
        Some("none") => {
            shape.stroke = None;
            shape.tracks.retain(|n, _| !n.starts_with("stroke_"));
        }
        Some(text) => {
            let color = color(text)?;
            match shape.stroke.as_mut() {
                Some(stroke) => stroke.color = color,
                None => shape.stroke = Some(Stroke::new(color, 4.0)),
            }
        }
    }
    // B-110b, D-170: how a stroke turns its corners and ends its open lines, and how far a
    // mitre may reach. D-170's range for the limit is the core's to refuse.
    // D-738 (b): the group the shape is in, by place, or none; the core refuses a place the
    // layer has no group at.
    match parameter(query, "group").as_deref().map(str::trim) {
        None => {}
        Some("none") => shape.group = None,
        Some(text) => {
            shape.group = Some(text.parse().map_err(|_| {
                format!("\"{text}\" is not a group. It is a group's number, or none.")
            })?)
        }
    }
    let join = parameter(query, "join");
    let cap = parameter(query, "cap");
    let limit = number("miter_limit", "a mitre limit")?;
    if join.is_some() || cap.is_some() || limit.is_some() {
        let name = shape.name.clone();
        let Some(stroke) = shape.stroke.as_mut() else {
            return Err(format!("\"{name}\" has no stroke to set the corners of."));
        };
        if let Some(text) = join {
            stroke.join = Join::named(&text)
                .ok_or_else(|| format!("\"{text}\" is not a join. It is miter, round or bevel."))?;
        }
        if let Some(text) = cap {
            stroke.cap = Cap::named(&text)
                .ok_or_else(|| format!("\"{text}\" is not a cap. It is butt, round or square."))?;
        }
        if let Some(v) = limit {
            stroke.miter_limit = v;
        }
    }
    // B-108b, D-168: `fill_gradient` and `stroke_gradient` are none, linear or radial. Turned on,
    // a gradient runs across the shape from the middle of its left side to the middle of its
    // right, from the paint's own colour to black; a change of type keeps the points and the
    // stops. `fill_stops` and `stroke_stops` are `offset,r,g,b,opacity` per stop, separated by
    // semicolons. D-168's ranges are the core's to refuse.
    let (xs, ys): (Vec<f64>, Vec<f64>) = shape.points.iter().map(|p| p.point).unzip();
    let low = |v: &[f64]| v.iter().copied().fold(f64::MAX, f64::min);
    let high = |v: &[f64]| v.iter().copied().fold(f64::MIN, f64::max);
    let middle = (low(&ys) + high(&ys)) / 2.0;
    let (left, right) = (
        Value::Vec2(low(&xs), middle),
        Value::Vec2(high(&xs), middle),
    );
    let name = shape.name.clone();
    for (paint, found) in [
        (
            "fill",
            shape.fill.as_mut().map(|f| (f.color, &mut f.gradient)),
        ),
        (
            "stroke",
            shape.stroke.as_mut().map(|s| (s.color, &mut s.gradient)),
        ),
    ] {
        let kind = parameter(query, &format!("{paint}_gradient"));
        let stops = parameter(query, &format!("{paint}_stops"));
        if kind.is_none() && stops.is_none() {
            continue;
        }
        let Some((color, gradient)) = found else {
            return Err(format!("\"{name}\" has no {paint} to give a gradient."));
        };
        match kind.as_deref() {
            None => {}
            Some("none") => *gradient = None,
            Some(text) => {
                let kind = GradientKind::named(text).ok_or_else(|| {
                    format!("\"{text}\" is not a kind of gradient. It is none, linear or radial.")
                })?;
                match gradient {
                    Some(g) => g.kind = kind,
                    None => {
                        *gradient = Some(Gradient {
                            kind,
                            start: Property::constant(left),
                            end: Property::constant(right),
                            stops: vec![
                                Stop {
                                    offset: 0.0,
                                    color,
                                    opacity: 1.0,
                                },
                                Stop {
                                    offset: 1.0,
                                    color: [0.0; 3],
                                    opacity: 1.0,
                                },
                            ],
                        })
                    }
                }
            }
        }
        if let Some(text) = stops {
            let Some(g) = gradient.as_mut() else {
                return Err(format!(
                    "\"{name}\" has no {paint} gradient to set the stops of."
                ));
            };
            g.stops = text
                .split(';')
                .filter(|piece| !piece.trim().is_empty())
                .map(|piece| {
                    let n: Vec<f64> = piece
                        .split(',')
                        .filter_map(|n| n.trim().parse().ok())
                        .collect();
                    match n[..] {
                        [offset, r, g, b, opacity] => Ok(Stop {
                            offset,
                            color: [r, g, b],
                            opacity,
                        }),
                        _ => Err(format!(
                            "A stop is five numbers, offset,r,g,b,opacity. Not \"{piece}\"."
                        )),
                    }
                })
                .collect::<Result<_, _>>()?;
        }
    }
    // B-109b, D-169: `trim` is on or off. Turned on, it is the whole path with no offset, so
    // nothing changes on the picture until a number does.
    match parameter(query, "trim").as_deref() {
        None => {}
        Some("on") => {
            shape
                .trim
                .get_or_insert_with(anime_compositor::shape::Trim::whole);
        }
        Some("off") => shape.trim = None,
        Some(text) => return Err(format!("\"{text}\" is not on or off.")),
    }
    if let Some(v) = number("fill_opacity", "an opacity")? {
        let Some(fill) = shape.fill.as_mut() else {
            return Err(format!(
                "\"{}\" has no fill to set the opacity of.",
                shape.name
            ));
        };
        fill.opacity = v / 100.0;
    }
    for (key, what) in [
        ("stroke_opacity", "an opacity"),
        ("stroke_width", "a width"),
    ] {
        if let Some(v) = number(key, what)? {
            let Some(stroke) = shape.stroke.as_mut() else {
                return Err(format!(
                    "\"{}\" has no stroke to set the {} of.",
                    shape.name,
                    &key[7..]
                ));
            };
            match key {
                "stroke_opacity" => stroke.opacity = v / 100.0,
                _ => stroke.width_px = v,
            }
        }
    }
    if parameter(query, "enabled").is_some() {
        shape.enabled = !shape.enabled;
    }
    // D-738 (a): After Effects' Type switch between a polygon and a star. A star switched to a
    // polygon keeps its inner settings, unused, for switching back; a polygon switched to a star
    // starts with an inner radius half its outer one.
    if let Some(to) = parameter(query, "polystar") {
        use anime_compositor::shape::ParametricKind::{Polygon, Star};
        let Some(p) = shape
            .parametric
            .as_mut()
            .filter(|p| matches!(p.kind, Polygon | Star))
        else {
            return Err(format!("\"{}\" is not a polygon or a star.", shape.name));
        };
        p.kind = match to.as_str() {
            "star" => Star,
            "polygon" => Polygon,
            _ => return Err(format!("\"{to}\" is not polygon or star.")),
        };
        let outer = p.settings["outer_radius"].base().as_scalar().unwrap_or(0.0);
        p.settings
            .entry("inner_radius".into())
            .or_insert_with(|| Property::constant(Value::Scalar(outer / 2.0)));
        p.settings
            .entry("inner_roundness".into())
            .or_insert_with(|| Property::constant(Value::Scalar(0.0)));
        shape.points = p.outline(None);
    }
    // D-738 (a): Convert to Bezier Path: the outline at the settings' bases, their keys dropped.
    if parameter(query, "bezier").is_some() {
        let Some(p) = shape.parametric.take() else {
            return Err(format!("\"{}\" is already a Bezier path.", shape.name));
        };
        shape.points = p.outline(None);
    }
    if parameter(query, "closed").is_some() {
        shape.closed = !shape.closed;
    }
    if let Some(name) = parameter(query, "name") {
        shape.name = name;
    }
    Ok(())
}

/// D-738 (a): a new rectangle, ellipse, polygon or star as the shape tool sends it:
/// `parametric=` its kind, `position=x,y`, `size=w,h`, `roundness`, `radius` (the outer one; a
/// star's inner is half of it), `points` and `rotation`. Ranges are the core's to refuse.
fn parametric_shape(
    query: Option<&str>,
    kind: &str,
) -> Result<anime_compositor::shape::Parametric, String> {
    let Some(kind) = anime_compositor::shape::ParametricKind::named(kind) else {
        return Err(format!(
            "\"{kind}\" is not a shape to draw. Say rectangle, ellipse, polygon or star."
        ));
    };
    let numbers = |key: &str, default: &[f64]| -> Result<Vec<f64>, String> {
        let Some(text) = parameter(query, key) else {
            return Ok(default.to_vec());
        };
        text.split(',')
            .map(|n| n.trim().parse::<f64>().ok())
            .collect::<Option<Vec<f64>>>()
            .filter(|n| n.len() == default.len())
            .ok_or_else(|| match default.len() {
                2 => format!("\"{text}\" is not a {key}. Say {key}=x,y."),
                _ => format!("\"{text}\" is not a {key}. It is a number."),
            })
    };
    let radius = numbers("radius", &[50.0])?[0];
    let mut settings = std::collections::BTreeMap::new();
    for name in kind.settings() {
        let value = match *name {
            "size" | "position" => {
                let default = if *name == "size" { 100.0 } else { 0.0 };
                let n = numbers(name, &[default, default])?;
                Value::Vec2(n[0], n[1])
            }
            "outer_radius" => Value::Scalar(radius),
            "inner_radius" => Value::Scalar(radius / 2.0),
            "points" => Value::Scalar(numbers("points", &[5.0])?[0]),
            "roundness" | "rotation" => Value::Scalar(numbers(name, &[0.0])?[0]),
            _ => Value::Scalar(0.0),
        };
        settings.insert(name.to_string(), Property::constant(value));
    }
    Ok(anime_compositor::shape::Parametric { kind, settings })
}

/// B-24c: a mask's path as the page sends it, `x,y,in_x,in_y,out_x,out_y` per point, separated
/// by semicolons, or the sentence to answer with.
///
/// Six numbers and not two, so that the pen's curve and a dragged handle travel by the same
/// route as a rectangle's corners; a corner is the same point with four zeros, which is what
/// D-77 calls no handle. How many points make a shape, and whether they cross, is the core's to
/// say — this refuses only text that is not a path at all.
fn mask_points(query: Option<&str>) -> Result<Vec<anime_compositor::mask::MaskPoint>, String> {
    let text = parameter(query, "points").unwrap_or_default();
    let mut points = Vec::new();
    for piece in text.split(';').filter(|p| !p.trim().is_empty()) {
        let numbers: Vec<f64> = piece
            .split(',')
            .filter_map(|n| n.trim().parse::<f64>().ok())
            .collect();
        let [x, y, in_x, in_y, out_x, out_y] = numbers[..] else {
            return Err(format!(
                "A mask point is six numbers, x,y,in_x,in_y,out_x,out_y. Not \"{piece}\"."
            ));
        };
        points.push(anime_compositor::mask::MaskPoint {
            point: (x, y),
            in_handle: (in_x, in_y),
            out_handle: (out_x, out_y),
        });
    }
    Ok(points)
}

fn parameter(query: Option<&str>, name: &str) -> Option<String> {
    let prefix = format!("{name}=");
    query
        .and_then(|q| q.split('&').find_map(|pair| pair.strip_prefix(&prefix)))
        .map(from_a_query)
}

/// The four numbers of a D-52 curve as the page sends them, or the sentence to answer with.
///
/// Document 19 bounds `x` to the segment and leaves `y` free for overshoot; `persist` refuses a
/// file that breaks that, and this refuses a drag that would write one, so the same rule is
/// enforced on the way in as on the way back.
fn handles(text: &str) -> Result<Interp, String> {
    let numbers: Vec<f64> = text
        .split(',')
        .filter_map(|n| n.trim().parse().ok())
        .collect();
    let [x1, y1, x2, y2] = numbers[..] else {
        return Err(format!(
            "A curve is four numbers, x1,y1,x2,y2. Not \"{text}\"."
        ));
    };
    if !(0.0..=1.0).contains(&x1) || !(0.0..=1.0).contains(&x2) {
        return Err(format!(
            "A curve handle cannot reach outside its own segment, so x1 and x2 are between 0 \
             and 1. Not {x1} and {x2}."
        ));
    }
    Ok(Interp::Ease { x1, y1, x2, y2 })
}

/// D-314: a row's keys, as `(frame, ease)` in order, with the chosen ones reversed in time.
/// `moves` is each chosen key's frame and the frame it goes to, earliest first. The key that
/// lands first leaves with the ease the last chosen key had; every other one takes, turned
/// round, the ease of the stretch it now plays backwards. A chosen frame with no key, or a key
/// not chosen in the way of one landing, is refused and nothing changes.
fn reversed(keys: &[(i32, Interp)], moves: &[(i32, i32)]) -> Result<Vec<(i32, Interp)>, String> {
    let chosen = moves
        .iter()
        .map(|(at, _)| {
            keys.iter()
                .position(|k| k.0 == *at)
                .ok_or_else(|| format!("There is no key on frame {at} to reverse."))
        })
        .collect::<Result<Vec<usize>, String>>()?;
    let mut out = keys.to_vec();
    for (j, &i) in chosen.iter().enumerate() {
        out[i] = (
            moves[j].1,
            match j {
                0 => keys[chosen[chosen.len() - 1]].1,
                _ => turned(keys[chosen[j - 1]].1),
            },
        );
    }
    let stays = keys.iter().enumerate().filter(|(i, _)| !chosen.contains(i));
    if let Some((_, (frame, _))) = stays
        .into_iter()
        .find(|(_, k)| moves.iter().any(|m| m.1 == k.0))
    {
        return Err(format!(
            "A key that is not chosen is on frame {frame}, where a reversed key would land. \
             Choose it too, or move it first."
        ));
    }
    Ok(out)
}

/// D-314: an ease played backwards, the curve turned half a turn about its middle.
fn turned(interp: Interp) -> Interp {
    match interp {
        Interp::Ease { x1, y1, x2, y2 } => Interp::Ease {
            x1: 1.0 - x2,
            y1: 1.0 - y2,
            x2: 1.0 - x1,
            y2: 1.0 - y1,
        },
        other => other,
    }
}

/// B-150c: a key the page names by the frame it is shown on, and the frame `by` further on, as
/// the frames they are stored at. D-216 stretches a layer's keys with it, so the timeline draws
/// a key where it plays - at 50% that can be half way between two frames - and this is the one
/// way back: document 20's key time, rounded half away from zero as a key set at the playhead
/// is. The camera and a layer at 100% take whole frames, as they always have.
fn stored_frames(layer: Option<&Layer>, shown: &str, by: i32) -> Option<(i32, i32)> {
    match layer.filter(|l| l.time_stretch != 100.0) {
        Some(l) => {
            let at: f64 = shown.parse().ok()?;
            Some((
                l.key_time(at).round() as i32,
                l.key_time(at + by as f64).round() as i32,
            ))
        }
        None => shown.parse::<i32>().ok().map(|at| (at, at + by)),
    }
}

/// B-150c: a request about one layer's keys with each frame that names a key taken back to the
/// frame it is stored at, by [`stored_frames`], or `None` when nothing needs to change.
fn stored_key_query(id: &str, query: Option<&str>, layer: &Layer) -> Option<String> {
    let names: &[&str] = match id {
        "keyframe.move" => &["from", "to"],
        "mask.set" | "shape.set" => &["key", "to"],
        "keyframe.add_remove"
        | "keyframe.set_interp"
        | "keyframe.set_path"
        | "property.set_base"
        | "property.drag_update"
        | "mask.add_remove_key"
        | "mask.set_path"
        | "shape.add_remove_key"
        | "shape.set_path" => &["frame"],
        _ => return None,
    };
    if layer.time_stretch == 100.0 {
        return None;
    }
    let pairs: Vec<String> = query?
        .split('&')
        .map(|pair| match pair.split_once('=') {
            Some((name, shown)) if names.contains(&name) => {
                match stored_frames(Some(layer), shown, 0) {
                    Some((at, _)) => format!("{name}={at}"),
                    None => pair.to_string(),
                }
            }
            _ => pair.to_string(),
        })
        .collect();
    Some(pairs.join("&"))
}

/// A frame number the page named, or the sentence to answer with when it did not.
fn frame_parameter(query: Option<&str>, name: &str) -> Result<i32, String> {
    let Some(value) = parameter(query, name) else {
        return Err(format!("Which frame? Say {name}=<frame>."));
    };
    value
        .parse::<i32>()
        .map_err(|_| format!("A frame is a whole number. Not \"{value}\"."))
}

/// Undo [`for_a_header`]: percent-decoded back into the string that was encoded.
///
/// The recent list travels to the page and a chosen entry travels back, and both directions
/// carry paths that can be Japanese. A half-decoded path is a path to a file that does not
/// exist, so this is the other half of the same contract and is checked against the same
/// hand-worked UTF-8 vectors.
fn from_a_query(text: &str) -> String {
    let raw = text.as_bytes();
    let mut bytes = Vec::with_capacity(raw.len());
    let mut i = 0;
    while i < raw.len() {
        let hex = |b: u8| (b as char).to_digit(16);
        match (raw[i], raw.get(i + 1).copied(), raw.get(i + 2).copied()) {
            (b'%', Some(h), Some(l)) => match (hex(h), hex(l)) {
                (Some(h), Some(l)) => {
                    bytes.push((h * 16 + l) as u8);
                    i += 3;
                }
                // Not an escape after all. Keep the % rather than eating the next two
                // characters, which would silently shorten a path.
                _ => {
                    bytes.push(b'%');
                    i += 1;
                }
            },
            _ => {
                bytes.push(raw[i]);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Ask the operating system where a project is, then open it.
///
/// The dialog is opened from Rust and its answer never passes through the page, which is why
/// the page has no dialog permission and cannot ask for a file on its own. A cancelled dialog
/// says nothing and changes nothing.
fn ask_to_open(app: &AppHandle) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Open a project")
        .add_filter("Anime Compositor project", &["json"])
        .pick_file(move |chosen| {
            let Some(path) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            if take(&handle.state::<Mutex<Viewer>>(), &path) {
                remember(&handle, &path);
            }
            refresh(&handle);
        });
}

/// Ask the operating system which drawings to import, then import them.
///
/// The dialog takes many files at once, because a sequence is a selection and B-03 groups the
/// files it is given. It filters to PNG and EXR, the formats this build reads (D-62); a person
/// who selects something else is told by the importer rather than by a dialog that will not
/// let them.
fn ask_what_to_import(app: &AppHandle) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Import drawings")
        .add_filter("Drawings and sound", &DRAWINGS_AND_SOUND)
        .pick_files(move |chosen| {
            let files: Vec<PathBuf> = chosen
                .unwrap_or_default()
                .into_iter()
                .filter_map(|c| c.into_path().ok())
                .collect();
            if files.is_empty() {
                return;
            }
            let viewer = handle.state::<Mutex<Viewer>>();
            let said = import(&viewer, &files);
            announce(&viewer, said);
            refresh(&handle);
        });
}

/// D-84: ask for a cut's folder, the one holding its `.xdts` file, and import what is in it.
fn ask_which_cut(app: &AppHandle) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Import a cut: choose the folder that holds its .xdts timesheet")
        .pick_folder(move |chosen| {
            let Some(folder) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let viewer = handle.state::<Mutex<Viewer>>();
            let said = import_cut(&viewer, &folder);
            announce(&viewer, said);
            refresh(&handle);
        });
}

/// Ask which drawings a sequence should point at instead, and work out what that would do.
///
/// Document 07: "Search only user-selected locations." Nothing here scans for a replacement; the
/// files are the ones a person chose, and what comes back is a proposal rather than a change.
fn ask_what_to_relink_to(app: &AppHandle, asset: Id) {
    let handle = app.clone();
    // D-182: a lookup file is relinked to a .cube file.
    let lookup = app
        .state::<Mutex<Viewer>>()
        .lock()
        .expect("the viewer lock was poisoned")
        .document
        .project()
        .assets
        .iter()
        .any(|a| a.id == asset && a.kind == AssetKind::Lut);
    let (title, filter, kinds): (&str, &str, &[&str]) = if lookup {
        (
            "Relink to this lookup file",
            "Colour lookup",
            &["cube", "amp"],
        )
    } else {
        (
            "Relink to these drawings",
            "Drawings and sound",
            &DRAWINGS_AND_SOUND,
        )
    };
    app.dialog()
        .file()
        .set_title(title)
        .add_filter(filter, kinds)
        .pick_files(move |chosen| {
            let files: Vec<PathBuf> = chosen
                .unwrap_or_default()
                .into_iter()
                .filter_map(|c| c.into_path().ok())
                .collect();
            if files.is_empty() {
                return;
            }
            let viewer = handle.state::<Mutex<Viewer>>();
            let said = propose_relink(&viewer, &asset, &files);
            announce(&viewer, said);
            refresh(&handle);
        });
}

/// Ask the operating system where to write the project, then write it there.
fn ask_where_to_save(app: &AppHandle) {
    let handle = app.clone();
    let suggestion = {
        let viewer = app.state::<Mutex<Viewer>>();
        let viewer = viewer.lock().expect("the viewer lock was poisoned");
        match &viewer.path {
            Some(path) => path.file_name().map(|n| n.to_string_lossy().into_owned()),
            None => None,
        }
        .unwrap_or_else(|| "project.json".to_string())
    };
    app.dialog()
        .file()
        .set_title("Save the project as")
        .set_file_name(suggestion)
        .add_filter("Anime Compositor project", &["json"])
        .save_file(move |chosen| {
            let Some(path) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let said = save_as(&handle.state::<Mutex<Viewer>>(), &path);
            announce(&handle.state::<Mutex<Viewer>>(), said);
            remember(&handle, &path);
            refresh(&handle);
        });
}

/// P-19: what the session log panel shows, as JSON: the summary, then the 20 slowest frames
/// with the three stages each spent longest in.
fn session_log_json(log: &SessionLog) -> String {
    let s = log.summary();
    let slowest: Vec<serde_json::Value> = log
        .slowest(20)
        .into_iter()
        .map(|r| {
            let mut stages: Vec<_> = r.stages.iter().filter(|&&(_, n)| n > 0).collect();
            stages.sort_by(|a, b| b.1.cmp(&a.1));
            stages.truncate(3);
            serde_json::json!({
                "frame": r.frame,
                "ms": r.ms,
                "quality": r.quality.label(),
                "playing": r.playing,
                "dropped": r.dropped,
                "read_ahead": r.read_ahead,
                "exporting": r.exporting,
                "stages": stages
                    .iter()
                    .map(|&&(st, n)| serde_json::json!([st.label(), n as f64 / 1e6]))
                    .collect::<Vec<_>>(),
                "warnings": r.warnings,
            })
        })
        .collect();
    serde_json::json!({
        "on": log.is_on(),
        "frames": s.frames,
        "let_go": s.let_go,
        "median_ms": s.median_ms,
        "slowest_5_percent_ms": s.slowest_5_percent_ms,
        "dropped": s.dropped,
        "costliest": s.costliest.map(|(st, _)| st.label()),
        "costliest_ms": s.costliest.map_or(0.0, |(_, n)| n as f64 / 1e6),
        "slowest": slowest,
    })
    .to_string()
}

/// P-19: the lines a saved copy of the session log is headed with, which are what make it
/// evidence someone else can read: when, on what, and from which build.
fn session_log_heading() -> Vec<String> {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |d| d.as_secs());
    // The proleptic Gregorian date of a day count, which is Howard Hinnant's `civil_from_days`.
    let z = (now / 86_400) as i64 + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    vec![
        format!(
            "Saved: {year}-{month:02}-{day:02} {:02}:{:02} UTC",
            now % 86_400 / 3600,
            now % 3600 / 60
        ),
        format!("Operating system: {}", std::env::consts::OS),
        format!(
            "Processor cores: {}",
            std::thread::available_parallelism().map_or(0, |n| n.get())
        ),
        format!(
            "Build: {}, {}",
            env!("CARGO_PKG_VERSION"),
            if cfg!(debug_assertions) {
                "debug"
            } else {
                "release"
            }
        ),
    ]
}

/// P-19's "Save a copy...": the system Save dialog, then the log as one Markdown table.
fn ask_where_to_save_the_log(app: &AppHandle) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Save a copy of the session log")
        .set_file_name("session log.md")
        .add_filter("Markdown", &["md"])
        .save_file(move |chosen| {
            let Some(path) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let text = handle
                .state::<Mutex<SessionLog>>()
                .lock()
                .expect("the session log lock was poisoned")
                .to_markdown(&session_log_heading());
            let said = match std::fs::write(&path, text) {
                Ok(()) => format!("Saved a copy of the session log to {}.", path.display()),
                Err(e) => format!(
                    "The session log could not be saved to {}: {e}.",
                    path.display()
                ),
            };
            announce(&handle.state::<Mutex<Viewer>>(), said);
        });
}

/// D-180: Export presets. `presets` is the window's list as the page keeps it; it is checked
/// whole before anything is asked, and written to the file the person names.
fn ask_where_to_export_presets(app: &AppHandle, name: &str, presets: &str) -> String {
    let text = match persist::write_presets(presets) {
        Ok(text) => text,
        Err(refused) => return refused.sentence(),
    };
    let count = persist::read_presets(&text).map_or(0, |p| p.len());
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Export presets")
        .set_file_name(format!("{name}.fxpreset"))
        .add_filter("Effect presets", &["fxpreset"])
        .save_file(move |chosen| {
            let Some(path) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let said = match std::fs::write(&path, text) {
                Ok(()) if count == 1 => format!("Exported 1 preset to {}.", path.display()),
                Ok(()) => format!("Exported {count} presets to {}.", path.display()),
                Err(e) => format!(
                    "The presets could not be exported to {}: {e}.",
                    path.display()
                ),
            };
            tell_the_page_about_presets(&handle, None, said);
        });
    String::new()
}

/// D-289: a sketch, drawn by the page, saved as a PNG file where the person chooses. Only PNG
/// bytes are written, at most 256 MB, and only to a file picked in the Save dialog.
fn ask_where_to_save_sketch(app: &AppHandle, name: &str, png: Vec<u8>) -> String {
    if let Err(said) = sketch_png_ok(&png) {
        return said;
    }
    let name: String = name
        .chars()
        .filter(|c| !r#"\/:*?"<>|"#.contains(*c) && !c.is_control())
        .take(120)
        .collect();
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Save the sketch as a picture")
        .set_file_name(format!(
            "{}.png",
            if name.trim().is_empty() {
                "sketch"
            } else {
                name.trim()
            }
        ))
        .add_filter("PNG picture", &["png"])
        .save_file(move |chosen| {
            let Some(path) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let said = match std::fs::write(&path, &png) {
                Ok(()) => format!("Saved the sketch as {}.", path.display()),
                Err(e) => format!("The sketch could not be saved to {}: {e}.", path.display()),
            };
            announce(&handle.state::<Mutex<Viewer>>(), said.clone());
            if let Some(page) = handle.get_webview_window("main") {
                let _ = page.eval(format!(
                    "$('status').textContent = {}",
                    serde_json::Value::String(said)
                ));
            }
        });
    String::new()
}

/// D-289: what the page sent is a PNG picture of a size a sketch can be.
fn sketch_png_ok(png: &[u8]) -> Result<(), String> {
    const SIGNATURE: [u8; 8] = [0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a];
    if !png.starts_with(&SIGNATURE) {
        return Err(
            "The sketch did not arrive as a PNG picture, so nothing was saved.".to_string(),
        );
    }
    if png.len() > 256 << 20 {
        return Err("The sketch picture is over 256 MB, so nothing was saved.".to_string());
    }
    Ok(())
}

/// D-180: Import presets. `have` is the names the window already has, which an imported preset
/// never replaces.
fn ask_which_presets_to_import(app: &AppHandle, have: Vec<String>) {
    let handle = app.clone();
    app.dialog()
        .file()
        .set_title("Import presets")
        .add_filter("Effect presets", &["fxpreset"])
        .pick_file(move |chosen| {
            let Some(path) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let file = path.file_name().map_or_else(
                || path.display().to_string(),
                |n| n.to_string_lossy().into_owned(),
            );
            let answer = match std::fs::read_to_string(&path) {
                Ok(text) => imported_presets(&text, &file, &have),
                Err(e) => Err(format!(
                    "{file} could not be read, so nothing was imported: {e}."
                )),
            };
            match answer {
                Ok((list, said)) => tell_the_page_about_presets(&handle, Some(list), said),
                Err(said) => tell_the_page_about_presets(&handle, None, said),
            }
        });
}

/// D-395: whether `file` is a Photoshop arbitrary map, by its name.
fn is_amp(file: &Path) -> bool {
    file.extension()
        .is_some_and(|e| e.eq_ignore_ascii_case("amp"))
}

/// A lookup file read by its own rule, a .amp's (D-395) or a .cube's (D-182), or why not.
fn read_lookup_file(file: &Path, amp: bool) -> Result<(), String> {
    if amp {
        lut::read_map(file).map(drop)
    } else {
        lut::read(file).map(drop)
    }
}

/// The effect `instance_id` on `layer_id` in the open composition, as it is now.
fn effect_now_in(viewer: &Viewer, layer_id: &Id, instance_id: &Id) -> Option<Effect> {
    let layer = viewer
        .document
        .project()
        .composition(&viewer.composition)?
        .layer(layer_id)?;
    layer
        .effects
        .iter()
        .find(|e| &e.instance_id == instance_id)
        .map(|e| e.effect.clone())
}

/// D-182: the .cube file for the Color Lookup `effect` on `layer`, or the .amp file for an
/// Arbitrary Map (D-395).
fn ask_which_lut(app: &AppHandle, layer: Id, effect: Id) {
    let handle = app.clone();
    let amp = matches!(
        effect_now_in(
            &app.state::<Mutex<Viewer>>()
                .lock()
                .expect("the viewer lock was poisoned"),
            &layer,
            &effect
        ),
        Some(Effect::ArbitraryMap { .. })
    );
    let (title, filter, kinds): (&str, &str, &[&str]) = if amp {
        ("Choose an arbitrary map file", "Arbitrary map", &["amp"])
    } else {
        ("Choose a colour lookup file", "Colour lookup", &["cube"])
    };
    app.dialog()
        .file()
        .set_title(title)
        .add_filter(filter, kinds)
        .pick_file(move |chosen| {
            let Some(file) = chosen.and_then(|c| c.into_path().ok()) else {
                return;
            };
            let viewer = handle.state::<Mutex<Viewer>>();
            let said = choose_lut(&viewer, layer, effect, &file);
            announce(&viewer, said);
            refresh(&handle);
        });
}

/// D-182: `file` as the Color Lookup's file. One the reading rule refuses changes nothing and
/// says why; one the project has already is named again rather than brought in twice; a new one
/// comes in as a lookup asset in the same entry to undo as the setting.
/// D-395: an Arbitrary Map's .amp file the same way, its phase and alpha setting kept.
fn choose_lut(viewer: &Mutex<Viewer>, layer_id: Id, instance_id: Id, file: &Path) -> String {
    let name = file.file_name().map_or_else(
        || file.display().to_string(),
        |n| n.to_string_lossy().into_owned(),
    );
    let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
    let now = effect_now_in(held, &layer_id, &instance_id);
    let amp = matches!(now, Some(Effect::ArbitraryMap { .. }));
    if let Err(why) = read_lookup_file(file, amp) {
        return format!("{name} was not used: {why}. Nothing was changed.");
    }
    let stored = persist::stored_path(&held.root, file);
    let project = held.document.project();
    let mut commands = Vec::new();
    let lut = match project
        .assets
        .iter()
        .find(|a| a.kind == AssetKind::Lut && a.path.as_deref() == Some(stored.as_str()))
    {
        Some(a) => a.id.clone(),
        None => {
            let stem = file
                .file_stem()
                .map_or_else(|| name.clone(), |s| s.to_string_lossy().into_owned());
            let mut asset = Asset::still(unused_asset_id(project), stem, stored);
            asset.kind = AssetKind::Lut;
            let id = asset.id.clone();
            commands.push(Command::AddAsset { asset });
            id
        }
    };
    // D-397: a Color Grade keeps its other settings and takes the file as its look.
    let (chosen, what) = match now {
        Some(Effect::ArbitraryMap {
            phase,
            apply_to_alpha,
            ..
        }) => (
            Effect::ArbitraryMap {
                map: lut.as_str().to_string(),
                phase,
                apply_to_alpha,
                table: None,
            },
            "Arbitrary Map",
        ),
        Some(Effect::ColorGrade {
            values,
            curves,
            hue_saturation,
            more_curves,
            secondary_wheels,
            key_invert,
            key_view,
            ..
        }) => (
            Effect::ColorGrade {
                values,
                look: lut.as_str().to_string(),
                curves,
                hue_saturation,
                more_curves,
                secondary_wheels,
                key_invert,
                key_view,
                table: None,
            },
            "Color Grade",
        ),
        _ => (
            Effect::ColorLookup {
                lut: lut.as_str().to_string(),
                table: None,
            },
            "Color Lookup",
        ),
    };
    commands.push(Command::SetEffectParameters {
        composition: held.composition.clone(),
        layer_id,
        instance_id,
        effect: chosen,
    });
    match held.document.apply_all(commands) {
        Ok(_) => format!("{what} now uses {name}."),
        Err(diagnostic) => diagnostic.sentence(),
    }
}

/// The presets are the window's, not the project's, so the page is handed them rather than
/// reloaded; the sentence goes to the status line as well.
fn tell_the_page_about_presets(app: &AppHandle, list: Option<serde_json::Value>, said: String) {
    announce(&app.state::<Mutex<Viewer>>(), said.clone());
    if let Some(page) = app.get_webview_window("main") {
        let list = list.unwrap_or(serde_json::Value::Null);
        let _ = page.eval(format!(
            "receivePresets({list}, {})",
            serde_json::Value::String(said)
        ));
    }
}

/// D-181: the starter presets as the page keeps presets, `{name, about, effects}`, the effects as
/// the file wrote them, since `effect.paste` reads them by the same rules again. D-739: then the
/// starter animation presets, whole with their `about`, since `preset.apply` reads them again.
fn starter_presets() -> String {
    let root: serde_json::Value =
        serde_json::from_str(persist::STARTER_PRESETS).expect("the starter presets are JSON");
    let mut list: Vec<serde_json::Value> = root["presets"]
        .as_array()
        .expect("the starter presets are a list")
        .iter()
        .zip(persist::STARTER_ABOUT)
        .map(|(p, about)| serde_json::json!({ "name": p["name"], "about": about, "effects": p["effects"] }))
        .collect();
    let moving: serde_json::Value =
        serde_json::from_str(persist::STARTER_ANIMATIONS).expect("the starter animations are JSON");
    for (p, about) in moving["presets"]
        .as_array()
        .expect("the starter animations are a list")
        .iter()
        .zip(persist::STARTER_ANIMATIONS_ABOUT)
    {
        let mut p = p.clone();
        p["about"] = about.into();
        list.push(p);
    }
    serde_json::Value::Array(list).to_string()
}

/// D-180: a preset file's presets as the window keeps them, with the sentence saying what came
/// in, or the sentence refusing it whole. A name in `have` comes in with " 2", or the next number
/// free both in the window and in the file.
fn imported_presets(
    text: &str,
    file: &str,
    have: &[String],
) -> Result<(serde_json::Value, String), String> {
    let read = persist::read_presets(text).map_err(|refused| refused.sentence())?;
    // Each preset goes on as the file wrote it; `effect.paste` and `preset.apply` read it by the
    // same rules again.
    let root: serde_json::Value = serde_json::from_str(text).expect("read_presets read it as JSON");
    let mut taken: Vec<String> = have
        .iter()
        .cloned()
        .chain(read.iter().map(|p| p.name.clone()))
        .collect();
    let mut renamed = Vec::new();
    let mut list = Vec::new();
    for (i, preset) in read.iter().enumerate() {
        let mut name = preset.name.clone();
        if have.contains(&name) {
            let n = (2..)
                .find(|n| !taken.contains(&format!("{name} {n}")))
                .expect("a free number");
            let new = format!("{name} {n}");
            renamed.push(format!("\"{name}\" as \"{new}\""));
            taken.push(new.clone());
            name = new;
        }
        let mut whole = root["presets"][i].clone();
        whole["name"] = name.into();
        list.push(whole);
    }
    let mut said = match list.len() {
        1 => format!("Imported 1 preset from {file}."),
        n => format!("Imported {n} presets from {file}."),
    };
    if !renamed.is_empty() {
        said += &format!(
            " Names you already had came in with a number: {}.",
            renamed.join(", ")
        );
    }
    Ok((serde_json::Value::Array(list), said))
}

#[cfg(test)]
#[path = "tests/panics.rs"]
mod panics;

#[cfg(test)]
#[path = "tests/preset_files.rs"]
mod preset_files;

/// Reload the page, which is the whole of the update after a command that changed what is open.
///
/// The page holds no state about the project — everything it says arrives with a frame — so
/// there is nothing to keep in step and nothing to invalidate.
fn refresh(app: &AppHandle) {
    if let Some(page) = app.get_webview_window("main") {
        let _ = page.eval("location.reload()");
    }
}

/// B-45, D-102: where the viewer's picture lies in the window, and what is under it, from the page.
///
/// `p` is one paint per `;`, each sixteen numbers: kind, the box that shows (left, top, right,
/// bottom), the whole box, colour and second colour (0 to 255), and the checkerboard's square,
/// all in window pixels. The first time, with the switch on, the card starts painting the window
/// and the page is made see-through. The answer is `on` when the card paints the window.
fn place(app: &AppHandle, query: Option<&str>) -> Response<Vec<u8>> {
    let paints: Vec<Paint> = parameter(query, "p")
        .unwrap_or_default()
        .split(';')
        .filter_map(|item| {
            let n: Vec<f32> = item
                .split(',')
                .filter_map(|n| n.trim().parse().ok())
                .collect();
            let [kind, l, t, r, b, ol, ot, or, ob, r1, g1, b1, r2, g2, b2, square] = n[..] else {
                return None;
            };
            Some(Paint {
                kind: kind as u32,
                rect: [l, t, r, b],
                origin: [ol, ot, or, ob],
                colour: [r1 / 255.0, g1 / 255.0, b1 / 255.0, 1.0],
                colour2: [r2 / 255.0, g2 / 255.0, b2 / 255.0, 1.0],
                square,
                pad: [0; 2],
            })
        })
        .collect();
    let answer = |said: &str| {
        allow_the_page_to_read_this(Response::builder())
            .header("content-type", "text/plain; charset=utf-8")
            .body(said.as_bytes().to_vec())
            .expect("build the place response")
    };
    // W-41b (D-250): nothing to paint is the page asking for the picture's pixels again (a turned
    // or mirrored view, a snapshot, a compare). The card stops painting until the next place.
    if paints.is_empty() {
        app.state::<Mutex<Card>>()
            .lock()
            .expect("the card lock was poisoned")
            .paints
            .clear();
    }
    let (Some(window), false) = (app.get_webview_window("main"), paints.is_empty()) else {
        return answer("off");
    };
    let alpha_only = app
        .state::<Mutex<Viewer>>()
        .lock()
        .expect("the viewer lock was poisoned")
        .alpha_only;
    let state = app.state::<Mutex<Card>>();
    let card = &mut *state.lock().expect("the card lock was poisoned");
    let Card {
        on: true,
        gpu: Some(Ok(gpu)),
        paints: placed,
        size,
        off_screen: off_screen @ None,
        ..
    } = card
    else {
        return answer("off");
    };
    if !gpu.on_screen() {
        // SAFETY: the card lets go of the window when it is destroyed (`on_window_event`).
        let attached = window
            .set_background_color(Some(tauri::window::Color(0, 0, 0, 0)))
            .map_err(|e| e.to_string())
            .and_then(|()| unsafe { gpu.attach(&window) });
        if let Err(why) = attached {
            let _ = window.set_background_color(None);
            *off_screen = Some(why);
            return answer("off");
        }
    }
    let inner = window.inner_size().unwrap_or_default();
    *size = (inner.width, inner.height);
    *placed = paints;
    // The picture already on the card, where the page now has it: a zoom or a scroll costs no render.
    if let Err(why) = gpu.show(*size, placed, alpha_only) {
        gpu.let_go();
        *off_screen = Some(why);
        return answer("off");
    }
    answer("on")
}

/// Answer one command from the page. The body is what the status line should say, or the recent
/// list, one path per line.
fn command(app: &AppHandle, path: &str, query: Option<&str>) -> Response<Vec<u8>> {
    let viewer = app.state::<Mutex<Viewer>>();
    let path = path.trim_matches('/');
    // The one answer that is not a sentence. It is large, it is asked for after every edit, and
    // it is JSON, so it leaves here rather than through the status line the rest of these share.
    if path == "state" {
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(state(&viewer).into_bytes())
            .expect("build the state response");
    }
    // P-19: the session log panel's one question, and its switch, Clear and Save a copy. Every
    // one of them is answered with what the panel should now show.
    if path == "session-log" {
        let session = app.state::<Mutex<SessionLog>>();
        let what = parameter(query, "do");
        if what.as_deref() == Some("save") {
            ask_where_to_save_the_log(app);
        }
        let mut log = session.lock().expect("the session log lock was poisoned");
        match what.as_deref() {
            Some("on") => log.switch(true),
            Some("off") => log.switch(false),
            Some("clear") => log.clear(),
            _ => {}
        }
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(session_log_json(&log).into_bytes())
            .expect("build the session log response");
    }
    // B-44: the CPU / GPU switch. The window's, like the alpha view, so it is answered here
    // rather than by `edit_command`, which has only the viewer. B-48: `to` is auto, gpu or cpu.
    if path == "gpu-switch" {
        let mode = match parameter(query, "to").as_deref() {
            Some("gpu") => DrawOn::Gpu,
            Some("cpu") => DrawOn::Cpu,
            _ => DrawOn::Auto,
        };
        let said = app
            .state::<Mutex<Card>>()
            .lock()
            .expect("the card lock was poisoned")
            .set(mode);
        announce(&viewer, said.clone());
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "text/plain; charset=utf-8")
            .body(said.into_bytes())
            .expect("build the command response");
    }
    // B-48, D-105: the memory setting, and what is in use. `ram` and `card` are gigabytes, or
    // `auto`; a number is kept between 1 GiB and the most D-105 allows.
    // D-181: the starter presets, the same on every window, each with the sentence the panel shows.
    if path == "presets-builtin" {
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(starter_presets().into_bytes())
            .expect("build the starter presets response");
    }
    if path == "memory" {
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(memory(app, &viewer, query).into_bytes())
            .expect("build the memory response");
    }
    // D-257: the files gone from the disk, asked every few seconds for the health chip.
    if path == "files-gone" {
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(files_gone(&viewer).to_string().into_bytes())
            .expect("build the files gone response");
    }
    // D-264: every font the text layer can draw, by file, family and style, for the Character
    // section's two lists; and one font's bytes, for the page to show the words being typed in it.
    if path == "fonts" {
        let list: Vec<serde_json::Value> = anime_compositor::text::fonts()
            .iter()
            .map(|f| serde_json::json!({ "file": f.file, "family": f.family, "style": f.style }))
            .collect();
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(serde_json::Value::from(list).to_string().into_bytes())
            .expect("build the fonts response");
    }
    if path == "font" {
        let name = parameter(query, "name").unwrap_or_default();
        return match anime_compositor::text::font_bytes(name.trim()) {
            Some(bytes) => allow_the_page_to_read_this(Response::builder())
                .header("content-type", "font/ttf")
                .body(bytes.to_vec())
                .expect("build the font response"),
            None => allow_the_page_to_read_this(Response::builder().status(404))
                .header("content-type", "text/plain; charset=utf-8")
                .body(format!("This computer has no font \"{}\".", name.trim()).into_bytes())
                .expect("build the font-missing response"),
        };
    }
    // D-251: the render queue, `?do=` add, tick, remove, move, refresh or render; the list after.
    if path == "queue" {
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(queue(app, query).to_string().into_bytes())
            .expect("build the render queue response");
    }
    // D-260: the Project panel's small pictures, `?item=` and, for a composition, `&frame=`.
    // D-252: the job being written, as the Render panel watches it, and its frames, `?frame=`.
    if path == "export-watch" {
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "application/json; charset=utf-8")
            .body(
                watching(&app.state::<Mutex<Export>>())
                    .to_string()
                    .into_bytes(),
            )
            .expect("build the export watch response");
    }
    if path == "export-frame" {
        let frame = parameter(query, "frame")
            .and_then(|f| f.parse().ok())
            .unwrap_or(0);
        return match watched_frame(&app.state::<Mutex<Export>>(), frame) {
            Some((width, height, pixels)) => allow_the_page_to_read_this(Response::builder())
                .header("content-type", "application/octet-stream")
                .header("x-width", width.to_string())
                .header("x-height", height.to_string())
                .body(pixels)
                .expect("build the watched frame response"),
            None => allow_the_page_to_read_this(Response::builder().status(404))
                .header("content-type", "text/plain; charset=utf-8")
                .body(format!("frame {frame} is not one of the frames last written").into_bytes())
                .expect("build the watched frame refusal"),
        };
    }
    if path == "thumb" {
        let item = Id::new(parameter(query, "item").unwrap_or_default());
        let frame = parameter(query, "frame")
            .and_then(|f| f.parse().ok())
            .unwrap_or(0);
        return match thumbnail(&viewer, &item, frame) {
            Ok((width, height, pixels)) => allow_the_page_to_read_this(Response::builder())
                .header("content-type", "application/octet-stream")
                .header("x-width", width.to_string())
                .header("x-height", height.to_string())
                .body(pixels)
                .expect("build the thumbnail response"),
            Err(why) => allow_the_page_to_read_this(Response::builder().status(404))
                .header("content-type", "text/plain; charset=utf-8")
                .body(why.into_bytes())
                .expect("build the thumbnail refusal"),
        };
    }
    // D-258: one layer by itself, `?layer=&frame=&q=`, for the onion skin. Never on screen as it is.
    if path == "alone" {
        let layer = Id::new(parameter(query, "layer").unwrap_or_default());
        let frame = parameter(query, "frame")
            .and_then(|f| f.parse().ok())
            .unwrap_or(0);
        let quality = quality_asked(query)
            .unwrap_or_else(|| viewer.lock().expect("the viewer lock was poisoned").quality);
        return match alone(&viewer, &layer, frame, quality) {
            Ok((width, height, pixels)) => allow_the_page_to_read_this(Response::builder())
                .header("content-type", "application/octet-stream")
                .header("x-width", width.to_string())
                .header("x-height", height.to_string())
                .body(pixels)
                .expect("build the layer alone response"),
            Err(why) => allow_the_page_to_read_this(Response::builder().status(404))
                .header("content-type", "text/plain; charset=utf-8")
                .body(why.into_bytes())
                .expect("build the layer alone refusal"),
        };
    }
    // An import with no files named is the button in the media bin, and what it needs is the
    // operating system's file dialog, which belongs to the app handle and not to the viewer.
    // Answered before `edit_command`, which would otherwise refuse it for naming no files.
    // The same for the first step of a relink, which needs the dialog for the same reason. The
    // other two steps carry `apply` or `cancel` and are answered by `edit_command` below.
    if path == "media.relink"
        && parameter(query, "file").is_none()
        && parameter(query, "apply").is_none()
        && parameter(query, "cancel").is_none()
    {
        if let Some(asset) = parameter(query, "asset") {
            ask_what_to_relink_to(app, Id::new(&asset));
            return allow_the_page_to_read_this(Response::builder())
                .header("content-type", "text/plain; charset=utf-8")
                .body(Vec::new())
                .expect("build the relink response");
        }
    }
    if path == "media.import" && parameter(query, "file").is_none() {
        ask_what_to_import(app);
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "text/plain; charset=utf-8")
            .body(Vec::new())
            .expect("build the import response");
    }
    // D-84: Import cut with no folder named is the button, and asks for the folder in the same way.
    if path == "timesheet.import" && parameter(query, "folder").is_none() {
        ask_which_cut(app);
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "text/plain; charset=utf-8")
            .body(Vec::new())
            .expect("build the import response");
    }
    // An edit answers here rather than falling through the match below, because its answer is
    // also what the status line should say: the page asks for the frame again straight
    // afterwards, and that answer carries `x-status`, so a sentence not written into the viewer
    // would be replaced by the one before it before anybody could read it.
    if let Some(said) = edit_command(&viewer, path, query) {
        announce(&viewer, said.clone());
        return allow_the_page_to_read_this(Response::builder())
            .header("content-type", "text/plain; charset=utf-8")
            .body(said.into_bytes())
            .expect("build the command response");
    }
    let said = match path {
        "recent" => recent(app).join("\n"),
        // With a path, the recent list chose it. Without one, ask. Both end at `take`.
        // P-25: a path not on the recent list is refused, as `recover` refuses one it did not
        // offer: a command scheme is reachable by anything running in the page.
        "open" => match parameter(query, "path") {
            Some(chosen) if !recent(app).contains(&chosen) => {
                format!("{chosen} is not on the recent list. Choose it with the Open button instead.")
            }
            Some(chosen) => {
                let chosen = PathBuf::from(chosen);
                if take(&viewer, &chosen) {
                    remember(app, &chosen);
                }
                viewer
                    .lock()
                    .expect("the viewer lock was poisoned")
                    .status
                    .clone()
            }
            None => {
                ask_to_open(app);
                String::new()
            }
        },
        "save" => {
            let said = save(&viewer);
            announce(&viewer, said.clone());
            // A project with nowhere to go asks where, rather than refusing and leaving the
            // person to find the other button.
            if viewer
                .lock()
                .expect("the viewer lock was poisoned")
                .path
                .is_none()
            {
                ask_where_to_save(app);
            }
            said
        }
        "save-as" => {
            ask_where_to_save(app);
            String::new()
        }
        // D-732: Increment and Save. A project with no file yet has no name to count up from,
        // so it asks where, as Save does.
        "save-increment" => {
            match increment_and_save(&viewer) {
                Some(said) => {
                    announce(&viewer, said.clone());
                    if let Some(path) = viewer.lock().expect("the viewer lock was poisoned").path.clone() {
                        remember(app, &path);
                    }
                    said
                }
                None => {
                    ask_where_to_save(app);
                    String::new()
                }
            }
        }
        // D-180: the window's presets to a file and back. Neither touches the project.
        "presets-export" => ask_where_to_export_presets(
            app,
            &parameter(query, "name").unwrap_or_else(|| "presets".to_string()),
            &parameter(query, "presets").unwrap_or_default(),
        ),
        "presets-import" => {
            ask_which_presets_to_import(app, parameters(query, "have"));
            String::new()
        }
        // D-182: a Color Lookup's card chooses its .cube file.
        "lut-choose" => match (parameter(query, "layer"), parameter(query, "effect")) {
            (Some(layer), Some(effect)) => {
                ask_which_lut(app, Id::new(layer), Id::new(effect));
                String::new()
            }
            _ => "Which effect? Choose the file from a Color Lookup's card.".to_string(),
        },
        // The page only ever offers a path it was given in `x-recovery`, but this checks anyway:
        // a command scheme is reachable by anything running in the page.
        "recover" => match parameter(query, "path") {
            Some(chosen) => {
                let chosen = PathBuf::from(chosen);
                let known = viewer
                    .lock()
                    .expect("the viewer lock was poisoned")
                    .recovery
                    .contains(&chosen);
                if known {
                    recover(&viewer, &chosen)
                } else {
                    format!(
                        "{} is not a recovery snapshot for this project.",
                        chosen.display()
                    )
                }
            }
            None => "Which snapshot? Choose one from the recovery list.".to_string(),
        },
        // Document 07's default is that a missing drawing blocks a final export. `?missing=write`
        // is the person overriding it in front of the checkbox that says what it does, which is
        // document 28's recorded override rather than a silent fallback.
        // D-62: `?format=` is the page's format list; a value it does not offer is refused
        // before any folder is asked for.
        // D-73: a quality the page does not offer is refused the same way.
        "export" => match output_format(query).and_then(|f| Ok((f, export_choices(query)?))) {
            Ok((format, choices)) => {
                ask_where_to_export(app, missing_source(query), format, choices);
                String::new()
            }
            Err(refused) => refused,
        },
        "cancel-export" => cancel_export(app),
        "pause-export" => pause_export(&app.state::<Mutex<Export>>()),
        "collect" => {
            ask_where_to_collect(app);
            String::new()
        }
        "check-package" => {
            let said = check_package(&viewer);
            announce(&viewer, said.clone());
            said
        }
        // W-24: document 24's `project.new`. The page asks twice first when there is unsaved
        // work; whatever was unsaved is still in its recovery snapshot.
        "new" => {
            let held = &mut *viewer.lock().expect("the viewer lock was poisoned");
            *held = blank(held.root.clone());
            held.status = "A new, empty project. Save As gives it a file.".to_string();
            held.status.clone()
        }
        _ => {
            return allow_the_page_to_read_this(Response::builder().status(404))
                .header("content-type", "text/plain; charset=utf-8")
                .body(
                    b"ask for /state, /open, /save, /save-as, /save-increment, /recover, /export, \
                      /cancel-export, /pause-export, /export-watch, /export-frame, /collect, /check-package, /recent, /new, /session-log, /gpu-switch, /memory, \
                      /files-gone, /fonts, /font, /thumb, /alone, /queue, /presets-builtin, /presets-export, /presets-import, /lut-choose, /sketch-png, or one of \
                      document 24's command IDs"
                        .to_vec(),
                )
                .expect("build the not-found response")
        }
    };
    allow_the_page_to_read_this(Response::builder())
        .header("content-type", "text/plain; charset=utf-8")
        .body(said.into_bytes())
        .expect("build the command response")
}

/// R-11 asks that nothing leaves the device. Nothing in this program tries to; the web view
/// component Windows supplies does, on its own account, and `tools/offline_check.ps1` caught it
/// holding connections to Microsoft addresses in twenty idle seconds. That is not this
/// application's code and it cannot be removed. These are the switches Chromium offers for it —
/// no background networking, no component updates, no reporting, no pings, no sync — and they are
/// set because leaving them unset would be a choice too.
///
/// They do not close it. The same twenty seconds with these on still showed connections, which is
/// recorded in `verification/B-11_offline_run.md` and registered as a decision for the owner
/// rather than described as solved. Do not delete these arguments on the grounds that they did not
/// work; they narrow what is left to explain.
///
/// B-113 (D-173) added the last two. The page only ever asks for `*.localhost` addresses, which
/// the web view answers inside the process, so every other name is made unresolvable and every
/// other request is sent to a proxy on a closed local port. That removed the DNS-over-HTTPS
/// connection; one to a Microsoft address remains, made outside Chromium's network code, where
/// no switch reaches. `verification/B-113_offline_run.md` has the runs.
///
/// Anything already in the variable is kept and appended to, because `tools/capture_window.ps1`
/// uses it to photograph the window at other display scales and overwriting it would silently
/// undo that.
fn quieten_the_web_view() {
    const OURS: &str = "--disable-background-networking --disable-component-update \
                        --disable-domain-reliability --no-pings --disable-sync \
                        --disable-features=DnsOverHttps,msSmartScreenProtection,msWebOOUI,msPdfOOUI \
                        --proxy-server=127.0.0.1:9 \
                        --host-resolver-rules=\"MAP * ~NOTFOUND , EXCLUDE *.localhost\"";
    const NAME: &str = "WEBVIEW2_ADDITIONAL_BROWSER_ARGUMENTS";
    let value = match std::env::var(NAME) {
        Ok(existing) if !existing.trim().is_empty() => format!("{OURS} {existing}"),
        _ => OURS.to_string(),
    };
    std::env::set_var(NAME, value);
}

/// Audit B7: `job`, with a panic inside it caught and said as a sentence instead of closing the
/// window.
fn caught<T>(what: &str, job: impl FnOnce() -> T) -> Result<T, String> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(job)).map_err(|e| {
        let why = e
            .downcast_ref::<&str>()
            .map(|s| s.to_string())
            .or_else(|| e.downcast_ref::<String>().cloned())
            .unwrap_or_else(|| "no reason was given".to_string());
        format!(
      "Something went wrong inside the program while {what}: {why}. That was stopped and the window kept going. If anything looks wrong, save under a new name."
        )
    })
}

/// [`caught`] in the running window: the locks the panic left marked are released as they
/// stand, so the next request does not fail on them, and the status line says what happened.
fn contained<T>(app: &AppHandle, what: &str, job: impl FnOnce() -> T) -> Result<T, String> {
    caught(what, job).inspect_err(|said| {
        app.state::<Mutex<Viewer>>().clear_poison();
        app.state::<Mutex<Export>>().clear_poison();
        app.state::<Mutex<SessionLog>>().clear_poison();
        app.state::<Mutex<Card>>().clear_poison();
        app.state::<Mutex<[Option<Placement>; 2]>>().clear_poison();
        announce(&app.state::<Mutex<Viewer>>(), said.clone());
    })
}

/// A request that panicked, answered with what [`contained`] said.
fn failed_response(said: String) -> Response<Vec<u8>> {
    allow_the_page_to_read_this(Response::builder().status(500))
        .header("content-type", "text/plain; charset=utf-8")
        .body(said.into_bytes())
        .expect("build the failed response")
}

fn main() {
    quieten_the_web_view();
    // A project named on the command line goes through the same `take` a dropped one does, so
    // the two ways in cannot behave differently. It is also the only one a script can drive,
    // which is how the photographs under `verification/` are taken.
    let viewer = Mutex::new(demo());
    if let Some(named) = std::env::args_os().nth(1) {
        take(&viewer, Path::new(&named));
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .manage(viewer)
        .manage(Mutex::new(Export::default()))
        // P-19: empty and switched off at every launch, and gone when the window closes.
        .manage(Mutex::new(SessionLog::default()))
        // B-44: the CPU / GPU switch, on the CPU at every launch.
        .manage(Mutex::new(Card::default()))
        // B-174: the window's last place while neither maximized nor minimized.
        .manage(Mutex::new([None::<Placement>; 2]))
        // The autosave timer. A thread rather than anything cleverer: it sleeps for all but a
        // few microseconds of its life, it must run whether or not the page is asking for
        // frames, and it holds the viewer lock only for as long as the check takes. It writes
        // nothing to the page — a reload while somebody is working would be a worse
        // interruption than the one it is protecting them from — so what it did appears with
        // the next frame, which is within a sixtieth of a second of it happening.
        .setup(|app| {
            // D-256: where a project that was never saved keeps its copies.
            if let Ok(dir) = app.path().app_local_data_dir() {
                let folder = dir.join("Unsaved");
                offer_unsaved(
                    &mut app
                        .state::<Mutex<Viewer>>()
                        .lock()
                        .expect("the viewer lock was poisoned"),
                    &folder,
                );
                *UNSAVED
                    .lock()
                    .expect("the unsaved folder lock was poisoned") = Some(folder);
            }
            let handle = app.handle().clone();
            std::thread::spawn(move || loop {
                std::thread::sleep(AUTOSAVE_TICK);
                let _ = contained(&handle, "autosaving", || {
                    let state = handle.state::<Mutex<Viewer>>();
                    let mut viewer = state.lock().expect("the viewer lock was poisoned");
                    if let Some(said) = autosave_tick(&mut viewer, Instant::now()) {
                        viewer.autosaved = said;
                    }
                });
            });
            open_window(app.handle())?;
            Ok(())
        })
        .on_window_event(|window, event| {
            follow_window(window, event);
            // B-45: the card lets go of the window before the window goes.
            if let WindowEvent::Destroyed = event {
                if let Some(Ok(gpu)) = &mut window
                    .state::<Mutex<Card>>()
                    .lock()
                    .expect("the card lock was poisoned")
                    .gpu
                {
                    gpu.let_go();
                }
            }
            let WindowEvent::DragDrop(DragDropEvent::Drop { paths, .. }) = event else {
                return;
            };
            let Some(path) = paths.first() else { return };
            let viewer = window.state::<Mutex<Viewer>>();
            // The owner's B-16c playtest: drawings dropped on the window are imported, as Import
            // drawings... would, and one file on its own is a still there too.
            // D-71: sound files dropped on the window are imported the same way.
            if paths.iter().all(|p| is_drawing(p)) || paths.iter().all(|p| is_sound(p)) {
                let said = import(&viewer, paths);
                announce(&viewer, said);
                refresh(window.app_handle());
                return;
            }
            if take(&viewer, path) {
                remember(window.app_handle(), path);
            }
            refresh(window.app_handle());
        })
        .register_uri_scheme_protocol("project", |ctx, request: Request<Vec<u8>>| {
            contained(ctx.app_handle(), "doing what was asked", || {
                // D-289: the page sends the sketch's PNG bytes as the body; the person chooses where.
                if request.uri().path().trim_matches('/') == "sketch-png" {
                    let said = ask_where_to_save_sketch(
                        ctx.app_handle(),
                        &parameter(request.uri().query(), "name")
                            .unwrap_or_else(|| "sketch".to_string()),
                        request.body().clone(),
                    );
                    return allow_the_page_to_read_this(Response::builder())
                        .header("content-type", "text/plain; charset=utf-8")
                        .body(said.into_bytes())
                        .expect("build the sketch response");
                }
                command(
                    ctx.app_handle(),
                    request.uri().path(),
                    request.uri().query(),
                )
            })
            .unwrap_or_else(failed_response)
        })
        .register_uri_scheme_protocol("frame", |ctx, request: Request<Vec<u8>>| {
            contained(ctx.app_handle(), "drawing a frame", || {
                let viewer = ctx.app_handle().state::<Mutex<Viewer>>();
                let export = ctx.app_handle().state::<Mutex<Export>>();
                // `/boxes/<n>` rides on the frame scheme rather than the command one because it is
                // the same kind of thing a frame is: a question about what is on screen, whose
                // answer changes nothing and needs no undo record.
                if let Some(n) = request
                    .uri()
                    .path()
                    .trim_matches('/')
                    .strip_prefix("boxes/")
                    .and_then(|n| n.parse::<i32>().ok())
                {
                    return boxes(&viewer, n, quality_asked(request.uri().query()));
                }
                // D-71: `/sound?asset=`, one sound file's bytes for the page's decoder.
                if request.uri().path().trim_matches('/') == "sound" {
                    return sound(&viewer, request.uri().query());
                }
                // D-640: `/mix?from=&frames=`, the composition's sound as the core mixes it.
                if request.uri().path().trim_matches('/') == "mix" {
                    return mix(&viewer, request.uri().query());
                }
                // `/curve?layer=&prop=&from=&to=`, the graph editor's samples: the same kind of
                // question as `boxes` above, on the same scheme, for the same reason.
                if request.uri().path().trim_matches('/') == "curve" {
                    return curve(&viewer, request.uri().query());
                }
                // D-84a: `/sheet`, the Sheet tab's grid, for the same reason again.
                if request.uri().path().trim_matches('/') == "sheet" {
                    return sheet(&viewer);
                }
                // B-45: `/place?p=`, where the viewer's picture and what is under it lie in the window.
                if request.uri().path().trim_matches('/') == "place" {
                    return place(ctx.app_handle(), request.uri().query());
                }
                // W-40c (D-249): `/ready`, the frames the green line under the ruler marks.
                if request.uri().path().trim_matches('/') == "ready" {
                    let card = ctx.app_handle().state::<Mutex<Card>>();
                    return allow_the_page_to_read_this(Response::builder())
                        .header("content-type", "application/json; charset=utf-8")
                        .body(ready_frames(&viewer, Some(&card)).to_string().into_bytes())
                        .expect("build the ready response");
                }
                // D-84d: `/sheet/print`, the same grid as a page to print.
                if request.uri().path().trim_matches('/') == "sheet/print" {
                    return sheet_print(&viewer, request.uri().query());
                }
                match parse(request.uri().path(), request.uri().query()) {
                    Some((ask, quality)) => {
                        let session = ctx.app_handle().state::<Mutex<SessionLog>>();
                        let card = ctx.app_handle().state::<Mutex<Card>>();
                        // B-158: `v=`, the part of the picture on screen.
                        let seen = parameter(request.uri().query(), "v");
                        let reply = serve_seen(
                            &viewer,
                            &export,
                            Some(&session),
                            Some(&card),
                            ask,
                            quality,
                            seen.as_deref(),
                        );
                        // B-154: while playing, the rest of the loop is made in the background, one
                        // worker at a time.
                        static AHEAD: AtomicBool = AtomicBool::new(false);
                        if matches!(ask, Ask::At(_) | Ask::Play(_))
                            && !AHEAD.swap(true, Ordering::SeqCst)
                        {
                            let app = ctx.app_handle().clone();
                            std::thread::spawn(move || {
                                let _ = contained(&app, "drawing frames ahead", || {
                                    render_ahead(
                                        &app.state::<Mutex<Viewer>>(),
                                        Some(&app.state::<Mutex<Card>>()),
                                    )
                                });
                                AHEAD.store(false, Ordering::SeqCst);
                            });
                        }
                        reply
                    }
                    None => allow_the_page_to_read_this(Response::builder().status(404))
                        .header("content-type", "text/plain; charset=utf-8")
                        .body(
                            b"ask for /at/<milliseconds>, /frame/<number> or /play/<number>"
                                .to_vec(),
                        )
                        .expect("build the not-found response"),
                }
            })
            .unwrap_or_else(failed_response)
        })
        .run(tauri::generate_context!())
        .expect("the window could not be created");
}

/// What saving from the window does, checked without a window.
///
/// The dialogs cannot be reached from a test — a file dialog is the operating system's and a
/// test has no hands — but everything on this side of the person's answer can be, and that is
/// where a save can go wrong quietly. The three questions worth asking are whether the file that
/// arrives is the project that was open, whether the parts of it this build does not model
/// survive the trip, and whether a failed save leaves the previous file alone.
///
/// Writes `verification/B-09_save_table.md`.
#[cfg(test)]
#[path = "tests/saving.rs"]
mod saving;

/// What the editing panels can do to the open document, checked without a window.
///
/// B-12a's first half. The core already knows how to add, delete, rename, reorder and hide a
/// layer, and `tests/` checks that it does. What could go wrong *here* is everything between a
/// row in a list and that core: a button that names a layer which is no longer there, a new
/// layer given an identifier something else is already using, a toggle that decides what a layer
/// currently is from a list drawn a minute ago, a refusal that leaves the person with a control
/// that sprang back and no sentence. None of those look like a failure at the time.
///
/// Writes `verification/B-12a_editing_table.md`.
#[cfg(test)]
#[path = "tests/editing.rs"]
mod editing;

/// What the autosave timer and the recovery path do, checked without a window.
///
/// B-09's other half. The core already knows how to write a recovery snapshot and how to find
/// one; what could go wrong here is everything around that — a timer that writes too early or
/// never, a snapshot that quietly becomes the file Save writes to, a recovered project the
/// window calls saved. None of those would look like a failure at the time. They would look
/// like a person's afternoon disappearing later.
///
/// Writes `verification/B-09_recovery_table.md`.
#[cfg(test)]
#[path = "tests/recovery_and_autosave.rs"]
mod recovery_and_autosave;

/// What the page is handed when it asks for a frame, checked without a page.
///
/// The renderer's picture is checked pixel by pixel elsewhere, and the playback clock is checked
/// in `verification/B-08_preview_table.md`. What is new here is the transport between them: which
/// frame comes back for a given request, how many bytes the picture is, whether the page is
/// allowed to read the answer at all, and whether everything the window has to *say* about the
/// frame - its number, its resolution, the project's name, the notes, the dirty flag - arrives
/// beside the same picture it describes.
///
/// The window itself cannot be started in a test, so `serve` is called directly with the same
/// arguments the request handler passes it. What that leaves out is named at the end of the table.
///
/// Writes `verification/B-08_shell_table.md`.
#[cfg(test)]
#[path = "tests/serving.rs"]
mod serving;

/// B-162 and B-163, D-233: what the viewer asks for while a hand is moving, and what it draws.
///
/// The decisions are the page's, in two functions between the `B-162 / B-163` markers in
/// `ui/index.html`. `SIMULATION` below runs those very lines under Node, around a copy of the
/// page's one-request-at-a-time mailbox (P-04, `show`), a window that makes one frame at a time,
/// and a hand on a pretend clock. The requests that come out are then asked of the real request
/// handler, so the pixels in the tables are the window's own.
///
/// Writes `verification/B-162_latest_request_table.md` and
/// `verification/B-163_refine_on_stop_table.md`.
#[cfg(test)]
#[path = "tests/latest_request_and_refine.rs"]
mod latest_request_and_refine;

/// What exporting from the window does, checked without a window.
///
/// The renderer's export path is already checked to the frame in `T-08_export_table.md` and to
/// the whole shot in `B-10_full_shot_table.md`. What is new here, and what nothing else can see,
/// is the part between the button and that path: which frames the window asks for, what it names
/// them, whether the project it writes is the one that was open when the person asked, and
/// whether a refusal, a cancellation and a failure each reach the person in words.
///
/// Writes `verification/B-10_export_table.md`.
#[cfg(test)]
#[path = "tests/exporting.rs"]
mod exporting;

#[cfg(test)]
#[path = "tests/tests.rs"]
mod tests;

/// The window against document 24, in both directions.
///
/// Every other table under `verification/` checks what a command *does*. These two check that
/// the command is *reachable*: that the page asks for the identifier document 24 names, that
/// something answers every identifier the page asks for, and that a shortcut the document
/// promises is bound to something. Both read the real files rather than a copy of them -
/// `app/ui/index.html` and `Markdown/24_UI_Command_and_Interaction_Map.md` - so a rename on one
/// side and not the other is what they are for.
///
/// This is the first test in this project that reads the page at all. It is string matching over
/// JavaScript and not a browser, which bounds it exactly: it can see which identifier a handler
/// names, and it cannot see whether the button that handler is attached to is on the screen. The
/// photographs beside it are what say that.
#[cfg(test)]
#[path = "tests/contract.rs"]
mod contract;

/// W-01 walked end to end, from an empty composition to an exported sequence.
///
/// B-12 is the owner's acceptance run of this workflow by hand. This is the same thirteen steps
/// driven by script first, for one reason: a defect found here costs a re-run of a test, and the
/// same defect found in the middle of the owner's run costs the run. It is not a substitute for
/// it -- a script cannot say whether the window was pleasant to use, whether a sentence read
/// well, or whether the picture looked right -- and it does not touch the page, which is
/// `verification/B-12b_page_table.md` and the photographs beside it.
///
/// Every step goes through the same function the window's URL scheme calls, with the same text
/// the page would put in it, so what is walked is the window from the request inwards.
///
/// Writes `verification/B-12b_w01_walkthrough.md` and `verification/B-12b_w01_frame_12.png`,
/// and leaves the exported sequence in `target/b12b_w01/frames`.
#[cfg(test)]
#[path = "tests/acceptance.rs"]
mod acceptance;

/// Where app tests make their scratch folders: one folder per test program, so two checkouts
/// testing at once (lanes) do not delete each other's files under the same fixed name.
#[cfg(test)]
fn test_temp() -> std::path::PathBuf {
    use std::hash::{Hash, Hasher};
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    std::env::current_exe().ok().hash(&mut hasher);
    std::env::temp_dir().join(format!("anime_compositor_tests_{:016x}", hasher.finish()))
}

/// The fixture document 08 line 41 declares, built the way every other timing test builds it.
/// Included rather than copied: `tests/common/mod.rs` is where this project keeps the fixture
/// builder, and P-04 is the fourth measurement to want it.
#[cfg(test)]
#[path = "../../tests/common/mod.rs"]
mod common;

/// P-04: whether the editor answers while the renderer works.
///
/// The question is not how fast a frame is. It is whether a person pressing a key during one
/// gets an answer, and until P-04 they did not: `serve` took the viewer lock and held it through
/// planning, rendering and the encode, so every command queued behind the frame.
///
/// Both tests here put a render thread and a command thread on the same viewer. The first runs
/// on every build and proves the property without a stopwatch: a command answered *strictly
/// inside* one frame's own interval cannot have waited for that frame. The second is the
/// measurement the entry asks for and is `#[ignore]`d, because a number is only worth reading
/// from a release build on the recorded machine.
#[cfg(test)]
#[path = "tests/responsiveness.rs"]
mod responsiveness;

/// B-154 (G3, D-223): RAM preview. A frame the viewer has made is remembered as it was sent, and
/// asked for again it is sent from memory; while playback runs, the frames it will ask for next
/// are made ahead of it. Checked through `serve_logged`, the whole-frame form of `serve_seen`,
/// which every frame on screen comes from, against frames made cold by a viewer that has remembered nothing.
///
/// Writes `verification/B-154_ram_preview_table.md`.
#[cfg(test)]
#[path = "tests/ram_preview.rs"]
mod ram_preview;

/// W-40c (D-249): `/ready`, the frames the green line under the ruler marks. Asked
/// after frames are made through `serve_logged`, the whole-frame form of `serve_seen`, which every
/// frame on screen comes from, and after the render-ahead fills the loop. A frame is ready only for the project, quality and
/// processor it was made with.
///
/// Writes `verification/W-40c_ready_frames_table.md`.
#[cfg(test)]
#[path = "tests/ready_frames_check.rs"]
mod ready_frames_check;

/// B-158 (G8, D-229): only the part of the picture on screen. Zoomed in, the page asks for the
/// part of the frame it can show (`v=left,top,right,bottom`, fractions of the picture) and the
/// window makes and sends those pixels alone. Every part must be byte for byte the same pixels
/// of the whole frame, which is what these checks compare, through `serve_seen`, the function
/// every frame on screen comes from, on the CPU and on the card.
///
/// Writes `verification/B-158_part_table.md`.
#[cfg(test)]
#[path = "tests/part_on_screen.rs"]
mod part_on_screen;

#[cfg(test)]
#[path = "tests/window_place.rs"]
mod window_place;

/// D-257: drawings whose file has gone from the disk. `/files-gone` changes nothing and lists the
/// project's files that are not on the disk now, so the top bar's health chip can count them.
/// The reference shot is copied first, so deleting a drawing touches no fixture.
///
/// Writes `verification/D-257_files_gone_table.md`.
#[cfg(test)]
#[path = "tests/files_gone_check.rs"]
mod files_gone_check;

/// D-260: the Project panel's small pictures. `/thumb` gives a footage item's first drawing or a
/// composition's frame, each side a whole number of times smaller. These checks shrink the
/// full pictures here, by their own loop, and compare every byte.
///
/// Writes `verification/D-260_thumbnails_table.md`.
#[cfg(test)]
#[path = "tests/thumbnail_check.rs"]
mod thumbnail_check;

/// D-258: the onion skin's one layer alone. `/alone` draws one layer of the composition on screen
/// by itself, for one request, as solo would, without soloing anything. These checks compare it
/// with the frame the window sends when that layer really is soloed.
///
/// Writes `verification/D-258_alone_table.md`.
#[cfg(test)]
#[path = "tests/alone_check.rs"]
mod alone_check;

/// D-256: recovery copies of a project that was never saved. Such a project has no folder of its
/// own, so its copies go into the app's own folder, five at most, and the next start offers them
/// on the recovery list as it would a saved project's.
///
/// The folder is handed in rather than read from the window's setting, so these checks never
/// share it with B-09's, which run at the same time and check there is no copy without one.
///
/// Writes `verification/D-256_unsaved_recovery_table.md`.
#[cfg(test)]
#[path = "tests/unsaved_recovery_check.rs"]
mod unsaved_recovery_check;

/// D-251: the render queue. Rows are added with their own format and folder, each holding a copy
/// of the project taken when it was added, and ▷ Render writes the ticked ones in turn. These
/// checks compare every row's folder with what one Export of the same format writes today.
///
/// Writes `verification/D-251_render_queue_table.md`.
#[cfg(test)]
#[path = "tests/render_queue_check.rs"]
mod render_queue_check;

#[cfg(test)]
#[path = "tests/watch_check.rs"]
mod watch_check;

/// D-325: a layer's outline is its own box, whatever its effects do to its picture. Gaussian Blur
/// grows the picture by three times its blur on every side; the outline grew with it, and every
/// mask point the page carries through that outline landed up and to the left by as much.
///
/// Writes `verification/D-325_blur_outline_table.md`.
#[cfg(test)]
#[path = "tests/blur_outline.rs"]
mod blur_outline;

/// D-332 (FX-KEYALL-005): every setting a key can be put on, on every effect the app adds, is
/// keyed, saved and opened again with the same keys. The file reader once read keys only on a
/// list of names of its own, which fell behind, and a project keyed on one left out was saved and
/// then would not open.
#[cfg(test)]
#[path = "tests/keyed_settings.rs"]
mod keyed_settings;

/// D-741: chosen keys of a mask's, a shape's and a group's numbers moved, deleted and reversed
/// together, as the timeline sends them.
#[cfg(test)]
#[path = "tests/key_requests.rs"]
mod key_requests;
/// B-619: the page's lists of shape, mask, group, repeater and path operation names are the core's.
#[cfg(test)]
#[path = "tests/name_lists.rs"]
mod name_lists;
#[cfg(test)]
#[path = "tests/stretched_values.rs"]
mod stretched_values;
