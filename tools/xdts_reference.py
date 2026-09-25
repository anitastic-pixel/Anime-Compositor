"""XDTS timesheets read into a composition, worked a second way.

D-84 proposes "Import cut": the person chooses a cut's folder, which holds one XDTS timesheet
and the drawings for its columns, and gets a composition with one layer per cell column, timed
as the sheet says. This file is the reference for what document 25 pins against that.

**Nothing here was written from anyone's reader.** The format is CELSYS's public specification,
"XDTS file format", dated 2018/11/29: a first line reading `exchangeDigitalTimeSheet Save Data`
and JSON after it. How that JSON becomes layers is D-84's reading, which this file implements
on its own, in `read_cut`, so that B-28b's Rust has something independent to agree with. One
rule came from a reader afterwards, by the owner's choice on 2026-09-24: an entry before frame
0 is carried in, as OpenToonz carries in what Clip Studio Paint writes there (FX-XDTS-028).

No real timesheet was available, so every sheet here is synthetic, written by `sheet()` from the
specification's own structure. Every drawing is 160 by 90 and blank but for one 16-pixel square
in the column's colour: its row says which column it is (the bottom column on the top row) and
how far it sits to the right says its drawing number, so a cut played in the window shows its
timing as a square that steps, holds and vanishes.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

    python tools/xdts_reference.py
"""

import json
import re
import shutil
import struct
import zlib
from collections import Counter
from pathlib import Path

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "xdts"
MAGIC = "exchangeDigitalTimeSheet Save Data"
FRAME_RATE = 24  # the sheet does not say one; D-84 uses anime's 24
SIZE = (160, 90)
NULL, HYPHEN = "SYMBOL_NULL_CELL", "SYMBOL_HYPHEN"
TICKS = {"SYMBOL_TICK_1": "inbetween", "SYMBOL_TICK_2": "reverse sheet"}
FIELDS = {0: "cells", 3: "dialogue", 5: "camerawork"}
# D-84c: the two text fields, as the kind of column each becomes and its name when the
# sheet's headers give none.
TEXT = {3: ("dialogue", "Dialogue"), 5: ("camera", "Camera")}
# What media.import reads: PNG, EXR (D-62) and D-72's formats.
DRAWING = {".png", ".exr", ".jpg", ".jpeg", ".tif", ".tiff", ".tga", ".bmp", ".webp"}
COLOURS = [(220, 40, 40), (40, 170, 60), (50, 90, 230), (230, 190, 30), (150, 60, 200)]


# --- Writing the inputs -------------------------------------------------------------------

def png(number, row, colour):
    """Drawing `number` of the column on `row`: a square stepped 12 pixels right per number."""
    w, h = SIZE
    x0, y0 = 6 + 12 * ((number - 1) % 12), 6 + 20 * (row % 4)
    rows = []
    for y in range(h):
        line = bytearray(b"\0")
        for x in range(w):
            inside = x0 <= x < x0 + 16 and y0 <= y < y0 + 16
            line += bytes((*colour, 255) if inside else (0, 0, 0, 0))
        rows.append(bytes(line))

    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data) & 0xFFFFFFFF))

    return (b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", struct.pack(">IIBBBBB", w, h, 8, 6, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress(b"".join(rows), 9)) + chunk(b"IEND", b""))


def track(no, entries):
    """One column's frames as the specification writes them: only where something is said."""
    return {"trackNo": no, "frames": [{"frame": f, "data": [{"id": 0, "values": list(v)}]}
                                      for f, v in entries]}


def cells(entries):
    """`{0: "1", 2: "2"}` read as frame and cell, or a list of pairs where a frame repeats."""
    return [(f, [v]) for f, v in (entries.items() if isinstance(entries, dict) else entries)]


def sheet(name, duration, columns, extra_fields=(), version=5, extra_headers=()):
    """`columns` is (trackNo, name, entries) in file order; a name of None is left out."""
    names = {}
    for no, col, _ in columns:
        if col is not None:
            names[no] = col
    header_names = [names.get(i, "") for i in range(max(names, default=-1) + 1)]
    table = {
        "fields": [{"fieldId": 0, "tracks": [track(no, cells(e)) for no, _, e in columns]}]
        + [{"fieldId": fid, "tracks": tracks} for fid, tracks in extra_fields],
        "duration": duration,
        "name": name,
        "timeTableHeaders": [{"fieldId": 0, "names": header_names}]
        + [{"fieldId": fid, "names": list(n)} for fid, n in extra_headers],
    }
    return {"header": {"cut": "1", "scene": "1"},
            "timeTables": [table], "version": version}


def text(data):
    return MAGIC + "\n" + json.dumps(data, indent=1, ensure_ascii=False) + "\n"


# --- Reading, by D-84 ---------------------------------------------------------------------

def number_in(name):
    """media.import's rule: the last run of digits in the name before its extension."""
    runs = re.findall(r"\d+", Path(name).stem)
    return int(runs[-1]) if runs else None


def is_drawing(p):
    return p.is_file() and p.suffix.lower() in DRAWING


def png_size(p):
    return struct.unpack(">II", p.read_bytes()[16:24])


def read_cut(folder):
    """D-84's reading of a cut folder: a composition and its layers, or a refusal, and notes."""
    notes = []

    def refuse(code, **facts):
        return {"refused": code, **facts, "notes": notes}

    sheets = sorted(p.name for p in folder.iterdir() if p.is_file() and p.suffix.lower() == ".xdts")
    if len(sheets) != 1:
        return refuse("TIMESHEET_NOT_FOUND", sheets=sheets)
    sheet_path = folder / sheets[0]
    first, _, rest = sheet_path.read_text(encoding="utf-8-sig").partition("\n")
    if first.rstrip() != MAGIC:
        return refuse("TIMESHEET_UNREADABLE", reason="the first line is not " + MAGIC)
    try:
        data = json.loads(rest)
    except ValueError:
        return refuse("TIMESHEET_UNREADABLE", reason="what follows the first line is not JSON")
    tables = data.get("timeTables") if isinstance(data, dict) else None
    if not isinstance(tables, list) or not tables:
        return refuse("TIMESHEET_UNREADABLE", reason="it has no timetable")
    table = tables[0]
    duration = table.get("duration")
    if not isinstance(duration, int) or isinstance(duration, bool) or duration < 1:
        return refuse("TIMESHEET_UNREADABLE", reason="its length is not a number of frames")

    if data.get("version") != 5:
        notes.append({"id": "TIMESHEET_VERSION", "version": data.get("version")})
    if len(tables) > 1:
        notes.append({"id": "TIMESHEET_TABLE_NOT_READ",
                      "tables": [t.get("name", "") for t in tables[1:]]})
    tracks, text_fields = [], []
    for field in table.get("fields", []):
        if field.get("fieldId") == 0:
            tracks += field.get("tracks", [])
        elif field.get("fieldId") in TEXT:
            text_fields.append(field)
        else:
            notes.append({"id": "TIMESHEET_FIELD_NOT_READ",
                          "field": FIELDS.get(field.get("fieldId"), str(field.get("fieldId"))),
                          "tracks": len(field.get("tracks", []))})
    names = next((h.get("names", []) for h in table.get("timeTableHeaders", [])
                  if h.get("fieldId") == 0), [])
    if not tracks:
        return refuse("TIMESHEET_NO_CELLS")

    things = {p.name for p in folder.iterdir() if p.is_dir() or is_drawing(p)}
    used = set()
    layers = []
    # The bottom column first: the specification numbers them from 0 at the bottom. A stable
    # sort, so two columns claiming one number stay in the file's order.
    for t in sorted(tracks, key=lambda t: t.get("trackNo", 0)):
        no = t.get("trackNo", 0)
        name = names[no].strip() if 0 <= no < len(names) and isinstance(names[no], str) else ""
        if not name:
            notes.append({"id": "TIMESHEET_COLUMN_UNNAMED", "track": no})
            continue

        said, outside, twice, early = {}, [], [], []
        entries = [e for e in t.get("frames", []) if type(e.get("frame")) is int]
        for entry in sorted(entries, key=lambda e: e["frame"]):
            f = entry["frame"]
            if f < 0:
                early.append(entry)
                continue
            if f >= duration:
                outside.append(f)
                continue
            if f in said:
                twice.append(f)
                continue
            values = next((d.get("values") for d in entry.get("data", []) if d.get("id") == 0),
                          None)
            said[f] = values[0] if values else None
        # Clip Studio Paint can write a column's first drawing before frame 0, which the
        # specification does not allow. As OpenToonz reads it (xdtsio.cpp, since 23910493a1),
        # the last entry before frame 0 stands on frame 0 unless frame 0 has its own.
        if early and 0 not in said:
            carried = early.pop()
            values = next((d.get("values") for d in carried.get("data", []) if d.get("id") == 0),
                          None)
            said[0] = values[0] if values else None
            notes.append({"id": "TIMESHEET_ENTRY_CARRIED_IN", "column": name,
                          "frame": carried["frame"]})
        outside = [e["frame"] for e in early] + outside
        if outside:
            notes.append({"id": "TIMESHEET_ENTRY_IGNORED", "column": name, "frames": outside,
                          "reason": "outside the sheet"})
        if twice:
            notes.append({"id": "TIMESHEET_ENTRY_IGNORED", "column": name, "frames": twice,
                          "reason": "a second entry on the same frame"})

        shown, now, marks = [], None, {}
        for f in range(duration):
            if f in said:
                v = said[f]
                if isinstance(v, str) and v.strip().isascii() and v.strip().isdigit():
                    now = int(v.strip())
                elif v == NULL:
                    now = None
                elif v == HYPHEN:
                    pass
                elif v in TICKS:
                    marks.setdefault(TICKS[v], []).append(f)
                else:
                    notes.append({"id": "TIMESHEET_CELL_UNREADABLE", "column": name, "frame": f,
                                  "value": v if isinstance(v, str) else "(none)"})
                    now = None
            shown.append(now)
        for mark, frames in marks.items():
            notes.append({"id": "TIMESHEET_MARK", "column": name, "mark": mark,
                          "frames": frames})

        spans = []
        for f, d in enumerate(shown):
            if d is None:
                continue
            if spans and spans[-1][1] == f and spans[-1][2] == d:
                spans[-1][1] = f + 1
            else:
                spans.append([f, f + 1, d])
        if not spans:
            notes.append({"id": "TIMESHEET_COLUMN_EMPTY", "column": name})
            continue

        # Its drawings: a folder of its name beside the sheet, else loose files named for it.
        low = name.lower()
        inside = [p for p in folder.iterdir() if p.is_dir() and p.name.lower() == low]
        loose = [p for p in folder.iterdir() if is_drawing(p)
                 and re.fullmatch(re.escape(low) + r"[_\-. ]\d+", p.stem.lower())]
        if inside:
            files = [p for p in inside[0].iterdir() if is_drawing(p)]
            used.add(inside[0].name)
        else:
            files = loose
            used.update(p.name for p in loose)
        drawings = {}
        for p in sorted(files):
            n = number_in(p.name)
            if n is not None and n not in drawings:
                drawings[n] = p.relative_to(folder).as_posix()
        if not drawings:
            notes.append({"id": "TIMESHEET_COLUMN_NO_DRAWINGS", "column": name})
            continue
        called = {d for _, _, d in spans}
        missing = sorted(called - drawings.keys())
        unused = sorted(drawings.keys() - called)
        if missing:
            notes.append({"id": "TIMESHEET_DRAWING_MISSING", "column": name, "drawings": missing})
        if unused:
            notes.append({"id": "TIMESHEET_DRAWING_UNUSED", "column": name, "drawings": unused})
        layers.append({"name": name, "track": no, "spans": spans,
                       "drawings": {str(n): p for n, p in sorted(drawings.items())},
                       "timesheet": {"sheet": sheet_path.name, "column": name, "track": no}})

    left = sorted(things - used)
    if left:
        notes.append({"id": "TIMESHEET_NOT_USED", "names": left})
    if not layers:
        return refuse("TIMESHEET_NO_CELLS")
    text = []
    for fid in sorted(TEXT):
        for field in (f for f in text_fields if f.get("fieldId") == fid):
            text += read_text(fid, field, table, duration, notes)
    sizes = Counter(png_size(folder / p) for l in layers for p in l["drawings"].values())
    width, height = sizes.most_common(1)[0][0]
    return {"composition": {"name": table.get("name", "").strip() or sheet_path.stem,
                            "duration": duration, "frame_rate": FRAME_RATE,
                            "width": width, "height": height},
            "layers": layers, "notes": notes, **({"text": text} if text else {})}


def read_text(fid, field, table, duration, notes):
    """D-84c: a dialogue or camera field as text columns. An entry lasts from its frame
    through the SYMBOL_HYPHEN written on each frame straight after it."""
    kind, default = TEXT[fid]
    names = next((h.get("names", []) for h in table.get("timeTableHeaders", [])
                  if h.get("fieldId") == fid), [])
    columns = []
    for t in sorted(field.get("tracks", []), key=lambda t: t.get("trackNo", 0)):
        no = t.get("trackNo", 0)
        name = names[no].strip() if 0 <= no < len(names) and isinstance(names[no], str) else ""
        name = name or default
        said, outside, twice = {}, [], []
        entries = [e for e in t.get("frames", []) if type(e.get("frame")) is int]
        for entry in sorted(entries, key=lambda e: e["frame"]):
            f = entry["frame"]
            if not 0 <= f < duration:
                outside.append(f)
                continue
            if f in said:
                twice.append(f)
                continue
            said[f] = next((d.get("values") for d in entry.get("data", []) if d.get("id") == 0),
                           None)
        runs, orphan, not_text = [], [], []
        for f in sorted(said):
            v = said[f]
            if v == [HYPHEN]:
                if runs and runs[-1][1] == f:
                    runs[-1][1] = f + 1
                else:
                    orphan.append(f)
            elif v == [NULL]:
                pass
            elif isinstance(v, list) and v and all(isinstance(s, str) for s in v):
                runs.append([f, f + 1, v])
            else:
                not_text.append(f)
        for frames, reason in ((outside, "outside the sheet"),
                               (twice, "a second entry on the same frame"),
                               (orphan, "a continuation with nothing before it"),
                               (not_text, "not text")):
            if frames:
                notes.append({"id": "TIMESHEET_ENTRY_IGNORED", "column": name,
                              "frames": frames, "reason": reason})
        columns.append({"kind": kind, "name": name, "track": no, "entries": runs})
    return columns


# --- The cases ----------------------------------------------------------------------------

def on(first, step, count, start=1):
    """Drawings start, start+1, ... each written once and held for `step` frames."""
    return {first + i * step: str(start + i) for i in range(count)}


def folder_of(col, numbers):
    return [f"{col}/{col}_{n:04d}.png" for n in numbers]


ONES = on(0, 1, 6)
TWOS = on(0, 2, 4)

CASES = {
    "FX-XDTS-001": ("One column on ones: a new drawing every frame.",
                    sheet("c001", 6, [(0, "A", ONES)]), folder_of("A", range(1, 7))),
    "FX-XDTS-002": ("On twos: each drawing written once and held for two frames. The sheet "
                    "says nothing on the frames between, and a drawing holds until the next "
                    "entry.", sheet("c002", 8, [(0, "A", TWOS)]), folder_of("A", range(1, 5))),
    "FX-XDTS-003": ("FX-XDTS-002 with SYMBOL_HYPHEN written on every held frame, as the "
                    "specification writes a held line of dialogue. The same timing.",
                    sheet("c003", 8, [(0, "A", {**TWOS, **{f: HYPHEN for f in (1, 3, 5, 7)}})]),
                    folder_of("A", range(1, 5))),
    "FX-XDTS-004": ("A long hold: the last drawing written holds to the end of the sheet.",
                    sheet("c004", 12, [(0, "A", {0: "1", 3: "2"})]), folder_of("A", (1, 2))),
    "FX-XDTS-005": ("Blank cells (SYMBOL_NULL_CELL, the X on a paper sheet): the column shows "
                    "nothing from there until the next drawing.",
                    sheet("c005", 10, [(0, "A", {0: "1", 3: NULL, 5: "2", 7: NULL})]),
                    folder_of("A", (1, 2))),
    "FX-XDTS-006": ("A column that starts late: nothing is shown before its first entry.",
                    sheet("c006", 8, [(0, "A", {4: "1"})]), folder_of("A", (1,))),
    "FX-XDTS-007": ("Drawings reused out of order, as a cycle goes 1, 2, 3, 2, 1.",
                    sheet("c007", 10, [(0, "A", {0: "1", 2: "2", 4: "3", 6: "2", 8: "1"})]),
                    folder_of("A", (1, 2, 3))),
    "FX-XDTS-008": ("The same drawing written twice in a row is one exposure, not two.",
                    sheet("c008", 6, [(0, "A", {0: "1", 2: "1", 4: "2"})]),
                    folder_of("A", (1, 2))),
    "FX-XDTS-009": ("Three columns, written in the file in the order 2, 0, 1: they stack by "
                    "track number, 0 at the bottom, whatever the file order. A is on threes, "
                    "B on twos, C on ones.",
                    sheet("c009", 6, [(2, "C", on(0, 1, 6)), (0, "A", on(0, 3, 2)),
                                      (1, "B", on(0, 2, 3))]),
                    folder_of("A", (1, 2)) + folder_of("B", (1, 2, 3))
                    + folder_of("C", range(1, 7))),
    "FX-XDTS-010": ("No folders: loose files beside the sheet named for their column, with "
                    "each of the four separators D-84 accepts.",
                    sheet("c010", 2, [(0, "A", {0: "1", 1: "2"}), (1, "B", {0: "1"}),
                                      (2, "C", {0: "1"}), (3, "D", {0: "1"})]),
                    ["A_0001.png", "A_0002.png", "B-0001.png", "C.0001.png", "D 0001.png"]),
    "FX-XDTS-011": ("Both a folder and loose files for column A: the folder is used, and the "
                    "loose files are named as not used.",
                    sheet("c011", 2, [(0, "A", {0: "1", 1: "2"})]),
                    folder_of("A", (1, 2)) + ["A_0001.png", "A_0002.png"]),
    "FX-XDTS-012": ("A column named in lower case finds a folder named in upper case.",
                    sheet("c012", 2, [(0, "a", {0: "1"})]), folder_of("A", (1,))),
    "FX-XDTS-013": ("The two tick marks change nothing that is shown: the drawing before each "
                    "holds through it, and each is reported as a mark on its frames.",
                    sheet("c013", 4, [(0, "A", {0: "1", 1: "SYMBOL_TICK_1", 2: "2",
                                                3: "SYMBOL_TICK_2"})]),
                    folder_of("A", (1, 2))),
    "FX-XDTS-014": ("A sheet with a dialogue column and a camerawork column: all three are read, "
                    "the two text columns named Dialogue and Camera, since the headers name "
                    "only the cells.",
                    sheet("c014", 4, [(0, "A", {0: "1"})], extra_fields=[
                        (3, [track(0, [(0, ["MIKA", "Wait!"]), (1, [HYPHEN]), (2, [HYPHEN])])]),
                        (5, [track(0, [(0, ["PAN"]), (1, [HYPHEN]), (2, [HYPHEN]),
                                       (3, [HYPHEN])])])]),
                    folder_of("A", (1,))),
    "FX-XDTS-015": ("Two timetables in one file: the first is read, the second is named as not "
                    "read.",
                    {**sheet("c015", 2, [(0, "A", {0: "1"})]),
                     "timeTables": [sheet("c015", 2, [(0, "A", {0: "1"})])["timeTables"][0],
                                    sheet("c015 retake", 2, [(0, "A", {0: "2"})])[
                                        "timeTables"][0]]},
                    folder_of("A", (1, 2))),
    "FX-XDTS-016": ("A version other than 5 is read anyway, and reported.",
                    sheet("c016", 2, [(0, "A", {0: "1"})], version=4), folder_of("A", (1,))),
    "FX-XDTS-017": ("A sheet saved with a byte-order mark and Windows line endings reads as "
                    "any other.", sheet("c017", 2, [(0, "A", {0: "1"})]), folder_of("A", (1,))),
    "FX-XDTS-018": ("A timetable with no name: the composition is named after the sheet's file.",
                    sheet("", 2, [(0, "A", {0: "1"})]), folder_of("A", (1,))),
    "FX-XDTS-019": ("Text columns as a sheet can write them: a line held with hyphens, a line on "
                    "one frame, a cross after it, a hyphen with nothing before it, a number where "
                    "text belongs, an entry past the end, and a field this program does not "
                    "know.",
                    sheet("c019", 8, [(0, "A", {0: "1"})], extra_fields=[
                        (3, [track(0, [(0, ["MIKA", "Wait!"]), (1, [HYPHEN]), (2, [HYPHEN]),
                                       (4, ["KAI", "No."]), (5, [NULL]), (6, [HYPHEN]),
                                       (7, [5]), (9, ["Late"])])]),
                        (7, [track(0, [(0, ["?"])])])],
                          extra_headers=[(3, ["S1"])]),
                    folder_of("A", (1,))),
    "FX-XDTS-020": ("The sheet calls for drawing 2, which is not in A's folder. The timing is "
                    "kept, so those frames will say MEDIA_SEQUENCE_GAP when drawn, and the "
                    "import names the drawing.",
                    sheet("c020", 6, [(0, "A", on(0, 2, 3))]), folder_of("A", (1, 3))),
    "FX-XDTS-021": ("A's folder holds drawings 2 and 4, which the sheet never shows.",
                    sheet("c021", 4, [(0, "A", {0: "1", 2: "3"})]),
                    folder_of("A", (1, 2, 3, 4))),
    "FX-XDTS-022": ("Column B has no folder and no loose files: A becomes a layer and B does "
                    "not, and the import says so.",
                    sheet("c022", 2, [(0, "A", {0: "1"}), (1, "B", {0: "1"})]),
                    folder_of("A", (1,))),
    "FX-XDTS-023": ("A background folder and a background still beside the sheet match no "
                    "column, and are named as not used. A text file is not a drawing and is "
                    "not mentioned.",
                    sheet("c023", 2, [(0, "A", {0: "1"})]),
                    folder_of("A", (1,)) + ["BG/BG_0001.png", "BG.png", "notes.txt"]),
    "FX-XDTS-024": ("A cell that is not a whole number (`3a`): the column is blank from there "
                    "to its next entry, and the value and frame are reported.",
                    sheet("c024", 6, [(0, "A", {0: "1", 2: "3a", 4: "2"})]),
                    folder_of("A", (1, 2, 3))),
    "FX-XDTS-025": ("Entries the sheet cannot use: a second entry on frame 2, and one on frame "
                    "6 of a 4-frame sheet. Both are left out and reported.",
                    sheet("c025", 4, [(0, "A", [(0, "1"), (2, "2"), (2, "3"), (6, "4")])]),
                    folder_of("A", (1, 2, 3, 4))),
    "FX-XDTS-026": ("Column B holds only blank cells: it makes no layer, and its folder is "
                    "named as not used.",
                    sheet("c026", 2, [(0, "A", {0: "1"}), (1, "B", {0: NULL})]),
                    folder_of("A", (1,)) + folder_of("B", (1,))),
    "FX-XDTS-027": ("Track 1 has no name in the sheet, so no drawings can be found for it and "
                    "it makes no layer.",
                    sheet("c027", 2, [(0, "A", {0: "1"}), (1, None, {0: "1"})]),
                    folder_of("A", (1,))),
    "FX-XDTS-028": ("Clip Studio Paint can write a column's first drawing before frame 0, which "
                    "the specification does not allow. As OpenToonz reads it, the last entry "
                    "before frame 0 is shown from frame 0 until the column's next entry, and "
                    "is reported; A's earlier one is left out. B has its own entry on frame 0, "
                    "so its entry before it is left out.",
                    sheet("c028", 4, [(0, "A", {-2: "2", -1: "1", 2: "2"}),
                                      (1, "B", {-1: "1", 0: "2"})]),
                    folder_of("A", (1, 2)) + folder_of("B", (2,))),
}

# Nothing is imported from these. Each is (says, sheets, files), `sheets` being (name, text).
REFUSED = {
    "FX-XDTS-030": ("The folder holds no timesheet.", [], folder_of("A", (1,))),
    "FX-XDTS-031": ("The folder holds two timesheets, and which one is meant is the person's "
                    "to say.",
                    [("fx_xdts_031a.xdts", text(sheet("a", 2, [(0, "A", {0: "1"})]))),
                     ("fx_xdts_031b.xdts", text(sheet("b", 2, [(0, "A", {0: "1"})])))],
                    folder_of("A", (1,))),
    "FX-XDTS-032": ("The first line is not the XDTS line: this is bare JSON.",
                    [("fx_xdts_032.xdts", json.dumps(sheet("c032", 2, [(0, "A", {0: "1"})])))],
                    folder_of("A", (1,))),
    "FX-XDTS-033": ("The first line is right and what follows is not JSON.",
                    [("fx_xdts_033.xdts", MAGIC + "\n{\"timeTables\": [\n")],
                    folder_of("A", (1,))),
    "FX-XDTS-034": ("The sheet has a dialogue column and no cell column.",
                    [("fx_xdts_034.xdts", text({**sheet("c034", 2, []), "timeTables": [{
                        "fields": [{"fieldId": 3, "tracks": [track(0, [(0, ["MIKA", "Hi"])])]}],
                        "duration": 2, "name": "c034",
                        "timeTableHeaders": [{"fieldId": 3, "names": ["S1"]}]}]}))],
                    folder_of("A", (1,))),
    "FX-XDTS-035": ("The sheet's length is 0 frames.",
                    [("fx_xdts_035.xdts", text(sheet("c035", 0, [(0, "A", {0: "1"})])))],
                    folder_of("A", (1,))),
    "FX-XDTS-036": ("No column has any drawings beside the sheet, so no layer can be made.",
                    [("fx_xdts_036.xdts", text(sheet("c036", 2, [(0, "A", {0: "1"})])))],
                    ["notes.txt"]),
}

# FX-XDTS-040, the sample cut: two seconds of a made-up cut, to open, play and learn from. A
# is a body on twos that holds for a beat, B a mouth that opens and closes on ones and threes
# and is blank while the body turns, C an effect that comes in late. It has a line of
# dialogue and a camera instruction, which D-84c reads, and a background on no column.
SAMPLE = sheet(
    "s01 c012", 48,
    [(0, "A", {0: "1", 2: "2", 4: "3", 6: "4", 8: "5", 10: "6", 12: "7", 14: "8",
               16: "8", 24: "7", 26: "6", 28: "5", 30: "4", 32: "3", 34: "2", 36: "1"}),
     (1, "B", {0: "1", 3: "2", 6: "3", 7: "2", 8: "1", 11: "2", 14: "3", 15: "2",
               16: NULL, 30: "1", 33: "2", 36: "3", 39: "1"}),
     (2, "C", {20: "1", 21: "2", 22: "3", 23: "4", 24: "5", 26: "6", 28: NULL,
               40: "1", 41: "SYMBOL_TICK_1", 42: "2"})],
    extra_fields=[(3, [track(0, [(0, ["MIKA", "Over here!"])]
                             + [(f, [HYPHEN]) for f in range(1, 16)])]),
                  (5, [track(0, [(20, ["FOLLOW"])] + [(f, [HYPHEN]) for f in range(21, 48)])])])
SAMPLE_FILES = (folder_of("A", range(1, 9)) + folder_of("B", (1, 2, 3))
                + folder_of("C", range(1, 7)) + ["BG.png"])


def write(folder, sheets, files, columns):
    shutil.rmtree(folder, ignore_errors=True)
    folder.mkdir(parents=True)
    for name, body in sheets:
        (folder / name).write_bytes(body.encode("utf-8") if isinstance(body, str) else body)
    for rel in files:
        p = folder / rel
        p.parent.mkdir(parents=True, exist_ok=True)
        if p.suffix == ".txt":
            p.write_text("Not a drawing.\n", encoding="utf-8")
            continue
        col = rel.split("/")[0] if "/" in rel else re.split(r"[_\-. ]", rel)[0]
        row = columns.index(col.upper()) if col.upper() in columns else 4
        p.write_bytes(png(number_in(rel) or 1, row, COLOURS[row]))


def columns_of(data):
    """Column names bottom to top, upper-cased, for the colours and rows of the drawings."""
    names = data["timeTables"][0]["timeTableHeaders"][0]["names"]
    return [n.upper() for n in names]


def sheet_line(spans, duration):
    shown = ["x"] * duration
    for s, e, d in spans:
        shown[s:e] = [str(d)] * (e - s)
    return " ".join(shown)


def written(entries):
    return ", ".join(f"{f}: {v}" for f, v in entries.items())


def main():
    shutil.rmtree(OUT, ignore_errors=True)
    OUT.mkdir(parents=True)
    expected = {"frame_rate": FRAME_RATE, "cases": {}, "refused": {}}

    all_cases = dict(CASES)
    all_cases["FX-XDTS-040"] = ("The sample cut: two seconds, three columns, a line of "
                                "dialogue, a camera instruction and a background.", SAMPLE,
                                SAMPLE_FILES)
    for fx, (says, data, files) in all_cases.items():
        name = fx.lower().replace("-", "_")
        body = text(data)
        if fx == "FX-XDTS-017":
            body = b"\xef\xbb\xbf" + body.replace("\n", "\r\n").encode("utf-8")
        write(OUT / name, [(name + ".xdts", body)], files, columns_of(data))
        got = read_cut(OUT / name)
        assert "refused" not in got, (fx, got)
        expected["cases"][fx] = {"says": says, "folder": name, **got}

    for fx, (says, sheets, files) in REFUSED.items():
        name = fx.lower().replace("-", "_")
        write(OUT / name, sheets, files, ["A"])
        got = read_cut(OUT / name)
        assert "refused" in got, (fx, got)
        expected["refused"][fx] = {"says": says, "folder": name, **got}

    (OUT / "expected_xdts.json").write_text(json.dumps(expected, indent=1) + "\n",
                                            encoding="utf-8")

    # What document 25 prints: each case as the frames it shows, a column to a line.
    for fx, case in expected["cases"].items():
        if fx == "FX-XDTS-040":
            continue
        print(f"- {fx}: {case['says']}")
        for layer in reversed(case["layers"]):
            print(f"  - {layer['name']}: `{sheet_line(layer['spans'], case['composition']['duration'])}`")
        for column in case.get("text", []):
            print(f"  - {column['kind']} column {column['name']}: "
                  + "; ".join(f"frames {s} to {e - 1} `{json.dumps(t, ensure_ascii=False)}`"
                              if e - s > 1 else f"frame {s} `{json.dumps(t, ensure_ascii=False)}`"
                              for s, e, t in column["entries"]))
        for note in case["notes"]:
            print(f"  - note `{json.dumps(note)}`")
    print()
    for fx, case in expected["refused"].items():
        print(f"- {fx}: {case['says']} `{case['refused']}`"
              + "".join(f"; note `{json.dumps(n)}`" for n in case["notes"]))
    print()
    sample = expected["cases"]["FX-XDTS-040"]
    layers = sample["layers"]
    print("| frame | " + " | ".join(l["name"] for l in layers) + " |")
    print("| --- " * (len(layers) + 1) + "|")
    lines = [sheet_line(l["spans"], 48).split() for l in layers]
    for f in range(48):
        print(f"| {f} | " + " | ".join(line[f] for line in lines) + " |")
    for note in sample["notes"]:
        print(json.dumps(note))
    print(json.dumps(sample["text"], ensure_ascii=False))

    # The claims the cases are there to make, checked on what was just read.
    c = {fx: v for fx, v in expected["cases"].items()}
    spans = {fx: [l["spans"] for l in v["layers"]] for fx, v in c.items()}
    assert spans["FX-XDTS-002"] == spans["FX-XDTS-003"] == [[[0, 2, 1], [2, 4, 2], [4, 6, 3],
                                                             [6, 8, 4]]]
    assert spans["FX-XDTS-004"] == [[[0, 3, 1], [3, 12, 2]]]
    assert spans["FX-XDTS-005"] == [[[0, 3, 1], [5, 7, 2]]]
    assert spans["FX-XDTS-008"] == [[[0, 4, 1], [4, 6, 2]]]
    assert [l["name"] for l in c["FX-XDTS-009"]["layers"]] == ["A", "B", "C"]
    assert spans["FX-XDTS-013"] == [[[0, 2, 1], [2, 4, 2]]]
    assert spans["FX-XDTS-017"] == spans["FX-XDTS-016"] and not c["FX-XDTS-017"]["notes"]
    assert c["FX-XDTS-018"]["composition"]["name"] == "fx_xdts_018"
    assert spans["FX-XDTS-020"] == [[[0, 2, 1], [2, 4, 2], [4, 6, 3]]]
    assert spans["FX-XDTS-024"] == [[[0, 2, 1], [4, 6, 2]]]
    assert spans["FX-XDTS-025"] == [[[0, 2, 1], [2, 4, 2]]]
    assert spans["FX-XDTS-028"] == [[[0, 2, 1], [2, 4, 2]], [[0, 4, 2]]]
    assert all(v["composition"]["width"] == SIZE[0] for v in c.values())
    assert c["FX-XDTS-040"]["text"] == [
        {"kind": "dialogue", "name": "Dialogue", "track": 0,
         "entries": [[0, 16, ["MIKA", "Over here!"]]]},
        {"kind": "camera", "name": "Camera", "track": 0, "entries": [[20, 48, ["FOLLOW"]]]}]
    assert [e[:2] for e in c["FX-XDTS-019"]["text"][0]["entries"]] == [[0, 3], [4, 5]]


if __name__ == "__main__":
    main()
