//! The project model of document 19, and the property evaluation rules of document 20.
//!
//! Document 19: "editable state is changed only through commands". Nothing here is public
//! for mutation: [`Composition`]'s layers are readable but not writable from outside, and the
//! only way to change one is [`crate::command`]. That is why the fields below are private
//! despite the accessors reading like a plain struct.
//!
//! Document 19 also says "Stable IDs are opaque UUID strings; display names are never
//! identity." [`Id`] is a newtype over `String` so a name cannot be passed where an ID is
//! wanted, and layer lookup is by ID only. Index is not identity: reordering moves entries in
//! `layer_order` and rewrites no IDs.
//!
//! Not modelled yet, and deliberately: effects (B-06), masks (parked with R-04 under D-12),
//! and colour4 properties, which G1 needs only once effects have colour parameters.

use std::collections::BTreeMap;
use std::fmt;

use crate::time::{ExposureSpan, FrameRate};

/// An opaque stable identifier. Document 19: display names are never identity.
#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Id(String);

impl Id {
    pub fn new(text: impl Into<String>) -> Self {
        Id(text.into())
    }
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}

/// Document 19: "Supported G1 property value types: scalar, vec2, color4 and boolean".
///
/// Only the two the transform needs exist so far. A property's keyframes must all carry the
/// same variant as its base value; [`Property::keyframe_types_match`] is what the command
/// layer checks before committing.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Value {
    Scalar(f64),
    Vec2(f64, f64),
}

impl Value {
    pub fn kind(&self) -> &'static str {
        match self {
            Value::Scalar(_) => "scalar",
            Value::Vec2(_, _) => "vec2",
        }
    }

    pub fn as_scalar(&self) -> Option<f64> {
        match *self {
            Value::Scalar(v) => Some(v),
            _ => None,
        }
    }

    pub fn as_vec2(&self) -> Option<(f64, f64)> {
        match *self {
            Value::Vec2(x, y) => Some((x, y)),
            _ => None,
        }
    }

    /// Document 20: "with `u=(f-f0)/(f1-f0)`, linear evaluation is `v0 + u*(v1-v0)`."
    ///
    /// Written in exactly that form rather than the algebraically equal `(1-u)*v0 + u*v1`,
    /// because the two disagree in the last bit and the document names one of them.
    fn lerp(self, other: Value, u: f64) -> Value {
        match (self, other) {
            (Value::Scalar(a), Value::Scalar(b)) => Value::Scalar(a + u * (b - a)),
            (Value::Vec2(ax, ay), Value::Vec2(bx, by)) => {
                Value::Vec2(ax + u * (bx - ax), ay + u * (by - ay))
            }
            // Unreachable through the command layer, which rejects a mixed property. Holding
            // is the honest answer rather than inventing an interpolation between kinds.
            _ => self,
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Scalar(v) => write!(f, "{v}"),
            Value::Vec2(x, y) => write!(f, "({x}, {y})"),
        }
    }
}

/// Document 19: "supports interpolation values `hold` and `linear`".
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Interp {
    Hold,
    Linear,
}

impl Interp {
    pub fn as_str(self) -> &'static str {
        match self {
            Interp::Hold => "hold",
            Interp::Linear => "linear",
        }
    }
}

/// Document 20: "Each keyframe contains a value and the interpolation mode used from that
/// keyframe to the next keyframe." The mode belongs to the segment that starts here, not to
/// the one that ends here.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Keyframe {
    pub frame: i32,
    pub value: Value,
    pub interp: Interp,
}

/// A base value plus zero or more keyframes, sorted by frame and unique in frame.
#[derive(Clone, PartialEq, Debug)]
pub struct Property {
    base: Value,
    keyframes: Vec<Keyframe>,
}

impl Property {
    pub fn constant(base: Value) -> Self {
        Property {
            base,
            keyframes: Vec::new(),
        }
    }

    pub fn base(&self) -> Value {
        self.base
    }
    pub fn keyframes(&self) -> &[Keyframe] {
        &self.keyframes
    }
    pub fn is_animated(&self) -> bool {
        !self.keyframes.is_empty()
    }

    pub(crate) fn set_base(&mut self, value: Value) {
        self.base = value;
    }

    /// Insert or replace the keyframe at `frame`, keeping the list sorted.
    ///
    /// Document 19 calls two keyframes at one frame invalid, so this cannot create a pair;
    /// setting a keyframe where one exists replaces it, which is what an artist dragging a
    /// value on a keyframed frame means.
    pub(crate) fn set_keyframe(&mut self, key: Keyframe) {
        match self.keyframes.binary_search_by_key(&key.frame, |k| k.frame) {
            Ok(i) => self.keyframes[i] = key,
            Err(i) => self.keyframes.insert(i, key),
        }
    }

    pub(crate) fn remove_keyframe(&mut self, frame: i32) -> Option<Keyframe> {
        let i = self
            .keyframes
            .binary_search_by_key(&frame, |k| k.frame)
            .ok()?;
        Some(self.keyframes.remove(i))
    }

    pub fn keyframe_at(&self, frame: i32) -> Option<&Keyframe> {
        self.keyframes
            .binary_search_by_key(&frame, |k| k.frame)
            .ok()
            .map(|i| &self.keyframes[i])
    }

    /// True when every keyframe carries the same value kind as the base.
    pub fn keyframe_types_match(&self) -> bool {
        self.keyframes
            .iter()
            .all(|k| k.value.kind() == self.base.kind())
    }

    /// Document 20's five rules, in the order it states them.
    ///
    /// - zero keyframes: base value;
    /// - before the first: the first keyframe's value;
    /// - exactly on a keyframe: that keyframe's value;
    /// - after the last: the last keyframe's value;
    /// - between two: hold returns the left value, linear interpolates.
    pub fn value_at(&self, frame: i32) -> Value {
        let keys = &self.keyframes;
        let Some(first) = keys.first() else {
            return self.base;
        };
        if frame <= first.frame {
            return first.value;
        }
        let last = keys.last().expect("non-empty");
        if frame >= last.frame {
            return last.value;
        }
        // The segment containing `frame` starts at the last keyframe at or before it. The
        // strict inequalities above mean both neighbours exist here.
        let i = keys.partition_point(|k| k.frame <= frame) - 1;
        let (a, b) = (&keys[i], &keys[i + 1]);
        if a.frame == frame || a.interp == Interp::Hold {
            return a.value;
        }
        let u = (frame - a.frame) as f64 / (b.frame - a.frame) as f64;
        a.value.lerp(b.value, u)
    }
}

/// Which transform property a command addresses.
///
/// An enum rather than a string path: a typo in a property name should not compile, and the
/// five names are fixed by document 19.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Prop {
    Anchor,
    Position,
    Scale,
    Rotation,
    Opacity,
}

impl Prop {
    pub fn as_str(self) -> &'static str {
        match self {
            Prop::Anchor => "anchor",
            Prop::Position => "position",
            Prop::Scale => "scale",
            Prop::Rotation => "rotation",
            Prop::Opacity => "opacity",
        }
    }

    /// Document 19: anchor, position and scale are vec2; rotation and opacity are scalar.
    pub fn kind(self) -> &'static str {
        match self {
            Prop::Anchor | Prop::Position | Prop::Scale => "vec2",
            Prop::Rotation | Prop::Opacity => "scalar",
        }
    }
}

impl fmt::Display for Prop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Document 19: "Transform contains anchor, position, scale, rotation and opacity properties.
/// Scale is percentage-like in UI but serialized as explicit numeric pairs. Position and anchor
/// use pixels. Rotation uses degrees. Opacity uses normalized 0..1 in the model."
#[derive(Clone, PartialEq, Debug)]
pub struct Transform {
    pub anchor: Property,
    pub position: Property,
    pub scale: Property,
    pub rotation: Property,
    pub opacity: Property,
}

impl Default for Transform {
    /// Identity: no offset, 100% scale as the pair (1, 1), no rotation, fully opaque.
    fn default() -> Self {
        Transform {
            anchor: Property::constant(Value::Vec2(0.0, 0.0)),
            position: Property::constant(Value::Vec2(0.0, 0.0)),
            scale: Property::constant(Value::Vec2(1.0, 1.0)),
            rotation: Property::constant(Value::Scalar(0.0)),
            opacity: Property::constant(Value::Scalar(1.0)),
        }
    }
}

impl Transform {
    pub fn get(&self, prop: Prop) -> &Property {
        match prop {
            Prop::Anchor => &self.anchor,
            Prop::Position => &self.position,
            Prop::Scale => &self.scale,
            Prop::Rotation => &self.rotation,
            Prop::Opacity => &self.opacity,
        }
    }

    pub(crate) fn get_mut(&mut self, prop: Prop) -> &mut Property {
        match prop {
            Prop::Anchor => &mut self.anchor,
            Prop::Position => &mut self.position,
            Prop::Scale => &mut self.scale,
            Prop::Rotation => &mut self.rotation,
            Prop::Opacity => &mut self.opacity,
        }
    }

    /// Every property at one composition frame, in document 19's order.
    pub fn value_at(&self, frame: i32) -> [(Prop, Value); 5] {
        [
            (Prop::Anchor, self.anchor.value_at(frame)),
            (Prop::Position, self.position.value_at(frame)),
            (Prop::Scale, self.scale.value_at(frame)),
            (Prop::Rotation, self.rotation.value_at(frame)),
            (Prop::Opacity, self.opacity.value_at(frame)),
        ]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BlendMode {
    Normal,
    Multiply,
    Screen,
    Add,
}

impl BlendMode {
    pub fn as_str(self) -> &'static str {
        match self {
            BlendMode::Normal => "normal",
            BlendMode::Multiply => "multiply",
            BlendMode::Screen => "screen",
            BlendMode::Add => "add",
        }
    }
}

/// Document 19: "MatteReference stores another layer ID and mode `alpha`."
///
/// The record exists; matte *rendering* is parked to G1-rest with R-04 under D-12. Modelling
/// it now is what lets document 26's "deleting/recovering a matte preserves dependent records"
/// be tested at B-05 rather than waiting for the renderer.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct MatteReference {
    pub layer_id: Id,
}

/// Document 19's raster layer, minus the parts G1-core has not reached.
#[derive(Clone, PartialEq, Debug)]
pub struct Layer {
    pub id: Id,
    pub name: String,
    pub asset_id: Id,
    pub enabled: bool,
    pub locked: bool,
    pub in_frame: i32,
    pub out_frame: i32,
    pub source_offset_frames: i32,
    pub transform: Transform,
    pub exposure_spans: Vec<ExposureSpan>,
    pub matte: Option<MatteReference>,
    pub blend_mode: BlendMode,
}

impl Layer {
    /// A layer covering one composition span with an identity transform.
    pub fn new(
        id: Id,
        name: impl Into<String>,
        asset_id: Id,
        in_frame: i32,
        out_frame: i32,
    ) -> Self {
        Layer {
            id,
            name: name.into(),
            asset_id,
            enabled: true,
            locked: false,
            in_frame,
            out_frame,
            source_offset_frames: 0,
            transform: Transform::default(),
            exposure_spans: Vec::new(),
            matte: None,
            blend_mode: BlendMode::Normal,
        }
    }

    /// Document 19's layer invariant.
    pub fn timing_is_valid(&self) -> bool {
        self.in_frame < self.out_frame
    }

    pub fn timing(&self) -> crate::time::LayerTiming {
        crate::time::LayerTiming {
            in_frame: self.in_frame,
            out_frame: self.out_frame,
            source_offset_frames: self.source_offset_frames,
        }
    }
}

/// Document 19: "Layer order is composition order. Index is not identity. Reordering must not
/// rewrite layer IDs or references."
///
/// Layers live in a map keyed by ID and `layer_order` holds the drawing order, so reordering
/// touches only the order vector. A reordered layer is bit-identical to what it was.
#[derive(Clone, PartialEq, Debug)]
pub struct Composition {
    pub id: Id,
    pub name: String,
    pub width: u32,
    pub height: u32,
    pub frame_rate: FrameRate,
    pub start_frame: i32,
    pub duration_frames: u32,
    layer_order: Vec<Id>,
    layers: BTreeMap<Id, Layer>,
}

impl Composition {
    pub fn new(
        id: Id,
        name: impl Into<String>,
        width: u32,
        height: u32,
        frame_rate: FrameRate,
        start_frame: i32,
        duration_frames: u32,
    ) -> Self {
        Composition {
            id,
            name: name.into(),
            width,
            height,
            frame_rate,
            start_frame,
            duration_frames,
            layer_order: Vec::new(),
            layers: BTreeMap::new(),
        }
    }

    pub fn layer_order(&self) -> &[Id] {
        &self.layer_order
    }

    pub fn layer(&self, id: &Id) -> Option<&Layer> {
        self.layers.get(id)
    }

    /// Layers in composition order, which is the order they are drawn in.
    pub fn layers_in_order(&self) -> impl Iterator<Item = &Layer> {
        self.layer_order
            .iter()
            .map(|id| self.layers.get(id).expect("order and map agree"))
    }

    pub fn len(&self) -> usize {
        self.layer_order.len()
    }
    pub fn is_empty(&self) -> bool {
        self.layer_order.is_empty()
    }

    pub fn index_of(&self, id: &Id) -> Option<usize> {
        self.layer_order.iter().position(|l| l == id)
    }

    /// Layers whose matte points at `id`. Used to keep dependent records across a delete.
    pub fn dependents_of(&self, id: &Id) -> Vec<Id> {
        self.layers_in_order()
            .filter(|l| l.matte.as_ref().is_some_and(|m| &m.layer_id == id))
            .map(|l| l.id.clone())
            .collect()
    }

    /// Document 19: "the dependency graph must be acyclic."
    ///
    /// Each layer has at most one matte, so the graph is a functional one and a cycle is found
    /// by walking forward until a node repeats. No colouring needed.
    pub fn matte_cycle_from(&self, start: &Id) -> bool {
        let mut seen = vec![start.clone()];
        let mut at = start.clone();
        while let Some(next) = self.layers.get(&at).and_then(|l| l.matte.as_ref()) {
            if seen.contains(&next.layer_id) {
                return true;
            }
            seen.push(next.layer_id.clone());
            at = next.layer_id.clone();
        }
        false
    }

    pub(crate) fn insert_layer(&mut self, layer: Layer, index: usize) {
        let id = layer.id.clone();
        self.layers.insert(id.clone(), layer);
        self.layer_order
            .insert(index.min(self.layer_order.len()), id);
    }

    pub(crate) fn remove_layer(&mut self, id: &Id) -> Option<(Layer, usize)> {
        let index = self.index_of(id)?;
        self.layer_order.remove(index);
        self.layers.remove(id).map(|l| (l, index))
    }

    pub(crate) fn move_layer(&mut self, id: &Id, to: usize) -> Option<usize> {
        let from = self.index_of(id)?;
        let entry = self.layer_order.remove(from);
        self.layer_order
            .insert(to.min(self.layer_order.len()), entry);
        Some(from)
    }

    pub(crate) fn layer_mut(&mut self, id: &Id) -> Option<&mut Layer> {
        self.layers.get_mut(id)
    }
}

/// Document 19's asset record, in the form B-05 needs: identity and interpretation. The
/// frame-number-to-file map lives in [`crate::media::SequenceAsset`], which B-03 already built.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Asset {
    pub id: Id,
    pub name: String,
    pub pattern: String,
}

/// Document 19: "Project owns: schema version, project ID, project settings, assets,
/// compositions, color settings and application metadata."
///
/// Colour settings are the fixed pair document 21 requires of G1 and are not modelled as a
/// choice, because there is not yet a second option to choose.
#[derive(Clone, PartialEq, Debug)]
pub struct Project {
    pub id: Id,
    pub assets: Vec<Asset>,
    pub compositions: Vec<Composition>,
}

impl Project {
    pub fn new(id: Id) -> Self {
        Project {
            id,
            assets: Vec::new(),
            compositions: Vec::new(),
        }
    }

    pub fn composition(&self, id: &Id) -> Option<&Composition> {
        self.compositions.iter().find(|c| &c.id == id)
    }

    pub(crate) fn composition_mut(&mut self, id: &Id) -> Option<&mut Composition> {
        self.compositions.iter_mut().find(|c| &c.id == id)
    }
}
