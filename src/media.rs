//! PNG import and sequence manifests (B-03, requirement R-01).
//!
//! Document 19 line 17: a sequence asset stores a numeric pattern **and** a frame-number-to-file
//! map, "so missing numbers remain missing rather than being silently compacted". That sentence
//! decides the whole design here. The user's selection defines the sequence. The inferred pattern
//! is *descriptive*: it is what the names mostly look like, useful for display and for relink. The
//! map is *authoritative*: it is what will actually be read from disk.
//!
//! The reference shot exists to punish the other design. `layer2/` holds a drawing 13 under the
//! name `layer2_桜_013.png`, which the pattern `layer2_%03d.png` does not generate. An importer
//! that derives its frame list from the pattern reports a false gap at 13 and drops a drawing the
//! user can see in their own folder. An importer that derives the pattern from the frame list
//! reports layer 2 as complete, which it is, and layer 3 as missing exactly drawing 7, which it is.

use std::collections::btree_map::Entry;
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::{BufferError, ImageBuffer};

/// A file name split at its trailing digit run: `layer2_桜_013.png` -> `("layer2_桜_", 13, 3, ".png")`.
struct NameParts {
    prefix: String,
    number: u32,
    digits: usize,
    suffix: String,
}

/// Split a file name at the last run of ASCII digits in its stem.
///
/// The stem, not the whole name, so an extension that contains digits (`.jp2`) cannot be mistaken
/// for the frame number. Returns `None` when the stem has no digits, or when the digit run does
/// not fit a `u32` — a 30-digit run is not a frame number anyone meant to type.
fn split_name(name: &str) -> Option<NameParts> {
    let (stem, ext) = match name.rfind('.') {
        Some(i) => (&name[..i], &name[i..]),
        None => (name, ""),
    };
    let end = stem.rfind(|c: char| c.is_ascii_digit())? + 1;
    let start = stem[..end]
        .rfind(|c: char| !c.is_ascii_digit())
        .map(|i| i + 1)
        .unwrap_or(0);
    let run = &stem[start..end];
    Some(NameParts {
        prefix: stem[..start].to_string(),
        number: run.parse().ok()?,
        digits: run.len(),
        suffix: format!("{}{}", &stem[end..], ext),
    })
}

/// An imported image sequence.
///
/// `frames` is the authority. `pattern` is a human-readable description of the naming and is
/// never used to decide which files exist.
#[derive(Debug, Clone)]
pub struct SequenceAsset {
    pattern: String,
    frames: BTreeMap<u32, PathBuf>,
    width: u32,
    height: u32,
}

impl SequenceAsset {
    /// Printf-style, e.g. `layer3_%03d.png`. Descriptive only.
    pub fn pattern(&self) -> &str {
        &self.pattern
    }

    /// Frame number to file. Ordered by number, gaps preserved as absent keys.
    pub fn frames(&self) -> &BTreeMap<u32, PathBuf> {
        &self.frames
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// Lowest and highest frame number present. `None` only if the sequence is empty, which
    /// [`import_sequence`] never produces — it returns no asset at all in that case.
    pub fn range(&self) -> Option<(u32, u32)> {
        let mut keys = self.frames.keys();
        let first = *keys.next()?;
        Some((first, *self.frames.keys().next_back().unwrap_or(&first)))
    }

    /// Numbers absent between the lowest and highest present number.
    ///
    /// Not "numbers absent from 0", because a sequence that starts at 100 is not missing its
    /// first hundred drawings.
    pub fn missing(&self) -> Vec<u32> {
        match self.range() {
            None => Vec::new(),
            Some((lo, hi)) => (lo..=hi).filter(|n| !self.frames.contains_key(n)).collect(),
        }
    }

    /// File names that the pattern does not generate. The `layer2_桜_013.png` case.
    pub fn name_variants(&self) -> Vec<(u32, &Path)> {
        self.frames
            .iter()
            .filter(|(n, p)| {
                p.file_name().and_then(|s| s.to_str()) != Some(&expand(&self.pattern, **n))
            })
            .map(|(n, p)| (*n, p.as_path()))
            .collect()
    }

    /// Decode one drawing into a tagged sRGB straight-alpha buffer.
    ///
    /// Returns [`DiagnosticId::MediaSequenceGap`] rather than a substitute image when the number
    /// is absent. Document 28 is explicit: "do not substitute adjacent frame."
    pub fn decode(&self, number: u32) -> Result<ImageBuffer, Diagnostic> {
        match self.frames.get(&number) {
            Some(path) => decode_png(path),
            None => Err(Diagnostic::new(
                DiagnosticId::MediaSequenceGap,
                Severity::Warning,
                format!("Drawing {number} is missing from {}.", self.pattern),
                format!(
                    "No file in the sequence carries the number {number}. \
                     Frames exposing it render transparent; no neighbouring drawing is substituted."
                ),
            )),
        }
    }
}

/// Substitute a number into a `%0Nd` pattern.
fn expand(pattern: &str, number: u32) -> String {
    match pattern
        .find("%0")
        .map(|a| (a, pattern[a..].find('d').map(|i| i + a)))
    {
        Some((a, Some(b))) if b > a + 2 => {
            let width: usize = pattern[a + 2..b].parse().unwrap_or(0);
            format!("{}{:0width$}{}", &pattern[..a], number, &pattern[b + 1..])
        }
        _ => pattern.to_string(),
    }
}

/// What an import produced: an asset when anything usable was found, plus everything the user
/// should be told about it.
pub struct ImportResult {
    pub asset: Option<SequenceAsset>,
    pub diagnostics: Vec<Diagnostic>,
}

impl ImportResult {
    pub fn has(&self, id: DiagnosticId) -> bool {
        self.diagnostics.iter().any(|d| d.id == id)
    }

    pub fn get(&self, id: DiagnosticId) -> Option<&Diagnostic> {
        self.diagnostics.iter().find(|d| d.id == id)
    }
}

/// Group a selected set of files into one sequence.
///
/// The selection is the user's, so this never scans a directory on its own: R-01 says "group a
/// selected PNG sequence", and silently pulling in a neighbouring file the user did not pick is
/// the same class of surprise as silently dropping one they did.
pub fn import_sequence(files: &[PathBuf]) -> ImportResult {
    let mut diagnostics = Vec::new();
    let mut frames: BTreeMap<u32, PathBuf> = BTreeMap::new();
    let mut shapes: Vec<(String, usize, String)> = Vec::new();
    let mut unnumbered = Vec::new();
    let mut duplicates = Vec::new();

    let mut sorted: Vec<&PathBuf> = files.iter().collect();
    sorted.sort();

    for path in sorted {
        let name = path.file_name().and_then(|s| s.to_str()).unwrap_or("");
        match split_name(name) {
            None => unnumbered.push(path.clone()),
            Some(p) => {
                // First in sort order wins, so a zero-padded name beats an unpadded one and
                // the result does not depend on the order the file dialog handed them over.
                match frames.entry(p.number) {
                    Entry::Vacant(slot) => {
                        slot.insert(path.clone());
                        // Only a file that entered the map describes the pattern; a rejected
                        // duplicate must not drag the inferred naming toward a name nothing uses.
                        shapes.push((p.prefix, p.digits, p.suffix));
                    }
                    Entry::Occupied(kept) => {
                        duplicates.push((p.number, kept.get().clone(), path.clone()))
                    }
                }
            }
        }
    }

    if !unnumbered.is_empty() {
        let list = join_names(unnumbered.iter().map(|p| p.as_path()));
        diagnostics.push(
            Diagnostic::new(
                DiagnosticId::MediaSequenceUnnumbered,
                Severity::Error,
                pick(
                    unnumbered.len(),
                    "One selected file has no number in its name and was not imported.".to_string(),
                    format!(
                        "{} selected files have no number in their names and were not imported.",
                        unnumbered.len()
                    ),
                ),
                format!("Not imported: {list}"),
            )
            .with_remediation(pick(
                unnumbered.len(),
                "Import it as a still image, or rename it so it carries a drawing number.",
                "Import them as still images, or rename them so each carries a drawing number.",
            )),
        );
    }

    for (number, kept, rejected) in &duplicates {
        diagnostics.push(
            Diagnostic::new(
                DiagnosticId::MediaSequenceDuplicateNumber,
                Severity::Error,
                format!("Two selected files both claim drawing {number}."),
                format!(
                    "{} and {} both end in {number}. Only {} was imported.",
                    display(kept),
                    display(rejected),
                    display(kept)
                ),
            )
            .with_remediation("Rename one file, or select only one of them."),
        );
    }

    // The modal shape is the pattern. Ties break toward the smaller shape by sort order, which is
    // arbitrary but deterministic; a tie means the selection has no majority naming to describe.
    shapes.sort();
    let mut best: Option<(usize, (String, usize, String))> = None;
    for shape in shapes.iter() {
        let count = shapes.iter().filter(|s| *s == shape).count();
        if best.as_ref().is_none_or(|(c, _)| count > *c) {
            best = Some((count, shape.clone()));
        }
    }

    let asset = best.map(|(_, (prefix, digits, suffix))| SequenceAsset {
        pattern: format!("{prefix}%0{digits}d{suffix}"),
        frames,
        width: 0,
        height: 0,
    });

    let Some(mut asset) = asset else {
        return ImportResult {
            asset: None,
            diagnostics,
        };
    };

    // Dimensions come from PNG headers, not full decodes. Header reads are what make a
    // mismatch cheap enough to detect at import time on every file rather than at first render
    // on one of them.
    let mut sizes: BTreeMap<(u32, u32), Vec<u32>> = BTreeMap::new();
    let mut unreadable = Vec::new();
    for (number, path) in &asset.frames {
        match png_size(path) {
            Ok(size) => sizes.entry(size).or_default().push(*number),
            Err(d) => {
                unreadable.push(*number);
                diagnostics.push(d);
            }
        }
    }
    for number in unreadable {
        asset.frames.remove(&number);
    }

    if let Some((&(w, h), _)) = sizes.iter().max_by_key(|(_, ns)| ns.len()) {
        asset.width = w;
        asset.height = h;
    }
    if sizes.len() > 1 {
        let mut detail = String::new();
        for ((w, h), numbers) in &sizes {
            let _ = writeln!(
                detail,
                "  {w}x{h}: {} {}",
                pick(numbers.len(), "drawing", "drawings"),
                ranges(numbers)
            );
        }
        diagnostics.push(
            Diagnostic::new(
                DiagnosticId::MediaSequenceDimensionMismatch,
                Severity::Warning,
                format!(
                    "The drawings in {} are not all the same size. \
                     The sequence is treated as {}x{}.",
                    asset.pattern, asset.width, asset.height
                ),
                format!("Sizes found:\n{}", detail.trim_end()),
            )
            .with_remediation(
                "Re-export the odd drawings at the sequence size. \
                 Until then they are placed at the layer origin and not scaled.",
            ),
        );
    }

    let variants = asset.name_variants();
    if !variants.is_empty() {
        let list = join_names(variants.iter().map(|(_, p)| *p));
        diagnostics.push(Diagnostic::new(
            DiagnosticId::MediaSequenceNameVariant,
            Severity::Info,
            pick(
                variants.len(),
                format!(
                    "One file does not match the pattern {} but carries a clear number and was imported.",
                    asset.pattern
                ),
                format!(
                    "{} files do not match the pattern {} but carry a clear number                      and were imported.",
                    variants.len(),
                    asset.pattern
                ),
            ),
            format!(
                "Imported under {}: {list}",
                pick(variants.len(), "its own name", "their own names")
            ),
        ));
    }

    let missing = asset.missing();
    if !missing.is_empty() {
        let (lo, hi) = asset.range().unwrap();
        diagnostics.push(
            Diagnostic::new(
                DiagnosticId::MediaSequenceGap,
                Severity::Warning,
                format!(
                    "{} missing from {}: {}.",
                    pick(
                        missing.len(),
                        "One drawing is".to_string(),
                        format!("{} drawings are", missing.len())
                    ),
                    asset.pattern,
                    ranges(&missing)
                ),
                format!(
                    "The sequence runs {lo} to {hi} and contains {} files. \
                     Frames exposing a missing drawing render transparent; \
                     no neighbouring drawing is substituted.",
                    asset.frames.len()
                ),
            )
            .with_remediation(
                "Add the missing files to the folder and relink the sequence, \
                 or leave the gap if the hole is intended.",
            ),
        );
    }

    ImportResult {
        asset: Some(asset),
        diagnostics,
    }
}

/// Read width, height and format from a PNG header without decoding pixels.
///
/// The format check happens here, at import, rather than only at first decode. R-01 asks import
/// to "show dimensions, numbering gaps and frame interpretation", and a file this build cannot
/// interpret is something the user should learn while they are still looking at the import
/// dialog, not three hundred frames into a render.
pub fn png_size(path: &Path) -> Result<(u32, u32), Diagnostic> {
    let reader = open_png(path)?;
    let info = reader.info();
    let size = (info.width, info.height);
    check_format(path, info.color_type, info.bit_depth)?;
    Ok(size)
}

/// 8-bit RGBA and RGB only. Anything else is reported as unsupported rather than converted.
///
/// Document 28's rule for an unsupported decoder is to preserve the asset record and report the
/// format; quietly truncating a 16-bit file to 8 bits would be exactly the silent fidelity
/// fallback CLAUDE.md forbids, and the user would never learn their line art lost precision.
fn check_format(
    path: &Path,
    color: png::ColorType,
    depth: png::BitDepth,
) -> Result<(), Diagnostic> {
    if depth == png::BitDepth::Eight && matches!(color, png::ColorType::Rgba | png::ColorType::Rgb)
    {
        return Ok(());
    }
    Err(Diagnostic::new(
        DiagnosticId::MediaUnsupportedFormat,
        Severity::Error,
        format!(
            "{} uses a PNG format this build cannot read.",
            display(path)
        ),
        format!(
            "Found {color:?} at {} bits per channel. Supported: 8-bit RGBA and 8-bit RGB.",
            depth as u8
        ),
    )
    .with_remediation(
        "Re-export as 8-bit RGBA. The file is left untouched and the asset record is kept.",
    ))
}

/// Decode a PNG into a tagged buffer: sRGB encoded, straight alpha, per document 21.
///
/// Rechecks the format even though [`import_sequence`] already did: this is a public entry point
/// and the file on disk may not be the one that was imported.
pub fn decode_png(path: &Path) -> Result<ImageBuffer, Diagnostic> {
    let mut reader = open_png(path)?;
    let (color, depth) = {
        let info = reader.info();
        (info.color_type, info.bit_depth)
    };
    check_format(path, color, depth)?;

    let mut raw = vec![0u8; reader.output_buffer_size().unwrap_or(0)];
    let frame = reader
        .next_frame(&mut raw)
        .map_err(|e| decode_failed(path, &e.to_string()))?;
    let (w, h) = (frame.width as usize, frame.height as usize);

    let rgba = match color {
        png::ColorType::Rgba => raw[..frame.buffer_size()].to_vec(),
        // Opaque by definition: a PNG without an alpha channel has no transparency to lose.
        _ => raw[..frame.buffer_size()]
            .chunks_exact(3)
            .flat_map(|p| [p[0], p[1], p[2], 255])
            .collect(),
    };

    ImageBuffer::from_srgb8_straight(w, h, &rgba).map_err(|e: BufferError| {
        decode_failed(path, &format!("{e} while building a {w}x{h} buffer"))
    })
}

fn open_png(path: &Path) -> Result<png::Reader<BufReader<File>>, Diagnostic> {
    let file = File::open(path).map_err(|e| decode_failed(path, &e.to_string()))?;
    png::Decoder::new(BufReader::new(file))
        .read_info()
        .map_err(|e| decode_failed(path, &e.to_string()))
}

fn decode_failed(path: &Path, detail: &str) -> Diagnostic {
    Diagnostic::new(
        DiagnosticId::MediaDecodeFailed,
        Severity::Error,
        format!("{} could not be read.", display(path)),
        detail.to_string(),
    )
    .with_remediation("Check the file opens in another application, then relink the sequence.")
}

/// Lossy only for a path that is not valid Unicode, which cannot happen for a file whose name
/// this module already parsed as a `&str`. Used for messages, never for opening anything.
fn display(path: &Path) -> String {
    path.file_name()
        .unwrap_or(path.as_os_str())
        .to_string_lossy()
        .into_owned()
}

fn join_names<'a>(paths: impl Iterator<Item = &'a Path>) -> String {
    paths.map(display).collect::<Vec<_>>().join(", ")
}

/// Picks the singular or plural wording. These strings are the deliverable the owner reads, so
/// they are written as English rather than as `drawing(s)`.
fn pick<T>(n: usize, one: T, many: T) -> T {
    if n == 1 {
        one
    } else {
        many
    }
}

/// `[7]` -> `"7"`, `[3,4,5,9]` -> `"3-5, 9"`.
///
/// Document 28 requires frame-level warnings to be rate-limited "retaining counts and ranges",
/// so a sequence missing two hundred drawings produces one readable line rather than two hundred.
fn ranges(numbers: &[u32]) -> String {
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < numbers.len() {
        let start = i;
        while i + 1 < numbers.len() && numbers[i + 1] == numbers[i] + 1 {
            i += 1;
        }
        out.push(if i == start {
            numbers[i].to_string()
        } else {
            format!("{}-{}", numbers[start], numbers[i])
        });
        i += 1;
    }
    out.join(", ")
}
