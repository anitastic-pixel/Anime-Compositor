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
    /// D-202: how much of the effect's result is kept, 0 to 100 per cent, laid over what it was
    /// given. Its keys, when it has any, are `tracks["mix"]`.
    pub mix: f64,
}

impl EffectInstance {
    pub fn new(instance_id: Id, effect: Effect) -> Self {
        EffectInstance {
            instance_id,
            enabled: true,
            effect,
            tracks: BTreeMap::new(),
            mix: 100.0,
        }
    }

    /// D-202: every effect has a Mix but Posterize Time, which has no picture to mix, and one
    /// this build does not have. D-350: nor a text animator, which moves letters.
    pub fn has_mix(&self) -> bool {
        !matches!(self.effect, Effect::PosterizeTime { .. } | Effect::TextAnimator { .. } | Effect::Unsupported { .. })
    }

    /// [`Effect::arity`], with the Mix as a setting of one number.
    pub fn arity(&self, name: &str) -> Option<usize> {
        if name == "mix" {
            return self.has_mix().then_some(1);
        }
        self.effect.arity(name)
    }

    /// [`Effect::get`], with the Mix.
    pub fn get(&self, name: &str) -> Option<Vec<f64>> {
        if name == "mix" {
            return self.has_mix().then(|| vec![self.mix]);
        }
        self.effect.get(name)
    }

    /// [`Effect::set`], with the Mix.
    pub fn set(&mut self, name: &str, v: &[f64]) {
        match (name, v) {
            ("mix", [m]) => self.mix = *m,
            ("mix", _) => {}
            _ => self.effect.set(name, v),
        }
    }

    /// D-202: the first Mix, the constant's or a key's, outside 0 to 100; on Posterize Time, one
    /// that is not a plain 100 (NaN when it is 100 with keys).
    fn bad_mix(&self) -> Option<f64> {
        let keys: Vec<f64> = self.tracks.get("mix").map_or(Vec::new(), |t| {
            t[0].keyframes().iter().map(|k| k.value.as_scalar().unwrap_or(f64::NAN)).collect()
        });
        if matches!(self.effect, Effect::PosterizeTime { .. } | Effect::TextAnimator { .. }) {
            return (self.mix != 100.0 || !keys.is_empty())
                .then(|| keys.into_iter().chain([self.mix]).find(|m| *m != 100.0).unwrap_or(f64::NAN));
        }
        std::iter::once(self.mix).chain(keys).find(|m| !(0.0..=100.0).contains(m))
    }

    /// Whether this instance, as [`EffectInstance::at`] gives it, runs: its settings and its
    /// Mix inside their ranges (D-46, D-202).
    pub fn is_valid(&self) -> bool {
        self.effect.is_valid() && self.bad_mix().is_none()
    }

    /// Why this instance is bypassed on every frame, when it is: a setting or its Mix outside
    /// its range, the constant or a key.
    pub fn fault(&self) -> Option<String> {
        if let Some(bad) = self.invalid() {
            return Some(bad.why_invalid());
        }
        self.mix_fault()
    }

    /// D-202: why the Mix, the constant or a key, is outside its range, when it is.
    pub(crate) fn mix_fault(&self) -> Option<String> {
        let m = self.bad_mix()?;
        Some(if matches!(self.effect, Effect::PosterizeTime { .. } | Effect::TextAnimator { .. }) {
            // A NaN here is `bad_mix`'s mark for a Mix of 100 with keys.
            let m = if m.is_nan() { "keyed".to_string() } else { format!("{m}") };
            let what = if matches!(self.effect, Effect::TextAnimator { .. }) {
                "Text Animator moves the letters before there is a picture"
            } else {
                "Posterize Time holds its layer in time and has no picture to mix"
            };
            format!("{what}, so its Mix is a plain 100, and this is {m}.")
        } else {
            format!("Mix is 0 to 100 per cent, and this is {m}.")
        })
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
        let Some(count) = self.arity(name) else {
            return;
        };
        // D-291: an expression on the setting stays through any change of its keys.
        let expression = self.tracks.get(name).and_then(|t| t[0].expression().cloned());
        let mut track = vec![Property::constant(Value::Scalar(0.0)); count];
        track[0].set_expression(expression);
        if keys.is_empty() && track[0].expression().is_none() {
            self.tracks.remove(name);
            return;
        }
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

    /// D-291: the expression on the number setting `name`, or none. Its keys are kept, and a
    /// setting with neither keys nor an expression is constant again.
    pub(crate) fn set_expression(&mut self, name: &str, expression: Option<crate::model::Expression>) {
        let track = self
            .tracks
            .entry(name.to_string())
            .or_insert_with(|| vec![Property::constant(Value::Scalar(0.0))]);
        track[0].set_expression(expression);
        if track[0].keyframes().is_empty() && track[0].expression().is_none() {
            self.tracks.remove(name);
        }
    }

    /// D-291: one number setting of an instance already at its frame given an expression's
    /// value, held in the setting's range as a key's is.
    pub(crate) fn set_held(&mut self, name: &str, v: f64) {
        self.set(name, &[v]);
        for (_, slots, low, high) in self.effect.numbers() {
            for v in slots {
                *v = v.clamp(low, high);
            }
        }
    }

    /// D-291: the expression on the setting `name`, switched on or off.
    pub fn expression(&self, name: &str) -> Option<&crate::model::Expression> {
        self.tracks.get(name).and_then(|t| t[0].expression())
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
        self.at_time(frame, frame as f64)
    }

    /// D-216: [`EffectInstance::at`] with the keys read at the key time `u`, which a stretched
    /// layer's is; what moves with the frame number itself still reads `frame`.
    pub fn at_time(&self, frame: i32, u: f64) -> EffectInstance {
        let mut effect = self.effect.clone();
        if let Some(bad) = self.invalid() {
            effect = bad;
        } else {
            // D-291: a setting with an expression and no keys is its constant until the
            // expression is run, which needs the composition, in `expr::effect_at`.
            for (name, track) in self.tracks.iter().filter(|(_, t)| !t[0].keyframes().is_empty()) {
                let v: Vec<f64> = track
                    .iter()
                    .map(|p| p.value_at_time(u).as_scalar().unwrap_or(0.0))
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
            // D-204: the snow's frame.
            if let Effect::Snowfall { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-206: the bites' frame.
            if let Effect::RoughenEdges { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-186: the twinkle's frame.
            if let Effect::KiraKira { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-190: the bolt's frame.
            if let Effect::LightningBolt { frame: f, .. } = &mut effect {
                *f = frame;
            }
            // D-200: the waves' frame.
            if let Effect::RadioWaves { frame: f, .. } = &mut effect {
                *f = frame;
            }
        }
        // D-202: the Mix at this frame, held inside 0 to 100; one outside it is kept, so the
        // effect is bypassed.
        let mix = self.bad_mix().unwrap_or_else(|| {
            let keyed = self.tracks.get("mix").and_then(|t| t[0].value_at(frame).as_scalar());
            keyed.unwrap_or(self.mix).clamp(0.0, 100.0)
        });
        EffectInstance {
            instance_id: self.instance_id.clone(),
            enabled: self.enabled,
            effect,
            tracks: BTreeMap::new(),
            mix,
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
    /// alpha is unchanged." D-335: After Effects' Offset and Gamma Correction, and `bypass`, "off"
    /// or "on", its Bypass Linear Light Conversion.
    Exposure { stops: f64, offset: f64, gamma: f64, bypass: String },
    /// Document 21: "parameter `sigma_px >= 0` ... kernel radius `ceil(3*sigma_px)`."
    /// D-109: `edges`, "transparent" or "repeat", as Directional and Radial Blur have.
    /// D-303: `dimensions`, "both", "horizontal" or "vertical", the axes it blurs along.
    /// D-321: `units`, "sigma" (`sigma_px` is the sigma) or "blurriness" (After Effects'
    /// Blurriness, [`blur_reach`]).
    GaussianBlur { sigma_px: f64, edges: String, dimensions: String, units: String },
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
    /// D-322: `units`, `"classic"` (D-89's rule, what a file without it means) or
    /// `"after_effects"` (After Effects' radius and strength, which a new one takes).
    Glow {
        based_on: String,
        threshold: f64,
        colors: Vec<String>,
        tolerance: f64,
        radius: f64,
        intensity: f64,
        operation: String,
        tint: String,
        units: String,
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
    /// D-302: and `alpha`, the covering's curve, straight unless set.
    Curves {
        master: Vec<Vec<f64>>,
        red: Vec<Vec<f64>>,
        green: Vec<Vec<f64>>,
        blue: Vec<Vec<f64>>,
        alpha: Vec<Vec<f64>>,
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
    /// below 0. D-307: `ranges`, After Effects' Reds, Yellows, Greens, Cyans, Blues and Magentas
    /// in that order ([`HUE_RANGES`]), each a hue, saturation and lightness added to the three
    /// above where a pixel's hue lies in that range.
    HueSaturation {
        hue: f64,
        saturation: f64,
        lightness: f64,
        ranges: [[f64; 3]; 6],
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
    /// D-359, the blur map: `layer`, D-189's layer setting as written, "" for none; `fit`,
    /// `center` or `stretch`; `channel`, `luminance` or `alpha`; `focal_distance`, 0 to 255, the
    /// map value kept sharp; `invert`, `off` or `on`. `map` is not a setting and is never saved.
    LensBlur {
        radius: f64,
        edges: String,
        iris: String,
        roundness: f64,
        rotation: f64,
        aspect: f64,
        highlight_gain: f64,
        highlight_threshold: f64,
        layer: serde_json::Value,
        fit: String,
        channel: String,
        focal_distance: f64,
        invert: String,
        map: Option<crate::layer_map::Map>,
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
    /// setting and is never saved: it is the composition frame, as Noise's is. D-306:
    /// `displacement`, "turbulent", "horizontal" or "vertical", which ways a pixel is pushed;
    /// `pinning`, "none" or "all", whether the push fades out at the layer's edges. D-328:
    /// `units`, "classic" (D-127's push, what a file without it means) or "after_effects"
    /// (`turbulent_push`).
    TurbulentDisplace {
        amount: f64,
        size: f64,
        complexity: f64,
        evolution: f64,
        speed: f64,
        seed: f64,
        edges: String,
        frame: i32,
        displacement: String,
        pinning: String,
        units: String,
    },
    /// D-128: `size`, 1 to 1000 pixels a cloud; `complexity`, 1 to 20 (D-318), its whole part
    /// counted; `contrast`, 0 to 1000; `brightness`, -1000 to 1000 (D-318, D-326); `evolution`, -100000 to 100000
    /// degrees; `speed`, -360 to 360 degrees a frame; `seed`, 0 to 100000, its whole part
    /// counted; `dark_color` and `light_color`, `#rrggbb`; `opacity`, 0 to 100; and `blend`,
    /// "normal", "multiply", "screen" or "add". The words and the colours are kept as written,
    /// so a wrong one is reported. `frame` is not a setting and is never saved: it is the
    /// composition frame, as Noise's is. D-299, each kept as written and drawing as before at
    /// its start: `fractal_type`, "basic" or "turbulent"; `noise_type`, "smooth" or "block";
    /// `invert`, "off" or "on"; `offset`, x then y in pixels, -100000 to 100000; `scale_width`
    /// and `scale_height`, 1 to 10000 per cent of the size, 100 at the start; and `cycle`, 0 to
    /// 1000 turns of evolution after which it repeats, its whole part counted, 0 for never.
    /// `float` is not a setting either and is never saved: D-319's working depth of the
    /// composition the frame is drawn in, set when the frame is planned.
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
        fractal_type: String,
        noise_type: String,
        invert: String,
        offset: [f64; 2],
        scale_width: f64,
        scale_height: f64,
        cycle: f64,
        frame: i32,
        float: bool,
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
    /// D-295: `preserve_luminosity`, "off" or "on", kept as written; a file from before it is
    /// "off".
    ColorBalance {
        shadows: Vec<f64>,
        midtones: Vec<f64>,
        highlights: Vec<f64>,
        preserve_luminosity: String,
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
    Sharpen { amount: f64, radius: f64, threshold: f64 },
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
    /// and a pinch below. D-310: `vertical_radius`, 0 to 10000 pixels, the reach up and down,
    /// 0 following `radius`; `taper_radius`, 0 to 10000 pixels, how far in from the edge the
    /// swell fades to nothing, 0 keeping D-152's rule.
    Bulge {
        center: [f64; 2],
        radius: f64,
        height: f64,
        vertical_radius: f64,
        taper_radius: f64,
    },
    /// D-153: `center`, per cent of the drawing's width and height, -1000 to 1000 each, a point
    /// on the line; `angle`, -3600 to 3600 degrees, the line's turn from straight up and down.
    Mirror { center: [f64; 2], angle: f64 },
    /// D-154: `output_width` and `output_height`, 100 to 1000 per cent of the drawing's width
    /// and height; `mirror`, "off" or "on", every other tile turned over. D-304: `tile_center`,
    /// per cent of the drawing's width and height, -1000 to 1000 each, where a tile sits;
    /// `tile_width` and `tile_height`, 1 to 1000 per cent, each tile's size.
    MotionTile {
        output_width: f64,
        output_height: f64,
        mirror: String,
        tile_center: [f64; 2],
        tile_width: f64,
        tile_height: f64,
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
    /// D-185: `color`, `#rrggbb`; `direction`, 0 to 360 degrees clockwise from up, the way the
    /// wash comes from; `spread`, 0 to 100 per cent of the figure; `opacity`, 0 to 100; and
    /// `blend`, "normal", "multiply", "screen", "add", "overlay" or "soft_light".
    Paraffin {
        color: String,
        direction: f64,
        spread: f64,
        opacity: f64,
        blend: String,
    },
    /// D-186: `threshold`, 0 to 100; `spacing`, 2 to 1000 pixels; `density`, 0 to 100; `size`,
    /// 0 to 1000 pixels; `shape`, "cross" or "star"; `angle`, -3600 to 3600 degrees; `twinkle`,
    /// 0 to 100; `period`, 1 to 1000 frames; `seed`, 0 to 100000, its whole part counted;
    /// `opacity`, 0 to 100; and `color`, `#rrggbb`. `frame` is not a setting and is never
    /// saved: it is the composition frame, as Rain's is.
    KiraKira {
        threshold: f64,
        spacing: f64,
        density: f64,
        size: f64,
        shape: String,
        angle: f64,
        twinkle: f64,
        period: f64,
        seed: f64,
        opacity: f64,
        color: String,
        frame: i32,
    },
    /// D-190: `start` and `end`, per cent of the drawing's width and height, each -1000 to 1000;
    /// `jagged`, `branches` and `opacity`, 0 to 100; `detail`, 1 to 8; `width`, 0 to 100 pixels;
    /// `glow`, 0 to 500 pixels; `hold`, 1 to 100 frames; `seed`, 0 to 100000; detail, hold and
    /// seed by their whole parts; and `color` and `glow_color`, `#rrggbb`. D-300: `composite`,
    /// "on" or "off", After Effects' Composite on Original; a file from before it is "on", and
    /// "off" draws the bolt alone on a clear layer. `frame` is not a setting and is never saved:
    /// it is the composition frame, as Kira-kira's is. D-324, Advanced Lightning's: `kind`, the
    /// lightning type, one of "direction" (a file from before it), "strike", "breaking",
    /// "bouncy", "omni", "anywhere", "vertical" or "two_way"; `turbulence`, `decay` and
    /// `obstacle` (Alpha Obstacle), 0 to 100; and `conductivity`, 0 to 10000. Each at 0 draws
    /// D-190's bolt. D-329: `obstacle` -100 to 100, below 0 keeping the bolt inside what is
    /// solid; and `path`, "split" (a file from before it) or "around", going round obstacles.
    /// D-334: `core`, "hard" (a file from before it) or "soft", Advanced Lightning's core that
    /// fades from the middle of the bolt to its edge. D-338: `forks`, "short" (a file from before
    /// it) or "long", the main bolt's first forks running on down to the end. D-339: or "full",
    /// those long forks as wide as the main bolt where they leave it.
    LightningBolt {
        start: [f64; 2],
        end: [f64; 2],
        jagged: f64,
        detail: f64,
        branches: f64,
        width: f64,
        glow: f64,
        opacity: f64,
        hold: f64,
        seed: f64,
        color: String,
        glow_color: String,
        composite: String,
        kind: String,
        turbulence: f64,
        decay: f64,
        conductivity: f64,
        obstacle: f64,
        path: String,
        core: String,
        forks: String,
        frame: i32,
    },
    /// D-191: `layer`, D-189's layer setting as written (a word, or kept as found and refused
    /// when it is not one); `fit`, `center`, `stretch` or `tile`; `max_blur`, 0 to 500 pixels;
    /// `invert`, `off` or `on`; and `edges`, `transparent` or `repeat`. `map` is not a setting
    /// and is never saved: compose reads it for each frame.
    CompoundBlur {
        layer: serde_json::Value,
        fit: String,
        max_blur: f64,
        invert: String,
        edges: String,
        map: Option<crate::layer_map::Map>,
    },
    /// D-193: `layer` and `fit`, D-189's layer setting as Compound Blur has them;
    /// `horizontal` and `vertical`, which of the map's channels moves the picture across and
    /// down; `max_horizontal` and `max_vertical`, -1000 to 1000 pixels, how far; `wrap`,
    /// `off` or `on`; and D-315's `expand`, `off` or `on`. `map` is not a setting and is never
    /// saved: compose reads it for each frame.
    DisplacementMap {
        layer: serde_json::Value,
        fit: String,
        horizontal: String,
        max_horizontal: f64,
        vertical: String,
        max_vertical: f64,
        wrap: String,
        expand: String,
        map: Option<crate::layer_map::Map>,
    },
    /// D-194: `layer` and `fit`, D-189's layer setting as Compound Blur has them; `completion`
    /// and `softness`, 0 to 100; and `invert`, `off` or `on`. `map` is not a setting and is never
    /// saved: compose reads it for each frame.
    GradientWipe {
        layer: serde_json::Value,
        fit: String,
        completion: f64,
        softness: f64,
        invert: String,
        map: Option<crate::layer_map::Map>,
    },
    /// D-195: `echo_time`, -120 to 120 frames, and `echoes`, 0 to 30, each taken whole below;
    /// `intensity` and `decay`, 0 to 1; and `operator`, one of [`crate::layer_fx::ECHO_OPERATORS`].
    /// `picture` is not a setting and is never saved: compose draws the echoes into it for each
    /// frame.
    Echo {
        echo_time: f64,
        echoes: f64,
        intensity: f64,
        decay: f64,
        operator: String,
        picture: Option<crate::layer_map::Map>,
    },
    /// D-196: `frame_rate`, 0.1 to 99 frames a second. Compose takes the layer's content at the
    /// frame it holds (`crate::compose::posterized`); its own step in the stack changes nothing.
    PosterizeTime { frame_rate: f64 },
    /// D-197: `from` and `to`, `#rrggbb`; `change`, `hue`, `hue_lightness`, `hue_saturation` or
    /// `hue_lightness_saturation`, what of `to` is taken; `change_by`, `setting` or
    /// `transforming`; the three tolerances and `softness`, 0 to 100; `view_matte`, `off` or `on`.
    ChangeToColor {
        from: String,
        to: String,
        change: String,
        change_by: String,
        hue_tolerance: f64,
        lightness_tolerance: f64,
        saturation_tolerance: f64,
        softness: f64,
        view_matte: String,
    },
    /// D-198: `upper_left`, `upper_right`, `lower_left` and `lower_right`, each x then y, -400
    /// to 500 per cent of the drawing's width and height, where its corners are pinned.
    CornerPin {
        upper_left: [f64; 2],
        upper_right: [f64; 2],
        lower_left: [f64; 2],
        lower_right: [f64; 2],
    },
    /// D-199: `center`, x then y, -1000 to 1000 per cent of the drawing's width and height, where
    /// the band's middle line passes; `direction`, -3600 to 3600 degrees clockwise from up, the
    /// way that line runs; `shape`, "linear", "smooth" or "sharp"; `width`, 0 to 10000 pixels;
    /// `sweep_intensity` and `edge_intensity`, 0 to 100; `edge_thickness`, 1 to 50 pixels;
    /// `light_color`, `#rrggbb`; and `light_reception`, "add", "composite" or "cutout". The
    /// words and colour are kept as written, so a wrong one is reported.
    LightSweep {
        center: [f64; 2],
        direction: f64,
        shape: String,
        width: f64,
        sweep_intensity: f64,
        edge_intensity: f64,
        edge_thickness: f64,
        light_color: String,
        light_reception: String,
    },
    /// D-200: `producer_point`, x then y, -1000 to 1000 per cent of the drawing's width and
    /// height, where the waves start; `sides`, 3 to 64, its whole part counted; `interval` and
    /// `lifespan`, 1 to 1000 frames; `expansion` and `velocity`, 0 to 1000 pixels a frame;
    /// `orientation` and `direction`, -3600 to 3600 degrees; `spin`, -360 to 360 degrees a
    /// frame; `opacity`, 0 to 100; `fade_in_time` and `fade_out_time`, 0 to 1000 frames;
    /// `start_width` and `end_width`, 0 to 1000 pixels; `profile`, "square", "triangle" or
    /// "sine"; and `color`, `#rrggbb`. The words and colour are kept as written, so a wrong one
    /// is reported. `frame` is not a setting and is never saved: it is the composition frame, as
    /// Lightning Bolt's is.
    RadioWaves {
        producer_point: [f64; 2],
        sides: f64,
        interval: f64,
        expansion: f64,
        orientation: f64,
        direction: f64,
        velocity: f64,
        spin: f64,
        lifespan: f64,
        opacity: f64,
        fade_in_time: f64,
        fade_out_time: f64,
        start_width: f64,
        end_width: f64,
        profile: String,
        color: String,
        frame: i32,
    },
    /// D-201: `interpolation`, 0 to 100, how far the drawing is bent; and `conversion`,
    /// "rect_to_polar" or "polar_to_rect". The word is kept as written, so a wrong one is
    /// reported. D-320: `shape`, "ellipse", the one that touches the drawing's sides (a file
    /// without it), or "circle", round the middle, half the shorter side across.
    PolarCoordinates { interpolation: f64, conversion: String, shape: String },
    /// D-203: `radius`, 0 to 10 pixels; and `operate_on_alpha`, "off" or "on". The word is kept
    /// as written, so a wrong one is reported.
    Median { radius: f64, operate_on_alpha: String },
    /// D-203: `radius`, 0 to 10 pixels; and `threshold`, 0 to 255 8-bit steps.
    SmartBlur { radius: f64, threshold: f64 },
    /// D-358: `radius`, 0 to 50 pixels; `threshold`, 0 to 255 steps of the sRGB curve; and
    /// `colorize`, "on" or "off". The word is kept as written, so a wrong one is reported.
    BilateralBlur { radius: f64, threshold: f64, colorize: String },
    /// D-360: after CycoreFX's CC Cross Blur. The layer blurred across by one box of Fast Box
    /// Blur's `radius_x` and, apart, down by one of `radius_y`, each 0 to 500 pixels, laid
    /// together by `mode`, one of [`CROSS_MODES`]; `edges` as Gaussian Blur's. The words are kept
    /// as written, so a wrong one is reported.
    CrossBlur { radius_x: f64, radius_y: f64, mode: String, edges: String },
    /// D-361: after CycoreFX's CC Radial Blur. `kind`, the file's `type`, one of
    /// [`SPIN_ZOOM_TYPES`]; `amount`, -360 to 360, degrees for the turns or per cent of the
    /// distance for the zooms; `quality`, 1 to 100, samples a pixel of path by 50; and `center`
    /// as Radial Blur's (D-95).
    SpinZoomBlur { kind: String, amount: f64, quality: f64, center: [f64; 2] },
    /// D-362: after CycoreFX's CC Radial Fast Blur. `amount`, 0 to 100 per cent of the
    /// distance; `center` as Radial Blur's (D-95); and `zoom`, "standard", "brightest" or
    /// "darkest".
    FastZoomBlur { amount: f64, center: [f64; 2], zoom: String },
    /// D-204: `color`, `#rrggbb`, kept as written so a wrong one is reported; `density`, 0 to
    /// 100; `spacing`, 2 to 1000 pixels; `size`, 0 to 100 pixels; `depth`, 0 to 100; `speed`,
    /// 0 to 1000, and `wind`, -1000 to 1000, pixels a frame; `wiggle`, 0 to 100 pixels;
    /// `period`, 1 to 1000 frames; `seed`, 0 to 100000, its whole part counted; and `opacity`,
    /// 0 to 100. `frame` is not a setting and is never saved: it is the composition frame, as
    /// Rain's is.
    Snowfall {
        color: String,
        density: f64,
        spacing: f64,
        size: f64,
        depth: f64,
        speed: f64,
        wind: f64,
        wiggle: f64,
        period: f64,
        seed: f64,
        opacity: f64,
        frame: i32,
    },
    /// D-205: `segments`, 2 to 32, its whole part counted; `rotation`, -3600 to 3600 degrees;
    /// `size`, 10 to 1000 per cent; `center`, x then y, -1000 to 1000 per cent of the drawing;
    /// and `mode`, "mirror" or "repeat". The word is kept as written, so a wrong one is
    /// reported.
    Kaleidoscope {
        segments: f64,
        rotation: f64,
        size: f64,
        center: [f64; 2],
        mode: String,
    },
    /// D-206: `edge_type`, "roughen" or "roughen_color", and `edge_color`, `#rrggbb`, kept as
    /// written so a wrong one is reported; `border`, 0 to 500 pixels; `size`, 1 to 1000 pixels;
    /// `complexity`, 1 to 10, and `seed`, 0 to 100000, their whole parts counted; `evolution`,
    /// -100000 to 100000 degrees; and `speed`, -360 to 360 degrees a frame. `frame` is not a
    /// setting and is never saved: it is the composition frame, as Turbulent Displace's is.
    RoughenEdges {
        edge_type: String,
        edge_color: String,
        border: f64,
        size: f64,
        complexity: f64,
        evolution: f64,
        speed: f64,
        seed: f64,
        frame: i32,
    },
    /// D-207: `start` and `end`, per cent of the drawing's width and height, each -1000 to 1000;
    /// `length`, `time` and `softness`, 0 to 100; `start_thickness` and `end_thickness`, 0 to 500
    /// pixels; `inside_color` and `outside_color`, `#rrggbb`; and `composite`, "on" or "off",
    /// kept as written so a wrong one is reported.
    Beam {
        start: [f64; 2],
        end: [f64; 2],
        length: f64,
        time: f64,
        start_thickness: f64,
        end_thickness: f64,
        softness: f64,
        inside_color: String,
        outside_color: String,
        composite: String,
    },
    /// D-208: `point_1` to `point_4`, per cent of the drawing's width and height, each -1000 to
    /// 1000; `color_1` to `color_4`, `#rrggbb`; `blend`, 1 to 1000; `opacity`, 0 to 100; and
    /// `blending_mode`, "normal", "multiply", "screen" or "add", kept as written so a wrong one is
    /// reported.
    FourColorGradient {
        point_1: [f64; 2],
        point_2: [f64; 2],
        point_3: [f64; 2],
        point_4: [f64; 2],
        color_1: String,
        color_2: String,
        color_3: String,
        color_4: String,
        blend: f64,
        opacity: f64,
        blending_mode: String,
    },
    /// D-209: `pattern`, "bubbles", "crystals", "plates" or "static_plates", and `invert`, "on"
    /// or "off", kept as written so a wrong one is reported; `contrast`, 0 to 1000; `disperse`, 0
    /// to 1.5; `size`, pixels a cell, 1 to 1000; `evolution`, degrees, -100000 to 100000; `seed`,
    /// 0 to 100000, its whole part counted; `dark_color` and `light_color`, `#rrggbb`; `opacity`,
    /// 0 to 100; and `blend`, Fractal Noise's four words.
    CellPattern {
        pattern: String,
        invert: String,
        contrast: f64,
        disperse: f64,
        size: f64,
        evolution: f64,
        seed: f64,
        dark_color: String,
        light_color: String,
        opacity: f64,
        blend: String,
    },
    /// D-210: `field_of_view`, 0 to 180 degrees; `reverse`, "off" or "on", and `orientation`,
    /// "horizontal", "vertical" or "diagonal", kept as written so a wrong one is reported; and
    /// `center`, x then y, -1000 to 1000 per cent of the drawing.
    OpticsCompensation {
        field_of_view: f64,
        reverse: String,
        orientation: String,
        center: [f64; 2],
    },
    /// D-211: `color`, `#rrggbb`, kept as written so a wrong one is reported; `opacity`, 0 to
    /// 100; `light`, x then y, -1000 to 1000 per cent of the drawing; `distance`, 0 to 1000;
    /// `softness`, 0 to 500 pixels; `render`, "regular" or "glass_edge", and `shadow_only`, "off"
    /// or "on", kept as written; and `color_influence`, 0 to 100.
    RadialShadow {
        color: String,
        opacity: f64,
        light: [f64; 2],
        distance: f64,
        softness: f64,
        render: String,
        color_influence: f64,
        shadow_only: String,
    },
    /// D-212: `channel`, "luminance", "red", "green", "blue" or "alpha", and `invert`, "off" or
    /// "on", kept as written; `black_point`, `white_point`, `black_softness` and
    /// `white_softness`, each 0 to 255.
    Extract {
        channel: String,
        black_point: f64,
        white_point: f64,
        black_softness: f64,
        white_softness: f64,
        invert: String,
    },
    /// D-213: `edge_thickness`, 0 to 200 pixels; `light_angle`, -3600 to 3600 degrees clockwise
    /// from up; `light_color`, `#rrggbb`; `light_intensity`, 0 to 1.
    BevelAlpha {
        edge_thickness: f64,
        light_angle: f64,
        light_color: String,
        light_intensity: f64,
    },
    /// D-213: as Bevel Alpha, with `edge_thickness` 0 to 0.5 of the layer's smaller side.
    BevelEdges {
        edge_thickness: f64,
        light_angle: f64,
        light_color: String,
        light_intensity: f64,
    },
    /// D-214: `completion`, 0 to 100; `block_width` and `block_height`, 1 to 10000 pixels;
    /// `feather`, 0 to 10000 pixels.
    BlockDissolve {
        completion: f64,
        block_width: f64,
        block_height: f64,
        feather: f64,
    },
    /// D-305: where each channel comes from, After Effects' Take Alpha, Red, Green and Blue From:
    /// one of [`SHIFT_CHANNELS_FROM`].
    ShiftChannels {
        take_alpha: String,
        take_red: String,
        take_green: String,
        take_blue: String,
    },
    /// D-312: After Effects' Solid Composite. `source_opacity` and `opacity`, 0 to 100; `color`,
    /// `#rrggbb`; `blend`, "normal", "add", "screen" or "multiply". The layer, at its opacity, laid
    /// on a solid of the colour at its own.
    SolidComposite {
        source_opacity: f64,
        color: String,
        opacity: f64,
        blend: String,
    },
    /// D-313: After Effects' Channel Blur. Each channel's blur, 0 to 500, sigma in pixels as
    /// Blur's is; `edges` and `dimensions` as Blur's.
    ChannelBlur {
        red_blurriness: f64,
        green_blurriness: f64,
        blue_blurriness: f64,
        alpha_blurriness: f64,
        edges: String,
        dimensions: String,
    },
    /// D-327: After Effects' Fast Box Blur. A box `radius` pixels round each one, 0 to 500,
    /// laid on `iterations` times, 1 to 50, its whole part counted; `edges` and `dimensions`
    /// as Gaussian Blur's.
    FastBoxBlur {
        radius: f64,
        iterations: f64,
        edges: String,
        dimensions: String,
    },
    /// D-316: After Effects' Colorama, reduced. `get_phase`, one of [`COLORAMA_PHASES`];
    /// `layer` and `fit`, D-189's layer setting, whose brightness is added to the phase;
    /// `phase_shift`, -3600 to 3600 degrees; `cycle_repetitions`, 0 to 100; `stops`, 2 to 5,
    /// taken whole, how many of `color_1` to `color_5` (`#rrggbb`) make the ring; and
    /// `blend_with_original`, 0 to 100. `map` is not a setting and is never saved.
    Colorama {
        get_phase: String,
        layer: serde_json::Value,
        fit: String,
        phase_shift: f64,
        cycle_repetitions: f64,
        stops: f64,
        color_1: String,
        color_2: String,
        color_3: String,
        color_4: String,
        color_5: String,
        blend_with_original: f64,
        map: Option<crate::layer_map::Map>,
    },
    /// D-317: After Effects' CC Glass, reduced. `layer` and `fit`, D-189's layer setting, the
    /// bump map, "" the layer itself; `property`, one of [`COLORAMA_PHASES`]; `softness`, 0 to
    /// 100 pixels; `height`, -100 to 100; `displacement`, -500 to 500 pixels; `light_angle`,
    /// -3600 to 3600 degrees clockwise from up; `light_color`, `#rrggbb`; `light_intensity`, 0 to
    /// 100. `map` is not a setting and is never saved.
    Glass {
        layer: serde_json::Value,
        fit: String,
        property: String,
        softness: f64,
        height: f64,
        displacement: f64,
        light_angle: f64,
        light_color: String,
        light_intensity: f64,
        map: Option<crate::layer_map::Map>,
    },
    /// D-336: After Effects' CC Vector Blur, this program's own reading of CycoreFX's manual.
    /// `kind`, one of [`VECTOR_BLUR_TYPES`], saved as `type`; `amount`, 0 to 500 pixels;
    /// `angle_offset`, -3600 to 3600 degrees; `ridge_smoothness`, 0 to 100; `layer` and `fit`,
    /// D-189's layer setting, the vector map, "" the layer itself; `property`, one of
    /// [`VECTOR_BLUR_PROPERTIES`]; `map_softness`, 0 to 100 pixels. `map` is not a setting and is
    /// never saved.
    VectorBlur {
        kind: String,
        amount: f64,
        angle_offset: f64,
        ridge_smoothness: f64,
        layer: serde_json::Value,
        fit: String,
        property: String,
        map_softness: f64,
        map: Option<crate::layer_map::Map>,
    },
    /// D-347: Moment Map, after After Effects' Time Displacement: each pixel of the layer from
    /// another moment of it, later where the map is bright and earlier where it is dark.
    /// `max_time`, -10 to 10 seconds; `resolution`, 1 to 999 steps a second; `layer` and `fit`,
    /// D-189's layer setting, "" the layer itself. `map` and `picture` are not settings and are
    /// never saved: compose reads the map and draws the moments into `picture` for each frame.
    MomentMap {
        max_time: f64,
        resolution: f64,
        layer: serde_json::Value,
        fit: String,
        map: Option<crate::layer_map::Map>,
        picture: Option<crate::layer_map::Map>,
    },
    /// D-348: Pass Extract, after After Effects' 3D Channel Extract: the depth or the normals
    /// stored beside the colour in the layer's EXR file, as a picture. `pass`, "depth" or
    /// "normals" (D-349: or "object_id", "material_id", or "named" for the channel names in
    /// `channel`); `black_point` and `white_point`, -1,000,000 to 1,000,000; `invert` and
    /// `clamp`, "off" or "on". `channels` is not a setting and is never saved: compose reads the
    /// pass from the layer's file for each frame.
    PassExtract {
        pass: String,
        black_point: f64,
        white_point: f64,
        invert: String,
        clamp: String,
        channel: String,
        channels: Option<crate::layer_map::Map>,
    },
    /// D-349: ID Key, after After Effects' ID Matte: the pixels whose object or material id is
    /// `id` kept. `aux_channel`, "object_id" or "material_id"; `id`, 0 to 1,000,000; `feather`,
    /// 0 to 100 pixels; `invert`, "off" or "on". `channels` as Pass Extract's.
    IdKey {
        aux_channel: String,
        id: f64,
        feather: f64,
        invert: String,
        channels: Option<crate::layer_map::Map>,
    },
    /// D-350: Text Animator, after After Effects' text animators and their Range Selector: the
    /// letters of a text layer moved, scaled, turned, faded, recoloured and spaced one by one, each
    /// by how much the selector picks it (`text::lay_out`, document 21). `position`, x then y,
    /// -100,000 to 100,000 pixels; `scale`, x then y, -10,000 to 10,000 per cent; `rotation`,
    /// -36,000 to 36,000 degrees; `opacity`, 0 to 100 per cent; `fill`, "off" or "on", with
    /// `color`, linear 0 to 1; `tracking`, -1,000 to 1,000 thousandths of the size; `start` and
    /// `end`, 0 to 100 per cent; `offset`, -100 to 100; `amount`, -100 to 100; `based_on`,
    /// "characters", "characters_excluding_spaces" or "words"; `shape`, "square", "ramp_up",
    /// "ramp_down", "triangle", "round" or "smooth"; `smoothness`, `ease_high` and `ease_low`, 0
    /// to 100 per cent. It has no pixel work: compose hands it to the text's drawing.
    TextAnimator {
        position: [f64; 2],
        scale: [f64; 2],
        rotation: f64,
        opacity: f64,
        fill: String,
        color: [f64; 3],
        tracking: f64,
        start: f64,
        end: f64,
        offset: f64,
        amount: f64,
        based_on: String,
        shape: String,
        smoothness: f64,
        ease_high: f64,
        ease_low: f64,
    },
    /// D-348: Depth Key, after After Effects' Depth Matte: what is nearer than `depth` in the
    /// layer's EXR depth taken out. `depth`, -1,000,000 to 1,000,000; `feather`, 0 to 1,000,000,
    /// both in the depth's own units; `invert`, "off" or "on". `channels` as Pass Extract's.
    DepthKey {
        depth: f64,
        feather: f64,
        invert: String,
        channels: Option<crate::layer_map::Map>,
    },
    /// D-351: Stretch Levels, Stretch Contrast and Stretch Color, after After Effects' Auto
    /// Levels, Auto Contrast and Auto Color: the picture stretched by its own statistics
    /// (`frame_stats`, document 21). `kind`, "levels", "contrast" or "color", is which of the
    /// three, from its type id, never saved; `temporal_smoothing`, 0 to 10 seconds;
    /// `scene_detect` and `snap_neutral_midtones` (Stretch Color's only), "off" or "on";
    /// `black_clip` and `white_clip`, 0 to 10 per cent. `stats` is not a setting and is never
    /// saved: compose adds up the frames a temporal smoothing reads; `None` is this picture's own.
    AutoTone {
        kind: &'static str,
        temporal_smoothing: f64,
        scene_detect: String,
        black_clip: f64,
        white_clip: f64,
        snap_neutral_midtones: String,
        stats: Option<std::sync::Arc<crate::frame_stats::Stats>>,
    },
    /// D-351: Spread Tones, after After Effects' Equalize: each value moved to its place in the
    /// picture's histogram, so the tones spread evenly. `equalize`, "rgb", "brightness" or
    /// "photoshop"; `amount`, 0 to 100 per cent.
    SpreadTones { equalize: String, amount: f64 },
    /// D-352: Matte Choker, after After Effects' effect of that name (`matte_refine`, document
    /// 21): two stages, each a `geometric_softness` 0 to 100 pixels, a `choke` -127 to 127 and a
    /// `gray_level_softness` 0 to 100 per cent; stage 1 then stage 2, the pair repeated
    /// `iterations` times, 1 to 10, its whole part counted.
    MatteChoker {
        geometric_softness_1: f64,
        choke_1: f64,
        gray_level_softness_1: f64,
        geometric_softness_2: f64,
        choke_2: f64,
        gray_level_softness_2: f64,
        iterations: f64,
    },
    /// D-352: Refine Hard Matte and Refine Soft Matte, after After Effects' effects of those
    /// names (`matte_refine`, document 21). `kind`, "hard" or "soft", is which, from its type
    /// id, never saved. `edge_radius`, 0 to 100 pixels, and `view_edge_region`, "off" or "on",
    /// are Soft's only; `feather`, 0 to 100 pixels; `contrast`, 0 to 100 per cent;
    /// `shift_edge`, -100 to 100 per cent; `decontaminate` and `view_decontamination_map`, "off"
    /// or "on"; `decontamination_amount`, 0 to 100 per cent; `decontamination_radius`, 0 to 100
    /// pixels. The radii are counted in whole pixels.
    RefineMatte {
        kind: &'static str,
        edge_radius: f64,
        view_edge_region: String,
        feather: f64,
        contrast: f64,
        shift_edge: f64,
        decontaminate: String,
        decontamination_amount: f64,
        decontamination_radius: f64,
        view_decontamination_map: String,
    },
    /// D-353: Soft Physical Glow (EFFECTS.md P0-21 and pick #1), Glow's `falloff` "physical"
    /// (`soft_glow`, document 21); a Glow without `falloff`, or "classic", is [`Effect::Glow`].
    /// `falloff` is kept as written, so a wrong one is kept and reported. `threshold_mode`,
    /// "chroma" or "luminance"; `threshold`, `threshold_smooth` and `source_opacity` 0 to 100 per
    /// cent; `saturation_bias` -100 to 100 per cent; `radius` 0 to 2,000 pixels; `exposure` 0 to
    /// 100; `aspect_ratio` 0 to 2 and `aspect_angle` -3,600 to 3,600 degrees; `operation` "add"
    /// or "screen"; `unmult` "off" or "on".
    SoftGlow {
        falloff: String,
        threshold_mode: String,
        threshold: f64,
        threshold_smooth: f64,
        saturation_bias: f64,
        radius: f64,
        exposure: f64,
        aspect_ratio: f64,
        aspect_angle: f64,
        operation: String,
        source_opacity: f64,
        unmult: String,
    },
    /// D-356: Path Stroke, after After Effects' Stroke (`along`, document 21): a brush drawn
    /// along the layer's masks. `mask`, 1 to 1000, the mask numbered so, its floor taken;
    /// `all_masks` and `stroke_sequentially`, "off" or "on"; `color`, `#rrggbb`; `brush_size`, 0
    /// to 200 pixels; `brush_hardness`, `opacity`, `start`, `end` and `spacing`, 0 to 100 per
    /// cent; `paint_style`, "on_original", "on_transparent" or "reveal"; D-357: `source`, "masks"
    /// or "shapes", the layer's masks or its shapes (a shape layer's), `mask` and `all_masks`
    /// then choosing shapes. `paths` is not a setting and is never saved: compose fills it each
    /// frame with the paths chosen, flattened, each with whether it is closed, at the frame and
    /// the size the effects run at, `None` when there are none.
    Stroke {
        mask: f64,
        all_masks: String,
        stroke_sequentially: String,
        color: String,
        brush_size: f64,
        brush_hardness: f64,
        opacity: f64,
        start: f64,
        end: f64,
        spacing: f64,
        paint_style: String,
        source: String,
        paths: Option<Vec<(Vec<(f64, f64)>, bool)>>,
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
pub const PARAFFIN: &str = "core.paraffin";
pub const KIRA_KIRA: &str = "core.kira_kira";
pub const LIGHTNING_BOLT: &str = "core.lightning_bolt";
/// D-324: Lightning Bolt's lightning types.
pub const LIGHTNING_KINDS: [&str; 8] = ["direction", "strike", "breaking", "bouncy", "omni", "anywhere", "vertical", "two_way"];
pub const COMPOUND_BLUR: &str = "core.compound_blur";
pub const DISPLACEMENT_MAP: &str = "core.displacement_map";
pub const GRADIENT_WIPE: &str = "core.gradient_wipe";
pub const ECHO: &str = "core.echo";
pub const POSTERIZE_TIME: &str = "core.posterize_time";
pub const CHANGE_TO_COLOR: &str = "core.change_to_color";
pub const CORNER_PIN: &str = "core.corner_pin";
pub const LIGHT_SWEEP: &str = "core.light_sweep";
pub const RADIO_WAVES: &str = "core.radio_waves";
pub const POLAR_COORDINATES: &str = "core.polar_coordinates";
pub const MEDIAN: &str = "core.median";
pub const SMART_BLUR: &str = "core.smart_blur";
pub const BILATERAL_BLUR: &str = "core.bilateral_blur";
pub const CROSS_BLUR: &str = "core.cross_blur";
pub const SPIN_ZOOM_BLUR: &str = "core.spin_zoom_blur";
pub const FAST_ZOOM_BLUR: &str = "core.fast_zoom_blur";
/// D-360: Cross Blur's modes, in the order the card numbers them.
pub const CROSS_MODES: [&str; 6] = ["blend", "add", "screen", "multiply", "lighten", "darken"];
/// D-361: Spin & Zoom Blur's types.
pub const SPIN_ZOOM_TYPES: [&str; 6] = ["straight_zoom", "fading_zoom", "centered_zoom", "rotate", "scratch", "rotate_fading"];
pub const SNOWFALL: &str = "core.snowfall";
pub const KALEIDOSCOPE: &str = "core.kaleidoscope";
pub const ROUGHEN_EDGES: &str = "core.roughen_edges";
pub const BEAM: &str = "core.beam";
pub const FOUR_COLOR_GRADIENT: &str = "core.four_color_gradient";
pub const CELL_PATTERN: &str = "core.cell_pattern";
pub const OPTICS_COMPENSATION: &str = "core.optics_compensation";
pub const RADIAL_SHADOW: &str = "core.radial_shadow";
pub const EXTRACT: &str = "core.extract";
pub const BEVEL_ALPHA: &str = "core.bevel_alpha";
pub const BEVEL_EDGES: &str = "core.bevel_edges";
pub const BLOCK_DISSOLVE: &str = "core.block_dissolve";
pub const SHIFT_CHANNELS: &str = "core.shift_channels";
pub const SOLID_COMPOSITE: &str = "core.solid_composite";
pub const CHANNEL_BLUR: &str = "core.channel_blur";
pub const FAST_BOX_BLUR: &str = "core.fast_box_blur";
pub const COLORAMA: &str = "core.colorama";
/// D-316: what Colorama reads a pixel's phase from.
pub const COLORAMA_PHASES: [&str; 6] = ["intensity", "luminance", "red", "green", "blue", "alpha"];
pub const GLASS: &str = "core.glass";
pub const VECTOR_BLUR: &str = "core.vector_blur";
pub const MOMENT_MAP: &str = "core.moment_map";
pub const PASS_EXTRACT: &str = "core.pass_extract";
pub const DEPTH_KEY: &str = "core.depth_key";
pub const ID_KEY: &str = "core.id_key";
pub const TEXT_ANIMATOR: &str = "core.text_animator";
pub const STRETCH_LEVELS: &str = "core.stretch_levels";
pub const STRETCH_CONTRAST: &str = "core.stretch_contrast";
pub const STRETCH_COLOR: &str = "core.stretch_color";
pub const SPREAD_TONES: &str = "core.spread_tones";
pub const MATTE_CHOKER: &str = "core.matte_choker";
pub const REFINE_HARD_MATTE: &str = "core.refine_hard_matte";
pub const REFINE_SOFT_MATTE: &str = "core.refine_soft_matte";
pub const STROKE: &str = "core.stroke";
/// D-356: Path Stroke's paint styles.
pub const PAINT_STYLES: [&str; 3] = ["on_original", "on_transparent", "reveal"];
/// D-351: Spread Tones' ways.
pub const EQUALIZE: [&str; 3] = ["rgb", "brightness", "photoshop"];
/// D-350: what a text animator's selector counts, and the shapes of its range.
pub const TEXT_BASED_ON: [&str; 3] = ["characters", "characters_excluding_spaces", "words"];
pub const TEXT_SHAPES: [&str; 6] = ["square", "ramp_up", "ramp_down", "triangle", "round", "smooth"];
/// D-349: Pass Extract's passes.
pub const PASSES: [&str; 5] = ["depth", "normals", "object_id", "material_id", "named"];
/// D-336: CC Vector Blur's types.
pub const VECTOR_BLUR_TYPES: [&str; 5] = ["natural", "constant", "perpendicular", "direction_center", "direction_fading"];
/// D-336: what CC Vector Blur reads its height from.
pub const VECTOR_BLUR_PROPERTIES: [&str; 8] = ["red", "green", "blue", "alpha", "luminance", "lightness", "hue", "saturation"];
/// D-307: Hue/Saturation's colour ranges as the file names them, centred 0, 60 ... 300 degrees.
pub const HUE_RANGES: [&str; 6] = ["reds_hsl", "yellows_hsl", "greens_hsl", "cyans_hsl", "blues_hsl", "magentas_hsl"];
/// D-305: what Shift Channels can take a channel from.
pub const SHIFT_CHANNELS_FROM: [&str; 11] =
    ["alpha", "red", "green", "blue", "luminance", "hue", "lightness", "saturation", "full", "half", "off"];

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
    /// An Exposure of `stops` and nothing else.
    pub fn exposure(stops: f64) -> Effect {
        Effect::Exposure { stops, offset: 0.0, gamma: 1.0, bypass: "off".to_string() }
    }

    /// Every setting that is numbers: its name in the file, its numbers, and the range document
    /// 21 holds them to. The one table `arity`, `get`, `set`, `at` and the newer effects' range
    /// sentences read, so a setting is named in one place.
    fn numbers(&mut self) -> Vec<(&'static str, Vec<&mut f64>, f64, f64)> {
        match self {
            // D-90: a sigma past 500 a machine's memory. D-318: After Effects' Exposure runs
            // to 40 stops either way, and `2^40` is still a plain number in single precision.
            // D-335: After Effects' own ranges for the other two.
            Effect::Exposure { stops, offset, gamma, .. } => vec![
                ("stops", vec![stops], -40.0, 40.0),
                ("offset", vec![offset], -0.5, 0.5),
                ("gamma", vec![gamma], 0.01, 9.99),
            ],
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
                ranges,
            } => {
                let mut n = vec![
                    ("hue", vec![hue], -180.0, 180.0),
                    ("saturation", vec![saturation], -100.0, 100.0),
                    ("lightness", vec![lightness], -100.0, 100.0),
                ];
                // D-307: one range for all three; saturation and lightness are held to 100 where used.
                n.extend(HUE_RANGES.into_iter().zip(ranges.iter_mut()).map(|(k, r)| (k, r.iter_mut().collect(), -180.0, 180.0)));
                n
            }
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
                focal_distance,
                ..
            } => vec![
                ("radius", vec![radius], 0.0, 200.0),
                ("roundness", vec![roundness], 0.0, 100.0),
                ("rotation", vec![rotation], -3600.0, 3600.0),
                ("aspect", vec![aspect], 0.1, 10.0),
                ("highlight_gain", vec![highlight_gain], 0.0, 100.0),
                ("highlight_threshold", vec![highlight_threshold], 0.0, 100.0),
                ("focal_distance", vec![focal_distance], 0.0, 255.0),
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
                offset,
                scale_width,
                scale_height,
                cycle,
                ..
            } => vec![
                ("size", vec![size], 1.0, 1000.0),
                // D-318: After Effects' Complexity runs to 20; D-326: Brightness to 1000 either way, this program's choice.
                ("complexity", vec![complexity], 1.0, 20.0),
                ("contrast", vec![contrast], 0.0, 1000.0),
                ("brightness", vec![brightness], -1000.0, 1000.0),
                ("evolution", vec![evolution], -100000.0, 100000.0),
                ("speed", vec![speed], -360.0, 360.0),
                ("seed", vec![seed], 0.0, 100000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
                ("offset", offset.iter_mut().collect(), -100000.0, 100000.0),
                ("scale_width", vec![scale_width], 1.0, 10000.0),
                ("scale_height", vec![scale_height], 1.0, 10000.0),
                ("cycle", vec![cycle], 0.0, 1000.0),
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
                ..
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
            Effect::Sharpen { amount, radius, threshold } => vec![
                ("amount", vec![amount], 0.0, 500.0),
                ("radius", vec![radius], 0.0, 100.0),
                ("threshold", vec![threshold], 0.0, 255.0),
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
                vertical_radius,
                taper_radius,
            } => vec![
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("radius", vec![radius], 0.0, 10000.0),
                ("height", vec![height], -4.0, 4.0),
                ("vertical_radius", vec![vertical_radius], 0.0, 10000.0),
                ("taper_radius", vec![taper_radius], 0.0, 10000.0),
            ],
            Effect::Mirror { center, angle } => vec![
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("angle", vec![angle], -3600.0, 3600.0),
            ],
            Effect::MotionTile {
                output_width,
                output_height,
                tile_center,
                tile_width,
                tile_height,
                ..
            } => vec![
                ("output_width", vec![output_width], 100.0, 1000.0),
                ("output_height", vec![output_height], 100.0, 1000.0),
                ("tile_center", tile_center.iter_mut().collect(), -1000.0, 1000.0),
                ("tile_width", vec![tile_width], 1.0, 1000.0),
                ("tile_height", vec![tile_height], 1.0, 1000.0),
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
            Effect::ShiftChannels { .. } => vec![],
            Effect::SolidComposite { source_opacity, opacity, .. } => vec![
                ("source_opacity", vec![source_opacity], 0.0, 100.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::ChannelBlur {
                red_blurriness,
                green_blurriness,
                blue_blurriness,
                alpha_blurriness,
                ..
            } => vec![
                ("red_blurriness", vec![red_blurriness], 0.0, 500.0),
                ("green_blurriness", vec![green_blurriness], 0.0, 500.0),
                ("blue_blurriness", vec![blue_blurriness], 0.0, 500.0),
                ("alpha_blurriness", vec![alpha_blurriness], 0.0, 500.0),
            ],
            Effect::FastBoxBlur { radius, iterations, .. } => vec![
                ("radius", vec![radius], 0.0, 500.0),
                ("iterations", vec![iterations], 1.0, 50.0),
            ],
            Effect::Colorama {
                phase_shift,
                cycle_repetitions,
                stops,
                blend_with_original,
                ..
            } => vec![
                ("phase_shift", vec![phase_shift], -3600.0, 3600.0),
                ("cycle_repetitions", vec![cycle_repetitions], 0.0, 100.0),
                ("stops", vec![stops], 2.0, 5.0),
                ("blend_with_original", vec![blend_with_original], 0.0, 100.0),
            ],
            Effect::Glass {
                softness,
                height,
                displacement,
                light_angle,
                light_intensity,
                ..
            } => vec![
                ("softness", vec![softness], 0.0, 100.0),
                ("height", vec![height], -100.0, 100.0),
                ("displacement", vec![displacement], -500.0, 500.0),
                ("light_angle", vec![light_angle], -3600.0, 3600.0),
                ("light_intensity", vec![light_intensity], 0.0, 100.0),
            ],
            Effect::VectorBlur { amount, angle_offset, ridge_smoothness, map_softness, .. } => vec![
                ("amount", vec![amount], 0.0, 500.0),
                ("angle_offset", vec![angle_offset], -3600.0, 3600.0),
                ("ridge_smoothness", vec![ridge_smoothness], 0.0, 100.0),
                ("map_softness", vec![map_softness], 0.0, 100.0),
            ],
            Effect::MomentMap { max_time, resolution, .. } => vec![
                ("max_time", vec![max_time], -10.0, 10.0),
                ("resolution", vec![resolution], 1.0, 999.0),
            ],
            // D-348: a depth is in whatever units the program that rendered it chose.
            Effect::PassExtract { black_point, white_point, .. } => vec![
                ("black_point", vec![black_point], -1e6, 1e6),
                ("white_point", vec![white_point], -1e6, 1e6),
            ],
            Effect::DepthKey { depth, feather, .. } => vec![
                ("depth", vec![depth], -1e6, 1e6),
                ("feather", vec![feather], 0.0, 1e6),
            ],
            Effect::IdKey { id, feather, .. } => vec![
                ("id", vec![id], 0.0, 1e6),
                ("feather", vec![feather], 0.0, 100.0),
            ],
            Effect::AutoTone { temporal_smoothing, black_clip, white_clip, .. } => vec![
                ("temporal_smoothing", vec![temporal_smoothing], 0.0, 10.0),
                ("black_clip", vec![black_clip], 0.0, 10.0),
                ("white_clip", vec![white_clip], 0.0, 10.0),
            ],
            Effect::SpreadTones { amount, .. } => vec![("amount", vec![amount], 0.0, 100.0)],
            Effect::MatteChoker {
                geometric_softness_1,
                choke_1,
                gray_level_softness_1,
                geometric_softness_2,
                choke_2,
                gray_level_softness_2,
                iterations,
            } => vec![
                ("geometric_softness_1", vec![geometric_softness_1], 0.0, 100.0),
                ("choke_1", vec![choke_1], -127.0, 127.0),
                ("gray_level_softness_1", vec![gray_level_softness_1], 0.0, 100.0),
                ("geometric_softness_2", vec![geometric_softness_2], 0.0, 100.0),
                ("choke_2", vec![choke_2], -127.0, 127.0),
                ("gray_level_softness_2", vec![gray_level_softness_2], 0.0, 100.0),
                ("iterations", vec![iterations], 1.0, 10.0),
            ],
            Effect::RefineMatte {
                kind,
                edge_radius,
                feather,
                contrast,
                shift_edge,
                decontamination_amount,
                decontamination_radius,
                ..
            } => {
                let mut v = vec![
                    ("feather", vec![feather], 0.0, 100.0),
                    ("contrast", vec![contrast], 0.0, 100.0),
                    ("shift_edge", vec![shift_edge], -100.0, 100.0),
                    ("decontamination_amount", vec![decontamination_amount], 0.0, 100.0),
                    ("decontamination_radius", vec![decontamination_radius], 0.0, 100.0),
                ];
                if *kind == "soft" {
                    v.insert(0, ("edge_radius", vec![edge_radius], 0.0, 100.0));
                }
                v
            }
            Effect::SoftGlow {
                threshold,
                threshold_smooth,
                saturation_bias,
                radius,
                exposure,
                aspect_ratio,
                aspect_angle,
                source_opacity,
                ..
            } => vec![
                ("threshold", vec![threshold], 0.0, 100.0),
                ("threshold_smooth", vec![threshold_smooth], 0.0, 100.0),
                ("saturation_bias", vec![saturation_bias], -100.0, 100.0),
                ("radius", vec![radius], 0.0, 2000.0),
                ("exposure", vec![exposure], 0.0, 100.0),
                ("aspect_ratio", vec![aspect_ratio], 0.0, 2.0),
                ("aspect_angle", vec![aspect_angle], -3600.0, 3600.0),
                ("source_opacity", vec![source_opacity], 0.0, 100.0),
            ],
            Effect::Stroke { mask, brush_size, brush_hardness, opacity, start, end, spacing, .. } => vec![
                ("mask", vec![mask], 1.0, 1000.0),
                ("brush_size", vec![brush_size], 0.0, 200.0),
                ("brush_hardness", vec![brush_hardness], 0.0, 100.0),
                ("opacity", vec![opacity], 0.0, 100.0),
                ("start", vec![start], 0.0, 100.0),
                ("end", vec![end], 0.0, 100.0),
                ("spacing", vec![spacing], 0.0, 100.0),
            ],
            Effect::TextAnimator {
                position, scale, rotation, opacity, color, tracking, start, end, offset, amount, smoothness, ease_high, ease_low, ..
            } => vec![
                ("position", position.iter_mut().collect(), -1e5, 1e5),
                ("scale", scale.iter_mut().collect(), -1e4, 1e4),
                ("rotation", vec![rotation], -36000.0, 36000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
                ("color", color.iter_mut().collect(), 0.0, 1.0),
                ("tracking", vec![tracking], -1000.0, 1000.0),
                ("start", vec![start], 0.0, 100.0),
                ("end", vec![end], 0.0, 100.0),
                ("offset", vec![offset], -100.0, 100.0),
                ("amount", vec![amount], -100.0, 100.0),
                ("smoothness", vec![smoothness], 0.0, 100.0),
                ("ease_high", vec![ease_high], 0.0, 100.0),
                ("ease_low", vec![ease_low], 0.0, 100.0),
            ],
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
            Effect::Paraffin {
                direction,
                spread,
                opacity,
                ..
            } => vec![
                ("direction", vec![direction], 0.0, 360.0),
                ("spread", vec![spread], 0.0, 100.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::KiraKira {
                threshold,
                spacing,
                density,
                size,
                angle,
                twinkle,
                period,
                seed,
                opacity,
                ..
            } => vec![
                ("threshold", vec![threshold], 0.0, 100.0),
                ("spacing", vec![spacing], 2.0, 1000.0),
                ("density", vec![density], 0.0, 100.0),
                ("size", vec![size], 0.0, 1000.0),
                ("angle", vec![angle], -3600.0, 3600.0),
                ("twinkle", vec![twinkle], 0.0, 100.0),
                ("period", vec![period], 1.0, 1000.0),
                ("seed", vec![seed], 0.0, 100000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::LightningBolt {
                start,
                end,
                jagged,
                detail,
                branches,
                width,
                glow,
                opacity,
                hold,
                seed,
                turbulence,
                decay,
                conductivity,
                obstacle,
                ..
            } => vec![
                ("start", start.iter_mut().collect(), -1000.0, 1000.0),
                ("end", end.iter_mut().collect(), -1000.0, 1000.0),
                ("jagged", vec![jagged], 0.0, 100.0),
                ("detail", vec![detail], 1.0, 8.0),
                ("branches", vec![branches], 0.0, 100.0),
                ("width", vec![width], 0.0, 100.0),
                ("glow", vec![glow], 0.0, 500.0),
                ("opacity", vec![opacity], 0.0, 100.0),
                ("hold", vec![hold], 1.0, 100.0),
                ("seed", vec![seed], 0.0, 100000.0),
                ("turbulence", vec![turbulence], 0.0, 100.0),
                ("decay", vec![decay], 0.0, 100.0),
                ("conductivity", vec![conductivity], 0.0, 10000.0),
                ("obstacle", vec![obstacle], -100.0, 100.0),
            ],
            Effect::CompoundBlur { max_blur, .. } => vec![("max_blur", vec![max_blur], 0.0, 500.0)],
            Effect::DisplacementMap { max_horizontal, max_vertical, .. } => vec![
                ("max_horizontal", vec![max_horizontal], -1000.0, 1000.0),
                ("max_vertical", vec![max_vertical], -1000.0, 1000.0),
            ],
            Effect::GradientWipe { completion, softness, .. } => vec![
                ("completion", vec![completion], 0.0, 100.0),
                ("softness", vec![softness], 0.0, 100.0),
            ],
            Effect::Echo { echo_time, echoes, intensity, decay, .. } => vec![
                ("echo_time", vec![echo_time], -120.0, 120.0),
                ("echoes", vec![echoes], 0.0, 30.0),
                ("intensity", vec![intensity], 0.0, 1.0),
                ("decay", vec![decay], 0.0, 1.0),
            ],
            Effect::PosterizeTime { frame_rate } => vec![("frame_rate", vec![frame_rate], 0.1, 99.0)],
            Effect::ChangeToColor {
                hue_tolerance,
                lightness_tolerance,
                saturation_tolerance,
                softness,
                ..
            } => vec![
                ("hue_tolerance", vec![hue_tolerance], 0.0, 100.0),
                ("lightness_tolerance", vec![lightness_tolerance], 0.0, 100.0),
                ("saturation_tolerance", vec![saturation_tolerance], 0.0, 100.0),
                ("softness", vec![softness], 0.0, 100.0),
            ],
            Effect::PolarCoordinates { interpolation, .. } => vec![("interpolation", vec![interpolation], 0.0, 100.0)],
            Effect::Median { radius, .. } => vec![("radius", vec![radius], 0.0, 10.0)],
            Effect::SmartBlur { radius, threshold } => vec![
                ("radius", vec![radius], 0.0, 10.0),
                ("threshold", vec![threshold], 0.0, 255.0),
            ],
            Effect::BilateralBlur { radius, threshold, .. } => vec![
                ("radius", vec![radius], 0.0, 50.0),
                ("threshold", vec![threshold], 0.0, 255.0),
            ],
            Effect::CrossBlur { radius_x, radius_y, .. } => vec![
                ("radius_x", vec![radius_x], 0.0, 500.0),
                ("radius_y", vec![radius_y], 0.0, 500.0),
            ],
            Effect::SpinZoomBlur { amount, quality, center, .. } => vec![
                ("amount", vec![amount], -360.0, 360.0),
                ("quality", vec![quality], 1.0, 100.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
            ],
            Effect::FastZoomBlur { amount, center, .. } => vec![
                ("amount", vec![amount], 0.0, 100.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
            ],
            Effect::Snowfall {
                density,
                spacing,
                size,
                depth,
                speed,
                wind,
                wiggle,
                period,
                seed,
                opacity,
                ..
            } => vec![
                ("density", vec![density], 0.0, 100.0),
                ("spacing", vec![spacing], 2.0, 1000.0),
                ("size", vec![size], 0.0, 100.0),
                ("depth", vec![depth], 0.0, 100.0),
                ("speed", vec![speed], 0.0, 1000.0),
                ("wind", vec![wind], -1000.0, 1000.0),
                ("wiggle", vec![wiggle], 0.0, 100.0),
                ("period", vec![period], 1.0, 1000.0),
                ("seed", vec![seed], 0.0, 100000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::Kaleidoscope {
                segments,
                rotation,
                size,
                center,
                ..
            } => vec![
                ("segments", vec![segments], 2.0, 32.0),
                ("rotation", vec![rotation], -3600.0, 3600.0),
                ("size", vec![size], 10.0, 1000.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
            ],
            Effect::RoughenEdges {
                border,
                size,
                complexity,
                evolution,
                speed,
                seed,
                ..
            } => vec![
                ("border", vec![border], 0.0, 500.0),
                ("size", vec![size], 1.0, 1000.0),
                ("complexity", vec![complexity], 1.0, 10.0),
                ("evolution", vec![evolution], -100000.0, 100000.0),
                ("speed", vec![speed], -360.0, 360.0),
                ("seed", vec![seed], 0.0, 100000.0),
            ],
            Effect::Beam {
                start,
                end,
                length,
                time,
                start_thickness,
                end_thickness,
                softness,
                ..
            } => vec![
                ("start", start.iter_mut().collect(), -1000.0, 1000.0),
                ("end", end.iter_mut().collect(), -1000.0, 1000.0),
                ("length", vec![length], 0.0, 100.0),
                ("time", vec![time], 0.0, 100.0),
                ("start_thickness", vec![start_thickness], 0.0, 500.0),
                ("end_thickness", vec![end_thickness], 0.0, 500.0),
                ("softness", vec![softness], 0.0, 100.0),
            ],
            Effect::FourColorGradient {
                point_1,
                point_2,
                point_3,
                point_4,
                blend,
                opacity,
                ..
            } => vec![
                ("point_1", point_1.iter_mut().collect(), -1000.0, 1000.0),
                ("point_2", point_2.iter_mut().collect(), -1000.0, 1000.0),
                ("point_3", point_3.iter_mut().collect(), -1000.0, 1000.0),
                ("point_4", point_4.iter_mut().collect(), -1000.0, 1000.0),
                ("blend", vec![blend], 1.0, 1000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::CellPattern {
                contrast,
                disperse,
                size,
                evolution,
                seed,
                opacity,
                ..
            } => vec![
                ("contrast", vec![contrast], 0.0, 1000.0),
                ("disperse", vec![disperse], 0.0, 1.5),
                ("size", vec![size], 1.0, 1000.0),
                ("evolution", vec![evolution], -100000.0, 100000.0),
                ("seed", vec![seed], 0.0, 100000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
            ],
            Effect::OpticsCompensation {
                field_of_view, center, ..
            } => vec![
                ("field_of_view", vec![field_of_view], 0.0, 180.0),
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
            ],
            Effect::RadialShadow {
                opacity,
                light,
                distance,
                softness,
                color_influence,
                ..
            } => vec![
                ("opacity", vec![opacity], 0.0, 100.0),
                ("light", light.iter_mut().collect(), -1000.0, 1000.0),
                ("distance", vec![distance], 0.0, 1000.0),
                ("softness", vec![softness], 0.0, 500.0),
                ("color_influence", vec![color_influence], 0.0, 100.0),
            ],
            Effect::Extract {
                black_point,
                white_point,
                black_softness,
                white_softness,
                ..
            } => vec![
                ("black_point", vec![black_point], 0.0, 255.0),
                ("white_point", vec![white_point], 0.0, 255.0),
                ("black_softness", vec![black_softness], 0.0, 255.0),
                ("white_softness", vec![white_softness], 0.0, 255.0),
            ],
            Effect::BevelAlpha {
                edge_thickness,
                light_angle,
                light_intensity,
                ..
            } => vec![
                ("edge_thickness", vec![edge_thickness], 0.0, 200.0),
                ("light_angle", vec![light_angle], -3600.0, 3600.0),
                ("light_intensity", vec![light_intensity], 0.0, 1.0),
            ],
            Effect::BevelEdges {
                edge_thickness,
                light_angle,
                light_intensity,
                ..
            } => vec![
                ("edge_thickness", vec![edge_thickness], 0.0, 0.5),
                ("light_angle", vec![light_angle], -3600.0, 3600.0),
                ("light_intensity", vec![light_intensity], 0.0, 1.0),
            ],
            Effect::BlockDissolve {
                completion,
                block_width,
                block_height,
                feather,
            } => vec![
                ("completion", vec![completion], 0.0, 100.0),
                ("block_width", vec![block_width], 1.0, 10000.0),
                ("block_height", vec![block_height], 1.0, 10000.0),
                ("feather", vec![feather], 0.0, 10000.0),
            ],
            Effect::CornerPin {
                upper_left,
                upper_right,
                lower_left,
                lower_right,
            } => vec![
                ("upper_left", upper_left.iter_mut().collect(), -400.0, 500.0),
                ("upper_right", upper_right.iter_mut().collect(), -400.0, 500.0),
                ("lower_left", lower_left.iter_mut().collect(), -400.0, 500.0),
                ("lower_right", lower_right.iter_mut().collect(), -400.0, 500.0),
            ],
            Effect::LightSweep {
                center,
                direction,
                width,
                sweep_intensity,
                edge_intensity,
                edge_thickness,
                ..
            } => vec![
                ("center", center.iter_mut().collect(), -1000.0, 1000.0),
                ("direction", vec![direction], -3600.0, 3600.0),
                ("width", vec![width], 0.0, 10000.0),
                ("sweep_intensity", vec![sweep_intensity], 0.0, 100.0),
                ("edge_intensity", vec![edge_intensity], 0.0, 100.0),
                ("edge_thickness", vec![edge_thickness], 1.0, 50.0),
            ],
            Effect::RadioWaves {
                producer_point,
                sides,
                interval,
                expansion,
                orientation,
                direction,
                velocity,
                spin,
                lifespan,
                opacity,
                fade_in_time,
                fade_out_time,
                start_width,
                end_width,
                ..
            } => vec![
                ("producer_point", producer_point.iter_mut().collect(), -1000.0, 1000.0),
                ("sides", vec![sides], 3.0, 64.0),
                ("interval", vec![interval], 1.0, 1000.0),
                ("expansion", vec![expansion], 0.0, 1000.0),
                ("orientation", vec![orientation], -3600.0, 3600.0),
                ("direction", vec![direction], -3600.0, 3600.0),
                ("velocity", vec![velocity], 0.0, 1000.0),
                ("spin", vec![spin], -360.0, 360.0),
                ("lifespan", vec![lifespan], 1.0, 1000.0),
                ("opacity", vec![opacity], 0.0, 100.0),
                ("fade_in_time", vec![fade_in_time], 0.0, 1000.0),
                ("fade_out_time", vec![fade_out_time], 0.0, 1000.0),
                ("start_width", vec![start_width], 0.0, 1000.0),
                ("end_width", vec![end_width], 0.0, 1000.0),
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

    /// D-332: the name of every setting a key can be put on.
    pub fn names(&self) -> Vec<&'static str> {
        self.clone().numbers().into_iter().map(|n| n.0).collect()
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
            Effect::ChannelBlur {
                red_blurriness,
                green_blurriness,
                blue_blurriness,
                alpha_blurriness,
                ..
            } => {
                for s in [red_blurriness, green_blurriness, blue_blurriness, alpha_blurriness] {
                    *s = scale(*s);
                }
            }
            Effect::FastBoxBlur { radius, .. } => *radius = scale(*radius),
            Effect::CrossBlur { radius_x, radius_y, .. } => {
                *radius_x = scale(*radius_x);
                *radius_y = scale(*radius_y);
            }
            // D-87's blur and D-89's radius are distances in pixels too.
            Effect::SelectiveColorBlur { blur, .. } => *blur = scale(*blur),
            Effect::Glow { radius, .. } => *radius = scale(*radius),
            Effect::DirectionalBlur { length, .. } => *length = scale(*length),
            Effect::LineWidth { width, .. } => *width = scale(*width),
            Effect::LineBlur { length, .. } => *length = scale(*length),
            // D-203: a radius scaled under one leaves the layer as it is.
            Effect::Median { radius, .. } | Effect::SmartBlur { radius, .. } | Effect::BilateralBlur { radius, .. } => {
                *radius = scale(*radius)
            }
            Effect::Snowfall {
                spacing,
                size,
                speed,
                wind,
                wiggle,
                ..
            } => {
                *spacing = scale(*spacing);
                *size = scale(*size);
                *speed = scale(*speed);
                *wind = scale(*wind);
                *wiggle = scale(*wiggle);
            }
            // D-206: a bite under a pixel wide is held at one, as Turbulent Displace's wave is.
            Effect::RoughenEdges { border, size, .. } => {
                *border = scale(*border);
                *size = scale(*size).max(1.0);
            }
            Effect::Beam { start_thickness, end_thickness, .. } => {
                *start_thickness = scale(*start_thickness);
                *end_thickness = scale(*end_thickness);
            }
            Effect::Bloom { radius, length, .. } => {
                *radius = scale(*radius);
                *length = scale(*length);
            }
            Effect::ChromaticAberration { amount, .. } => *amount = scale(*amount),
            Effect::DistanceGradation { width, .. } => *width = scale(*width),
            Effect::TurbulentDisplace { amount, size, units, .. } => {
                let push = scale(turbulent_push(*amount, *size, units));
                // A wave under a pixel is held at one, as its range is, rather than bypassed.
                *size = scale(*size).max(1.0);
                // D-328: the push scaled, the amount worked back from it at the drafted size.
                *amount = if units == "after_effects" { push * 100.0 / size.min(100.0) } else { push };
            }
            // D-128: held at one in a draft, as D-127's wave is.
            Effect::FractalNoise { size, offset, .. } => {
                *size = scale(*size).max(1.0);
                // D-299: the slide in pixels, as Offset's.
                *offset = offset.map(&scale);
            }
            Effect::CellPattern { size, .. } => *size = scale(*size).max(1.0),
            // D-143: held at its smallest, two, rather than bypassed.
            Effect::Halftone { size, .. } => *size = scale(*size).max(2.0),
            // D-144: a block under a pixel is one pixel, which changes nothing.
            Effect::Mosaic { size } => *size = scale(*size).max(1.0),
            Effect::Emboss { relief, .. } => *relief = scale(*relief),
            // D-213: Bevel Edges' thickness is a share of the layer and is left.
            Effect::BevelAlpha { edge_thickness, .. } => *edge_thickness = scale(*edge_thickness),
            // D-214: a block is never less than a pixel, the least the command takes.
            Effect::BlockDissolve {
                block_width,
                block_height,
                feather,
                ..
            } => {
                *block_width = scale(*block_width).max(1.0);
                *block_height = scale(*block_height).max(1.0);
                *feather = scale(*feather);
            }
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
            Effect::Bulge { radius, vertical_radius, taper_radius, .. } => {
                *radius = scale(*radius);
                *vertical_radius = scale(*vertical_radius);
                *taper_radius = scale(*taper_radius);
            }
            Effect::LinearWipe { feather, .. } => *feather = scale(*feather),
            Effect::IrisWipe { feather, .. } => *feather = scale(*feather),
            Effect::SimpleChoker { choke } => *choke = scale(*choke),
            Effect::MatteChoker { geometric_softness_1, geometric_softness_2, .. } => {
                *geometric_softness_1 = scale(*geometric_softness_1);
                *geometric_softness_2 = scale(*geometric_softness_2);
            }
            Effect::RefineMatte { edge_radius, feather, decontamination_radius, .. } => {
                *edge_radius = scale(*edge_radius);
                *feather = scale(*feather);
                *decontamination_radius = scale(*decontamination_radius);
            }
            // D-356: the masks are scaled as they always are, before the stack.
            Effect::Stroke { brush_size, .. } => *brush_size = scale(*brush_size),
            Effect::SoftGlow { radius, .. } => *radius = scale(*radius),
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
            Effect::KiraKira { spacing, size, .. } => {
                *spacing = scale(*spacing);
                *size = scale(*size);
            }
            Effect::LightningBolt { width, glow, .. } => {
                *width = scale(*width);
                *glow = scale(*glow);
            }
            Effect::CompoundBlur { max_blur, .. } => *max_blur = scale(*max_blur),
            Effect::Glass { softness, displacement, .. } => {
                *softness = scale(*softness);
                *displacement = scale(*displacement);
            }
            Effect::VectorBlur { amount, map_softness, .. } => {
                *amount = scale(*amount);
                *map_softness = scale(*map_softness);
            }
            Effect::IdKey { feather, .. } => *feather = scale(*feather),
            Effect::DisplacementMap { max_horizontal, max_vertical, .. } => {
                *max_horizontal = scale(*max_horizontal);
                *max_vertical = scale(*max_vertical);
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
            // D-211: the light is a share of the drawing and the distance a ratio; only the
            // softening is in pixels.
            Effect::RadialShadow { softness, .. } => *softness = scale(*softness),
            // D-132: so is the wrap's width.
            Effect::LightWrap { width, .. } => *width = scale(*width),
            // D-131: the shift is a distance, so a draft slides by its share.
            Effect::Offset { shift } => *shift = shift.map(&scale),
            // D-199: so are the band's width and the edge's depth, which stays a pixel at least.
            Effect::LightSweep {
                width, edge_thickness, ..
            } => {
                *width = scale(*width);
                *edge_thickness = scale(*edge_thickness).max(1.0);
            }
            // D-200: how fast the waves grow and drift, and how wide they are.
            Effect::RadioWaves {
                expansion,
                velocity,
                start_width,
                end_width,
                ..
            } => {
                *expansion = scale(*expansion);
                *velocity = scale(*velocity);
                *start_width = scale(*start_width);
                *end_width = scale(*end_width);
            }
            _ => {}
        }
    }

    /// The name a person reads, the Effects panel's own: undo's list uses it.
    pub fn name(&self) -> &str {
        match self {
            Effect::Exposure { .. } => "Exposure",
            Effect::GaussianBlur { .. } => "Gaussian Blur",
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
            Effect::Paraffin { .. } => "Paraffin",
            Effect::KiraKira { .. } => "Kira-kira",
            Effect::LightningBolt { .. } => "Lightning Bolt",
            Effect::CompoundBlur { .. } => "Compound Blur",
            Effect::DisplacementMap { .. } => "Displacement Map",
            Effect::GradientWipe { .. } => "Gradient Wipe",
            Effect::Echo { .. } => "Echo",
            Effect::PosterizeTime { .. } => "Posterize Time",
            Effect::ChangeToColor { .. } => "Change to Color",
            Effect::CornerPin { .. } => "Corner Pin",
            Effect::LightSweep { .. } => "Light Sweep",
            Effect::RadioWaves { .. } => "Radio Waves",
            Effect::PolarCoordinates { .. } => "Polar Coordinates",
            Effect::Median { .. } => "Median",
            Effect::SmartBlur { .. } => "Smart Blur",
            Effect::BilateralBlur { .. } => "Bilateral Blur",
            Effect::CrossBlur { .. } => "Cross Blur",
            Effect::SpinZoomBlur { .. } => "Spin & Zoom Blur",
            Effect::FastZoomBlur { .. } => "Fast Zoom Blur",
            Effect::Snowfall { .. } => "Snowfall",
            Effect::Kaleidoscope { .. } => "Kaleidoscope",
            Effect::RoughenEdges { .. } => "Roughen Edges",
            Effect::Beam { .. } => "Beam",
            Effect::FourColorGradient { .. } => "4-Color Gradient",
            Effect::CellPattern { .. } => "Cell Pattern",
            Effect::OpticsCompensation { .. } => "Optics Compensation",
            Effect::RadialShadow { .. } => "Radial Shadow",
            Effect::Extract { .. } => "Extract",
            Effect::BevelAlpha { .. } => "Bevel Alpha",
            Effect::BevelEdges { .. } => "Bevel Edges",
            Effect::BlockDissolve { .. } => "Block Dissolve",
            Effect::ShiftChannels { .. } => "Shift Channels",
            Effect::SolidComposite { .. } => "Solid Composite",
            Effect::ChannelBlur { .. } => "Channel Blur",
            Effect::FastBoxBlur { .. } => "Fast Box Blur",
            Effect::Colorama { .. } => "Colorama",
            Effect::Glass { .. } => "CC Glass",
            Effect::VectorBlur { .. } => "CC Vector Blur",
            Effect::MomentMap { .. } => "Moment Map",
            Effect::PassExtract { .. } => "Pass Extract",
            Effect::DepthKey { .. } => "Depth Key",
            Effect::IdKey { .. } => "ID Key",
            Effect::TextAnimator { .. } => "Text Animator",
            Effect::AutoTone { kind: "levels", .. } => "Stretch Levels",
            Effect::AutoTone { kind: "contrast", .. } => "Stretch Contrast",
            Effect::AutoTone { .. } => "Stretch Color",
            Effect::SpreadTones { .. } => "Spread Tones",
            Effect::MatteChoker { .. } => "Matte Choker",
            Effect::RefineMatte { kind: "hard", .. } => "Refine Hard Matte",
            Effect::RefineMatte { .. } => "Refine Soft Matte",
            Effect::Stroke { .. } => "Path Stroke",
            Effect::SoftGlow { .. } => "Soft Physical Glow",
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
            Effect::Paraffin { .. } => PARAFFIN,
            Effect::KiraKira { .. } => KIRA_KIRA,
            Effect::LightningBolt { .. } => LIGHTNING_BOLT,
            Effect::CompoundBlur { .. } => COMPOUND_BLUR,
            Effect::DisplacementMap { .. } => DISPLACEMENT_MAP,
            Effect::GradientWipe { .. } => GRADIENT_WIPE,
            Effect::Echo { .. } => ECHO,
            Effect::PosterizeTime { .. } => POSTERIZE_TIME,
            Effect::ChangeToColor { .. } => CHANGE_TO_COLOR,
            Effect::CornerPin { .. } => CORNER_PIN,
            Effect::LightSweep { .. } => LIGHT_SWEEP,
            Effect::RadioWaves { .. } => RADIO_WAVES,
            Effect::PolarCoordinates { .. } => POLAR_COORDINATES,
            Effect::Median { .. } => MEDIAN,
            Effect::SmartBlur { .. } => SMART_BLUR,
            Effect::BilateralBlur { .. } => BILATERAL_BLUR,
            Effect::CrossBlur { .. } => CROSS_BLUR,
            Effect::SpinZoomBlur { .. } => SPIN_ZOOM_BLUR,
            Effect::FastZoomBlur { .. } => FAST_ZOOM_BLUR,
            Effect::Snowfall { .. } => SNOWFALL,
            Effect::Kaleidoscope { .. } => KALEIDOSCOPE,
            Effect::RoughenEdges { .. } => ROUGHEN_EDGES,
            Effect::Beam { .. } => BEAM,
            Effect::FourColorGradient { .. } => FOUR_COLOR_GRADIENT,
            Effect::CellPattern { .. } => CELL_PATTERN,
            Effect::OpticsCompensation { .. } => OPTICS_COMPENSATION,
            Effect::RadialShadow { .. } => RADIAL_SHADOW,
            Effect::Extract { .. } => EXTRACT,
            Effect::BevelAlpha { .. } => BEVEL_ALPHA,
            Effect::BevelEdges { .. } => BEVEL_EDGES,
            Effect::BlockDissolve { .. } => BLOCK_DISSOLVE,
            Effect::ShiftChannels { .. } => SHIFT_CHANNELS,
            Effect::SolidComposite { .. } => SOLID_COMPOSITE,
            Effect::ChannelBlur { .. } => CHANNEL_BLUR,
            Effect::FastBoxBlur { .. } => FAST_BOX_BLUR,
            Effect::Colorama { .. } => COLORAMA,
            Effect::Glass { .. } => GLASS,
            Effect::VectorBlur { .. } => VECTOR_BLUR,
            Effect::MomentMap { .. } => MOMENT_MAP,
            Effect::PassExtract { .. } => PASS_EXTRACT,
            Effect::DepthKey { .. } => DEPTH_KEY,
            Effect::IdKey { .. } => ID_KEY,
            Effect::TextAnimator { .. } => TEXT_ANIMATOR,
            Effect::AutoTone { kind: "levels", .. } => STRETCH_LEVELS,
            Effect::AutoTone { kind: "contrast", .. } => STRETCH_CONTRAST,
            Effect::AutoTone { .. } => STRETCH_COLOR,
            Effect::SpreadTones { .. } => SPREAD_TONES,
            Effect::MatteChoker { .. } => MATTE_CHOKER,
            Effect::RefineMatte { kind: "hard", .. } => REFINE_HARD_MATTE,
            Effect::RefineMatte { .. } => REFINE_SOFT_MATTE,
            Effect::Stroke { .. } => STROKE,
            Effect::SoftGlow { .. } => GLOW,
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
            Effect::GaussianBlur { sigma_px, units, .. } => {
                let (sigma, long) = blur_reach(*sigma_px, units);
                reach_radius(sigma, long)
            }
            // D-313: the alpha's blur, as Blur's; a colour reaching further has no alpha there.
            Effect::ChannelBlur { alpha_blurriness, edges, .. } if edges != "repeat" => {
                kernel_radius(*alpha_blurriness)
            }
            // D-327: the box's reach, once a pass.
            Effect::FastBoxBlur { radius, iterations, edges, .. } if edges != "repeat" => {
                box_reach(*radius, *iterations)
            }
            // D-360: the longer box's reach.
            Effect::CrossBlur { radius_x, radius_y, edges, .. } if edges != "repeat" => {
                box_reach(*radius_x, 1.0).max(box_reach(*radius_y, 1.0))
            }
            // D-353: as far as any level's cells reach, stretched or not (no silent crop).
            Effect::SoftGlow { radius, aspect_ratio, aspect_angle, .. } if self.fault().is_none() => {
                crate::soft_glow::plan(*radius, *aspect_ratio, *aspect_angle).grow
            }
            // D-89: the light reaches `radius` pixels, blur's reach at sigma radius / 3.
            Effect::Glow { radius, units, .. } => {
                let (sigma, long) = glow_reach(*radius, units);
                reach_radius(sigma, long)
            }
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
            // D-186: the size rounded up, unless nothing is added.
            Effect::KiraKira {
                size, density, opacity, ..
            } if *size > 0.0 && *density > 0.0 && *opacity > 0.0 => size.ceil() as usize,
            // D-127: the push rounded up, unless a push past the edge reads the edge.
            Effect::TurbulentDisplace { amount, size, edges, units, .. } if edges != "repeat" => {
                turbulent_push(*amount, *size, units).ceil() as usize
            }
            // D-315: with Expand Output the larger maximum rounded up, unless the push wraps.
            Effect::DisplacementMap { max_horizontal, max_vertical, wrap, expand, .. }
                if expand == "on" && wrap != "on" =>
            {
                max_horizontal.abs().max(max_vertical.abs()).ceil() as usize
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
            // D-90 and D-318, as in `numbers`.
            Effect::Exposure { stops, offset, gamma, bypass } => {
                (-40.0..=40.0).contains(stops)
                    && (-0.5..=0.5).contains(offset)
                    && (0.01..=9.99).contains(gamma)
                    && (bypass == "off" || bypass == "on")
            }
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
            Effect::Exposure { stops, offset, gamma, bypass } => {
                if !(-40.0..=40.0).contains(stops) {
                    format!("Exposure runs from -40 to 40 stops, and this is {stops}.")
                } else if !(-0.5..=0.5).contains(offset) {
                    format!("Exposure's Offset runs from -0.5 to 0.5, and this is {offset}.")
                } else if !(0.01..=9.99).contains(gamma) {
                    format!("Exposure's Gamma Correction runs from 0.01 to 9.99, and this is {gamma}.")
                } else {
                    format!("Exposure's Bypass Linear Light Conversion is off or on, and this is \"{bypass}\".")
                }
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
    /// D-189: the layer this effect's layer setting names, and its fit, when it has one
    /// written as a word.
    pub fn layer_setting(&self) -> Option<(&str, &str)> {
        match self {
            Effect::CompoundBlur { layer: serde_json::Value::String(layer), fit, .. }
            | Effect::DisplacementMap { layer: serde_json::Value::String(layer), fit, .. }
            | Effect::GradientWipe { layer: serde_json::Value::String(layer), fit, .. }
            | Effect::Colorama { layer: serde_json::Value::String(layer), fit, .. }
            | Effect::Glass { layer: serde_json::Value::String(layer), fit, .. }
            | Effect::VectorBlur { layer: serde_json::Value::String(layer), fit, .. }
            | Effect::MomentMap { layer: serde_json::Value::String(layer), fit, .. }
            | Effect::LensBlur { layer: serde_json::Value::String(layer), fit, .. } => Some((layer, fit)),
            _ => None,
        }
    }

    /// D-189: the layer setting as written, and the map compose reads into for a frame.
    pub fn layer_setting_mut(&mut self) -> Option<(&mut serde_json::Value, &mut Option<crate::layer_map::Map>)> {
        match self {
            Effect::CompoundBlur { layer, map, .. }
            | Effect::DisplacementMap { layer, map, .. }
            | Effect::GradientWipe { layer, map, .. }
            | Effect::Colorama { layer, map, .. }
            | Effect::Glass { layer, map, .. }
            | Effect::VectorBlur { layer, map, .. }
            | Effect::MomentMap { layer, map, .. }
            | Effect::LensBlur { layer, map, .. } => Some((layer, map)),
            _ => None,
        }
    }

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
            Effect::GaussianBlur { edges: e, dimensions, units, .. } => edges(e).or_else(|| {
                (!["both", "horizontal", "vertical"].contains(&dimensions.as_str())).then(|| {
                    format!("{name}'s dimensions are \"both\", \"horizontal\" or \"vertical\", and this is \"{dimensions}\".")
                })
            }).or_else(|| {
                (!["sigma", "blurriness"].contains(&units.as_str())).then(|| {
                    format!("{name}'s units are \"sigma\" or \"blurriness\", and this is \"{units}\".")
                })
            }),
            Effect::DirectionalBlur { edges: e, .. } => edges(e),
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
            Effect::CrossBlur { mode, edges: e, .. } => (!CROSS_MODES.contains(&mode.as_str()))
                .then(|| format!("{name}'s mode is \"blend\", \"add\", \"screen\", \"multiply\", \"lighten\" or \"darken\", and this is \"{mode}\"."))
                .or_else(|| edges(e)),
            Effect::SpinZoomBlur { kind, .. } => (!SPIN_ZOOM_TYPES.contains(&kind.as_str())).then(|| {
                format!("{name}'s type is \"straight_zoom\", \"fading_zoom\", \"centered_zoom\", \"rotate\", \"scratch\" or \"rotate_fading\", and this is \"{kind}\".")
            }),
            Effect::FastZoomBlur { zoom, .. } => (!["standard", "brightest", "darkest"].contains(&zoom.as_str()))
                .then(|| format!("{name}'s zoom is \"standard\", \"brightest\" or \"darkest\", and this is \"{zoom}\".")),
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
                alpha,
            } => [("master", master), ("red", red), ("green", green), ("blue", blue), ("alpha", alpha)]
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
            Effect::RadialShadow {
                color,
                render,
                shadow_only,
                ..
            } => hex_fault("Radial Shadow", "colour", color)
                .or_else(|| {
                    (!["regular", "glass_edge"].contains(&render.as_str())).then(|| {
                        format!("Radial Shadow's render is \"regular\" or \"glass_edge\", and this is \"{render}\".")
                    })
                })
                .or_else(|| {
                    (!["on", "off"].contains(&shadow_only.as_str())).then(|| {
                        format!("Radial Shadow's shadow only is \"on\" or \"off\", and this is \"{shadow_only}\".")
                    })
                }),
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
            Effect::LensBlur { layer, .. } if !layer.is_string() => Some(format!(
                "Lens Blur's blur layer is the name of a layer of this composition, and this is {layer}."
            )),
            Effect::LensBlur { fit, .. } if !["center", "stretch"].contains(&fit.as_str()) => Some(format!(
                "Lens Blur's map placement is \"center\" or \"stretch\", and this is \"{fit}\"."
            )),
            Effect::LensBlur { channel, .. } if !["luminance", "alpha"].contains(&channel.as_str()) => Some(format!(
                "Lens Blur's map channel is \"luminance\" or \"alpha\", and this is \"{channel}\"."
            )),
            Effect::LensBlur { invert, .. } if !["off", "on"].contains(&invert.as_str()) => Some(format!(
                "Lens Blur's invert blur map is \"off\" or \"on\", and this is \"{invert}\"."
            )),
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
            Effect::TurbulentDisplace { edges: e, displacement, pinning, units, .. } => edges(e)
                .or_else(|| {
                    (!["turbulent", "horizontal", "vertical"].contains(&displacement.as_str())).then(|| {
                        format!("{name}'s displacement is \"turbulent\", \"horizontal\" or \"vertical\", and this is \"{displacement}\".")
                    })
                })
                .or_else(|| {
                    (!["none", "all"].contains(&pinning.as_str()))
                        .then(|| format!("{name}'s pinning is \"none\" or \"all\", and this is \"{pinning}\"."))
                })
                .or_else(|| {
                    (!["classic", "after_effects"].contains(&units.as_str())).then(|| {
                        format!("{name}'s units are \"classic\" or \"after_effects\", and this is \"{units}\".")
                    })
                }),
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
                fractal_type,
                noise_type,
                invert,
                ..
            } => hex_fault("Fractal Noise", "dark colour", dark_color)
                .or_else(|| hex_fault("Fractal Noise", "light colour", light_color))
                .or_else(|| {
                    // D-299.
                    [
                        ("fractal type", fractal_type, ["basic", "turbulent"]),
                        ("noise type", noise_type, ["smooth", "block"]),
                        ("invert", invert, ["off", "on"]),
                    ]
                    .into_iter()
                    .find(|(_, word, allowed)| !allowed.contains(&word.as_str()))
                    .map(|(what, word, [a, b])| {
                        format!("Fractal Noise's {what} is \"{a}\" or \"{b}\", and this is \"{word}\".")
                    })
                }),
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
                preserve_luminosity,
            } => [("shadows", shadows), ("midtones", midtones), ("highlights", highlights)]
                .into_iter()
                .find(|(_, tone)| tone.len() != 3)
                .map(|(what, tone)| {
                    format!(
                        "{name}'s {what} are three numbers, red, green and blue, and this has {}.",
                        tone.len()
                    )
                })
                .or_else(|| {
                    (!["off", "on"].contains(&preserve_luminosity.as_str())).then(|| {
                        format!("{name}'s preserve luminosity is \"off\" or \"on\", and this is \"{preserve_luminosity}\".")
                    })
                }),
            Effect::LightWrap { blend, .. } if !["screen", "add"].contains(&blend.as_str()) => {
                Some(format!(
                    "Light Wrap's blend is \"screen\" or \"add\", and this is \"{blend}\"."
                ))
            }
            Effect::Glass { property, .. } if !COLORAMA_PHASES.contains(&property.as_str()) => Some(format!(
                "CC Glass's property is intensity, luminance, red, green, blue or alpha, and this is \"{property}\"."
            )),
            Effect::Glass { layer, .. } if !layer.is_string() => Some(format!(
                "CC Glass's bump map is the name of a layer of this composition, and this is {layer}."
            )),
            Effect::Glass { fit, .. } if !["center", "stretch", "tile"].contains(&fit.as_str()) => Some(format!(
                "CC Glass's fit is \"center\", \"stretch\" or \"tile\", and this is \"{fit}\"."
            )),
            Effect::Glass { light_color, .. } => hex_fault("CC Glass", "light colour", light_color),
            Effect::VectorBlur { kind, .. } if !VECTOR_BLUR_TYPES.contains(&kind.as_str()) => Some(format!(
                "CC Vector Blur's type is natural, constant, perpendicular, direction_center or direction_fading, and this is \"{kind}\"."
            )),
            Effect::VectorBlur { property, .. } if !VECTOR_BLUR_PROPERTIES.contains(&property.as_str()) => Some(format!(
                "CC Vector Blur's property is red, green, blue, alpha, luminance, lightness, hue or saturation, and this is \"{property}\"."
            )),
            Effect::VectorBlur { layer, .. } if !layer.is_string() => Some(format!(
                "CC Vector Blur's vector map is the name of a layer of this composition, and this is {layer}."
            )),
            Effect::VectorBlur { fit, .. } if !["center", "stretch", "tile"].contains(&fit.as_str()) => Some(format!(
                "CC Vector Blur's fit is \"center\", \"stretch\" or \"tile\", and this is \"{fit}\"."
            )),
            Effect::MomentMap { layer, .. } if !layer.is_string() => Some(format!(
                "Moment Map's map is the name of a layer of this composition, and this is {layer}."
            )),
            Effect::MomentMap { fit, .. } if !["center", "stretch", "tile"].contains(&fit.as_str()) => Some(format!(
                "Moment Map's fit is \"center\", \"stretch\" or \"tile\", and this is \"{fit}\"."
            )),
            Effect::PassExtract { pass, .. } if !PASSES.contains(&pass.as_str()) => Some(format!(
                "Pass Extract's pass is \"depth\", \"normals\", \"object_id\", \"material_id\" or \"named\", and this is \"{pass}\"."
            )),
            Effect::IdKey { aux_channel, .. } if !["object_id", "material_id"].contains(&aux_channel.as_str()) => Some(format!(
                "ID Key's channel is \"object_id\" or \"material_id\", and this is \"{aux_channel}\"."
            )),
            Effect::TextAnimator { fill, .. } if !["off", "on"].contains(&fill.as_str()) => Some(format!(
                "Text Animator's fill is \"off\" or \"on\", and this is \"{fill}\"."
            )),
            Effect::TextAnimator { based_on, .. } if !TEXT_BASED_ON.contains(&based_on.as_str()) => Some(format!(
                "Text Animator is based on \"characters\", \"characters_excluding_spaces\" or \"words\", and this is \"{based_on}\"."
            )),
            Effect::TextAnimator { shape, .. } if !TEXT_SHAPES.contains(&shape.as_str()) => Some(format!(
                "Text Animator's shape is \"square\", \"ramp_up\", \"ramp_down\", \"triangle\", \"round\" or \"smooth\", and this is \"{shape}\"."
            )),
            Effect::AutoTone { scene_detect, .. } if !["off", "on"].contains(&scene_detect.as_str()) => Some(format!(
                "{name}'s scene detect is \"off\" or \"on\", and this is \"{scene_detect}\"."
            )),
            Effect::AutoTone { snap_neutral_midtones, .. } if !["off", "on"].contains(&snap_neutral_midtones.as_str()) => Some(format!(
                "{name}'s snap neutral midtones is \"off\" or \"on\", and this is \"{snap_neutral_midtones}\"."
            )),
            Effect::SoftGlow { falloff, .. } if falloff != "physical" => Some(format!(
                "Glow's falloff is \"classic\" or \"physical\", and this is \"{falloff}\"."
            )),
            Effect::SoftGlow { threshold_mode: v, .. } if !["chroma", "luminance"].contains(&v.as_str()) => Some(format!(
                "{name}'s threshold mode is \"chroma\" or \"luminance\", and this is \"{v}\"."
            )),
            Effect::SoftGlow { operation: v, .. } if !["add", "screen"].contains(&v.as_str()) => Some(format!(
                "{name}'s blend mode is \"add\" or \"screen\", and this is \"{v}\"."
            )),
            Effect::SoftGlow { unmult: v, .. } if !["off", "on"].contains(&v.as_str()) => Some(format!(
                "{name}'s unmult is \"off\" or \"on\", and this is \"{v}\"."
            )),
            Effect::SpreadTones { equalize, .. } if !EQUALIZE.contains(&equalize.as_str()) => Some(format!(
                "Spread Tones equalizes by \"rgb\", \"brightness\" or \"photoshop\", and this is \"{equalize}\"."
            )),
            Effect::RefineMatte { kind: "soft", view_edge_region: v, .. } if !["off", "on"].contains(&v.as_str()) => Some(format!(
                "{name}'s view edge region is \"off\" or \"on\", and this is \"{v}\"."
            )),
            Effect::RefineMatte { decontaminate: v, .. } if !["off", "on"].contains(&v.as_str()) => Some(format!(
                "{name}'s decontaminate is \"off\" or \"on\", and this is \"{v}\"."
            )),
            Effect::RefineMatte { view_decontamination_map: v, .. } if !["off", "on"].contains(&v.as_str()) => Some(format!(
                "{name}'s view decontamination map is \"off\" or \"on\", and this is \"{v}\"."
            )),
            Effect::Stroke { all_masks: v, .. } if !["off", "on"].contains(&v.as_str()) => Some(format!(
                "Path Stroke's all masks is \"off\" or \"on\", and this is \"{v}\"."
            )),
            Effect::Stroke { stroke_sequentially: v, .. } if !["off", "on"].contains(&v.as_str()) => Some(format!(
                "Path Stroke's stroke sequentially is \"off\" or \"on\", and this is \"{v}\"."
            )),
            Effect::Stroke { paint_style: v, .. } if !PAINT_STYLES.contains(&v.as_str()) => Some(format!(
                "Path Stroke's paint style is \"on_original\", \"on_transparent\" or \"reveal\", and this is \"{v}\"."
            )),
            Effect::Stroke { source: v, .. } if !["masks", "shapes"].contains(&v.as_str()) => Some(format!(
                "Path Stroke's source is \"masks\" or \"shapes\", and this is \"{v}\"."
            )),
            Effect::Stroke { color, .. } => hex_fault("Path Stroke", "colour", color),
            Effect::PassExtract { invert, .. } | Effect::DepthKey { invert, .. } | Effect::IdKey { invert, .. } if !["off", "on"].contains(&invert.as_str()) => {
                Some(format!("{name}'s invert is \"off\" or \"on\", and this is \"{invert}\"."))
            }
            Effect::PassExtract { clamp, .. } if !["off", "on"].contains(&clamp.as_str()) => Some(format!(
                "Pass Extract's clamp is \"off\" or \"on\", and this is \"{clamp}\"."
            )),
            Effect::Colorama { get_phase, .. } if !COLORAMA_PHASES.contains(&get_phase.as_str()) => Some(format!(
                "Colorama gets its phase from intensity, luminance, red, green, blue or alpha, and this is \"{get_phase}\"."
            )),
            Effect::Colorama { layer, .. } if !layer.is_string() => Some(format!(
                "Colorama's layer is the name of a layer of this composition, and this is {layer}."
            )),
            Effect::Colorama { fit, .. } if !["center", "stretch", "tile"].contains(&fit.as_str()) => Some(format!(
                "Colorama's fit is \"center\", \"stretch\" or \"tile\", and this is \"{fit}\"."
            )),
            Effect::Colorama { color_1, color_2, color_3, color_4, color_5, .. } => [color_1, color_2, color_3, color_4, color_5]
                .iter()
                .enumerate()
                .find_map(|(i, c)| hex_fault("Colorama", &format!("colour {}", i + 1), c)),
            Effect::SolidComposite { blend, .. }
                if !["normal", "add", "screen", "multiply"].contains(&blend.as_str()) =>
            {
                Some(format!(
                    "Solid Composite's blend is \"normal\", \"add\", \"screen\" or \"multiply\", and \
                     this is \"{blend}\"."
                ))
            }
            Effect::SolidComposite { color, .. } => hex_fault("Solid Composite", "colour", color),
            Effect::ChannelBlur { edges: e, dimensions, .. }
            | Effect::FastBoxBlur { edges: e, dimensions, .. } => edges(e).or_else(|| {
                (!["both", "horizontal", "vertical"].contains(&dimensions.as_str())).then(|| {
                    format!("{name}'s dimensions are \"both\", \"horizontal\" or \"vertical\", and this is \"{dimensions}\".")
                })
            }),
            Effect::ShiftChannels {
                take_alpha,
                take_red,
                take_green,
                take_blue,
            } => [("alpha", take_alpha), ("red", take_red), ("green", take_green), ("blue", take_blue)]
                .into_iter()
                .find(|(_, w)| !SHIFT_CHANNELS_FROM.contains(&w.as_str()))
                .map(|(what, w)| {
                    format!(
                        "Shift Channels takes {what} from \"alpha\", \"red\", \"green\", \"blue\", \"luminance\", \"hue\", \"lightness\", \"saturation\", \"full\", \"half\" or \"off\", and this is \"{w}\"."
                    )
                }),
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
            Effect::ChangeToColor { from, to, change, change_by, view_matte, .. } => {
                hex_fault("Change to Color", "from colour", from)
                    .or_else(|| hex_fault("Change to Color", "to colour", to))
                    .or_else(|| {
                        (!["hue", "hue_lightness", "hue_saturation", "hue_lightness_saturation"].contains(&change.as_str()))
                            .then(|| {
                                format!(
                                    "Change to Color's change is \"hue\", \"hue_lightness\", \"hue_saturation\" or \
                                     \"hue_lightness_saturation\", and this is \"{change}\"."
                                )
                            })
                    })
                    .or_else(|| {
                        (!["setting", "transforming"].contains(&change_by.as_str())).then(|| {
                            format!("Change to Color's change by is \"setting\" or \"transforming\", and this is \"{change_by}\".")
                        })
                    })
                    .or_else(|| {
                        (!["off", "on"].contains(&view_matte.as_str())).then(|| {
                            format!("Change to Color's view matte is \"off\" or \"on\", and this is \"{view_matte}\".")
                        })
                    })
            }
            Effect::RadioWaves { profile, color, .. } => hex_fault("Radio Waves", "colour", color).or_else(|| {
                (!["square", "triangle", "sine"].contains(&profile.as_str())).then(|| {
                    format!("Radio Waves' profile is \"square\", \"triangle\" or \"sine\", and this is \"{profile}\".")
                })
            }),
            Effect::LightSweep { shape, light_color, light_reception, .. } => {
                hex_fault("Light Sweep", "light colour", light_color)
                    .or_else(|| {
                        (!["linear", "smooth", "sharp"].contains(&shape.as_str())).then(|| {
                            format!("Light Sweep's shape is \"linear\", \"smooth\" or \"sharp\", and this is \"{shape}\".")
                        })
                    })
                    .or_else(|| {
                        (!["add", "composite", "cutout"].contains(&light_reception.as_str())).then(|| {
                            format!(
                                "Light Sweep's light reception is \"add\", \"composite\" or \"cutout\", and this \
                                 is \"{light_reception}\"."
                            )
                        })
                    })
            }
            Effect::PolarCoordinates { conversion, .. } if !["rect_to_polar", "polar_to_rect"].contains(&conversion.as_str()) => {
                Some(format!(
                    "Polar Coordinates' conversion is \"rect_to_polar\" or \"polar_to_rect\", and this is \"{conversion}\"."
                ))
            }
            Effect::PolarCoordinates { shape, .. } if !["ellipse", "circle"].contains(&shape.as_str()) => {
                Some(format!("Polar Coordinates' shape is \"ellipse\" or \"circle\", and this is \"{shape}\"."))
            }
            Effect::Kaleidoscope { mode, .. } if !["mirror", "repeat"].contains(&mode.as_str()) => Some(format!(
                "Kaleidoscope's mirroring is \"mirror\" or \"repeat\", and this is \"{mode}\"."
            )),
            Effect::RoughenEdges { edge_type, .. } if !["roughen", "roughen_color"].contains(&edge_type.as_str()) => {
                Some(format!(
                    "Roughen Edges' edge type is \"roughen\" or \"roughen_color\", and this is \"{edge_type}\"."
                ))
            }
            Effect::RoughenEdges { edge_color, .. } => hex_fault("Roughen Edges", "edge colour", edge_color),
            Effect::Beam { composite, .. } if !["on", "off"].contains(&composite.as_str()) => Some(format!(
                "Beam's composite is \"on\" or \"off\", and this is \"{composite}\"."
            )),
            Effect::Beam { inside_color, outside_color, .. } => hex_fault("Beam", "inside colour", inside_color)
                .or_else(|| hex_fault("Beam", "outside colour", outside_color)),
            Effect::FourColorGradient { blending_mode, .. }
                if !["normal", "multiply", "screen", "add"].contains(&blending_mode.as_str()) =>
            {
                Some(format!(
                    "4-Color Gradient's blending mode is \"normal\", \"multiply\", \"screen\" or \"add\", \
                     and this is \"{blending_mode}\"."
                ))
            }
            Effect::FourColorGradient {
                color_1,
                color_2,
                color_3,
                color_4,
                ..
            } => [("colour 1", color_1), ("colour 2", color_2), ("colour 3", color_3), ("colour 4", color_4)]
                .into_iter()
                .find_map(|(what, c)| hex_fault("4-Color Gradient", what, c)),
            Effect::CellPattern { pattern, .. }
                if !["bubbles", "crystals", "plates", "static_plates"].contains(&pattern.as_str()) =>
            {
                Some(format!(
                    "Cell Pattern's pattern is \"bubbles\", \"crystals\", \"plates\" or \
                     \"static_plates\", and this is \"{pattern}\"."
                ))
            }
            Effect::CellPattern { invert, .. } if !["on", "off"].contains(&invert.as_str()) => Some(format!(
                "Cell Pattern's invert is \"on\" or \"off\", and this is \"{invert}\"."
            )),
            Effect::CellPattern { blend, .. }
                if !["normal", "multiply", "screen", "add"].contains(&blend.as_str()) =>
            {
                Some(format!(
                    "Cell Pattern's blend is \"normal\", \"multiply\", \"screen\" or \"add\", \
                     and this is \"{blend}\"."
                ))
            }
            Effect::CellPattern {
                dark_color,
                light_color,
                ..
            } => hex_fault("Cell Pattern", "dark colour", dark_color)
                .or_else(|| hex_fault("Cell Pattern", "light colour", light_color)),
            Effect::OpticsCompensation { reverse, .. } if !["on", "off"].contains(&reverse.as_str()) => Some(format!(
                "Optics Compensation's reverse lens distortion is \"on\" or \"off\", and this is \"{reverse}\"."
            )),
            Effect::OpticsCompensation { orientation, .. }
                if !["horizontal", "vertical", "diagonal"].contains(&orientation.as_str()) =>
            {
                Some(format!(
                    "Optics Compensation's FOV orientation is \"horizontal\", \"vertical\" or \
                     \"diagonal\", and this is \"{orientation}\"."
                ))
            }
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
            Effect::Median { operate_on_alpha, .. } if !["off", "on"].contains(&operate_on_alpha.as_str()) => Some(format!(
                "Median's operate on alpha is \"off\" or \"on\", and this is \"{operate_on_alpha}\"."
            )),
            Effect::BilateralBlur { colorize, .. } if !["off", "on"].contains(&colorize.as_str()) => Some(format!(
                "Bilateral Blur's colorize is \"off\" or \"on\", and this is \"{colorize}\"."
            )),
            Effect::LineBlur { lines_only, .. } if !["off", "on"].contains(&lines_only.as_str()) => Some(format!(
                "Line Blur's lines only is \"off\" or \"on\", and this is \"{lines_only}\"."
            )),
            Effect::HsvKey { invert, .. } if !["off", "on"].contains(&invert.as_str()) => Some(format!(
                "HSV Key's invert is \"off\" or \"on\", and this is \"{invert}\"."
            )),
            Effect::Extract { channel, .. }
                if !["luminance", "red", "green", "blue", "alpha"].contains(&channel.as_str()) =>
            {
                Some(format!(
                    "Extract's channel is \"luminance\", \"red\", \"green\", \"blue\" or \"alpha\", \
                     and this is \"{channel}\"."
                ))
            }
            Effect::Extract { invert, .. } if !["off", "on"].contains(&invert.as_str()) => Some(format!(
                "Extract's invert is \"off\" or \"on\", and this is \"{invert}\"."
            )),
            Effect::BevelAlpha { light_color, .. } => hex_fault("Bevel Alpha", "light colour", light_color),
            Effect::BevelEdges { light_color, .. } => hex_fault("Bevel Edges", "light colour", light_color),
            Effect::Paraffin { blend, .. }
                if !["normal", "multiply", "screen", "add", "overlay", "soft_light"].contains(&blend.as_str()) =>
            {
                Some(format!(
                    "Paraffin's blend is \"normal\", \"multiply\", \"screen\", \"add\", \"overlay\" \
                     or \"soft_light\", and this is \"{blend}\"."
                ))
            }
            Effect::Paraffin { color, .. } => hex_fault("Paraffin", "colour", color),
            Effect::KiraKira { shape, .. } if !["cross", "star"].contains(&shape.as_str()) => Some(format!(
                "Kira-kira's shape is \"cross\" or \"star\", and this is \"{shape}\"."
            )),
            Effect::KiraKira { color, .. } => hex_fault("Kira-kira", "colour", color),
            Effect::LightningBolt { composite, .. } if !["off", "on"].contains(&composite.as_str()) => Some(format!(
                "Lightning Bolt's composite on original is \"off\" or \"on\", and this is \"{composite}\"."
            )),
            Effect::LightningBolt { kind, .. } if !LIGHTNING_KINDS.contains(&kind.as_str()) => Some(format!(
                "Lightning Bolt's lightning type is \"direction\", \"strike\", \"breaking\", \"bouncy\", \"omni\", \"anywhere\", \"vertical\" or \"two_way\", and this is \"{kind}\"."
            )),
            Effect::LightningBolt { path, .. } if !["split", "around"].contains(&path.as_str()) => Some(format!(
                "Lightning Bolt's path at an obstacle is \"split\" or \"around\", and this is \"{path}\"."
            )),
            Effect::LightningBolt { core, .. } if !["hard", "soft"].contains(&core.as_str()) => Some(format!(
                "Lightning Bolt's core edge is \"hard\" or \"soft\", and this is \"{core}\"."
            )),
            Effect::LightningBolt { forks, .. } if !["short", "long", "full"].contains(&forks.as_str()) => Some(format!(
                "Lightning Bolt's forks are \"short\", \"long\" or \"full\", and this is \"{forks}\"."
            )),
            Effect::LightningBolt { color, glow_color, .. } => hex_fault("Lightning Bolt", "colour", color)
                .or_else(|| hex_fault("Lightning Bolt", "glow colour", glow_color)),
            Effect::CompoundBlur { layer, .. } if !layer.is_string() => Some(format!(
                "Compound Blur's layer is the name of a layer of this composition, and this is {layer}."
            )),
            Effect::CompoundBlur { fit, .. } if !["center", "stretch", "tile"].contains(&fit.as_str()) => Some(format!(
                "Compound Blur's fit is \"center\", \"stretch\" or \"tile\", and this is \"{fit}\"."
            )),
            Effect::CompoundBlur { invert, .. } if !["off", "on"].contains(&invert.as_str()) => Some(format!(
                "Compound Blur's invert is \"off\" or \"on\", and this is \"{invert}\"."
            )),
            Effect::CompoundBlur { edges: e, .. } => edges(e),
            Effect::DisplacementMap { layer, .. } if !layer.is_string() => Some(format!(
                "Displacement Map's layer is the name of a layer of this composition, and this is {layer}."
            )),
            Effect::DisplacementMap { fit, .. } if !["center", "stretch", "tile"].contains(&fit.as_str()) => Some(format!(
                "Displacement Map's fit is \"center\", \"stretch\" or \"tile\", and this is \"{fit}\"."
            )),
            Effect::DisplacementMap { horizontal: w, .. } | Effect::DisplacementMap { vertical: w, .. }
                if !["red", "green", "blue", "alpha", "luminance", "hue", "lightness", "saturation", "full", "off"].contains(&w.as_str()) =>
            {
                Some(format!(
                    "Displacement Map reads red, green, blue, alpha, luminance, hue, lightness, \
                     saturation, full or off, and this is \"{w}\"."
                ))
            }
            Effect::DisplacementMap { wrap, .. } if !["off", "on"].contains(&wrap.as_str()) => Some(format!(
                "Displacement Map's wrap is \"off\" or \"on\", and this is \"{wrap}\"."
            )),
            Effect::DisplacementMap { expand, .. } if !["off", "on"].contains(&expand.as_str()) => Some(format!(
                "Displacement Map's expand output is \"off\" or \"on\", and this is \"{expand}\"."
            )),
            Effect::GradientWipe { layer, .. } if !layer.is_string() => Some(format!(
                "Gradient Wipe's layer is the name of a layer of this composition, and this is {layer}."
            )),
            Effect::GradientWipe { fit, .. } if !["center", "stretch", "tile"].contains(&fit.as_str()) => Some(format!(
                "Gradient Wipe's fit is \"center\", \"stretch\" or \"tile\", and this is \"{fit}\"."
            )),
            Effect::GradientWipe { invert, .. } if !["off", "on"].contains(&invert.as_str()) => Some(format!(
                "Gradient Wipe's invert is \"off\" or \"on\", and this is \"{invert}\"."
            )),
            Effect::Echo { operator, .. } if !crate::layer_fx::ECHO_OPERATORS.contains(&operator.as_str()) => Some(format!(
                "Echo's operator is \"add\", \"maximum\", \"minimum\", \"screen\", \"composite_in_back\", \
                 \"composite_in_front\" or \"blend\", and this is \"{operator}\"."
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
            Effect::Snowfall { color, .. } => hex_fault("Snowfall", "colour", color),
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
        units,
    } = effect
    else {
        return None;
    };
    let hex = |c: &str| crate::selective_blur::parse_hex(c).is_some();
    Some(if !["bright", "colors"].contains(&based_on.as_str()) {
        format!("Glow is based on \"bright\" or \"colors\", and this is \"{based_on}\".")
    } else if !["classic", "after_effects"].contains(&units.as_str()) {
        format!("Glow's units are \"classic\" or \"after_effects\", and this is \"{units}\".")
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
    reach_radius(sigma_px, false)
}

/// D-321: Gaussian Blur's number as a sigma and whether its kernel takes the long reach. In
/// "blurriness" it is After Effects' Blurriness, sigma 0.3 B as lottie-web and Skia's Skottie
/// play After Effects' files; any other word is document 21's sigma.
pub fn blur_reach(number: f64, units: &str) -> (f64, bool) {
    if units == "blurriness" { (0.3 * number, true) } else { (number, false) }
}

/// D-322: a Glow's spread as a sigma and whether it takes the long reach. In "after_effects" the
/// radius is a Gaussian Blur's Blurriness (Creative COW's measurements: "Glow Radius pretty much
/// creates layer's Gaussian blur"); in "classic" it is D-89's sigma radius / 3.
pub fn glow_reach(radius: f64, units: &str) -> (f64, bool) {
    if units == "after_effects" { blur_reach(radius, "blurriness") } else { (radius / 3.0, false) }
}

/// D-328: Turbulent Displace's push in pixels. In "after_effects" it shrinks with a wave under
/// 100 pixels, this program's reading of After Effects (no source pins its own): Amount 80 at
/// Size 2 pushes 1.6 pixels, a shimmer, where D-127's "classic" pushes 80 and breaks it to dust.
pub fn turbulent_push(amount: f64, size: f64, units: &str) -> f64 {
    if units == "after_effects" { amount * size.min(100.0) / 100.0 } else { amount }
}

/// D-321: the kernel radius, `ceil(6.5 sigma)` with the long reach, else document 21's. Three
/// sigmas leave the last tap at 1% of the middle one, a straight edge an Exposure after the
/// blur can show; past 6.5 sigmas the tail weighs under 4e-11.
pub fn reach_radius(sigma_px: f64, long: bool) -> usize {
    if sigma_px.is_nan() || sigma_px <= 0.0 {
        return 0;
    }
    ((if long { 6.5 } else { 3.0 }) * sigma_px).ceil() as usize
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
    reach_weights(sigma_px, false)
}

/// D-321: [`gaussian_weights`] out to [`reach_radius`].
pub fn reach_weights(sigma_px: f64, long: bool) -> Vec<f32> {
    let radius = reach_radius(sigma_px, long);
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

/// D-327: how far Fast Box Blur's kernel reaches, `iterations` (its whole part) times
/// `ceil(radius)`, the pixels one box reaches.
///
/// ponytail: 500 by 50 reaches 25,000 pixels, and with transparent edges the layer grows that
/// much on every side; a cap on the reach is a later decision if anyone keys that high.
pub fn box_reach(radius: f64, iterations: f64) -> usize {
    (radius.ceil() as usize).saturating_mul(iterations.floor() as usize)
}

/// D-327: one box of weight 1 for `floor(radius)` pixels each side and the part past whole at
/// the next, over its sum, convolved with itself `iterations` times, index 0 being
/// [`box_reach`] pixels before the centre. Each pass is a running sum, so a long kernel costs
/// its length a pass.
pub fn box_weights(radius: f64, iterations: f64) -> Vec<f32> {
    let reach = box_reach(radius, iterations);
    if reach == 0 {
        return vec![1.0];
    }
    let (k, f) = (radius.floor(), radius - radius.floor());
    let (k, total) = (k as isize, 2.0 * k + 1.0 + 2.0 * f);
    let n = 2 * reach + 1;
    let mut line = vec![0.0f64; n];
    line[reach] = 1.0;
    // `sums[i]` is the sum of `line[..i]`, so `line[a..b]` sums to `sums[b] - sums[a]`.
    let mut sums = vec![0.0f64; n + 1];
    for _ in 0..iterations.floor() as usize {
        for (i, v) in line.iter().enumerate() {
            sums[i + 1] = sums[i] + v;
        }
        let at = |i: isize| if (0..n as isize).contains(&i) { line[i as usize] } else { 0.0 };
        line = (0..n as isize)
            .map(|x| {
                let (a, b) = ((x - k).max(0) as usize, ((x + k + 1) as usize).min(n));
                (sums[b] - sums[a] + f * (at(x - k - 1) + at(x + k + 1))) / total
            })
            .collect();
    }
    line.into_iter().map(|v| v as f32).collect()
}

impl Effect {
    /// D-361/D-362: a Spin & Zoom Blur's or Fast Zoom Blur's samples, and whether they turn;
    /// `None` for any other effect, or a word that is not one.
    pub(crate) fn sweep(&self) -> Option<(bool, crate::blurs::Sweep)> {
        use crate::blurs::{Sweep, Weigh};
        let sweep = |from, weigh, density| Sweep { from, weigh, density };
        match self {
            Effect::SpinZoomBlur { kind, amount, quality, .. } => {
                let (spin, from, weigh) = match kind.as_str() {
                    "straight_zoom" => (false, 0.0, Weigh::Even),
                    "fading_zoom" => (false, 0.0, Weigh::Fading),
                    "centered_zoom" => (false, amount / 2.0, Weigh::Even),
                    "rotate" => (true, 0.0, Weigh::Even),
                    "rotate_fading" => (true, 0.0, Weigh::Fading),
                    "scratch" => (true, amount / 2.0, Weigh::Even),
                    _ => return None,
                };
                Some((spin, sweep(from, weigh, quality / 50.0)))
            }
            Effect::FastZoomBlur { zoom, .. } => {
                let weigh = match zoom.as_str() {
                    "standard" => Weigh::Fading,
                    "brightest" => Weigh::Brightest,
                    "darkest" => Weigh::Darkest,
                    _ => return None,
                };
                Some((false, sweep(0.0, weigh, 1.0)))
            }
            _ => None,
        }
    }
}

/// D-95: a Radial Blur's centre in the pixels of a buffer `w` by `h`, from its share of the
/// drawing's own size, with the drawing's corner at `(ox, oy)` in it after the effects above it
/// grew it.
pub(crate) fn radial_center(center: [f64; 2], (w, h): (usize, usize), (ox, oy): (usize, usize)) -> (f64, f64) {
    let (w0, h0) = (w - 2 * ox, h - 2 * oy);
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
    apply_stack_at(source, stack, (0, 0), Bits::Linear, report)
}

/// D-330 and D-333: what a composition's working depth does to a layer's effects.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Bits {
    /// Display and Float: the effects as written.
    Linear,
    /// D-330, 8 bpc (After Effects).
    Eight,
    /// D-333, 32 bpc (After Effects).
    Ae32,
}

/// B-65: [`apply_stack`] on a buffer the effects before `stack` have already grown, the drawing's
/// corner at `origin`, as Gradient, Noise and Chromatic Aberration read it. Returns the corner
/// after `stack`, `origin` included. `bits`: D-330, a composition in 8 bpc, whose blurs average
/// display values and whose every effect's result is held to 8 bits; D-333, one in 32 bpc (After
/// Effects), whose blurs average display values and whose Exposure works through a 2.2 curve.
pub(crate) fn apply_stack_at(
    source: &mut WorkingBuffer,
    stack: &[EffectInstance],
    origin: (usize, usize),
    bits: Bits,
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
        // D-202: what the effect is given, kept only when its result is to be mixed with it.
        let given = (instance.mix < 100.0).then(|| (source.clone(), ox, oy));
        // D-330, D-333: After Effects in 8 bpc and in its own 32 bpc has no linear working space,
        // so its blurs average the display values; in 32 bpc they are not held at white. D-337:
        // Solid Composite and Glow are laid on there too.
        let top = if bits == Bits::Eight { 1.0 } else { f32::INFINITY };
        let display = bits != Bits::Linear
            && instance.is_valid()
            && matches!(
                instance.effect,
                Effect::GaussianBlur { .. }
                    | Effect::FastBoxBlur { .. }
                    | Effect::DirectionalBlur { .. }
                    | Effect::RadialBlur { .. }
                    | Effect::CrossBlur { .. }
                    | Effect::SpinZoomBlur { .. }
                    | Effect::FastZoomBlur { .. }
                    | Effect::SolidComposite { .. }
                    | Effect::Glow { .. }
            );
        if display {
            encode(source, true, top);
        }
        match &instance.effect {
            Effect::Unsupported { .. } => {
                report(at, instance, Bypassed::NotImplemented);
                continue;
            }
            _ if !instance.is_valid() => {
                report(at, instance, Bypassed::InvalidParameter);
                continue;
            }
            // One stage per kind of effect rather than one for the stack (P-11). The three are
            // disjoint and none of them nests, so `src/perf.rs`'s promise that the table can be
            // summed still holds; what they buy is the ranking P-11's entry says it needs before
            // it decides which effect is worth caching.
            Effect::Exposure { stops, offset, gamma, bypass } => {
                crate::perf::time(crate::perf::Stage::EffectExposure, || {
                    exposure(source, *stops, *offset, *gamma, bypass == "on", bits)
                })
            }
            Effect::Tint { color, amount } => {
                crate::perf::time(crate::perf::Stage::EffectTint, || {
                    tint(source, *color, *amount)
                })
            }
            Effect::GaussianBlur { sigma_px, edges, dimensions, units } => {
                let axes = (dimensions != "vertical", dimensions != "horizontal");
                let (sigma, long) = blur_reach(*sigma_px, units);
                let taps = reach_weights(sigma, long);
                let r = crate::perf::time(crate::perf::Stage::EffectBlur, || {
                    if edges == "repeat" {
                        held_blur_axes(source, &taps, axes);
                        0
                    } else {
                        blur_axes(source, &taps, axes)
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
                units,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectGlow, || {
                    let mut g = crate::glow::settings(
                        based_on, *threshold, colors, *tolerance, *radius, *intensity,
                        operation, tint, units,
                    );
                    g.display = display;
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
                let c = radial_center(*center, (source.width(), source.height()), (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectRadial, || {
                    crate::blurs::radial_blur(source, kind == "spin", *amount, c, edges == "repeat", None)
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
                alpha,
            } => crate::perf::time(crate::perf::Stage::EffectCurves, || {
                crate::grade::curves(source, master, [red, green, blue], alpha)
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
                ranges,
            } => crate::perf::time(crate::perf::Stage::EffectHueSaturation, || {
                crate::grade::hue_saturation(source, *hue, *saturation, *lightness, ranges)
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
                    start: radial_center(*start, (source.width(), source.height()), (ox, oy)),
                    end: radial_center(*end, (source.width(), source.height()), (ox, oy)),
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
            // D-211: the light is in per cent of the drawing's own box, however an effect above
            // grew it, and the layer grows to hold the shadow.
            Effect::RadialShadow {
                color,
                opacity,
                light,
                distance,
                softness,
                render,
                color_influence,
                shadow_only,
            } => {
                let (gx, gy) = crate::perf::time(crate::perf::Stage::EffectRadialShadow, || {
                    crate::layer_fx::radial_shadow(
                        source,
                        encoded(color),
                        *opacity,
                        *light,
                        *distance,
                        *softness,
                        render == "glass_edge",
                        *color_influence,
                        shadow_only == "on",
                        (ox, oy),
                    )
                });
                ox += gx;
                oy += gy;
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
                layer,
                channel,
                focal_distance,
                invert,
                map,
                ..
            } => {
                // D-359: a blur layer named and not read this frame (missing, or a circle) is
                // drawn without the effect, as its diagnostic says.
                let named = layer.as_str().is_some_and(|l| !l.is_empty());
                if !named || map.is_some() {
                    let iris = crate::layer_fx::Iris {
                        blades: crate::layer_fx::blades(iris).unwrap_or(0),
                        roundness: *roundness,
                        rotation: *rotation,
                        aspect: *aspect,
                        gain: *highlight_gain,
                        threshold: *highlight_threshold,
                    };
                    let r = crate::perf::time(crate::perf::Stage::EffectLensBlur, || match map {
                        Some(m) => {
                            let read = crate::layer_fx::LensMap {
                                alpha: channel == "alpha",
                                focus: *focal_distance / 255.0,
                                invert: invert == "on",
                            };
                            crate::layer_fx::lens_blur_map(source, *radius, edges == "repeat", &iris, &m.0, (ox, oy), &read)
                        }
                        None => crate::layer_fx::lens_blur(source, *radius, edges == "repeat", &iris),
                    });
                    ox += r;
                    oy += r;
                }
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
                let c = radial_center(*center, (source.width(), source.height()), (ox, oy));
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
                    exposure(source, stops, 0.0, 1.0, false, Bits::Linear)
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
                    (source.width(), source.height()),
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
                displacement,
                pinning,
                units,
            } => {
                let z = depth(*evolution, *speed, *frame);
                let r = crate::perf::time(crate::perf::Stage::EffectTurbulentDisplace, || {
                    crate::layer_fx::turbulent_displace(
                        source,
                        turbulent_push(*amount, *size, units),
                        *size,
                        complexity.floor() as usize,
                        *seed,
                        z,
                        edges == "repeat",
                        (ox, oy),
                        (displacement, pinning == "all"),
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
                fractal_type,
                noise_type,
                invert,
                offset,
                scale_width,
                scale_height,
                cycle,
                frame,
                float,
            } => {
                let f = crate::grade::Fractal {
                    size: size * (scale_width / 100.0),
                    size_y: size * (scale_height / 100.0),
                    offset: *offset,
                    invert: invert == "on",
                    look: fractal_look(fractal_type, noise_type, *cycle),
                    octaves: complexity.floor() as usize,
                    seed: *seed,
                    z: depth(*evolution, *speed, *frame),
                    contrast: *contrast,
                    brightness: *brightness,
                    colors: [encoded(dark_color), encoded(light_color)],
                    opacity: *opacity,
                    blend: blend.clone(),
                    float: *float,
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
                preserve_luminosity,
            } => crate::perf::time(crate::perf::Stage::EffectColorBalance, || {
                let tone = |t: &[f64]| [t[0], t[1], t[2]];
                crate::grade::color_balance(
                    source,
                    tone(shadows),
                    tone(midtones),
                    tone(highlights),
                    preserve_luminosity == "on",
                )
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
            Effect::Extract {
                channel,
                black_point,
                white_point,
                black_softness,
                white_softness,
                invert,
            } => crate::perf::time(crate::perf::Stage::EffectExtract, || {
                crate::grade::extract(
                    source,
                    channel,
                    [*black_point, *white_point, *black_softness, *white_softness],
                    invert == "on",
                )
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
            Effect::ChangeToColor {
                from,
                to,
                change,
                change_by,
                hue_tolerance,
                lightness_tolerance,
                saturation_tolerance,
                softness,
                view_matte,
            } => crate::perf::time(crate::perf::Stage::EffectChangeToColor, || {
                let c = crate::grade::ChangeToColor {
                    from: encoded(from),
                    to: encoded(to),
                    change,
                    transforming: change_by == "transforming",
                    tolerances: [*hue_tolerance, *lightness_tolerance, *saturation_tolerance],
                    softness: *softness,
                    matte: view_matte == "on",
                };
                crate::grade::change_to_color(source, &c)
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
            Effect::BevelAlpha {
                edge_thickness,
                light_angle,
                light_color,
                light_intensity,
            } => crate::perf::time(crate::perf::Stage::EffectBevelAlpha, || {
                let light = encoded(light_color).map(crate::grade::to_linear);
                crate::layer_fx::bevel_alpha(source, *edge_thickness, *light_angle, light, *light_intensity)
            }),
            Effect::BevelEdges {
                edge_thickness,
                light_angle,
                light_color,
                light_intensity,
            } => crate::perf::time(crate::perf::Stage::EffectBevelEdges, || {
                let light = encoded(light_color).map(crate::grade::to_linear);
                crate::layer_fx::bevel_edges(source, *edge_thickness, *light_angle, light, *light_intensity)
            }),
            // D-214: the blocks are the drawing's own, however an effect above grew it.
            Effect::BlockDissolve {
                completion,
                block_width,
                block_height,
                feather,
            } => crate::perf::time(crate::perf::Stage::EffectBlockDissolve, || {
                crate::layer_fx::block_dissolve(source, *completion, *block_width, *block_height, *feather, (ox, oy))
            }),
            Effect::ShiftChannels {
                take_alpha,
                take_red,
                take_green,
                take_blue,
            } => crate::perf::time(crate::perf::Stage::EffectShiftChannels, || {
                crate::grade::shift_channels(source, [take_red, take_green, take_blue, take_alpha].map(|w| w.as_str()))
            }),
            // D-336: the map compose read for this frame, if a layer is named, is the height.
            Effect::VectorBlur { kind, amount, angle_offset, ridge_smoothness, property, map_softness, map, .. } => {
                crate::perf::time(crate::perf::Stage::EffectVectorBlur, || {
                    crate::layer_fx::vector_blur(
                        source,
                        map.as_ref().map(|m| (&*m.0, (ox, oy))),
                        (kind, property),
                        [*amount, *angle_offset, *ridge_smoothness, *map_softness],
                    )
                })
            }
            // D-317: the map compose read for this frame, if a layer is named, is the bump.
            Effect::Glass {
                property,
                softness,
                height,
                displacement,
                light_angle,
                light_color,
                light_intensity,
                map,
                ..
            } => crate::perf::time(crate::perf::Stage::EffectGlass, || {
                let light = encoded(light_color).map(crate::grade::to_linear);
                crate::layer_fx::glass(
                    source,
                    map.as_ref().map(|m| (&*m.0, (ox, oy))),
                    property,
                    (*softness, *height, *displacement),
                    (*light_angle, light, *light_intensity / 100.0),
                )
            }),
            // D-316: the map compose read for this frame, if a layer is named, adds to the phase.
            Effect::Colorama {
                get_phase,
                phase_shift,
                cycle_repetitions,
                stops,
                color_1,
                color_2,
                color_3,
                color_4,
                color_5,
                blend_with_original,
                map,
                ..
            } => crate::perf::time(crate::perf::Stage::EffectColorama, || {
                let ring = [color_1, color_2, color_3, color_4, color_5].map(|c| encoded(c));
                crate::grade::colorama(
                    source,
                    map.as_ref().map(|m| (&*m.0, (ox, oy))),
                    get_phase,
                    (*phase_shift, *cycle_repetitions),
                    &ring[..(stops.floor() as usize).clamp(2, 5)],
                    *blend_with_original,
                )
            }),
            Effect::SolidComposite { source_opacity, color, opacity, blend } => {
                crate::perf::time(crate::perf::Stage::EffectSolidComposite, || {
                    solid_composite(source, *source_opacity, encoded(color), *opacity, blend, display)
                })
            }
            Effect::ChannelBlur {
                red_blurriness,
                green_blurriness,
                blue_blurriness,
                alpha_blurriness,
                edges,
                dimensions,
            } => {
                let r = crate::perf::time(crate::perf::Stage::EffectChannelBlur, || {
                    channel_blur(
                        source,
                        [*red_blurriness, *green_blurriness, *blue_blurriness, *alpha_blurriness],
                        edges == "repeat",
                        (dimensions != "vertical", dimensions != "horizontal"),
                    )
                });
                ox += r;
                oy += r;
            }
            Effect::FastBoxBlur { radius, iterations, edges, dimensions } => {
                let taps = box_weights(*radius, *iterations);
                let axes = (dimensions != "vertical", dimensions != "horizontal");
                let r = crate::perf::time(crate::perf::Stage::EffectFastBoxBlur, || {
                    if edges == "repeat" {
                        held_blur_axes(source, &taps, axes);
                        0
                    } else {
                        blur_axes(source, &taps, axes)
                    }
                });
                ox += r;
                oy += r;
            }
            Effect::FindEdges { invert, amount } => crate::perf::time(crate::perf::Stage::EffectFindEdges, || {
                crate::layer_fx::find_edges(source, invert == "on", *amount)
            }),
            Effect::Sharpen { amount, radius, threshold } => crate::perf::time(crate::perf::Stage::EffectSharpen, || {
                crate::layer_fx::sharpen(source, *amount, *radius, *threshold)
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
                let c = radial_center(*center, (source.width(), source.height()), (ox, oy));
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
                let c = radial_center(*center, (source.width(), source.height()), (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectTwirl, || {
                    crate::layer_fx::twirl(source, *angle, *radius, c)
                })
            }
            // D-152: the centre is a share of the drawing's own size, as Radial Blur's is.
            Effect::Bulge {
                center,
                radius,
                height,
                vertical_radius,
                taper_radius,
            } => {
                let c = radial_center(*center, (source.width(), source.height()), (ox, oy));
                crate::perf::time(crate::perf::Stage::EffectBulge, || {
                    crate::layer_fx::bulge(source, [*radius, *vertical_radius, *taper_radius], *height, c)
                })
            }
            // D-153: the centre is a share of the drawing's own size, as Radial Blur's is.
            Effect::Mirror { center, angle } => {
                let c = radial_center(*center, (source.width(), source.height()), (ox, oy));
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
                tile_center,
                tile_width,
                tile_height,
            } => {
                let (gx, gy) = crate::perf::time(crate::perf::Stage::EffectMotionTile, || {
                    crate::layer_fx::motion_tile(
                        source,
                        (*output_width, *output_height),
                        mirror == "on",
                        (*tile_center, *tile_width, *tile_height),
                    )
                });
                ox += gx;
                oy += gy;
            }
            // D-201: bent round the drawing's own middle, however an effect above grew it; the
            // layer never grows.
            Effect::PolarCoordinates { interpolation, conversion, shape } => {
                crate::perf::time(crate::perf::Stage::EffectPolarCoordinates, || {
                    crate::layer_fx::polar_coordinates(source, *interpolation, conversion == "rect_to_polar", shape == "circle", (ox, oy))
                })
            }
            // D-203: neither grows the layer.
            Effect::Median { radius, operate_on_alpha } => crate::perf::time(crate::perf::Stage::EffectMedian, || {
                crate::median::median(source, *radius, operate_on_alpha == "on")
            }),
            Effect::SmartBlur { radius, threshold } => crate::perf::time(crate::perf::Stage::EffectSmartBlur, || {
                crate::median::smart_blur(source, *radius, *threshold)
            }),
            Effect::BilateralBlur { radius, threshold, colorize } => crate::perf::time(crate::perf::Stage::EffectBilateralBlur, || {
                crate::median::bilateral_blur(source, *radius, *threshold, colorize == "on")
            }),
            Effect::CrossBlur { radius_x, radius_y, mode, edges } => {
                let mode = CROSS_MODES.iter().position(|m| m == mode).unwrap_or(0);
                let r = crate::perf::time(crate::perf::Stage::EffectCrossBlur, || {
                    crate::blurs::cross_blur(source, *radius_x, *radius_y, mode, edges == "repeat")
                });
                ox += r;
                oy += r;
            }
            // D-95: the centre as Radial Blur's.
            Effect::SpinZoomBlur { amount, center, .. } | Effect::FastZoomBlur { amount, center, .. } => {
                let c = radial_center(*center, (source.width(), source.height()), (ox, oy));
                let stage = if matches!(instance.effect, Effect::SpinZoomBlur { .. }) {
                    crate::perf::Stage::EffectSpinZoomBlur
                } else {
                    crate::perf::Stage::EffectFastZoomBlur
                };
                if let Some((spin, sweep)) = instance.effect.sweep() {
                    crate::perf::time(stage, || crate::blurs::radial_blur(source, spin, *amount, c, false, Some(sweep)))
                }
            }
            // D-204: the planes are fixed to the drawing's own space; it grows nothing.
            Effect::Snowfall {
                color,
                density,
                spacing,
                size,
                depth,
                speed,
                wind,
                wiggle,
                period,
                seed,
                opacity,
                frame,
            } => {
                let numbers = [*density, *spacing, *size, *depth, *speed, *wind, *wiggle, *period, *seed, *opacity];
                crate::perf::time(crate::perf::Stage::EffectSnowfall, || {
                    crate::layer_fx::snowfall(source, encoded(color).map(crate::grade::to_linear), numbers, *frame, (ox, oy))
                })
            }
            // D-206: the noise is the drawing's own, however an effect above grew it; the layer
            // never grows.
            Effect::RoughenEdges {
                edge_type,
                edge_color,
                border,
                size,
                complexity,
                evolution,
                speed,
                seed,
                frame,
            } => {
                let color = (edge_type == "roughen_color").then(|| encoded(edge_color).map(crate::grade::to_linear));
                let numbers = [*border, *size, complexity.floor(), *seed, depth(*evolution, *speed, *frame)];
                crate::perf::time(crate::perf::Stage::EffectRoughenEdges, || {
                    crate::layer_fx::roughen_edges(source, color, numbers, (ox, oy))
                })
            }
            // D-207: the points are the drawing's own, however an effect above grew it; the layer
            // never grows.
            Effect::Beam {
                start,
                end,
                length,
                time,
                start_thickness,
                end_thickness,
                softness,
                inside_color,
                outside_color,
                composite,
            } => crate::perf::time(crate::perf::Stage::EffectBeam, || {
                let colours = [encoded(inside_color), encoded(outside_color)].map(|c| c.map(crate::grade::to_linear));
                let ends = [radial_center(*start, (source.width(), source.height()), (ox, oy)), radial_center(*end, (source.width(), source.height()), (ox, oy))];
                let numbers = [*length, *time, *start_thickness, *end_thickness, *softness];
                crate::layer_fx::beam(source, ends, numbers, colours, composite == "off")
            }),
            // D-208: the points are the drawing's own, however an effect above grew it; the layer
            // never grows.
            Effect::FourColorGradient {
                point_1,
                point_2,
                point_3,
                point_4,
                color_1,
                color_2,
                color_3,
                color_4,
                blend,
                opacity,
                blending_mode,
            } => crate::perf::time(crate::perf::Stage::EffectFourColorGradient, || {
                let points = [point_1, point_2, point_3, point_4].map(|p| radial_center(*p, (source.width(), source.height()), (ox, oy)));
                let colors = [color_1, color_2, color_3, color_4].map(|c| encoded(c));
                crate::grade::four_color_gradient(source, points, colors, *blend, *opacity, blending_mode)
            }),
            // D-209: the cells are the drawing's own, however an effect above grew it.
            Effect::CellPattern {
                pattern,
                invert,
                contrast,
                disperse,
                size,
                evolution,
                seed,
                dark_color,
                light_color,
                opacity,
                blend,
            } => crate::perf::time(crate::perf::Stage::EffectCellPattern, || {
                crate::grade::cell_pattern(
                    source,
                    pattern,
                    invert == "on",
                    [*contrast, *disperse, *size, *evolution, *seed, *opacity],
                    [encoded(dark_color), encoded(light_color)],
                    blend,
                    (ox, oy),
                )
            }),
            // D-205: round the drawing's own centre, however an effect above grew it; the layer
            // never grows.
            Effect::Kaleidoscope {
                segments,
                rotation,
                size,
                center,
                mode,
            } => crate::perf::time(crate::perf::Stage::EffectKaleidoscope, || {
                crate::layer_fx::kaleidoscope(source, [*segments, *rotation, *size], *center, mode == "mirror", (ox, oy))
            }),
            // D-210: the lens is the drawing's own, however an effect above grew it; the layer
            // never grows.
            Effect::OpticsCompensation {
                field_of_view,
                reverse,
                orientation,
                center,
            } => crate::perf::time(crate::perf::Stage::EffectOpticsCompensation, || {
                crate::layer_fx::optics_compensation(source, *field_of_view, reverse == "on", orientation, *center, (ox, oy))
            }),
            // D-198: the corners are in per cent of the drawing's own box, however an effect
            // above grew it, and the layer grows so none is cut off.
            Effect::CornerPin {
                upper_left,
                upper_right,
                lower_left,
                lower_right,
            } => {
                let (gx, gy) = crate::perf::time(crate::perf::Stage::EffectCornerPin, || {
                    crate::layer_fx::corner_pin(source, [*upper_left, *upper_right, *lower_left, *lower_right], (ox, oy))
                });
                ox += gx;
                oy += gy;
            }
            // D-200: the producer is in per cent of the drawing's own box, however an effect above
            // grew it; the waves are painted inside the layer, which never grows.
            Effect::RadioWaves {
                producer_point,
                sides,
                interval,
                expansion,
                orientation,
                direction,
                velocity,
                spin,
                lifespan,
                opacity,
                fade_in_time,
                fade_out_time,
                start_width,
                end_width,
                profile,
                color,
                frame,
            } => crate::perf::time(crate::perf::Stage::EffectRadioWaves, || {
                let s = crate::layer_fx::RadioWaves {
                    producer: radial_center(*producer_point, (source.width(), source.height()), (ox, oy)),
                    sides: *sides,
                    interval: *interval,
                    expansion: *expansion,
                    orientation: *orientation,
                    direction: *direction,
                    velocity: *velocity,
                    spin: *spin,
                    lifespan: *lifespan,
                    opacity: *opacity,
                    fade_in: *fade_in_time,
                    fade_out: *fade_out_time,
                    widths: [*start_width, *end_width],
                    profile,
                    color: encoded(color).map(crate::grade::to_linear),
                    frame: *frame,
                };
                crate::layer_fx::radio_waves(source, &s)
            }),
            // D-199: the centre is in per cent of the drawing's own box, however an effect above
            // grew it.
            Effect::LightSweep {
                center,
                direction,
                shape,
                width,
                sweep_intensity,
                edge_intensity,
                edge_thickness,
                light_color,
                light_reception,
            } => crate::perf::time(crate::perf::Stage::EffectLightSweep, || {
                let s = crate::layer_fx::LightSweep {
                    center: *center,
                    direction: *direction,
                    shape,
                    width: *width,
                    sweep: *sweep_intensity,
                    edge: *edge_intensity,
                    thickness: *edge_thickness,
                    color: encoded(light_color),
                    reception: light_reception,
                };
                crate::layer_fx::light_sweep(source, &s, (ox, oy))
            }),
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
                let c = radial_center(*center, (source.width(), source.height()), (ox, oy));
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
            Effect::Paraffin {
                color,
                direction,
                spread,
                opacity,
                blend,
            } => crate::perf::time(crate::perf::Stage::EffectParaffin, || {
                crate::grade::paraffin(source, encoded(color), *direction, *spread, *opacity, blend)
            }),
            // D-186: the stars grow the layer by the size, rounded up, unless nothing is added.
            Effect::KiraKira {
                threshold,
                spacing,
                density,
                size,
                shape,
                angle,
                twinkle,
                period,
                seed,
                opacity,
                color,
                frame,
            } => {
                let numbers = [*threshold, *spacing, *density, *size, *angle, *twinkle, *period, *seed, *opacity];
                let r = crate::perf::time(crate::perf::Stage::EffectKiraKira, || {
                    let c = encoded(color).map(crate::grade::to_linear);
                    crate::layer_fx::kira_kira(source, c, numbers, shape == "star", *frame, (ox, oy))
                });
                ox += r;
                oy += r;
            }
            // D-190: drawn inside the layer, which never grows.
            Effect::LightningBolt {
                start,
                end,
                jagged,
                detail,
                branches,
                width,
                glow,
                opacity,
                hold,
                seed,
                color,
                glow_color,
                composite,
                kind,
                turbulence,
                decay,
                conductivity,
                obstacle,
                path,
                core,
                forks,
                frame,
            } => crate::perf::time(crate::perf::Stage::EffectLightningBolt, || {
                // D-324: Alpha Obstacle reads the layer as it is, before Composite on Original.
                // D-329: below 0 the mirror, the clear parts blocking.
                let edge = 1.0 - *obstacle / 100.0;
                let blocks: Vec<bool> = match *obstacle {
                    a if a > 0.0 => source.data().chunks_exact(4).map(|p| p[3] as f64 > edge).collect(),
                    a if a < 0.0 => source.data().chunks_exact(4).map(|p| (p[3] as f64) < -a / 100.0).collect(),
                    _ => Vec::new(),
                };
                // D-300: off, the layer's own picture goes and the bolt is all there is.
                if composite == "off" {
                    source.data_mut().fill(0.0);
                }
                let colours = [encoded(color), encoded(glow_color)].map(|c| c.map(crate::grade::to_linear));
                let size = (source.width(), source.height());
                let ends = [radial_center(*start, size, (ox, oy)), radial_center(*end, size, (ox, oy)), radial_center([start[0], 100.0], size, (ox, oy))];
                let numbers = [*jagged, *detail, *branches, *width, *glow, *opacity, *hold, *seed];
                crate::layer_fx::lightning_bolt(source, ends, numbers, (kind, [*turbulence, *decay, *conductivity], forks.as_str()), (&blocks, *obstacle < 0.0, path == "around"), (colours, core == "soft"), *frame)
            }),
            // D-191: the map compose read for this frame; with none, nothing is blurred.
            Effect::CompoundBlur { max_blur, invert, edges, map, .. } => {
                if let Some(map) = map {
                    crate::perf::time(crate::perf::Stage::EffectCompoundBlur, || {
                        crate::layer_fx::compound_blur(source, &map.0, (ox, oy), *max_blur, invert == "on", edges == "repeat")
                    })
                }
            }
            // D-193: the map compose read for this frame; with none, nothing moves. D-315: with
            // Expand Output the layer grows first.
            Effect::DisplacementMap { horizontal, max_horizontal, vertical, max_vertical, wrap, expand, map, .. } => {
                if let Some(map) = map {
                    let r = crate::perf::time(crate::perf::Stage::EffectDisplacementMap, || {
                        crate::layer_fx::displacement_map(
                            source,
                            &map.0,
                            (ox, oy),
                            [horizontal, vertical],
                            [*max_horizontal, *max_vertical],
                            (wrap == "on", expand == "on"),
                        )
                    });
                    ox += r;
                    oy += r;
                }
            }
            // D-194: the map compose read for this frame; with none, nothing is wiped.
            Effect::GradientWipe { completion, softness, invert, map, .. } => {
                if let Some(map) = map {
                    crate::perf::time(crate::perf::Stage::EffectGradientWipe, || {
                        crate::layer_fx::gradient_wipe(source, &map.0, (ox, oy), *completion, *softness, invert == "on")
                    })
                }
            }
            // D-195: the echoes compose drew for this frame replace the picture, laid on the
            // drawing; on an adjustment layer there are none, and nothing changes.
            Effect::Echo { picture, .. } => {
                if let Some(picture) = picture {
                    crate::perf::time(crate::perf::Stage::EffectEcho, || {
                        crate::layer_fx::lay(source, &picture.0, (ox, oy))
                    })
                }
            }
            // D-348: the pass compose read from the layer's file for this frame; with none, the
            // layer is left as it is (EFFECT_CHANNEL_MISSING was said when it was looked for).
            Effect::PassExtract { black_point, white_point, invert, clamp, channels, .. } => {
                if let Some(pass) = channels {
                    crate::perf::time(crate::perf::Stage::EffectPassExtract, || {
                        crate::layer_fx::pass_extract(source, &pass.0, (ox, oy), *black_point, *white_point, invert == "on", clamp == "on")
                    })
                }
            }
            Effect::DepthKey { depth, feather, invert, channels } => {
                if let Some(pass) = channels {
                    crate::perf::time(crate::perf::Stage::EffectDepthKey, || {
                        crate::layer_fx::depth_key(source, &pass.0, (ox, oy), *depth, *feather, invert == "on")
                    })
                }
            }
            // D-349: as Depth Key, with the ids compose read.
            Effect::IdKey { id, feather, invert, channels, .. } => {
                if let Some(pass) = channels {
                    crate::perf::time(crate::perf::Stage::EffectIdKey, || {
                        crate::layer_fx::id_key(source, &pass.0, (ox, oy), *id, *feather, invert == "on")
                    })
                }
            }
            // D-347: the moments compose drew for this frame replace the picture, as Echo's do.
            Effect::MomentMap { picture, .. } => {
                if let Some(picture) = picture {
                    crate::perf::time(crate::perf::Stage::EffectMomentMap, || {
                        crate::layer_fx::lay(source, &picture.0, (ox, oy))
                    })
                }
            }
            // D-196: the holding was done where the layer's content was resolved.
            Effect::PosterizeTime { .. } => {}
            // D-350: the letters were moved where the text was drawn.
            Effect::TextAnimator { .. } => {}
            // D-351: by the whole picture's statistics, or the frames' compose added up.
            Effect::AutoTone { .. } => crate::perf::time(crate::perf::Stage::EffectAutoTone, || {
                crate::frame_stats::apply(source, &instance.effect)
            }),
            Effect::SpreadTones { .. } => crate::perf::time(crate::perf::Stage::EffectSpreadTones, || {
                crate::frame_stats::apply(source, &instance.effect)
            }),
            // D-352: on the whole layer, which never grows.
            Effect::MatteChoker {
                geometric_softness_1,
                choke_1,
                gray_level_softness_1,
                geometric_softness_2,
                choke_2,
                gray_level_softness_2,
                iterations,
            } => crate::perf::time(crate::perf::Stage::EffectMatteChoker, || {
                let stages = [
                    [*geometric_softness_1, *choke_1, *gray_level_softness_1],
                    [*geometric_softness_2, *choke_2, *gray_level_softness_2],
                ];
                crate::matte_refine::matte_choker(source, stages, *iterations)
            }),
            Effect::RefineMatte {
                kind,
                edge_radius,
                view_edge_region,
                feather,
                contrast,
                shift_edge,
                decontaminate,
                decontamination_amount,
                decontamination_radius,
                view_decontamination_map,
            } => crate::perf::time(crate::perf::Stage::EffectRefineMatte, || {
                let s = crate::matte_refine::Refine {
                    soft: *kind == "soft",
                    edge_radius: *edge_radius,
                    view_edge_region,
                    feather: *feather,
                    contrast: *contrast,
                    shift_edge: *shift_edge,
                    decontaminate,
                    decontamination_amount: *decontamination_amount,
                    decontamination_radius: *decontamination_radius,
                    view_decontamination_map,
                };
                crate::matte_refine::refine_matte(source, &s)
            }),
            // D-356: the paths compose found for this frame, in the drawing's own space however an
            // effect above grew it; with none, the layer is left as it is (EFFECT_PATH_MISSING was
            // said when they were looked for). The layer never grows.
            Effect::Stroke { all_masks, stroke_sequentially, color, brush_size, brush_hardness, opacity, start, end, spacing, paint_style, paths, .. } => {
                if let Some(paths) = paths {
                    crate::perf::time(crate::perf::Stage::EffectStroke, || {
                        let sequential = all_masks == "on" && stroke_sequentially == "on";
                        let runs = crate::along::stroke_runs(paths, (ox, oy), [*start, *end, *spacing, *brush_size], sequential);
                        crate::along::path_stroke(source, &runs, [*brush_size, *brush_hardness, *opacity], encoded(color).map(crate::grade::to_linear), paint_style)
                    })
                }
            }
            // D-353: the layer grows by the plan's reach on every side.
            Effect::SoftGlow {
                threshold_mode,
                threshold,
                threshold_smooth,
                saturation_bias,
                radius,
                exposure,
                aspect_ratio,
                aspect_angle,
                operation,
                source_opacity,
                unmult,
                ..
            } => {
                let s = crate::soft_glow::Settings {
                    luminance: threshold_mode == "luminance",
                    threshold: *threshold,
                    smooth: *threshold_smooth,
                    bias: *saturation_bias,
                    radius: *radius,
                    exposure: *exposure,
                    aspect: *aspect_ratio,
                    angle: *aspect_angle,
                    screen: operation == "screen",
                    opacity: *source_opacity,
                    unmult: unmult == "on",
                };
                let r = crate::perf::time(crate::perf::Stage::EffectSoftGlow, || crate::soft_glow::soft_glow(source, &s));
                ox += r;
                oy += r;
            }
        }
        if display {
            encode(source, false, top);
        }
        if let Some((given, gx, gy)) = given {
            mix_back(source, &given, (ox - gx, oy - gy), instance.mix / 100.0);
        }
        if bits == Bits::Eight {
            eight_bits(source);
        }
    }
    (ox, oy)
}

/// D-330: linear premultiplied to display premultiplied, the straight colour held to 0..`top`
/// through the sRGB curve (`forward`), or back.
fn encode(buffer: &mut WorkingBuffer, forward: bool, top: f32) {
    use crate::color::{linear_to_srgb, srgb_to_linear};
    buffer.data_mut().par_chunks_mut(4).for_each(|p| {
        let a = p[3];
        if a <= 0.0 {
            p.fill(0.0);
            return;
        }
        for c in &mut p[..3] {
            let s = (*c / a).clamp(0.0, top);
            *c = if forward { linear_to_srgb(s) } else { srgb_to_linear(s) } * a;
        }
    });
}

/// D-330: each pixel held to what 8 bits keep, its alpha and its straight display colour each
/// rounded to a 255th. A pixel whose alpha rounds to 0 is clear.
fn eight_bits(buffer: &mut WorkingBuffer) {
    use crate::color::{linear_to_srgb, srgb_to_linear};
    buffer.data_mut().par_chunks_mut(4).for_each(|p| {
        let a = (p[3].clamp(0.0, 1.0) * 255.0).round() / 255.0;
        if a <= 0.0 || p[3] <= 0.0 {
            p.fill(0.0);
            return;
        }
        for i in 0..3 {
            let s = (p[i] / p[3]).clamp(0.0, 1.0);
            p[i] = srgb_to_linear((linear_to_srgb(s) * 255.0).round() / 255.0) * a;
        }
        p[3] = a;
    });
}

/// D-202: `out = before + m (after - before)` at every sample of `after`, all four premultiplied
/// channels, `before` placed `(dx, dy)` into the grown buffer and transparent outside its own
/// rectangle.
pub(crate) fn mix_back(after: &mut WorkingBuffer, before: &WorkingBuffer, (dx, dy): (usize, usize), m: f64) {
    let (w, bw, bh, m) = (after.width(), before.width(), before.height(), m as f32);
    let b = before.data();
    after.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
        for (x, px) in row.chunks_exact_mut(4).enumerate() {
            let inside = x >= dx && y >= dy && x - dx < bw && y - dy < bh;
            for (c, v) in px.iter_mut().enumerate() {
                let was = if inside { b[((y - dy) * bw + x - dx) * 4 + c] } else { 0.0 };
                *v = was + m * (*v - was);
            }
        }
    });
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

/// D-299: Fractal Noise's type, noise type and cycle as the noise reads them.
pub(crate) fn fractal_look(fractal_type: &str, noise_type: &str, cycle: f64) -> crate::grade::Look {
    crate::grade::Look { turbulent: fractal_type == "turbulent", block: noise_type == "block", cycle: cycle.floor() as i64 }
}

/// D-126: the ellipse in the drawing's own size, however far the layer has grown (its corner at
/// `(ox, oy)` in a buffer `w` by `h`). `[size, roundness, softness]` as the effect holds them.
pub(crate) fn vignette_settings(
    amount: f64,
    color: &str,
    [size, roundness, softness]: [f64; 3],
    center: [f64; 2],
    (w, h): (usize, usize),
    (ox, oy): (usize, usize),
) -> crate::grade::Vignette {
    let w0 = (w - 2 * ox) as f64;
    let h0 = (h - 2 * oy) as f64;
    let (m, r) = (roundness / 100.0, (w0 * h0).sqrt() / 2.0);
    let outer = size / 100.0;
    crate::grade::Vignette {
        center: radial_center(center, (w, h), (ox, oy)),
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
///
/// D-333, `ae`: in 32 bpc (After Effects) the straight display value is multiplied by
/// `(2^stops)^(1/2.2)`, which is a 2.2 curve to linear light, the gain, and the curve back.
///
/// D-335: `offset` is added after the gain and `1 / gamma` taken after that, on the straight
/// value, and `bypass` at 8 bpc or 32 bpc (After Effects) works on the display value itself.
/// With neither offset nor gamma nor bypass it is the code before, bit for bit.
fn exposure(source: &mut WorkingBuffer, stops: f64, offset: f64, gamma: f64, bypass: bool, bits: Bits) {
    use crate::color::{linear_to_srgb, srgb_to_linear};
    let ae = bits == Bits::Ae32;
    let bypass = bypass && bits != Bits::Linear;
    if offset != 0.0 || gamma != 1.0 || bypass {
        let (gain, inv) = ((2.0f64).powf(stops), 1.0 / gamma);
        // A colour the offset took below 0 keeps its sign through the power.
        let power = |u: f64, g: f64| u.signum() * u.abs().powf(g);
        source.data_mut().par_chunks_mut(4).for_each(|px| {
            let a = px[3];
            for c in &mut px[..3] {
                if a <= 0.0 {
                    *c *= gain as f32;
                    continue;
                }
                let s = (*c / a).max(0.0);
                let v = if bypass {
                    linear_to_srgb(s) as f64
                } else if ae {
                    (linear_to_srgb(s) as f64).powf(2.2)
                } else {
                    s as f64
                };
                let u = power(v * gain + offset, inv);
                let back = if bypass {
                    srgb_to_linear(u as f32)
                } else if ae {
                    srgb_to_linear(power(u, 1.0 / 2.2) as f32)
                } else {
                    u as f32
                };
                *c = back * a;
            }
        });
        return;
    }
    let gain = (2.0f64).powf(stops) as f32;
    if gain == 1.0 {
        return;
    }
    if ae {
        let k = (2.0f64).powf(stops / 2.2) as f32;
        source.data_mut().par_chunks_mut(4).for_each(|px| {
            let a = px[3];
            for c in &mut px[..3] {
                *c = if a > 0.0 { srgb_to_linear(linear_to_srgb((*c / a).max(0.0)) * k) * a } else { *c * gain };
            }
        });
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
    blur_axes(source, &gaussian_weights(sigma_px), (true, true))
}

/// D-303: the taps of an axis not blurred along, which take each pixel as it is.
pub(crate) fn still_weights(r: usize) -> Vec<f32> {
    let mut w = vec![0.0; 2 * r + 1];
    w[r] = 1.0;
    w
}

/// D-312: the layer at `source_opacity` % laid by `blend` on a solid of `color` (encoded) at
/// `opacity` %, as After Effects' Solid Composite lays it. D-337: on a layer already in display
/// values the colour is used as written.
fn solid_composite(source: &mut WorkingBuffer, source_opacity: f64, color: [f64; 3], opacity: f64, blend: &str, display: bool) {
    use crate::model::BlendMode;
    let mode = match blend {
        "add" => BlendMode::Add,
        "screen" => BlendMode::Screen,
        "multiply" => BlendMode::Multiply,
        _ => BlendMode::Normal,
    };
    let (k, o) = ((source_opacity / 100.0) as f32, (opacity / 100.0) as f32);
    let lin = color.map(|c| if display { c as f32 } else { crate::color::srgb_to_linear(c as f32) } * o);
    let solid = [lin[0], lin[1], lin[2], o];
    source.data_mut().par_chunks_mut(4).for_each(|px| {
        let src = [px[0] * k, px[1] * k, px[2] * k, px[3] * k];
        px.copy_from_slice(&crate::composite::blend_pixel(mode, src, solid));
    });
}

/// D-313: red, green, blue and alpha each blurred as Blur blurs, by their own sigma. A colour
/// is blurred premultiplied and divided by the alpha blurred with it, then multiplied by the
/// alpha's own blur; where its sigma is the alpha's it is the alpha's blur as it is, so four
/// the same are Blur exactly. Where the colour's own blur has no alpha it keeps the colour as
/// the alpha's blur has it, rather than black. Returns how far the buffer grew, the alpha's reach.
fn channel_blur(source: &mut WorkingBuffer, sigma: [f64; 4], repeat: bool, axes: (bool, bool)) -> usize {
    let blurred = |s: f64| {
        let mut b = source.clone();
        let r = if repeat {
            held_blur_axes(&mut b, &gaussian_weights(s), axes);
            0
        } else {
            blur_axes(&mut b, &gaussian_weights(s), axes)
        };
        (b, r)
    };
    let (mut out, g) = blurred(sigma[3]);
    let w = out.width();
    for c in 0..3 {
        if sigma[c] == sigma[3] {
            continue;
        }
        let (b, r) = blurred(sigma[c]);
        let (bw, bh) = (b.width() as isize, b.height() as isize);
        let shift = r as isize - g as isize;
        let from = b.data();
        out.data_mut().par_chunks_mut(w * 4).enumerate().for_each(|(y, row)| {
            for (x, px) in row.chunks_exact_mut(4).enumerate() {
                let (bx, by) = (x as isize + shift, y as isize + shift);
                if (0..bw).contains(&bx) && (0..bh).contains(&by) {
                    let q = &from[((by * bw + bx) * 4) as usize..][..4];
                    if q[3] > 0.0 {
                        px[c] = q[c] / q[3] * px[3];
                    }
                }
            }
        });
    }
    *source = out;
    g
}

/// D-303: [`blur`] along across (`axes.0`), down (`axes.1`) or both. An axis not blurred along
/// takes [`still_weights`], so the buffer grows the same either way. `taps` are the kernel's
/// weights, [`gaussian_weights`] or D-321's [`reach_weights`].
pub(crate) fn blur_axes(source: &mut WorkingBuffer, taps: &[f32], axes: (bool, bool)) -> usize {
    let radius = taps.len() / 2;
    if radius == 0 {
        return 0;
    }
    let still = still_weights(radius);
    let (weights, down) = (if axes.0 { taps } else { &still[..] }, if axes.1 { taps } else { &still[..] });
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
        weights,
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
        down,
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
    held_blur_axes(source, &gaussian_weights(sigma_px), (true, true))
}

/// D-303: [`held_blur`] along across, down or both, as [`blur_axes`], with its `taps`.
pub(crate) fn held_blur_axes(source: &mut WorkingBuffer, taps: &[f32], axes: (bool, bool)) {
    let r = taps.len() / 2;
    if r == 0 {
        return;
    }
    let still = still_weights(r);
    let (weights, down) = (if axes.0 { taps } else { &still[..] }, if axes.1 { taps } else { &still[..] });
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
            for (k, &weight) in down.iter().enumerate() {
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
