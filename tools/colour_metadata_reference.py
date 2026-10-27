"""D-902: PNG files carrying colour chunks, and what MEDIA_COLOR_METADATA_IGNORED must say.

Writes Fixtures/colour/metadata/*.png and expected_colour_metadata.json. Every file is built here
byte by byte with the standard library (struct, zlib), sharing nothing with the build's decoder,
and the expected chunk descriptions come from the values written, not from reading them back.

Every file holds the same 2x2 8-bit RGB picture, so a build must decode each one to the bytes of
plain.png: the chunks change what is said, never the picture.

Run: python -I tools/colour_metadata_reference.py
"""

import json
import os
import struct
import zlib

HERE = os.path.dirname(os.path.abspath(__file__))
OUT = os.path.join(HERE, "..", "Fixtures", "colour", "metadata")

# Rows of 2 RGB pixels, each row led by filter byte 0.
PIXELS = [(255, 128, 0), (0, 64, 255), (10, 200, 30), (250, 250, 250)]
RAW = b"".join(
    b"\x00" + bytes(c for p in PIXELS[r * 2 : r * 2 + 2] for c in p) for r in range(2)
)

# What the build assumes of every PNG (document 21): sRGB. Chunks saying exactly this are quiet.
SRGB_WHITE_RGB = (31270, 32900, 64000, 33000, 30000, 60000, 15000, 6000)
REC2020 = (31270, 32900, 70800, 29200, 17000, 79700, 13100, 4600)
PROFILE = b"not a real ICC profile, D-902 fixture " * 3  # 114 bytes; the build never reads it


def chunk(kind, data):
    body = kind + data
    return struct.pack(">I", len(data)) + body + struct.pack(">I", zlib.crc32(body))


def png(chunks):
    ihdr = struct.pack(">IIBBBBB", 2, 2, 8, 2, 0, 0, 0)  # 2x2, 8 bits, RGB
    out = b"\x89PNG\r\n\x1a\n" + chunk(b"IHDR", ihdr)
    for kind, data in chunks:
        out += chunk(kind, data)
    out += chunk(b"IDAT", zlib.compress(RAW, 9)) + chunk(b"IEND", b"")
    return out


def srgb():
    return (b"sRGB", b"\x00"), "sRGB"


def gama(v):
    return (b"gAMA", struct.pack(">I", v)), f"gAMA {v}"


def chrm(v):
    w = (
        f"cHRM white {v[0]} {v[1]}, red {v[2]} {v[3]}, "
        f"green {v[4]} {v[5]}, blue {v[6]} {v[7]}"
    )
    return (b"cHRM", struct.pack(">8I", *v)), w


def iccp():
    data = b"D-902 test\x00\x00" + zlib.compress(PROFILE)
    return (b"iCCP", data), f"iCCP {len(PROFILE)} bytes"


def cicp(p, t, m, full):
    return (b"cICP", bytes([p, t, m, full])), f"cICP {p}/{t}/{m}/{full}"


# name -> (chunks in file order, warned?). The description lists every colour chunk the file
# carries, in the fixed order sRGB, gAMA, cHRM, iCCP, cICP, once any one of them is not quiet.
# Quiet: sRGB; a cHRM of the sRGB primaries; cICP 1/13/0/1; gAMA 45455 only beside sRGB (alone
# it asks for a plain 2.2 power curve, up to 9 levels of 255 from the sRGB curve).
CASES = {
    "plain.png": ([], False),
    "srgb.png": ([srgb()], False),
    "srgb_gama_chrm.png": ([srgb(), gama(45455), chrm(SRGB_WHITE_RGB)], False),
    "gama_45455.png": ([gama(45455)], True),
    "cicp_srgb.png": ([cicp(1, 13, 0, 1)], False),
    "gama_linear.png": ([gama(100000)], True),
    "chrm_rec2020.png": ([chrm(REC2020)], True),
    "iccp.png": ([iccp()], True),
    "cicp_pq.png": ([cicp(9, 16, 0, 1)], True),
    "srgb_with_iccp.png": ([srgb(), iccp()], True),
    "gama_srgb_chrm_2020.png": ([gama(45455), chrm(REC2020)], True),
}
ORDER = ["sRGB", "gAMA", "cHRM", "iCCP", "cICP"]
# A three-drawing sequence of gAMA 100000: one warning for the sequence, naming drawings 1-3.
SEQUENCE = ["seq/frame_0001.png", "seq/frame_0002.png", "seq/frame_0003.png"]


def main():
    os.makedirs(os.path.join(OUT, "seq"), exist_ok=True)
    expected = {"pixels_rgba": [c for p in PIXELS for c in (*p, 255)], "files": {}}
    for name, (made, warned) in CASES.items():
        with open(os.path.join(OUT, name), "wb") as f:
            f.write(png([c for c, _ in made]))
        said = sorted((d for _, d in made), key=lambda d: ORDER.index(d.split(" ")[0]))
        expected["files"][name] = {"warned": warned, "chunks": said if warned else []}
    for name in SEQUENCE:
        with open(os.path.join(OUT, name), "wb") as f:
            f.write(png([gama(100000)[0]]))
    expected["sequence"] = {
        "files": SEQUENCE,
        "warnings": 1,
        "chunks": [gama(100000)[1]],
        "drawings": "1-3",
    }
    with open(os.path.join(OUT, "expected_colour_metadata.json"), "w", encoding="utf-8", newline="") as f:
        json.dump(expected, f, indent=1)
        f.write("\n")


if __name__ == "__main__":
    main()
