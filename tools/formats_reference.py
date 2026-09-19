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

    (OUT / "expected_formats.json").write_text(json.dumps({"cases": cases}, indent=2) + "\n",
                                               encoding="utf-8")


if __name__ == "__main__":
    main()
