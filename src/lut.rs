//! D-182: Color Lookup's .cube files. Document 19's "Colour lookup files" is the reading rule and
//! document 21 the colour rule; `tools/cube_lut_reference.py` is the reference for both, and a
//! refused file's reason here is its reason word for word.
//!
//! D-395: and Arbitrary Map's Photoshop arbitrary map (.amp) files, read as Adobe's file format
//! specification gives them; `tools/arbitrary_map_reference.py` is the reference, reasons too.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock};
use std::time::SystemTime;

use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::effects::{Effect, EffectInstance};
use crate::model::{AssetKind, Project};

/// A .cube file as read: `size` points a side, one or three dimensions, the domain `lo..hi` of
/// each channel, and the table, red changing fastest.
#[derive(Debug, PartialEq)]
pub struct Cube {
    three: bool,
    size: usize,
    lo: [f64; 3],
    hi: [f64; 3],
    table: Vec<[f32; 3]>,
}

/// The file an effect's `lut` named at this frame, read. It is not a setting and is never saved;
/// it rides in the effect so the effect cache's key changes when the file does.
#[derive(Clone)]
pub struct Table(pub Arc<Cube>);

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || *self.0 == *other.0
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table({} points a side)", self.0.size)
    }
}

/// D-395: a .amp file as read, the table of each channel it holds, in the order master, red,
/// green, blue and alpha; a channel it has no table for is straight.
#[derive(PartialEq)]
pub struct Amp([Option<[u8; 256]>; 5]);

/// The .amp file an Arbitrary Map's `map` named at this frame, read; as [`Table`], never saved.
#[derive(Clone)]
pub struct Map(pub Arc<Amp>);

impl PartialEq for Map {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0) || *self.0 == *other.0
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Map({} tables)", self.0 .0.iter().flatten().count())
    }
}

/// D-395: a .amp file's bytes as its tables, or the reason it is refused. One table is the
/// master, exactly three red, green and blue, any other count the master and then the channels
/// in turn.
pub fn parse_amp(bytes: &[u8]) -> Result<Amp, String> {
    let n = bytes.len();
    if n == 0 {
        return Err("the file is empty".to_string());
    }
    if n % 256 != 0 {
        return Err(format!("the file is {n} bytes long, not a whole number of 256-byte tables"));
    }
    let k = n / 256;
    if k > 5 {
        return Err(format!(
            "the file holds {k} tables, and this program reads at most five: master, red, green, \
             blue and alpha"
        ));
    }
    let first = if k == 3 { 1 } else { 0 };
    let mut tables = [None; 5];
    for (i, t) in bytes.chunks_exact(256).enumerate() {
        tables[first + i] = Some(t.try_into().expect("256 bytes"));
    }
    Ok(Amp(tables))
}

impl Amp {
    /// Channel `c`'s table cycled right by `phase` at `u`, between entries mixed, the entry past
    /// 255 being entry 0; a channel the file has no table for is straight and not cycled.
    fn curve(&self, c: usize, phase: f64, u: f64) -> f64 {
        let Some(t) = &self.0[c] else {
            return u;
        };
        let v = (u - phase).rem_euclid(256.0);
        let i = (v.floor() as usize).min(255);
        let f = v - i as f64;
        f64::from(t[i]) * (1.0 - f) + f64::from(t[(i + 1) % 256]) * f
    }

    /// The colour tables at `phase` as one 1D lookup of 256 entries a colour, each channel's
    /// table then the master's, for Color Lookup's rule and its card pass.
    pub(crate) fn colours(&self, phase: f64) -> Cube {
        let table = (0..256)
            .map(|x| std::array::from_fn(|c| (self.curve(0, phase, self.curve(c + 1, phase, x as f64)) / 255.0) as f32))
            .collect();
        Cube { three: false, size: 256, lo: [0.0; 3], hi: [1.0; 3], table }
    }

    /// The alpha table at `phase`, 0 to 255 in and out, when the file holds one.
    pub(crate) fn alpha(&self, phase: f64) -> Option<impl Fn(f64) -> f64 + Sync + '_> {
        self.0[4].is_some().then(move || move |u| self.curve(4, phase, u))
    }
}

/// `k` finite numbers, or `None`.
fn finite(tokens: &[&str], k: usize) -> Option<Vec<f64>> {
    if tokens.len() != k {
        return None;
    }
    let v: Vec<f64> = tokens.iter().map(|t| t.parse::<f64>().ok()).collect::<Option<_>>()?;
    v.iter().all(|x| x.is_finite()).then_some(v)
}

/// Document 19's reading rule: the file's bytes as a table, or the reason it is refused.
pub fn parse(bytes: &[u8]) -> Result<Cube, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "the file is not text".to_string())?;
    let text = text.strip_prefix('\u{feff}').unwrap_or(text);
    let mut seen: Vec<&str> = Vec::new();
    let mut table: Vec<[f32; 3]> = Vec::new();
    let mut sizes: Vec<(&str, usize)> = Vec::new();
    let (mut lo, mut hi) = ([0.0; 3], [1.0; 3]);
    for (n, line) in text.split('\n').enumerate().map(|(i, l)| (i + 1, l.trim())) {
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if !line.as_bytes()[0].is_ascii_alphabetic() {
            let v = finite(&tokens, 3)
                .ok_or_else(|| format!("line {n}: a table line must be three numbers"))?;
            table.push([v[0] as f32, v[1] as f32, v[2] as f32]);
            continue;
        }
        let (word, rest) = (tokens[0], &tokens[1..]);
        if !table.is_empty() {
            return Err(format!("line {n}: {word} comes after the table has begun"));
        }
        const KEYWORDS: [&str; 7] = [
            "TITLE",
            "LUT_3D_SIZE",
            "LUT_1D_SIZE",
            "DOMAIN_MIN",
            "DOMAIN_MAX",
            "LUT_1D_INPUT_RANGE",
            "LUT_3D_INPUT_RANGE",
        ];
        if !KEYWORDS.contains(&word) {
            return Err(format!(
                "line {n}: {word} is not a keyword of a .cube file this program reads"
            ));
        }
        if seen.contains(&word) {
            return Err(format!("line {n}: {word} is given twice"));
        }
        seen.push(word);
        match word {
            "LUT_3D_SIZE" | "LUT_1D_SIZE" => {
                let top = if word == "LUT_3D_SIZE" { 256 } else { 65536 };
                let size = match rest {
                    [r] if r.bytes().all(|b| b.is_ascii_digit()) => r.parse::<usize>().ok(),
                    _ => None,
                }
                .filter(|s| (2..=top).contains(s))
                .ok_or_else(|| format!("line {n}: {word} must be one whole number from 2 to {top}"))?;
                sizes.push((word, size));
            }
            "DOMAIN_MIN" | "DOMAIN_MAX" => {
                let v = finite(rest, 3)
                    .ok_or_else(|| format!("line {n}: {word} must be three numbers"))?;
                let v = [v[0], v[1], v[2]];
                if word == "DOMAIN_MIN" {
                    lo = v;
                } else {
                    hi = v;
                }
            }
            "TITLE" => {}
            _ => {
                let v = finite(rest, 2)
                    .ok_or_else(|| format!("line {n}: {word} must be two numbers"))?;
                (lo, hi) = ([v[0]; 3], [v[1]; 3]);
            }
        }
    }
    let (word, size) = match sizes[..] {
        [one] => one,
        [] => return Err("the file gives no LUT_3D_SIZE or LUT_1D_SIZE".to_string()),
        _ => {
            return Err("the file gives both LUT_1D_SIZE and LUT_3D_SIZE, a 1D table before a 3D \
                        one, which this program does not read"
                .to_string())
        }
    };
    let has = |w: &str| seen.contains(&w);
    let ways = [
        has("DOMAIN_MIN") || has("DOMAIN_MAX"),
        has("LUT_1D_INPUT_RANGE"),
        has("LUT_3D_INPUT_RANGE"),
    ];
    if ways.iter().filter(|w| **w).count() > 1 {
        return Err("the file gives its domain more than one way".to_string());
    }
    if (0..3).any(|c| hi[c] <= lo[c]) {
        return Err("the domain's top must be above its bottom in each colour".to_string());
    }
    let three = word == "LUT_3D_SIZE";
    let need = if three { size * size * size } else { size };
    if table.len() != need {
        return Err(format!(
            "the table has {} lines where {word} {size} needs {need}",
            table.len()
        ));
    }
    Ok(Cube { three, size, lo, hi, table })
}

impl Cube {
    /// Document 21: what the table makes of the encoded straight colour `e`.
    pub(crate) fn lookup(&self, e: [f64; 3]) -> [f64; 3] {
        let n = self.size;
        let at: [(usize, f64); 3] = std::array::from_fn(|c| {
            let u = (e[c].clamp(self.lo[c], self.hi[c]) - self.lo[c]) / (self.hi[c] - self.lo[c])
                * (n - 1) as f64;
            let i = (u.floor().max(0.0) as usize).min(n - 2);
            (i, u - i as f64)
        });
        let t = |i: usize, c: usize| self.table[i][c] as f64;
        if !self.three {
            return std::array::from_fn(|c| {
                let (i, f) = at[c];
                t(i, c) * (1.0 - f) + t(i + 1, c) * f
            });
        }
        let [(ri, rf), (gi, gf), (bi, bf)] = at;
        let weight = |d: usize, f: f64| if d == 1 { f } else { 1.0 - f };
        let mut out = [0.0; 3];
        for db in 0..2 {
            for dg in 0..2 {
                for dr in 0..2 {
                    let w = weight(dr, rf) * weight(dg, gf) * weight(db, bf);
                    let i = (ri + dr) + (gi + dg) * n + (bi + db) * n * n;
                    for (c, o) in out.iter_mut().enumerate() {
                        *o += w * t(i, c);
                    }
                }
            }
        }
        out
    }

    /// B-123: the table as the card's colour pass reads it, `[three, size, lo, hi, table...]`.
    pub(crate) fn packed(&self) -> Vec<f64> {
        let mut k = vec![f64::from(u8::from(self.three)), self.size as f64];
        k.extend(self.lo.iter().chain(&self.hi));
        k.extend(self.table.iter().flatten().map(|&v| f64::from(v)));
        k
    }

    /// How many colours the table holds.
    pub(crate) fn entries(&self) -> usize {
        self.table.len()
    }
}

type Read = Result<Arc<Cube>, String>;
type Held<T> = OnceLock<Mutex<HashMap<PathBuf, (u64, Option<SystemTime>, Result<Arc<T>, String>)>>>;

/// A .cube file read and parsed, kept while its length and time of change stay the same, so a
/// frame does not read it again and a file changed on disk is read afresh.
pub fn read(path: &Path) -> Read {
    static READ: Held<Cube> = OnceLock::new();
    held(&READ, path, parse)
}

/// D-395: a .amp file read and parsed, kept as [`read`] keeps a .cube file.
pub fn read_map(path: &Path) -> Result<Arc<Amp>, String> {
    static READ: Held<Amp> = OnceLock::new();
    held(&READ, path, parse_amp)
}

// ponytail: never emptied; a session reads a handful of lookup files. Evict by age if one day
// it reads hundreds.
fn held<T>(store: &Held<T>, path: &Path, parse: fn(&[u8]) -> Result<T, String>) -> Result<Arc<T>, String> {
    let meta = crate::cache::looked_at(path).map_err(|e| format!("it could not be read: {e}"))?;
    let stamp = (meta.len(), meta.modified().ok());
    let mut held = store.get_or_init(Default::default).lock().expect("the lookup files' lock");
    if let Some((len, modified, read)) = held.get(path) {
        if (*len, *modified) == stamp {
            return read.clone();
        }
    }
    let read = std::fs::read(path)
        .map_err(|e| format!("it could not be read: {e}"))
        .and_then(|bytes| parse(&bytes))
        .map(Arc::new);
    held.insert(path.to_path_buf(), (stamp.0, stamp.1, read.clone()));
    read
}

/// The `lut` of `effect`, or an Arbitrary Map's `map` (D-395), when it names nothing the project
/// has as a lookup file.
pub fn dangling<'a>(project: &Project, effect: &'a Effect) -> Option<&'a str> {
    match effect {
        Effect::ColorLookup { lut, .. } | Effect::ArbitraryMap { map: lut, .. }
            if !lut.is_empty()
                && !project
                    .assets
                    .iter()
                    .any(|a| a.id.as_str() == lut && a.kind == AssetKind::Lut) =>
        {
            Some(lut)
        }
        _ => None,
    }
}

/// The kind of file `effect` reads: a .amp for an Arbitrary Map (D-395), else a .cube.
pub fn file_kind(effect: &Effect) -> &'static str {
    if matches!(effect, Effect::ArbitraryMap { .. }) {
        ".amp"
    } else {
        ".cube"
    }
}

/// What opening the project and each frame say of a `lut` that `dangling` found in `effect`.
pub(crate) fn not_a_lookup_file(layer: &str, effect: &Effect, lut: &str) -> Diagnostic {
    let (name, setting) = match effect {
        Effect::ArbitraryMap { .. } => ("Arbitrary Map", "map"),
        _ => ("Color Lookup", "lut"),
    };
    Diagnostic::new(
        DiagnosticId::EffectParameterInvalid,
        Severity::Warning,
        format!(
            "The layer \"{layer}\"'s {name} names {lut}, which is not a lookup file of this \
             project, so the layer is drawn without it."
        ),
        format!(
            "D-182: {name}'s {setting} is the id of an asset of kind lut, or empty. The setting \
             is kept as written."
        ),
    )
    .with_remediation(format!("Choose a {} file on the effect's card.", file_kind(effect)))
}

/// The file each switched-on Color Lookup or Arbitrary Map (D-395) of `effects` names, read into
/// its `table`, and what kept one from being read: a `lut` naming no lookup file, a missing
/// file, or a refused one. An effect left without its table leaves the layer as it is.
pub(crate) fn fill(
    effects: &mut [EffectInstance],
    project: &Project,
    root: &Path,
    layer: &str,
) -> Vec<Diagnostic> {
    let mut said = Vec::new();
    for instance in effects.iter_mut().filter(|i| i.enabled) {
        if let Some(lut) = dangling(project, &instance.effect) {
            said.push(not_a_lookup_file(layer, &instance.effect, lut));
            continue;
        }
        let (Effect::ColorLookup { lut, .. } | Effect::ArbitraryMap { map: lut, .. }) = &instance.effect else {
            continue;
        };
        let Some(asset) = project.assets.iter().find(|a| a.id.as_str() == lut) else {
            continue;
        };
        let relative = asset.path.as_deref().unwrap_or("");
        let path = root.join(relative);
        if relative.is_empty() || !crate::cache::looked_at(&path).is_ok_and(|m| m.is_file()) {
            said.push(
                Diagnostic::new(
                    DiagnosticId::MediaMissing,
                    Severity::Warning,
                    format!(
                        "The lookup file \"{}\" is not where the project says it is, so the \
                         layer \"{layer}\" is drawn without its look.",
                        asset.name
                    ),
                    format!(
                        "Asset {} names {relative:?}, looked for at {}. The reference is kept.",
                        asset.id,
                        path.display()
                    ),
                )
                .with_remediation("Relink the lookup file, or put it back where it was."),
            );
            continue;
        }
        let kind = file_kind(&instance.effect);
        let read = match &mut instance.effect {
            Effect::ArbitraryMap { table, .. } => read_map(&path).map(|m| *table = Some(Map(m))),
            Effect::ColorLookup { table, .. } => read(&path).map(|cube| *table = Some(Table(cube))),
            _ => Ok(()),
        };
        match read {
            Ok(()) => {}
            Err(why) => said.push(
                Diagnostic::new(
                    DiagnosticId::MediaDecodeFailed,
                    Severity::Error,
                    format!(
                        "The lookup file \"{}\" cannot be read, so the layer \"{layer}\" is \
                         drawn without its look.",
                        asset.name
                    ),
                    format!("{}: {why}.", path.display()),
                )
                .with_remediation(format!(
                    "Export the look again as a {kind} file, or relink the asset to another one."
                )),
            ),
        }
    }
    said
}
