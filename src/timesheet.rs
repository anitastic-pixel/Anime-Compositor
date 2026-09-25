//! D-84: a cut imported from its XDTS timesheet (B-28b, requirement R-01).
//!
//! The person chooses a cut's folder: one `.xdts` file, and beside it a folder of drawings, or
//! loose drawings named for it, for each of the sheet's cell columns. [`read_cut`] reads that
//! into a composition's worth of layers timed as the sheet says, and [`commands`] turns what it
//! read into one undoable step.
//!
//! The format is CELSYS's public specification, "XDTS file format", 2018/11/29: a first line
//! reading `exchangeDigitalTimeSheet Save Data`, then JSON. A column is written only where
//! something changes, as an animator's pencil writes it, and a drawing holds until the column's
//! next entry. `tools/xdts_reference.py` reads the same folders a second way;
//! `Fixtures/xdts/expected_xdts.json` is what it found, and `tests/b28b_timesheet.rs` holds this
//! reader to it.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde_json::{json, Map, Value as J};

use crate::command::Command;
use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::media;
use crate::model::{Asset, Composition, Id, Layer, Project, SheetText, SheetTextEntry, Timesheet};
use crate::time::{ExposureSpan, FrameRate};

const MAGIC: &str = "exchangeDigitalTimeSheet Save Data";
/// The sheet says no frame rate, so D-84 uses anime's 24.
pub const FRAME_RATE: u32 = 24;
const NULL_CELL: &str = "SYMBOL_NULL_CELL";
const HYPHEN: &str = "SYMBOL_HYPHEN";

/// Something the import says about the cut: document 25's ID and the facts it names, as the
/// reference writes them.
#[derive(Clone, PartialEq, Debug)]
pub struct Note {
    pub id: DiagnosticId,
    pub facts: Map<String, J>,
}

/// One cell column that became a layer.
#[derive(Clone, PartialEq, Debug)]
pub struct Column {
    pub name: String,
    pub spans: Vec<ExposureSpan>,
    /// Drawing number to file, as `media::import_sequence` grouped the column's files.
    pub drawings: BTreeMap<u32, PathBuf>,
    /// The naming the drawings mostly follow, which is also the asset's name.
    pub pattern: String,
    pub timesheet: Timesheet,
}

/// A cut as read: a composition's facts, its columns bottom first, and what was said on the way.
#[derive(Clone, PartialEq, Debug)]
pub struct Cut {
    pub name: String,
    pub duration: u32,
    pub width: u32,
    pub height: u32,
    pub columns: Vec<Column>,
    /// D-84c: the dialogue columns, then the camera columns, each by track.
    pub text: Vec<SheetText>,
    pub notes: Vec<Note>,
    /// What the drawing importer said about the columns' files: a file with no number, two
    /// files claiming one number, a size that differs.
    pub media: Vec<Diagnostic>,
}

/// Nothing is imported: the refusal, and what was said before it.
#[derive(Clone, PartialEq, Debug)]
pub struct Refused {
    pub refusal: Note,
    pub notes: Vec<Note>,
}

fn note(id: DiagnosticId, facts: J) -> Note {
    let J::Object(facts) = facts else {
        unreachable!("facts are written as an object")
    };
    Note { id, facts }
}

/// A number in the file compared as JSON compares it, so that `5` and `5.0` are both 5.
fn is(v: Option<&J>, n: f64) -> bool {
    v.and_then(J::as_f64) == Some(n)
}

/// The value an entry says for its column: the first of its data's `values` with `id` 0.
fn said_in(entry: &J) -> Option<J> {
    entry
        .get("data")?
        .as_array()?
        .iter()
        .find(|d| is(d.get("id"), 0.0))?
        .get("values")?
        .as_array()?
        .first()
        .cloned()
}

/// A file the drawing importer reads: PNG, EXR by D-62, or one of D-72's formats.
fn is_drawing(path: &Path) -> bool {
    path.is_file()
        && (path
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("png"))
            || crate::exr_io::is_exr(&path.to_string_lossy())
            || media::is_other_format(path))
}

fn name_of(path: &Path) -> String {
    path.file_name()
        .map_or_else(String::new, |n| n.to_string_lossy().into_owned())
}

/// A loose drawing named for column `low`: its stem is the name, one of `_ - . space`, and a
/// number, as `A_0001.png` or `a 12.png`.
fn loose_for(path: &Path, low: &str) -> bool {
    let stem = path
        .file_stem()
        .map_or_else(String::new, |s| s.to_string_lossy().to_lowercase());
    stem.strip_prefix(low)
        .and_then(|rest| rest.strip_prefix(['_', '-', '.', ' ']))
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}

/// D-84's reading of a cut folder.
pub fn read_cut(folder: &Path) -> Result<Cut, Refused> {
    let mut notes = Vec::new();
    macro_rules! refuse {
        ($id:expr, $facts:tt) => {
            return Err(Refused {
                refusal: note($id, json!($facts)),
                notes,
            })
        };
    }
    let mut beside: Vec<PathBuf> = fs::read_dir(folder)
        .map(|dir| dir.filter_map(|e| e.ok().map(|e| e.path())).collect())
        .unwrap_or_default();
    beside.sort();
    let sheets: Vec<&PathBuf> = beside
        .iter()
        .filter(|p| {
            p.is_file()
                && p.extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("xdts"))
        })
        .collect();
    let [sheet_path] = sheets.as_slice() else {
        let names: Vec<String> = sheets.iter().map(|p| name_of(p)).collect();
        refuse!(DiagnosticId::TimesheetNotFound, { "sheets": names });
    };
    let sheet_name = name_of(sheet_path);
    let Ok(text) = fs::read_to_string(sheet_path) else {
        refuse!(DiagnosticId::TimesheetUnreadable, { "reason": "it is not UTF-8 text" });
    };
    let text = text.strip_prefix('\u{feff}').unwrap_or(&text);
    let (first, rest) = text.split_once('\n').unwrap_or((text, ""));
    if first.trim_end() != MAGIC {
        let reason = format!("the first line is not {MAGIC}");
        refuse!(DiagnosticId::TimesheetUnreadable, { "reason": reason });
    }
    let Ok(data) = serde_json::from_str::<J>(rest) else {
        refuse!(DiagnosticId::TimesheetUnreadable, { "reason": "what follows the first line is not JSON" });
    };
    let Some(tables) = data
        .get("timeTables")
        .and_then(J::as_array)
        .filter(|t| !t.is_empty())
    else {
        refuse!(DiagnosticId::TimesheetUnreadable, { "reason": "it has no timetable" });
    };
    let table = &tables[0];
    let Some(duration) = table
        .get("duration")
        .and_then(J::as_i64)
        .filter(|d| (1..=i32::MAX as i64).contains(d))
    else {
        refuse!(DiagnosticId::TimesheetUnreadable, { "reason": "its length is not a number of frames" });
    };
    let duration = duration as i32;

    if !is(data.get("version"), 5.0) {
        let version = data.get("version").cloned().unwrap_or(J::Null);
        notes.push(note(
            DiagnosticId::TimesheetVersion,
            json!({ "version": version }),
        ));
    }
    if tables.len() > 1 {
        let others: Vec<J> = tables[1..]
            .iter()
            .map(|t| t.get("name").cloned().unwrap_or(J::from("")))
            .collect();
        notes.push(note(
            DiagnosticId::TimesheetTableNotRead,
            json!({ "tables": others }),
        ));
    }
    let mut tracks: Vec<&J> = Vec::new();
    let (mut dialogue, mut camera): (Vec<&J>, Vec<&J>) = (Vec::new(), Vec::new());
    for field in table
        .get("fields")
        .and_then(J::as_array)
        .into_iter()
        .flatten()
    {
        let columns = field.get("tracks").and_then(J::as_array);
        let id = field.get("fieldId");
        let into = if is(id, 0.0) {
            &mut tracks
        } else if is(id, 3.0) {
            &mut dialogue
        } else if is(id, 5.0) {
            &mut camera
        } else {
            {
                let id = field.get("fieldId").cloned().unwrap_or(J::Null);
                notes.push(note(
                    DiagnosticId::TimesheetFieldNotRead,
                    json!({ "field": id.to_string(), "tracks": columns.map_or(0, Vec::len) }),
                ));
                continue;
            }
        };
        into.extend(columns.into_iter().flatten());
    }
    let names: Vec<J> = table
        .get("timeTableHeaders")
        .and_then(J::as_array)
        .into_iter()
        .flatten()
        .find(|h| is(h.get("fieldId"), 0.0))
        .and_then(|h| h.get("names")?.as_array().cloned())
        .unwrap_or_default();
    if tracks.is_empty() {
        refuse!(DiagnosticId::TimesheetNoCells, {});
    }

    let mut used: Vec<String> = Vec::new();
    let mut columns = Vec::new();
    let mut media_said = Vec::new();
    // The bottom column first: the specification numbers them from 0 at the bottom. A stable
    // sort, so two columns claiming one number stay in the file's order.
    let track_no = |t: &J| t.get("trackNo").and_then(J::as_i64).unwrap_or(0);
    tracks.sort_by_key(|t| track_no(t));
    for t in tracks {
        let no = track_no(t);
        let name = usize::try_from(no)
            .ok()
            .and_then(|i| names.get(i)?.as_str())
            .map_or("", str::trim)
            .to_string();
        if name.is_empty() {
            notes.push(note(
                DiagnosticId::TimesheetColumnUnnamed,
                json!({ "track": no }),
            ));
            continue;
        }

        let mut entries: Vec<(i64, &J)> = t
            .get("frames")
            .and_then(J::as_array)
            .into_iter()
            .flatten()
            .filter_map(|e| Some((e.get("frame")?.as_i64()?, e)))
            .collect();
        entries.sort_by_key(|(f, _)| *f);
        let (mut said, mut outside, mut twice, mut early) =
            (BTreeMap::new(), Vec::new(), Vec::new(), Vec::new());
        for (f, entry) in entries {
            if f < 0 {
                early.push((f, entry));
            } else if f >= duration as i64 {
                outside.push(f);
            } else if said.contains_key(&f) {
                twice.push(f);
            } else {
                said.insert(f, said_in(entry));
            }
        }
        // Clip Studio Paint can write a column's first drawing before frame 0, which the
        // specification does not allow. As OpenToonz reads it (xdtsio.cpp, since 23910493a1),
        // the last entry before frame 0 stands on frame 0 unless frame 0 has its own.
        if !said.contains_key(&0) {
            if let Some((f, entry)) = early.pop() {
                said.insert(0, said_in(entry));
                notes.push(note(
                    DiagnosticId::TimesheetEntryCarriedIn,
                    json!({ "column": name, "frame": f }),
                ));
            }
        }
        let outside: Vec<i64> = early.iter().map(|(f, _)| *f).chain(outside).collect();
        for (frames, reason) in [
            (outside, "outside the sheet"),
            (twice, "a second entry on the same frame"),
        ] {
            if !frames.is_empty() {
                notes.push(note(
                    DiagnosticId::TimesheetEntryIgnored,
                    json!({ "column": name, "frames": frames, "reason": reason }),
                ));
            }
        }

        let mut shown: Vec<Option<u32>> = Vec::with_capacity(duration as usize);
        let mut now = None;
        let mut marks: Vec<(&str, Vec<i64>)> = Vec::new();
        for f in 0..duration as i64 {
            if let Some(v) = said.get(&f) {
                let text = v.as_ref().and_then(J::as_str);
                let number = text
                    .map(str::trim)
                    .filter(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
                    .and_then(|n| n.parse::<u32>().ok());
                let mark = match text {
                    Some("SYMBOL_TICK_1") => Some("inbetween"),
                    Some("SYMBOL_TICK_2") => Some("reverse sheet"),
                    _ => None,
                };
                if number.is_some() {
                    now = number;
                } else if text == Some(NULL_CELL) {
                    now = None;
                } else if text == Some(HYPHEN) {
                } else if let Some(mark) = mark {
                    match marks.iter_mut().find(|(m, _)| *m == mark) {
                        Some((_, frames)) => frames.push(f),
                        None => marks.push((mark, vec![f])),
                    }
                } else {
                    // ponytail: a number too large for a u32 lands here too, where the
                    // reference would take it as a drawing no folder holds.
                    notes.push(note(
                        DiagnosticId::TimesheetCellUnreadable,
                        json!({ "column": name, "frame": f, "value": text.unwrap_or("(none)") }),
                    ));
                    now = None;
                }
            }
            shown.push(now);
        }
        for (mark, frames) in marks {
            notes.push(note(
                DiagnosticId::TimesheetMark,
                json!({ "column": name, "mark": mark, "frames": frames }),
            ));
        }

        let mut spans: Vec<ExposureSpan> = Vec::new();
        for (f, d) in shown.iter().enumerate() {
            let (f, Some(d)) = (f as i32, *d) else {
                continue;
            };
            match spans.last_mut() {
                Some(s) if s.end_frame_exclusive == f && s.drawing_number == d => {
                    s.end_frame_exclusive = f + 1
                }
                _ => spans.push(ExposureSpan {
                    start_frame: f,
                    end_frame_exclusive: f + 1,
                    drawing_number: d,
                }),
            }
        }
        if spans.is_empty() {
            notes.push(note(
                DiagnosticId::TimesheetColumnEmpty,
                json!({ "column": name }),
            ));
            continue;
        }

        // Its drawings: a folder of its name beside the sheet, else loose files named for it.
        let low = name.to_lowercase();
        let files: Vec<PathBuf> = match beside
            .iter()
            .find(|p| p.is_dir() && name_of(p).to_lowercase() == low)
        {
            Some(inside) => {
                used.push(name_of(inside));
                fs::read_dir(inside)
                    .map(|dir| dir.filter_map(|e| e.ok().map(|e| e.path())).collect())
                    .unwrap_or_default()
            }
            None => {
                let loose: Vec<PathBuf> = beside
                    .iter()
                    .filter(|p| is_drawing(p) && loose_for(p, &low))
                    .cloned()
                    .collect();
                used.extend(loose.iter().map(|p| name_of(p)));
                loose
            }
        };
        let files: Vec<PathBuf> = files.into_iter().filter(|p| is_drawing(p)).collect();
        let imported = media::import_sequence(&files);
        media_said.extend(imported.diagnostics);
        let Some(sequence) = imported.asset.filter(|s| !s.frames().is_empty()) else {
            notes.push(note(
                DiagnosticId::TimesheetColumnNoDrawings,
                json!({ "column": name }),
            ));
            continue;
        };
        let called: std::collections::BTreeSet<u32> =
            spans.iter().map(|s| s.drawing_number).collect();
        let missing: Vec<u32> = called
            .iter()
            .filter(|n| !sequence.frames().contains_key(n))
            .copied()
            .collect();
        let unused: Vec<u32> = sequence
            .frames()
            .keys()
            .filter(|n| !called.contains(n))
            .copied()
            .collect();
        if !missing.is_empty() {
            notes.push(note(
                DiagnosticId::TimesheetDrawingMissing,
                json!({ "column": name, "drawings": missing }),
            ));
        }
        if !unused.is_empty() {
            notes.push(note(
                DiagnosticId::TimesheetDrawingUnused,
                json!({ "column": name, "drawings": unused }),
            ));
        }
        columns.push(Column {
            timesheet: Timesheet {
                sheet: sheet_name.clone(),
                column: name.clone(),
                track: no,
            },
            name,
            spans,
            drawings: sequence.frames().clone(),
            pattern: sequence.pattern().to_string(),
        });
    }

    let left: Vec<String> = beside
        .iter()
        .filter(|p| p.is_dir() || is_drawing(p))
        .map(|p| name_of(p))
        .filter(|n| !used.contains(n))
        .collect();
    if !left.is_empty() {
        notes.push(note(
            DiagnosticId::TimesheetNotUsed,
            json!({ "names": left }),
        ));
    }
    if columns.is_empty() {
        refuse!(DiagnosticId::TimesheetNoCells, {});
    }
    let mut text = read_text(table, 3.0, "dialogue", "Dialogue", dialogue, duration, &mut notes);
    text.extend(read_text(table, 5.0, "camera", "Camera", camera, duration, &mut notes));

    // The most common size among every drawing of every column, the first seen on a tie.
    let mut sizes: Vec<((u32, u32), usize)> = Vec::new();
    for path in columns.iter().flat_map(|c| c.drawings.values()) {
        // ponytail: an EXR is read whole a second time here, after the importer's read.
        if let Ok(size) = media::file_size(path, &mut Vec::new()) {
            match sizes.iter_mut().find(|(s, _)| *s == size) {
                Some((_, n)) => *n += 1,
                None => sizes.push((size, 1)),
            }
        }
    }
    let (width, height) = sizes
        .iter()
        .rev()
        .max_by_key(|(_, n)| *n)
        .map_or((0, 0), |(s, _)| *s);
    let name = match table.get("name").and_then(J::as_str).map(str::trim) {
        Some(n) if !n.is_empty() => n.to_string(),
        _ => sheet_path
            .file_stem()
            .map_or_else(String::new, |s| s.to_string_lossy().into_owned()),
    };
    Ok(Cut {
        name,
        duration: duration as u32,
        width,
        height,
        columns,
        text,
        notes,
        media: media_said,
    })
}

/// D-84c: a dialogue or camera field's tracks as text columns. An entry is the strings written
/// on a frame and lasts through the SYMBOL_HYPHEN written on each frame straight after it, as
/// the specification writes a held line.
fn read_text(
    table: &J,
    field: f64,
    kind: &str,
    default: &str,
    mut tracks: Vec<&J>,
    duration: i32,
    notes: &mut Vec<Note>,
) -> Vec<SheetText> {
    let names: Vec<J> = table
        .get("timeTableHeaders")
        .and_then(J::as_array)
        .into_iter()
        .flatten()
        .find(|h| is(h.get("fieldId"), field))
        .and_then(|h| h.get("names")?.as_array().cloned())
        .unwrap_or_default();
    let track_no = |t: &J| t.get("trackNo").and_then(J::as_i64).unwrap_or(0);
    tracks.sort_by_key(|t| track_no(t));
    let mut columns = Vec::new();
    for t in tracks {
        let no = track_no(t);
        let name = usize::try_from(no)
            .ok()
            .and_then(|i| names.get(i)?.as_str())
            .map(str::trim)
            .filter(|n| !n.is_empty())
            .unwrap_or(default)
            .to_string();
        let mut frames: Vec<(i64, &J)> = t
            .get("frames")
            .and_then(J::as_array)
            .into_iter()
            .flatten()
            .filter_map(|e| Some((e.get("frame")?.as_i64()?, e)))
            .collect();
        frames.sort_by_key(|(f, _)| *f);
        let (mut said, mut outside, mut twice) = (BTreeMap::new(), Vec::new(), Vec::new());
        for (f, entry) in frames {
            if !(0..duration as i64).contains(&f) {
                outside.push(f);
            } else if said.contains_key(&f) {
                twice.push(f);
            } else {
                let values = entry
                    .get("data")
                    .and_then(J::as_array)
                    .and_then(|d| d.iter().find(|d| is(d.get("id"), 0.0)))
                    .and_then(|d| d.get("values")?.as_array().cloned());
                said.insert(f, values);
            }
        }
        let (mut entries, mut orphan, mut not_text) = (Vec::<SheetTextEntry>::new(), Vec::new(), Vec::new());
        for (f, values) in said {
            let strings: Option<Vec<String>> = values
                .as_ref()
                .filter(|v| !v.is_empty())
                .and_then(|v| v.iter().map(|s| s.as_str().map(str::to_string)).collect());
            match strings.as_deref() {
                Some([one]) if one == HYPHEN => match entries.last_mut() {
                    Some(e) if e.end_frame_exclusive as i64 == f => e.end_frame_exclusive += 1,
                    _ => orphan.push(f),
                },
                Some([one]) if one == NULL_CELL => {}
                Some(lines) => entries.push(SheetTextEntry {
                    start_frame: f as i32,
                    end_frame_exclusive: f as i32 + 1,
                    text: lines.to_vec(),
                }),
                None => not_text.push(f),
            }
        }
        for (frames, reason) in [
            (outside, "outside the sheet"),
            (twice, "a second entry on the same frame"),
            (orphan, "a continuation with nothing before it"),
            (not_text, "not text"),
        ] {
            if !frames.is_empty() {
                notes.push(note(
                    DiagnosticId::TimesheetEntryIgnored,
                    json!({ "column": name, "frames": frames, "reason": reason }),
                ));
            }
        }
        columns.push(SheetText {
            kind: kind.to_string(),
            name,
            track: no,
            entries,
        });
    }
    columns
}

/// One past the highest `<prefix>N` in use: counted as the window counts its IDs, so the same
/// import twice writes the same file.
fn next_number<'a>(ids: impl Iterator<Item = &'a Id>, prefix: &str) -> u64 {
    ids.filter_map(|id| id.as_str().strip_prefix(prefix)?.parse::<u64>().ok())
        .max()
        .map_or(1, |n| n + 1)
}

/// The commands that add a read cut to `project` as one step: an image sequence asset for each
/// column, then a composition holding a layer per column, bottom first. Paths are stored as
/// `persist::stored_path` stores them for a project in `project_dir`. Returns the new
/// composition's ID with them.
pub fn commands(cut: &Cut, project: &Project, project_dir: &Path) -> (Id, Vec<Command>) {
    let mut asset_no = next_number(project.assets.iter().map(|a| &a.id), "asset-");
    let mut layer_no = next_number(
        project.compositions.iter().flat_map(|c| c.layer_order()),
        "layer-",
    );
    let id = Id::new(format!(
        "comp-{}",
        next_number(project.compositions.iter().map(|c| &c.id), "comp-")
    ));
    let rate = FrameRate::new(FRAME_RATE, 1).expect("24 is a frame rate");
    let mut composition = Composition::new(
        id.clone(),
        cut.name.clone(),
        cut.width,
        cut.height,
        rate,
        0,
        cut.duration,
    );
    composition.sheet_text = cut.text.clone();
    let mut commands = Vec::new();
    for (index, column) in cut.columns.iter().enumerate() {
        let frames = column
            .drawings
            .iter()
            .map(|(n, p)| (*n, crate::persist::stored_path(project_dir, p)))
            .collect();
        let asset = Asset::sequence(
            Id::new(format!("asset-{asset_no}")),
            column.pattern.clone(),
            column.pattern.clone(),
        )
        .with_frames(frames);
        asset_no += 1;
        let mut layer = Layer::new(
            Id::new(format!("layer-{layer_no}")),
            column.name.clone(),
            asset.id.clone(),
            0,
            cut.duration as i32,
        );
        layer_no += 1;
        layer.exposure_spans = column.spans.clone();
        layer.timesheet = Some(column.timesheet.clone());
        composition.insert_layer(layer, index);
        commands.push(Command::AddAsset { asset });
    }
    commands.push(Command::AddComposition {
        composition: Box::new(composition),
    });
    (id, commands)
}

impl Note {
    /// The note as document 28's diagnostic, with its facts said as a sentence.
    pub fn diagnostic(&self) -> Diagnostic {
        let f = &self.facts;
        let text = |k: &str| f.get(k).and_then(J::as_str).unwrap_or("").to_string();
        let list = |k: &str| {
            f.get(k)
                .and_then(J::as_array)
                .map(|a| {
                    a.iter()
                        .map(|v| v.as_str().map_or_else(|| v.to_string(), str::to_string))
                        .collect::<Vec<_>>()
                        .join(", ")
                })
                .unwrap_or_default()
        };
        let number = |k: &str| f.get(k).map_or_else(String::new, J::to_string);
        let column = text("column");
        use DiagnosticId as D;
        let (severity, message, remediation) = match self.id {
            D::TimesheetNotFound => (
                Severity::Error,
                match list("sheets").as_str() {
                    "" => "The chosen folder holds no .xdts timesheet.".to_string(),
                    many => format!("The chosen folder holds more than one timesheet: {many}."),
                },
                Some("Choose a cut's folder holding exactly one .xdts file."),
            ),
            D::TimesheetUnreadable => (
                Severity::Error,
                format!("The timesheet could not be read: {}.", text("reason")),
                Some("Save it again as XDTS from the program that made it."),
            ),
            D::TimesheetNoCells => (
                Severity::Error,
                "No drawing column of the timesheet could become a layer.".to_string(),
                Some("The other notes say why for each column."),
            ),
            D::TimesheetVersion => (
                Severity::Warning,
                format!(
                    "The timesheet says version {}, not 5; it was read anyway.",
                    number("version")
                ),
                None,
            ),
            D::TimesheetTableNotRead => (
                Severity::Info,
                format!("Only the first timetable was read, not: {}.", list("tables")),
                None,
            ),
            D::TimesheetFieldNotRead => (
                Severity::Info,
                format!(
                    "The {} field ({} columns) was not read; only drawing, dialogue and camera columns are.",
                    text("field"),
                    number("tracks")
                ),
                None,
            ),
            D::TimesheetColumnUnnamed => (
                Severity::Warning,
                format!(
                    "Column {} has no name, so its drawings cannot be found; it made no layer.",
                    number("track")
                ),
                Some("Name the column in the program that made the sheet."),
            ),
            D::TimesheetEntryIgnored => (
                Severity::Warning,
                format!(
                    "Column {column}: the entries on frames {} were left out ({}).",
                    list("frames"),
                    text("reason")
                ),
                None,
            ),
            D::TimesheetEntryCarriedIn => (
                Severity::Info,
                format!(
                    "Column {column}: the entry written on frame {} is shown from frame 0.",
                    number("frame")
                ),
                None,
            ),
            D::TimesheetCellUnreadable => (
                Severity::Warning,
                format!(
                    "Column {column}, frame {}: the cell \"{}\" is not a drawing number, so the column is blank until its next entry.",
                    number("frame"),
                    text("value")
                ),
                None,
            ),
            D::TimesheetMark => (
                Severity::Info,
                format!(
                    "Column {column}: {} marks on frames {}; they change nothing shown.",
                    text("mark"),
                    list("frames")
                ),
                None,
            ),
            D::TimesheetColumnEmpty => (
                Severity::Info,
                format!("Column {column} never shows a drawing, so it made no layer."),
                None,
            ),
            D::TimesheetColumnNoDrawings => (
                Severity::Warning,
                format!("Column {column} has no folder and no loose drawings, so it made no layer."),
                Some("Put its drawings in a folder named for the column, beside the sheet."),
            ),
            D::TimesheetDrawingMissing => (
                Severity::Warning,
                format!(
                    "Column {column}: the sheet calls for drawings {} that its folder does not have.",
                    list("drawings")
                ),
                Some("The timing is kept; add the drawings and relink."),
            ),
            D::TimesheetDrawingUnused => (
                Severity::Info,
                format!(
                    "Column {column}: drawings {} are never shown by the sheet.",
                    list("drawings")
                ),
                None,
            ),
            D::TimesheetNotUsed => (
                Severity::Info,
                format!("Not used by any column: {}.", list("names")),
                None,
            ),
            other => unreachable!("{other} is not a timesheet note"),
        };
        let facts = J::Object(f.clone()).to_string();
        let d = Diagnostic::new(self.id, severity, message, facts);
        match remediation {
            Some(r) => d.with_remediation(r),
            None => d,
        }
    }
}
