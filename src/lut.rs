//! D-182: Color Lookup's .cube files. Document 19's "Colour lookup files" is the reading rule and
//! document 21 the colour rule; `tools/cube_lut_reference.py` is the reference for both, and a
//! refused file's reason here is its reason word for word.

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

/// A .cube file read and parsed, kept while its length and time of change stay the same, so a
/// frame does not read it again and a file changed on disk is read afresh.
// ponytail: never emptied; a session reads a handful of lookup files. Evict by age if one day
// it reads hundreds.
pub fn read(path: &Path) -> Read {
    static READ: OnceLock<Mutex<HashMap<PathBuf, (u64, Option<SystemTime>, Read)>>> =
        OnceLock::new();
    let meta = crate::cache::looked_at(path).map_err(|e| format!("it could not be read: {e}"))?;
    let stamp = (meta.len(), meta.modified().ok());
    let mut held = READ.get_or_init(Default::default).lock().expect("the lookup files' lock");
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

/// The `lut` of `effect` when it names nothing the project has as a lookup file.
pub fn dangling<'a>(project: &Project, effect: &'a Effect) -> Option<&'a str> {
    match effect {
        Effect::ColorLookup { lut, .. }
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

/// What opening the project and each frame say of a `lut` that `dangling` found.
pub(crate) fn not_a_lookup_file(layer: &str, lut: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::EffectParameterInvalid,
        Severity::Warning,
        format!(
            "The layer \"{layer}\"'s Color Lookup names {lut}, which is not a lookup file of \
             this project, so the layer is drawn without it."
        ),
        "D-182: Color Lookup's lut is the id of an asset of kind lut, or empty. The setting is \
         kept as written."
            .to_string(),
    )
    .with_remediation("Choose a .cube file on the effect's card.")
}

/// The file each switched-on Color Lookup of `effects` names, read into its `table`, and what
/// kept one from being read: a `lut` naming no lookup file, a missing file, or a refused one.
/// An effect left without its table leaves the layer as it is.
pub(crate) fn fill(
    effects: &mut [EffectInstance],
    project: &Project,
    root: &Path,
    layer: &str,
) -> Vec<Diagnostic> {
    let mut said = Vec::new();
    for instance in effects.iter_mut().filter(|i| i.enabled) {
        if let Some(lut) = dangling(project, &instance.effect) {
            said.push(not_a_lookup_file(layer, lut));
            continue;
        }
        let Effect::ColorLookup { lut, table } = &mut instance.effect else {
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
        match read(&path) {
            Ok(cube) => *table = Some(Table(cube)),
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
                .with_remediation(
                    "Export the look again as a .cube file, or relink the asset to another one.",
                ),
            ),
        }
    }
    said
}
