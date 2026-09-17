"""EXR, read and written a second way.

D-62 proposes how R-15 reads and writes OpenEXR. The build will do both through the `exr` crate.
This file does both through the OpenEXR project's own library, by way of its Python bindings,
and applies D-62's rules to what that library reads in numpy. The two share nothing but the
file format, so they agree only if both are right. That is document 11's T-14: "format-specific
files from independent sources".

It writes, under `Fixtures/exr/`:

- the test files, each written by the OpenEXR library, in folders named for what they test:
  `compression/`, `types/`, `channels/`, `windows/`, `values/`, `colour/`, `refused/` and
  `sequence/`;
- `expected_exr.json`: for every file, whether it is drawn or refused, and if drawn its size,
  every pixel as it must reach the working buffer, and every adjustment D-62 requires the
  build to report; and for the four export cases, the pixels an exported file must hold.

It also checks files the build exported, reading them with the OpenEXR library:

    python tools/exr_reference.py check <folder>

The folder holds `<case>_0000.exr` for each export case named in `expected_exr.json`. The
answer is a Markdown table on standard output, and the exit status is 1 if any row fails.

Fixtures are read-only to implementation work: this file is run when the specification changes,
and never to make a build pass.

Needs Python 3 and, from PyPI, exactly these two (a virtual environment is enough):

    pip install OpenEXR==3.4.15 numpy
    python tools/exr_reference.py
"""

import json
import re
import shutil
import sys
from pathlib import Path

import numpy as np
import OpenEXR

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "Fixtures" / "exr"
EXPECTED = OUT / "expected_exr.json"

W, H = 8, 6
HALF_MAX = 65504.0
# ITU-R BT.709 primaries and D65 white, as OpenEXR's own default `chromaticities`.
REC709 = (0.64, 0.33, 0.30, 0.60, 0.15, 0.06, 0.3127, 0.3290)
ACESCG = (0.713, 0.293, 0.165, 0.830, 0.128, 0.044, 0.32168, 0.33767)
FRAME_RATES = {"none_float_as_float": (24, 1), "none_float_as_half": (24, 1),
               "specials_as_float": (24000, 1001), "specials_as_half": (24000, 1001)}
COMPRESSIONS = {
    "none": OpenEXR.NO_COMPRESSION, "rle": OpenEXR.RLE_COMPRESSION,
    "zips": OpenEXR.ZIPS_COMPRESSION, "zip": OpenEXR.ZIP_COMPRESSION,
    "piz": OpenEXR.PIZ_COMPRESSION, "pxr24": OpenEXR.PXR24_COMPRESSION,
    "b44": OpenEXR.B44_COMPRESSION, "b44a": OpenEXR.B44A_COMPRESSION,
    "dwaa": OpenEXR.DWAA_COMPRESSION, "dwab": OpenEXR.DWAB_COMPRESSION,
}
HTJ2K = {OpenEXR.HTJ2K32_COMPRESSION, OpenEXR.HTJ2K256_COMPRESSION}


# --- The pictures -------------------------------------------------------------------------


def picture():
    """8 by 6, premultiplied linear RGBA: negative and over-range colour, every kind of
    coverage, and a pixel that adds light where there is no coverage at all."""
    y, x = np.mgrid[0:H, 0:W].astype(np.float32)
    a = np.tile(np.array([1, 1, 0.5, 0.25, 0, 1, 0.75, 1], np.float32), (H, 1))
    a[5, :] = 0.125
    r = ((x - 2) * 0.375 + y * 0.125) * a
    g = (x * y / 7) * a
    b = (2.5 - (x + y) * 0.3) * a
    r[0, 7], g[0, 7], b[0, 7] = 1000.0, 1 / 3, 0.1
    r[0, 4], g[0, 4], b[0, 4] = 0.5, 0.25, 0.125  # alpha 0 here: light with no coverage
    return {"R": r, "G": g, "B": b, "A": a}


def specials():
    """Float only. Values a half cannot hold, and values no picture should hold."""
    p = {k: v.copy() for k, v in picture().items()}
    p["R"][1, 1], p["G"][1, 2], p["B"][1, 3] = np.nan, np.inf, -np.inf
    p["R"][2, 0] = 1e30
    p["G"][2, 1] = -70000.0
    p["A"][3, 0], p["A"][3, 1], p["A"][3, 2] = 1.5, -0.25, np.nan
    return p


def constant(value):
    return {"R": np.full((H, W), value, np.float32), "G": np.zeros((H, W), np.float32),
            "B": np.zeros((H, W), np.float32), "A": np.ones((H, W), np.float32)}


# --- Writing, with the OpenEXR library -------------------------------------------------------


def box(x0, y0, x1, y1):
    return (np.array([x0, y0], np.int32), np.array([x1, y1], np.int32))


def write(rel, channels, dtype="f2", compression=OpenEXR.ZIP_COMPRESSION, data=None,
          display=None, **attributes):
    """One single-part file. `data` and `display` are (x0, y0, x1, y1), inclusive; `channels`
    must already be the data window's size."""
    shape = next(iter(channels.values())).shape
    data = data or (0, 0, shape[1] - 1, shape[0] - 1)
    display = display or data
    header = {"compression": compression, "type": OpenEXR.scanlineimage,
              "dataWindow": box(*data), "displayWindow": box(*display),
              "lineOrder": OpenEXR.INCREASING_Y, "pixelAspectRatio": 1.0}
    header.update(attributes)
    typed = {k: v.astype(dtype) if dtype else v for k, v in channels.items()}
    path = OUT / rel
    path.parent.mkdir(parents=True, exist_ok=True)
    OpenEXR.File(header, typed).write(str(path))


def tiles(size):
    t = OpenEXR.TileDescription()
    t.xSize = t.ySize = size
    t.mode = OpenEXR.ONE_LEVEL
    t.roundingMode = OpenEXR.ROUND_DOWN
    return t


def pad(p, left, top, right, bottom, value):
    return {k: np.pad(v, ((top, bottom), (left, right)), constant_values=value(k))
            for k, v in p.items()}


def make_files():
    """Every test file, and what it is for. Returns the list in the order the table reads."""
    p = picture()
    cases = []

    def add(case, rel, what):
        cases.append((case, rel, what))

    for name, method in COMPRESSIONS.items():
        for dtype, label in (("f2", "half"), ("f4", "float")):
            rel = f"compression/{name}_{label}.exr"
            write(rel, p, dtype, method)
            add("FX-EXR-001", rel, f"{name.upper()} compression, {label} samples")

    write("types/uint.exr", {"R": np.arange(W * H, dtype=np.uint32).reshape(H, W),
                             "G": np.full((H, W), 3, np.uint32),
                             "B": np.zeros((H, W), np.uint32),
                             "A": np.tile(np.array([0, 1, 2, 1, 1, 1, 1, 1], np.uint32),
                                          (H, 1))}, "u4")
    add("FX-EXR-002", "types/uint.exr", "whole-number samples, alpha 0, 1 and 2")
    write("types/mixed.exr", {"R": p["R"].astype("f2"), "G": p["G"].astype("f4"),
                              "B": p["B"].astype("f2"), "A": p["A"].astype("f4")}, None)
    add("FX-EXR-002", "types/mixed.exr", "half and float channels in one file")

    write("channels/rgb.exr", {k: p[k] for k in "RGB"})
    add("FX-EXR-003", "channels/rgb.exr", "no alpha channel")
    write("channels/y.exr", {"Y": p["R"]})
    add("FX-EXR-003", "channels/y.exr", "luminance only")
    write("channels/ya.exr", {"Y": p["G"], "A": p["A"]})
    add("FX-EXR-003", "channels/ya.exr", "luminance and alpha")
    write("channels/r_only.exr", {"R": p["R"]})
    add("FX-EXR-003", "channels/r_only.exr", "red only")
    write("channels/extra.exr", dict(p, Z=p["G"] * 10, **{"diffuse.R": p["B"],
                                                         "diffuse.G": p["A"]}))
    add("FX-EXR-003", "channels/extra.exr", "RGBA with depth and a render layer beside it")
    write("channels/rgb_and_y.exr", dict(p, Y=p["A"]))
    add("FX-EXR-003", "channels/rgb_and_y.exr", "RGBA and luminance both")
    write("channels/lowercase.exr", {"r": p["R"], "g": p["G"], "b": p["B"], "a": p["A"]})
    add("FX-EXR-003", "channels/lowercase.exr", "channel names in lower case")

    inner = {k: v[1:4, 2:6] for k, v in p.items()}
    write("windows/inset.exr", inner, data=(2, 1, 5, 3), display=(0, 0, W - 1, H - 1))
    add("FX-EXR-004", "windows/inset.exr", "data window inside the display window")
    over = pad(p, 2, 1, 2, 1, lambda k: 9.0)
    write("windows/overscan.exr", over, data=(-2, -1, W + 1, H), display=(0, 0, W - 1, H - 1))
    add("FX-EXR-004", "windows/overscan.exr", "data window beyond the display window")
    write("windows/offset.exr", p, data=(10, 20, 17, 25), display=(10, 20, 17, 25))
    add("FX-EXR-004", "windows/offset.exr", "both windows away from the origin")
    write("windows/disjoint.exr", {k: v[:2, :2] for k, v in p.items()},
          data=(20, 20, 21, 21), display=(0, 0, W - 1, H - 1))
    add("FX-EXR-004", "windows/disjoint.exr", "data window wholly outside the display window")
    write("windows/decreasing_y.exr", p, lineOrder=OpenEXR.DECREASING_Y)
    add("FX-EXR-004", "windows/decreasing_y.exr", "lines stored bottom to top")
    write("windows/tiled.exr", p, compression=OpenEXR.PIZ_COMPRESSION,
          type=OpenEXR.tiledimage, tiles=tiles(3))
    add("FX-EXR-004", "windows/tiled.exr", "tiled, 3 by 3 tiles, one level")
    write("windows/aspect.exr", p, pixelAspectRatio=2.0)
    add("FX-EXR-004", "windows/aspect.exr", "pixel aspect ratio 2")

    write("values/specials.exr", specials(), "f4")
    add("FX-EXR-005", "values/specials.exr", "NaN, infinities, huge values, alpha out of range")

    write("colour/rec709.exr", p, chromaticities=REC709)
    add("FX-EXR-006", "colour/rec709.exr", "primaries stated, and Rec. 709")
    write("colour/acescg.exr", p, chromaticities=ACESCG)
    add("FX-EXR-006", "colour/acescg.exr", "primaries stated, and ACEScg")

    parts = [OpenEXR.Part({"compression": OpenEXR.ZIP_COMPRESSION}, {k: v for k, v in p.items()},
                          "beauty"),
             OpenEXR.Part({"compression": OpenEXR.ZIP_COMPRESSION}, {"Z": p["R"]}, "depth")]
    (OUT / "refused").mkdir(parents=True, exist_ok=True)
    OpenEXR.File(parts).write(str(OUT / "refused/multipart.exr"))
    add("FX-EXR-007", "refused/multipart.exr", "two parts")
    deep = np.empty((H, W), dtype=object)
    for y in range(H):
        for x in range(W):
            deep[y, x] = np.array([1.0, 2.0][: 1 + (x + y) % 2], np.float32)
    OpenEXR.File({"type": OpenEXR.deepscanline, "compression": OpenEXR.ZIPS_COMPRESSION},
                 {"Z": deep}).write(str(OUT / "refused/deep.exr"))
    add("FX-EXR-007", "refused/deep.exr", "deep data")
    write("refused/htj2k.exr", p, compression=OpenEXR.HTJ2K32_COMPRESSION)
    add("FX-EXR-007", "refused/htj2k.exr", "HTJ2K compression")
    write("refused/layers_only.exr", {f"beauty.{k}": v for k, v in p.items()})
    add("FX-EXR-007", "refused/layers_only.exr", "colour only inside a render layer")
    write("refused/luminance_chroma.exr", {"Y": p["G"], "RY": p["R"] * 0.1, "BY": p["B"] * 0.1})
    add("FX-EXR-007", "refused/luminance_chroma.exr", "luminance with colour difference")
    whole = (OUT / "compression/zip_half.exr").read_bytes()
    (OUT / "refused/truncated.exr").write_bytes(whole[: len(whole) // 2])
    add("FX-EXR-007", "refused/truncated.exr", "the first half of a good file")
    (OUT / "refused/empty.exr").write_bytes(b"")
    add("FX-EXR-007", "refused/empty.exr", "no bytes at all")
    (OUT / "refused/not_exr.exr").write_bytes(b"This is a text file named .exr.\n")
    add("FX-EXR-007", "refused/not_exr.exr", "text with an .exr name")

    for n in (1, 2, 3, 5):
        rel = f"sequence/render_{n:04d}.exr"
        write(rel, constant(n / 10))
        add("FX-EXR-008", rel, f"drawing {n} of a sequence with drawing 4 missing")
    return cases


# --- Reading, with the OpenEXR library, then D-62's rules ------------------------------------


def refused(diagnostic, reason):
    return {"answer": "refused", "diagnostic": diagnostic, "reason": reason}


def read(path):
    """What D-62 says the working buffer receives from `path`."""
    try:
        f = OpenEXR.File(str(path), separate_channels=True)
        count = len(f.parts)
    except Exception:
        return refused("MEDIA_DECODE_FAILED", "unreadable")
    if count == 0:
        return refused("MEDIA_DECODE_FAILED", "unreadable")
    if count > 1:
        return refused("MEDIA_UNSUPPORTED_FORMAT", "multipart")
    header = f.header()
    if header["type"] in (OpenEXR.deepscanline, OpenEXR.deeptile):
        return refused("MEDIA_UNSUPPORTED_FORMAT", "deep")
    if header["compression"] in HTJ2K:
        return refused("MEDIA_UNSUPPORTED_FORMAT", "htj2k")
    channels = f.channels()
    if any(c.xSampling != 1 or c.ySampling != 1 for c in channels.values()):
        return refused("MEDIA_UNSUPPORTED_FORMAT", "subsampled")
    names = list(channels)  # the library lists them as the file stores them: sorted
    if any(n in channels for n in "RGB"):
        used = [n for n in "RGBA" if n in channels]
    elif "RY" in channels or "BY" in channels:
        return refused("MEDIA_UNSUPPORTED_FORMAT", "luminance_chroma")
    elif "Y" in channels:
        used = [n for n in "YA" if n in channels]
    else:
        return refused("MEDIA_UNSUPPORTED_FORMAT", "no_colour_channels")

    (dx0, dy0), (dx1, dy1) = (tuple(int(v) for v in c) for c in header["dataWindow"])
    (px0, py0), (px1, py1) = (tuple(int(v) for v in c) for c in header["displayWindow"])
    dw, dh = dx1 - dx0 + 1, dy1 - dy0 + 1
    width, height = px1 - px0 + 1, py1 - py0 + 1

    def plane(name, default):
        if name in channels:
            return channels[name].pixels.astype(np.float32)
        return np.full((dh, dw), default, np.float32)

    if "Y" in used:
        y = plane("Y", 0.0)
        data = np.stack([y, y, y, plane("A", 1.0)], -1)
    else:
        data = np.stack([plane("R", 0.0), plane("G", 0.0), plane("B", 0.0), plane("A", 1.0)],
                        -1)

    image = np.zeros((height, width, 4), np.float32)
    # The overlap of the two windows, in file coordinates.
    ox0, oy0 = max(dx0, px0), max(dy0, py0)
    ox1, oy1 = min(dx1, px1), min(dy1, py1)
    inside = 0
    if ox0 <= ox1 and oy0 <= oy1:
        image[oy0 - py0: oy1 - py0 + 1, ox0 - px0: ox1 - px0 + 1] = \
            data[oy0 - dy0: oy1 - dy0 + 1, ox0 - dx0: ox1 - dx0 + 1]
        inside = (ox1 - ox0 + 1) * (oy1 - oy0 + 1)

    adjusted = []
    ignored = [n for n in names if n not in used]
    if ignored:
        adjusted.append({"reason": "channels_ignored", "detail": ", ".join(ignored)})
    if dw * dh - inside:
        adjusted.append({"reason": "outside_display_window", "detail": str(dw * dh - inside)})
    bad = ~np.isfinite(image)
    if bad.any():
        adjusted.append({"reason": "non_finite", "detail": str(int(bad.sum()))})
        image[bad] = 0.0
    alpha = image[..., 3]
    out = (alpha < 0) | (alpha > 1)
    if out.any():
        adjusted.append({"reason": "alpha_clamped", "detail": str(int(out.sum()))})
        image[..., 3] = np.clip(alpha, 0, 1)
    aspect = float(header.get("pixelAspectRatio", 1.0))
    if aspect != 1.0:
        adjusted.append({"reason": "pixel_aspect", "detail": repr(aspect)})
    stated = header.get("chromaticities")
    if stated is not None and tuple(np.float32(v) for v in stated) != tuple(
            np.float32(v) for v in REC709):
        adjusted.append({"reason": "primaries", "detail": ", ".join(
            repr(float(np.float32(v))) for v in stated)})
    return {"answer": "drawn", "width": width, "height": height, "adjusted": adjusted,
            "pixels": [[float(v) for v in px] for px in image.reshape(-1, 4)]}


# --- Exporting ---------------------------------------------------------------------------------


def exported(pixels, depth):
    """D-62's export rule for one pixel list: float is written as it is; half is clamped to
    the half range and then rounded to the nearest half, ties to even."""
    a = np.array(pixels, np.float32)
    if depth == "half":
        a = np.clip(a, -HALF_MAX, HALF_MAX).astype(np.float16).astype(np.float32)
    return [[float(v) for v in px] for px in a]


def export_cases(answers):
    cases = []
    for name, (num, den) in FRAME_RATES.items():
        source = "compression/none_float.exr" if name.startswith("none") else "values/specials.exr"
        depth = name.rsplit("_", 1)[1]
        cases.append({
            "case": "FX-EXR-009" if name.startswith("none") else "FX-EXR-010",
            "name": name, "source": source, "depth": depth, "frame_rate": [num, den],
            "width": W, "height": H,
            "pixels": exported(answers[source]["pixels"], depth),
        })
    return cases


def check(folder):
    """Read what the build exported and say, row by row, whether it is what D-62 asks for."""
    expected = json.loads(EXPECTED.read_text("utf-8"))
    rows = []

    def row(case, what, want, got):
        rows.append((case, what, str(want), str(got), "pass" if want == got else "FAIL"))

    for case in expected["export"]:
        name = case["name"]
        path = Path(folder) / f"{name}_0000.exr"
        try:
            f = OpenEXR.File(str(path), separate_channels=True)
            parts = len(f.parts)
        except Exception as error:
            row(name, "the file opens in the OpenEXR library", "yes", f"no: {error}")
            continue
        row(name, "parts", 1, parts)
        if parts != 1:
            continue
        h = f.header()
        c = f.channels()
        w1, h1 = case["width"] - 1, case["height"] - 1
        # `chunkCount` is structural: OpenEXR 2 and later may write it in any file, or leave it out.
        row(name, "attributes, and no others",
            ["channels", "chromaticities", "compression", "dataWindow", "displayWindow",
             "framesPerSecond", "lineOrder", "pixelAspectRatio", "screenWindowCenter",
             "screenWindowWidth", "type"], sorted(k for k in h if k != "chunkCount"))
        row(name, "type", "scanlineimage", h["type"].name)
        row(name, "compression", "ZIP_COMPRESSION", h["compression"].name)
        row(name, "channels, as stored", "A, B, G, R", ", ".join(c.name for c in h["channels"]))
        row(name, "sample type", {"float16" if case["depth"] == "half" else "float32"},
            {str(v.pixels.dtype) for v in c.values()})
        row(name, "data window", [0, 0, w1, h1], [int(v) for corner in h["dataWindow"] for v in corner])
        row(name, "display window", [0, 0, w1, h1],
            [int(v) for corner in h["displayWindow"] for v in corner])
        row(name, "line order", "INCREASING_Y", h["lineOrder"].name)
        row(name, "pixel aspect ratio", 1.0, float(h["pixelAspectRatio"]))
        row(name, "screen window centre", [0.0, 0.0],
            [float(v) for v in h["screenWindowCenter"]])
        row(name, "screen window width", 1.0, float(h["screenWindowWidth"]))
        row(name, "chromaticities", [float(np.float32(v)) for v in REC709],
            [float(v) for v in h["chromaticities"]])
        fps = h["framesPerSecond"]
        row(name, "frames per second, as a fraction", case["frame_rate"],
            [fps.numerator, fps.denominator] if hasattr(fps, "denominator") else repr(fps))
        got = np.stack([c[k].pixels.astype(np.float32) for k in "RGBA"], -1).reshape(-1, 4)
        want = np.array(case["pixels"], np.float32)
        same = got.shape == want.shape and np.array_equal(got, want)
        row(name, "every sample, exactly", f"{len(want) * 4} samples",
            f"{len(want) * 4} samples" if same else "differs")

    print("| Case | Check | Expected | Actual | Result |")
    print("|---|---|---|---|---|")
    for r in rows:
        print("| " + " | ".join(v.replace("|", "\\|") for v in r) + " |")
    failed = sum(r[4] != "pass" for r in rows)
    print(f"\n**{len(rows) - failed} of {len(rows)} checks pass.**")
    return 1 if failed else 0


# --- Output ------------------------------------------------------------------------------------


def dump(value):
    """Two-space JSON, with each short list of numbers kept on one line so a pixel reads as one."""
    text = json.dumps(value, indent=2, ensure_ascii=False, allow_nan=False)
    number = r"-?\d+(?:\.\d+)?(?:e[-+]?\d+)?"
    pattern = re.compile(r"\[\s+(" + number + r"(?:,\s+" + number + r")*)\s+\]")
    return pattern.sub(lambda m: "[" + re.sub(r",\s+", ", ", m.group(1)) + "]", text) + "\n"


def main():
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir(parents=True)
    cases = make_files()
    answers = {}
    files = []
    for case, rel, what in cases:
        answer = read(OUT / rel)
        answers[rel] = answer
        files.append({"case": case, "file": rel, "what": what, **answer})
    expected = {
        "library": f"OpenEXR {OpenEXR.__version__}, numpy {np.__version__}",
        "files": files,
        "sequence": {"files": [f"sequence/render_{n:04d}.exr" for n in (1, 2, 3, 5)],
                     "pattern": "render_%04d.exr", "drawings": [1, 2, 3, 5], "missing": [4]},
        "export": export_cases(answers),
    }
    EXPECTED.write_bytes(dump(expected).encode("utf-8"))
    for f in files:
        print(f["case"], f["file"], f["answer"], f.get("reason", ""),
              "; ".join(a["reason"] for a in f.get("adjusted", [])))


if __name__ == "__main__":
    if sys.argv[1:2] == ["check"] and len(sys.argv) == 3:
        sys.exit(check(sys.argv[2]))
    main()
