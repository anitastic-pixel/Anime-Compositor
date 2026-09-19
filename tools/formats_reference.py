"""D-72, proposed: more drawing formats in, and GIF, animated PNG and MP4 out.

Writes `Fixtures/formats/` and prints the tables document 25 shows. Written before any code.

Two kinds of claim are pinned here.

1. A drawing in another format is the same picture as its PNG twin. The files are written by
   Pillow, which is not the library the build will read them with, and each twin is Pillow's own
   reading of the file. Formats that keep every value (BMP, TGA, TIFF, lossless WebP) must match
   exactly. Formats that do not (JPEG, lossy WebP) must match within 3 of 255, because two
   correct decoders may round differently.

2. Frame timing in a GIF and in an MP4 is whole-number arithmetic, as ADR-018 made audio. A GIF
   counts in hundredths of a second, which 24 frames a second does not divide into, so frame n
   is shown from floor(n * 100 * den / num) to floor((n + 1) * 100 * den / num) hundredths and
   the delays differ by one so that the total never drifts. An MP4 counts in any unit, so the
   unit is the frame rate's numerator, every frame lasts the denominator, and it is exact.
"""
import json
from pathlib import Path

from PIL import Image

OUT = Path(__file__).resolve().parent.parent / "Fixtures" / "formats"
W, H = 16, 12
LOSSY_TOLERANCE = 3


def picture():
    """Smooth ramps, so a lossy format has nothing sharp to smear, and a ramp of alpha."""
    im = Image.new("RGBA", (W, H))
    for y in range(H):
        for x in range(W):
            im.putpixel((x, y), (x * 255 // (W - 1), y * 255 // (H - 1),
                                 (x + y) * 255 // (W + H - 2), 255 - x * 255 // (W - 1)))
    return im


def gif_delays(frames, num, den):
    return [(n + 1) * 100 * den // num - n * 100 * den // num for n in range(frames)]


# name: (what it says, mode, save arguments, exact?)
DRAWINGS = {
    "FX-FMT-001": ("bmp24.bmp", "A 24-bit BMP.", "RGB", {}, True),
    "FX-FMT-002": ("tga24.tga", "A 24-bit TGA, not compressed.", "RGB", {}, True),
    "FX-FMT-003": ("tga32.tga", "A 32-bit TGA with its alpha, which is how cels leave most "
                   "Japanese paint software.", "RGBA", {}, True),
    "FX-FMT-004": ("tga32_rle.tga", "The same, run-length compressed.", "RGBA",
                   {"compression": "tga_rle"}, True),
    "FX-FMT-005": ("tiff_rgba.tif", "An 8-bit TIFF with alpha, not compressed.", "RGBA",
                   {"compression": "raw"}, True),
    "FX-FMT-006": ("tiff_lzw.tif", "The same, LZW compressed.", "RGBA",
                   {"compression": "tiff_lzw"}, True),
    "FX-FMT-007": ("webp_lossless.webp", "A lossless WebP with alpha.", "RGBA",
                   {"lossless": True, "exact": True}, True),
    "FX-FMT-008": ("jpeg_q95.jpg", "A JPEG. It has no alpha, so it is opaque.", "RGB",
                   {"quality": 95, "subsampling": 0}, False),
    "FX-FMT-009": ("jpeg_grey.jpg", "A greyscale JPEG: the one value is red, green and blue.",
                   "L", {"quality": 95}, False),
    "FX-FMT-010": ("webp_lossy.webp", "A lossy WebP.", "RGB", {"quality": 95}, False),
}

# frames, numerator, denominator
GIF = {
    "FX-FMT-020": ("24 frames a second: 4.1667 hundredths to a frame, so every sixth delay is 5.",
                   24, 24, 1),
    "FX-FMT-021": ("12 frames a second: 8.33 hundredths.", 12, 12, 1),
    "FX-FMT-022": ("24000/1001 frames a second.", 24, 24000, 1001),
    "FX-FMT-023": ("25 frames a second divides exactly: every delay is 4.", 25, 25, 1),
}

# D-73. An MP4's quality is a named level, and a level is thousandths of a bit for every pixel of
# every frame. The bitrate asked of the encoder is whole bits a second, floored, and never below
# 1 or above 100 megabits.
LEVELS = {"preview": 100, "standard": 200, "high": 500}
# width, height, numerator, denominator
SIZES = ((1920, 1080, 24, 1), (1280, 720, 24000, 1001), (64, 64, 24, 1), (3840, 2160, 60, 1))


def bitrate(w, h, num, den, thousandths):
    return min(max(w * h * num * thousandths // (den * 1000), 1_000_000), 100_000_000)


LIMIT = 16  # D-73a: the most of a difference, in each of red, green and blue, passed on


def dither(pixels, width, palette, limit=None):
    """D-73's GIF dithering, Floyd and Steinberg's, in whole numbers so two programs agree.

    Pixels are (r, g, b, a) rows left to right, top to bottom. A pixel under half alpha is
    see-through: it gets no colour (None), takes no error and passes none on. Otherwise the pixel
    plus the error carried to it goes to the nearest palette colour (least squared distance, the
    first one on a tie), and the difference goes 7 sixteenths right, 3 below left, 5 below and 1
    below right, each floored (toward minus infinity).

    D-73a (`limit` given): the pixel plus its carried error is first held within 0 to
    255, and the difference is held within plus and minus `limit` before it is shared out. One
    pixel the palette has nothing near cannot then tint its neighbours.
    """
    height = len(pixels) // width
    carried = [[0, 0, 0] for _ in pixels]
    out = []
    for i, (r, g, b, a) in enumerate(pixels):
        if a < 128:
            out.append(None)
            continue
        want = [v + c for v, c in zip((r, g, b), carried[i])]
        if limit is not None:
            want = [max(0, min(255, v)) for v in want]
        pick = min(range(len(palette)),
                   key=lambda p: (sum((x - y) ** 2 for x, y in zip(want, palette[p])), p))
        out.append(pick)
        x, y = i % width, i // width
        for dx, dy, part in ((1, 0, 7), (-1, 1, 3), (0, 1, 5), (1, 1, 1)):
            if 0 <= x + dx < width and y + dy < height:
                for c in range(3):
                    e = want[c] - palette[pick][c]
                    if limit is not None:
                        e = max(-limit, min(limit, e))
                    carried[(y + dy) * width + x + dx][c] += e * part >> 4
    return out


GREY = lambda v: (v, v, v, 255)
# says, width, pixels, palette
DITHER = {
    "FX-FMT-050": ("A flat mid grey with only black and white to spend becomes a checker of the two.",
                   4, [GREY(128)] * 16, [(0, 0, 0), (255, 255, 255)]),
    "FX-FMT-051": ("A grey ramp, black to white, in black, mid grey and white.",
                   8, [GREY(x * 255 // 7) for x in range(8)] * 2,
                   [(0, 0, 0), (128, 128, 128), (255, 255, 255)]),
    "FX-FMT-052": ("Orange with red and yellow to spend, and a see-through pixel in the way: "
                   "it gets no colour and carries no error.",
                   4, [(255, 128, 0, 255)] * 5 + [(255, 128, 0, 0)] + [(255, 128, 0, 255)] * 6,
                   [(255, 0, 0), (255, 255, 0)]),
}


# D-73a: the same, with the passed-on difference held within LIMIT. The palettes are
# close together, as the 256 colours picked from a picture are; FX-FMT-050 to 052 spend two or
# three far-apart colours, which is the case the limit gives up.
DITHER_LIMITED = {
    "FX-FMT-053": ("A flat grey of 100 between greys of 96 and 104 becomes a mix of the two.",
                   4, [GREY(100)] * 16, [(96, 96, 96), (104, 104, 104)]),
    "FX-FMT-054": ("A soft grey ramp, 90 to 118, in greys of 88, 104 and 120, with a see-through "
                   "pixel in the way: it gets no colour and carries no error.",
                   8, [GREY(90 + 4 * x) for x in range(8)] + [GREY(90 + 4 * x) for x in range(3)]
                   + [(102, 102, 102, 0)] + [GREY(90 + 4 * x) for x in range(4, 8)],
                   [(88, 88, 88), (104, 104, 104), (120, 120, 120)]),
    "FX-FMT-055": ("The specks: a field of dull red the palette has nothing near, with two greys "
                   "and a bright red to spend. It becomes the nearer grey, with no bright red dots.",
                   6, [(140, 100, 100, 255)] * 24,
                   [(96, 96, 96), (104, 104, 104), (255, 40, 40)]),
}


def main():
    (OUT / "media").mkdir(parents=True, exist_ok=True)
    cases = {}
    src = picture()
    print("| case | file | says | its PNG twin | match |\n| --- | --- | --- | --- | --- |")
    for fx, (name, says, mode, args, exact) in DRAWINGS.items():
        path = OUT / "media" / name
        src.convert(mode).save(path, **args)
        twin = Path(name).stem + ".twin.png"
        with Image.open(path) as back:
            back.convert("RGBA").save(OUT / "media" / twin)
        with Image.open(OUT / "media" / twin) as t:
            worst = max(abs(a - b) for p, q in zip(t.getdata(), src.convert(mode).convert("RGBA").getdata())
                        for a, b in zip(p, q))
        assert worst == 0 if exact else worst <= 32, (name, worst)
        tolerance = 0 if exact else LOSSY_TOLERANCE
        cases[fx] = {"says": says, "file": "media/" + name, "same_picture_as": "media/" + twin,
                     "width": W, "height": H, "tolerance_of_255": tolerance}
        print(f"| {fx} | `{name}` | {says} | `{twin}` | "
              f"{'exact' if exact else f'within {tolerance} of 255'} |")
    print()

    for fx, (says, frames, num, den) in GIF.items():
        delays = gif_delays(frames, num, den)
        assert sum(delays) == frames * 100 * den // num
        cases[fx] = {"says": says, "frame_rate": [num, den], "frames": frames,
                     "delays_hundredths": delays, "total_hundredths": sum(delays)}
        print(f"{fx}: {says}\n\n| frames | delays, in hundredths of a second | total |\n"
              f"| --- | --- | --- |\n| {frames} | {' '.join(map(str, delays))} | {sum(delays)} |\n")

    cases["FX-FMT-030"] = {
        "says": "An MP4 counts time in the frame rate's numerator and every frame lasts the "
                "denominator, so 24000/1001 is exact and no frame is dropped or doubled.",
        "rows": [{"frame_rate": [n, d], "timescale": n, "frame_duration": d, "frames": f,
                  "duration": f * d} for n, d, f in ((24, 1, 48), (24000, 1001, 48), (30, 1, 1))]}
    print("FX-FMT-030: " + cases["FX-FMT-030"]["says"] + "\n\n| frame rate | timescale | "
          "each frame lasts | frames | the film lasts |\n| --- | --- | --- | --- | --- |")
    for r in cases["FX-FMT-030"]["rows"]:
        print(f"| {r['frame_rate'][0]}/{r['frame_rate'][1]} | {r['timescale']} | "
              f"{r['frame_duration']} | {r['frames']} | {r['duration']} |")
    print()

    rows = [{"size": [w, h], "frame_rate": [n, d], "level": level,
             "bits_a_second": bitrate(w, h, n, d, t)}
            for w, h, n, d in SIZES for level, t in LEVELS.items()]
    cases["FX-FMT-040"] = {
        "says": "An MP4's quality level is thousandths of a bit for every pixel of every frame: "
                "preview 100, standard 200, high 500. The bitrate asked of the encoder is floored "
                "to whole bits a second and kept between 1 and 100 megabits.",
        "thousandths_of_a_bit": LEVELS, "rows": rows}
    print("FX-FMT-040: " + cases["FX-FMT-040"]["says"] + "\n\n| size | frame rate | level | "
          "bits a second asked for |\n| --- | --- | --- | --- |")
    for r in rows:
        print(f"| {r['size'][0]}x{r['size'][1]} | {r['frame_rate'][0]}/{r['frame_rate'][1]} | "
              f"{r['level']} | {r['bits_a_second']} |")
    print()

    for fx, (says, width, pixels, palette) in DITHER.items():
        picked = dither(pixels, width, palette)
        # D-73a, accepted: kept as the record of D-73's first rule; no build answers to them.
        cases[fx] = {"says": says, "retired_by": "D-73a", "width": width, "pixels_rgba": pixels,
                     "palette_rgb": palette, "palette_index_or_null": picked}
        grid = ["".join("." if p is None else str(p) for p in picked[i:i + width])
                for i in range(0, len(picked), width)]
        print(f"{fx}: {says}\n\nPalette: " + ", ".join(f"{i} is {c}" for i, c in enumerate(palette))
              + ". A dot is see-through.\n\n```\n" + "\n".join(grid) + "\n```\n")

    grids = lambda picked, width: ["".join("." if p is None else str(p) for p in picked[i:i + width])
                                   for i in range(0, len(picked), width)]
    print(f"D-73a: the difference passed on is held within {LIMIT}.\n")
    for fx, (says, width, pixels, palette) in DITHER_LIMITED.items():
        picked = dither(pixels, width, palette, LIMIT)
        cases[fx] = {"says": says, "width": width, "pixels_rgba": pixels, "palette_rgb": palette,
                     "limit": LIMIT, "palette_index_or_null": picked}
        print(f"{fx}: {says}\n\nPalette: " + ", ".join(f"{i} is {c}" for i, c in enumerate(palette))
              + ". A dot is see-through.\n\n```\n" + "\n".join(grids(picked, width)) + "\n```\n")
    # What D-73 as accepted makes of the speck, so the two can be told apart.
    _, width, pixels, palette = DITHER_LIMITED["FX-FMT-055"]
    before = dither(pixels, width, palette)
    assert before.count(2) > 1 and dither(pixels, width, palette, LIMIT).count(2) == 0
    print("The same picture under D-73 as accepted, for comparison, not a fixture:\n\n```\n"
          + "\n".join(grids(before, width)) + "\n```\n")

    cases["FX-FMT-060"] = {
        "says": "An MP4 says what its colour is: BT.709 primaries, transfer and matrix, video "
                "range. Read back from the file, from the H.264 header or the container's colour "
                "box, whichever the file carries.",
        "colour_primaries": 1, "transfer_characteristics": 1, "matrix_coefficients": 1,
        "full_range": False}
    print("FX-FMT-060: " + cases["FX-FMT-060"]["says"] + "\n\n| primaries | transfer | matrix | "
          "full range |\n| --- | --- | --- | --- |\n| 1 | 1 | 1 | no |\n")

    (OUT / "expected_formats.json").write_text(json.dumps({"cases": cases}, indent=2) + "\n",
                                               encoding="utf-8")


if __name__ == "__main__":
    main()
