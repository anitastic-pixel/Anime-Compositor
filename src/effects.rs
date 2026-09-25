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
    GaussianBlur { sigma_px: f64 },
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
    /// streak in pixels, 0 to 500.
    DirectionalBlur { direction: f64, length: f64 },
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
            Effect::GaussianBlur { sigma_px } => vec![("sigma_px", vec![sigma_px], 0.0, 500.0)],
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
            Effect::DirectionalBlur { direction, length } => vec![
                ("direction", vec![direction], -3600.0, 3600.0),
                ("length", vec![length], 0.0, 500.0),
            ],
            Effect::LineWidth {
                width, tolerance, ..
            } => vec![
                ("width", vec![width], -20.0, 20.0),
                ("tolerance", vec![tolerance], 0.0, 255.0),
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
            Effect::GaussianBlur { sigma_px } => *sigma_px = scale(*sigma_px),
            // D-87's blur and D-89's radius are distances in pixels too.
            Effect::SelectiveColorBlur { blur, .. } => *blur = scale(*blur),
            Effect::Glow { radius, .. } => *radius = scale(*radius),
            Effect::DirectionalBlur { length, .. } => *length = scale(*length),
            Effect::LineWidth { width, .. } => *width = scale(*width),
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
            Effect::GaussianBlur { sigma_px } => kernel_radius(*sigma_px),
            // D-89: the light reaches `radius` pixels, blur's reach at sigma radius / 3.
            Effect::Glow { radius, .. } => kernel_radius(*radius / 3.0),
            // D-92: half the streak, on each side.
            Effect::DirectionalBlur { length, .. } => (*length / 2.0).ceil() as usize,
            // D-94: a thicker line reaches its width further out; a thinner one nowhere.
            Effect::LineWidth { width, .. } if *width > 0.0 => width.ceil() as usize,
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
            Effect::GaussianBlur { sigma_px } => (0.0..=500.0).contains(sigma_px),
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
            Effect::GaussianBlur { sigma_px } => {
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
        let own = match self {
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
    mut report: impl FnMut(usize, &EffectInstance, Bypassed),
) -> (usize, usize) {
    let (mut ox, mut oy) = (0usize, 0usize);
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
            Effect::GaussianBlur { sigma_px } => {
                let r =
                    crate::perf::time(crate::perf::Stage::EffectBlur, || blur(source, *sigma_px));
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
                    crate::glow::glow(
                        source, based_on, *threshold, colors, *tolerance, *radius, *intensity,
                        operation, tint,
                    )
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
            Effect::DirectionalBlur { direction, length } => {
                let r = crate::perf::time(crate::perf::Stage::EffectDirBlur, || {
                    crate::blurs::directional_blur(source, *direction, *length)
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
        }
    }
    (ox, oy)
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
}
