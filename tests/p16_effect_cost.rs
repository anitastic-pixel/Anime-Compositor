//! P-16: what each effect costs on one 1920x1080 cel, and the fingerprint of what it drew.
//!
//! Ignored, because it is a timer: `cargo test --release --test p16_effect_cost -- --ignored
//! --nocapture` prints one table row per case. Nothing is asserted about time (document 12). The
//! SHA-256 of every result is printed beside its time, so a run before a change and a run after
//! it show whether the change moved a single bit of any effect's output, which is the claim
//! `verification/P-16_effect_cost.md` rests on.

use std::time::Instant;

use anime_compositor::color::srgb_to_linear;
use anime_compositor::effects::{apply_stack, Effect, EffectInstance};
use anime_compositor::model::Id;
use anime_compositor::WorkingBuffer;

const W: usize = 1920;
const H: usize = 1080;

/// Flat 8-bit colours as a drawing program paints them, premultiplied into working values.
fn paint(data: &mut [f32], i: usize, rgb: [u8; 3], a: f32) {
    for c in 0..3 {
        data[i + c] = srgb_to_linear(rgb[c] as f32 / 255.0) * a;
    }
    data[i + 3] = a;
}

/// A character cel: a figure in three flat colours with a dark line round it and a soft edge
/// on one side, on nothing. About a fifth of the frame shows.
fn character() -> WorkingBuffer {
    let mut b = WorkingBuffer::transparent(W, H);
    let d = b.data_mut();
    for y in 0..H {
        for x in 0..W {
            let (dx, dy) = (x as f32 - 960.0, y as f32 - 540.0);
            let r = (dx * dx + dy * dy).sqrt();
            let i = (y * W + x) * 4;
            if r < 380.0 {
                let rgb = match (x / 97 + y / 83) % 3 {
                    0 => [0xf6, 0xd6, 0xbe],
                    1 => [0x50, 0x23, 0x8c],
                    _ => [0xff, 0xe6, 0xfa],
                };
                paint(d, i, rgb, 1.0);
            } else if r < 390.0 {
                paint(d, i, [0x0f, 0x0c, 0x14], if dx > 0.0 { (390.0 - r) / 10.0 } else { 1.0 });
            }
        }
    }
    b
}

/// A background plate: opaque everywhere, a gradient sky over flat ground.
fn background() -> WorkingBuffer {
    let mut b = WorkingBuffer::transparent(W, H);
    let d = b.data_mut();
    for y in 0..H {
        for x in 0..W {
            let i = (y * W + x) * 4;
            let rgb = if y > 700 {
                [0x3c, 0xb4, 0x28]
            } else {
                [(40 + y / 5) as u8, (120 + y / 8) as u8, 0xe6 - (x / 40) as u8]
            };
            paint(d, i, rgb, 1.0);
        }
    }
    b
}

fn glow(radius: f64, based_on: &str) -> Effect {
    Effect::Glow {
        based_on: based_on.to_string(),
        threshold: 60.0,
        colors: vec!["#50238c".to_string()],
        tolerance: 0.0,
        radius,
        intensity: 1.0,
        operation: "add".to_string(),
        tint: String::new(),
    }
}

fn cases() -> Vec<(&'static str, Effect)> {
    vec![
        ("Exposure +1", Effect::Exposure { stops: 1.0 }),
        ("Tint 50%", Effect::Tint { color: [1.0, 0.5, 0.25], amount: 0.5 }),
        ("Gaussian Blur 4", Effect::GaussianBlur { sigma_px: 4.0, edges: "transparent".into() }),
        ("Gaussian Blur 10", Effect::GaussianBlur { sigma_px: 10.0, edges: "transparent".into() }),
        ("Gaussian Blur 40", Effect::GaussianBlur { sigma_px: 40.0, edges: "transparent".into() }),
        ("Line Smooth", Effect::LineSmooth { softness: 50.0, threshold: 16.0 }),
        (
            "Selective Colour Blur 12",
            Effect::SelectiveColorBlur {
                blur: 12.0,
                colors: vec!["#50238c".into(), "#f6d6be".into(), "#3cb428".into()],
                tolerance: 0.0,
            },
        ),
        (
            "Selective Colour Blur 100",
            Effect::SelectiveColorBlur {
                blur: 100.0,
                colors: vec!["#50238c".into(), "#f6d6be".into(), "#3cb428".into()],
                tolerance: 0.0,
            },
        ),
        ("Glow 10", glow(10.0, "bright")),
        ("Glow 50", glow(50.0, "bright")),
        ("Glow 250", glow(250.0, "bright")),
        ("Glow 50, chosen colour", glow(50.0, "colors")),
        // P-17: the batch of seven, D-91 to D-97.
        (
            "Line Recolour",
            Effect::LineRecolor {
                colors: vec!["#0f0c14".into()],
                tolerance: 0.0,
                new_color: "#ff0000".into(),
            },
        ),
        ("Directional Blur 10", Effect::DirectionalBlur { direction: 30.0, length: 10.0, edges: "transparent".into() }),
        ("Directional Blur 100", Effect::DirectionalBlur { direction: 30.0, length: 100.0, edges: "transparent".into() }),
        (
            "Select Colour",
            Effect::SelectColor {
                colors: vec!["#50238c".into()],
                tolerance: 0.0,
                keep: "chosen".into(),
            },
        ),
        ("Line Width 3", line_width(3.0, "shape")),
        ("Line Width 10", line_width(10.0, "shape")),
        ("Line Width -3", line_width(-3.0, "shape")),
        ("Line Width 3, chosen colour", line_width(3.0, "colors")),
        ("Radial Blur spin 10", radial("spin", 10.0)),
        ("Radial Blur zoom 20", radial("zoom", 20.0)),
        ("Bloom 20", bloom("none")),
        ("Bloom 20, star 60", bloom("star")),
        ("Colour Key rgb", color_key("rgb")),
        ("Colour Key hue", color_key("hue")),
        // P-20: Repeat Edge Pixels (D-109), which walks every pixel a transparent edge could skip.
        ("Gaussian Blur 10, edges repeat", Effect::GaussianBlur { sigma_px: 10.0, edges: "repeat".into() }),
        ("Directional Blur 100, edges repeat", Effect::DirectionalBlur { direction: 30.0, length: 100.0, edges: "repeat".into() }),
        (
            "Radial Blur zoom 20, edges repeat",
            Effect::RadialBlur { kind: "zoom".into(), amount: 20.0, center: [50.0, 50.0], edges: "repeat".into() },
        ),
        // The batch of ten, D-111 to D-120.
        (
            "Curves, all four",
            Effect::Curves {
                master: vec![vec![0.0, 0.0], vec![128.0, 180.0], vec![255.0, 255.0]],
                red: vec![vec![0.0, 0.0], vec![64.0, 40.0], vec![192.0, 215.0], vec![255.0, 255.0]],
                green: vec![vec![0.0, 0.0], vec![255.0, 128.0]],
                blue: vec![vec![0.0, 255.0], vec![255.0, 0.0]],
            },
        ),
        (
            "Levels, all five",
            Effect::Levels {
                input_black: 32.0,
                input_white: 224.0,
                gamma: 1.5,
                output_black: 16.0,
                output_white: 240.0,
            },
        ),
        (
            "Hue/Saturation, all three",
            Effect::HueSaturation {
                hue: 60.0,
                saturation: -50.0,
                lightness: 20.0,
            },
        ),
        (
            "Gradient, radial, screen",
            Effect::Gradient {
                shape: "radial".into(),
                start: [50.0, 50.0],
                end: [100.0, 50.0],
                start_color: "#ff8000".into(),
                end_color: "#6450a0".into(),
                start_opacity: 100.0,
                end_opacity: 20.0,
                blend: "screen".into(),
            },
        ),
        (
            "Drop Shadow, softness 6",
            Effect::DropShadow {
                color: "#000000".into(),
                opacity: 50.0,
                direction: 135.0,
                distance: 5.0,
                softness: 6.0,
            },
        ),
        (
            "Lens Blur, radius 10",
            Effect::LensBlur {
                radius: 10.0,
                edges: "transparent".into(),
                iris: "circle".into(),
                roundness: 0.0,
                rotation: 0.0,
                aspect: 1.0,
                highlight_gain: 0.0,
                highlight_threshold: 100.0,
            },
        ),
        // P-21: D-121's iris and a large radius, the cases that cost the most.
        (
            "Lens Blur, radius 40",
            Effect::LensBlur {
                radius: 40.0,
                edges: "transparent".into(),
                iris: "circle".into(),
                roundness: 0.0,
                rotation: 0.0,
                aspect: 1.0,
                highlight_gain: 0.0,
                highlight_threshold: 100.0,
            },
        ),
        (
            "Lens Blur 10, hexagon, highlights",
            Effect::LensBlur {
                radius: 10.0,
                edges: "transparent".into(),
                iris: "hexagon".into(),
                roundness: 20.0,
                rotation: 15.0,
                aspect: 1.5,
                highlight_gain: 3.0,
                highlight_threshold: 80.0,
            },
        ),
        (
            "Rim Light, the defaults",
            Effect::RimLight {
                color: "#ffffff".into(),
                direction: 45.0,
                width: 3.0,
                softness: 1.0,
                intensity: 100.0,
                blend: "normal".into(),
            },
        ),
        (
            "Outline, width 3, softness 2",
            Effect::Outline {
                color: "#ffffff".into(),
                width: 3.0,
                softness: 2.0,
                opacity: 100.0,
            },
        ),
        (
            "Noise, amount 10, colour",
            Effect::Noise {
                amount: 10.0,
                mode: "color".into(),
                seed: 0.0,
                animate: "on".into(),
                frame: 0,
            },
        ),
        (
            "Chromatic Aberration, amount 3",
            Effect::ChromaticAberration {
                amount: 3.0,
                center: [50.0, 50.0],
            },
        ),
        (
            "Distance Gradation, width 10",
            Effect::DistanceGradation {
                color: "#6450a0".to_string(),
                width: 10.0,
                opacity: 50.0,
                invert: "off".to_string(),
                blend: "multiply".to_string(),
            },
        ),
        (
            "Light Rays, length 50",
            Effect::LightRays {
                center: [50.0, 50.0],
                length: 50.0,
                threshold: 70.0,
                intensity: 1.0,
                color: "#ffffff".to_string(),
            },
        ),
        (
            "Exposure Flicker, amount 1, frame 5",
            Effect::ExposureFlicker {
                amount: 1.0,
                hold: 1.0,
                seed: 0.0,
                frame: 5,
            },
        ),
        (
            "Vignette, amount 50, roundness 40",
            Effect::Vignette {
                amount: 50.0,
                color: "#000000".to_string(),
                size: 100.0,
                roundness: 40.0,
                softness: 50.0,
                center: [50.0, 50.0],
            },
        ),
        (
            "Turbulent Displace, amount 10, complexity 2",
            Effect::TurbulentDisplace {
                amount: 10.0,
                size: 60.0,
                complexity: 2.0,
                evolution: 0.0,
                speed: 20.0,
                seed: 0.0,
                edges: "transparent".to_string(),
                frame: 5,
            },
        ),
        (
            "Fractal Noise, size 100, complexity 4",
            Effect::FractalNoise {
                size: 100.0,
                complexity: 4.0,
                contrast: 100.0,
                brightness: 0.0,
                evolution: 0.0,
                speed: 20.0,
                seed: 0.0,
                dark_color: "#000000".to_string(),
                light_color: "#ffffff".to_string(),
                opacity: 100.0,
                blend: "normal".to_string(),
                frame: 5,
            },
        ),
        (
            "Gradient Map, sunset",
            Effect::GradientMap {
                shadow_color: "#2a1650".to_string(),
                midtone_color: "#c85a50".to_string(),
                highlight_color: "#ffe6b4".to_string(),
                midpoint: 50.0,
                amount: 100.0,
            },
        ),
        (
            "Color Balance, cool shadows and warm lights",
            Effect::ColorBalance {
                shadows: vec![0.0, 0.0, 40.0],
                midtones: vec![-10.0, 5.0, 0.0],
                highlights: vec![30.0, 10.0, -20.0],
                preserve_luminosity: "off".to_string(),
            },
        ),
        (
            "Offset, a part-pixel slide",
            Effect::Offset { shift: [40.5, -12.25] },
        ),
        (
            "Invert, a negative",
            Effect::Invert {
                channel: "rgb".to_string(),
                amount: 100.0,
            },
        ),
        (
            "Brightness & Contrast, both moved",
            Effect::BrightnessContrast {
                brightness: 30.0,
                contrast: 40.0,
            },
        ),
        (
            "Black & White, a red filter",
            Effect::BlackWhite {
                reds: 120.0,
                yellows: 110.0,
                greens: -10.0,
                cyans: -50.0,
                blues: -50.0,
                magentas: 120.0,
            },
        ),
        (
            "Posterize, six levels",
            Effect::Posterize { levels: 6.0 },
        ),
        (
            "Threshold, at the middle",
            Effect::Threshold { level: 128.0 },
        ),
        (
            "Channel Mixer, red and blue swapped",
            Effect::ChannelMixer {
                red: vec![0.0, 0.0, 100.0, 0.0],
                green: vec![0.0, 100.0, 0.0, 0.0],
                blue: vec![100.0, 0.0, 0.0, 0.0],
                monochrome: "off".to_string(),
            },
        ),
        (
            "Vibrance, an everyday grade",
            Effect::Vibrance {
                vibrance: 40.0,
                saturation: 20.0,
            },
        ),
        (
            "Leave Color, the reds kept",
            Effect::LeaveColor {
                color: "#ff0000".to_string(),
                tolerance: 15.0,
                softness: 10.0,
                amount: 100.0,
            },
        ),
        (
            "Solarize, at the middle",
            Effect::Solarize { threshold: 128.0 },
        ),
        (
            "Halftone, as it starts",
            Effect::Halftone {
                size: 8.0,
                angle: 45.0,
                ink: "#000000".to_string(),
                paper: "#ffffff".to_string(),
                amount: 100.0,
            },
        ),
        (
            "Mosaic, as it starts",
            Effect::Mosaic { size: 10.0 },
        ),
        (
            "Emboss, as it starts",
            Effect::Emboss {
                direction: 135.0,
                relief: 1.0,
                contrast: 100.0,
                mode: "grey".to_string(),
            },
        ),
        (
            "Find Edges, as it starts",
            Effect::FindEdges {
                invert: "off".to_string(),
                amount: 100.0,
            },
        ),
        (
            "Sharpen, as it starts",
            Effect::Sharpen {
                amount: 100.0,
                radius: 1.0,
            },
        ),
        (
            "Diffusion, as it starts",
            Effect::Diffusion {
                radius: 10.0,
                amount: 50.0,
                blend: "screen".to_string(),
            },
        ),
        (
            "Wave Warp, as it starts",
            Effect::WaveWarp {
                shape: "sine".to_string(),
                height: 10.0,
                width: 40.0,
                direction: 90.0,
                speed: 0.0,
                phase: 0.0,
                edges: "transparent".to_string(),
                frame: 0,
            },
        ),
        (
            "Ripple, as it starts",
            Effect::Ripple {
                center: [50.0, 50.0],
                amplitude: 5.0,
                wavelength: 30.0,
                speed: 20.0,
                phase: 0.0,
                fade: 0.0,
                frame: 0,
            },
        ),
        (
            "Twirl, as it starts",
            Effect::Twirl {
                angle: 90.0,
                radius: 50.0,
                center: [50.0, 50.0],
            },
        ),
        (
            "Bulge, as it starts",
            Effect::Bulge {
                center: [50.0, 50.0],
                radius: 50.0,
                height: 1.0,
            },
        ),
        (
            "Mirror, as it starts",
            Effect::Mirror {
                center: [50.0, 50.0],
                angle: 0.0,
            },
        ),
        (
            "Motion Tile, as it starts",
            Effect::MotionTile {
                output_width: 100.0,
                output_height: 100.0,
                mirror: "off".to_string(),
            },
        ),
        (
            "Linear Wipe, half way",
            Effect::LinearWipe {
                completion: 50.0,
                angle: 90.0,
                feather: 0.0,
            },
        ),
        (
            "Radial Wipe, half way",
            Effect::RadialWipe {
                completion: 50.0,
                start_angle: 0.0,
                center: [50.0, 50.0],
                wipe: "clockwise".to_string(),
                feather: 0.0,
            },
        ),
        (
            "Venetian Blinds, half way",
            Effect::VenetianBlinds {
                completion: 50.0,
                angle: 0.0,
                width: 20.0,
                feather: 0.0,
            },
        ),
        (
            "Iris Wipe, half way",
            Effect::IrisWipe {
                completion: 50.0,
                center: [50.0, 50.0],
                feather: 0.0,
                invert: "off".to_string(),
            },
        ),
        (
            "Simple Choker, spread 3",
            Effect::SimpleChoker { choke: -3.0 },
        ),
        (
            "Speed Lines, as they start",
            Effect::SpeedLines {
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
            },
        ),
        (
            "Cross Glare, as it starts",
            Effect::CrossGlare {
                threshold: 80.0,
                length: 40.0,
                points: 4.0,
                angle: 45.0,
                intensity: 1.0,
                color: "#ffffff".to_string(),
            },
        ),
        (
            "Camera Shake, as it starts",
            Effect::CameraShake {
                amount: 10.0,
                rotation: 0.0,
                hold: 1.0,
                seed: 0.0,
                frame: 0,
            },
        ),
        (
            "Rain, as it starts",
            Effect::Rain {
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
            },
        ),
        // P-23: the fourth batch, B-115 to B-149, each as the window adds it unless that shows
        // nothing, when a setting is moved as named. The effects that read another layer or other
        // frames are in `p23_layer_cost` below.
        ("Color Lookup, cool_3.cube", Effect::ColorLookup { lut: "p23".into(), table: Some(cool_lut()) }),
        ("Line Blur, as it starts", Effect::LineBlur { length: 4.0, strength: 100.0, lines_only: "off".into() }),
        (
            "HSV Key, as it starts",
            Effect::HsvKey {
                hue: 120.0,
                saturation: 60.0,
                value: 60.0,
                hue_range: 40.0,
                saturation_range: 40.0,
                value_range: 40.0,
                invert: "off".into(),
            },
        ),
        (
            "Paraffin, as it starts",
            Effect::Paraffin {
                color: "#6450a0".into(),
                direction: 180.0,
                spread: 70.0,
                opacity: 50.0,
                blend: "multiply".into(),
            },
        ),
        (
            "Kira-kira, as it starts",
            Effect::KiraKira {
                threshold: 95.0,
                spacing: 64.0,
                density: 60.0,
                size: 40.0,
                shape: "star".into(),
                angle: 0.0,
                twinkle: 100.0,
                period: 24.0,
                seed: 0.0,
                opacity: 100.0,
                color: "#ffffff".into(),
                frame: 0,
            },
        ),
        (
            "Lightning Bolt, as it starts",
            Effect::LightningBolt {
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
                color: "#ffffff".into(),
                glow_color: "#6e8cff".into(),
                frame: 0,
            },
        ),
        (
            "Change to Color, as it starts",
            Effect::ChangeToColor {
                from: "#ff0000".into(),
                to: "#0080ff".into(),
                change: "hue".into(),
                change_by: "setting".into(),
                hue_tolerance: 5.0,
                lightness_tolerance: 50.0,
                saturation_tolerance: 50.0,
                softness: 50.0,
                view_matte: "off".into(),
            },
        ),
        (
            "Corner Pin, one corner pulled in",
            Effect::CornerPin {
                upper_left: [0.0, 0.0],
                upper_right: [80.0, 10.0],
                lower_left: [0.0, 100.0],
                lower_right: [100.0, 100.0],
            },
        ),
        (
            "Light Sweep, as it starts",
            Effect::LightSweep {
                center: [50.0, 50.0],
                direction: -30.0,
                shape: "smooth".into(),
                width: 50.0,
                sweep_intensity: 50.0,
                edge_intensity: 100.0,
                edge_thickness: 1.0,
                light_color: "#ffffff".into(),
                light_reception: "add".into(),
            },
        ),
        (
            "Radio Waves, at frame 48",
            Effect::RadioWaves {
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
                profile: "square".into(),
                color: "#ffffff".into(),
                frame: 48,
            },
        ),
        (
            "Polar Coordinates, as it starts",
            Effect::PolarCoordinates { interpolation: 100.0, conversion: "rect_to_polar".into() },
        ),
        ("Median, radius 2, as it starts", Effect::Median { radius: 2.0, operate_on_alpha: "off".into() }),
        ("Median, radius 10", Effect::Median { radius: 10.0, operate_on_alpha: "off".into() }),
        ("Smart Blur, radius 3, as it starts", Effect::SmartBlur { radius: 3.0, threshold: 64.0 }),
        ("Smart Blur, radius 10", Effect::SmartBlur { radius: 10.0, threshold: 64.0 }),
        (
            "Snowfall, as it starts",
            Effect::Snowfall {
                color: "#ffffff".into(),
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
            },
        ),
        (
            "Kaleidoscope, as it starts",
            Effect::Kaleidoscope {
                segments: 6.0,
                rotation: 0.0,
                size: 100.0,
                center: [50.0, 50.0],
                mode: "mirror".into(),
            },
        ),
        (
            "Roughen Edges, as it starts",
            Effect::RoughenEdges {
                edge_type: "roughen".into(),
                edge_color: "#8a3c14".into(),
                border: 8.0,
                size: 10.0,
                complexity: 3.0,
                evolution: 0.0,
                speed: 0.0,
                seed: 0.0,
                frame: 0,
            },
        ),
        (
            "Beam, as it starts",
            Effect::Beam {
                start: [10.0, 50.0],
                end: [90.0, 50.0],
                length: 25.0,
                time: 0.0,
                start_thickness: 8.0,
                end_thickness: 8.0,
                softness: 50.0,
                inside_color: "#ffffff".into(),
                outside_color: "#3c8cff".into(),
                composite: "on".into(),
            },
        ),
        (
            "4-Color Gradient, as it starts",
            Effect::FourColorGradient {
                point_1: [10.0, 10.0],
                point_2: [90.0, 10.0],
                point_3: [10.0, 90.0],
                point_4: [90.0, 90.0],
                color_1: "#ffff00".into(),
                color_2: "#00ff00".into(),
                color_3: "#ff00ff".into(),
                color_4: "#0000ff".into(),
                blend: 100.0,
                opacity: 100.0,
                blending_mode: "normal".into(),
            },
        ),
        (
            "Cell Pattern, as it starts",
            Effect::CellPattern {
                pattern: "bubbles".into(),
                invert: "off".into(),
                contrast: 100.0,
                disperse: 1.0,
                size: 60.0,
                evolution: 0.0,
                seed: 0.0,
                dark_color: "#000000".into(),
                light_color: "#ffffff".into(),
                opacity: 100.0,
                blend: "normal".into(),
            },
        ),
        (
            "Optics Compensation, field of view 60",
            Effect::OpticsCompensation {
                field_of_view: 60.0,
                reverse: "off".into(),
                orientation: "horizontal".into(),
                center: [50.0, 50.0],
            },
        ),
        ("Radial Shadow, as it starts", radial_shadow(0.0)),
        ("Radial Shadow, softness 20", radial_shadow(20.0)),
        (
            "Extract, black point 64, softness 32",
            Effect::Extract {
                channel: "luminance".into(),
                black_point: 64.0,
                white_point: 255.0,
                black_softness: 32.0,
                white_softness: 0.0,
                invert: "off".into(),
            },
        ),
        (
            "Bevel Alpha, as it starts",
            Effect::BevelAlpha { edge_thickness: 2.0, light_angle: -60.0, light_color: "#ffffff".into(), light_intensity: 0.4 },
        ),
        (
            "Bevel Edges, as it starts",
            Effect::BevelEdges { edge_thickness: 0.1, light_angle: -60.0, light_color: "#ffffff".into(), light_intensity: 0.4 },
        ),
        (
            "Block Dissolve, half way, 8-pixel blocks",
            Effect::BlockDissolve { completion: 50.0, block_width: 8.0, block_height: 8.0, feather: 4.0 },
        ),
        // D-202: Mix, the given picture laid back under the result.
        ("Gaussian Blur 10 at Mix 50%", Effect::GaussianBlur { sigma_px: 10.0, edges: "transparent".into() }),
    ]
}

fn cool_lut() -> anime_compositor::lut::Table {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("Fixtures/cube_lut/luts/cool_3.cube");
    let cube = anime_compositor::lut::parse(&std::fs::read(path).unwrap()).unwrap();
    anime_compositor::lut::Table(std::sync::Arc::new(cube))
}

fn radial_shadow(softness: f64) -> Effect {
    Effect::RadialShadow {
        color: "#000000".into(),
        opacity: 50.0,
        light: [50.0, 0.0],
        distance: 10.0,
        softness,
        render: "regular".into(),
        color_influence: 100.0,
        shadow_only: "off".into(),
    }
}

fn line_width(width: f64, based_on: &str) -> Effect {
    Effect::LineWidth {
        width,
        based_on: based_on.into(),
        colors: vec!["#0f0c14".into()],
        tolerance: 0.0,
    }
}

fn radial(kind: &str, amount: f64) -> Effect {
    Effect::RadialBlur { kind: kind.into(), amount, center: [50.0, 50.0], edges: "transparent".into() }
}

fn bloom(streaks: &str) -> Effect {
    Effect::Bloom {
        threshold: 80.0,
        radius: 20.0,
        intensity: 1.0,
        streaks: streaks.into(),
        length: 60.0,
        angle: 0.0,
    }
}

fn color_key(match_by: &str) -> Effect {
    Effect::ColorKey {
        colors: vec!["#3cb428".into()],
        tolerance: 20.0,
        softness: 20.0,
        match_by: match_by.into(),
    }
}

fn fingerprint(b: &WorkingBuffer) -> String {
    let bytes: Vec<u8> = b.data().iter().flat_map(|v| v.to_bits().to_le_bytes()).collect();
    anime_compositor::sha256::hex(&bytes)[..16].to_string()
}

#[test]
#[ignore]
fn p16_effect_cost() {
    println!("| Effect | Cel | Median ms | Fastest ms | Runs | Result SHA-256 (first 16) |");
    println!("|---|---|---|---|---|---|");
    // `P16_ONLY=Median` runs only the cases whose name holds that text.
    let only = std::env::var("P16_ONLY").unwrap_or_default();
    for (cel_name, cel) in [("character", character()), ("background", background())] {
        for (name, effect) in cases().into_iter().filter(|(n, _)| n.contains(only.as_str())) {
            let mut instance = EffectInstance::new(Id::new("p16"), effect);
            if name.ends_with("at Mix 50%") {
                instance.mix = 50.0;
            }
            let stack = [instance];
            let mut times = Vec::new();
            let mut print = String::new();
            let started = Instant::now();
            while times.len() < 7 && (times.len() < 2 || started.elapsed().as_secs() < 20) {
                let mut b = cel.clone();
                let t = Instant::now();
                apply_stack(&mut b, &stack, |_, _, why| panic!("{name} bypassed: {why:?}"));
                times.push(t.elapsed().as_secs_f64() * 1000.0);
                print = fingerprint(&b);
            }
            let mut sorted = times.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            println!(
                "| {name} | {cel_name} | {:.1} | {:.1} | {} | `{print}` |",
                sorted[sorted.len() / 2],
                sorted[0],
                sorted.len()
            );
        }
    }
}

/// P-23: the fourth batch's effects that read another layer or other frames, and D-188's motion
/// blur and D-216's time stretch, on the reference shot at frame 101, drawn as export draws it
/// (`render_frame`, Full), so the drawings are read in each run as well; the first row, with
/// nothing added, is that part. At frame 101 layer 3 (on twos) and layer 4 (on threes) are each
/// on the last frame of a drawing, where the drawing dissolve shows.
#[test]
#[ignore]
fn p23_layer_cost() {
    use anime_compositor::compose::{render_frame, DEFAULT_TILE_SIZE};
    use anime_compositor::diagnostics::FrameLog;
    use anime_compositor::persist;
    use serde_json::{json, Value as J};

    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let base: J =
        serde_json::from_str(&std::fs::read_to_string(repo.join("verification/B-08a_project.json")).unwrap()).unwrap();
    let root = repo.join("Fixtures/reference_shot");
    let on = |p: &mut J, layer: usize, type_id: &str, parameters: J| {
        p["compositions"][0]["layers"][layer]["effects"] = json!([{
            "instance_id": "fx-p23", "type_id": type_id, "enabled": true, "parameters": parameters,
        }]);
    };
    let slide = |p: &mut J| {
        for layer in p["compositions"][0]["layers"].as_array_mut().unwrap() {
            if layer["id"] == "layer-3" || layer["id"] == "layer-4" {
                let at = layer["transform"]["position"]["base"].clone();
                let (x, y) = (at[0].as_f64().unwrap(), at[1].as_f64().unwrap());
                layer["transform"]["position"]["keyframes"] = json!([
                    {"frame": 90, "value": [x - 200.0, y], "interp": "linear"},
                    {"frame": 110, "value": [x + 200.0, y], "interp": "linear"}
                ]);
                layer["motion_blur"] = J::Bool(true);
            }
        }
    };
    let stretch = |p: &mut J, blend: bool| {
        let layer = &mut p["compositions"][0]["layers"][1];
        layer["time_stretch"] = J::from(150.0);
        if blend {
            layer["frame_blend"] = J::from("frame_mix");
            p["compositions"][0]["frame_blending"] = J::Bool(true);
        }
    };
    let cases: Vec<(&str, Box<dyn Fn(&mut J)>)> = vec![
        ("the reference shot, nothing added", Box::new(|_| {})),
        (
            "Compound Blur 20 on layer 1, reading layer 2",
            Box::new(move |p| on(p, 0, "core.compound_blur", json!({"layer": "layer-2", "fit": "stretch", "max_blur": 20, "invert": "off", "edges": "transparent"}))),
        ),
        (
            "Displacement Map as it starts on layer 1, reading layer 2",
            Box::new(move |p| on(p, 0, "core.displacement_map", json!({"layer": "layer-2", "fit": "stretch", "horizontal": "red", "max_horizontal": 5, "vertical": "green", "max_vertical": 5, "wrap": "off"}))),
        ),
        (
            "Gradient Wipe half way, softness 10, on layer 1, reading layer 2",
            Box::new(move |p| on(p, 0, "core.gradient_wipe", json!({"layer": "layer-2", "fit": "stretch", "completion": 50, "softness": 10, "invert": "off"}))),
        ),
        (
            "Echo on layer 3, 4 echoes 2 frames apart",
            Box::new(move |p| on(p, 2, "core.echo", json!({"echo_time": -2, "echoes": 4, "intensity": 1, "decay": 0.7, "operator": "add"}))),
        ),
        ("Posterize Time 12 on layer 2", Box::new(move |p| on(p, 1, "core.posterize_time", json!({"frame_rate": 12})))),
        ("layers 3 and 4 sliding, no motion blur", Box::new(move |p| slide(p))),
        (
            "layers 3 and 4 sliding, motion blur 180 degrees, 16 samples",
            Box::new(move |p| {
                slide(p);
                p["compositions"][0]["motion_blur"] =
                    json!({"enabled": true, "shutter_angle": 180, "shutter_phase": -90, "samples": 16});
            }),
        ),
        ("layer 2 at time stretch 150%, no blending", Box::new(move |p| stretch(p, false))),
        ("layer 2 at time stretch 150%, Frame Mix", Box::new(move |p| stretch(p, true))),
        (
            "drawing dissolve of 2 on layers 3 and 4",
            Box::new(|p| {
                for layer in [2, 3] {
                    p["compositions"][0]["layers"][layer]["drawing_dissolve"] = J::from(2);
                }
            }),
        ),
    ];
    println!("| Case | Median ms | Fastest ms | Runs | Frame SHA-256 (first 16) |");
    println!("|---|---|---|---|---|");
    let only = std::env::var("P16_ONLY").unwrap_or_default();
    for (name, change) in cases.iter().filter(|(n, _)| n.contains(only.as_str())) {
        let mut p = base.clone();
        change(&mut p);
        let loaded = persist::load_str(&p.to_string()).unwrap();
        let mut times = Vec::new();
        let mut print = String::new();
        while times.len() < 7 {
            let mut log = FrameLog::new(3);
            let t = Instant::now();
            let frame = render_frame(loaded.document.project(), &Id::new("comp-reference-shot"), 101, &root, DEFAULT_TILE_SIZE, &mut log)
                .unwrap();
            times.push(t.elapsed().as_secs_f64() * 1000.0);
            let said = log.finish();
            assert!(said.is_empty(), "{name}: {said:?}");
            print = fingerprint(&frame);
        }
        times.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("| {name} | {:.1} | {:.1} | {} | `{print}` |", times[3], times[0], times.len());
    }
}
