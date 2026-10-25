"""Visual comparison toolkit: an After Effects tutorial still against our render, measured by more
than one number (wave 8, lane w8h, under the owner's instruction of 2026-10-25: "use better visual
analytical techniques to improve at visual analysis of these behaviors").

    python -I tools/visual_compare.py selftest
    python -I tools/visual_compare.py compare --ae AE.png --ours OURS.png [--before OLD.png]
        [--ae-off AE_OFF.png --ours-off OURS_OFF.png] [--src-threshold 0.6] [--scale 3]
        [--exclude x0,y0,x1,y1 ...] [--space display|linear] [--title T] --out SHEET.jpg [--json M.json]

What it gives, at matched geometry:
- register(): scale and offset of one picture in another (coarse scale search with phase
  correlation, then OpenCV's ECC refinement), and warp() to put it there.
- noise_floor(): the recording's noise in levels, from its flat areas (median absolute deviation
  of a high-pass), and the 8x8 block-edge step that marks a YouTube encode.
- kernel_fit(): the shape of a spread (glow, blur, shadow): target = base + sum_k w_k G(sigma_k) * src,
  fitted on the trusted pixels as one Gaussian, an exponential kernel and a non-negative sum of
  Gaussians, with each residual. edge_profile() is the mean by distance from what spreads.
- colour: delta_e2000() maps and percentiles, halo_colour() (hue and chroma of the added light),
  tone_curve() (ours against theirs by quantile match, with a gamma and gain fit).
- structure: ssim(), ms_ssim(), gradient_compare() (Sobel magnitude correlation and edge F1).
- frequency: radial_spectrum() and spectrum_compare() (slope, and the ratio by band).
- motion: flow_compare(), Farneback optical flow between two frames of each.
- sheet(): tutorial / before / after / signed difference heat map / profile plot, under 400 KB.

Permissive dependencies only: numpy, scipy, Pillow (user site packages) and OpenCV (Apache 2.0).
Pictures are floats 0..1, RGB, display (sRGB-encoded) values unless a function says linear.
"""
import io, json, math, site, sys
sys.path.append(site.getusersitepackages())
import numpy as np
from PIL import Image, ImageDraw, ImageFont
from scipy import ndimage, optimize
import cv2

LUMA = np.array([0.2126, 0.7152, 0.0722])


def load(p, box=None):
    im = Image.open(p).convert("RGB")
    if box: im = im.crop(tuple(box))
    return np.asarray(im, np.float64) / 255.0


def to_linear(a): return np.where(a <= 0.04045, a / 12.92, ((np.clip(a, 0, None) + 0.055) / 1.055) ** 2.4)
def to_display(a): a = np.clip(a, 0, None); return np.where(a <= 0.0031308, a * 12.92, 1.055 * a ** (1 / 2.4) - 0.055)
def luma(a): return a[..., :3] @ LUMA
def resize(a, size, interp=cv2.INTER_AREA): return cv2.resize(a.astype(np.float32), size, interpolation=interp).astype(np.float64)


# ---------------------------------------------------------------------------------- registration
def _grad(a):
    g = luma(a) if a.ndim == 3 else a
    g = cv2.GaussianBlur(g.astype(np.float32), (0, 0), 1.0)
    return np.hypot(cv2.Sobel(g, cv2.CV_32F, 1, 0), cv2.Sobel(g, cv2.CV_32F, 0, 1))


def register(ref, mov, scales=None, refine=True):
    """Find M (2x3, scale s and offset) with ref(x) ~ mov(M^-1 x): mov scaled by s, then moved by (dx, dy).
    Returns dict(s, dx, dy, ncc, M). scales: candidate s values (default 0.25..4, log-spaced)."""
    R = _grad(ref)
    best = None
    for s in (scales if scales is not None else np.exp(np.linspace(np.log(0.25), np.log(4), 41))):
        h, w = R.shape
        M = np.array([[s, 0, 0], [0, s, 0]], np.float32)
        G = cv2.warpAffine(_grad(mov), M, (w, h), flags=cv2.INTER_LINEAR)
        win = cv2.createHanningWindow((w, h), cv2.CV_32F)
        (dx, dy), resp = cv2.phaseCorrelate(R.astype(np.float32), G.astype(np.float32), win)
        M[:, 2] = (-dx, -dy)
        G2 = cv2.warpAffine(_grad(mov), M, (w, h), flags=cv2.INTER_LINEAR)
        ncc = float(np.corrcoef(R.ravel(), G2.ravel())[0, 1])
        if best is None or ncc > best[0]: best = (ncc, M)
    ncc, M = best
    if refine:
        try:
            r8 = luma(ref).astype(np.float32); m8 = luma(mov).astype(np.float32)
            crit = (cv2.TERM_CRITERIA_EPS | cv2.TERM_CRITERIA_COUNT, 200, 1e-7)
            # ECC's warp maps the reference grid into the moving picture: the inverse of M
            _, Wi = cv2.findTransformECC(r8, m8, cv2.invertAffineTransform(M).astype(np.float32), cv2.MOTION_AFFINE, crit, None, 5)
            M2 = cv2.invertAffineTransform(Wi).astype(np.float32)
            s2 = math.sqrt(abs(np.linalg.det(M2[:, :2])))
            if abs(s2 / M[0, 0] - 1) < 0.05: M = M2
        except cv2.error:
            pass
    G = cv2.warpAffine(luma(mov).astype(np.float32), M, R.shape[::-1], flags=cv2.INTER_LINEAR)
    ncc = float(np.corrcoef(luma(ref).ravel(), G.ravel())[0, 1])
    s = math.sqrt(abs(np.linalg.det(M[:, :2])))
    return dict(s=s, dx=float(M[0, 2]), dy=float(M[1, 2]), ncc=ncc, M=M)


def warp(mov, M, shape):
    """mov put on the reference grid (h, w) by register()'s M; area-averaged first when it shrinks."""
    s = math.sqrt(abs(np.linalg.det(M[:, :2])))
    m = mov.astype(np.float32); M = np.asarray(M, np.float32).copy()
    if s < 0.9:  # shrink by area first, so the warp does not alias
        f = s * 1.0
        m = cv2.resize(m, (max(1, round(m.shape[1] * f)), max(1, round(m.shape[0] * f))), interpolation=cv2.INTER_AREA)
        M[:, :2] /= f
    return cv2.warpAffine(m, M, (shape[1], shape[0]), flags=cv2.INTER_LINEAR, borderMode=cv2.BORDER_REPLICATE).astype(np.float64)


# ------------------------------------------------------------------------------------ noise floor
def noise_floor(a, valid=None):
    """Noise of a recording in levels of 255: the median absolute deviation (x 1.4826) of a 3x3 high-pass,
    over flat areas (local gradient in its lowest 30%). Also the YouTube block step: mean |jump| across
    8-pixel block edges less that inside blocks, in levels."""
    y = luma(a) * 255
    hp = y - ndimage.uniform_filter(y, 3)
    g = ndimage.uniform_filter(np.abs(np.gradient(ndimage.uniform_filter(y, 5))[0]) + np.abs(np.gradient(ndimage.uniform_filter(y, 5))[1]), 9)
    v = np.ones(y.shape, bool) if valid is None else valid
    flat = v & (g <= np.quantile(g[v], 0.3))
    sigma = float(1.4826 * np.median(np.abs(hp[flat] - np.median(hp[flat])))) / math.sqrt(1 - 1 / 9)
    dx = np.abs(np.diff(y, axis=1))
    edge = dx[:, 7::8].mean() if dx.shape[1] > 8 else 0.0
    block = float(edge - dx.mean())
    return dict(sigma=sigma, block_step=block, floor=2.5 * sigma)


# ------------------------------------------------------------------------------- spread and shape
def gauss(a, s):
    """Gaussian blur of a picture (each channel on its own), zero outside, cut at 4 sigma."""
    if s <= 0: return a.copy()
    return ndimage.gaussian_filter(a, (s, s) + (0,) * (a.ndim - 2), mode="constant", truncate=4.0)


def exp_kernel(a, lam):
    """a convolved with exp(-r / lam), normalised to sum 1 (cut at 8 lam), by FFT."""
    r = int(math.ceil(8 * lam)); yy, xx = np.mgrid[-r:r + 1, -r:r + 1]
    k = np.exp(-np.hypot(xx, yy) / lam); k /= k.sum()
    from scipy.signal import fftconvolve
    if a.ndim == 3: return np.stack([fftconvolve(a[..., c], k, mode="same") for c in range(a.shape[2])], -1)
    return fftconvolve(a, k, mode="same")


def kernel_fit(src, target, valid, base=None, sigmas=None):
    """Fit target = base + spread(src) on valid pixels (2-D arrays, or H x W x C with each channel spread
    on its own and one amplitude for all). Returns dict with
    gauss (sigma, amp, rms), exp (lambda, amp, rms), multi (sigmas, weights, rms, sigma_eff,
    tail_share = weight beyond 2x the strongest sigma), and none_rms (spread = 0)."""
    t = target - (0 if base is None else base)
    tv = t[valid]
    def lin1(img):
        x = img[valid]; d = float(x @ x)
        a = float(x @ tv) / d if d > 0 else 0.0
        return a, float(np.sqrt(np.mean((tv - a * x) ** 2)))
    out = dict(none_rms=float(np.sqrt(np.mean(tv ** 2))))
    def gs(s): return lin1(gauss(src, s))[1]
    r = optimize.minimize_scalar(lambda ls: gs(math.exp(ls)), bounds=(math.log(0.3), math.log(max(src.shape[:2]) / 3)), method="bounded",
                                 options=dict(xatol=1e-4))
    s = math.exp(r.x); a, e = lin1(gauss(src, s)); out["gauss"] = dict(sigma=s, amp=a, rms=e)
    r = optimize.minimize_scalar(lambda ll: lin1(exp_kernel(src, math.exp(ll)))[1], bounds=(math.log(0.3), math.log(max(src.shape[:2]) / 6)),
                                 method="bounded", options=dict(xatol=1e-3))
    lam = math.exp(r.x); a2, e2 = lin1(exp_kernel(src, lam)); out["exp"] = dict(lam=lam, amp=a2, rms=e2)
    if sigmas is None: sigmas = s * np.array([0.125, 0.25, 0.5, 0.707, 1, 1.41, 2, 4, 8])
    sigmas = [float(x) for x in sigmas if 0.3 <= x <= max(src.shape[:2]) / 2]
    X = np.stack([gauss(src, x)[valid] for x in sigmas], 1)
    w, _ = optimize.nnls(X, tv)
    e3 = float(np.sqrt(np.mean((tv - X @ w) ** 2)))
    sw = w.sum()
    seff = float(np.sqrt((w * np.square(sigmas)).sum() / sw)) if sw > 0 else 0.0
    k = int(np.argmax(w)) if sw > 0 else 0
    tail = float(w[np.array(sigmas) > 2 * sigmas[k]].sum() / sw) if sw > 0 else 0.0
    out["multi"] = dict(sigmas=sigmas, weights=w.tolist(), rms=e3, sigma_eff=seff, amp=float(sw), tail_share=tail)
    return out


def edge_profile(values, src_mask, valid, scale=1.0, bins=None):
    """Mean (and count) of values by distance from src_mask's edge, outside it, in comp px: the distance
    from a pixel's centre to the nearest masked pixel's centre less half a pixel, times scale."""
    d = (ndimage.distance_transform_edt(~src_mask) - 0.5) * scale
    if not (valid & ~src_mask).any(): return np.array([]), np.array([]), np.array([], int)  # nothing outside (threshold 0)
    if bins is None: bins = np.unique(np.round(np.geomspace(0.25, max(2.0, d[valid & ~src_mask].max()), 28), 1))
    mids, means, counts = [], [], []
    for a, b in zip(bins[:-1], bins[1:]):
        m = valid & ~src_mask & (d > a) & (d <= b)
        if m.sum() >= 12:
            mids.append(float(d[m].mean())); means.append(float(values[m].mean())); counts.append(int(m.sum()))
    return np.array(mids), np.array(means), np.array(counts)


# ---------------------------------------------------------------------------------------- colour
def rgb_to_lab(a):
    l = to_linear(np.clip(a[..., :3], 0, 1))
    M = np.array([[0.4124564, 0.3575761, 0.1804375], [0.2126729, 0.7151522, 0.0721750], [0.0193339, 0.1191920, 0.9503041]])
    xyz = l @ M.T / np.array([0.95047, 1.0, 1.08883])
    f = np.where(xyz > (6 / 29) ** 3, np.cbrt(xyz), xyz / (3 * (6 / 29) ** 2) + 4 / 29)
    return np.stack([116 * f[..., 1] - 16, 500 * (f[..., 0] - f[..., 1]), 200 * (f[..., 1] - f[..., 2])], -1)


def delta_e2000(a, b):
    """CIEDE2000 between two sRGB pictures (Sharma, Wu and Dalal 2005), per pixel."""
    return delta_e2000_lab(rgb_to_lab(a), rgb_to_lab(b))


def delta_e2000_lab(la, lb):
    L1, a1, b1 = np.moveaxis(la, -1, 0); L2, a2, b2 = np.moveaxis(lb, -1, 0)
    C1, C2 = np.hypot(a1, b1), np.hypot(a2, b2); Cm = (C1 + C2) / 2
    G = 0.5 * (1 - np.sqrt(Cm ** 7 / (Cm ** 7 + 25.0 ** 7)))
    a1p, a2p = (1 + G) * a1, (1 + G) * a2
    C1p, C2p = np.hypot(a1p, b1), np.hypot(a2p, b2)
    h1p = np.degrees(np.arctan2(b1, a1p)) % 360; h2p = np.degrees(np.arctan2(b2, a2p)) % 360
    dL, dC = L2 - L1, C2p - C1p
    dh = h2p - h1p; dh = np.where(dh > 180, dh - 360, np.where(dh < -180, dh + 360, dh)); dh = np.where(C1p * C2p == 0, 0, dh)
    dH = 2 * np.sqrt(C1p * C2p) * np.sin(np.radians(dh / 2))
    Lm, Cmp = (L1 + L2) / 2, (C1p + C2p) / 2
    hs = h1p + h2p
    hm = np.where(C1p * C2p == 0, hs, np.where(np.abs(h1p - h2p) <= 180, hs / 2, np.where(hs < 360, (hs + 360) / 2, (hs - 360) / 2)))
    T = 1 - 0.17 * np.cos(np.radians(hm - 30)) + 0.24 * np.cos(np.radians(2 * hm)) + 0.32 * np.cos(np.radians(3 * hm + 6)) - 0.20 * np.cos(np.radians(4 * hm - 63))
    dth = 30 * np.exp(-(((hm - 275) / 25) ** 2))
    Rc = 2 * np.sqrt(Cmp ** 7 / (Cmp ** 7 + 25.0 ** 7))
    SL = 1 + 0.015 * (Lm - 50) ** 2 / np.sqrt(20 + (Lm - 50) ** 2); SC = 1 + 0.045 * Cmp; SH = 1 + 0.015 * Cmp * T
    RT = -np.sin(np.radians(2 * dth)) * Rc
    return np.sqrt((dL / SL) ** 2 + (dC / SC) ** 2 + (dH / SH) ** 2 + RT * (dC / SC) * (dH / SH))


def percentiles(x, ps=(50, 90, 99)): return {f"p{p}": float(np.percentile(x, p)) for p in ps} if x.size else {}


def halo_colour(added, mask):
    """Hue (degrees, circular mean weighted by chroma) and chroma of the added light (display RGB, may be
    negative-free) over mask, in CIELab of the added light alone."""
    lab = rgb_to_lab(np.clip(added, 0, 1))[mask]
    C = np.hypot(lab[:, 1], lab[:, 2])
    if C.sum() <= 0: return dict(hue=None, chroma=0.0, L=float(lab[:, 0].mean()) if len(lab) else 0.0)
    h = math.degrees(math.atan2((lab[:, 2]).sum(), (lab[:, 1]).sum())) % 360
    return dict(hue=h, chroma=float(C.mean()), L=float(lab[:, 0].mean()))


def tone_curve(ours, theirs, valid, n=17):
    """Per channel: quantiles of ours against the same quantiles of theirs (a monotone tone curve),
    with ours ~ gain x theirs ^ gamma fitted on it, and the largest gap in levels."""
    out = {}
    qs = np.linspace(0.02, 0.98, n)
    for c, nm in enumerate("rgb"):
        o, t = np.quantile(ours[..., c][valid], qs), np.quantile(theirs[..., c][valid], qs)
        ok = (t > 0.02) & (o > 0.02)
        if ok.sum() >= 3:
            g, lg = np.polyfit(np.log(t[ok]), np.log(o[ok]), 1)
        else:
            g, lg = float("nan"), float("nan")
        out[nm] = dict(theirs=t.tolist(), ours=o.tolist(), gamma=float(g), gain=float(math.exp(lg)) if lg == lg else float("nan"),
                       max_gap=float(255 * np.max(np.abs(o - t))))
    return out


# ------------------------------------------------------------------------------------- structure
def ssim(a, b, valid=None, map_out=False):
    """SSIM on luma (Wang et al. 2004: Gaussian window sigma 1.5, K1 0.01, K2 0.03, range 1)."""
    x, y = (luma(a) if a.ndim == 3 else a), (luma(b) if b.ndim == 3 else b)
    f = lambda z: ndimage.gaussian_filter(z, 1.5, truncate=3.5)
    mx, my = f(x), f(y); sxx, syy, sxy = f(x * x) - mx * mx, f(y * y) - my * my, f(x * y) - mx * my
    C1, C2 = 0.01 ** 2, 0.03 ** 2
    cs = (2 * sxy + C2) / (sxx + syy + C2)
    m = (2 * mx * my + C1) / (mx * mx + my * my + C1) * cs
    v = np.ones(x.shape, bool) if valid is None else valid
    return (m, cs) if map_out else float(m[v].mean())


def ms_ssim(a, b, valid=None):
    """Multi-scale SSIM (Wang, Simoncelli and Bovik 2003), five scales and their weights; fewer when small."""
    W = [0.0448, 0.2856, 0.3001, 0.2363, 0.1333]
    x, y = luma(a), luma(b); v = np.ones(x.shape, bool) if valid is None else valid
    vals = []
    for i, w in enumerate(W):
        m, cs = ssim(x, y, map_out=True)
        last = i == len(W) - 1 or min(x.shape) < 32
        vals.append((w, max(1e-6, float((m if last else cs)[v].mean()))))
        if last: break
        x, y = (resize(z, (z.shape[1] // 2, z.shape[0] // 2)) for z in (x, y))
        v = resize(v.astype(float), (v.shape[1] // 2, v.shape[0] // 2)) > 0.5
    ws = np.array([w for w, _ in vals]); ws /= ws.sum()
    return float(np.prod([max(s, 1e-6) ** w for (_, s), w in zip(vals, ws)]))


def gradient_compare(a, b, valid=None):
    """Sobel gradient magnitude: correlation and mean |difference| (levels per px), and Canny edge F1
    (an edge within 1 px counts)."""
    ga, gb = _grad(a).astype(np.float64), _grad(b).astype(np.float64)
    v = np.ones(ga.shape, bool) if valid is None else valid
    corr = float(np.corrcoef(ga[v], gb[v])[0, 1]) if ga[v].std() > 0 and gb[v].std() > 0 else 1.0
    ea = cv2.Canny((np.clip(luma(a), 0, 1) * 255).astype(np.uint8), 40, 100) > 0
    eb = cv2.Canny((np.clip(luma(b), 0, 1) * 255).astype(np.uint8), 40, 100) > 0
    ea &= v; eb &= v
    da, db = ndimage.binary_dilation(ea, iterations=1), ndimage.binary_dilation(eb, iterations=1)
    p = (eb & da).sum() / max(1, eb.sum()); r = (ea & db).sum() / max(1, ea.sum())
    return dict(grad_corr=corr, grad_mad=float(255 * np.abs(ga - gb)[v].mean() / 8), edge_f1=float(2 * p * r / max(1e-9, p + r)) if ea.any() or eb.any() else 1.0)


# ------------------------------------------------------------------------------------- frequency
def radial_spectrum(a, nbins=24):
    """Radially averaged power spectrum of luma (Hann window, mean removed), log-spaced bins in
    cycles per pixel 1/128..1/2. Returns (freqs, power, slope of log power on log frequency)."""
    y = luma(a) if a.ndim == 3 else a
    y = y - y.mean(); h, w = y.shape
    win = np.outer(np.hanning(h), np.hanning(w))
    P = np.abs(np.fft.fftshift(np.fft.fft2(y * win))) ** 2 / (win ** 2).sum()
    fy, fx = np.meshgrid(np.fft.fftshift(np.fft.fftfreq(h)), np.fft.fftshift(np.fft.fftfreq(w)), indexing="ij")
    fr = np.hypot(fx, fy)
    edges = np.geomspace(1 / 128, 0.5, nbins + 1)
    f, p = [], []
    for lo, hi in zip(edges[:-1], edges[1:]):
        m = (fr >= lo) & (fr < hi)
        if m.any(): f.append(math.sqrt(lo * hi)); p.append(float(P[m].mean()))
    f, p = np.array(f), np.array(p)
    ok = p > 0
    slope = float(np.polyfit(np.log(f[ok]), np.log(p[ok]), 1)[0]) if ok.sum() > 2 else float("nan")
    return f, p, slope


def spectrum_compare(ours, theirs):
    f, po, so = radial_spectrum(ours); _, pt, st = radial_spectrum(theirs)
    ratio = np.sqrt(po / np.maximum(pt, 1e-30))  # amplitude ratio per band
    cen = lambda p: float(np.exp((np.log(f) * p).sum() / p.sum()))
    return dict(slope_ours=so, slope_theirs=st, centroid_ours=cen(po), centroid_theirs=cen(pt),
                amp_ratio_bands=dict(zip([round(x, 4) for x in f], [round(float(r), 3) for r in ratio])),
                amp_ratio_median=float(np.median(ratio)))


# ---------------------------------------------------------------------------------------- motion
def flow(a0, a1):
    g = lambda z: (np.clip(luma(z), 0, 1) * 255).astype(np.uint8)
    return cv2.calcOpticalFlowFarneback(g(a0), g(a1), None, 0.5, 4, 21, 5, 7, 1.5, 0)


def flow_compare(ours0, ours1, theirs0, theirs1, valid=None):
    """Farneback flow of each pair: median and p90 magnitude (px), mean direction (degrees), and the
    per-pixel end-point difference between the two flows."""
    fo, ft = flow(ours0, ours1), flow(theirs0, theirs1)
    v = np.ones(fo.shape[:2], bool) if valid is None else valid
    mo, mt = np.hypot(*np.moveaxis(fo, -1, 0)), np.hypot(*np.moveaxis(ft, -1, 0))
    ang = lambda f: math.degrees(math.atan2(f[..., 1][v].mean(), f[..., 0][v].mean()))
    epe = np.hypot(*np.moveaxis(fo - ft, -1, 0))[v]
    return dict(ours_median=float(np.median(mo[v])), ours_p90=float(np.percentile(mo[v], 90)), theirs_median=float(np.median(mt[v])),
                theirs_p90=float(np.percentile(mt[v], 90)), ours_dir=ang(fo), theirs_dir=ang(ft), epe_median=float(np.median(epe)))


# ----------------------------------------------------------------------------------------- sheet
def _font(sz):
    try: return ImageFont.truetype("arial.ttf", sz)
    except OSError: return ImageFont.load_default()


def heat(diff, lim):
    """Signed difference (levels) to RGB: blue below zero, red above, black at zero, full at +-lim."""
    t = np.clip(diff / max(lim, 1e-9), -1, 1)
    r = np.clip(t, 0, 1); b = np.clip(-t, 0, 1)
    return np.stack([r, 0.25 * np.abs(t) ** 2, b], -1)


def plot(curves, w, h, xlabel="distance (comp px)", ylabel="levels", logx=True):
    """curves: list of (label, xs, ys, (r,g,b), dashed). A small line plot drawn with PIL."""
    im = Image.new("RGB", (w, h), (24, 24, 24)); d = ImageDraw.Draw(im); f = _font(12)
    xs = np.concatenate([c[1] for c in curves if len(c[1])]) if any(len(c[1]) for c in curves) else np.array([1.0])
    ys = np.concatenate([c[2] for c in curves if len(c[2])]) if any(len(c[2]) for c in curves) else np.array([0.0])
    x0, x1 = (max(xs.min(), 1e-3), xs.max()) if logx else (xs.min(), xs.max())
    y0, y1 = min(0.0, ys.min()), max(1e-6, ys.max()) * 1.05
    L, R, T, B = 44, w - 8, 8, h - 34
    X = (lambda x: L + (R - L) * (math.log(max(x, x0)) - math.log(x0)) / max(1e-9, math.log(x1) - math.log(x0))) if logx else \
        (lambda x: L + (R - L) * (x - x0) / max(1e-9, x1 - x0))
    Y = lambda y: B - (B - T) * (y - y0) / max(1e-9, y1 - y0)
    d.rectangle([L, T, R, B], outline=(90, 90, 90))
    for k in range(5):
        yv = y0 + (y1 - y0) * k / 4; d.line([L, Y(yv), R, Y(yv)], fill=(50, 50, 50)); d.text((2, Y(yv) - 6), f"{yv:.3g}", fill=(170, 170, 170), font=f)
    for xv in ([1, 2, 5, 10, 20, 50, 100, 200, 500, 1000] if logx else np.linspace(x0, x1, 5)):
        if x0 <= xv <= x1: d.line([X(xv), T, X(xv), B], fill=(50, 50, 50)); d.text((X(xv) - 8, B + 3), f"{xv:g}", fill=(170, 170, 170), font=f)
    if y0 < 0: d.line([L, Y(0), R, Y(0)], fill=(120, 120, 120))
    for i, (lab, cx, cy, col, dash) in enumerate(curves):
        pts = [(X(a), Y(b)) for a, b in zip(cx, cy)]
        for j, (p, q) in enumerate(zip(pts[:-1], pts[1:])):
            if not dash or j % 2 == 0: d.line([p, q], fill=col, width=2)
        d.text((L + 6, T + 4 + 14 * i), lab, fill=col, font=f)
    d.text((L, h - 16), f"{xlabel}; {ylabel}", fill=(170, 170, 170), font=f)
    return im


def sheet(tiles, labels, lines, out, plot_im=None, tile_w=360, max_bytes=400_000):
    """tiles: list of float RGB pictures (or None); one row with the plot at the end, text below. JPEG
    (or PNG when out ends .png) re-encoded smaller until under max_bytes."""
    ims = []
    for t in tiles:
        im = Image.fromarray((np.clip(t, 0, 1) * 255 + 0.5).astype(np.uint8))
        ims.append(im.resize((tile_w, max(1, round(im.height * tile_w / im.width))), Image.LANCZOS))
    th = max(i.height for i in ims)
    if plot_im is not None: ims.append(plot_im.resize((tile_w + 80, th))); labels = list(labels) + ["fall-off profile"]
    f = _font(14)
    W = sum(i.width for i in ims) + 6 * (len(ims) + 1); H = th + 24 + 18 * len(lines) + 12
    S = Image.new("RGB", (W, H), (16, 16, 16)); d = ImageDraw.Draw(S); x = 6
    for im, lab in zip(ims, labels):
        d.text((x, 4), lab, fill=(235, 235, 235), font=f); S.paste(im, (x, 22)); x += im.width + 6
    for i, ln in enumerate(lines): d.text((8, th + 30 + 18 * i), ln, fill=(230, 230, 230), font=f)
    q, scale = 90, 1.0
    while True:
        im = S if scale == 1 else S.resize((round(W * scale), round(H * scale)), Image.LANCZOS)
        buf = io.BytesIO()
        im.save(buf, "PNG" if str(out).lower().endswith(".png") else "JPEG", **({} if str(out).lower().endswith(".png") else dict(quality=q)))
        if buf.tell() <= max_bytes or (q <= 50 and scale < 0.5): break
        if q > 60: q -= 10
        else: scale *= 0.85
    open(out, "wb").write(buf.getvalue())
    return buf.tell()


# ------------------------------------------------------------------------------------ comparison
def compare(ae, ours, valid, before=None, ae_off=None, ours_off=None, src_mask=None, scale=1.0, space="display", noise=None):
    """All metrics for one case at matched geometry (all pictures on the same grid). With the glow-off
    pictures, 'added' light = on - off is profiled by distance from src_mask, in display or linear values."""
    m = dict()
    de = delta_e2000(ae, ours)[valid]; m["dE2000"] = percentiles(de)
    m["ssim"] = ssim(ae, ours, valid); m["ms_ssim"] = ms_ssim(ae, ours, valid)
    m.update(gradient_compare(ae, ours, valid))
    m["tone"] = {k: dict(gamma=v["gamma"], gain=v["gain"], max_gap=v["max_gap"]) for k, v in tone_curve(ours, ae, valid).items()}
    if before is not None:
        m["before_dE2000"] = percentiles(delta_e2000(ae, before)[valid]); m["before_ssim"] = ssim(ae, before, valid)
    if ae_off is not None and src_mask is not None:
        cv_ = (lambda a: luma(a)) if space == "display" else (lambda a: luma(to_linear(a)))
        prof = {}
        for tag, on, off in [("ae", ae, ae_off), ("ours", ours, ours_off)] + ([("before", before, ours_off)] if before is not None else []):
            if on is None or off is None: continue
            add = cv_(on) - cv_(off)
            prof[tag] = edge_profile(add * (255 if space == "display" else 1), src_mask, valid, scale)
            halo = valid & ~src_mask & (ndimage.distance_transform_edt(~src_mask) * scale < 60)
            m[f"halo_{tag}"] = halo_colour(np.clip(on - off, 0, 1), halo)
        m["profiles"] = {k: (v[0].tolist(), v[1].tolist()) for k, v in prof.items()}
    if noise is not None: m["noise"] = noise
    return m


def selftest():
    rng = np.random.default_rng(7)
    H, W = 240, 320
    yy, xx = np.mgrid[0:H, 0:W]
    src = np.zeros((H, W, 3))
    src[60:110, 40:150] = (1.0, 0.2, 0.8)
    src[((xx - 230) ** 2 + (yy - 150) ** 2) < 40 ** 2] = (0.3, 0.6, 1.0)
    src[150:200, 50:60] = (1.0, 1.0, 1.0)
    img = np.clip(src * 0.9 + 0.05 * np.sin(xx / 9.0)[..., None] * np.cos(yy / 13.0)[..., None] + 0.05, 0, 1)
    res = {}
    # 1. the picture against itself: every difference zero, every likeness one
    valid = np.ones((H, W), bool)
    de = delta_e2000(img, img); res["self_dE_max"] = float(de.max())
    res["self_ssim"] = ssim(img, img); res["self_ms_ssim"] = ms_ssim(img, img)
    gc = gradient_compare(img, img); res["self_grad_corr"] = gc["grad_corr"]; res["self_edge_f1"] = gc["edge_f1"]
    tc = tone_curve(img, img, valid); res["self_tone_gap"] = max(v["max_gap"] for v in tc.values())
    sp = spectrum_compare(img, img); res["self_spec_ratio"] = sp["amp_ratio_median"]
    fc = flow_compare(img, img, img, img); res["self_flow"] = fc["ours_p90"]
    rg = register(img, img, scales=[0.9, 1.0, 1.1]); res["self_reg"] = (rg["s"], rg["dx"], rg["dy"])
    # 2. a known blur is recovered: sigma 3.0 single, and 0.6 G(2) + 0.4 G(8)
    s1 = src[..., 0]; mask = s1 > 0
    t1 = gauss(s1, 3.0); f1 = kernel_fit(s1, t1, valid)
    res["blur3_sigma"] = f1["gauss"]["sigma"]; res["blur3_amp"] = f1["gauss"]["amp"]; res["blur3_multi_eff"] = f1["multi"]["sigma_eff"]
    t2 = 0.6 * gauss(s1, 2.0) + 0.4 * gauss(s1, 8.0)
    f2 = kernel_fit(s1, t2, valid, sigmas=[1, 2, 4, 8, 16])
    res["two_scale_weights"] = [round(x, 3) for x in f2["multi"]["weights"]]
    res["two_scale_gauss_rms_over_multi"] = f2["gauss"]["rms"] / max(f2["multi"]["rms"], 1e-12)
    # the same blur through 8-bit rounding and noise of 2 levels: still recovered, and the noise measured
    t3 = np.round(np.clip(gauss(s1, 3.0) + rng.normal(0, 2 / 255, s1.shape), 0, 1) * 255) / 255
    f3 = kernel_fit(s1, t3, valid); res["blur3_noisy_sigma"] = f3["gauss"]["sigma"]
    ok_, enc = cv2.imencode(".jpg", (np.clip(gauss(s1, 3.0), 0, 1) * 255).astype(np.uint8), [cv2.IMWRITE_JPEG_QUALITY, 50])
    tj = cv2.imdecode(enc, cv2.IMREAD_GRAYSCALE) / 255.0
    res["blur3_jpeg50_sigma"] = kernel_fit(s1, tj, valid)["gauss"]["sigma"]
    fe = kernel_fit(s1, exp_kernel(s1, 4.0), valid); res["exp4_lambda"] = fe["exp"]["lam"]
    res["exp4_gauss_rms_over_exp"] = fe["gauss"]["rms"] / max(fe["exp"]["rms"], 1e-12)
    nf = noise_floor(np.repeat((np.full((H, W), 0.4) + rng.normal(0, 2 / 255, (H, W)))[..., None], 3, 2))
    res["noise_sigma_levels(2)"] = nf["sigma"]
    # 3. registration of a half-size copy moved by (13.3, -7.2)
    M = np.array([[0.5, 0, 13.3], [0, 0.5, -7.2]], np.float32)
    small = cv2.warpAffine(img.astype(np.float32), M, (W, H), flags=cv2.INTER_AREA, borderValue=(0, 0, 0)).astype(np.float64)
    rr = register(small, img); res["reg_half"] = (rr["s"], rr["dx"], rr["dy"])
    # 4. a profile: the edge profile of a sigma-3 blur of a half plane falls to half at the edge
    hp = np.zeros((H, W)); hp[:, :160] = 1
    mids, means, _ = edge_profile(gauss(hp, 3.0), hp > 0, valid, 1.0, bins=np.arange(0, 12.5, 0.5))
    from scipy.special import erfc
    res["edge_profile_max_err"] = float(np.max(np.abs(means - 0.5 * erfc((mids) / (3 * math.sqrt(2))))))
    # 5. ΔE2000 against Sharma's published pair 1: Lab (50, 2.6772, -79.7751) vs (50, 0, -82.7485) = 2.0425
    res["sheet_bytes"] = sheet([img, img, img, heat(np.zeros((H, W)), 1)], ["a", "b", "c", "diff"], ["self test"],
                               _tmp("vc_selftest.jpg"), plot([("x", np.array([1, 2, 4]), np.array([3, 2, 1]), (255, 200, 0), False)], 300, 200))
    checks = [("self dE max", res["self_dE_max"], abs(res["self_dE_max"]) < 1e-9),
              ("self SSIM", res["self_ssim"], abs(res["self_ssim"] - 1) < 1e-9),
              ("self MS-SSIM", res["self_ms_ssim"], abs(res["self_ms_ssim"] - 1) < 1e-9),
              ("self gradient corr", res["self_grad_corr"], res["self_grad_corr"] > 0.999999),
              ("self edge F1", res["self_edge_f1"], res["self_edge_f1"] > 0.999999),
              ("self tone gap (levels)", res["self_tone_gap"], res["self_tone_gap"] < 1e-9),
              ("self spectrum ratio", res["self_spec_ratio"], abs(res["self_spec_ratio"] - 1) < 1e-9),
              ("self flow p90 (px)", res["self_flow"], res["self_flow"] < 1e-3),
              ("self registration", res["self_reg"], abs(res["self_reg"][0] - 1) < 1e-3 and abs(res["self_reg"][1]) < 0.05 and abs(res["self_reg"][2]) < 0.05),
              ("blur 3.0 sigma", res["blur3_sigma"], abs(res["blur3_sigma"] - 3.0) < 0.01),
              ("blur 3.0 amplitude", res["blur3_amp"], abs(res["blur3_amp"] - 1.0) < 0.01),
              ("blur 3.0 multi sigma_eff", res["blur3_multi_eff"], abs(res["blur3_multi_eff"] - 3.0) < 0.15),
              ("two-scale weights at 1,2,4,8,16", res["two_scale_weights"], abs(res["two_scale_weights"][1] - 0.6) < 0.02 and abs(res["two_scale_weights"][3] - 0.4) < 0.02),
              ("two-scale: one Gaussian fits worse (rms ratio)", res["two_scale_gauss_rms_over_multi"], res["two_scale_gauss_rms_over_multi"] > 10),
              ("blur 3.0 through 8 bits and 2-level noise", res["blur3_noisy_sigma"], abs(res["blur3_noisy_sigma"] - 3.0) < 0.05),
              ("blur 3.0 through JPEG quality 50", res["blur3_jpeg50_sigma"], abs(res["blur3_jpeg50_sigma"] - 3.0) < 0.1),
              ("exponential kernel lambda 4 recovered", res["exp4_lambda"], abs(res["exp4_lambda"] - 4.0) < 0.05),
              ("exponential: one Gaussian fits worse (rms ratio)", res["exp4_gauss_rms_over_exp"], res["exp4_gauss_rms_over_exp"] > 5),
              ("noise floor of 2-level noise", res["noise_sigma_levels(2)"], abs(res["noise_sigma_levels(2)"] - 2) < 0.25),
              ("registration of half size moved (13.3, -7.2)", res["reg_half"], abs(res["reg_half"][0] - 0.5) < 0.005 and abs(res["reg_half"][1] - 13.3) < 0.3 and abs(res["reg_half"][2] + 7.2) < 0.3),
              ("edge profile vs erfc", res["edge_profile_max_err"], res["edge_profile_max_err"] < 0.02),
              ("sheet under 400 KB", res["sheet_bytes"], res["sheet_bytes"] < 400_000)]
    lab1 = np.array([50, 2.6772, -79.7751]); lab2 = np.array([50, 0, -82.7485])
    de_sharma = float(delta_e2000_lab(lab1, lab2)); checks.append(("CIEDE2000 Sharma pair 1 (2.0425)", de_sharma, abs(de_sharma - 2.0425) < 1e-3))
    ok = True
    for name, val, good in checks:
        ok &= bool(good)
        print(f"{'PASS' if good else 'FAIL'}  {name}: {val if not isinstance(val, float) else round(val, 6)}")
    print("selftest", "passed" if ok else "FAILED", f"({sum(bool(c[2]) for c in checks)} of {len(checks)})")
    return ok


def _tmp(name):
    import tempfile, os
    return os.path.join(tempfile.gettempdir(), name)


def _main():
    import argparse
    ap = argparse.ArgumentParser()
    sub = ap.add_subparsers(dest="cmd", required=True)
    sub.add_parser("selftest")
    c = sub.add_parser("compare")
    for k in ("ae", "ours", "before", "ae-off", "ours-off", "out", "json", "title"): c.add_argument("--" + k)
    c.add_argument("--src-threshold", type=float, default=0.6); c.add_argument("--scale", type=float, default=1.0)
    c.add_argument("--exclude", nargs="*", default=[]); c.add_argument("--space", default="display")
    a = ap.parse_args()
    if a.cmd == "selftest": sys.exit(0 if selftest() else 1)
    ae = load(a.ae); shp = ae.shape[:2]
    def on_grid(p):
        if not p: return None
        x = load(p)
        return x if x.shape[:2] == shp else resize(x, (shp[1], shp[0]))
    ours, before, ae_off, ours_off = on_grid(a.ours), on_grid(a.before), on_grid(a.ae_off), on_grid(a.ours_off)
    valid = np.ones(shp, bool)
    for r in a.exclude:
        x0, y0, x1, y1 = map(int, r.split(",")); valid[y0:y1, x0:x1] = False
    src = (ae_off.max(axis=2) >= a.src_threshold) if ae_off is not None else None
    m = compare(ae, ours, valid, before, ae_off, ours_off, src, a.scale, a.space, noise_floor(ae, valid))
    diff = (luma(ours) - luma(ae)) * 255; lim = max(8.0, float(np.percentile(np.abs(diff[valid]), 99)))
    curves = []
    cols = dict(ae=(255, 255, 255), before=(140, 140, 255), ours=(255, 170, 40))
    for k, (xs, ys) in m.get("profiles", {}).items(): curves.append((k, np.array(xs), np.array(ys), cols[k], False))
    lines = [a.title or "", f"dE2000 p50/p90/p99 {m['dE2000']}", f"SSIM {m['ssim']:.4f} MS-SSIM {m['ms_ssim']:.4f} grad corr {m['grad_corr']:.3f} edge F1 {m['edge_f1']:.3f}",
             f"noise floor {m['noise']['sigma']:.2f} levels; heat map +-{lim:.0f} levels (red: ours brighter)"]
    tiles = [ae] + ([before] if before is not None else []) + [ours, heat(diff, lim)]
    labels = ["After Effects"] + (["before"] if before is not None else []) + ["ours", "ours - AE"]
    if a.out: sheet(tiles, labels, lines, a.out, plot(curves, 360, 240) if curves else None)
    js = json.dumps(m, indent=1, default=float)
    if a.json: open(a.json, "w", encoding="utf-8", newline="").write(js)
    print(js[:3000])


if __name__ == "__main__":
    _main()
