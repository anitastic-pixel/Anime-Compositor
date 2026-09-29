//! Step 3 of document 21: the ordered effect stack, evaluated in layer space.
//!
//! Three effects are in G1, and document 21 states each one as arithmetic rather than as a
//! description: exposure multiplies premultiplied RGB by `2^e` and leaves alpha alone; tint
//! recovers straight RGB, mixes toward a colour, and premultiplies back; Gaussian blur filters
//! premultiplied RGB and alpha together with separable normalised weights and a radius of
//! `ceil(3*sigma)`. Each is written here in that form, and `tests/b07_effects.rs` checks it
//! against weights generated a second way.
//!
//! Effects run after the mask (step 2) and before the transform (step 4), on the layer's own
//! pixels. That ordering is what makes the blur radius a number in source pixels: a layer scaled
//! to half size blurs by the same source-pixel sigma and the result is scaled with everything
//! else, which is what document 09 means by "specify radius units in source pixels".
//!
//! **Bounds.** Document 21 line 89 requires every effect to declare its input bounds expansion.
//! Exposure and tint declare zero -- they read one pixel and write it. Blur declares its kernel
//! radius, and this module honours that literally: the buffer grows by the radius on all four
//! sides, so light that leaves the edge of the cel is kept rather than cropped. The caller is
//! handed the offset and shifts the layer transform by it, which leaves every unblurred pixel
//! exactly where it was.
//!
//! **Tiling.** ADR-011 warns that a neighbourhood operation cannot be tiled without a declared
//! margin. Nothing here is tiled: the stack runs once over the whole layer buffer while the
//! frame plan is being built, and the renderer's tiles are cut from the composition afterwards,
//! by which time the blur is already baked into the layer's pixels. So there is no margin to
//! get wrong, and no ROI optimisation to prove equal to full-frame math. What there is to prove
//! is that the frame still does not depend on tile size once a blur is in it, and the fixture
//! renders the same blurred frame at six tile sizes to say so.

use rayon::prelude::*;

use std::collections::BTreeMap;

use crate::model::{Id, Interp, Keyframe, Property, Value};
use crate::WorkingBuffer;

/// One entry in a layer's ordered effect stack.
///
/// Document 19: "EffectInstance stores stable instance ID, effect type ID, enabled flag and a
/// typed parameter map."
#[derive(Clone, PartialEq, Debug)]
pub struct EffectInstance {
    pub instance_id: Id,
    pub enabled: bool,
    /// Every setting's constant value, which is what a setting with no keys is.
    pub effect: Effect,
    /// D-68: the keys of each setting that has any, by the setting's name in the file. One
    /// property of one number for a number, three for a colour, which share their frames and
    /// eases. A property's base is not read; the constant lives in `effect`.
    pub tracks: BTreeMap<String, Vec<Property>>,
}

impl EffectInstance {
    pub fn new(instance_id: Id, effect: Effect) -> Self {
        EffectInstance {
            instance_id,
            enabled: true,
            effect,
            tracks: BTreeMap::new(),
        }
    }

    /// D-68: every key of the setting `name`, as a command would give them back.
    pub fn keys(&self, name: &str) -> Vec<EffectKey> {
        let Some(track) = self.tracks.get(name) else {
            return Vec::new();
        };
        (0..track[0].keyframes().len())
            .map(|i| EffectKey {
                frame: track[0].keyframes()[i].frame,
                value: key_numbers(track, i),
                interp: track[0].keyframes()[i].interp,
            })
            .collect()
    }

    /// Replace every key of the setting `name`. None is a setting that is constant again. The
    /// command has checked the keys; a name this effect does not have changes nothing.
    pub(crate) fn set_keys(&mut self, name: &str, keys: &[EffectKey]) {
        let Some(count) = self.effect.arity(name) else {
            return;
        };
        if keys.is_empty() {
            self.tracks.remove(name);
            return;
        }
        let mut track = vec![Property::constant(Value::Scalar(0.0)); count];
        for key in keys {
            for (c, property) in track.iter_mut().enumerate() {
                property.set_keyframe(Keyframe {
                    frame: key.frame,
                    value: Value::Scalar(key.value[c]),
                    interp: key.interp,
                    spatial: None,
                    kind: Default::default(),
                    roving: false,
                });
            }
        }
        self.tracks.insert(name.to_string(), track);
    }

    /// Every key of every setting `by` frames later, as a layer's other keys go with it.
    pub(crate) fn shift_keys(&mut self, by: i32) {
        for property in self.tracks.values_mut().flatten() {
            property.shift_keyframes(by);
        }
    }

    /// D-68 and D-46: the effect with the first constant or key value that is outside its
    /// range, when there is one. The whole effect is then bypassed on every frame.
    pub fn invalid(&self) -> Option<Effect> {
        if !self.effect.is_valid() {
            return Some(self.effect.clone());
        }
        for (name, track) in &self.tracks {
            for i in 0..track[0].keyframes().len() {
                let mut e = self.effect.clone();
                e.set(name, &key_numbers(track, i));
                if !e.is_valid() {
                    return Some(e);
                }
            }
        }
        None
    }

    /// D-68: this instance as it is at a composition frame, with no keys left in it. Each keyed
    /// setting is document 20's value, then held inside the setting's range, because an ease
    /// between two keys that are in range may overshoot it.
    pub fn at(&self, frame: i32) -> EffectInstance {
        let mut effect = self.effect.clone();
        if let Some(bad) = self.invalid() {
            effect = bad;
        } else {
            for (name, track) in &self.tracks {
                let v: Vec<f64> = track
                    .iter()
                    .map(|p| p.value_at(frame).as_scalar().unwrap_or(0.0))
                    .collect();
                effect.set(name, &v);
            }
            for (_, slots, low, high) in effect.numbers() {
                for v in slots {
                    *v = v.clamp(low, high);
                }
            }
            // D-119: the grain's frame, 0 when it does not move.
            if let Effect::Noise { animate, frame: f, .. } = &mut effect {
                *f = if animate == "on" { frame } else { 0 };
            }
            // D-125, D-127 and D-128: the flicker's, the wobble's and the clouds' frame.
            if let Effect::ExposureFlicker { frame: f, .. } = &mut effect {
                *f = frame;
            }
            if let Effect::TurbulentDisplace { frame: f, .. } = &mut effect {
                *f = frame;
            }
            if let Effect::FractalNoise { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-150: the rings' frame.
            if let Effect::Ripple { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-149: the wave's frame.
            if let Effect::WaveWarp { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-160: the lines' frame.
            if let Effect::SpeedLines { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-162: the jolt's frame.
            if let Effect::CameraShake { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-163: the rain's frame.
            if let Effect::Rain { frame: f, .. } = &mut effect {
                *f = frame;
            }
        }
        EffectInstance {
            instance_id: self.instance_id.clone(),
            enabled: self.enabled,
            effect,
            tracks: BTreeMap::new(),
        }
    }

    pub fn type_id(&self) -> &str {
        self.effect.type_id()
    }
}

/// The typed parameter map of document 19, made a type per effect rather than a map.
///
/// A map would have to be validated at every read; document 19 requires that "effect parameter
/// types match the registered effect schema", and the cheapest way to guarantee that is to make
/// the wrong shape unrepresentable once the record has been read. Validation therefore happens
/// exactly once, where the file is parsed.
///
/// `Unsupported` is the exception and is the point of the enum being open at all. Document 19:
/// "Unknown effect records must survive project load/save where feasible but render as
/// unsupported with an explicit warning; they may not be silently discarded."
#[derive(Clone, PartialEq, Debug)]
pub enum Effect {
    /// Document 21: "parameter is stops `e`; linear premultiplied RGB is multiplied by `2^e`;
    /// alpha is unchanged."
    Exposure { stops: f64 },
    /// Document 21: "parameter `sigma_px >= 0` ... kernel radius `ceil(3*sigma_px)`."
    /// D-109: `edges`, "transparent" or "repeat", as Directional and Radial Blur have.
    GaussianBlur { sigma_px: f64, edges: String },
    /// Document 21: "parameter color is linear RGB and amount `t` in 0..1."
    Tint { color: [f64; 3], amount: f64 },
    /// D-86: "`softness`, 0 to 100 ... and `threshold`, 0 to 255". Document 21's line smoothing.
    LineSmooth { softness: f64, threshold: f64 },
    /// D-87: "`blur`, 0 to 200 pixels ... `colors`, the chosen colours, up to eight, each
    /// written `"#rrggbb"`". Document 21's selective colour blur. D-88: "`tolerance`, 0 to
    /// 255, starting at 0, which is D-87 exactly".
    SelectiveColorBlur {
        blur: f64,
        colors: Vec<String>,
        tolerance: f64,
    },
    /// D-89: "`based_on`, Glow Based On, either Bright parts (`"bright"`) or Chosen colours
    /// (`"colors"`)", `threshold` 0 to 100, `colors` and `tolerance` as D-88's, `radius` 0 to
    /// 500, `intensity` 0 to 10, `operation` `"add"` or `"screen"`, and `tint`, empty or one
    /// colour. The words are kept as written, so a file's wrong one is kept and reported.
    Glow {
        based_on: String,
        threshold: f64,
        colors: Vec<String>,
        tolerance: f64,
        radius: f64,
        intensity: f64,
        operation: String,
        tint: String,
    },
    /// D-91: `colors` and `tolerance` as D-88's, and `new_color`, the colour `#rrggbb` the chosen
    /// pixels become.
    LineRecolor {
        colors: Vec<String>,
        tolerance: f64,
        new_color: String,
    },
    /// D-92: `direction`, degrees clockwise from up, -3600 to 3600, and `length`, the whole
    /// streak in pixels, 0 to 500. D-109: `edges`, "transparent" or "repeat".
    DirectionalBlur { direction: f64, length: f64, edges: String },
    /// D-93: `colors` and `tolerance` as D-88's, and `keep`, "chosen" or "others": the pixels
    /// kept, every other one made transparent.
    SelectColor {
        colors: Vec<String>,
        tolerance: f64,
        keep: String,
    },
    /// D-94: `width`, -20 to 20 pixels, thicker or thinner; `based_on`, "shape" or "colors";
    /// and `colors` and `tolerance` as D-88's, used when it is based on colours.
    LineWidth {
        width: f64,
        based_on: String,
        colors: Vec<String>,
        tolerance: f64,
    },
    /// D-95: `kind`, the file's `type`, "spin" or "zoom"; `amount`, 0 to 100, degrees of arc
    /// or per cent of the distance; and `center`, per cent of the drawing's width and height,
    /// each -1000 to 1000. D-109: `edges`, "transparent" or "repeat".
    RadialBlur {
        kind: String,
        amount: f64,
        center: [f64; 2],
        edges: String,
    },
    /// D-96: `threshold`, 0 to 100, glow's bright test; `radius`, 0 to 500 pixels; `intensity`,
    /// 0 to 10; `streaks`, "none", "cross" or "star"; `length`, 0 to 500 pixels each way; and
    /// `angle`, -3600 to 3600 degrees clockwise from up.
    Bloom {
        threshold: f64,
        radius: f64,
        intensity: f64,
        streaks: String,
        length: f64,
        angle: f64,
    },
    /// D-97: `colors` as D-87's; `tolerance` and `softness`, 0 to 255; and `match_by`, "rgb" or
    /// "hue", written `match` in the file: the pixels near a chosen colour made transparent,
    /// wholly within the tolerance and partly across the softness past it.
    ColorKey {
        colors: Vec<String>,
        tolerance: f64,
        softness: f64,
        match_by: String,
    },
    /// D-111: `master`, `red`, `green` and `blue`, each a curve of 2 to 16 points `[in, out]`,
    /// 0 to 255, in rising order of in. Kept as written, so a file's wrong point is reported.
    Curves {
        master: Vec<Vec<f64>>,
        red: Vec<Vec<f64>>,
        green: Vec<Vec<f64>>,
        blue: Vec<Vec<f64>>,
    },
    /// D-112: `input_black` and `input_white`, 0 to 255, the range taken to 0..1; `gamma`, 0.1
    /// to 10, above 1 lightening the middle; and `output_black` and `output_white`, 0 to 255,
    /// the range it is laid on. A white below its black inverts.
    Levels {
        input_black: f64,
        input_white: f64,
        gamma: f64,
        output_black: f64,
        output_white: f64,
    },
    /// D-113: `hue`, -180 to 180 degrees the hue is turned; `saturation`, -100 to 100 per cent
    /// it is scaled by; and `lightness`, -100 to 100 per cent of the way to white, or to black
    /// below 0.
    HueSaturation {
        hue: f64,
        saturation: f64,
        lightness: f64,
    },
    /// D-114: `shape`, "linear" or "radial"; `start` and `end`, per cent of the drawing's width
    /// and height, each -1000 to 1000; `start_color` and `end_color`, `#rrggbb`;
    /// `start_opacity` and `end_opacity`, 0 to 100; and `blend`, "normal", "multiply",
    /// "screen" or "add". The words and colours are kept as written, so a wrong one is reported.
    Gradient {
        shape: String,
        start: [f64; 2],
        end: [f64; 2],
        start_color: String,
        end_color: String,
        start_opacity: f64,
        end_opacity: f64,
        blend: String,
    },
    /// D-115: `color`, `#rrggbb`, kept as written so a wrong one is reported; `opacity`, 0 to
    /// 100; `direction`, -3600 to 3600 degrees clockwise from up; `distance`, 0 to 1000 pixels;
    /// and `softness`, 0 to 500 pixels.
    DropShadow {
        color: String,
        opacity: f64,
        direction: f64,
        distance: f64,
        softness: f64,
    },
    /// D-116: `radius`, 0 to 200 pixels, and `edges`, "transparent" or "repeat", kept as
    /// written so a wrong one is reported. D-121: `iris`, "circle" or "triangle" to "decagon",
    /// kept as written; `roundness`, 0 to 100; `rotation`, -3600 to 3600 degrees clockwise from
    /// up; `aspect`, 0.1 to 10; `highlight_gain`, 0 to 100; and `highlight_threshold`, 0 to 100.
    LensBlur {
        radius: f64,
        edges: String,
        iris: String,
        roundness: f64,
        rotation: f64,
        aspect: f64,
        highlight_gain: f64,
        highlight_threshold: f64,
    },
    /// D-117: `color`, `#rrggbb`; `direction`, -3600 to 3600 degrees clockwise from up, where
    /// the light is; `width`, 0 to 100 pixels; `softness`, 0 to 100 pixels; `intensity`, 0 to
    /// 100; and `blend`, "normal", "add", "screen" or "multiply". The words and colour are kept
    /// as written, so a wrong one is reported.
    RimLight {
        color: String,
        direction: f64,
        width: f64,
        softness: f64,
        intensity: f64,
        blend: String,
    },
    /// D-118: `color`, `#rrggbb`, kept as written so a wrong one is reported; `width`, 0 to 100
    /// pixels; `softness`, 0 to 100 pixels; and `opacity`, 0 to 100.
    Outline {
        color: String,
        width: f64,
        softness: f64,
        opacity: f64,
    },
    /// D-119: `amount`, 0 to 100; `mode`, "mono" or "color"; `seed`, 0 to 100000, its whole
    /// part counted; and `animate`, "on" or "off". The words are kept as written, so a wrong
    /// one is reported. `frame` is not a setting and is never saved: it is the composition frame
    /// the settings were resolved at, or 0 when the grain does not move, so the grain and the
    /// effect cache's key both change with it.
    Noise {
        amount: f64,
        mode: String,
        seed: f64,
        animate: String,
        frame: i32,
    },
    /// D-120: `amount`, 0 to 100 pixels, how far red and blue each move at the drawing's
    /// corner; and `center`, per cent of the drawing's width and height, -1000 to 1000.
    ChromaticAberration { amount: f64, center: [f64; 2] },
    /// D-123: `color`, `#rrggbb`; `width`, 0 to 1000 pixels in from the edge; `opacity`, 0 to
    /// 100; `invert`, "off" or "on"; and `blend`, "normal", "multiply", "screen" or "add". The
    /// words and the colour are kept as written, so a wrong one is reported.
    DistanceGradation {
        color: String,
        width: f64,
        opacity: f64,
        invert: String,
        blend: String,
    },
    /// D-124: `center`, per cent of the drawing's width and height, -1000 to 1000; `length`,
    /// 0 to 100 per cent of the way to the centre; `threshold`, 0 to 100, D-89's bright test;
    /// `intensity`, 0 to 10; and `color`, `#rrggbb`, kept as written so a wrong one is reported.
    LightRays {
        center: [f64; 2],
        length: f64,
        threshold: f64,
        intensity: f64,
        color: String,
    },
    /// D-125: `amount`, 0 to 4 stops either way; `hold`, 1 to 100 frames, its whole part
    /// counted; and `seed`, 0 to 100000, its whole part counted. `frame` is not a setting and is
    /// never saved: it is the composition frame the settings were resolved at, as Noise's is.
    ExposureFlicker {
        amount: f64,
        hold: f64,
        seed: f64,
        frame: i32,
    },
    /// D-126: `amount`, 0 to 100; `color`, `#rrggbb`, kept as written so a wrong one is
    /// reported; `size`, 1 to 200, 100 putting the outer edge at the drawing's corners;
    /// `roundness`, 0 to 100; `softness`, 0 to 100; and `center`, per cent of the drawing's
    /// width and height, -1000 to 1000.
    Vignette {
        amount: f64,
        color: String,
        size: f64,
        roundness: f64,
        softness: f64,
        center: [f64; 2],
    },
    /// D-127: `amount`, 0 to 1000 pixels; `size`, 1 to 1000 pixels a wave; `complexity`, 1 to
    /// 8, its whole part counted; `evolution`, -100000 to 100000 degrees; `speed`, -360 to 360
    /// degrees a frame; `seed`, 0 to 100000, its whole part counted; and `edges`,
    /// "transparent" or "repeat", kept as written so a wrong one is reported. `frame` is not a
    /// setting and is never saved: it is the composition frame, as Noise's is.
    TurbulentDisplace {
        amount: f64,
        size: f64,
        complexity: f64,
        evolution: f64,
        speed: f64,
        seed: f64,
        edges: String,
        frame: i32,
    },
    /// D-128: `size`, 1 to 1000 pixels a cloud; `complexity`, 1 to 8, its whole part counted;
    /// `contrast`, 0 to 1000; `brightness`, -100 to 100; `evolution`, -100000 to 100000
    /// degrees; `speed`, -360 to 360 degrees a frame; `seed`, 0 to 100000, its whole part
    /// counted; `dark_color` and `light_color`, `#rrggbb`; `opacity`, 0 to 100; and `blend`,
    /// "normal", "multiply", "screen" or "add". The words and the colours are kept as written,
    /// so a wrong one is reported. `frame` is not a setting and is never saved: it is the
    /// composition frame, as Noise's is.
    FractalNoise {
        size: f64,
        complexity: f64,
        contrast: f64,
        brightness: f64,
        evolution: f64,
        speed: f64,
        seed: f64,
        dark_color: String,
        light_color: String,
        opacity: f64,
        blend: String,
        frame: i32,
    },
    /// D-129: `shadow_color`, `midtone_color` and `highlight_color`, `#rrggbb`, kept as written
    /// so a wrong one is reported; `midpoint`, 1 to 99; and `amount`, 0 to 100.
    GradientMap {
        shadow_color: String,
        midtone_color: String,
        highlight_color: String,
        midpoint: f64,
        amount: f64,
    },
    /// D-130: `shadows`, `midtones` and `highlights`, each red, green and blue, -100 to 100.
    /// Kept as written, so a tone of the wrong count is reported rather than refusing the file.
    ColorBalance {
        shadows: Vec<f64>,
        midtones: Vec<f64>,
        highlights: Vec<f64>,
    },
    /// D-131: `shift`, x then y in pixels, each -100000 to 100000, slid with wrap-around.
    Offset { shift: [f64; 2] },
    /// D-132: `width`, 0 to 500, how far in from the layer's edge the light reaches; `intensity`,
    /// 0 to 100 per cent up to 400; and `blend`, "screen" or "add", kept as written so a wrong
    /// one is reported. It reads the frame beneath the layer, so the renderer runs it.
    LightWrap {
        width: f64,
        intensity: f64,
        blend: String,
    },
    /// D-134: `channel`, "rgb", "red", "green", "blue" or "alpha", kept as written so a wrong one
    /// is reported; `amount`, 0 to 100 per cent of the way to the opposite.
    Invert {
        channel: String,
        amount: f64,
    },
    /// D-135: `brightness`, -150 to 150 levels of 255 added to every channel; `contrast`, -100
    /// to 100, the tones drawn together (below 0) or pushed apart (above 0) around half.
    BrightnessContrast {
        brightness: f64,
        contrast: f64,
    },
    /// D-136: how light each of six colour ranges turns in grey, -200 to 300 per cent.
    BlackWhite {
        reds: f64,
        yellows: f64,
        greens: f64,
        cyans: f64,
        blues: f64,
        magentas: f64,
    },
    /// D-137: `levels`, 2 to 256 steps per channel; only its whole part counts.
    Posterize { levels: f64 },
    /// D-138: `level`, 0 to 255: a pixel whose lightness is at or above it turns white, the rest
    /// black.
    Threshold { level: f64 },
    /// D-139: `red`, `green` and `blue`, each the row that remakes that channel: how much of the
    /// red, green and blue it takes and a constant, -200 to 200 per cent. Kept as written, so a
    /// row of the wrong count is reported rather than refusing the file. `monochrome`, "off" or
    /// "on": on, every channel takes the red row.
    ChannelMixer {
        red: Vec<f64>,
        green: Vec<f64>,
        blue: Vec<f64>,
        monochrome: String,
    },
    /// D-140: `vibrance`, -100 to 100, the dull colours moved from their grey most and the vivid
    /// ones least; `saturation`, -100 to 100, every colour moved alike.
    Vibrance {
        vibrance: f64,
        saturation: f64,
    },
    /// D-141: `color`, `#rrggbb`, the hue kept; `tolerance`, 0 to 100, how far round the wheel a
    /// hue is kept whole; `softness`, 0 to 100, the band past it where the keeping fades; and
    /// `amount`, 0 to 100, how far every other colour is turned grey.
    LeaveColor {
        color: String,
        tolerance: f64,
        softness: f64,
        amount: f64,
    },
    /// D-142: `threshold`, 0 to 255: each colour channel at or above it is turned to its
    /// opposite, the rest kept.
    Solarize { threshold: f64 },
    /// D-143: manga screentone. `size`, 2 to 200 pixels, one dot's square cell; `angle`, -3600 to
    /// 3600 degrees, the screen's turn; `ink` and `paper`, `#rrggbb`; `amount`, 0 to 100, how
    /// strongly they are laid over the picture.
    Halftone {
        size: f64,
        angle: f64,
        ink: String,
        paper: String,
        amount: f64,
    },
    /// D-144: `size`, 1 to 1000 pixels, each square block painted its mean colour.
    Mosaic { size: f64 },
    /// D-145: `direction`, -3600 to 3600 degrees clockwise from up, where the light comes from;
    /// `relief`, 0 to 100 pixels, how far ahead and behind each pixel looks; `contrast`, 0 to
    /// 1000, how strongly a difference shows; `mode`, "grey" or "color".
    Emboss {
        direction: f64,
        relief: f64,
        contrast: f64,
        mode: String,
    },
    /// D-146: `invert`, "off" for dark lines on white or "on" for light lines on black;
    /// `amount`, 0 to 100, how far each pixel goes toward its line.
    FindEdges { invert: String, amount: f64 },
    /// D-147: `amount`, 0 to 500, how far each colour is pushed from its blur; `radius`, 0 to
    /// 100 pixels, the sigma of that blur.
    Sharpen { amount: f64, radius: f64 },
    /// D-148: `radius`, 0 to 500 pixels, how far the glow spreads, three times its blur's sigma;
    /// `amount`, 0 to 100, how much of it is laid on; `blend`, "screen", "lighten" or "normal".
    Diffusion {
        radius: f64,
        amount: f64,
        blend: String,
    },
    /// D-149: `shape`, "sine" or "triangle"; `height`, 0 to 1000 pixels, how far the wave
    /// pushes; `width`, 1 to 10000 pixels, how long one wave is; `direction`, -3600 to 3600
    /// degrees clockwise from up, the way the wave runs; `speed`, -360 to 360 degrees a frame;
    /// `phase`, -100000 to 100000 degrees; and `edges`, "transparent" or "repeat". The words are
    /// kept as written, so a wrong one is reported. `frame` is not a setting and is never saved:
    /// it is the composition frame, as Noise's is.
    WaveWarp {
        shape: String,
        height: f64,
        width: f64,
        direction: f64,
        speed: f64,
        phase: f64,
        edges: String,
        frame: i32,
    },
    /// D-150: `center`, per cent of the drawing's width and height, -1000 to 1000 each;
    /// `amplitude`, 0 to 1000 pixels, how far the rings push; `wavelength`, 1 to 10000 pixels
    /// between rings; `speed`, -360 to 360 degrees a frame; `phase`, -100000 to 100000 degrees;
    /// and `fade`, 0 to 100000 pixels, where the rings die away, or 0 for never. `frame` is not a
    /// setting and is never saved: it is the composition frame, as Noise's is.
    Ripple {
        center: [f64; 2],
        amplitude: f64,
        wavelength: f64,
        speed: f64,
        phase: f64,
        fade: f64,
        frame: i32,
    },
    /// D-151: `angle`, -3600 to 3600 degrees, clockwise when above 0, the turn at the middle;
    /// `radius`, 0 to 10000 pixels, how far out it reaches; `center`, per cent of the drawing's
    /// width and height, -1000 to 1000 each.
    Twirl {
        angle: f64,
        radius: f64,
        center: [f64; 2],
    },
    /// D-152: `center`, per cent of the drawing's width and height, -1000 to 1000 each;
    /// `radius`, 0 to 10000 pixels, how far out it reaches; `height`, -4 to 4, a swell above 0
    /// and a pinch below.
    Bulge {
        center: [f64; 2],
        radius: f64,
        height: f64,
    },
    /// D-153: `center`, per cent of the drawing's width and height, -1000 to 1000 each, a point
    /// on the line; `angle`, -3600 to 3600 degrees, the line's turn from straight up and down.
    Mirror { center: [f64; 2], angle: f64 },
    /// D-154: `output_width` and `output_height`, 100 to 1000 per cent of the drawing's width
    /// and height; `mirror`, "off" or "on", every other tile turned over.
    MotionTile {
        output_width: f64,
        output_height: f64,
        mirror: String,
    },
    /// D-155: `completion`, 0 to 100 per cent, how far the edge has gone; `angle`, -3600 to 3600
    /// degrees, the way it moves, 90 to the right; `feather`, 0 to 10000 pixels, how soft it is.
    LinearWipe {
        completion: f64,
        angle: f64,
        feather: f64,
    },
    /// D-156: `completion`, 0 to 100 per cent of a turn swept away; `start_angle`, -3600 to 3600
    /// degrees, 0 straight up; `center`, per cent of the drawing's width and height, -1000 to
    /// 1000 each; `wipe`, "clockwise", "counterclockwise" or "both"; `feather`, 0 to 360
    /// degrees.
    RadialWipe {
        completion: f64,
        start_angle: f64,
        center: [f64; 2],
        wipe: String,
        feather: f64,
    },
    /// D-157: `completion`, 0 to 100 per cent of each slat cleared; `angle`, -3600 to 3600
    /// degrees, 0 level; `width`, 1 to 10000 pixels, each slat; `feather`, 0 to 10000 pixels.
    VenetianBlinds {
        completion: f64,
        angle: f64,
        width: f64,
        feather: f64,
    },
    /// D-158: `completion`, 0 to 100 per cent closed; `center`, per cent of the drawing's width
    /// and height, -1000 to 1000 each; `feather`, 0 to 10000 pixels; `invert`, "off" or "on", a
    /// hole opening instead.
    IrisWipe {
        completion: f64,
        center: [f64; 2],
        feather: f64,
        invert: String,
    },
    /// D-159: `choke`, -100 to 100 pixels, positive to shrink the covering and negative to
    /// spread it.
    SimpleChoker { choke: f64 },
    /// D-160: `center`, per cent of the drawing's width and height, -1000 to 1000 each; `color`,
    /// `#rrggbb`, kept as written so a wrong one is reported; `count`, 4 to 1000, its whole part
    /// counted; `thickness`, 0 to 30 degrees; `inner`, 0 to 100000 pixels; `inner_jitter` and
    /// `angle_jitter`, 0 to 100; `seed`, 0 to 100000, its whole part counted; `hold`, 1 to 100
    /// frames, its whole part counted; and `opacity`, 0 to 100. `frame` is not a setting and is
    /// never saved: it is the composition frame, as Noise's is.
    SpeedLines {
        center: [f64; 2],
        color: String,
        count: f64,
        thickness: f64,
        inner: f64,
        inner_jitter: f64,
        angle_jitter: f64,
        seed: f64,
        hold: f64,
        opacity: f64,
        frame: i32,
    },
    /// D-161: `threshold`, 0 to 100, D-89's bright test; `length`, 0 to 1000 pixels, its whole
    /// part counted; `points`, 1 to 8 arms, its whole part counted; `angle`, -3600 to 3600
    /// degrees, the first arm's way; `intensity`, 0 to 10; and `color`, `#rrggbb`, kept as
    /// written so a wrong one is reported.
    CrossGlare {
        threshold: f64,
        length: f64,
        points: f64,
        angle: f64,
        intensity: f64,
        color: String,
    },
    /// D-162: `amount`, 0 to 1000 pixels, the most a jolt moves the drawing each way; `rotation`,
    /// 0 to 45 degrees, the most it tips it; `hold`, 1 to 100 frames, and `seed`, 0 to 100000,
    /// their whole parts counted. `frame` is not a setting and is never saved: it is the
    /// composition frame, as Noise's is.
    CameraShake {
        amount: f64,
        rotation: f64,
        hold: f64,
        seed: f64,
        frame: i32,
    },
    /// D-163: `color`, `#rrggbb`, kept as written so a wrong one is reported; `density`, 0 to
    /// 100; `spacing`, 2 to 1000 pixels; `length`, 0 to 1000 pixels; `width`, 0 to 20 pixels;
    /// `direction`, -3600 to 3600 degrees, the way the rain falls; `speed`, 0 to 1000 pixels a
    /// frame; `seed`, 0 to 100000, its whole part counted; and `opacity`, 0 to 100. `frame` is
    /// not a setting and is never saved: it is the composition frame, as Noise's is.
    Rain {
        color: String,
        density: f64,
        spacing: f64,
        length: f64,
        width: f64,
        direction: f64,
        speed: f64,
        seed: f64,
        opacity: f64,
        frame: i32,
    },
    /// D-182: `lut`, the id of an asset of kind lut, or empty for none. `table` is not a setting
    /// and is never saved: it is the file `lut` names, read for the frame by `crate::lut::fill`.
    ColorLookup { lut: String, table: Option<crate::lut::Table> },
    /// D-183: `length`, 0 to 50 pixels along the line; `strength`, 0 to 100 per cent; and
    /// `lines_only`, "off" or "on", each pixel's move scaled by its own ink.
    LineBlur { length: f64, strength: f64, lines_only: String },
    /// D-184: windows of `hue`, 0 to 360 degrees, within `hue_range`, 0 to 180, and of
    /// `saturation` and `value`, 0 to 100 per cent, within their ranges, 0 to 100; and
    /// `invert`, "off" or "on".
    HsvKey {
        hue: f64,
        saturation: f64,
        value: f64,
        hue_range: f64,
        saturation_range: f64,
        value_range: f64,
        invert: String,
    },
    /// An effect this build does not have. Preserved, never drawn, always reported.
    Unsupported { type_id: String },
}

/// The identifiers used in the project file. Namespaced the way the fixture's
/// `vendor.future.effect` is, so a built-in and a third-party effect can never collide.
pub const EXPOSURE: &str = "core.exposure";
pub const GAUSSIAN_BLUR: &str = "core.gaussian_blur";
pub const TINT: &str = "core.tint";
pub const LINE_SMOOTH: &str = "core.line_smooth";
pub const SELECTIVE_COLOR_BLUR: &str = "core.selective_color_blur";
pub const GLOW: &str = "core.glow";
pub const LINE_RECOLOR: &str = "core.line_recolor";
pub const DIRECTIONAL_BLUR: &str = "core.directional_blur";
pub const SELECT_COLOR: &str = "core.select_color";
pub const LINE_WIDTH: &str = "core.line_width";
pub const RADIAL_BLUR: &str = "core.radial_blur";
pub const BLOOM: &str = "core.bloom";
pub const COLOR_KEY: &str = "core.color_key";
pub const CURVES: &str = "core.curves";
pub const LEVELS: &str = "core.levels";
pub const HUE_SATURATION: &str = "core.hue_saturation";
pub const GRADIENT: &str = "core.gradient";
pub const DROP_SHADOW: &str = "core.drop_shadow";
pub const LENS_BLUR: &str = "core.lens_blur";
pub const RIM_LIGHT: &str = "core.rim_light";
pub const OUTLINE: &str = "core.outline";
pub const NOISE: &str = "core.noise";
pub const CHROMATIC_ABERRATION: &str = "core.chromatic_aberration";
pub const DISTANCE_GRADATION: &str = "core.distance_gradation";
pub const LIGHT_RAYS: &str = "core.light_rays";
pub const EXPOSURE_FLICKER: &str = "core.exposure_flicker";
pub const VIGNETTE: &str = "core.vignette";
pub const TURBULENT_DISPLACE: &str = "core.turbulent_displace";
pub const FRACTAL_NOISE: &str = "core.fractal_noise";
pub const GRADIENT_MAP: &str = "core.gradient_map";
pub const COLOR_BALANCE: &str = "core.color_balance";
pub const OFFSET: &str = "core.offset";
pub const LIGHT_WRAP: &str = "core.light_wrap";
pub const INVERT: &str = "core.invert";
pub const BRIGHTNESS_CONTRAST: &str = "core.brightness_contrast";
pub const BLACK_WHITE: &str = "core.black_white";
pub const POSTERIZE: &str = "core.posterize";
pub const THRESHOLD: &str = "core.threshold";
pub const CHANNEL_MIXER: &str = "core.channel_mixer";
pub const VIBRANCE: &str = "core.vibrance";
pub const LEAVE_COLOR: &str = "core.leave_color";
pub const SOLARIZE: &str = "core.solarize";
pub const HALFTONE: &str = "core.halftone";
pub const MOSAIC: &str = "core.mosaic";
pub const EMBOSS: &str = "core.emboss";
pub const FIND_EDGES: &str = "core.find_edges";
pub const SHARPEN: &str = "core.sharpen";
pub const DIFFUSION: &str = "core.diffusion";
pub const WAVE_WARP: &str = "core.wave_warp";
pub const RIPPLE: &str = "core.ripple";
pub const TWIRL: &str = "core.twirl";
pub const BULGE: &str = "core.bulge";
pub const MIRROR: &str = "core.mirror";
pub const MOTION_TILE: &str = "core.motion_tile";
pub const LINEAR_WIPE: &str = "core.linear_wipe";
pub const RADIAL_WIPE: &str = "core.radial_wipe";
pub const VENETIAN_BLINDS: &str = "core.venetian_blinds";
pub const IRIS_WIPE: &str = "core.iris_wipe";
pub const SIMPLE_CHOKER: &str = "core.simple_choker";
pub const SPEED_LINES: &str = "core.speed_lines";
pub const CROSS_GLARE: &str = "core.cross_glare";
pub const CAMERA_SHAKE: &str = "core.camera_shake";
pub const RAIN: &str = "core.rain";
pub const COLOR_LOOKUP: &str = "core.color_lookup";
pub const LINE_BLUR: &str = "core.line_blur";
pub const HSV_KEY: &str = "core.hsv_key";

/// D-68: one key of an effect's setting, as a command gives it. `value` is one number, or a
/// colour's three.
#[derive(Clone, PartialEq, Debug)]
pub struct EffectKey {
    pub frame: i32,
    pub value: Vec<f64>,
    pub interp: Interp,
}

/// The numbers of key `i` of a setting's track: one, or a colour's three.
fn key_numbers(track: &[Property], i: usize) -> Vec<f64> {
    track
        .iter()
        .map(|p| p.keyframes()[i].value.as_scalar().unwrap_or(0.0))
        .collect()
}

impl Effect {
    /// Every setting that is numbers: its name in the file, its numbers, and the range document
    /// 21 holds them to. The one table `arity`, `get`, `set`, `at` and the newer effects' range
    /// sentences read, so a setting is named in one place.
    fn numbers(&mut self) -> Vec<(&'static str, Vec<&mut f64>, f64, f64)> {
        match self {
            // D-90: past 20 stops `2^e` soon overflows, and a sigma past 500 a machine's memory.
            Effect::Exposure { stops } => vec![("stops", vec![stops], -20.0, 20.0)],
            Effect::GaussianBlur { sigma_px, .. } => vec![("sigma_px", vec![sigma_px], 0.0, 500.0)],
            // A colour in linear light has no range but being a number.
            Effect::Tint { color, amount } => vec![
                ("color", color.iter_mut().collect(), f64::MIN, f64::MAX),
                ("amount", vec![amount], 0.0, 1.0),
            ],
            Effect::LineSmooth {
                softness,
                threshold,
            } => vec![
                ("softness", vec![softness], 0.0, 100.0),
                ("threshold", vec![threshold], 0.0, 255.0),
            ],
            Effect::SelectiveColorBlur {
                blur, tolerance, ..
            } => vec![
                ("blur", vec![blur], 0.0, 200.0),
                ("tolerance", vec![tolerance], 0.0, 255.0),
            ],
            Effect::Glow {
                threshold,
                tolerance,
                radius,
                intensity,
                ..
            } => vec![
                ("threshold", vec![threshold], 0.0, 100.0),
                ("tolerance", vec![tolerance], 0.0, 255.0),
                ("radius", vec![radius], 0.0, 500.0),
                ("intensity", vec![intensity], 0.0, 10.0),
            ],
            Effect::LineRecolor { tolerance, .. } | Effect::SelectColor { tolerance, .. } => {
                vec![("tolerance", vec![tolerance], 0.0, 255.0)]
            }
            Effect::DirectionalBlur { direction, length, .. } => vec![
                ("direction", vec![direction], -3600.0, 3600.0),
                ("length", vec![length], 0.0, 500.0),
            ],
            Effect::LineWidth {
                width, tolerance, ..
            } => vec![
                ("width", vec![width], -20.0, 20.0),
                ("tolerance", vec![tolerance], 0.0, 255.0),
            ],
            Effect::RadialBlur { amount, center, .. } => vec![
                ("amount", vec![amount], 0.0, 100.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
            ],
            Effect::Bloom {
                threshold,
                radius,
                intensity,
                length,
                angle,
                ..
            } => vec![
                ("threshold", vec![threshold], 0.0, 100.0),
                ("radius", vec![radius], 0.0, 500.0),
                ("intensity", vec![intensity], 0.0, 10.0),
                ("length", vec![length], 0.0, 500.0),
                ("angle", vec![angle], -3600.0, 3600.0),
            ],
            Effect::ColorKey {
                tolerance,
                softness,
                ..
            } => vec![
                ("tolerance", vec![tolerance], 0.0, 255.0),
                ("softness", vec![softness], 0.0, 255.0),
            ],
            // D-111: points are not keyed, so a curve is no setting of numbers here.
            Effect::Curves { .. } => vec![],
            Effect::Levels {
                input_black,
                input_white,
                gamma,
                output_black,
                output_white,
            } => vec![
                ("input_black", vec![input_black], 0.0, 255.0),
                ("input_white", vec![input_white], 0.0, 255.0),
                ("gamma", vec![gamma], 0.1, 10.0),
                ("output_black", vec![output_black], 0.0, 255.0),
                ("output_white", vec![output_white], 0.0, 255.0),
            ],
            Effect::HueSaturation {
                hue,
                saturation,
                lightness,
            } => vec![
                ("hue", vec![hue], -180.0, 180.0),
                ("saturation", vec![saturation], -100.0, 100.0),
                ("lightness", vec![lightness], -100.0, 100.0),
            ],
            Effect::Gradient {
                start,
                end,
                start_opacity,
                end_opacity,
                ..
            } => vec![
                ("start", start.iter_mut().collect(), -1000.0, 1000.0),
                ("end", end.iter_mut().collect(), -1000.0, 1000.0),
                ("start_opacity", vec![start_opacity], 0.0, 100.0),
                ("end_opacity", vec![end_opacity], 0.0, 100.0),
            ],
            Effect::DropShadow {
                opacity,
                direction,
                distance,
                softness,
                ..
            } => vec![
                ("opacity", vec![opacity], 0.0, 100.0),
                ("direction", vec![direction], -3600.0, 3600.0),
                ("distance", vec![distance], 0.0, 1000.0),
                ("softness", vec![softness], 0.0, 500.0),
            ],
            Effect::LensBlur {
                radius,
                roundness,
                rotation,
                aspect,
                highlight_gain,
                highlight_threshold,
                ..
            } => vec![
                ("radius", vec![radius], 0.0, 200.0),
                ("roundness", vec![roundness], 0.0, 100.0),
                ("rotation", vec![rotation], -3600.0, 3600.0),
                ("aspect", vec![aspect], 0.1, 10.0),
                ("highlight_gain", vec![highlight_gain], 0.0, 100.0),
                ("highlight_threshold", vec![highlight_threshold], 0.0, 100.0),
            ],
            Effect::RimLight {
                direction,
                width,
                softness,
                intensity,
                ..
            } => vec![
                ("direction", vec![direction], -3600.0, 3600.0),
                ("width", vec![width], 0.0, 100.0),
                ("softness", vec![softness], 0.0, 100.0),
                ("intensity", vec![intensity], 0.0, 100.0),
            ],
            Effect::Outline {
                width,
                softness,
                opacity,
                ..
            } => vec![
                ("width", vec![width], 0.0, 100.0),
                ("softness", vec![softness], 0.0, 100.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::Noise { amount, seed, .. } => vec![
                ("amount", vec![amount], 0.0, 100.0),
                ("seed", vec![seed], 0.0, 100000.0),
            ],
            Effect::ChromaticAberration { amount, center } => vec![
                ("amount", vec![amount], 0.0, 100.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
            ],
            Effect::DistanceGradation { width, opacity, .. } => vec![
                ("width", vec![width], 0.0, 1000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::LightRays {
                center,
                length,
                threshold,
                intensity,
                ..
            } => vec![
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("length", vec![length], 0.0, 100.0),
                ("threshold", vec![threshold], 0.0, 100.0),
                ("intensity", vec![intensity], 0.0, 10.0),
            ],
            Effect::ExposureFlicker {
                amount, hold, seed, ..
            } => vec![
                ("amount", vec![amount], 0.0, 4.0),
                ("hold", vec![hold], 1.0, 100.0),
                ("seed", vec![seed], 0.0, 100000.0),
            ],
            Effect::Vignette {
                amount,
                size,
                roundness,
                softness,
                center,
                ..
            } => vec![
                ("amount", vec![amount], 0.0, 100.0),
                ("size", vec![size], 1.0, 200.0),
                ("roundness", vec![roundness], 0.0, 100.0),
                ("softness", vec![softness], 0.0, 100.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
            ],
            Effect::TurbulentDisplace {
                amount,
                size,
                complexity,
                evolution,
                speed,
                seed,
                ..
            } => vec![
                ("amount", vec![amount], 0.0, 1000.0),
                ("size", vec![size], 1.0, 1000.0),
                ("complexity", vec![complexity], 1.0, 8.0),
                ("evolution", vec![evolution], -100000.0, 100000.0),
                ("speed", vec![speed], -360.0, 360.0),
                ("seed", vec![seed], 0.0, 100000.0),
            ],
            Effect::FractalNoise {
                size,
                complexity,
                contrast,
                brightness,
                evolution,
                speed,
                seed,
                opacity,
                ..
            } => vec![
                ("size", vec![size], 1.0, 1000.0),
                ("complexity", vec![complexity], 1.0, 8.0),
                ("contrast", vec![contrast], 0.0, 1000.0),
                ("brightness", vec![brightness], -100.0, 100.0),
                ("evolution", vec![evolution], -100000.0, 100000.0),
                ("speed", vec![speed], -360.0, 360.0),
                ("seed", vec![seed], 0.0, 100000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::GradientMap {
                midpoint, amount, ..
            } => vec![
                ("midpoint", vec![midpoint], 1.0, 99.0),
                ("amount", vec![amount], 0.0, 100.0),
            ],
            Effect::ColorBalance {
                shadows,
                midtones,
                highlights,
            } => vec![
                ("shadows", shadows.iter_mut().collect(), -100.0, 100.0),
                ("midtones", midtones.iter_mut().collect(), -100.0, 100.0),
                ("highlights", highlights.iter_mut().collect(), -100.0, 100.0),
            ],
            Effect::Offset { shift } => {
                vec![("shift", shift.iter_mut().collect(), -100000.0, 100000.0)]
            }
            Effect::LightWrap {
                width, intensity, ..
            } => vec![
                ("width", vec![width], 0.0, 500.0),
                ("intensity", vec![intensity], 0.0, 400.0),
            ],
            Effect::Invert { amount, .. } => vec![("amount", vec![amount], 0.0, 100.0)],
            Effect::BrightnessContrast { brightness, contrast } => vec![
                ("brightness", vec![brightness], -150.0, 150.0),
                ("contrast", vec![contrast], -100.0, 100.0),
            ],
            Effect::BlackWhite { reds, yellows, greens, cyans, blues, magentas } => vec![
                ("reds", vec![reds], -200.0, 300.0),
                ("yellows", vec![yellows], -200.0, 300.0),
                ("greens", vec![greens], -200.0, 300.0),
                ("cyans", vec![cyans], -200.0, 300.0),
                ("blues", vec![blues], -200.0, 300.0),
                ("magentas", vec![magentas], -200.0, 300.0),
            ],
            Effect::Posterize { levels } => vec![("levels", vec![levels], 2.0, 256.0)],
            Effect::Threshold { level } => vec![("level", vec![level], 0.0, 255.0)],
            Effect::ChannelMixer { red, green, blue, .. } => vec![
                ("red", red.iter_mut().collect(), -200.0, 200.0),
                ("green", green.iter_mut().collect(), -200.0, 200.0),
                ("blue", blue.iter_mut().collect(), -200.0, 200.0),
            ],
            Effect::Vibrance { vibrance, saturation } => vec![
                ("vibrance", vec![vibrance], -100.0, 100.0),
                ("saturation", vec![saturation], -100.0, 100.0),
            ],
            Effect::LeaveColor {
                tolerance,
                softness,
                amount,
                ..
            } => vec![
                ("tolerance", vec![tolerance], 0.0, 100.0),
                ("softness", vec![softness], 0.0, 100.0),
                ("amount", vec![amount], 0.0, 100.0),
            ],
            Effect::Solarize { threshold } => vec![("threshold", vec![threshold], 0.0, 255.0)],
            Effect::Halftone {
                size,
                angle,
                amount,
                ..
            } => vec![
                ("size", vec![size], 2.0, 200.0),
                ("angle", vec![angle], -3600.0, 3600.0),
                ("amount", vec![amount], 0.0, 100.0),
            ],
            Effect::Mosaic { size } => vec![("size", vec![size], 1.0, 1000.0)],
            Effect::Emboss {
                direction,
                relief,
                contrast,
                ..
            } => vec![
                ("direction", vec![direction], -3600.0, 3600.0),
                ("relief", vec![relief], 0.0, 100.0),
                ("contrast", vec![contrast], 0.0, 1000.0),
            ],
            Effect::FindEdges { amount, .. } => vec![("amount", vec![amount], 0.0, 100.0)],
            Effect::Sharpen { amount, radius } => vec![
                ("amount", vec![amount], 0.0, 500.0),
                ("radius", vec![radius], 0.0, 100.0),
            ],
            Effect::Diffusion { radius, amount, .. } => vec![
                ("radius", vec![radius], 0.0, 500.0),
                ("amount", vec![amount], 0.0, 100.0),
            ],
            Effect::WaveWarp {
                height,
                width,
                direction,
                speed,
                phase,
                ..
            } => vec![
                ("height", vec![height], 0.0, 1000.0),
                ("width", vec![width], 1.0, 10000.0),
                ("direction", vec![direction], -3600.0, 3600.0),
                ("speed", vec![speed], -360.0, 360.0),
                ("phase", vec![phase], -100000.0, 100000.0),
            ],
            Effect::Ripple {
                center,
                amplitude,
                wavelength,
                speed,
                phase,
                fade,
                ..
            } => vec![
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("amplitude", vec![amplitude], 0.0, 1000.0),
                ("wavelength", vec![wavelength], 1.0, 10000.0),
                ("speed", vec![speed], -360.0, 360.0),
                ("phase", vec![phase], -100000.0, 100000.0),
                ("fade", vec![fade], 0.0, 100000.0),
            ],
            Effect::Twirl {
                angle,
                radius,
                center,
            } => vec![
                ("angle", vec![angle], -3600.0, 3600.0),
                ("radius", vec![radius], 0.0, 10000.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
            ],
            Effect::Bulge {
                center,
                radius,
                height,
            } => vec![
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("radius", vec![radius], 0.0, 10000.0),
                ("height", vec![height], -4.0, 4.0),
            ],
            Effect::Mirror { center, angle } => vec![
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("angle", vec![angle], -3600.0, 3600.0),
            ],
            Effect::MotionTile {
                output_width,
                output_height,
                ..
            } => vec![
                ("output_width", vec![output_width], 100.0, 1000.0),
                ("output_height", vec![output_height], 100.0, 1000.0),
            ],
            Effect::LinearWipe {
                completion,
                angle,
                feather,
            } => vec![
                ("completion", vec![completion], 0.0, 100.0),
                ("angle", vec![angle], -3600.0, 3600.0),
                ("feather", vec![feather], 0.0, 10000.0),
            ],
            Effect::RadialWipe {
                completion,
                start_angle,
                center,
                feather,
                ..
            } => vec![
                ("completion", vec![completion], 0.0, 100.0),
                ("start_angle", vec![start_angle], -3600.0, 3600.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("feather", vec![feather], 0.0, 360.0),
            ],
            Effect::VenetianBlinds {
                completion,
                angle,
                width,
                feather,
            } => vec![
                ("completion", vec![completion], 0.0, 100.0),
                ("angle", vec![angle], -3600.0, 3600.0),
                ("width", vec![width], 1.0, 10000.0),
                ("feather", vec![feather], 0.0, 10000.0),
            ],
            Effect::IrisWipe {
                completion,
                center,
                feather,
                ..
            } => vec![
                ("completion", vec![completion], 0.0, 100.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("feather", vec![feather], 0.0, 10000.0),
            ],
            Effect::SimpleChoker { choke } => vec![("choke", vec![choke], -100.0, 100.0)],
            Effect::SpeedLines {
                center,
                count,
                thickness,
                inner,
                inner_jitter,
                angle_jitter,
                seed,
                hold,
                opacity,
                ..
            } => vec![
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("count", vec![count], 4.0, 1000.0),
                ("thickness", vec![thickness], 0.0, 30.0),
                ("inner", vec![inner], 0.0, 100000.0),
                ("inner_jitter", vec![inner_jitter], 0.0, 100.0),
                ("angle_jitter", vec![angle_jitter], 0.0, 100.0),
                ("seed", vec![seed], 0.0, 100000.0),
                ("hold", vec![hold], 1.0, 100.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::CrossGlare {
                threshold,
                length,
                points,
                angle,
                intensity,
                ..
            } => vec![
                ("threshold", vec![threshold], 0.0, 100.0),
                ("length", vec![length], 0.0, 1000.0),
                ("points", vec![points], 1.0, 8.0),
                ("angle", vec![angle], -3600.0, 3600.0),
                ("intensity", vec![intensity], 0.0, 10.0),
            ],
            Effect::CameraShake {
                amount,
                rotation,
                hold,
                seed,
                ..
            } => vec![
                ("amount", vec![amount], 0.0, 1000.0),
                ("rotation", vec![rotation], 0.0, 45.0),
                ("hold", vec![hold], 1.0, 100.0),
                ("seed", vec![seed], 0.0, 100000.0),
            ],
            Effect::Rain {
                density,
                spacing,
                length,
                width,
                direction,
                speed,
                seed,
                opacity,
                ..
            } => vec![
                ("density", vec![density], 0.0, 100.0),
                ("spacing", vec![spacing], 2.0, 1000.0),
                ("length", vec![length], 0.0, 1000.0),
                ("width", vec![width], 0.0, 20.0),
                ("direction", vec![direction], -3600.0, 3600.0),
                ("speed", vec![speed], 0.0, 1000.0),
                ("seed", vec![seed], 0.0, 100000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::ColorLookup { .. } => vec![],
            Effect::LineBlur { length, strength, .. } => vec![
                ("length", vec![length], 0.0, 50.0),
                ("strength", vec![strength], 0.0, 100.0),
            ],
            Effect::HsvKey {
                hue,
                saturation,
                value,
                hue_range,
                saturation_range,
                value_range,
                ..
            } => vec![
                ("hue", vec![hue], 0.0, 360.0),
                ("saturation", vec![saturation], 0.0, 100.0),
                ("value", vec![value], 0.0, 100.0),
                ("hue_range", vec![hue_range], 0.0, 180.0),
                ("saturation_range", vec![saturation_range], 0.0, 100.0),
                ("value_range", vec![value_range], 0.0, 100.0),
            ],
            Effect::Unsupported { .. } => vec![],
        }
    }

    /// How many numbers the setting of this name holds, or `None` when this effect has no such
    /// setting.
    pub fn arity(&self, name: &str) -> Option<usize> {
        let mut e = self.clone();
        let n = e
            .numbers()
            .into_iter()
            .find(|n| n.0 == name)
            .map(|n| n.1.len());
        n
    }

    /// The numbers the setting of this name holds, or `None` when this effect has no such setting.
    pub fn get(&self, name: &str) -> Option<Vec<f64>> {
        let mut e = self.clone();
        let v = e
            .numbers()
            .into_iter()
            .find(|n| n.0 == name)
            .map(|n| n.1.into_iter().map(|v| *v).collect());
        v
    }

    /// Put `v` in the setting of this name. A name or a count that does not fit changes nothing.
    pub fn set(&mut self, name: &str, v: &[f64]) {
        if let Some((_, slots, ..)) = self.numbers().into_iter().find(|n| n.0 == name) {
            if slots.len() == v.len() {
                for (slot, value) in slots.into_iter().zip(v) {
                    *slot = *value;
                }
            }
        }
    }

    /// Every setting that is a distance in pixels put through `scale`: a draft preview's
    /// smaller frame (D-66), or a precomposition's picture drawn at another size (D-67). An
    /// invalid setting is left as it is, so it is bypassed as it is at full size rather than
    /// scaled back inside its range.
    pub fn scale_distances(&mut self, scale: impl Fn(f64) -> f64) {
        if !self.is_valid() {
            return;
        }
        match self {
            Effect::GaussianBlur { sigma_px, .. } => *sigma_px = scale(*sigma_px),
            // D-87's blur and D-89's radius are distances in pixels too.
            Effect::SelectiveColorBlur { blur, .. } => *blur = scale(*blur),
            Effect::Glow { radius, .. } => *radius = scale(*radius),
            Effect::DirectionalBlur { length, .. } => *length = scale(*length),
            Effect::LineWidth { width, .. } => *width = scale(*width),
            Effect::LineBlur { length, .. } => *length = scale(*length),
            Effect::Bloom { radius, length, .. } => {
                *radius = scale(*radius);
                *length = scale(*length);
            }
            Effect::ChromaticAberration { amount, .. } => *amount = scale(*amount),
            Effect::DistanceGradation { width, .. } => *width = scale(*width),
            Effect::TurbulentDisplace { amount, size, .. } => {
                *amount = scale(*amount);
                // A wave under a pixel is held at one, as its range is, rather than bypassed.
                *size = scale(*size).max(1.0);
            }
            // D-128: held at one in a draft, as D-127's wave is.
            Effect::FractalNoise { size, .. } => *size = scale(*size).max(1.0),
            // D-143: held at its smallest, two, rather than bypassed.
            Effect::Halftone { size, .. } => *size = scale(*size).max(2.0),
            // D-144: a block under a pixel is one pixel, which changes nothing.
            Effect::Mosaic { size } => *size = scale(*size).max(1.0),
            Effect::Emboss { relief, .. } => *relief = scale(*relief),
            Effect::Sharpen { radius, .. } => *radius = scale(*radius),
            Effect::Diffusion { radius, .. } => *radius = scale(*radius),
            Effect::WaveWarp { height, width, .. } => {
                *height = scale(*height);
                *width = scale(*width).max(1.0);
            }
            Effect::Ripple {
                amplitude,
                wavelength,
                fade,
                ..
            } => {
                *amplitude = scale(*amplitude);
                *wavelength = scale(*wavelength).max(1.0);
                *fade = scale(*fade);
            }
            Effect::Twirl { radius, .. } => *radius = scale(*radius),
            Effect::Bulge { radius, .. } => *radius = scale(*radius),
            Effect::LinearWipe { feather, .. } => *feather = scale(*feather),
            Effect::IrisWipe { feather, .. } => *feather = scale(*feather),
            Effect::SimpleChoker { choke } => *choke = scale(*choke),
            Effect::SpeedLines { inner, .. } => *inner = scale(*inner),
            Effect::CrossGlare { length, .. } => *length = scale(*length),
            Effect::CameraShake { amount, .. } => *amount = scale(*amount),
            Effect::Rain {
                spacing,
                length,
                width,
                speed,
                ..
            } => {
                *spacing = scale(*spacing);
                *length = scale(*length);
                *width = scale(*width);
                *speed = scale(*speed);
            }
            // D-157: a slat is never less than a pixel, the least the command takes.
            Effect::VenetianBlinds { width, feather, .. } => {
                *width = scale(*width).max(1.0);
                *feather = scale(*feather);
            }
            Effect::Outline {
                width, softness, ..
            } => {
                *width = scale(*width);
                *softness = scale(*softness);
            }
            Effect::RimLight {
                width, softness, ..
            } => {
                *width = scale(*width);
                *softness = scale(*softness);
            }
            Effect::LensBlur { radius, .. } => *radius = scale(*radius),
            Effect::DropShadow {
                distance, softness, ..
            } => {
                *distance = scale(*distance);
                *softness = scale(*softness);
            }
            // D-132: so is the wrap's width.
            Effect::LightWrap { width, .. } => *width = scale(*width),
            // D-131: the shift is a distance, so a draft slides by its share.
            Effect::Offset { shift } => *shift = shift.map(&scale),
            _ => {}
        }
    }

    /// The name a person reads, the Effects panel's own: undo's list uses it.
    pub fn name(&self) -> &str {
        match self {
            Effect::Exposure { .. } => "Exposure",
            Effect::GaussianBlur { .. } => "Blur",
            Effect::Tint { .. } => "Tint",
            Effect::LineSmooth { .. } => "Line Smoothing",
            Effect::SelectiveColorBlur { .. } => "Selective Colour Blur",
            Effect::Glow { .. } => "Glow",
            Effect::LineRecolor { .. } => "Line Recolour",
            Effect::DirectionalBlur { .. } => "Directional Blur",
            Effect::SelectColor { .. } => "Select Colour",
            Effect::LineWidth { .. } => "Line Width",
            Effect::RadialBlur { .. } => "Radial Blur",
            Effect::Bloom { .. } => "Bloom",
            Effect::ColorKey { .. } => "Colour Key",
            Effect::Curves { .. } => "Curves",
            Effect::Levels { .. } => "Levels",
            Effect::HueSaturation { .. } => "Hue/Saturation",
            Effect::Gradient { .. } => "Gradient",
            Effect::DropShadow { .. } => "Drop Shadow",
            Effect::LensBlur { .. } => "Lens Blur",
            Effect::RimLight { .. } => "Rim Light",
            Effect::Outline { .. } => "Outline",
            Effect::Noise { .. } => "Noise",
            Effect::ChromaticAberration { .. } => "Chromatic Aberration",
            Effect::DistanceGradation { .. } => "Distance Gradation",
            Effect::LightRays { .. } => "Light Rays",
            Effect::ExposureFlicker { .. } => "Exposure Flicker",
            Effect::Vignette { .. } => "Vignette",
            Effect::TurbulentDisplace { .. } => "Turbulent Displace",
            Effect::FractalNoise { .. } => "Fractal Noise",
            Effect::GradientMap { .. } => "Gradient Map",
            Effect::ColorBalance { .. } => "Color Balance",
            Effect::Offset { .. } => "Offset",
            Effect::LightWrap { .. } => "Light Wrap",
            Effect::Invert { .. } => "Invert",
            Effect::BrightnessContrast { .. } => "Brightness & Contrast",
            Effect::BlackWhite { .. } => "Black & White",
            Effect::Posterize { .. } => "Posterize",
            Effect::Threshold { .. } => "Threshold",
            Effect::ChannelMixer { .. } => "Channel Mixer",
            Effect::Vibrance { .. } => "Vibrance",
            Effect::LeaveColor { .. } => "Leave Color",
            Effect::Solarize { .. } => "Solarize",
            Effect::Halftone { .. } => "Halftone",
            Effect::Mosaic { .. } => "Mosaic",
            Effect::Emboss { .. } => "Emboss",
            Effect::FindEdges { .. } => "Find Edges",
            Effect::Sharpen { .. } => "Sharpen",
            Effect::Diffusion { .. } => "Diffusion",
            Effect::WaveWarp { .. } => "Wave Warp",
            Effect::Ripple { .. } => "Ripple",
            Effect::Twirl { .. } => "Twirl",
            Effect::Bulge { .. } => "Bulge",
            Effect::Mirror { .. } => "Mirror",
            Effect::MotionTile { .. } => "Motion Tile",
            Effect::LinearWipe { .. } => "Linear Wipe",
            Effect::RadialWipe { .. } => "Radial Wipe",
            Effect::VenetianBlinds { .. } => "Venetian Blinds",
            Effect::IrisWipe { .. } => "Iris Wipe",
            Effect::SimpleChoker { .. } => "Simple Choker",
            Effect::SpeedLines { .. } => "Speed Lines",
            Effect::CrossGlare { .. } => "Cross Glare",
            Effect::CameraShake { .. } => "Camera Shake",
            Effect::Rain { .. } => "Rain",
            Effect::ColorLookup { .. } => "Color Lookup",
            Effect::LineBlur { .. } => "Line Blur",
            Effect::HsvKey { .. } => "HSV Key",
            Effect::Unsupported { type_id } => type_id,
        }
    }

    pub fn type_id(&self) -> &str {
        match self {
            Effect::Exposure { .. } => EXPOSURE,
            Effect::GaussianBlur { .. } => GAUSSIAN_BLUR,
            Effect::Tint { .. } => TINT,
            Effect::LineSmooth { .. } => LINE_SMOOTH,
            Effect::SelectiveColorBlur { .. } => SELECTIVE_COLOR_BLUR,
            Effect::Glow { .. } => GLOW,
            Effect::LineRecolor { .. } => LINE_RECOLOR,
            Effect::DirectionalBlur { .. } => DIRECTIONAL_BLUR,
            Effect::SelectColor { .. } => SELECT_COLOR,
            Effect::LineWidth { .. } => LINE_WIDTH,
            Effect::RadialBlur { .. } => RADIAL_BLUR,
            Effect::Bloom { .. } => BLOOM,
            Effect::ColorKey { .. } => COLOR_KEY,
            Effect::Curves { .. } => CURVES,
            Effect::Levels { .. } => LEVELS,
            Effect::HueSaturation { .. } => HUE_SATURATION,
            Effect::Gradient { .. } => GRADIENT,
            Effect::DropShadow { .. } => DROP_SHADOW,
            Effect::LensBlur { .. } => LENS_BLUR,
            Effect::RimLight { .. } => RIM_LIGHT,
            Effect::Outline { .. } => OUTLINE,
            Effect::Noise { .. } => NOISE,
            Effect::ChromaticAberration { .. } => CHROMATIC_ABERRATION,
            Effect::DistanceGradation { .. } => DISTANCE_GRADATION,
            Effect::LightRays { .. } => LIGHT_RAYS,
            Effect::ExposureFlicker { .. } => EXPOSURE_FLICKER,
            Effect::Vignette { .. } => VIGNETTE,
            Effect::TurbulentDisplace { .. } => TURBULENT_DISPLACE,
            Effect::FractalNoise { .. } => FRACTAL_NOISE,
            Effect::GradientMap { .. } => GRADIENT_MAP,
            Effect::ColorBalance { .. } => COLOR_BALANCE,
            Effect::Offset { .. } => OFFSET,
            Effect::LightWrap { .. } => LIGHT_WRAP,
            Effect::Invert { .. } => INVERT,
            Effect::BrightnessContrast { .. } => BRIGHTNESS_CONTRAST,
            Effect::BlackWhite { .. } => BLACK_WHITE,
            Effect::Posterize { .. } => POSTERIZE,
            Effect::Threshold { .. } => THRESHOLD,
            Effect::ChannelMixer { .. } => CHANNEL_MIXER,
            Effect::Vibrance { .. } => VIBRANCE,
            Effect::LeaveColor { .. } => LEAVE_COLOR,
            Effect::Solarize { .. } => SOLARIZE,
            Effect::Halftone { .. } => HALFTONE,
            Effect::Mosaic { .. } => MOSAIC,
            Effect::Emboss { .. } => EMBOSS,
            Effect::FindEdges { .. } => FIND_EDGES,
            Effect::Sharpen { .. } => SHARPEN,
            Effect::Diffusion { .. } => DIFFUSION,
            Effect::WaveWarp { .. } => WAVE_WARP,
            Effect::Ripple { .. } => RIPPLE,
            Effect::Twirl { .. } => TWIRL,
            Effect::Bulge { .. } => BULGE,
            Effect::Mirror { .. } => MIRROR,
            Effect::MotionTile { .. } => MOTION_TILE,
            Effect::LinearWipe { .. } => LINEAR_WIPE,
            Effect::RadialWipe { .. } => RADIAL_WIPE,
            Effect::VenetianBlinds { .. } => VENETIAN_BLINDS,
            Effect::IrisWipe { .. } => IRIS_WIPE,
            Effect::SimpleChoker { .. } => SIMPLE_CHOKER,
            Effect::SpeedLines { .. } => SPEED_LINES,
            Effect::CrossGlare { .. } => CROSS_GLARE,
            Effect::CameraShake { .. } => CAMERA_SHAKE,
            Effect::Rain { .. } => RAIN,
            Effect::ColorLookup { .. } => COLOR_LOOKUP,
            Effect::LineBlur { .. } => LINE_BLUR,
            Effect::HsvKey { .. } => HSV_KEY,
            Effect::Unsupported { type_id } => type_id,
        }
    }

    /// Document 21 line 89: "Every effect declares input bounds expansion." In pixels, on every
    /// side.
    ///
    /// An unsupported effect declares zero because it is not run. That is not a claim about what
    /// the effect would expand by if this build had it -- it is the bounds of doing nothing,
    /// which is what actually happens, and the export says fidelity is incomplete so that the
    /// difference is never mistaken for a rendered result.
    pub fn bounds_expansion(&self) -> usize {
        match self {
            // D-109: a blur repeating its edge pixels draws nothing past them.
            Effect::GaussianBlur { edges, .. } | Effect::DirectionalBlur { edges, .. }
                if edges == "repeat" =>
            {
                0
            }
            Effect::GaussianBlur { sigma_px, .. } => kernel_radius(*sigma_px),
            // D-89: the light reaches `radius` pixels, blur's reach at sigma radius / 3.
            Effect::Glow { radius, .. } => kernel_radius(*radius / 3.0),
            // D-92: half the streak, on each side.
            Effect::DirectionalBlur { length, .. } => (*length / 2.0).ceil() as usize,
            // D-94: a thicker line reaches its width further out; a thinner one nowhere.
            Effect::LineWidth { width, .. } if *width > 0.0 => width.ceil() as usize,
            // D-183: the tensor's reach past anything drawn, at any length but 0.
            Effect::LineBlur { length, .. } if *length > 0.0 => crate::line_blur::GROW,
            // D-96: the widest blur's reach, or the streaks' length if that is more.
            Effect::Bloom {
                radius,
                streaks,
                length,
                ..
            } => crate::bloom::reach(*radius, crate::bloom::lines(streaks), *length),
            // D-118: the width rounded up and the blur's reach, or nothing at width 0.
            Effect::Outline {
                width, softness, ..
            } => {
                if *width == 0.0 {
                    0
                } else {
                    width.ceil() as usize + kernel_radius(*softness / 3.0)
                }
            }
            // D-116: nothing past the edge pixels when they repeat, the radius rounded up if not;
            // D-121: stretched by the aspect.
            Effect::LensBlur {
                radius,
                edges,
                aspect,
                ..
            } => {
                if edges == "repeat" {
                    0
                } else {
                    crate::layer_fx::lens_reach(*radius, *aspect).ceil() as usize
                }
            }
            // D-149: the height rounded up, unless a push past the edge reads the edge.
            Effect::WaveWarp { height, edges, .. } if edges != "repeat" => height.ceil() as usize,
            // D-159: a spread's reach, rounded down; a shrink grows nothing.
            Effect::SimpleChoker { choke } if *choke < 0.0 => (-choke).floor() as usize,
            // D-161: the length taken down to a whole number, unless nothing is added.
            Effect::CrossGlare {
                length, intensity, ..
            } if *intensity > 0.0 => length.floor() as usize,
            // D-127: the amount rounded up, unless a push past the edge reads the edge.
            Effect::TurbulentDisplace { amount, edges, .. } if edges != "repeat" => {
                amount.ceil() as usize
            }
            // D-115: the shadow's move, rounded up, and its blur's reach.
            Effect::DropShadow {
                distance, softness, ..
            } => distance.ceil() as usize + kernel_radius(*softness / 3.0),
            _ => 0,
        }
    }

    /// Whether the parameters are inside the ranges document 21 states.
    ///
    /// A negative sigma has no kernel and a tint amount outside 0..1 is an extrapolation the
    /// specification does not define, so both are refused at the command boundary rather than
    /// clamped. Clamping would accept a number and silently render a different one.
    pub fn is_valid(&self) -> bool {
        match self {
            // D-90: past 20 stops `2^e` soon overflows, and a sigma past 500 a machine's memory.
            Effect::Exposure { stops } => (-20.0..=20.0).contains(stops),
            Effect::GaussianBlur { sigma_px, .. } => {
                (0.0..=500.0).contains(sigma_px) && self.fault().is_none()
            }
            Effect::Tint { color, amount } => {
                color.iter().all(|c| c.is_finite())
                    && amount.is_finite()
                    && (0.0..=1.0).contains(amount)
            }
            Effect::LineSmooth {
                softness,
                threshold,
            } => (0.0..=100.0).contains(softness) && (0.0..=255.0).contains(threshold),
            Effect::SelectiveColorBlur {
                blur,
                colors,
                tolerance,
            } => {
                (0.0..=200.0).contains(blur)
                    && (0.0..=255.0).contains(tolerance)
                    && colors.len() <= 8
                    && colors.iter().all(|c| crate::selective_blur::parse_hex(c).is_some())
            }
            Effect::Glow { .. } => glow_fault(self).is_none(),
            Effect::Unsupported { .. } => true,
            _ => self.fault().is_none(),
        }
    }

    /// Why [`is_valid`](Self::is_valid) said no, as a sentence for a person.
    pub fn why_invalid(&self) -> String {
        match self {
            Effect::Exposure { stops } => {
                format!("Exposure runs from -20 to 20 stops, and this is {stops}.")
            }
            Effect::GaussianBlur { sigma_px, .. } if !(0.0..=500.0).contains(sigma_px) => {
                format!("A Gaussian blur's sigma runs from 0 to 500, and this is {sigma_px}.")
            }
            Effect::Tint { amount, .. } => {
                format!("A tint amount runs from 0 to 1, and this is {amount}.")
            }
            Effect::LineSmooth {
                softness,
                threshold,
            } => format!(
                "Line smoothing's softness runs from 0 to 100 and its threshold from 0 to 255, \
                 and these are {softness} and {threshold}."
            ),
            Effect::SelectiveColorBlur {
                blur,
                colors,
                tolerance,
            } => {
                match colors.iter().find(|c| crate::selective_blur::parse_hex(c).is_none()) {
                    Some(bad) => format!(
                        "A chosen colour is written # and six hexadecimal digits, such as \
                         #f6d6be, and this is \"{bad}\"."
                    ),
                    None if colors.len() > 8 => format!(
                        "Selective colour blur takes up to eight colours, and this has {}.",
                        colors.len()
                    ),
                    None if !(0.0..=255.0).contains(tolerance) => format!(
                        "Selective colour blur's tolerance runs from 0 to 255, and this is \
                         {tolerance}."
                    ),
                    None => format!(
                        "Selective colour blur's blur runs from 0 to 200, and this is {blur}."
                    ),
                }
            }
            Effect::Glow { .. } => glow_fault(self).unwrap_or_default(),
            Effect::Unsupported { type_id } => {
                format!("{type_id} has no parameters this build checks.")
            }
            _ => self.fault().unwrap_or_default(),
        }
    }

    /// D-91 on: what is wrong with a newer effect's settings, as a sentence, or `None` when
    /// nothing is. Its words and colours first, then its numbers from the one table.
    fn fault(&self) -> Option<String> {
        let name = self.name();
        let hex = |c: &str| crate::selective_blur::parse_hex(c).is_some();
        let chosen = |colors: &[String]| {
            if colors.len() > 8 {
                return Some(format!(
                    "{name} takes up to eight colours, and this has {}.",
                    colors.len()
                ));
            }
            colors.iter().find(|c| !hex(c)).map(|bad| {
                format!(
                    "A chosen colour is written # and six hexadecimal digits, such as #f6d6be, \
                     and this is \"{bad}\"."
                )
            })
        };
        let one = |setting: &str, c: &str| {
            (!hex(c)).then(|| {
                format!(
                    "{name}'s {setting} is written # and six hexadecimal digits, such as \
                     #ff4000, and this is \"{c}\"."
                )
            })
        };
        // D-109.
        let edges = |e: &String| {
            (!["transparent", "repeat"].contains(&e.as_str())).then(|| {
                format!("{name}'s edges are \"transparent\" or \"repeat\", and this is \"{e}\".")
            })
        };
        let own = match self {
            Effect::GaussianBlur { edges: e, .. } | Effect::DirectionalBlur { edges: e, .. } => {
                edges(e)
            }
            Effect::LineRecolor {
                colors, new_color, ..
            } => chosen(colors).or_else(|| one("new colour", new_color)),
            Effect::SelectColor { colors, keep, .. } => chosen(colors).or_else(|| {
                (!["chosen", "others"].contains(&keep.as_str())).then(|| {
                    format!("{name} keeps \"chosen\" or \"others\", and this is \"{keep}\".")
                })
            }),
            Effect::LineWidth {
                based_on, colors, ..
            } => chosen(colors).or_else(|| {
                (!["shape", "colors"].contains(&based_on.as_str())).then(|| {
                    format!(
                        "{name} is based on \"shape\" or \"colors\", and this is \"{based_on}\"."
                    )
                })
            }),
            Effect::RadialBlur { kind, edges: e, .. } => (!["spin", "zoom"]
                .contains(&kind.as_str()))
            .then(|| format!("{name}'s type is \"spin\" or \"zoom\", and this is \"{kind}\"."))
            .or_else(|| edges(e)),
            Effect::Bloom { streaks, .. } => (!["none", "cross", "star"]
                .contains(&streaks.as_str()))
            .then(|| {
                format!(
                    "{name}'s streaks are \"none\", \"cross\" or \"star\", and this is \"{streaks}\"."
                )
            }),
            Effect::ColorKey {
                colors, match_by, ..
            } => chosen(colors).or_else(|| {
                (!["rgb", "hue"].contains(&match_by.as_str())).then(|| {
                    format!("{name} matches by \"rgb\" or \"hue\", and this is \"{match_by}\".")
                })
            }),
            Effect::Curves {
                master,
                red,
                green,
                blue,
            } => [("master", master), ("red", red), ("green", green), ("blue", blue)]
                .into_iter()
                .find_map(|(curve, points)| curve_fault(curve, points)),
            Effect::Gradient {
                shape,
                start_color,
                end_color,
                blend,
                ..
            } => gradient_fault(shape, start_color, end_color, blend),
            Effect::DropShadow { color, .. } => hex_fault("Drop Shadow", "colour", color),
            Effect::LensBlur { edges, .. } if !["transparent", "repeat"].contains(&edges.as_str()) => {
                Some(format!(
                    "Lens Blur's edges are \"transparent\" or \"repeat\", and this is \"{edges}\"."
                ))
            }
            Effect::LensBlur { iris, .. } if crate::layer_fx::blades(iris).is_none() => {
                Some(format!(
                    "Lens Blur's iris is \"circle\", \"triangle\", \"square\", \"pentagon\", \
                     \"hexagon\", \"heptagon\", \"octagon\", \"nonagon\" or \"decagon\", and \
                     this is \"{iris}\"."
                ))
            }
            Effect::RimLight { blend, .. }
                if !["normal", "add", "screen", "multiply"].contains(&blend.as_str()) =>
            {
                Some(format!(
                    "Rim Light's blend is \"normal\", \"add\", \"screen\" or \"multiply\", and \
                     this is \"{blend}\"."
                ))
            }
            Effect::RimLight { color, .. } => hex_fault("Rim Light", "colour", color),
            Effect::Outline { color, .. } => hex_fault("Outline", "colour", color),
            Effect::Noise { mode, .. } if mode != "mono" && mode != "color" => Some(format!(
                "Noise's mode is \"mono\" or \"color\", and this is \"{mode}\"."
            )),
            Effect::Noise { animate, .. } if animate != "on" && animate != "off" => Some(format!(
                "Noise's animate is \"on\" or \"off\", and this is \"{animate}\"."
            )),
            Effect::DistanceGradation { invert, .. } if invert != "off" && invert != "on" => {
                Some(format!(
                    "Distance Gradation's invert is \"off\" or \"on\", and this is \"{invert}\"."
                ))
            }
            Effect::DistanceGradation { blend, .. }
                if !["normal", "multiply", "screen", "add"].contains(&blend.as_str()) =>
            {
                Some(format!(
                    "Distance Gradation's blend is \"normal\", \"multiply\", \"screen\" or \
                     \"add\", and this is \"{blend}\"."
                ))
            }
            Effect::DistanceGradation { color, .. } => {
                hex_fault("Distance Gradation", "colour", color)
            }
            Effect::LightRays { color, .. } => hex_fault("Light Rays", "colour", color),
            Effect::Vignette { color, .. } => hex_fault("Vignette", "colour", color),
            Effect::TurbulentDisplace { edges: e, .. } => edges(e),
            Effect::FractalNoise { blend, .. }
                if !["normal", "multiply", "screen", "add"].contains(&blend.as_str()) =>
            {
                Some(format!(
                    "Fractal Noise's blend is \"normal\", \"multiply\", \"screen\" or \"add\", \
                     and this is \"{blend}\"."
                ))
            }
            Effect::FractalNoise {
                dark_color,
                light_color,
                ..
            } => hex_fault("Fractal Noise", "dark colour", dark_color)
                .or_else(|| hex_fault("Fractal Noise", "light colour", light_color)),
            Effect::GradientMap {
                shadow_color,
                midtone_color,
                highlight_color,
                ..
            } => hex_fault("Gradient Map", "shadow colour", shadow_color)
                .or_else(|| hex_fault("Gradient Map", "midtone colour", midtone_color))
                .or_else(|| hex_fault("Gradient Map", "highlight colour", highlight_color)),
            Effect::ColorBalance {
                shadows,
                midtones,
                highlights,
            } => [("shadows", shadows), ("midtones", midtones), ("highlights", highlights)]
                .into_iter()
                .find(|(_, tone)| tone.len() != 3)
                .map(|(what, tone)| {
                    format!(
                        "{name}'s {what} are three numbers, red, green and blue, and this has {}.",
                        tone.len()
                    )
                }),
            Effect::LightWrap { blend, .. } if !["screen", "add"].contains(&blend.as_str()) => {
                Some(format!(
                    "Light Wrap's blend is \"screen\" or \"add\", and this is \"{blend}\"."
                ))
            }
            Effect::Invert { channel, .. }
                if !["rgb", "red", "green", "blue", "alpha"].contains(&channel.as_str()) =>
            {
                Some(format!(
                    "Invert's channel is \"rgb\", \"red\", \"green\", \"blue\" or \"alpha\", and this is \"{channel}\"."
                ))
            }
            Effect::ChannelMixer {
                red,
                green,
                blue,
                monochrome,
            } => [("red", red), ("green", green), ("blue", blue)]
                .into_iter()
                .find(|(_, row)| row.len() != 4)
                .map(|(what, row)| {
                    format!(
                        "{name}'s {what} row is four numbers, from red, green, blue and a constant, and this has {}.",
                        row.len()
                    )
                })
                .or_else(|| {
                    (!["off", "on"].contains(&monochrome.as_str())).then(|| {
                        format!("Channel Mixer's monochrome is \"off\" or \"on\", and this is \"{monochrome}\".")
                    })
                }),
            Effect::LeaveColor { color, .. } => hex_fault("Leave Color", "colour", color),
            Effect::Halftone { ink, paper, .. } => {
                hex_fault("Halftone", "ink", ink).or_else(|| hex_fault("Halftone", "paper", paper))
            }
            Effect::Emboss { mode, .. } if !["grey", "color"].contains(&mode.as_str()) => Some(format!(
                "Emboss's mode is \"grey\" or \"color\", and this is \"{mode}\"."
            )),
            Effect::FindEdges { invert, .. } if !["off", "on"].contains(&invert.as_str()) => Some(format!(
                "Find Edges's invert is \"off\" or \"on\", and this is \"{invert}\"."
            )),
            Effect::Diffusion { blend, .. } if !["screen", "lighten", "normal"].contains(&blend.as_str()) => Some(format!(
                "Diffusion's blend is \"screen\", \"lighten\" or \"normal\", and this is \"{blend}\"."
            )),
            Effect::WaveWarp { shape, edges: e, .. } => (!["sine", "triangle"].contains(&shape.as_str()))
                .then(|| format!("{name}'s shape is \"sine\" or \"triangle\", and this is \"{shape}\"."))
                .or_else(|| edges(e)),
            Effect::MotionTile { mirror, .. } if !["off", "on"].contains(&mirror.as_str()) => Some(format!(
                "Motion Tile's mirror is \"off\" or \"on\", and this is \"{mirror}\"."
            )),
            Effect::LineBlur { lines_only, .. } if !["off", "on"].contains(&lines_only.as_str()) => Some(format!(
                "Line Blur's lines only is \"off\" or \"on\", and this is \"{lines_only}\"."
            )),
            Effect::HsvKey { invert, .. } if !["off", "on"].contains(&invert.as_str()) => Some(format!(
                "HSV Key's invert is \"off\" or \"on\", and this is \"{invert}\"."
            )),
            Effect::RadialWipe { wipe, .. }
                if !["clockwise", "counterclockwise", "both"].contains(&wipe.as_str()) =>
            {
                Some(format!(
                    "Radial Wipe's wipe is \"clockwise\", \"counterclockwise\" or \"both\", and this is \"{wipe}\"."
                ))
            }
            Effect::IrisWipe { invert, .. } if !["off", "on"].contains(&invert.as_str()) => Some(format!(
                "Iris Wipe's invert is \"off\" or \"on\", and this is \"{invert}\"."
            )),
            Effect::SpeedLines { color, .. } => hex_fault("Speed Lines", "colour", color),
            Effect::CrossGlare { color, .. } => hex_fault("Cross Glare", "colour", color),
            Effect::Rain { color, .. } => hex_fault("Rain", "colour", color),
            _ => None,
        };
        own.or_else(|| {
            let mut e = self.clone();
            let bad = e
                .numbers()
                .into_iter()
                .find_map(|(setting, slots, low, high)| {
                    slots
                        .into_iter()
                        .find(|v| !(low..=high).contains(&**v))
                        .map(|v| {
                            format!(
                                "{name}'s {} runs from {low} to {high}, and this is {v}.",
                                setting.replace('_', " ")
                            )
                        })
                });
            bad
        })
    }
}

/// D-115: what is wrong with `effect`'s `what`, a colour written `#rrggbb`, as a sentence, or
/// `None` when nothing is.
fn hex_fault(effect: &str, what: &str, c: &str) -> Option<String> {
    crate::selective_blur::parse_hex(c)
        .is_none()
        .then(|| format!("{effect}'s {what} is written #rrggbb, and this is \"{c}\"."))
}

/// A colour already found valid, encoded 0 to 1.
pub(crate) fn encoded(c: &str) -> [f64; 3] {
    crate::selective_blur::parse_hex(c)
        .unwrap_or_default()
        .map(|v| v as f64 / 255.0)
}

/// D-114: what is wrong with a gradient's words and colours, as a sentence, or `None` when
/// nothing is.
fn gradient_fault(shape: &str, start_color: &str, end_color: &str, blend: &str) -> Option<String> {
    if !["linear", "radial"].contains(&shape) {
        return Some(format!(
            "Gradient's shape is \"linear\" or \"radial\", and this is \"{shape}\"."
        ));
    }
    if !["normal", "multiply", "screen", "add"].contains(&blend) {
        return Some(format!(
            "Gradient's blend is \"normal\", \"multiply\", \"screen\" or \"add\", and this \
             is \"{blend}\"."
        ));
    }
    [("start colour", start_color), ("end colour", end_color)]
        .into_iter()
        .find(|(_, c)| crate::selective_blur::parse_hex(c).is_none())
        .map(|(what, c)| format!("Gradient's {what} is written #rrggbb, and this is \"{c}\"."))
}

/// D-111: what is wrong with one curve's points, as a sentence, or `None` when nothing is.
fn curve_fault(curve: &str, points: &[Vec<f64>]) -> Option<String> {
    if !(2..=16).contains(&points.len()) {
        return Some(format!(
            "Curves' {curve} curve takes 2 to 16 points, and this has {}.",
            points.len()
        ));
    }
    if let Some(p) = points.iter().find(|p| p.len() != 2) {
        return Some(format!(
            "Each point of Curves' {curve} curve is two numbers, in then out, and this one is \
             {p:?}."
        ));
    }
    if let Some(v) = points.iter().flatten().find(|v| !(0.0..=255.0).contains(*v)) {
        return Some(format!(
            "Curves' {curve} curve's points run from 0 to 255, and this has {v}."
        ));
    }
    points.windows(2).find(|w| w[1][0] <= w[0][0]).map(|w| {
        format!(
            "Curves' {curve} curve's in goes up from point to point, and {} is not above {}.",
            w[1][0], w[0][0]
        )
    })
}

/// D-89: what is wrong with a glow's settings, as a sentence, or `None` when nothing is.
fn glow_fault(effect: &Effect) -> Option<String> {
    let Effect::Glow {
        based_on,
        threshold,
        colors,
        tolerance,
        radius,
        intensity,
        operation,
        tint,
    } = effect
    else {
        return None;
    };
    let hex = |c: &str| crate::selective_blur::parse_hex(c).is_some();
    Some(if !["bright", "colors"].contains(&based_on.as_str()) {
        format!("Glow is based on \"bright\" or \"colors\", and this is \"{based_on}\".")
    } else if !["add", "screen"].contains(&operation.as_str()) {
        format!("Glow's operation is \"add\" or \"screen\", and this is \"{operation}\".")
    } else if !(0.0..=100.0).contains(threshold) {
        format!("Glow's threshold runs from 0 to 100, and this is {threshold}.")
    } else if !(0.0..=500.0).contains(radius) {
        format!("Glow's radius runs from 0 to 500, and this is {radius}.")
    } else if !(0.0..=10.0).contains(intensity) {
        format!("Glow's intensity runs from 0 to 10, and this is {intensity}.")
    } else if !(0.0..=255.0).contains(tolerance) {
        format!("Glow's tolerance runs from 0 to 255, and this is {tolerance}.")
    } else if colors.len() > 8 {
        format!("Glow takes up to eight colours, and this has {}.", colors.len())
    } else if let Some(bad) = colors.iter().find(|c| !hex(c)) {
        format!(
            "A chosen colour is written # and six hexadecimal digits, such as #f6d6be, and this \
             is \"{bad}\"."
        )
    } else if !tint.is_empty() && !hex(tint) {
        format!(
            "Glow's colour is empty or written # and six hexadecimal digits, such as #ff4000, \
             and this is \"{tint}\"."
        )
    } else {
        return None;
    })
}

/// Document 21: "kernel radius `ceil(3*sigma_px)`". Sigma zero gives radius zero, which is the
/// identity the same document asks for.
pub fn kernel_radius(sigma_px: f64) -> usize {
    if sigma_px.is_nan() || sigma_px <= 0.0 {
        return 0;
    }
    (3.0 * sigma_px).ceil() as usize
}

/// The separable normalised weights of document 21, index 0 being the sample `radius` pixels
/// before the centre.
///
/// Normalised after truncation, not before: the untruncated Gaussian integrates to one over the
/// whole line and this kernel is cut at three sigma, so weights taken straight from the
/// exponential would sum slightly under one and darken the image by that much. Dividing by the
/// realised sum is what makes a flat region survive the blur unchanged, which is the property
/// the fixture checks first.
pub fn gaussian_weights(sigma_px: f64) -> Vec<f32> {
    let radius = kernel_radius(sigma_px);
    if radius == 0 {
        return vec![1.0];
    }
    let mut w: Vec<f64> = (0..=2 * radius)
        .map(|i| {
            let d = i as f64 - radius as f64;
            (-(d * d) / (2.0 * sigma_px * sigma_px)).exp()
        })
        .collect();
    let sum: f64 = w.iter().sum();
    for v in &mut w {
        *v /= sum;
    }
    w.into_iter().map(|v| v as f32).collect()
}

/// D-95: a Radial Blur's centre in `source`'s pixels, from its share of the drawing's own size,
/// with the drawing's corner at `(ox, oy)` in `source` after the effects above it grew it.
pub(crate) fn radial_center(center: [f64; 2], source: &WorkingBuffer, (ox, oy): (usize, usize)) -> (f64, f64) {
    let (w0, h0) = (source.width() - 2 * ox, source.height() - 2 * oy);
    (
        ox as f64 + center[0] / 100.0 * w0 as f64,
        oy as f64 + center[1] / 100.0 * h0 as f64,
    )
}

/// Run one layer's stack over its pixels, in order.
///
/// Returns how far the buffer's origin moved, in pixels: `(0, 0)` unless a blur expanded it.
/// The caller must shift the layer transform by that offset, or every pixel moves.
///
/// `report` is called once for each instance that was skipped and why, which is how a bypassed
/// effect reaches the frame log and, through it, the incomplete-fidelity mark on an export.
pub fn apply_stack(
    source: &mut WorkingBuffer,
    stack: &[EffectInstance],
    report: impl FnMut(usize, &EffectInstance, Bypassed),
) -> (usize, usize) {
    apply_stack_at(source, stack, (0, 0), report)
}

/// B-65: [`apply_stack`] on a buffer the effects before `stack` have already grown, the drawing's
/// corner at `origin`, as Gradient, Noise and Chromatic Aberration read it. Returns the corner
/// after `stack`, `origin` included.
pub(crate) fn apply_stack_at(
    source: &mut WorkingBuffer,
    stack: &[EffectInstance],
    origin: (usize, usize),
    mut report: impl FnMut(usize, &EffectInstance, Bypassed),
) -> (usize, usize) {
    let (mut ox, mut oy) = origin;
    // The position is reported alongside the instance because P-11's effect cache replays a
    // bypass on a hit, and a position is the one thing about an instance that survives being
    // written down and read back next frame.
    for (at, instance) in stack.iter().enumerate() {
        if !instance.enabled {
            // Deliberately silent. A bypassed effect is a setting a person chose, not a fault,
            // and document 28's incomplete-fidelity mark is for what this build could not do.
            continue;
        }
        match &instance.effect {
            Effect::Unsupported { .. } => {
                report(at, instance, Bypassed::NotImplemented);
                continue;
            }
            e if !e.is_valid() => {
                report(at, instance, Bypassed::InvalidParameter);
                continue;
            }
            // One stage per kind of effect rather than one for the stack (P-11). The three are
            // disjoint and none of them nests, so `src/perf.rs`'s promise that the table can be
            // summed still holds; what they buy is the ranking P-11's entry says it needs before
            // it decides which effect is worth caching.
            Effect::Exposure { stops } => {
                crate::perf::time(crate::perf::Stage::EffectExposure, || {
                    exposure(source, *stops)
                })
            }
            Effect::Tint { color, amount } => {
                crate::perf::time(crate::perf::Stage::EffectTint, || {
                    tint(source, *color, *amount)
                })
            }
            Effect::GaussianBlur { sigma_px, edges } => {
                let r = crate::perf::time(crate::perf::Stage::EffectBlur, || {
                    if edges == "repeat" {
                        held_blur(source, *sigma_px);
                        0
                    } else {
                        blur(source, *sigma_px)
                    }
                });
                ox += r;
                oy += r;
            }
            Effect::LineSmooth {
                softness,
                threshold,
            } => crate::perf::time(crate::perf::Stage::EffectSmooth, || {
                crate::line_smooth::line_smooth(source, *softness, *threshold)
            }),
            Effect::SelectiveColorBlur {
                blur,
                colors,
                tolerance,
            } => {
                crate::perf::time(crate::perf::Stage::EffectSelBlur, || {
                    crate::selective_blur::selective_color_blur(source, *blur, colors, *tolerance)
                })
            }
            Effect::Glow {
                based_on,
                threshold,
                colors,
                tolerance,
                radius,
                intensity,
                operation,
                tint,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectGlow, || {
                    let g = crate::glow::settings(
                        based_on, *threshold, colors, *tolerance, *radius, *intensity,
                        operation, tint,
                    );
                    crate::glow::glow(source, &g)
                });
                ox += r;
                oy += r;
            }
            Effect::LineRecolor {
                colors,
                tolerance,
                new_color,
            } => crate::perf::time(crate::perf::Stage::EffectRecolor, || {
                crate::cel_fx::line_recolor(source, colors, *tolerance, new_color)
            }),
            Effect::DirectionalBlur {
                direction,
                length,
                edges,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectDirBlur, || {
                    crate::blurs::directional_blur(source, *direction, *length, edges == "repeat")
                });
                ox += r;
                oy += r;
            }
            Effect::SelectColor {
                colors,
                tolerance,
                keep,
            } => crate::perf::time(crate::perf::Stage::EffectSelect, || {
                crate::cel_fx::select_color(source, colors, *tolerance, keep == "chosen")
            }),
            Effect::LineWidth {
                width,
                based_on,
                colors,
                tolerance,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectWidth, || {
                    crate::line_width::line_width(
                        source,
                        *width,
                        based_on == "shape",
                        colors,
                        *tolerance,
                    )
                });
                ox += r;
                oy += r;
            }
            // D-95: the centre is a share of the drawing's own size, wherever an effect above
            // has moved its corner in the buffer.
            Effect::RadialBlur {
                kind,
                amount,
                center,
                edges,
            } => {
                let c = radial_center(*center, source, (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectRadial, || {
                    crate::blurs::radial_blur(source, kind == "spin", *amount, c, edges == "repeat")
                })
            }
            Effect::Bloom {
                threshold,
                radius,
                intensity,
                streaks,
                length,
                angle,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectBloom, || {
                    crate::bloom::bloom(
                        source,
                        *threshold,
                        *radius,
                        *intensity,
                        crate::bloom::lines(streaks),
                        *length,
                        *angle,
                    )
                });
                ox += r;
                oy += r;
            }
            Effect::ColorKey {
                colors,
                tolerance,
                softness,
                match_by,
            } => crate::perf::time(crate::perf::Stage::EffectColorKey, || {
                crate::cel_fx::color_key(source, colors, *tolerance, *softness, match_by == "hue")
            }),
            Effect::Curves {
                master,
                red,
                green,
                blue,
            } => crate::perf::time(crate::perf::Stage::EffectCurves, || {
                crate::grade::curves(source, master, [red, green, blue])
            }),
            Effect::Levels {
                input_black,
                input_white,
                gamma,
                output_black,
                output_white,
            } => crate::perf::time(crate::perf::Stage::EffectLevels, || {
                crate::grade::levels(
                    source,
                    [*input_black, *input_white, *gamma, *output_black, *output_white],
                )
            }),
            Effect::HueSaturation {
                hue,
                saturation,
                lightness,
            } => crate::perf::time(crate::perf::Stage::EffectHueSaturation, || {
                crate::grade::hue_saturation(source, *hue, *saturation, *lightness)
            }),
            // D-114: the two points are shares of the drawing's own size, as Radial Blur's
            // centre is.
            Effect::Gradient {
                shape,
                start,
                end,
                start_color,
                end_color,
                start_opacity,
                end_opacity,
                blend,
            } => {
                let g = crate::grade::Gradient {
                    radial: shape == "radial",
                    start: radial_center(*start, source, (ox, oy)),
                    end: radial_center(*end, source, (ox, oy)),
                    colors: [encoded(start_color), encoded(end_color)],
                    opacity: [*start_opacity, *end_opacity],
                    blend: blend.clone(),
                };
                crate::perf::time(crate::perf::Stage::EffectGradient, || {
                    crate::grade::gradient(source, &g)
                })
            }
            Effect::DropShadow {
                color,
                opacity,
                direction,
                distance,
                softness,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectDropShadow, || {
                    crate::layer_fx::drop_shadow(
                        source, encoded(color), *opacity, *direction, *distance, *softness,
                    )
                });
                ox += r;
                oy += r;
            }
            Effect::LensBlur {
                radius,
                edges,
                iris,
                roundness,
                rotation,
                aspect,
                highlight_gain,
                highlight_threshold,
            } => {
                let iris = crate::layer_fx::Iris {
                    blades: crate::layer_fx::blades(iris).unwrap_or(0),
                    roundness: *roundness,
                    rotation: *rotation,
                    aspect: *aspect,
                    gain: *highlight_gain,
                    threshold: *highlight_threshold,
                };
                let r = crate::perf::time(crate::perf::Stage::EffectLensBlur, || {
                    crate::layer_fx::lens_blur(source, *radius, edges == "repeat", &iris)
                });
                ox += r;
                oy += r;
            }
            Effect::RimLight {
                color,
                direction,
                width,
                softness,
                intensity,
                blend,
            } => crate::perf::time(crate::perf::Stage::EffectRimLight, || {
                crate::layer_fx::rim_light(
                    source, encoded(color), *direction, *width, *softness, *intensity, blend,
                )
            }),
            Effect::Outline {
                color,
                width,
                softness,
                opacity,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectOutline, || {
                    crate::layer_fx::outline(source, encoded(color), *width, *softness, *opacity)
                });
                ox += r;
                oy += r;
            }
            Effect::Noise {
                amount,
                mode,
                seed,
                frame,
                ..
            } => crate::perf::time(crate::perf::Stage::EffectNoise, || {
                crate::grade::noise(source, *amount, mode == "color", *seed, *frame, (ox, oy))
            }),
            Effect::ChromaticAberration { amount, center } => {
                crate::perf::time(crate::perf::Stage::EffectChromaticAberration, || {
                    crate::layer_fx::chromatic_aberration(source, *amount, *center, (ox, oy))
                })
            }
            Effect::DistanceGradation {
                color,
                width,
                opacity,
                invert,
                blend,
            } => crate::perf::time(crate::perf::Stage::EffectDistanceGradation, || {
                crate::layer_fx::distance_gradation(
                    source,
                    encoded(color),
                    *width,
                    *opacity,
                    invert == "on",
                    blend,
                )
            }),
            // D-124: the centre is a share of the drawing's own size, as Radial Blur's is.
            Effect::LightRays {
                center,
                length,
                threshold,
                intensity,
                color,
            } => {
                let c = radial_center(*center, source, (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectLightRays, || {
                    crate::layer_fx::light_rays(
                        source,
                        c,
                        *length,
                        *threshold,
                        *intensity,
                        encoded(color),
                    )
                })
            }
            // D-125: Noise's hash of the seed and the frame over the hold gives the stops.
            Effect::ExposureFlicker {
                amount,
                hold,
                seed,
                frame,
            } => {
                let stops = flicker_stops(*amount, *hold, *seed, *frame);
                crate::perf::time(crate::perf::Stage::EffectExposureFlicker, || {
                    exposure(source, stops)
                })
            }
            // D-126: the ellipse in the drawing's own size, however far the layer has grown.
            Effect::Vignette {
                amount,
                color,
                size,
                roundness,
                softness,
                center,
            } => {
                let v = vignette_settings(
                    *amount,
                    color,
                    [*size, *roundness, *softness],
                    *center,
                    source,
                    (ox, oy),
                );
                crate::perf::time(crate::perf::Stage::EffectVignette, || {
                    crate::grade::vignette(source, &v)
                })
            }
            // D-127: one full turn of evolution moves the field one wave.
            Effect::TurbulentDisplace {
                amount,
                size,
                complexity,
                evolution,
                speed,
                seed,
                edges,
                frame,
            } => {
                let z = depth(*evolution, *speed, *frame);
                let r = crate::perf::time(crate::perf::Stage::EffectTurbulentDisplace, || {
                    crate::layer_fx::turbulent_displace(
                        source,
                        *amount,
                        *size,
                        complexity.floor() as usize,
                        *seed,
                        z,
                        edges == "repeat",
                        (ox, oy),
                    )
                });
                ox += r;
                oy += r;
            }
            // D-128: one full turn of evolution moves the clouds one cloud.
            Effect::FractalNoise {
                size,
                complexity,
                contrast,
                brightness,
                evolution,
                speed,
                seed,
                dark_color,
                light_color,
                opacity,
                blend,
                frame,
            } => {
                let f = crate::grade::Fractal {
                    size: *size,
                    octaves: complexity.floor() as usize,
                    seed: *seed,
                    z: depth(*evolution, *speed, *frame),
                    contrast: *contrast,
                    brightness: *brightness,
                    colors: [encoded(dark_color), encoded(light_color)],
                    opacity: *opacity,
                    blend: blend.clone(),
                };
                crate::perf::time(crate::perf::Stage::EffectFractalNoise, || {
                    crate::grade::fractal_noise(source, &f, (ox, oy))
                })
            }
            Effect::GradientMap {
                shadow_color,
                midtone_color,
                highlight_color,
                midpoint,
                amount,
            } => crate::perf::time(crate::perf::Stage::EffectGradientMap, || {
                crate::grade::gradient_map(
                    source,
                    [shadow_color, midtone_color, highlight_color].map(|c| encoded(c)),
                    *midpoint,
                    *amount,
                )
            }),
            Effect::ColorBalance {
                shadows,
                midtones,
                highlights,
            } => crate::perf::time(crate::perf::Stage::EffectColorBalance, || {
                let tone = |t: &[f64]| [t[0], t[1], t[2]];
                crate::grade::color_balance(source, tone(shadows), tone(midtones), tone(highlights))
            }),
            Effect::Offset { shift } => crate::perf::time(crate::perf::Stage::EffectOffset, || {
                crate::layer_fx::offset(source, *shift)
            }),
            // D-132: not here. The wrap reads the frame beneath the layer, so the renderer runs
            // it as the layer is drawn (`render::wrap_layer`). In the layer's own space, and on
            // an adjustment layer, which has no drawing of its own, it changes nothing.
            Effect::LightWrap { .. } => {}
            Effect::Invert { channel, amount } => crate::perf::time(crate::perf::Stage::EffectInvert, || {
                match channel.as_str() {
                    "alpha" => crate::grade::invert_alpha(source, *amount),
                    "red" => crate::grade::invert(source, 0, *amount),
                    "green" => crate::grade::invert(source, 1, *amount),
                    "blue" => crate::grade::invert(source, 2, *amount),
                    _ => crate::grade::invert(source, 3, *amount),
                }
            }),
            Effect::BrightnessContrast { brightness, contrast } => {
                crate::perf::time(crate::perf::Stage::EffectBrightnessContrast, || {
                    crate::grade::brightness_contrast(source, *brightness, *contrast)
                })
            }
            Effect::BlackWhite { reds, yellows, greens, cyans, blues, magentas } => {
                crate::perf::time(crate::perf::Stage::EffectBlackWhite, || {
                    crate::grade::black_white(source, [*reds, *yellows, *greens, *cyans, *blues, *magentas])
                })
            }
            Effect::Posterize { levels } => crate::perf::time(crate::perf::Stage::EffectPosterize, || {
                crate::grade::posterize(source, *levels)
            }),
            Effect::Threshold { level } => crate::perf::time(crate::perf::Stage::EffectThreshold, || {
                crate::grade::threshold(source, *level)
            }),
            Effect::ChannelMixer {
                red,
                green,
                blue,
                monochrome,
            } => crate::perf::time(crate::perf::Stage::EffectChannelMixer, || {
                let row = |r: &[f64]| [r[0], r[1], r[2], r[3]];
                crate::grade::channel_mixer(source, [row(red), row(green), row(blue)], monochrome == "on")
            }),
            Effect::Vibrance { vibrance, saturation } => {
                crate::perf::time(crate::perf::Stage::EffectVibrance, || {
                    crate::grade::vibrance(source, *vibrance, *saturation)
                })
            }
            Effect::LeaveColor {
                color,
                tolerance,
                softness,
                amount,
            } => crate::perf::time(crate::perf::Stage::EffectLeaveColor, || {
                crate::grade::leave_color(source, encoded(color), *tolerance, *softness, *amount)
            }),
            Effect::Solarize { threshold } => crate::perf::time(crate::perf::Stage::EffectSolarize, || {
                crate::grade::solarize(source, *threshold)
            }),
            Effect::Halftone {
                size,
                angle,
                ink,
                paper,
                amount,
            } => crate::perf::time(crate::perf::Stage::EffectHalftone, || {
                let h = crate::grade::Halftone {
                    size: *size,
                    angle: *angle,
                    ink: encoded(ink),
                    paper: encoded(paper),
                    amount: *amount,
                };
                crate::grade::halftone(source, &h, (ox, oy))
            }),
            Effect::Mosaic { size } => crate::perf::time(crate::perf::Stage::EffectMosaic, || {
                crate::layer_fx::mosaic(source, *size, (ox, oy))
            }),
            Effect::Emboss {
                direction,
                relief,
                contrast,
                mode,
            } => crate::perf::time(crate::perf::Stage::EffectEmboss, || {
                crate::layer_fx::emboss(source, *direction, *relief, *contrast, mode == "color")
            }),
            Effect::FindEdges { invert, amount } => crate::perf::time(crate::perf::Stage::EffectFindEdges, || {
                crate::layer_fx::find_edges(source, invert == "on", *amount)
            }),
            Effect::Sharpen { amount, radius } => crate::perf::time(crate::perf::Stage::EffectSharpen, || {
                crate::layer_fx::sharpen(source, *amount, *radius)
            }),
            Effect::Diffusion { radius, amount, blend } => crate::perf::time(crate::perf::Stage::EffectDiffusion, || {
                crate::layer_fx::diffusion(source, *radius, *amount, blend)
            }),
            // D-149: the wave slides `speed` degrees a frame.
            Effect::WaveWarp {
                shape,
                height,
                width,
                direction,
                speed,
                phase,
                edges,
                frame,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectWaveWarp, || {
                    crate::layer_fx::wave_warp(
                        source,
                        shape == "triangle",
                        (*height, *width),
                        *direction,
                        phase + speed * *frame as f64,
                        edges == "repeat",
                        (ox, oy),
                    )
                });
                ox += r;
                oy += r;
            }
            // D-150: the centre is a share of the drawing's own size, as Radial Blur's is, and
            // the rings move `speed` degrees a frame.
            Effect::Ripple {
                center,
                amplitude,
                wavelength,
                speed,
                phase,
                fade,
                frame,
            } => {
                let c = radial_center(*center, source, (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectRipple, || {
                    crate::layer_fx::ripple(source, c, *amplitude, *wavelength, phase + speed * *frame as f64, *fade)
                })
            }
            // D-151: the centre is a share of the drawing's own size, as Radial Blur's is.
            Effect::Twirl {
                angle,
                radius,
                center,
            } => {
                let c = radial_center(*center, source, (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectTwirl, || {
                    crate::layer_fx::twirl(source, *angle, *radius, c)
                })
            }
            // D-152: the centre is a share of the drawing's own size, as Radial Blur's is.
            Effect::Bulge {
                center,
                radius,
                height,
            } => {
                let c = radial_center(*center, source, (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectBulge, || {
                    crate::layer_fx::bulge(source, *radius, *height, c)
                })
            }
            // D-153: the centre is a share of the drawing's own size, as Radial Blur's is.
            Effect::Mirror { center, angle } => {
                let c = radial_center(*center, source, (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectMirror, || {
                    crate::layer_fx::mirror(source, *angle, c)
                })
            }
            // D-154: the growth depends on the size the drawing reaches it at, and need not be
            // the same across as down.
            Effect::MotionTile {
                output_width,
                output_height,
                mirror,
            } => {
                let (gx, gy) = crate::perf::time(crate::perf::Stage::EffectMotionTile, || {
                    crate::layer_fx::motion_tile(source, (*output_width, *output_height), mirror == "on")
                });
                ox += gx;
                oy += gy;
            }
            // D-155: the edge crosses the drawing's own box, however an effect above grew it.
            Effect::LinearWipe {
                completion,
                angle,
                feather,
            } => crate::perf::time(crate::perf::Stage::EffectLinearWipe, || {
                crate::layer_fx::linear_wipe(source, *completion, *angle, *feather, (ox, oy))
            }),
            // D-156: the centre and the edge are the drawing's own, however an effect above grew it.
            Effect::RadialWipe {
                completion,
                start_angle,
                center,
                wipe,
                feather,
            } => crate::perf::time(crate::perf::Stage::EffectRadialWipe, || {
                let way = match wipe.as_str() {
                    "counterclockwise" => 1,
                    "both" => 2,
                    _ => 0,
                };
                crate::layer_fx::radial_wipe(source, *completion, *start_angle, *center, way, *feather, (ox, oy))
            }),
            // D-157: the slats are the drawing's own, however an effect above grew it.
            Effect::VenetianBlinds {
                completion,
                angle,
                width,
                feather,
            } => crate::perf::time(crate::perf::Stage::EffectVenetianBlinds, || {
                crate::layer_fx::venetian_blinds(source, *completion, *angle, *width, *feather, (ox, oy))
            }),
            // D-158: the circle is the drawing's own, however an effect above grew it.
            Effect::IrisWipe {
                completion,
                center,
                feather,
                invert,
            } => crate::perf::time(crate::perf::Stage::EffectIrisWipe, || {
                crate::layer_fx::iris_wipe(source, *completion, *center, *feather, invert == "on", (ox, oy))
            }),
            // D-159: a spread grows the layer by its reach, rounded down.
            Effect::SimpleChoker { choke } => {
                let r = crate::perf::time(crate::perf::Stage::EffectSimpleChoker, || {
                    crate::layer_fx::simple_choker(source, *choke)
                });
                ox += r;
                oy += r;
            }
            // D-160: the centre is a share of the drawing's own size, as Radial Blur's is.
            Effect::SpeedLines {
                center,
                color,
                count,
                thickness,
                inner,
                inner_jitter,
                angle_jitter,
                seed,
                hold,
                opacity,
                frame,
            } => {
                let c = radial_center(*center, source, (ox, oy));
                let numbers = [*count, *thickness, *inner, *inner_jitter, *angle_jitter, *seed, *hold, *opacity];
                crate::perf::time(crate::perf::Stage::EffectSpeedLines, || {
                    crate::layer_fx::speed_lines(source, c, encoded(color).map(crate::grade::to_linear), numbers, *frame)
                })
            }
            // D-161: the arms grow the layer by the length, taken down to a whole number.
            Effect::CrossGlare {
                threshold,
                length,
                points,
                angle,
                intensity,
                color,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectCrossGlare, || {
                    crate::layer_fx::cross_glare(source, *threshold, *length, *points, *angle, *intensity, encoded(color))
                });
                ox += r;
                oy += r;
            }
            // D-162: the jolt grows the layer by its reach, which depends on the drawing's size.
            Effect::CameraShake {
                amount,
                rotation,
                hold,
                seed,
                frame,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectCameraShake, || {
                    crate::layer_fx::camera_shake(source, [*amount, *rotation, *hold, *seed], *frame, (ox, oy))
                });
                ox += r;
                oy += r;
            }
            // D-163: the field is fixed to the drawing's own space; it grows nothing.
            Effect::Rain {
                color,
                density,
                spacing,
                length,
                width,
                direction,
                speed,
                seed,
                opacity,
                frame,
            } => {
                let numbers = [*density, *spacing, *length, *width, *direction, *speed, *seed, *opacity];
                crate::perf::time(crate::perf::Stage::EffectRain, || {
                    crate::layer_fx::rain(source, encoded(color).map(crate::grade::to_linear), numbers, *frame, (ox, oy))
                })
            }
            // D-182: without its table, a lookup naming no file, or one missing or refused,
            // changes nothing; `fill` said why.
            Effect::ColorLookup { table, .. } => {
                if let Some(t) = table {
                    crate::perf::time(crate::perf::Stage::EffectColorLookup, || {
                        crate::grade::color_lookup(source, &t.0)
                    })
                }
            }
            Effect::LineBlur {
                length,
                strength,
                lines_only,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectLineBlur, || {
                    crate::line_blur::line_blur(source, *length, *strength, lines_only == "on")
                });
                ox += r;
                oy += r;
            }
            Effect::HsvKey {
                hue,
                saturation,
                value,
                hue_range,
                saturation_range,
                value_range,
                invert,
            } => crate::perf::time(crate::perf::Stage::EffectHsvKey, || {
                let windows = crate::hsv_key::Windows {
                    hue: *hue,
                    hue_range: *hue_range,
                    saturation: *saturation,
                    saturation_range: *saturation_range,
                    value: *value,
                    value_range: *value_range,
                };
                crate::hsv_key::hsv_key(source, &windows, invert == "on")
            }),
        }
    }
    (ox, oy)
}

/// D-125: the stops an Exposure Flicker gives at `frame`, Noise's hash of the seed and the
/// frame over the hold. B-76's card takes the same.
pub(crate) fn flicker_stops(amount: f64, hold: f64, seed: f64, frame: i32) -> f64 {
    let m = (frame as i64).div_euclid(hold.floor() as i64);
    amount * crate::grade::unit(crate::grade::mix(seed.floor() as u64), m, 0, 0, 3)
}

/// D-127 and D-128: one full turn of evolution moves the field one cell.
pub(crate) fn depth(evolution: f64, speed: f64, frame: i32) -> f64 {
    (evolution + speed * frame as f64) / 360.0
}

/// D-126: the ellipse in the drawing's own size, however far the layer has grown (its corner at
/// `(ox, oy)` in `source`). `[size, roundness, softness]` as the effect holds them.
pub(crate) fn vignette_settings(
    amount: f64,
    color: &str,
    [size, roundness, softness]: [f64; 3],
    center: [f64; 2],
    source: &WorkingBuffer,
    (ox, oy): (usize, usize),
) -> crate::grade::Vignette {
    let w0 = (source.width() - 2 * ox) as f64;
    let h0 = (source.height() - 2 * oy) as f64;
    let (m, r) = (roundness / 100.0, (w0 * h0).sqrt() / 2.0);
    let outer = size / 100.0;
    crate::grade::Vignette {
        center: radial_center(center, source, (ox, oy)),
        radii: ((1.0 - m) * w0 / 2.0 + m * r, (1.0 - m) * h0 / 2.0 + m * r),
        inner: outer * (1.0 - softness / 100.0),
        outer,
        color: encoded(color),
        amount,
    }
}

/// Why an effect in the stack did not run.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Bypassed {
    /// This build does not have the effect. Document 28's `EFFECT_UNSUPPORTED`.
    NotImplemented,
    /// The build has the effect, but the stored parameters are outside what document 21 defines.
    /// Document 28's `EFFECT_PARAMETER_INVALID`.
    InvalidParameter,
}

/// Document 21: "linear premultiplied RGB is multiplied by `2^e`; alpha is unchanged."
///
/// Three channels, not four, and that is the whole difference between this and the mask's
/// coverage multiply. Exposure changes how much light a pixel carries and not how much of the
/// pixel there is, so the premultiplied product moves and the coverage does not.
fn exposure(source: &mut WorkingBuffer, stops: f64) {
    let gain = (2.0f64).powf(stops) as f32;
    if gain == 1.0 {
        return;
    }
    for px in source.data_mut().chunks_exact_mut(4) {
        for c in &mut px[..3] {
            *c *= gain;
        }
    }
}

/// Document 21: "Recover straight source RGB where alpha > 0, compute `mix(source_rgb,
/// tint_rgb, t)`, then premultiply by original alpha. Alpha is unchanged."
///
/// Written in exactly that order. Mixing the premultiplied values directly would be a different
/// operation: a half-transparent red would mix toward half the tint colour rather than toward
/// the tint colour, so the same paint would read as a different hue wherever the artwork is
/// soft. Where alpha is zero there is no straight colour to recover, and zero times anything is
/// zero, so those pixels are left alone -- which is document 09's "avoid ambiguities around
/// fully transparent pixels".
fn tint(source: &mut WorkingBuffer, color: [f64; 3], amount: f64) {
    if amount == 0.0 {
        return;
    }
    let t = amount as f32;
    let tint = [color[0] as f32, color[1] as f32, color[2] as f32];
    for px in source.data_mut().chunks_exact_mut(4) {
        let a = px[3];
        if a <= 0.0 {
            continue;
        }
        for i in 0..3 {
            let straight = px[i] / a;
            px[i] = (straight + (tint[i] - straight) * t) * a;
        }
    }
}

/// Document 21's Gaussian blur, separable, on premultiplied RGB and alpha together.
///
/// Returns the radius the buffer grew by on each side. The growth is document 21's "Bounds
/// expand by the kernel radius", and it is not a nicety: without it a character blurred near the
/// edge of its own cel would have the glow cut off in a straight line, which is a visible fault
/// that no amount of correct arithmetic inside the old extent would fix.
///
/// Two one-dimensional passes rather than one two-dimensional kernel, which is what "separable"
/// means and is why a sigma of 10 costs 61 multiplies per pixel per axis instead of 3,721.
/// Samples outside the source are transparent black, exactly as the bilinear sampler treats
/// them, so a pixel near the edge is a weighted sum in which the missing neighbours contribute
/// nothing -- and because the weights are not renormalised for them, an edge fades out rather
/// than staying artificially bright.
pub(crate) fn blur(source: &mut WorkingBuffer, sigma_px: f64) -> usize {
    let radius = kernel_radius(sigma_px);
    if radius == 0 {
        return 0;
    }
    let weights = gaussian_weights(sigma_px);
    let (w, h) = (source.width(), source.height());

    // Horizontal, into a buffer wider by the radius on each side.
    let wide_w = w + 2 * radius;
    let mut wide = WorkingBuffer::transparent(wide_w, h);
    convolve(
        source.data(),
        w,
        wide.data_mut(),
        wide_w,
        radius,
        &weights,
        Axis::X,
    );

    // Vertical, into a buffer taller by the radius on each side.
    let tall_h = h + 2 * radius;
    let mut tall = WorkingBuffer::transparent(wide_w, tall_h);
    convolve(
        wide.data(),
        wide_w,
        tall.data_mut(),
        wide_w,
        radius,
        &weights,
        Axis::Y,
    );

    *source = tall;
    radius
}

/// D-109: [`blur`] with the edge pixels repeated. Each tap's column, and then its row, is held
/// inside the picture, so nothing outside it is read and the buffer keeps its size. The taps of
/// a pixel are added in [`convolve`]'s order.
///
/// P-20: a tap at a time along the whole row, as [`convolve`] does. Across, a tap reads one run
/// of the row shifted by it, with the pixels before and after the run held at the row's ends;
/// down, a tap reads one whole held row. Each pixel still starts at zero and takes its taps in
/// ascending order, so the bits are the pixel-at-a-time loop's.
pub(crate) fn held_blur(source: &mut WorkingBuffer, sigma_px: f64) {
    let r = kernel_radius(sigma_px);
    if r == 0 {
        return;
    }
    let weights = gaussian_weights(sigma_px);
    let (w, h) = (source.width(), source.height());
    let add = |out: &mut [f32], from: &[f32], weight: f32| {
        for (o, &v) in out.iter_mut().zip(from) {
            *o += v * weight;
        }
    };
    let mut wide = WorkingBuffer::transparent(w, h);
    wide.data_mut()
        .par_chunks_mut(w * 4)
        .zip(source.data().par_chunks(w * 4))
        .for_each(|(out, row)| {
            for (k, &weight) in weights.iter().enumerate() {
                // Pixel x reads x + k - r: before `lo` that is left of the row, from `hi` right of it.
                let lo = r.saturating_sub(k).min(w);
                let hi = (w + r).saturating_sub(k).clamp(lo, w);
                for px in out[..lo * 4].chunks_exact_mut(4) {
                    add(px, &row[..4], weight);
                }
                if hi > lo {
                    let from = (lo + k - r) * 4;
                    add(&mut out[lo * 4..hi * 4], &row[from..from + (hi - lo) * 4], weight);
                }
                for px in out[hi * 4..].chunks_exact_mut(4) {
                    add(px, &row[(w - 1) * 4..], weight);
                }
            }
        });
    let mut tall = WorkingBuffer::transparent(w, h);
    let src = wide.data();
    tall.data_mut()
        .par_chunks_mut(w * 4)
        .enumerate()
        .for_each(|(y, out)| {
            for (k, &weight) in weights.iter().enumerate() {
                let sy = (y + k).saturating_sub(r).min(h - 1);
                add(out, &src[sy * w * 4..(sy + 1) * w * 4], weight);
            }
        });
    *source = tall;
}

#[derive(Clone, Copy)]
enum Axis {
    X,
    Y,
}

/// One separable pass. `dst` is `src` grown by `radius` on both ends of `axis`.
///
/// **Across the thread pool, one destination row at a time (P-13).** A row of the destination
/// reads the source and writes nothing but itself, so the rows are independent whichever axis is
/// being filtered, and `par_chunks_mut` hands each one out without any of them being able to see
/// another. Nothing about the arithmetic moves: a destination pixel is still the same taps
/// accumulated in the same order into the same `acc`, so the result is the same bits on one
/// thread or on sixteen, which is what `verification/P-13_parallel_blur.md` compares rather than
/// asserts. `verification/P-01_frame_trace.md` is why the blur is the loop that got this: on the
/// declared ten-layer fixture with everything warm, the effect stack is 65.2% of a draft frame
/// and the tile loop beside it, already spread across this same pool, is 2.1%.
///
/// **A tap at a time along the whole row (P-16).** Destination pixel `x` takes source pixel
/// `x + k - 2 * radius` at tap `k`, so each tap is one weighted source run added onto one
/// destination run, which the compiler turns into wide instructions. Each pixel still starts at
/// zero and receives its taps in ascending `k`, so the bits are the ones the pixel-at-a-time loop
/// made, which `the_row_blur_is_the_pixel_blur` below holds to the old loop. A source run that is
/// all zeros is skipped: adding zero to a sum that started at +0 and never becomes -0 changes no
/// bit, and a character cel is mostly zeros.
fn convolve(
    src: &[f32],
    src_w: usize,
    dst: &mut [f32],
    dst_w: usize,
    radius: usize,
    weights: &[f32],
    axis: Axis,
) {
    let row = src_w * 4;
    let src_h = src.len() / row;
    // Each source row's shown extent, in floats: from its first nonzero sample's pixel to just
    // past its last's. NaN is nonzero, so it is carried as the old loop carried it.
    let extents: Vec<(usize, usize)> = src
        .par_chunks(row)
        .map(|s| {
            let first = s.iter().position(|&v| v != 0.0).map_or(row, |i| i / 4 * 4);
            let last = s.iter().rposition(|&v| v != 0.0).map_or(0, |i| i / 4 * 4 + 4);
            (first, last.max(first))
        })
        .collect();
    let span = 2 * radius;
    dst.par_chunks_mut(dst_w * 4)
        .enumerate()
        .for_each(|(y, out)| {
            for (k, &weight) in weights.iter().enumerate() {
                // The source row and where in `out` its pixel 0 lands.
                let (sy, at) = match axis {
                    Axis::X => (y, (span - k) * 4),
                    Axis::Y => match (y + k).checked_sub(span) {
                        Some(sy) if sy < src_h => (sy, 0),
                        _ => continue,
                    },
                };
                let (a, b) = extents[sy];
                let s = &src[sy * row + a..sy * row + b];
                for (o, &v) in out[at + a..at + b].iter_mut().zip(s) {
                    *o += v * weight;
                }
            }
        });
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The pixel-at-a-time pass P-13 left, kept only to hold the row pass to it.
    fn pixel_convolve(src: &[f32], src_w: usize, dst: &mut [f32], dst_w: usize, radius: usize, weights: &[f32], axis: Axis) {
        let src_h = src.len() / (src_w * 4);
        for (y, out) in dst.chunks_mut(dst_w * 4).enumerate() {
            for x in 0..dst_w {
                let mut acc = [0.0f32; 4];
                for (k, &weight) in weights.iter().enumerate() {
                    let offset = k as isize - radius as isize;
                    let (sx, sy) = match axis {
                        Axis::X => (x as isize - radius as isize + offset, y as isize),
                        Axis::Y => (x as isize, y as isize - radius as isize + offset),
                    };
                    if sx < 0 || sy < 0 || sx >= src_w as isize || sy >= src_h as isize {
                        continue;
                    }
                    let i = (sy as usize * src_w + sx as usize) * 4;
                    for c in 0..4 {
                        acc[c] += src[i + c] * weight;
                    }
                }
                out[x * 4..x * 4 + 4].copy_from_slice(&acc);
            }
        }
    }

    /// Every bit, over cels with empty rows, empty runs, a lone pixel at each edge, negative and
    /// huge values, and a buffer narrower than the kernel.
    #[test]
    fn the_row_blur_is_the_pixel_blur() {
        for (w, h, sigma) in [(37, 23, 1.3), (5, 4, 4.0), (64, 9, 0.2), (1, 1, 2.0), (40, 41, 7.7)] {
            let mut src = vec![0.0f32; w * h * 4];
            for (i, v) in src.iter_mut().enumerate() {
                let (x, y) = (i / 4 % w, i / 4 / w);
                *v = match (x * 7 + y * 13 + i % 4) % 11 {
                    _ if y % 5 == 2 || (x > w / 3 && x < w / 2) => 0.0,
                    0 => -0.75,
                    1 => 3.0e6,
                    n => n as f32 / 9.0,
                };
            }
            src[0] = 0.5;
            let last = src.len() - 1;
            src[last] = 0.25;
            let weights = gaussian_weights(sigma);
            let r = kernel_radius(sigma);
            for axis in [Axis::X, Axis::Y] {
                let (dw, dh) = match axis {
                    Axis::X => (w + 2 * r, h),
                    Axis::Y => (w, h + 2 * r),
                };
                let (mut fast, mut slow) = (vec![0.0f32; dw * dh * 4], vec![0.0f32; dw * dh * 4]);
                convolve(&src, w, &mut fast, dw, r, &weights, axis);
                pixel_convolve(&src, w, &mut slow, dw, r, &weights, axis);
                let bits = |v: &[f32]| v.iter().map(|f| f.to_bits()).collect::<Vec<_>>();
                assert_eq!(bits(&fast), bits(&slow), "{w}x{h} at sigma {sigma}");
            }
        }
    }


    /// P-20: the pixel-at-a-time held blur D-109 shipped, kept only to hold the row version to it.
    fn pixel_held_blur(source: &WorkingBuffer, sigma_px: f64) -> WorkingBuffer {
        let (r, weights) = (kernel_radius(sigma_px), gaussian_weights(sigma_px));
        let (w, h) = (source.width(), source.height());
        let pass = |src: &WorkingBuffer, axis: Axis| {
            let mut dst = WorkingBuffer::transparent(w, h);
            for y in 0..h {
                for x in 0..w {
                    for (k, &weight) in weights.iter().enumerate() {
                        let (sx, sy) = match axis {
                            Axis::X => ((x + k).saturating_sub(r).min(w - 1), y),
                            Axis::Y => (x, (y + k).saturating_sub(r).min(h - 1)),
                        };
                        let (i, o) = ((sy * w + sx) * 4, (y * w + x) * 4);
                        for c in 0..4 {
                            dst.data_mut()[o + c] += src.data()[i + c] * weight;
                        }
                    }
                }
            }
            dst
        };
        pass(&pass(source, Axis::X), Axis::Y)
    }

    /// Every bit, including buffers narrower and shorter than the kernel.
    #[test]
    fn the_row_held_blur_is_the_pixel_held_blur() {
        for (w, h, sigma) in [(37, 23, 1.3), (5, 4, 4.0), (64, 9, 0.2), (1, 1, 2.0), (40, 41, 7.7), (3, 50, 10.0)] {
            let mut src = WorkingBuffer::transparent(w, h);
            for (i, v) in src.data_mut().iter_mut().enumerate() {
                let (x, y) = (i / 4 % w, i / 4 / w);
                *v = match (x * 7 + y * 13 + i % 4) % 11 {
                    _ if y % 5 == 2 => 0.0,
                    0 => -0.75,
                    1 => 3.0e6,
                    n => n as f32 / 9.0,
                };
            }
            let slow = pixel_held_blur(&src, sigma);
            held_blur(&mut src, sigma);
            let bits = |b: &WorkingBuffer| b.data().iter().map(|f| f.to_bits()).collect::<Vec<_>>();
            assert_eq!(bits(&src), bits(&slow), "{w}x{h} at sigma {sigma}");
        }
    }
}
