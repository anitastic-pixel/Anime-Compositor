//! B-157: the double-precision audit (GPU plan G6). What each effect the card draws costs it, so
//! the passes worth moving from double to single precision can be found, and the check that any
//! that is moved still draws within 1 level of 255 of the processor is the effect's own card
//! table (B-65, B-76, B-107, B-115, B-123, B-151), which are left as they are.

use std::fmt::Write as _;
use std::fs;

use anime_compositor::cache::CelCache;
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::gpu::Gpu;
use anime_compositor::model::{Id, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use serde_json::{json, Value};

mod common;
use common::repo;

/// Every effect the card draws, with the settings its own card check gives it (as
/// `tests/b156_gpu_adjust.rs` has them).
fn effects() -> Vec<(&'static str, &'static str, Value)> {
    vec![
        ("Radial Blur", "core.radial_blur", json!({"type": "spin", "amount": 12, "center": [35, 60]})),
        ("Bloom", "core.bloom", json!({"threshold": 60, "radius": 10, "intensity": 1, "streaks": "star", "length": 60, "angle": 15})),
        ("Directional Blur", "core.directional_blur", json!({"direction": 45, "length": 30})),
        ("Gaussian Blur", "core.gaussian_blur", json!({"sigma_px": 4})),
        ("Glow", "core.glow", json!({"based_on": "bright", "threshold": 60, "colors": [], "tolerance": 0, "radius": 30, "intensity": 1, "operation": "add", "tint": ""})),
        ("Curves", "core.curves", json!({"master": [[0, 0], [128, 150], [255, 255]], "red": [[0, 0], [64, 40], [192, 215], [255, 255]], "green": [[0, 0], [255, 255]], "blue": [[0, 20], [255, 235]]})),
        ("Levels", "core.levels", json!({"input_black": 20, "input_white": 230, "gamma": 1.6, "output_black": 10, "output_white": 245})),
        ("Hue/Saturation", "core.hue_saturation", json!({"hue": 40, "saturation": 30, "lightness": -10})),
        ("Gradient", "core.gradient", json!({"shape": "radial", "start": [50, 50], "end": [100, 100], "start_color": "#ffcc00", "end_color": "#3050ff", "start_opacity": 80, "end_opacity": 40, "blend": "multiply"})),
        ("Drop Shadow", "core.drop_shadow", json!({"color": "#102040", "opacity": 70, "direction": 135, "distance": 12, "softness": 9})),
        ("Lens Blur", "core.lens_blur", json!({"radius": 12, "edges": "transparent", "iris": "hexagon", "roundness": 20, "rotation": 15, "aspect": 1.3, "highlight_gain": 2, "highlight_threshold": 80})),
        ("Rim Light", "core.rim_light", json!({"color": "#ffe0a0", "direction": 45, "width": 4, "softness": 3, "intensity": 80, "blend": "screen"})),
        ("Outline", "core.outline", json!({"color": "#ffffff", "width": 3.5, "softness": 2, "opacity": 90})),
        ("Noise", "core.noise", json!({"amount": 12, "mode": "color", "seed": 7, "animate": "on"})),
        ("Chromatic Aberration", "core.chromatic_aberration", json!({"amount": 5, "center": [40, 60]})),
        ("Distance Gradation", "core.distance_gradation", json!({"color": "#3040a0", "width": 20, "opacity": 70, "invert": "off", "blend": "multiply"})),
        ("Light Rays", "core.light_rays", json!({"center": [50, 40], "length": 40, "threshold": 60, "intensity": 1.5, "color": "#ffe0b0"})),
        ("Exposure Flicker", "core.exposure_flicker", json!({"amount": 0.5, "hold": 3, "seed": 4})),
        ("Vignette", "core.vignette", json!({"amount": 60, "color": "#201030", "size": 90, "roundness": 50, "softness": 60, "center": [45, 55]})),
        ("Turbulent Displace", "core.turbulent_displace", json!({"amount": 12, "size": 50, "complexity": 3, "evolution": 30, "speed": 10, "seed": 5, "edges": "transparent"})),
        ("Fractal Noise", "core.fractal_noise", json!({"size": 80, "complexity": 5, "contrast": 130, "brightness": 5, "evolution": 0, "speed": 15, "seed": 3, "dark_color": "#102030", "light_color": "#ffe8c0", "opacity": 60, "blend": "screen"})),
        ("Gradient Map", "core.gradient_map", json!({"shadow_color": "#1a0a40", "midtone_color": "#c05060", "highlight_color": "#fff0c0", "midpoint": 40, "amount": 80})),
        ("Color Balance", "core.color_balance", json!({"shadows": [-20, 10, 30], "midtones": [15, -5, -10], "highlights": [10, 5, -25]})),
        ("Offset", "core.offset", json!({"shift": [37.5, -21.25]})),
        ("Invert", "core.invert", json!({"channel": "rgb", "amount": 80})),
        ("Brightness & Contrast", "core.brightness_contrast", json!({"brightness": 30, "contrast": 40})),
        ("Black & White", "core.black_white", json!({"reds": 120, "yellows": 110, "greens": -10, "cyans": -50, "blues": -50, "magentas": 120})),
        ("Posterize", "core.posterize", json!({"levels": 6})),
        ("Threshold", "core.threshold", json!({"level": 128})),
        ("Channel Mixer", "core.channel_mixer", json!({"red": [0, 0, 100, 0], "green": [0, 100, 0, 0], "blue": [100, 0, 0, 0], "monochrome": "off"})),
        ("Vibrance", "core.vibrance", json!({"vibrance": 40, "saturation": 20})),
        ("Leave Color", "core.leave_color", json!({"color": "#ff0000", "tolerance": 15, "softness": 10, "amount": 100})),
        ("Solarize", "core.solarize", json!({"threshold": 128})),
        ("Halftone", "core.halftone", json!({"size": 8, "angle": 45, "ink": "#000000", "paper": "#ffffff", "amount": 100})),
        ("Mosaic", "core.mosaic", json!({"size": 10})),
        ("Emboss", "core.emboss", json!({"direction": 135, "relief": 1, "contrast": 100, "mode": "grey"})),
        ("Find Edges", "core.find_edges", json!({"invert": "off", "amount": 100})),
        ("Sharpen", "core.sharpen", json!({"amount": 100, "radius": 1})),
        ("Diffusion", "core.diffusion", json!({"radius": 10, "amount": 50, "blend": "screen"})),
        ("Wave Warp", "core.wave_warp", json!({"shape": "sine", "height": 10, "width": 40, "direction": 90, "speed": 1, "phase": 0, "edges": "transparent"})),
        ("Ripple", "core.ripple", json!({"center": [50, 50], "amplitude": 5, "wavelength": 30, "speed": 20, "phase": 0, "fade": 0})),
        ("Twirl", "core.twirl", json!({"angle": 90, "radius": 50, "center": [50, 50]})),
        ("Bulge", "core.bulge", json!({"center": [50, 50], "radius": 50, "height": 1})),
        ("Mirror", "core.mirror", json!({"center": [50, 50], "angle": 0})),
        ("Linear Wipe", "core.linear_wipe", json!({"completion": 50, "angle": 90, "feather": 0})),
        ("Radial Wipe", "core.radial_wipe", json!({"completion": 50, "start_angle": 0, "center": [50, 50], "wipe": "clockwise", "feather": 0})),
        ("Venetian Blinds", "core.venetian_blinds", json!({"completion": 50, "angle": 0, "width": 20, "feather": 0})),
        ("Iris Wipe", "core.iris_wipe", json!({"completion": 50, "center": [50, 50], "feather": 0, "invert": "off"})),
        ("Simple Choker", "core.simple_choker", json!({"choke": -3})),
        ("Speed Lines", "core.speed_lines", json!({"center": [50, 50], "color": "#000000", "count": 120, "thickness": 1.5, "inner": 150, "inner_jitter": 40, "angle_jitter": 50, "seed": 0, "hold": 2, "opacity": 100})),
        ("Cross Glare", "core.cross_glare", json!({"threshold": 80, "length": 40, "points": 4, "angle": 45, "intensity": 1, "color": "#ffffff"})),
        ("Camera Shake", "core.camera_shake", json!({"amount": 10, "rotation": 0, "hold": 1, "seed": 0})),
        ("Rain", "core.rain", json!({"color": "#c8d8ff", "density": 30, "spacing": 24, "length": 20, "width": 1, "direction": 170, "speed": 30, "seed": 0, "opacity": 60})),
        ("Motion Tile", "core.motion_tile", json!({"output_width": 150, "output_height": 130, "mirror": "on"})),
        ("Color Lookup", "core.color_lookup", json!({"lut": "asset-lut"})),
        ("Line Blur", "core.line_blur", json!({"length": 12, "strength": 80, "lines_only": "off"})),
        ("HSV Key", "core.hsv_key", json!({"hue": 30, "hue_range": 40, "saturation": 50, "saturation_range": 50, "value": 60, "value_range": 40, "invert": "off"})),
        ("Paraffin", "core.paraffin", json!({"color": "#ffc890", "direction": 45, "spread": 60, "opacity": 70, "blend": "overlay"})),
        ("Kira-kira", "core.kira_kira", json!({"threshold": 60, "spacing": 24, "density": 80, "size": 12, "angle": 15, "twinkle": 50, "period": 24, "seed": 7, "opacity": 90, "shape": "star", "color": "#fff0c0"})),
        ("Median", "core.median", json!({"radius": 4, "operate_on_alpha": "on"})),
        ("Smart Blur", "core.smart_blur", json!({"radius": 4, "threshold": 40})),
        ("Roughen Edges", "core.roughen_edges", json!({"edge_type": "roughen_color", "edge_color": "#8a3c14", "border": 6, "size": 8, "complexity": 3, "evolution": 30, "speed": 10, "seed": 3})),
        ("Radial Shadow", "core.radial_shadow", json!({"color": "#203040", "opacity": 70, "light": [30, 10], "distance": 15, "softness": 8, "render": "glass_edge", "color_influence": 60, "shadow_only": "off"})),
        ("Bevel Alpha", "core.bevel_alpha", json!({"edge_thickness": 4, "light_angle": -45, "light_color": "#fff0d0", "light_intensity": 0.6})),
        ("Snowfall", "core.snowfall", json!({"color": "#ffffff", "density": 60, "spacing": 40, "size": 5, "depth": 40, "speed": 3, "wind": 0.5, "wiggle": 3, "period": 48, "seed": 5, "opacity": 90})),
        ("Cell Pattern", "core.cell_pattern", json!({"pattern": "crystals", "invert": "off", "contrast": 120, "disperse": 0.8, "size": 30, "evolution": 20, "seed": 2, "opacity": 60, "dark_color": "#102040", "light_color": "#f0e0c0", "blend": "screen"})),
        ("Polar Coordinates", "core.polar_coordinates", json!({"interpolation": 70, "conversion": "rect_to_polar"})),
        ("Optics Compensation", "core.optics_compensation", json!({"field_of_view": 60, "reverse": "off", "orientation": "diagonal", "center": [45, 55]})),
        ("Corner Pin", "core.corner_pin", json!({"upper_left": [5, 3], "upper_right": [92, 8], "lower_left": [0, 100], "lower_right": [110, 96]})),
    ]
}

/// The reference shot with `effect` on its first three layers, or as it is.
fn shot(effect: Option<(&str, &Value)>) -> Project {
    let text = fs::read_to_string(repo("verification/B-08a_project.json")).expect("read the reference shot");
    let mut j: Value = serde_json::from_str(&text).expect("the reference shot is JSON");
    if let Some((type_id, parameters)) = effect {
        for i in 0..3 {
            j["compositions"][0]["layers"][i]["effects"] = json!([{"instance_id": format!("b157-{i}"), "type_id": type_id, "enabled": true, "parameters": parameters}]);
        }
    }
    // Color Lookup's file, from its own fixtures.
    j["assets"].as_array_mut().expect("the reference shot's assets").push(json!({"id": "asset-lut", "kind": "lut", "name": "warm_17", "path": "../cube_lut/luts/warm_17.cube"}));
    persist::load_str(&j.to_string()).expect("the shot").document.project().clone()
}

fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    v[v.len() / 2]
}

/// What each effect the card draws costs it: the reference shot with the effect on three layers,
/// every eighth frame of 240, each asked for as the viewer asks, with the card told to forget
/// what it holds before each frame, so every frame sends its drawings and runs every pass again.
/// One loop fills the processor's caches, then three are timed; the median of their 90 frames,
/// in ms, less the same for the shot with no effect, is what the effect's passes cost.
#[test]
#[ignore = "B-157: a measurement, run deliberately with --release --ignored"]
fn b157_card_effect_timing() {
    let mut gpu = Gpu::new().expect("a usable card");
    let comp = Id::new("comp-reference-shot");
    let root = repo("Fixtures/reference_shot");
    let time = |gpu: &mut Gpu, project: &Project, quality: PreviewQuality| -> (f64, bool) {
        let mut cache = CelCache::viewer();
        let (mut times, mut on_cpu) = (Vec::new(), false);
        for pass in 0..4 {
            for frame in (0..240).step_by(8) {
                let mut log = FrameLog::new(3);
                gpu.forget();
                let t = std::time::Instant::now();
                drop(preview::preview_frame_srgb8(project, &comp, frame, &root, quality, DEFAULT_TILE_SIZE, &mut log, &mut cache, gpu).expect("GPU frame"));
                if pass > 0 {
                    times.push(t.elapsed().as_secs_f64() * 1000.0);
                }
                on_cpu |= log.finish().iter().any(|d| d.id == DiagnosticId::GpuPreviewOnCpu);
            }
        }
        (median(times), on_cpu)
    };
    let plain = shot(None);
    // Once untimed first: the card's clocks rise over the first seconds of work, which would
    // otherwise make the shot with no effect look the slowest.
    for q in [PreviewQuality::Full, PreviewQuality::Draft] {
        time(&mut gpu, &plain, q);
    }
    let floor = [PreviewQuality::Full, PreviewQuality::Draft].map(|q| time(&mut gpu, &plain, q).0);
    let mut s = format!(
        "- Card: {}\n- Processor: {}, {} threads\n- System: {}\n- Build: {}\n\n| Effect | Full ms | Full, less none | Draft ms | Draft, less none |\n|---|---:|---:|---:|---:|\n| None | {:.1} | 0.0 | {:.1} | 0.0 |\n",
        gpu.about(),
        std::env::var("PROCESSOR_IDENTIFIER").unwrap_or_else(|_| "not reported".into()),
        std::thread::available_parallelism().map_or(0, |n| n.get()),
        std::env::consts::OS,
        if cfg!(debug_assertions) { "debug" } else { "release" },
        floor[0],
        floor[1],
    );
    for (name, type_id, parameters) in effects() {
        // B157_ONLY=<name>,<name> times only those effects, for a before and after of their passes.
        if std::env::var("B157_ONLY").is_ok_and(|only| !only.split(',').any(|o| o == name)) {
            continue;
        }
        let project = shot(Some((type_id, &parameters)));
        let (full, cpu_full) = time(&mut gpu, &project, PreviewQuality::Full);
        let (draft, cpu_draft) = time(&mut gpu, &project, PreviewQuality::Draft);
        let mark = |cpu: bool| if cpu { " (processor)" } else { "" };
        let _ = writeln!(s, "| {name} | {full:.1}{} | {:.1} | {draft:.1}{} | {:.1} |", mark(cpu_full), full - floor[0], mark(cpu_draft), draft - floor[1]);
    }
    fs::write(repo("verification/B-157_timing_raw.md"), s).expect("write the timing table");
}
