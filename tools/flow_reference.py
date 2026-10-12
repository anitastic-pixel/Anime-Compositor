"""The motion estimate between two pictures, worked a second way (D-650, document 21).

This program's own motion estimate (P0-13, D-600: "Blur + retiming only"), read by Pixel Motion
Blur and Timewarp. Given pictures A and B of the same size, linear and premultiplied, it finds
for each pixel p a vector u(p), in pixels, such that A(p) is close to B(p + u(p)). The rule is
dense pyramidal Lucas-Kanade with a ridge term: a fixed count of steps, no search and no choice
made on the data, so the same two pictures always give the same field.

Pixels are indexed (x, y) from 0; "held" means an index outside the picture is moved to its
nearest edge.

1. Two grey channels for each picture: c0 = wr R + wg G + wb B (the premultiplied colour,
   weighted), and c1 = alpha, so a drawing over nothing moves the estimate by its edges. With
   Correct Luminance Changes, B's c0 is multiplied by mean(A's c0) / mean(B's c0) when the
   latter is above 0 (means over every pixel).
2. A pyramid: level 0 is the picture; level l + 1 is ceil(w/2) by ceil(h/2), each pixel the mean
   of level l's (2x, 2y), (2x+1, 2y), (2x, 2y+1) and (2x+1, 2y+1), held. The top level L is the
   largest whole number up to 6 with min(w, h) / 2^L >= 8 (0 when the picture is smaller).
3. Vector Detail VD, 0 to 100, picks the finest level estimated: lf = floor(log2(100 / VD) +
   0.5) held inside 0 to L, and L for VD 0. VD 100 is a vector for every pixel; 20 one every 4.
4. Block Size B gives the window's reach r = max(1, floor(B / 2)); box_r(f) at p is the sum of f
   over the (2r + 1)^2 pixels around p, held, summed across first, then down.
   lambda = (2r + 1)^2 (0.0001 + 0.01 Global Smoothness).
5. From level L down to lf:
   a. u starts at 0 on level L. On a lower level it is the level above's u read bilinearly,
      held, at ((x + 0.5)/2 - 0.5, (y + 0.5)/2 - 0.5), times 2.
   b. A's gradients, for each channel: Ax = (A(x+1, y) - A(x-1, y)) / 2, Ay likewise, held.
      G = box_r of (sum over channels of Ax Ax, Ax Ay, Ay Ay).
   c. Smoothing Iterations times: e = B(x + ux, y + uy) - A(x, y) for each channel, B read
      bilinearly at that place, held; b = box_r of (sum over channels of Ax e, Ay e);
      with a = Gxx + lambda, d = Gyy + lambda, c = Gxy and det = a d - c^2:
      ux -= (d bx - c by) / det, uy -= (a by - c bx) / det.
   d. Local Smoothness s: u += s (box_r(u) / (2r + 1)^2 - u), each part.
6. The field at full size: level lf's u read bilinearly, held, at ((x + 0.5)/2^lf - 0.5, (y +
   0.5)/2^lf - 0.5), times 2^lf.

Bilinear reading "held" at (x, y): x and y are first held inside 0 to w - 1 and 0 to h - 1;
x0 = floor(x), x1 = min(x0 + 1, w - 1), and the four pixels weighted as usual.

Everything is in double precision. This file never runs the build's code path; it is imported by
`tools/pixel_motion_blur_reference.py` and `tools/timewarp_reference.py`.
"""

from math import floor, log2

DEFAULTS = {"vector_detail": 20.0, "global_smoothness": 0.5, "local_smoothness": 0.5,
            "iterations": 4, "block_size": 6.0, "weights": (0.3, 0.6, 0.1),
            "correct_luminance": False}


def grey(p, weights):
    wr, wg, wb = weights
    return ([wr * px[0] + wg * px[1] + wb * px[2] for px in p["px"]],
            [px[3] for px in p["px"]])


def held(i, n):
    return 0 if i < 0 else n - 1 if i >= n else i


def halve(plane, w, h):
    w2, h2 = -(-w // 2), -(-h // 2)
    out = []
    for y in range(h2):
        for x in range(w2):
            s = 0.0
            for dy in (0, 1):
                for dx in (0, 1):
                    s += plane[held(2 * y + dy, h) * w + held(2 * x + dx, w)]
            out.append(s / 4)
    return out, w2, h2


def box(plane, w, h, r):
    across = [0.0] * (w * h)
    for y in range(h):
        for x in range(w):
            s = 0.0
            for i in range(-r, r + 1):
                s += plane[y * w + held(x + i, w)]
            across[y * w + x] = s
    out = [0.0] * (w * h)
    for y in range(h):
        for x in range(w):
            s = 0.0
            for j in range(-r, r + 1):
                s += across[held(y + j, h) * w + x]
            out[y * w + x] = s
    return out


def bilinear_held(plane, w, h, x, y):
    x = min(max(x, 0.0), w - 1.0)
    y = min(max(y, 0.0), h - 1.0)
    x0, y0 = floor(x), floor(y)
    x1, y1 = min(x0 + 1, w - 1), min(y0 + 1, h - 1)
    ux, uy = x - x0, y - y0
    top = plane[y0 * w + x0] * (1 - ux) + plane[y0 * w + x1] * ux
    bottom = plane[y1 * w + x0] * (1 - ux) + plane[y1 * w + x1] * ux
    return top * (1 - uy) + bottom * uy


def top_level(w, h):
    level = 0
    while level < 6 and min(w, h) / 2 ** (level + 1) >= 8:
        level += 1
    return level


def finest(vector_detail, top):
    if vector_detail <= 0:
        return top
    return min(max(floor(log2(100 / vector_detail) + 0.5), 0), top)


def enlarge(field, w, h, w2, h2, f):
    """`field` (w by h) read at (x + 0.5)/f - 0.5 for each pixel of a w2 by h2 picture, times f."""
    ux, uy = field
    out_x, out_y = [], []
    for y in range(h2):
        for x in range(w2):
            sx, sy = (x + 0.5) / f - 0.5, (y + 0.5) / f - 0.5
            out_x.append(f * bilinear_held(ux, w, h, sx, sy))
            out_y.append(f * bilinear_held(uy, w, h, sx, sy))
    return out_x, out_y


def estimate(a, b, settings=None):
    """The field from picture `a` to picture `b`: two lists, across and down, w by h."""
    s = dict(DEFAULTS, **(settings or {}))
    w, h = a["w"], a["h"]
    ca, cb = grey(a, s["weights"]), grey(b, s["weights"])
    if s["correct_luminance"]:
        ma, mb = sum(ca[0]) / len(ca[0]), sum(cb[0]) / len(cb[0])
        if mb > 0:
            cb = ([v * (ma / mb) for v in cb[0]], cb[1])
    top = top_level(w, h)
    lf = finest(s["vector_detail"], top)
    levels = [(ca, cb, w, h)]
    for _ in range(top):
        (pa, pb, lw, lh) = levels[-1]
        na = [halve(c, lw, lh)[0] for c in pa]
        nb = [halve(c, lw, lh)[0] for c in pb]
        levels.append((na, nb, -(-lw // 2), -(-lh // 2)))
    r = max(1, floor(s["block_size"] / 2))
    area = (2 * r + 1) ** 2
    ridge = area * (0.0001 + 0.01 * s["global_smoothness"])
    field = None
    for level in range(top, lf - 1, -1):
        pa, pb, lw, lh = levels[level]
        n = lw * lh
        if field is None:
            ux, uy = [0.0] * n, [0.0] * n
        else:
            fw, fh = levels[level + 1][2], levels[level + 1][3]
            ux, uy = enlarge(field, fw, fh, lw, lh, 2)
        gx = [[(c[y * lw + held(x + 1, lw)] - c[y * lw + held(x - 1, lw)]) / 2
               for y in range(lh) for x in range(lw)] for c in pa]
        gy = [[(c[held(y + 1, lh) * lw + x] - c[held(y - 1, lh) * lw + x]) / 2
               for y in range(lh) for x in range(lw)] for c in pa]
        gxx = box([sum(g[i] * g[i] for g in gx) for i in range(n)], lw, lh, r)
        gxy = box([sum(gx[c][i] * gy[c][i] for c in range(2)) for i in range(n)], lw, lh, r)
        gyy = box([sum(g[i] * g[i] for g in gy) for i in range(n)], lw, lh, r)
        for _ in range(int(s["iterations"])):
            e = [[bilinear_held(pb[c], lw, lh, (i % lw) + ux[i], (i // lw) + uy[i]) - pa[c][i]
                  for i in range(n)] for c in range(2)]
            bx = box([sum(gx[c][i] * e[c][i] for c in range(2)) for i in range(n)], lw, lh, r)
            by = box([sum(gy[c][i] * e[c][i] for c in range(2)) for i in range(n)], lw, lh, r)
            for i in range(n):
                aa, dd, cc = gxx[i] + ridge, gyy[i] + ridge, gxy[i]
                det = aa * dd - cc * cc
                ux[i] -= (dd * bx[i] - cc * by[i]) / det
                uy[i] -= (aa * by[i] - cc * bx[i]) / det
        smooth = s["local_smoothness"]
        mx, my = box(ux, lw, lh, r), box(uy, lw, lh, r)
        ux = [u + smooth * (m / area - u) for u, m in zip(ux, mx)]
        uy = [u + smooth * (m / area - u) for u, m in zip(uy, my)]
        field = (ux, uy)
    if lf == 0:
        return field
    fw, fh = levels[lf][2], levels[lf][3]
    return enlarge(field, fw, fh, w, h, 2 ** lf)


if __name__ == "__main__":
    # A soft square moving 2 right and 1 down is found moving so at its middle.
    def square(cx, cy):
        px = []
        for y in range(32):
            for x in range(32):
                d = max(abs(x + 0.5 - cx), abs(y + 0.5 - cy))
                c = min(max((7 - d) / 3, 0.0), 1.0)
                px.append([0.8 * c, 0.4 * c, 0.1 * c, c])
        return {"w": 32, "h": 32, "px": px}
    u = estimate(square(14, 15), square(16, 16), {"vector_detail": 100})
    i = 15 * 32 + 14
    print(f"at the middle: {u[0][i]:.3f}, {u[1][i]:.3f}")
    assert abs(u[0][i] - 2) < 0.3 and abs(u[1][i] - 1) < 0.3
    same = estimate(square(14, 15), square(14, 15))
    assert all(v == 0.0 for v in same[0] + same[1])
    print("checks passed")
