//! B-14b: D-59's expression language, read and evaluated.
//!
//! Document 09's "Native expressions" section is the language and this module is written from
//! it. `tools/expression_reference.py` is a second implementation written from the same section
//! and sharing nothing with this one; document 25's FX-EXPR cases are its output and
//! `tests/b14b_expressions.rs` holds this module to them.
//!
//! There is no word here for a file, a network, a process, a loop or a function definition, so
//! there is nothing to keep out: a text that names one is refused when it is read.
//!
//! Units: an expression reads and gives every property in the file's units. The model keeps
//! scale as a factor (D-22) and opacity as 0..1, so both are multiplied by 100 on the way in and
//! divided by 100 on the way out; opacity is 0..100 inside an expression because D-59 says so, and
//! scale because the file holds a percentage.

use std::borrow::Cow;
use std::collections::HashMap;

use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::model::{Camera, Composition, Id, Prop, Property, Value};

const MAX_TEXT_BYTES: usize = 4096;
const MAX_STEPS: u32 = 10_000;
const MAX_DEPTH: usize = 16;
const MAX_OCTAVES: f64 = 16.0;

/// The words an expression may start from. Anything else is `EXPRESSION_SYNTAX`.
const ROOTS: &[&str] = &[
    "time",
    "value",
    "thisComp",
    "thisLayer",
    "thisProperty",
    "Math",
    "wiggle",
    "loopOut",
    "linear",
    "ease",
    "clamp",
    "random",
    "seedRandom",
    "posterizeTime",
];
const DECLARATIONS: &[&str] = &["var", "let", "const"];
const MATH: &[&str] = &[
    "sin", "cos", "tan", "abs", "floor", "ceil", "round", "sqrt", "pow", "min", "max",
];

/// Whose property an expression sits on or reads.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Target {
    Layer(Id),
    /// The composition's camera, the default one of D-58 if the file gives none.
    Camera,
}

/// Why an expression gave no value. `id` is one of document 28's five `EXPRESSION_` entries.
#[derive(Clone, PartialEq, Debug)]
pub struct ExprError {
    pub id: DiagnosticId,
    pub message: String,
}

impl ExprError {
    /// Document 28's shape, naming the property and the frame. An ERROR, as document 28 lists
    /// it, though the frame is still drawn with the property at its keyed value; it is the export
    /// that is refused.
    pub fn diagnostic(&self, owner: &str, prop: Prop, frame: i32) -> Diagnostic {
        Diagnostic::new(
            self.id,
            Severity::Error,
            format!("The expression on {owner}'s {prop} does not work at frame {frame}."),
            format!(
                "{}. The {prop} is drawn at its keyed value on this frame.",
                self.message
            ),
        )
        .with_remediation("Correct the expression, or switch it off.")
    }
}

fn syntax(message: impl Into<String>) -> ExprError {
    ExprError {
        id: DiagnosticId::ExpressionSyntax,
        message: message.into(),
    }
}

fn type_error(message: impl Into<String>) -> ExprError {
    ExprError {
        id: DiagnosticId::ExpressionType,
        message: message.into(),
    }
}

type R<T> = Result<T, ExprError>;

/// A property's value at `frame` after its expression, in model units, or why the expression
/// failed. A property with no expression switched on is its keys, and never fails.
pub fn evaluate(comp: &Composition, target: &Target, prop: Prop, frame: i32) -> R<Value> {
    let mut run = Run {
        comp,
        steps: 0,
        stack: Vec::new(),
    };
    let v = run.finished(target, prop, frame)?;
    Ok(to_model(prop, &v))
}

/// `property`, which is `target`'s `prop`, at `frame` after its expression, and why the
/// expression failed if it did, in which case the value is the keyed one a frame is drawn with.
/// `target` is only built when there is an expression to run, so a property without one costs
/// what reading its keys costs.
pub fn resolve(
    comp: &Composition,
    property: &Property,
    target: impl FnOnce() -> Target,
    prop: Prop,
    frame: i32,
) -> (Value, Option<ExprError>) {
    // D-69: a separated position has no expression of its own, and X and Y may each have one.
    let halves = property
        .split()
        .is_some_and(|(x, y)| x.live_expression().is_some() || y.live_expression().is_some());
    if property.live_expression().is_none() && !halves {
        return (property.value_at(frame), None);
    }
    match evaluate(comp, &target(), prop, frame) {
        Ok(v) => (v, None),
        Err(e) => (property.value_at(frame), Some(e)),
    }
}

/// Every property of `comp` carrying an expression that is switched on, which is what an export
/// has to check frame by frame.
pub fn live_properties(comp: &Composition) -> Vec<(Target, Prop)> {
    let mut out = Vec::new();
    for layer in comp.layers_in_order() {
        let target = Target::Layer(layer.id.clone());
        for prop in LAYER_PROPS {
            if property(comp, &target, prop).is_ok_and(|p| p.live_expression().is_some()) {
                out.push((target.clone(), prop));
            }
        }
    }
    for prop in CAMERA_PROPS {
        if property(comp, &Target::Camera, prop).is_ok_and(|p| p.live_expression().is_some()) {
            out.push((Target::Camera, prop));
        }
    }
    out
}

/// The name a person knows the target by: a layer's name, or "the camera".
pub fn owner_name(comp: &Composition, target: &Target) -> String {
    match target {
        Target::Camera => "the camera".to_string(),
        Target::Layer(id) => comp
            .layer(id)
            .map_or_else(|| id.as_str().to_string(), |l| l.name.clone()),
    }
}

const LAYER_PROPS: [Prop; 8] = [
    Prop::PositionX,
    Prop::PositionY,
    Prop::Anchor,
    Prop::Position,
    Prop::Scale,
    Prop::Rotation,
    Prop::Opacity,
    Prop::Depth,
];
const CAMERA_PROPS: [Prop; 3] = [Prop::Position, Prop::Depth, Prop::Zoom];

fn property<'a>(comp: &'a Composition, target: &Target, prop: Prop) -> R<Cow<'a, Property>> {
    match target {
        Target::Camera => {
            if !CAMERA_PROPS.contains(&prop) {
                return Err(syntax(format!("the camera has no {prop}")));
            }
            let camera_prop = crate::model::CameraProp::from_str(prop.as_str())
                .expect("the three camera properties are named alike");
            Ok(match &comp.camera {
                Some(camera) => Cow::Borrowed(camera.get(camera_prop)),
                None => Cow::Owned(
                    Camera::default_for(comp.width, comp.height)
                        .get(camera_prop)
                        .clone(),
                ),
            })
        }
        Target::Layer(id) => {
            let layer = comp.layer(id).ok_or_else(|| ExprError {
                id: DiagnosticId::ExpressionReferenceMissing,
                message: format!("There is no layer with the id \"{}\"", id.as_str()),
            })?;
            match prop {
                Prop::Depth => Ok(match &layer.depth {
                    Some(d) => Cow::Borrowed(d),
                    None => Cow::Owned(Property::constant(Value::Scalar(0.0))),
                }),
                _ => layer
                    .transform
                    .get(prop)
                    .map(Cow::Borrowed)
                    .ok_or_else(|| syntax(format!("a layer has no {prop}"))),
            }
        }
    }
}

fn percent(prop: Prop) -> bool {
    matches!(prop, Prop::Opacity | Prop::Scale)
}

fn to_expr(prop: Prop, v: Value) -> V {
    let k = if percent(prop) { 100.0 } else { 1.0 };
    match v {
        Value::Scalar(x) => V::Num(x * k),
        Value::Vec2(x, y) => V::List(vec![x * k, y * k]),
    }
}

fn to_model(prop: Prop, v: &V) -> Value {
    let k = if percent(prop) { 100.0 } else { 1.0 };
    match v {
        V::List(l) => Value::Vec2(l[0] / k, l[1] / k),
        V::Num(x) => Value::Scalar(x / k),
        _ => unreachable!("the result check lets only numbers through"),
    }
}

fn dims(prop: Prop) -> usize {
    match prop.kind() {
        "vec2" => 2,
        _ => 1,
    }
}

// --- noise, to the operation in document 09 ------------------------------------------------------

fn splitmix64(x: u64) -> u64 {
    let mut z = x.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

fn mix(a: u64, b: u64) -> u64 {
    splitmix64(a ^ splitmix64(b))
}

fn unit(x: u64) -> f64 {
    (x >> 11) as f64 * (1.0 / (1u64 << 53) as f64)
}

fn fnv1a64(text: &str) -> u64 {
    text.bytes().fold(0xCBF2_9CE4_8422_2325, |h, b| {
        (h ^ b as u64).wrapping_mul(0x0000_0100_0000_01B3)
    })
}

fn lattice(seed: u64, stream: u64, i: i64) -> f64 {
    2.0 * unit(mix(mix(seed, stream), i as u64)) - 1.0
}

fn noise(seed: u64, stream: u64, x: f64) -> f64 {
    let fi = x.floor();
    let i = fi as i64;
    let s = x - fi;
    let a = lattice(seed, stream, i);
    let b = lattice(seed, stream, i.wrapping_add(1));
    let w = s * s * s * (s * (s * 6.0 - 15.0) + 10.0);
    a + (b - a) * w
}

// --- reading -------------------------------------------------------------------------------------

#[derive(Clone, PartialEq, Debug)]
enum Tok {
    Nl,
    Num(f64),
    Name(String),
    Str(String),
    Op(char),
    End,
}

fn show(t: &Tok) -> String {
    match t {
        Tok::Nl => "a line break".into(),
        Tok::Num(n) => n.to_string(),
        Tok::Name(s) | Tok::Str(s) => s.clone(),
        Tok::Op(c) => c.to_string(),
        Tok::End => "the end".into(),
    }
}

fn tokens(text: &str) -> R<Vec<Tok>> {
    let b = text.as_bytes();
    let (mut out, mut i) = (Vec::new(), 0);
    while i < b.len() {
        let c = b[i];
        if matches!(c, b' ' | b'\t' | b'\r') {
            i += 1;
        } else if c == b'\n' {
            out.push(Tok::Nl);
            i += 1;
        } else if b[i..].starts_with(b"//") {
            while i < b.len() && b[i] != b'\n' {
                i += 1;
            }
        } else if c.is_ascii_digit() || (c == b'.' && b.get(i + 1).is_some_and(u8::is_ascii_digit))
        {
            let mut j = i;
            while j < b.len() && (b[j].is_ascii_digit() || b[j] == b'.') {
                j += 1;
            }
            if j < b.len() && (b[j] == b'e' || b[j] == b'E') {
                let mut k = j + 1;
                if k < b.len() && (b[k] == b'+' || b[k] == b'-') {
                    k += 1;
                }
                if k < b.len() && b[k].is_ascii_digit() {
                    j = k;
                    while j < b.len() && b[j].is_ascii_digit() {
                        j += 1;
                    }
                }
            }
            let word = &text[i..j];
            let n = word
                .parse::<f64>()
                .map_err(|_| syntax(format!("\"{word}\" is not a number")))?;
            out.push(Tok::Num(n));
            i = j;
        } else if c.is_ascii_alphabetic() || c == b'_' {
            let mut j = i;
            while j < b.len() && (b[j].is_ascii_alphanumeric() || b[j] == b'_') {
                j += 1;
            }
            out.push(Tok::Name(text[i..j].to_string()));
            i = j;
        } else if c == b'"' || c == b'\'' {
            let close = b[i + 1..].iter().position(|&x| x == c);
            match close {
                Some(n) if !b[i + 1..i + 1 + n].contains(&b'\n') => {
                    out.push(Tok::Str(text[i + 1..i + 1 + n].to_string()));
                    i += n + 2;
                }
                _ => return Err(syntax("A quoted text is not closed")),
            }
        } else if b"+-*/%()[],.;=".contains(&c) {
            out.push(Tok::Op(c as char));
            i += 1;
        } else {
            let ch = text[i..].chars().next().expect("inside the text");
            return Err(syntax(format!("\"{ch}\" is not part of the language")));
        }
    }
    out.push(Tok::End);
    Ok(out)
}

#[derive(Debug)]
enum Node {
    Num(f64),
    Str(String),
    Name(String),
    Array(Vec<Node>),
    Neg(Box<Node>),
    Bin(char, Box<Node>, Box<Node>),
    Index(Box<Node>, Box<Node>),
    Get(Box<Node>, String),
    Call(Box<Node>, Vec<Node>),
}

enum Stmt {
    Let(String, Node),
    Do(Node),
}

struct Reader {
    toks: Vec<Tok>,
    at: usize,
    locals: Vec<String>,
}

impl Reader {
    fn tok(&self, at: usize) -> &Tok {
        self.toks.get(at).unwrap_or(&Tok::End)
    }

    fn is_op(&self, c: char) -> bool {
        *self.tok(self.at) == Tok::Op(c)
    }

    fn at_break(&self) -> bool {
        matches!(self.tok(self.at), Tok::Nl | Tok::End) || self.is_op(';')
    }

    fn expect_op(&mut self, c: char) -> R<()> {
        if !self.is_op(c) {
            return Err(syntax(format!(
                "Expected {c}, found {}",
                show(self.tok(self.at))
            )));
        }
        self.at += 1;
        Ok(())
    }

    fn name(&mut self) -> R<String> {
        match self.tok(self.at).clone() {
            Tok::Name(n) => {
                self.at += 1;
                Ok(n)
            }
            t => Err(syntax(format!("Expected a name, found {}", show(&t)))),
        }
    }

    fn program(&mut self) -> R<Vec<Stmt>> {
        let mut stmts = Vec::new();
        loop {
            while matches!(self.tok(self.at), Tok::Nl) || self.is_op(';') {
                self.at += 1;
            }
            if *self.tok(self.at) == Tok::End {
                break;
            }
            stmts.push(self.statement()?);
            if !self.at_break() {
                return Err(syntax(format!(
                    "Expected the end of a line, found {}",
                    show(self.tok(self.at))
                )));
            }
        }
        if stmts.is_empty() {
            return Err(syntax("The expression is empty"));
        }
        Ok(stmts)
    }

    fn assigns(&self) -> bool {
        matches!(self.tok(self.at), Tok::Name(_)) && *self.tok(self.at + 1) == Tok::Op('=')
    }

    fn statement(&mut self) -> R<Stmt> {
        if matches!(self.tok(self.at), Tok::Name(n) if DECLARATIONS.contains(&n.as_str())) {
            self.at += 1;
            if !self.assigns() {
                return Err(syntax("A declaration names a value and gives it one"));
            }
        }
        if self.assigns() {
            let name = self.name()?;
            self.expect_op('=')?;
            if ROOTS.contains(&name.as_str()) || DECLARATIONS.contains(&name.as_str()) {
                return Err(syntax(format!(
                    "\"{name}\" is a word of the language and cannot be given a value"
                )));
            }
            let node = self.expr()?;
            self.locals.push(name.clone());
            return Ok(Stmt::Let(name, node));
        }
        Ok(Stmt::Do(self.expr()?))
    }

    fn expr(&mut self) -> R<Node> {
        let mut node = self.term()?;
        while self.is_op('+') || self.is_op('-') {
            let op = if self.is_op('+') { '+' } else { '-' };
            self.at += 1;
            node = Node::Bin(op, Box::new(node), Box::new(self.term()?));
        }
        Ok(node)
    }

    fn term(&mut self) -> R<Node> {
        let mut node = self.unary()?;
        while let Some(op) = ['*', '/', '%'].into_iter().find(|&c| self.is_op(c)) {
            self.at += 1;
            node = Node::Bin(op, Box::new(node), Box::new(self.unary()?));
        }
        Ok(node)
    }

    fn unary(&mut self) -> R<Node> {
        if self.is_op('-') {
            self.at += 1;
            return Ok(Node::Neg(Box::new(self.unary()?)));
        }
        if self.is_op('+') {
            self.at += 1;
            return self.unary();
        }
        self.postfix()
    }

    fn postfix(&mut self) -> R<Node> {
        let mut node = self.primary()?;
        loop {
            if self.is_op('.') {
                self.at += 1;
                node = Node::Get(Box::new(node), self.name()?);
            } else if self.is_op('(') {
                self.at += 1;
                let mut args = Vec::new();
                if !self.is_op(')') {
                    args.push(self.expr()?);
                    while self.is_op(',') {
                        self.at += 1;
                        args.push(self.expr()?);
                    }
                }
                self.expect_op(')')?;
                node = Node::Call(Box::new(node), args);
            } else if self.is_op('[') {
                self.at += 1;
                let index = self.expr()?;
                self.expect_op(']')?;
                node = Node::Index(Box::new(node), Box::new(index));
            } else {
                return Ok(node);
            }
        }
    }

    fn primary(&mut self) -> R<Node> {
        let tok = self.tok(self.at).clone();
        self.at += 1;
        match tok {
            Tok::Num(n) => Ok(Node::Num(n)),
            Tok::Str(s) => Ok(Node::Str(s)),
            Tok::Name(n) => {
                if !ROOTS.contains(&n.as_str()) && !self.locals.contains(&n) {
                    return Err(syntax(format!("\"{n}\" is not a word this language knows")));
                }
                Ok(Node::Name(n))
            }
            Tok::Op('(') => {
                let node = self.expr()?;
                self.expect_op(')')?;
                Ok(node)
            }
            Tok::Op('[') => {
                let mut items = vec![self.expr()?];
                while self.is_op(',') {
                    self.at += 1;
                    items.push(self.expr()?);
                }
                self.expect_op(']')?;
                Ok(Node::Array(items))
            }
            t => Err(syntax(format!("Expected a value, found {}", show(&t)))),
        }
    }
}

fn read(text: &str) -> R<Vec<Stmt>> {
    if text.len() > MAX_TEXT_BYTES {
        return Err(syntax(format!(
            "An expression is at most {MAX_TEXT_BYTES} bytes long"
        )));
    }
    Reader {
        toks: tokens(text)?,
        at: 0,
        locals: Vec::new(),
    }
    .program()
}

// --- evaluating ----------------------------------------------------------------------------------

/// A value while an expression runs.
#[derive(Clone, Debug)]
enum V {
    Num(f64),
    List(Vec<f64>),
    Text(String),
    /// What `posterizeTime` and `seedRandom` give.
    Nothing,
    ThisComp,
    Math,
    Layer(Target),
    /// A property. `own` reads keys only, which is `thisProperty`.
    Prop(Target, Prop, bool),
    /// `prop.valueAtTime`, waiting for its argument.
    Method(Target, Prop, bool),
    Func(&'static str),
}

/// One top-level evaluation: its step budget and the properties being evaluated.
struct Run<'a> {
    comp: &'a Composition,
    steps: u32,
    stack: Vec<(Target, Prop, i32)>,
}

impl<'a> Run<'a> {
    fn keyed(&self, target: &Target, prop: Prop, frame: i32) -> R<V> {
        Ok(to_expr(
            prop,
            property(self.comp, target, prop)?.value_at(frame),
        ))
    }

    /// A property after its expression, in expression units.
    fn finished(&mut self, target: &Target, prop: Prop, frame: i32) -> R<V> {
        let record = property(self.comp, target, prop)?;
        // D-69: a separated position is X and Y, each after its own expression.
        if prop == Prop::Position && record.split().is_some() {
            let x = self.finished(target, Prop::PositionX, frame)?;
            let y = self.finished(target, Prop::PositionY, frame)?;
            return match (x, y) {
                (V::Num(x), V::Num(y)) => Ok(V::List(vec![x, y])),
                _ => Err(syntax(
                    "position_x and position_y are one number each".to_string(),
                )),
            };
        }
        let Some(text) = record.live_expression() else {
            return self.keyed(target, prop, frame);
        };
        let here = (target.clone(), prop, frame);
        if self.stack.contains(&here) {
            return Err(ExprError {
                id: DiagnosticId::ExpressionCycle,
                message: format!(
                    "{} {prop} depends on itself at frame {frame}",
                    owner_name(self.comp, target)
                ),
            });
        }
        if self.stack.len() >= MAX_DEPTH {
            return Err(ExprError {
                id: DiagnosticId::ExpressionTimeout,
                message: format!("More than {MAX_DEPTH} expressions deep"),
            });
        }
        let text = text.to_string();
        self.stack.push(here);
        let mut scope = Scope {
            run: self,
            target: target.clone(),
            prop,
            frame,
            posterize: None,
            user_seed: 0,
            timeless: false,
            wiggles: 0,
            randoms: 0,
            locals: HashMap::new(),
        };
        let out = scope.outcome(&text);
        self.stack.pop();
        out
    }
}

struct Scope<'r, 'a> {
    run: &'r mut Run<'a>,
    target: Target,
    prop: Prop,
    frame: i32,
    posterize: Option<f64>,
    user_seed: u64,
    timeless: bool,
    wiggles: u64,
    randoms: u64,
    locals: HashMap<String, V>,
}

impl Scope<'_, '_> {
    fn outcome(&mut self, text: &str) -> R<V> {
        let mut result = V::Nothing;
        for stmt in read(text)? {
            match stmt {
                Stmt::Let(name, node) => {
                    let v = self.eval(&node)?;
                    let v = self.number_or_list(v)?;
                    self.locals.insert(name, v);
                }
                Stmt::Do(node) => result = self.eval(&node)?,
            }
        }
        let result = self.plain(result)?;
        let dims = dims(self.prop);
        let numbers = match &result {
            V::Nothing => return Err(type_error("The expression ends without a value")),
            V::Text(_) => return Err(type_error("The expression ends on a text, not a number")),
            V::Num(n) if dims == 1 => vec![*n],
            V::List(l) if dims > 1 && l.len() == dims => l.clone(),
            V::List(l) if dims == 1 => {
                return Err(type_error(format!(
                    "{} is one number and the expression gives {}",
                    self.prop,
                    l.len()
                )))
            }
            V::List(l) => {
                return Err(type_error(format!(
                    "{} is {dims} numbers and the expression gives {}",
                    self.prop,
                    l.len()
                )))
            }
            V::Num(_) => {
                return Err(type_error(format!(
                    "{} is {dims} numbers and the expression gives 1",
                    self.prop
                )))
            }
            _ => {
                return Err(type_error(
                    "The expression ends on something that is not a number",
                ))
            }
        };
        if numbers.iter().any(|n| !n.is_finite()) {
            return Err(type_error(
                "The expression gives a number that is not finite",
            ));
        }
        if self.prop == Prop::Zoom && numbers[0] <= 0.0 {
            return Err(type_error("A camera's zoom must be greater than zero"));
        }
        if self.prop == Prop::Opacity {
            let r = numbers[0];
            let r = if r > 0.0 { r } else { 0.0 };
            return Ok(V::Num(if r < 100.0 { r } else { 100.0 }));
        }
        Ok(result)
    }

    fn step(&mut self) -> R<()> {
        self.run.steps += 1;
        if self.run.steps > MAX_STEPS {
            return Err(ExprError {
                id: DiagnosticId::ExpressionTimeout,
                message: format!("More than {MAX_STEPS} steps"),
            });
        }
        Ok(())
    }

    fn rate(&self) -> (f64, f64) {
        let r = self.run.comp.frame_rate;
        (r.numerator() as f64, r.denominator() as f64)
    }

    fn time(&self) -> f64 {
        let (num, den) = self.rate();
        let f = self.frame as f64 * den;
        match self.posterize {
            None => f / num,
            Some(p) => (f * p / num + 1e-9).floor() / p,
        }
    }

    fn frame_of(&self, seconds: f64) -> i32 {
        let (num, den) = self.rate();
        let x = seconds * num / den;
        let m = (x.abs() + 0.5).floor();
        (if x >= 0.0 { m } else { -m }) as i32
    }

    fn seed(&self) -> u64 {
        let owner = match &self.target {
            Target::Camera => format!("{}/camera", self.run.comp.id.as_str()),
            Target::Layer(id) => id.as_str().to_string(),
        };
        mix(
            fnv1a64(&format!("{owner}/{}", self.prop.as_str())),
            self.user_seed,
        )
    }

    fn pre(&self, frame: i32) -> R<V> {
        self.run.keyed(&self.target, self.prop, frame)
    }

    fn value_of(&mut self, target: &Target, prop: Prop, own: bool, frame: i32) -> R<V> {
        if own {
            self.run.keyed(target, prop, frame)
        } else {
            self.run.finished(target, prop, frame)
        }
    }

    fn plain(&mut self, v: V) -> R<V> {
        match v {
            V::Prop(t, p, own) => self.value_of(&t, p, own, self.frame),
            v => Ok(v),
        }
    }

    fn number_or_list(&mut self, v: V) -> R<V> {
        match self.plain(v)? {
            v @ (V::Num(_) | V::List(_)) => Ok(v),
            _ => Err(type_error(
                "Only a number or a list of numbers can be used here",
            )),
        }
    }

    fn number(&mut self, v: V, what: &str) -> R<f64> {
        match self.plain(v)? {
            V::Num(n) => Ok(n),
            _ => Err(type_error(format!("{what} must be one number"))),
        }
    }

    fn eval(&mut self, node: &Node) -> R<V> {
        self.step()?;
        match node {
            Node::Num(n) => Ok(V::Num(*n)),
            Node::Str(s) => Ok(V::Text(s.clone())),
            Node::Name(name) => {
                if let Some(v) = self.locals.get(name) {
                    return Ok(v.clone());
                }
                Ok(match name.as_str() {
                    "time" => V::Num(self.time()),
                    "value" => self.pre(self.frame)?,
                    "thisProperty" => V::Prop(self.target.clone(), self.prop, true),
                    "thisLayer" => match &self.target {
                        Target::Camera => {
                            return Err(syntax("The camera is not a layer, so it has no thisLayer"))
                        }
                        t => V::Layer(t.clone()),
                    },
                    "thisComp" => V::ThisComp,
                    "Math" => V::Math,
                    other => V::Func(
                        ROOTS
                            .iter()
                            .find(|r| **r == other)
                            .expect("the reader lets only roots and locals through"),
                    ),
                })
            }
            Node::Array(items) => {
                let mut out = Vec::with_capacity(items.len());
                for n in items {
                    let v = self.eval(n)?;
                    out.push(self.number(v, "Each item of a list")?);
                }
                if out.len() > 4 {
                    return Err(type_error("A list holds at most four numbers"));
                }
                Ok(V::List(out))
            }
            Node::Neg(inner) => {
                let v = self.eval(inner)?;
                Ok(match self.number_or_list(v)? {
                    V::List(l) => V::List(l.into_iter().map(|x| -x).collect()),
                    V::Num(n) => V::Num(-n),
                    _ => unreachable!(),
                })
            }
            Node::Bin(op, a, b) => {
                let a = self.eval(a)?;
                let b = self.eval(b)?;
                self.arith(*op, a, b)
            }
            Node::Index(list, index) => {
                let v = self.eval(list)?;
                let v = self.plain(v)?;
                let i = self.eval(index)?;
                let i = self.number(i, "A position in a list")?;
                let V::List(l) = v else {
                    return Err(type_error("Only a list can be indexed"));
                };
                if i.fract() != 0.0 || !(0.0 <= i && i < l.len() as f64) {
                    return Err(type_error(format!("A list of {} has no item {i}", l.len())));
                }
                Ok(V::Num(l[i as usize]))
            }
            Node::Get(obj, name) => {
                let obj = self.eval(obj)?;
                self.member(obj, name)
            }
            Node::Call(func, args) => {
                let func = self.eval(func)?;
                self.call(func, args)
            }
        }
    }

    fn arith(&mut self, op: char, a: V, b: V) -> R<V> {
        let a = self.number_or_list(a)?;
        let b = self.number_or_list(b)?;
        Ok(match (a, b) {
            (V::Num(a), V::Num(b)) => V::Num(divide(op, a, b)),
            (V::List(a), V::List(b)) if op == '+' || op == '-' => {
                if a.len() != b.len() {
                    return Err(type_error(format!(
                        "A list of {} and a list of {} cannot be added",
                        a.len(),
                        b.len()
                    )));
                }
                V::List(
                    a.iter()
                        .zip(&b)
                        .map(|(x, y)| if op == '+' { x + y } else { x - y })
                        .collect(),
                )
            }
            (V::List(v), V::Num(n)) | (V::Num(n), V::List(v)) if op == '*' => {
                V::List(v.into_iter().map(|x| x * n).collect())
            }
            (V::List(v), V::Num(n)) if op == '/' => {
                V::List(v.into_iter().map(|x| divide('/', x, n)).collect())
            }
            (a, b) => {
                let kind = |v: &V| match v {
                    V::List(_) => "a list",
                    _ => "a number",
                };
                return Err(type_error(format!(
                    "\"{op}\" cannot be used between {} and {}",
                    kind(&a),
                    kind(&b)
                )));
            }
        })
    }

    fn member(&mut self, obj: V, name: &str) -> R<V> {
        let comp = self.run.comp;
        match (&obj, name) {
            (V::ThisComp, "layer") => return Ok(V::Func("layer")),
            (V::ThisComp, "activeCamera") => return Ok(V::Layer(Target::Camera)),
            (V::ThisComp, "width") => return Ok(V::Num(comp.width as f64)),
            (V::ThisComp, "height") => return Ok(V::Num(comp.height as f64)),
            (V::ThisComp, "frameDuration") => {
                let (num, den) = self.rate();
                return Ok(V::Num(den / num));
            }
            (V::Math, "PI") => return Ok(V::Num(std::f64::consts::PI)),
            (V::Math, m) => {
                if let Some(m) = MATH.iter().find(|x| **x == m) {
                    return Ok(V::Func(m));
                }
            }
            (V::Layer(Target::Layer(_)), "transform") => return Ok(obj),
            (V::Layer(t), n) => {
                let camera = *t == Target::Camera;
                let n = if !camera && n == "anchorPoint" {
                    "anchor"
                } else {
                    n
                };
                let found = if camera {
                    &CAMERA_PROPS[..]
                } else {
                    &LAYER_PROPS[..]
                }
                .iter()
                .find(|p| p.as_str() == n);
                if let Some(p) = found {
                    return Ok(V::Prop(t.clone(), *p, false));
                }
            }
            (V::Prop(t, p, own), "value") => {
                let (t, p, own) = (t.clone(), *p, *own);
                return self.value_of(&t, p, own, self.frame);
            }
            (V::Prop(t, p, own), "valueAtTime") => return Ok(V::Method(t.clone(), *p, *own)),
            _ => {}
        }
        Err(syntax(format!(
            "\"{name}\" is not something this can be asked for"
        )))
    }

    fn call(&mut self, func: V, arg_nodes: &[Node]) -> R<V> {
        if let V::Method(t, p, own) = func {
            if arg_nodes.len() != 1 {
                return Err(type_error("valueAtTime takes one time, in seconds"));
            }
            let v = self.eval(&arg_nodes[0])?;
            let secs = self.number(v, "A time")?;
            if !secs.is_finite() {
                return Err(type_error("A time must be finite"));
            }
            let frame = self.frame_of(secs);
            return self.value_of(&t, p, own, frame);
        }
        let V::Func(name) = func else {
            return Err(syntax("Only a function can be called"));
        };
        let mut args = Vec::with_capacity(arg_nodes.len());
        for n in arg_nodes {
            args.push(self.eval(n)?);
        }
        if name == "layer" {
            let [V::Text(id)] = &args[..] else {
                return Err(type_error("thisComp.layer takes a layer's id, in quotes"));
            };
            let target = Target::Layer(Id::new(id.clone()));
            property(self.run.comp, &target, Prop::Position)?;
            return Ok(V::Layer(target));
        }
        if name != "loopOut" && args.iter().any(|a| matches!(a, V::Text(_))) {
            return Err(type_error(format!("{name} does not take a text")));
        }
        if MATH.contains(&name) {
            return self.math(name, args);
        }
        match name {
            "posterizeTime" => {
                if args.len() != 1 {
                    return Err(type_error("posterizeTime takes one frame rate"));
                }
                let p = self.number(args.remove(0), "A frame rate")?;
                if !(p.is_finite() && p > 0.0) {
                    return Err(type_error(
                        "posterizeTime needs a frame rate greater than zero",
                    ));
                }
                self.posterize = Some(p);
                Ok(V::Nothing)
            }
            "seedRandom" => {
                if !(1..=2).contains(&args.len()) {
                    return Err(type_error(
                        "seedRandom takes a seed and, if wanted, timeless",
                    ));
                }
                let mut args = args.into_iter();
                let s = self.number(args.next().expect("one"), "A seed")?;
                if !s.is_finite() {
                    return Err(type_error("A seed must be finite"));
                }
                self.user_seed = wrap_u64(s.floor());
                self.timeless = match args.next() {
                    Some(t) => self.number(t, "Timeless")? != 0.0,
                    None => false,
                };
                Ok(V::Nothing)
            }
            "wiggle" => self.wiggle(args),
            "random" => self.random(args),
            "linear" | "ease" => self.remap(name, args),
            "clamp" => {
                if args.len() != 3 {
                    return Err(type_error("clamp takes a value, a lowest and a highest"));
                }
                let mut args = args.into_iter();
                let v = self.number_or_list(args.next().expect("three"))?;
                let lo = self.number(args.next().expect("three"), "The lowest")?;
                let hi = self.number(args.next().expect("three"), "The highest")?;
                let one = |x: f64| {
                    let y = if x > lo { x } else { lo };
                    if y < hi {
                        y
                    } else {
                        hi
                    }
                };
                Ok(match v {
                    V::List(l) => V::List(l.into_iter().map(one).collect()),
                    V::Num(n) => V::Num(one(n)),
                    _ => unreachable!(),
                })
            }
            "loopOut" => self.loop_out(args),
            _ => Err(syntax(format!("\"{name}\" cannot be called"))),
        }
    }

    fn math(&mut self, name: &str, args: Vec<V>) -> R<V> {
        let mut nums = Vec::with_capacity(args.len());
        for a in args {
            nums.push(self.number(a, &format!("Math.{name}'s argument"))?);
        }
        let want = match name {
            "pow" => Some(2),
            "min" | "max" => None,
            _ => Some(1),
        };
        if want.is_some_and(|w| nums.len() != w) || (want.is_none() && nums.is_empty()) {
            return Err(type_error(match want {
                Some(w) => format!("Math.{name} takes {w} number"),
                None => format!("Math.{name} takes at least one number"),
            }));
        }
        let x = nums[0];
        Ok(V::Num(match name {
            "round" => (x + 0.5).floor(),
            "abs" => x.abs(),
            "floor" => x.floor(),
            "ceil" => x.ceil(),
            "sqrt" => {
                if x >= 0.0 {
                    x.sqrt()
                } else {
                    f64::NAN
                }
            }
            "pow" => {
                // Document 09: an impossible power is not-a-number, including one too large to
                // hold and nought to a negative power.
                let r = x.powf(nums[1]);
                if !r.is_finite() && x.is_finite() && nums[1].is_finite() {
                    f64::NAN
                } else {
                    r
                }
            }
            "min" => nums[1..].iter().fold(x, |m, &y| if y < m { y } else { m }),
            "max" => nums[1..].iter().fold(x, |m, &y| if y > m { y } else { m }),
            "sin" => x.sin(),
            "cos" => x.cos(),
            _ => x.tan(),
        }))
    }

    fn wiggle(&mut self, args: Vec<V>) -> R<V> {
        if !(2..=5).contains(&args.len()) {
            return Err(type_error(
                "wiggle takes a frequency and an amount, then up to three more",
            ));
        }
        let n = args.len();
        let mut args = args.into_iter();
        let freq = self.number(args.next().expect("two"), "wiggle's frequency")?;
        let amp = self.number(args.next().expect("two"), "wiggle's amount")?;
        let octaves = match args.next() {
            Some(v) => self.number(v, "wiggle's octaves")?,
            None => 1.0,
        };
        let mult = match args.next() {
            Some(v) => self.number(v, "wiggle's amount multiplier")?,
            None => 0.5,
        };
        let t = match args.next() {
            Some(v) => self.number(v, "wiggle's time")?,
            None => self.time(),
        };
        debug_assert!(n <= 5);
        if [freq, amp, octaves, mult, t].iter().any(|v| !v.is_finite()) {
            return Err(type_error("wiggle's numbers must be finite"));
        }
        let octaves = octaves.floor();
        if freq < 0.0 || !(1.0..=MAX_OCTAVES).contains(&octaves) {
            return Err(type_error(format!(
                "wiggle needs a frequency of zero or more and 1 to {MAX_OCTAVES} octaves"
            )));
        }
        let call = self.wiggles;
        self.wiggles += 1;
        let seed = self.seed();
        let base = match self.pre(self.frame)? {
            V::List(l) => l,
            V::Num(x) => vec![x],
            _ => unreachable!(),
        };
        let out: Vec<f64> = base
            .iter()
            .enumerate()
            .map(|(d, v)| {
                let (mut total, mut weight, mut scale) = (0.0, 1.0, 1.0);
                for k in 0..octaves as u64 {
                    let stream = call
                        .wrapping_mul(256)
                        .wrapping_add(d as u64 * 16)
                        .wrapping_add(k);
                    total += weight * noise(seed, stream, t * freq * scale);
                    weight *= mult;
                    scale *= 2.0;
                }
                v + amp * total
            })
            .collect();
        Ok(if dims(self.prop) > 1 {
            V::List(out)
        } else {
            V::Num(out[0])
        })
    }

    fn random(&mut self, args: Vec<V>) -> R<V> {
        if args.len() > 2 {
            return Err(type_error("random takes at most two limits"));
        }
        let call = self.randoms;
        self.randoms += 1;
        let seed = self.seed();
        let key = if self.timeless {
            0
        } else {
            self.time().to_bits()
        };
        let r = |d: u64| unit(mix(mix(seed, 0x10000 + call * 16 + d), key));
        if args.is_empty() {
            return Ok(V::Num(r(0)));
        }
        let two = args.len() == 2;
        let mut vals = Vec::with_capacity(2);
        for a in args {
            vals.push(self.number_or_list(a)?);
        }
        let hi = vals.pop().expect("one or two");
        let lo = vals.pop().unwrap_or(V::Num(0.0));
        match (lo, hi) {
            (V::Num(lo), V::Num(hi)) => Ok(V::Num(lo + r(0) * (hi - lo))),
            (V::List(lo), V::List(hi)) if lo.len() != hi.len() => {
                Err(type_error("random's two lists must be the same length"))
            }
            (V::List(lo), V::List(hi)) => Ok(V::List(
                lo.iter()
                    .zip(&hi)
                    .enumerate()
                    .map(|(d, (a, b))| a + r(d as u64) * (b - a))
                    .collect(),
            )),
            (V::Num(_), V::List(hi)) if !two => Ok(V::List(
                hi.iter()
                    .enumerate()
                    .map(|(d, b)| 0.0 + r(d as u64) * (b - 0.0))
                    .collect(),
            )),
            _ => Err(type_error(
                "random's two limits must both be numbers or both be lists",
            )),
        }
    }

    fn remap(&mut self, name: &str, args: Vec<V>) -> R<V> {
        let mut args = args;
        if args.len() == 3 {
            args.insert(1, V::Num(1.0));
            args.insert(1, V::Num(0.0));
        }
        if args.len() != 5 {
            return Err(type_error(format!(
                "{name} takes a time and two values, or a time, two limits and two values"
            )));
        }
        let mut args = args.into_iter();
        let t = self.number(args.next().expect("five"), &format!("{name}'s input"))?;
        let mut t0 = self.number(args.next().expect("five"), "A limit")?;
        let mut t1 = self.number(args.next().expect("five"), "A limit")?;
        let mut v0 = self.number_or_list(args.next().expect("five"))?;
        let mut v1 = self.number_or_list(args.next().expect("five"))?;
        let same = match (&v0, &v1) {
            (V::Num(_), V::Num(_)) => true,
            (V::List(a), V::List(b)) => a.len() == b.len(),
            _ => false,
        };
        if !same {
            return Err(type_error(format!(
                "{name}'s two values must be the same shape"
            )));
        }
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
            std::mem::swap(&mut v0, &mut v1);
        }
        let f = if t <= t0 {
            0.0
        } else if t >= t1 {
            1.0
        } else {
            let f = (t - t0) / (t1 - t0);
            if name == "ease" {
                f * f * (3.0 - 2.0 * f)
            } else {
                f
            }
        };
        Ok(match (v0, v1) {
            (V::List(a), V::List(b)) => {
                V::List(a.iter().zip(&b).map(|(a, b)| a + (b - a) * f).collect())
            }
            (V::Num(a), V::Num(b)) => V::Num(a + (b - a) * f),
            _ => unreachable!(),
        })
    }

    fn loop_out(&mut self, args: Vec<V>) -> R<V> {
        if args.len() > 2 {
            return Err(type_error(
                "loopOut takes a kind and, if wanted, how many keyframes",
            ));
        }
        let mut args = args.into_iter();
        let kind = match args.next() {
            None => "cycle".to_string(),
            Some(V::Text(k))
                if ["cycle", "pingpong", "offset", "continue"].contains(&k.as_str()) =>
            {
                k
            }
            Some(_) => {
                return Err(type_error(
                    "loopOut's kind is \"cycle\", \"pingpong\", \"offset\" or \"continue\"",
                ))
            }
        };
        let n = match args.next() {
            Some(v) => self.number(v, "How many keyframes")?,
            None => 0.0,
        };
        if !n.is_finite() || n < 0.0 || n.fract() != 0.0 {
            return Err(type_error(
                "loopOut's keyframe count is a whole number, zero or more",
            ));
        }
        let keys: Vec<i64> = property(self.run.comp, &self.target, self.prop)?
            .keyframes()
            .iter()
            .map(|k| k.frame as i64)
            .collect();
        let f = self.frame as i64;
        let Some(&last) = keys.last() else {
            return self.pre(self.frame);
        };
        if keys.len() < 2 || f <= last {
            return self.pre(self.frame);
        }
        let first = if n == 0.0 {
            keys[0]
        } else {
            let back = (keys.len() - 1).saturating_sub(n.min(usize::MAX as f64) as usize);
            keys[back]
        };
        let period = last - first;
        if period <= 0 {
            return self.pre(self.frame);
        }
        let pre = |s: &Self, g: i64| s.pre(g as i32);
        // a + s * b, item by item.
        let vec = |a: V, b: V, s: f64| match (a, b) {
            (V::List(a), V::List(b)) => V::List(a.iter().zip(&b).map(|(x, y)| x + s * y).collect()),
            (V::Num(a), V::Num(b)) => V::Num(a + s * b),
            _ => unreachable!("one property's keys are all one kind"),
        };
        Ok(match kind.as_str() {
            "cycle" => pre(self, first + (f - first).rem_euclid(period))?,
            "pingpong" => {
                let q = f - last;
                let (laps, r) = (q.div_euclid(period), q.rem_euclid(period));
                if laps % 2 == 0 {
                    pre(self, last - r)?
                } else {
                    pre(self, first + r)?
                }
            }
            "offset" => {
                let (laps, r) = (
                    (f - first).div_euclid(period),
                    (f - first).rem_euclid(period),
                );
                let gain = vec(pre(self, last)?, pre(self, first)?, -1.0);
                vec(pre(self, first + r)?, gain, laps as f64)
            }
            _ => {
                let step = vec(pre(self, last)?, pre(self, last - 1)?, -1.0);
                vec(pre(self, last)?, step, (f - last) as f64)
            }
        })
    }
}

/// A whole number as its 64-bit two's complement, however large, which is what document 09's
/// seed takes. `%` and the Sterbenz subtraction are both exact.
fn wrap_u64(n: f64) -> u64 {
    const TWO_64: f64 = 18_446_744_073_709_551_616.0;
    let r = n % TWO_64;
    if r >= 0.0 {
        r as u64
    } else if r >= -(TWO_64 / 2.0) {
        r as i64 as u64
    } else {
        (r + TWO_64) as u64
    }
}

/// Number with number, where D-59 lets division by zero through to the result check.
fn divide(op: char, a: f64, b: f64) -> f64 {
    match op {
        '+' => a + b,
        '-' => a - b,
        '*' => a * b,
        _ if b == 0.0 => {
            if a != 0.0 && op == '/' {
                f64::INFINITY.copysign(a) * 1f64.copysign(b)
            } else {
                f64::NAN
            }
        }
        '/' => a / b,
        _ => a % b,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_noise_words_are_the_published_ones() {
        // SplitMix64's first output from seed 0 and FNV-1a of the empty text and of "a".
        assert_eq!(splitmix64(0), 0xE220_A839_7B1D_CDAF);
        assert_eq!(fnv1a64(""), 0xCBF2_9CE4_8422_2325);
        assert_eq!(fnv1a64("a"), 0xAF63_DC4C_8601_EC8C);
        assert_eq!(wrap_u64(-1.0), u64::MAX);
        assert_eq!(wrap_u64(-(2f64.powi(64)) - 4096.0), u64::MAX - 4095);
        assert_eq!(wrap_u64(2f64.powi(64) + 4096.0), 4096);
    }
}
