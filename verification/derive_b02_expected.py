"""Independently derive the B-02 expected values for T-09.

Run with: python verification/derive_b02_expected.py

This computes at Python float (IEEE 754 binary64) directly from the published IEC 61966-2-1
sRGB transfer function and from the document 21 formulae. It does not import, call or
observe the Rust implementation under test. Its output is transcribed into
tests/b02_color_alpha.rs as literals.
"""

def srgb_to_linear(c):
    return c / 12.92 if c <= 0.04045 else ((c + 0.055) / 1.055) ** 2.4

def linear_to_srgb(c):
    return 12.92 * c if c <= 0.0031308 else 1.055 * c ** (1 / 2.4) - 0.055

def quantise(c):
    c = min(1.0, max(0.0, c))
    import math
    return int(math.floor(c * 255.0 + 0.5))

print("# sRGB encoded -> linear light")
for name, v in [
    ("black", 0.0),
    ("linear-segment 10/255", 10 / 255),
    ("kink 0.04045", 0.04045),
    ("just above kink", 0.05),
    ("mid 128/255", 128 / 255),
    ("mid 0.5", 0.5),
    ("white", 1.0),
]:
    print("%-24s %.17g -> %.17g" % (name, v, srgb_to_linear(v)))

print()
print("# linear light -> sRGB encoded")
for name, v in [
    ("black", 0.0),
    ("kink 0.0031308", 0.0031308),
    ("18%% grey 0.18", 0.18),
    ("mid 0.5", 0.5),
    ("white", 1.0),
]:
    print("%-24s %.17g -> %.17g" % (name, v, linear_to_srgb(v)))

print()
print("# 8-bit round trip: v -> linear -> sRGB -> quantise")
bad = [v for v in range(256)
       if quantise(linear_to_srgb(srgb_to_linear(v / 255))) != v]
print("codes that do not survive:", bad)

print()
print("# T-09 straight/premultiplied through the full input path")
print("# straight sRGB8 (255,0,0,128) -> linear premultiplied")
a = 128 / 255
r = srgb_to_linear(255 / 255)
print("  alpha           %.17g" % a)
print("  linear straight %.17g" % r)
print("  premultiplied   %.17g" % (r * a))
print("  back to straight %.17g" % ((r * a) / a))

print()
print("# document 21 normal over, premultiplied")
def over(s, d):
    inv = 1 - s[3]
    return [s[i] + d[i] * inv for i in range(4)]
print("  FX-A-001", over([0, 0, 0, 0], [0.2, 0.4, 0.6, 1]))
print("  FX-A-002", over([0.8, 0.1, 0.2, 1], [0.2, 0.4, 0.6, 1]))
print("  FX-A-003", over([0.5, 0, 0, 0.5], [0, 0, 1, 1]))
