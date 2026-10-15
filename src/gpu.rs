//! B-44, D-100 (a): the viewer's picture drawn on the graphics card.
//!
//! This is `render::render_tile` and `WorkingBuffer::to_srgb8_straight` again, as two compute
//! shaders: one pass per layer blends it onto a running sum of the frame, and one last pass
//! encodes the sum to eight-bit straight sRGB, so only the finished picture comes back. It is
//! written fresh from those two functions; nothing is taken from `spikes/p07_gpu`.
//!
//! It is for the viewer only. Exports, fixtures and Full-quality checking stay on the CPU, which
//! remains the authority (ADR-006 as amended by D-100). `tests/b44_gpu_preview.rs` holds the
//! card's picture to within 1 level of 255 of the CPU's.
//!
//! Two rules from P-07:
//! - Every inverse transform is made on the CPU in f64. The card gets the source position of the
//!   first pixel it draws and the step per pixel, so it never adds a huge offset in f32.
//! - Drawings stay on the card between frames, and only the eight-bit picture comes back. The
//!   store keeps a `Weak` to each drawing it holds: while it does, no other drawing can be given
//!   that drawing's address, so a drawing is never mistaken for a different one.
//!
//! B-44 measured the card slower than the CPU, because nearly all its time went on sending
//! drawings. B-44b makes that trip smaller and rarer:
//! - A drawing goes as sixteen-bit floats, half the CPU's bytes, converted on every thread
//!   straight into the buffer the card copies from.
//! - A drawing the CPU's cache holds is also known by that cache's name for it, so the card keeps
//!   it after the CPU lets go, and the same file read again is not sent again.
//! - The card may hold half its own memory, as Windows reports it, rather than D-40's gibibyte.
//!
//! B-45 (D-102) takes away the last trip: the finished picture does not come back at all. The
//! card paints it into the window itself, under the page, which is made see-through only where
//! the picture is. The page says where that is, as a list of [`Paint`]s: the colours of the
//! panels the picture sits in, the checkerboard, and the picture, in the order the page would
//! have painted them. The card paints exactly those, so the window looks as it did.

use std::cell::{Cell, RefCell};
use std::sync::{Arc, Weak};

use half::slice::HalfFloatSliceExt as _;
use rayon::prelude::*;
use wgpu::util::DeviceExt as _;

use crate::cache::{CelCache, Name};
use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::model::BlendMode;
use crate::perf::{self, Stage};
use crate::render::{bounds, Bloom, Directional, FramePlan, Fx, Gaussian, Glow, OnCard, Radial};
use crate::WorkingBuffer;

/// What the card may hold in drawings when Windows cannot say how much memory it has: D-40's
/// gibibyte, the CPU's own budget.
pub const DEFAULT_BUDGET_BYTES: usize = 1024 * 1024 * 1024;

/// A drawing on the card costs eight bytes a pixel: four sixteen-bit floats. One a Bloom starts
/// from costs sixteen, four 32-bit floats as the CPU holds it (B-47): the bright test compares
/// each pixel with the threshold, and half precision moves pixels across it.
const BYTES_PER_PIXEL: usize = 8;

/// The card's own memory in bytes, found by its PCI vendor and device numbers, which Direct3D 12
/// and Vulkan report alike.
#[cfg(windows)]
fn card_memory(vendor: u32, device: u32) -> Option<u64> {
    use windows::Win32::Graphics::Dxgi::{CreateDXGIFactory1, IDXGIFactory1};
    // SAFETY: plain COM calls with no pointers passed in; each result is checked.
    let factory: IDXGIFactory1 = unsafe { CreateDXGIFactory1() }.ok()?;
    (0..)
        .map_while(|i| unsafe { factory.EnumAdapters1(i) }.ok())
        .filter_map(|adapter| unsafe { adapter.GetDesc1() }.ok())
        .find(|d| d.VendorId == vendor && d.DeviceId == device)
        .map(|d| d.DedicatedVideoMemory as u64)
}

#[cfg(not(windows))]
fn card_memory(_: u32, _: u32) -> Option<u64> {
    None
}

const SHADER: &str = r#"
struct Layer {
    s0: vec2<f32>,
    sx: vec2<f32>,
    sy: vec2<f32>,
    m0: vec2<f32>,
    mx: vec2<f32>,
    my: vec2<f32>,
    origin: vec2<u32>,
    size: vec2<u32>,
    width: u32,
    blend: u32,
    matte: u32,
    opacity: f32,
}

@group(0) @binding(0) var<uniform> L: Layer;
@group(0) @binding(1) var source: texture_2d<f32>;
@group(0) @binding(2) var matte: texture_2d<f32>;
@group(0) @binding(3) var<storage, read_write> sum: array<vec4<f32>>;

// render::sample_bilinear: pixel centres at +0.5, and a neighbour outside the drawing adds
// nothing, which is transparent black.
fn bilinear(t: texture_2d<f32>, p: vec2<f32>) -> vec4<f32> {
    let size = vec2<f32>(textureDimensions(t));
    let f = p - vec2(0.5);
    // One pixel or more outside, every neighbour is outside. Tested before the conversion to
    // integers, which a huge coordinate would overflow.
    if !(all(f > vec2(-1.0)) && all(f < size)) {
        return vec4(0.0);
    }
    let base = floor(f);
    let u = f - base;
    let b = vec2<i32>(base);
    let n = vec2<i32>(size);
    var out = vec4(0.0);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0 - u.y, u.y, j == 1);
        let y = b.y + j;
        if wy == 0.0 || y < 0 || y >= n.y {
            continue;
        }
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0 - u.x, u.x, i == 1);
            let x = b.x + i;
            if wx == 0.0 || x < 0 || x >= n.x {
                continue;
            }
            out += textureLoad(t, vec2(x, y), 0) * (wx * wy);
        }
    }
    return out;
}

fn straight(p: vec4<f32>) -> vec3<f32> {
    if p.w == 0.0 {
        return vec3(0.0);
    }
    return p.xyz / p.w;
}

// composite::blend_pixel.
fn blend(s: vec4<f32>, d: vec4<f32>) -> vec4<f32> {
    if L.blend == 0u {
        return s + d * (1.0 - s.w);
    }
    let cs = straight(s);
    let cd = straight(d);
    var b: vec3<f32>;
    switch L.blend {
        case 1u: { b = cs * cd; }
        case 2u: { b = cs + cd - cs * cd; }
        default: { b = min(cs + cd, vec3(1.0)); }
    }
    let rgb = (1.0 - s.w) * d.xyz + (1.0 - d.w) * s.xyz + s.w * d.w * b;
    return vec4(rgb, s.w + d.w - s.w * d.w);
}

@compute @workgroup_size(16, 16)
fn layer(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= L.size.x || id.y >= L.size.y {
        return;
    }
    let d = vec2<f32>(id.xy);
    var src = bilinear(source, L.s0 + L.sx * d.x + L.sy * d.y) * L.opacity;
    if L.matte == 1u {
        src *= bilinear(matte, L.m0 + L.mx * d.x + L.my * d.y).w;
    }
    let at = (L.origin.y + id.y) * L.width + L.origin.x + id.x;
    sum[at] = blend(src, sum[at]);
}

// B-156b (D-226), render::average: one moment of a motion-blurred layer added to the sum, at
// `L.opacity`, one over the number of moments.
@compute @workgroup_size(16, 16)
fn moment(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= L.size.x || id.y >= L.size.y {
        return;
    }
    let d = vec2<f32>(id.xy);
    let at = (L.origin.y + id.y) * L.width + L.origin.x + id.x;
    sum[at] = sum[at] + bilinear(source, L.s0 + L.sx * d.x + L.sy * d.y) * L.opacity;
}

struct Size {
    width: u32,
    height: u32,
    pad0: u32,
    pad1: u32,
}

@group(0) @binding(0) var<uniform> S: Size;
@group(0) @binding(1) var<storage, read> frame: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> bytes: array<u32>;

// color::linear_to_srgb and color::quantise_u8.
fn srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        return 12.92 * c;
    }
    return 1.055 * pow(c, 1.0 / 2.4) - 0.055;
}

fn level(c: f32) -> u32 {
    return u32(floor(clamp(c, 0.0, 1.0) * 255.0 + 0.5));
}

@compute @workgroup_size(16, 16)
fn encode(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= S.width || id.y >= S.height {
        return;
    }
    let at = id.y * S.width + id.x;
    let p = frame[at];
    let c = straight(p);
    bytes[at] = level(srgb(c.x)) | (level(srgb(c.y)) << 8u) | (level(srgb(c.z)) << 16u)
        | (level(p.w) << 24u);
}

struct Radial {
    center: vec2<f32>,
    amount: f32,
    spin: u32,
    held: u32,
    // D-361/D-362: blurs::Weigh, 0 even, 1 fading, 2 brightest, 3 darkest; samples a pixel.
    weigh: u32,
    density: f32,
}

@group(0) @binding(0) var<uniform> R: Radial;
@group(0) @binding(1) var still: texture_2d<f32>;
@group(0) @binding(2) var moved: texture_storage_2d<rgba32float, write>;
@group(0) @binding(3) var<storage, read> turns: array<vec2<f32>>;

// B-46: blurs::radial_blur, one pixel a thread. `turns` is blurs::radial_turns from 2 samples
// up, worked on the CPU in f64: n samples start at n(n-1)/2 - 1.
@compute @workgroup_size(16, 16)
fn radial(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(still);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let d = vec2<f32>(id.xy) + vec2(0.5) - R.center;
    let r = length(d);
    var path = r * R.amount / 100.0;
    if R.spin == 1u {
        path = r * R.amount * 3.141592653589793 / 180.0;
    }
    let n = min(u32(ceil(path * R.density)) + 1u, 256u);
    if n == 1u {
        textureStore(moved, id.xy, textureLoad(still, id.xy, 0));
        return;
    }
    let start = n * (n - 1u) / 2u - 1u;
    var total = vec4(0.0);
    var weight = 0.0;
    var first = vec4(0.0);
    for (var k = 0u; k < n; k++) {
        let t = turns[start + k];
        var p = R.center + t.x * d;
        if R.spin == 1u {
            p = R.center + vec2(d.x * t.y - d.y * t.x, d.x * t.x + d.y * t.y);
        }
        // D-109: held inside the drawing's pixel centres.
        if R.held == 1u {
            p = clamp(p, vec2(0.5), vec2<f32>(size) - vec2(0.5));
        }
        let s = bilinear(still, p);
        let f = f32(n - k);
        if R.weigh == 0u {
            total += s;
        } else if R.weigh == 1u {
            total += f * s;
        } else if k == 0u {
            first = s;
            total = s;
        } else if R.weigh == 2u {
            total = max(total, first + f / f32(n) * (s - first));
        } else {
            total = min(total, first + f / f32(n) * (s - first));
        }
        weight += f;
    }
    if R.weigh == 0u {
        total /= f32(n);
    } else if R.weigh == 1u {
        total /= weight;
    }
    textureStore(moved, id.xy, total);
}

@group(0) @binding(2) var<storage, read> lone: array<vec4<f32>>;
@group(0) @binding(3) var drawn: texture_storage_2d<rgba32float, write>;
@group(0) @binding(4) var light: texture_storage_2d<rgba32float, write>;

// B-76, D-132: a layer with a Light Wrap, laid alone in `lone`, as a texture, and its light: the
// colours of the frame beneath with the layer's covering, which blur alike.
@compute @workgroup_size(16, 16)
fn unpack(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= S.width || id.y >= S.height {
        return;
    }
    let at = id.y * S.width + id.x;
    let l = lone[at];
    textureStore(drawn, id.xy, l);
    textureStore(light, id.xy, vec4(frame[at].xyz, l.w));
}

// B-156: the frame laid so far, as a texture an adjustment layer's effects run on.
@compute @workgroup_size(16, 16)
fn take(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= S.width || id.y >= S.height {
        return;
    }
    textureStore(drawn, id.xy, frame[id.y * S.width + id.x]);
}
"#;

/// B-47: `bloom::bloom` as passes, one after another: the light, the halo's four blurs, the
/// streaks, and the bloom laid on the drawing. `halo` sums in the CPU's order. The streaks keep
/// their running totals in double precision, as the CPU does, so this module needs a card that
/// offers it (`SHADER_F64`); without one, a frame with a Bloom left for the card goes to the CPU.
const BLOOM_SHADER: &str = r#"
struct Params {
    s: f64,
    a: f64,
    b: f64,
    k0: i32,
    n: u32,
    count: u32,
    g: i32,
    down: u32,
    level: u32,
    weight: f32,
    width: u32,
    height: u32,
    axis: u32,
    reach: u32,
    ends: u32,
    tolerance: u32,
    tinted: u32,
    screen: u32,
    held: u32,
    cover: f32,
    pad: u32,
}

@group(0) @binding(0) var<uniform> P: Params;
@group(0) @binding(1) var input: texture_2d<f32>;
@group(0) @binding(2) var output: texture_storage_2d<rgba32float, write>;
@group(0) @binding(3) var<storage, read_write> halo: array<vec4<f32>>;
@group(0) @binding(4) var<storage, read> weights: array<f32>;

// color::linear_to_srgb and color::quantise_u8.
fn srgb(c: f32) -> f32 {
    if c <= 0.0031308 {
        return 12.92 * c;
    }
    return 1.055 * pow(c, 1.0 / 2.4) - 0.055;
}

fn level(c: f32) -> u32 {
    return u32(floor(clamp(c, 0.0, 1.0) * 255.0 + 0.5));
}

// The pixel at `p` of `input`, or transparent black outside it; (D-109) with `held`, the
// nearest pixel of `input`, which for the straight mix of two is the mix at the held point.
fn at(p: vec2<i32>) -> vec4<f32> {
    let size = vec2<i32>(textureDimensions(input));
    if P.held == 1u {
        return textureLoad(input, clamp(p, vec2(0), size - vec2(1)), 0);
    }
    if any(p < vec2(0)) || any(p >= size) {
        return vec4(0.0);
    }
    return textureLoad(input, p, 0);
}

// bloom::bright: the pixel where its largest 8-bit channel reaches `level`, else nothing.
// B-51, glow::glows: or where each 8-bit channel is within `tolerance` of one of the `count`
// colours in `weights`, and with `tinted` the light is the tint after them at the pixel's
// covering.
@compute @workgroup_size(16, 16)
fn bright(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    var light = vec4(0.0);
    if p.w > 0.0 {
        let c = p.xyz / p.w;
        let q = vec3<i32>(vec3(level(srgb(c.x)), level(srgb(c.y)), level(srgb(c.z))));
        var lit = max(q.x, max(q.y, q.z)) >= i32(P.level);
        for (var t = 0u; t < P.count; t++) {
            let v = vec3<i32>(vec3(weights[3u * t], weights[3u * t + 1u], weights[3u * t + 2u]));
            lit = lit || all(abs(q - v) <= vec3(i32(P.tolerance)));
        }
        if lit {
            let n = 3u * P.count;
            let tint = vec4(vec3(weights[n], weights[n + 1u], weights[n + 2u]) * p.w, p.w);
            light = select(p, tint, P.tinted == 1u);
        }
    }
    textureStore(output, id.xy, light);
}

// effects::convolve: `output` is `input` grown by `count` on both ends of the axis, and each
// pixel adds its taps in the same order. D-109, effects::held_blur: with `held`, the same size.
@compute @workgroup_size(16, 16)
fn gauss(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let r = i32(P.count);
    let step = select(vec2(1, 0), vec2(0, 1), P.axis == 1u);
    let first = vec2<i32>(id.xy) - step * select(2 * r, r, P.held == 1u);
    var o = vec4(0.0);
    for (var k = 0; k <= 2 * r; k++) {
        o += at(first + step * k) * weights[k];
    }
    textureStore(output, id.xy, o);
}

// `input` added to the halo at `weight`, its corner `g` pixels in from the halo's.
@compute @workgroup_size(16, 16)
fn add(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= P.width || id.y >= P.height {
        return;
    }
    let i = id.y * P.width + id.x;
    halo[i] += at(vec2<i32>(id.xy) - vec2(P.g)) * P.weight;
}

// B-164 (D-235): `input` shrunk `n` times, transparent outside it, the output's corner `g`
// pixels out from the input's: each pixel the tent-weighted mean of the 2n by 2n pixels round
// its n by n block. A tent rather than the block's plain mean, whose sharp edges let a sharp
// drawing through as a faint pattern the blur does not remove.
@compute @workgroup_size(16, 16)
fn shrink(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let n = i32(P.n);
    let corner = (vec2<i32>(id.xy) - vec2(P.g)) * n - vec2(n / 2);
    var o = vec4(0.0);
    for (var y = 0; y < 2 * n; y++) {
        let wy = f32(n) - abs(f32(y) + 0.5 - f32(n));
        for (var x = 0; x < 2 * n; x++) {
            let wx = f32(n) - abs(f32(x) + 0.5 - f32(n));
            o += at(corner + vec2(x, y)) * (wx * wy);
        }
    }
    textureStore(output, id.xy, o / f32(n * n * n * n));
}

// B-164 (D-235): `input`, a picture shrunk `n` times, enlarged bilinearly from pixel centres,
// transparent outside it: the output's pixel `g` in from its corner is the picture's first,
// which sits at (x + 0.5) / n - 0.5 in the small one, `count` pixels in from its texture's corner.
@compute @workgroup_size(16, 16)
fn enlarge(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let u = (vec2<f32>(vec2<i32>(id.xy) - vec2(P.g)) + 0.5) / f32(P.n) - 0.5 + f32(P.count);
    let base = floor(u);
    let t = u - base;
    let b = vec2<i32>(base);
    let top = at(b) * (1.0 - t.x) + at(b + vec2(1, 0)) * t.x;
    let bottom = at(b + vec2(0, 1)) * (1.0 - t.x) + at(b + vec2(1, 1)) * t.x;
    textureStore(output, id.xy, top * (1.0 - t.y) + bottom * t.y);
}

// render::sample_bilinear at column c's centre, height y on line space; x and y exchanged when
// the lines run mostly down.
fn tap(c: i32, y: f32) -> vec4<f32> {
    let f = y - 0.5;
    let base = floor(f);
    let u = f - base;
    let r = i32(base);
    var o = vec4(0.0);
    if 1.0 - u != 0.0 {
        o += at(select(vec2(c, r), vec2(r, c), P.down == 1u)) * (1.0 - u);
    }
    if u != 0.0 {
        o += at(select(vec2(c, r + 1), vec2(r + 1, c), P.down == 1u)) * u;
    }
    return o;
}

fn nz(v: vec4<f32>) -> u32 {
    return select(0u, 1u, any(v != vec4(0.0)));
}

// Line space's height of column j on line k, `k0 + line + 0.5 + j s`, from double precision.
fn height(k: f64, j: i32) -> f32 {
    return f32(k + f64(j) * P.s);
}

// blurs::by_lines for one line a thread: the tent a - b|j| over |j| <= n, from running totals,
// then (B-49) the `ends` taps, pairs of column and weight in `weights`. `lp` and `rp` are the
// samples left and right of the point, `lw` and `rw` the same times their distance, and `seen`
// how many within `reach` are not zero. B-157 (G6): the totals in single precision.
@compute @workgroup_size(32)
fn streak(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= P.count {
        return;
    }
    let k = f64(P.k0 + i32(id.x)) + 0.5lf;
    let n = i32(P.n);
    let m = i32(P.reach);
    let x0 = -P.g;
    let a = f32(P.a);
    let b = f32(P.b);
    var lp = vec4<f32>(0.0);
    var rp = lp;
    var lw = lp;
    var rw = lp;
    var seen = 0u;
    for (var j = 1; j <= m; j++) {
        let vr = tap(x0 + j, height(k, x0 + j));
        let vl = tap(x0 - j, height(k, x0 - j));
        if j <= n {
            rp += vr;
            rw += f32(j) * vr;
            lp += vl;
            lw += f32(j) * vl;
        }
        seen += nz(vr) + nz(vl);
    }
    var vc = tap(x0, height(k, x0));
    seen += nz(vc);
    for (var x = 0u; x < P.width; x++) {
        let c = i32(x) + x0;
        var r = a * (lp + vc + rp) - b * (rw + lw);
        for (var e = 0u; e < P.ends; e++) {
            let j = c + i32(weights[2u * e]);
            r += weights[2u * e + 1u] * tap(j, height(k, j));
        }
        textureStore(output, vec2(x, id.x), select(r, vec4(0.0), seen == 0u));
        let vn = tap(c + n + 1, height(k, c + n + 1));
        let vo = tap(c - n, height(k, c - n));
        let v1 = tap(c + 1, height(k, c + 1));
        rw = rw - rp + f32(n) * vn;
        rp = rp - v1 + vn;
        lw = lw + lp + vc - f32(n + 1) * vo;
        lp = lp + vc - vo;
        if m == n {
            seen = seen + nz(vn) - nz(vo);
        } else {
            seen = seen + nz(tap(c + m + 1, height(k, c + m + 1))) - nz(tap(c - m, height(k, c - m)));
        }
        vc = v1;
    }
}

// The two lines round each pixel of the halo, mixed straight, added at `weight`. `input` holds
// the lines, one a row.
@compute @workgroup_size(16, 16)
fn mix(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= P.width || id.y >= P.height {
        return;
    }
    let o = select(vec2<i32>(id.xy), vec2<i32>(id.yx), P.down == 1u);
    let d = f64(o.y - P.g) - f64(o.x - P.g) * P.s;
    let kf = floor(d);
    let f = f32(d - kf);
    let line = i32(kf) - P.k0;
    let lo = textureLoad(input, vec2(o.x, line), 0);
    let hi = textureLoad(input, vec2(o.x, line + 1), 0);
    halo[id.y * P.width + id.x] += ((1.0 - f) * lo + f * hi) * P.weight;
}

// B-49: the same two lines mixed, written as the picture: a Directional Blur's last step.
@compute @workgroup_size(16, 16)
fn lay(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= P.width || id.y >= P.height {
        return;
    }
    let o = select(vec2<i32>(id.xy), vec2<i32>(id.yx), P.down == 1u);
    let d = f64(o.y - P.g) - f64(o.x - P.g) * P.s;
    let kf = floor(d);
    let f = f32(d - kf);
    let line = i32(kf) - P.k0;
    let lo = textureLoad(input, vec2(o.x, line), 0);
    let hi = textureLoad(input, vec2(o.x, line + 1), 0);
    textureStore(output, id.xy, (1.0 - f) * lo + f * hi);
}

// bloom::bloom's last step: the halo times the intensity, on the drawing, grown by `g`.
// B-51: with `screen`, glow::glow's screen instead.
@compute @workgroup_size(16, 16)
fn combine(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = at(vec2<i32>(id.xy) - vec2(P.g));
    let h = halo[id.y * size.x + id.x];
    // D-331: held, the covering is laid on times `cover`, not the colour's weight.
    let k = vec4(P.weight, P.weight, P.weight, select(P.weight, P.cover, P.held == 1u));
    if P.screen == 1u {
        let v = clamp(h * k, vec4(0.0), vec4(1.0));
        textureStore(output, id.xy, p + v - p * v);
        return;
    }
    textureStore(output, id.xy, vec4(p.xyz + h.xyz * k.xyz, min(p.w + h.w * k.w, 1.0)));
}
"#;

/// B-47: [`BLOOM_SHADER`]'s numbers, laid out as its `Params`; each pass reads what it needs.
#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct Params {
    s: f64,
    a: f64,
    b: f64,
    k0: i32,
    n: u32,
    count: u32,
    g: i32,
    down: u32,
    level: u32,
    weight: f32,
    width: u32,
    height: u32,
    axis: u32,
    reach: u32,
    ends: u32,
    tolerance: u32,
    tinted: u32,
    screen: u32,
    held: u32,
    cover: f32,
    pad: u32,
}

/// B-164, D-235's safety rule: the block a blur of `sigma` may be worked small by in the viewer,
/// the largest of 8, 4 and 2 whose small sigma is at least 6, else 1, the exact blur.
fn shrink_factor(sigma: f64) -> usize {
    let mut f = 8;
    while f > 1 && small_sigma(sigma, f) < 6.0 {
        f /= 2;
    }
    f
}

/// D-235: the sigma to blur the picture shrunk `f` times by, less the blur that shrinking and
/// enlarging add. B-164 (D-221): the shrink's tent adds (2f^2 + 1) / 12 where D-235's block
/// mean added (f^2 - 1) / 12; enlarging bilinearly adds f^2 / 6.
fn small_sigma(sigma: f64, f: usize) -> f64 {
    let f = f as f64;
    (sigma * sigma - (2.0 * f * f + 1.0) / 12.0 - f * f / 6.0)
        .max(0.0)
        .sqrt()
        / f
}

/// B-164 (D-221): the small blur's weights. Document 21's kernel stops at its radius r, so the
/// small one stops where r + 1/2 falls among the small pixels, its last weights cut by the part
/// of their pixel inside it, and is normalised as the exact one is; a small kernel of its own
/// length leaves the exact one's edge up to f pixels out, which a dark picture shows.
fn small_taps(sigma: f64, f: usize, long: bool) -> Vec<f32> {
    let s = small_sigma(sigma, f);
    let edge = (crate::effects::reach_radius(sigma, long) as f64 + 0.5) / f as f64;
    let rs = (edge - 0.5).ceil() as i64;
    let cut = edge - (rs as f64 - 0.5);
    let w: Vec<f64> = (-rs..=rs)
        .map(|k| (-((k * k) as f64) / (2.0 * s * s)).exp() * if k.abs() == rs { cut } else { 1.0 })
        .collect();
    let total: f64 = w.iter().sum();
    w.iter().map(|v| (v / total) as f32).collect()
}

/// A compute pass and the bindings it takes.
type Pass = (wgpu::ComputePipeline, wgpu::BindGroupLayout);

/// B-47: [`BLOOM_SHADER`]'s passes. B-49's Directional Blur uses `streak` and `lay`.
struct BloomPasses {
    bright: Pass,
    gauss: Pass,
    add: Pass,
    streak: Pass,
    mix: Pass,
    lay: Pass,
    combine: Pass,
    shrink: Pass,
    enlarge: Pass,
}

/// One dispatch before the layers are drawn: the pass, its bindings and its workgroups.
type Step = (wgpu::ComputePipeline, wgpu::BindGroup, (u32, u32));

/// B-65: the batch of ten (D-122) as passes, each the CPU's rule pixel for pixel in double
/// precision where the CPU works in it. Like [`BLOOM_SHADER`] it needs `SHADER_F64`. Drop Shadow,
/// Rim Light and Outline blur in Bloom's `gauss` pass.
const FX_SHADER: &str = r#"
struct Fx {
    mode: u32,
    blend: u32,
    count: u32,
    flag: u32,
    g: i32,
    r: i32,
    n: u32,
    frame: i32,
    base: vec2<u32>,
    ox: i32,
    oy: i32,
}

@group(0) @binding(0) var<uniform> F: Fx;
@group(0) @binding(1) var input: texture_2d<f32>;
@group(0) @binding(2) var output: texture_storage_2d<rgba32float, write>;
@group(0) @binding(3) var<storage, read> k: array<f64>;
@group(0) @binding(4) var other: texture_2d<f32>;
@group(0) @binding(5) var<storage, read_write> row: array<f32>;
@group(0) @binding(6) var<storage, read_write> band: array<f32>;
@group(0) @binding(7) var<storage, read_write> sums: array<vec4<f64>>;
@group(0) @binding(8) var<storage, read_write> dist: array<f64>;
@group(0) @binding(9) var<storage, read_write> glints: array<atomic<u32>>;

// The pixel of `t` at `p`, transparent outside it (layer_fx::at).
fn at(t: texture_2d<f32>, p: vec2<i32>) -> vec4<f32> {
    let size = vec2<i32>(textureDimensions(t));
    if any(p < vec2(0)) || any(p >= size) {
        return vec4(0.0);
    }
    return textureLoad(t, p, 0);
}

// render::sample_bilinear, its position and weights in double precision as the CPU's are.
fn bilinear(t: texture_2d<f32>, x: f64, y: f64) -> vec4<f32> {
    let size = vec2<i32>(textureDimensions(t));
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    // One pixel or more outside, every neighbour is outside; tested before the conversion to
    // integers, which a huge position would overflow.
    if !(fx > -1.0lf && fy > -1.0lf && fx < f64(size.x) && fy < f64(size.y)) {
        return vec4(0.0);
    }
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;
    var out = vec4(0.0);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0lf - uy, uy, j == 1);
        let sy = i32(y0) + j;
        if wy == 0.0lf || sy < 0 || sy >= size.y {
            continue;
        }
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0lf - ux, ux, i == 1);
            let sx = i32(x0) + i;
            if wx == 0.0lf || sx < 0 || sx >= size.x {
                continue;
            }
            out += textureLoad(t, vec2(sx, sy), 0) * f32(wx * wy);
        }
    }
    return out;
}

// D-408, layer_fx::keys_cubic: Keys' cubic with a = -0.5, in double precision as the CPU's.
fn keys_cubic(t: f64) -> f64 {
    let d = abs(t);
    if d <= 1.0lf {
        return (1.5lf * d - 2.5lf) * d * d + 1.0lf;
    }
    if d < 2.0lf {
        return ((-0.5lf * d + 2.5lf) * d - 4.0lf) * d + 2.0lf;
    }
    return 0.0lf;
}

// D-408, layer_fx::sample_bicubic: 4 by 4 taps from pixel centres, transparent outside, then
// each number held at 0 or above and the covering at 1 or below.
fn bicubic(t: texture_2d<f32>, x: f64, y: f64) -> vec4<f32> {
    let size = vec2<i32>(textureDimensions(t));
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    if !(fx > -2.0lf && fy > -2.0lf && fx < f64(size.x) + 1.0lf && fy < f64(size.y) + 1.0lf) {
        return vec4(0.0);
    }
    let x0 = floor(fx);
    let y0 = floor(fy);
    var out = vec4(0.0);
    for (var j = -1; j < 3; j++) {
        let wy = keys_cubic(fy - (y0 + f64(j)));
        let sy = i32(y0) + j;
        if wy == 0.0lf || sy < 0 || sy >= size.y {
            continue;
        }
        for (var i = -1; i < 3; i++) {
            let wx = keys_cubic(fx - (x0 + f64(i)));
            let sx = i32(x0) + i;
            if wx == 0.0lf || sx < 0 || sx >= size.x {
                continue;
            }
            out += textureLoad(t, vec2(sx, sy), 0) * f32(wx * wy);
        }
    }
    return vec4(max(out.x, 0.0), max(out.y, 0.0), max(out.z, 0.0), clamp(out.w, 0.0, 1.0));
}

// B-157 (G6): `bilinear` in single precision, for a pass whose many samples are summed, where each
// sample's own rounding is far below a level of 255.
fn bilinear32(t: texture_2d<f32>, x: f32, y: f32) -> vec4<f32> {
    let size = vec2<i32>(textureDimensions(t));
    let fx = x - 0.5;
    let fy = y - 0.5;
    if !(fx > -1.0 && fy > -1.0 && fx < f32(size.x) && fy < f32(size.y)) {
        return vec4(0.0);
    }
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;
    var out = vec4(0.0);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0 - uy, uy, j == 1);
        let sy = i32(y0) + j;
        if wy == 0.0 || sy < 0 || sy >= size.y {
            continue;
        }
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0 - ux, ux, i == 1);
            let sx = i32(x0) + i;
            if wx == 0.0 || sx < 0 || sx >= size.x {
                continue;
            }
            out += textureLoad(t, vec2(sx, sy), 0) * (wx * wy);
        }
    }
    return out;
}

// grade::to_srgb and grade::to_linear in double precision. The card's powers are single
// precision, so each starts from one and takes two Newton steps: y^12 = c^5 for the power
// 1/2.4, and y^5 = t^12 for 2.4.
fn to_srgb(c: f64) -> f64 {
    if c <= 0.0031308lf {
        return 12.92lf * c;
    }
    return 1.055lf * root(c) - 0.055lf;
}

// `c` to the power 1/2.4.
fn root(c: f64) -> f64 {
    let c2 = c * c;
    let c5 = c2 * c2 * c;
    var y = f64(pow(f32(c), 1.0 / 2.4));
    for (var i = 0; i < 2; i++) {
        let y2 = y * y;
        let y4 = y2 * y2;
        let y11 = y4 * y4 * y2 * y;
        y = y - (y11 * y - c5) / (12.0lf * y11);
    }
    return y;
}

fn to_linear(c: f64) -> f64 {
    if c <= 0.04045lf {
        return c / 12.92lf;
    }
    let t = (c + 0.055lf) / 1.055lf;
    let t2 = t * t;
    let t4 = t2 * t2;
    let t12 = t4 * t4 * t4;
    var y = f64(pow(f32(t), 2.4));
    for (var i = 0; i < 2; i++) {
        let y2 = y * y;
        let y4 = y2 * y2;
        y = y - (y4 * y - t12) / (5.0lf * y4);
    }
    return y;
}

fn rem(a: f64, b: f64) -> f64 {
    let r = a - b * floor(a / b);
    return select(r, r - b, r >= b);
}

// D-730: grade::noise_whole, the W3C non-separable modes: Saturation (13), Color (14) and
// Luminosity (15).
fn w3c_lum(c: vec3<f64>) -> f64 {
    return 0.3lf * c.x + 0.59lf * c.y + 0.11lf * c.z;
}

fn w3c_set_lum(c0: vec3<f64>, l0: f64) -> vec3<f64> {
    var c = c0 + vec3<f64>(l0 - w3c_lum(c0));
    let l = w3c_lum(c);
    let n = min(min(c.x, c.y), c.z);
    let x = max(max(c.x, c.y), c.z);
    if n < 0.0lf && l > n {
        c = vec3<f64>(l) + (c - vec3<f64>(l)) * l / (l - n);
    }
    if x > 1.0lf && x > l {
        c = vec3<f64>(l) + (c - vec3<f64>(l)) * (1.0lf - l) / (x - l);
    }
    return c;
}

fn w3c_set_sat(c: vec3<f64>, s: f64) -> vec3<f64> {
    let n = min(min(c.x, c.y), c.z);
    let x = max(max(c.x, c.y), c.z);
    if x > n {
        return (c - vec3<f64>(n)) * s / (x - n);
    }
    return vec3<f64>(0.0lf);
}

fn noise_whole(m: u32, b: vec3<f64>, s: vec3<f64>) -> vec3<f64> {
    if m == 13u {
        let t = max(max(s.x, s.y), s.z) - min(min(s.x, s.y), s.z);
        return w3c_set_lum(w3c_set_sat(b, t), w3c_lum(b));
    }
    if m == 14u {
        return w3c_set_lum(s, w3c_lum(b));
    }
    // D-731: Hue (16).
    if m == 16u {
        let t = max(max(b.x, b.y), b.z) - min(min(b.x, b.y), b.z);
        return w3c_set_lum(w3c_set_sat(s, t), w3c_lum(b));
    }
    return w3c_set_lum(b, w3c_lum(s));
}

// grade::mixer.
fn mixed(b: f64, c: f64) -> f64 {
    return mixed_with(F.blend, b, c);
}

fn mixed_with(m: u32, b: f64, c: f64) -> f64 {
    switch m {
        case 1u: { return b * c; }
        case 2u: { return 1.0lf - (1.0lf - b) * (1.0lf - c); }
        case 3u: { return b + c; }
        case 4u: { return select(1.0lf - 2.0lf * (1.0lf - b) * (1.0lf - c), 2.0lf * b * c, b <= 0.5lf); }
        case 5u: {
            if c <= 0.5lf {
                return b - (1.0lf - 2.0lf * c) * b * (1.0lf - b);
            }
            var d = sqrt(b);
            if b <= 0.25lf {
                d = ((16.0lf * b - 12.0lf) * b + 4.0lf) * b;
            }
            return b + (2.0lf * c - 1.0lf) * (d - b);
        }
        // D-647: Turbulent Noise's hard light, darken, lighten and difference.
        case 6u: { return select(1.0lf - 2.0lf * (1.0lf - b) * (1.0lf - c), 2.0lf * b * c, c <= 0.5lf); }
        case 7u: { return min(b, c); }
        case 8u: { return max(b, c); }
        case 9u: { return abs(b - c); }
        default: { return c; }
    }
}

// D-111: curve j's spline at x; k[j] is where it starts in `k`: its count, ins, outs and second
// derivatives.
fn curve(j: u32, x: f64) -> f64 {
    let o = u32(k[j]);
    let n = u32(k[o]);
    let xs = o + 1u;
    let ys = xs + n;
    let ms = ys + n;
    if x <= k[xs] {
        return k[ys];
    }
    if x >= k[xs + n - 1u] {
        return k[ys + n - 1u];
    }
    var i = n - 2u;
    while i > 0u && k[xs + i] > x {
        i--;
    }
    let t = x - k[xs + i];
    let hi = k[xs + i + 1u] - k[xs + i];
    let y0 = k[ys + i];
    let y1 = k[ys + i + 1u];
    let m0 = k[ms + i];
    let m1 = k[ms + i + 1u];
    return y0 + t * ((y1 - y0) / hi - hi * (2.0lf * m0 + m1) / 6.0lf) + t * t * m0 / 2.0lf
        + t * t * t * (m1 - m0) / (6.0lf * hi);
}

fn hsl(c: vec3<f64>) -> vec3<f64> {
    let mx = max(max(c.x, c.y), c.z);
    let mn = min(min(c.x, c.y), c.z);
    let d = mx - mn;
    let l = (mx + mn) / 2.0lf;
    if d == 0.0lf {
        return vec3(0.0lf, 0.0lf, l);
    }
    let s = d / (1.0lf - abs(2.0lf * l - 1.0lf));
    var h: f64;
    if mx == c.x {
        h = 60.0lf * rem((c.y - c.z) / d, 6.0lf);
    } else if mx == c.y {
        h = 60.0lf * ((c.z - c.x) / d + 2.0lf);
    } else {
        h = 60.0lf * ((c.x - c.y) / d + 4.0lf);
    }
    return vec3(h, s, l);
}

fn from_hsl(v: vec3<f64>) -> vec3<f64> {
    let c = (1.0lf - abs(2.0lf * v.z - 1.0lf)) * v.y;
    let x = c * (1.0lf - abs(rem(v.x / 60.0lf, 2.0lf) - 1.0lf));
    let m = v.z - c / 2.0lf;
    var o: vec3<f64>;
    switch ((i32(floor(v.x / 60.0lf)) % 6) + 6) % 6 {
        case 0: { o = vec3(c, x, 0.0lf); }
        case 1: { o = vec3(x, c, 0.0lf); }
        case 2: { o = vec3(0.0lf, c, x); }
        case 3: { o = vec3(0.0lf, x, c); }
        case 4: { o = vec3(x, 0.0lf, c); }
        default: { o = vec3(c, 0.0lf, x); }
    }
    return o + vec3(m);
}

// D-119: SplitMix64's finaliser on 64-bit words held as (low, high) pairs of 32 bits.
fn add64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let lo = a.x + b.x;
    return vec2(lo, a.y + b.y + select(0u, 1u, lo < a.x));
}

fn mul32(a: u32, b: u32) -> vec2<u32> {
    let a0 = a & 0xffffu;
    let a1 = a >> 16u;
    let b0 = b & 0xffffu;
    let b1 = b >> 16u;
    let p00 = a0 * b0;
    let p01 = a0 * b1;
    let p10 = a1 * b0;
    let mid = (p00 >> 16u) + (p01 & 0xffffu) + (p10 & 0xffffu);
    return vec2((p00 & 0xffffu) | (mid << 16u), a1 * b1 + (p01 >> 16u) + (p10 >> 16u) + (mid >> 16u));
}

fn mul64(a: vec2<u32>, b: vec2<u32>) -> vec2<u32> {
    let p = mul32(a.x, b.x);
    return vec2(p.x, p.y + a.x * b.y + a.y * b.x);
}

fn shr64(a: vec2<u32>, s: u32) -> vec2<u32> {
    return vec2((a.x >> s) | (a.y << (32u - s)), a.y >> s);
}

fn splitmix(w: vec2<u32>) -> vec2<u32> {
    var z = add64(w, vec2(0x7F4A7C15u, 0x9E3779B9u));
    z = mul64(z ^ shr64(z, 30u), vec2(0x1CE4E5B9u, 0xBF58476Du));
    z = mul64(z ^ shr64(z, 27u), vec2(0x133111EBu, 0x94D049BBu));
    return z ^ shr64(z, 31u);
}

// An i32 widened to 64 bits.
fn wide(v: i32) -> vec2<u32> {
    return vec2(bitcast<u32>(v), select(0u, 0xffffffffu, v < 0));
}

// grade::unit for the seed in `base`.
fn hashed(x: i32, y: i32, f: i32, ch: u32) -> f64 {
    let h = shr64(splitmix(splitmix(splitmix(splitmix(F.base ^ wide(x)) ^ wide(y)) ^ wide(f)) ^ vec2(ch, 0u)), 11u);
    let n = f64(h.y) * 4294967296.0lf + f64(h.x);
    return n / 9007199254740992.0lf * 2.0lf - 1.0lf;
}

fn fade(t: f64) -> f64 {
    return t * t * t * (t * (6.0lf * t - 15.0lf) + 10.0lf);
}

// grade::value, B-76.
fn cell_noise(ch: u32, p: vec3<f64>) -> f64 {
    let c = floor(p);
    let s = vec3(fade(p.x - c.x), fade(p.y - c.y), fade(p.z - c.z));
    let i = vec3<i32>(c);
    var v = 0.0lf;
    for (var corner = 0u; corner < 8u; corner++) {
        let d = vec3(corner & 1u, (corner >> 1u) & 1u, corner >> 2u);
        let w = select(vec3(1.0lf) - s, s, d == vec3(1u));
        v += w.x * w.y * w.z * hashed(i.x + i32(d.x), i.y + i32(d.y), i.z + i32(d.z), ch);
    }
    return v;
}

// grade::fractal.
fn fractal(ch: u32, p: vec3<f64>, octaves: u32) -> f64 {
    var sum = 0.0lf;
    var total = 0.0lf;
    var amp = 1.0lf;
    var fine = 1.0lf;
    for (var o = 0u; o < octaves; o++) {
        sum += amp * cell_noise(8u * o + ch, p * fine);
        total += amp;
        amp *= 0.5lf;
        fine *= 2.0lf;
    }
    return sum / total;
}

// D-299: grade::value with its block and period, and grade::fractal_with. Kept apart from
// `cell_noise` and `fractal`, which draw D-128's noise and Turbulent Displace as before.
fn cell_noise_look(ch: u32, p: vec3<f64>, block: bool, period: i32) -> f64 {
    let c = floor(p);
    var s = vec3(fade(p.x - c.x), fade(p.y - c.y), fade(p.z - c.z));
    if block {
        s = vec3(0.0lf, 0.0lf, s.z);
    }
    let i = vec3<i32>(c);
    var v = 0.0lf;
    for (var corner = 0u; corner < 8u; corner++) {
        let d = vec3(corner & 1u, (corner >> 1u) & 1u, corner >> 2u);
        let w = select(vec3(1.0lf) - s, s, d == vec3(1u));
        var z = i.z + i32(d.z);
        if period > 0 {
            // D-547: as rem_euclid, with `%` kept off negative numbers, which this card's
            // signed `%` takes as unsigned (-7 % 6 gave 3).
            z = select(z % period, period - 1 - (-1 - z) % period, z < 0);
        }
        // D-443: a corner that weighs nothing adds nothing, as grade::value skips it.
        let wt = w.x * w.y * w.z;
        if wt != 0.0lf {
            v += wt * hashed(i.x + i32(d.x), i.y + i32(d.y), z, ch);
        }
    }
    return v;
}

fn fractal_look(ch: u32, p: vec3<f64>, octaves: u32, turbulent: bool, block: bool, cycle: i32) -> f64 {
    var sum = 0.0lf;
    var total = 0.0lf;
    var amp = 1.0lf;
    var fine = 1.0lf;
    for (var o = 0u; o < octaves; o++) {
        var v = cell_noise_look(8u * o + ch, p * fine, block, cycle << o);
        if turbulent {
            v = abs(v);
        }
        sum += amp * v;
        total += amp;
        amp *= 0.5lf;
        fine *= 2.0lf;
    }
    if turbulent {
        return 2.0lf * sum / total - 1.0lf;
    }
    return sum / total;
}

// D-567: grade::value_kind and grade::fractal_kind, the noise types past smooth and block and
// the fractal types past basic and turbulent, each in the same order as there.
fn wrap_depth(z: i32, period: i32) -> i32 {
    if period > 0 {
        return select(z % period, period - 1 - (-1 - z) % period, z < 0);
    }
    return z;
}

fn catmull(t: f64) -> vec4<f64> {
    let t2 = t * t;
    let t3 = t2 * t;
    return vec4((-t3 + 2.0lf * t2 - t) / 2.0lf, (3.0lf * t3 - 5.0lf * t2 + 2.0lf) / 2.0lf,
        (-3.0lf * t3 + 4.0lf * t2 + t) / 2.0lf, (t3 - t2) / 2.0lf);
}

fn cell_noise_kind(ch: u32, p: vec3<f64>, noise: u32, period: i32) -> f64 {
    if noise < 2u {
        return cell_noise_look(ch, p, noise == 1u, period);
    }
    let c = floor(p);
    let t = p - c;
    let i = vec3<i32>(c);
    var v = 0.0lf;
    if noise == 4u {
        var wx = catmull(t.x);
        var wy = catmull(t.y);
        let sz = fade(t.z);
        for (var dk = 0; dk < 2; dk++) {
            let wz = select(1.0lf - sz, sz, dk == 1);
            let z = wrap_depth(i.z + dk, period);
            for (var b = 0; b < 4; b++) {
                for (var a = 0; a < 4; a++) {
                    let w = wx[a] * wy[b] * wz;
                    if w != 0.0lf {
                        v += w * hashed(i.x + a - 1, i.y + b - 1, z, ch);
                    }
                }
            }
        }
        return clamp(v, -1.0lf, 1.0lf);
    }
    var s = t;
    if noise == 3u {
        s = t * t * (vec3(3.0lf) - 2.0lf * t);
    }
    for (var corner = 0u; corner < 8u; corner++) {
        let d = vec3(corner & 1u, (corner >> 1u) & 1u, corner >> 2u);
        let w = select(vec3(1.0lf) - s, s, d == vec3(1u));
        let wt = w.x * w.y * w.z;
        if wt != 0.0lf {
            v += wt * hashed(i.x + i32(d.x), i.y + i32(d.y), wrap_depth(i.z + i32(d.z), period), ch);
        }
    }
    return v;
}

// D-647: with `warp`, `p` is the pixel's centre in the drawing's own pixels and octave o is read
// where grade::fractal_warped reads it, by k[21..]: the middle, the Sub Offset, Perspective
// Offset, Sub Influence, Sub Scaling, Center Subscale, then each octave's cosine and sine.
fn fractal_kind(ch: u32, p: vec3<f64>, octaves: u32, kind: u32, noise: u32, cycle: i32, warp: bool) -> f64 {
    let pi = 3.141592653589793lf;
    var step = select(0.5lf, 0.35lf, kind == 11u);
    var ratio = 2.0lf;
    if warp {
        step = select(k[26] / 100.0lf, 0.35lf * (k[26] / 50.0lf), kind == 11u);
        ratio = 100.0lf / k[27];
    }
    var w = 1.0lf;
    var fine = select(1.0lf, 4.0lf, kind == 14u);
    var sum = 0.0lf;
    var total = 0.0lf;
    var amp = 1.0lf;
    var so_far = 0.0lf;
    var prev = 0.0lf;
    var prev_ridge = 1.0lf;
    var best = 0.0lf;
    for (var o = 0u; o < octaves; o++) {
        var px = p.x * fine;
        var py = p.y * fine;
        let fo = f64(o);
        if warp {
            let dx = p.x - w * k[12] - fo * k[23] - k[21];
            let dy = p.y - w * k[13] - fo * k[24] - k[22];
            let c = k[29u + 2u * o];
            let s = k[30u + 2u * o];
            let qx = k[21] + dx * c + dy * s;
            let qy = k[22] - dx * s + dy * c;
            if k[28] == 1.0lf {
                px = (k[21] + fine * (qx - k[21])) / k[0];
                py = (k[22] + fine * (qy - k[22])) / k[11];
            } else {
                px = fine * qx / k[0];
                py = fine * qy / k[11];
            }
            if k[25] == 1.0lf {
                w *= 0.5lf;
            }
        }
        if kind == 4u {
            px += 4.0lf * so_far;
            py += 4.0lf * prev;
        } else if kind == 5u {
            px += 2.0lf * fo * so_far;
            py += 2.0lf * fo * prev;
        } else if kind == 6u && o > 0u {
            px += 2.0lf * cos64(2.0lf * pi * so_far);
            py += 2.0lf * sin64(2.0lf * pi * so_far);
        } else if kind == 8u && o > 0u {
            px = px / 4.0lf + 2.0lf * so_far;
        }
        let v = cell_noise_kind(8u * o + ch, vec3(px, py, p.z * fine), noise, cycle << o);
        let ridge = (1.0lf - abs(v)) * (1.0lf - abs(v));
        var g = v;
        if kind == 1u {
            g = abs(v);
        } else if kind == 2u {
            g = v * v;
        } else if kind == 3u {
            g = sqrt(abs(v));
        } else if kind == 12u {
            g = ridge * prev_ridge;
        } else if kind == 16u {
            g = 2.0lf * max(1.0lf - 6.0lf * abs(v), 0.0lf) - 1.0lf;
        }
        if kind == 13u && o > 0u {
            g *= (prev + 1.0lf) / 2.0lf;
        }
        sum += amp * g;
        total += amp;
        so_far = sum / total;
        best = select(best + amp * max(v - best, 0.0lf), v, o == 0u);
        prev = v;
        prev_ridge = ridge;
        amp *= step;
        fine *= ratio;
    }
    let b = sum / total;
    if kind == 1u || kind == 2u || kind == 3u || kind == 12u {
        return 2.0lf * b - 1.0lf;
    } else if kind == 7u {
        return best;
    } else if kind == 9u {
        return sin64(3.0lf * pi * b);
    } else if kind == 10u {
        return clamp(floor(2.0lf * b + 0.5lf) / 2.0lf, -1.0lf, 1.0lf);
    } else if kind == 11u {
        return (b + 1.0lf) * (b + 1.0lf) / 2.0lf - 1.0lf;
    } else if kind == 15u {
        return 2.0lf * max(1.0lf - 6.0lf * abs(b), 0.0lf) - 1.0lf;
    }
    return b;
}

// B-157 (G6): `hashed`, `cell_noise` and `fractal` in single precision, for Turbulent Displace,
// where the noise only moves where a pixel is read from, by far less than a pixel's rounding.
// `hashed`'s 53 bits are (h.y 2^32 + h.x) / 2^52 - 1, h.y's 21 bits exact in single precision.
fn hashed32(x: i32, y: i32, f: i32, ch: u32) -> f32 {
    let h = shr64(splitmix(splitmix(splitmix(splitmix(F.base ^ wide(x)) ^ wide(y)) ^ wide(f)) ^ vec2(ch, 0u)), 11u);
    return f32(h.y) / 1048576.0 + f32(h.x) / 4503599627370496.0 - 1.0;
}

fn fade32(t: f32) -> f32 {
    return t * t * t * (t * (6.0 * t - 15.0) + 10.0);
}

fn cell_noise32(ch: u32, p: vec3<f32>) -> f32 {
    let c = floor(p);
    let s = vec3(fade32(p.x - c.x), fade32(p.y - c.y), fade32(p.z - c.z));
    let i = vec3<i32>(c);
    var v = 0.0;
    for (var corner = 0u; corner < 8u; corner++) {
        let d = vec3(corner & 1u, (corner >> 1u) & 1u, corner >> 2u);
        let w = select(vec3(1.0) - s, s, d == vec3(1u));
        v += w.x * w.y * w.z * hashed32(i.x + i32(d.x), i.y + i32(d.y), i.z + i32(d.z), ch);
    }
    return v;
}

fn fractal32(ch: u32, p: vec3<f32>, octaves: u32) -> f32 {
    var sum = 0.0;
    var total = 0.0;
    var amp = 1.0;
    var fine = 1.0;
    for (var o = 0u; o < octaves; o++) {
        sum += amp * cell_noise32(8u * o + ch, p * fine);
        total += amp;
        amp *= 0.5;
        fine *= 2.0;
    }
    return sum / total;
}

// The batch's colour effects, a pixel a thread: grade::curves (mode 0), levels (1),
// hue_saturation (2), gradient (3) and noise (4); B-76: exposure (5, Exposure Flicker),
// color_balance (6), gradient_map (7), vignette (8) and fractal_noise (9); D-443: add_grain (10).
@compute @workgroup_size(16, 16)
fn grade(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    if F.mode == 5u {
        // k: the gain, in single precision as the CPU's.
        textureStore(output, id.xy, vec4(p.xyz * f32(k[0]), p.w));
        return;
    }
    if F.mode == 11u {
        // D-450, grade::noise_alpha. k: the amount as a share, the depth (0 for the Random
        // kinds), the period, squared (1), the original alpha (0 Clamp, 1 Add, 2 Scale, 3 Edges)
        // and the overflow (0 Clip, 1 Wrap Back, 2 Wrap). `base` is the seed, 0 when animated.
        let a = f64(p.w);
        var skip = false;
        if k[4] == 0.0lf {
            skip = a != 1.0lf;
        } else if k[4] == 2.0lf {
            skip = a <= 0.0lf;
        } else if k[4] == 3.0lf {
            skip = !(a > 0.0lf && a < 1.0lf);
        }
        if skip {
            textureStore(output, id.xy, p);
            return;
        }
        let q = vec3(f64(i32(id.x) - F.ox), f64(i32(id.y) - F.oy), k[1]);
        var n = cell_noise_look(0u, q, true, i32(k[2]));
        if k[3] == 1.0lf {
            let r = 1.0lf - abs(n);
            n = select(-(1.0lf - r * r), 1.0lf - r * r, n >= 0.0lf);
        }
        let d = k[0] * n;
        var t = a + d;
        if k[4] == 2.0lf {
            t = a + d * a;
        }
        if t < 0.0lf || t > 1.0lf {
            if k[5] == 1.0lf {
                t = select(-t, 2.0lf - t, t > 1.0lf);
            } else if k[5] == 2.0lf {
                t = select(t + 1.0lf, t - 1.0lf, t > 1.0lf);
            }
            t = clamp(t, 0.0lf, 1.0lf);
        }
        if t == a {
            textureStore(output, id.xy, p);
            return;
        }
        var out = vec4(0.0, 0.0, 0.0, f32(t));
        if a > 0.0lf {
            out = vec4(vec3<f32>(vec3<f64>(p.xyz) / a * t), f32(t));
        }
        textureStore(output, id.xy, out);
        return;
    }
    let a = f64(p.w);
    // D-730: Turbulent Noise's None draws on clear pixels too.
    let noise_none = (F.mode == 9u || F.mode == 111u || F.mode == 112u) && F.blend == 10u;
    if a <= 0.0lf && !noise_none {
        textureStore(output, id.xy, p);
        return;
    }
    let px = vec3<f64>(p.xyz);
    if F.mode == 7u {
        // k: the midpoint and the amount as shares, the three colours encoded.
        let b = px / a;
        let t = to_srgb(clamp(0.2126lf * b.x + 0.7152lf * b.y + 0.0722lf * b.z, 0.0lf, 1.0lf));
        var lo = 2u;
        var s = t / k[0];
        if t > k[0] {
            lo = 5u;
            s = (t - k[0]) / (1.0lf - k[0]);
        }
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            let g = to_linear(k[lo + c] + s * (k[lo + 3u + c] - k[lo + c]));
            out[c] = f32((b[c] + k[1] * (g - b[c])) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 8u {
        // k: the centre, the half-axes, where the fall starts and ends, the amount, the root of
        // 2, the colour in linear light, and D-637's 1 / p of the norm (0.5 the circle's).
        let dx = (f64(id.x) + 0.5lf - k[0]) / k[2];
        let dy = (f64(id.y) + 0.5lf - k[1]) / k[3];
        var d = sqrt(dx * dx + dy * dy) / k[7];
        if k[11] != 0.5lf {
            let most = max(abs(dx), abs(dy));
            d = most;
            if k[11] > 0.0lf && most > 0.0lf {
                let p = f32(1.0lf / k[11]);
                let sum = pow(f32(abs(dx) / most), p) + pow(f32(abs(dy) / most), p);
                d = most * f64(pow(sum, f32(k[11])) / pow(2.0, f32(k[11])));
            }
        }
        var t = select(0.0lf, 1.0lf, d >= k[5]);
        if k[4] != k[5] {
            let u = clamp((d - k[4]) / (k[5] - k[4]), 0.0lf, 1.0lf);
            t = u * u * (3.0lf - 2.0lf * u);
        }
        let o = t * k[6] / 100.0lf;
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            let b = px[c] / a;
            out[c] = f32((b + o * (k[8u + c] - b)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 9u || F.mode == 111u || F.mode == 112u {
        // k: the size, the depth, the contrast and brightness, the opacity as a share, the dark
        // and light colours encoded. `count` is the octaves. D-299: then the size down, the
        // offset, turbulent, block, the cycle and invert, each 1 when on. D-567: mode 111 is
        // the types past basic, turbulent, smooth and block, their places then. D-647: k[20] the
        // Overflow; mode 112 the Transform and Sub Settings, k[21..] as fractal_kind's.
        let x = (f64(i32(id.x) - F.ox) + 0.5lf - k[12]) / k[0];
        let y = (f64(i32(id.y) - F.oy) + 0.5lf - k[13]) / k[11];
        var n = 0.0lf;
        if F.mode == 112u {
            let c = vec3(f64(i32(id.x) - F.ox) + 0.5lf, f64(i32(id.y) - F.oy) + 0.5lf, k[1]);
            n = fractal_kind(0u, c, F.count, u32(k[18]), u32(k[19]), i32(k[16]), true);
        } else if F.mode == 111u {
            n = fractal_kind(0u, vec3(x, y, k[1]), F.count, u32(k[18]), u32(k[19]), i32(k[16]), false);
        } else if k[14] == 0.0lf && k[15] == 0.0lf && k[16] == 0.0lf {
            n = fractal(0u, vec3(x, y, k[1]), F.count);
        } else {
            n = fractal_look(0u, vec3(x, y, k[1]), F.count, k[14] == 1.0lf, k[15] == 1.0lf, i32(k[16]));
        }
        if k[17] == 1.0lf {
            n = -n;
        }
        var v = 0.5lf + 0.5lf * n * k[2] / 100.0lf + k[3] / 100.0lf;
        if k[20] == 2.0lf {
            // single precision's tanh: within a level of the processor's.
            v = 0.5lf + 0.5lf * f64(tanh(f32(2.0lf * (v - 0.5lf))));
        } else if k[20] == 3.0lf {
            v = v - 2.0lf * floor(v / 2.0lf);
            v = select(v, 2.0lf - v, v > 1.0lf);
        } else {
            v = clamp(v, 0.0lf, 1.0lf);
        }
        var out = p;
        var col = vec3<f64>(0.0lf);
        for (var c = 0u; c < 3u; c++) {
            col[c] = to_linear(k[5u + c] + v * (k[8u + c] - k[5u + c]));
        }
        // D-730: None (10), Stencil Alpha (11) and Stencil Luma (12), as grade::fractal_noise
        // lays them through composite::blend_pixel.
        if F.blend == 10u {
            textureStore(output, id.xy, vec4<f32>(vec3<f32>(col * k[4]), f32(k[4])));
            return;
        }
        if F.blend == 11u || F.blend == 12u {
            var s = f32(k[4]);
            if F.blend == 12u {
                let q = vec3<f32>(col * k[4]);
                let y = 0.2126lf * f64(q.x) + 0.7152lf * f64(q.y) + 0.0722lf * f64(q.z);
                s = f32(to_srgb(clamp(y, 0.0lf, 1.0lf)));
            }
            textureStore(output, id.xy, p * s);
            return;
        }
        var b = px / a;
        var m = vec3<f64>(0.0lf);
        if F.blend >= 13u {
            m = noise_whole(F.blend, b, col);
        } else {
            for (var c = 0u; c < 3u; c++) {
                m[c] = mixed(b[c], col[c]);
            }
        }
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32((b[c] + k[4] * (m[c] - b[c])) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 3u {
        // k: start, end less start, its length squared and that's root, radial, the two
        // opacities, the two colours.
        let dx = f64(id.x) + 0.5lf - k[0];
        let dy = f64(id.y) + 0.5lf - k[1];
        let ll = k[4];
        var t = 1.0lf;
        if ll != 0.0lf {
            if k[6] == 1.0lf {
                t = sqrt(dx * dx + dy * dy) / k[5];
            } else {
                t = (dx * k[2] + dy * k[3]) / ll;
            }
        }
        t = clamp(t, 0.0lf, 1.0lf);
        let o = (k[7] + t * (k[8] - k[7])) / 100.0lf;
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            let color = to_linear(k[9u + c] + t * (k[12u + c] - k[9u + c]));
            let b = px[c] / a;
            out[c] = f32((b + o * (mixed(b, color) - b)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    var e: vec3<f64>;
    for (var c = 0u; c < 3u; c++) {
        e[c] = to_srgb(clamp(px[c] / a, 0.0lf, 1.0lf));
    }
    var o = e;
    switch F.mode {
        case 0u: {
            for (var c = 0u; c < 3u; c++) {
                o[c] = clamp(curve(3u, clamp(curve(c, e[c] * 255.0lf), 0.0lf, 255.0lf)), 0.0lf, 255.0lf) / 255.0lf;
            }
        }
        case 1u: {
            // k: four sets of input black and white, one over the gamma, output black and white:
            // RGB, then D-383's red, green and blue. Each channel through its own set, then
            // through RGB's; a set that changes nothing is skipped.
            for (var c = 0u; c < 3u; c++) {
                var x = e[c] * 255.0lf;
                for (var n = 0u; n < 2u; n++) {
                    let b = select(0u, 5u * (c + 1u), n == 0u);
                    if k[b] == 0.0lf && k[b + 1u] == 255.0lf && k[b + 2u] == 1.0lf && k[b + 3u] == 0.0lf && k[b + 4u] == 255.0lf {
                        continue;
                    }
                    // The power in single precision, the one step here the card cannot do in
                    // double; its ends are exact.
                    let v = clamp((x - k[b]) / (k[b + 1u] - k[b]), 0.0lf, 1.0lf);
                    var bent = f64(pow(f32(v), f32(k[b + 2u])));
                    if v == 0.0lf || v == 1.0lf {
                        bent = v;
                    }
                    x = k[b + 3u] + bent * (k[b + 4u] - k[b + 3u]);
                }
                o[c] = x / 255.0lf;
            }
        }
        case 2u: {
            // k: hue, saturation, lightness.
            let v = hsl(e);
            var l = v.z * (1.0lf + k[2] / 100.0lf);
            if k[2] >= 0.0lf {
                l = v.z + (1.0lf - v.z) * k[2] / 100.0lf;
            }
            o = from_hsl(vec3(rem(v.x + k[0], 360.0lf), clamp(v.y * (1.0lf + k[1] / 100.0lf), 0.0lf, 1.0lf), l));
        }
        case 6u: {
            // k: shadows, midtones, highlights.
            let l = 0.2126lf * e.x + 0.7152lf * e.y + 0.0722lf * e.z;
            let ws = clamp(1.0lf - 2.0lf * l, 0.0lf, 1.0lf);
            let wh = clamp(2.0lf * l - 1.0lf, 0.0lf, 1.0lf);
            let wm = 1.0lf - ws - wh;
            for (var c = 0u; c < 3u; c++) {
                o[c] = e[c] + (ws * k[c] + wm * k[3u + c] + wh * k[6u + c]) / 200.0lf;
            }
            // k[9]: D-295's Preserve Luminosity, as grade::color_balance.
            if k[9] == 1.0lf {
                let d = l - (0.2126lf * o.x + 0.7152lf * o.y + 0.0722lf * o.z);
                o = o + vec3(d, d, d);
                let n = min(o.x, min(o.y, o.z));
                let x = max(o.x, max(o.y, o.z));
                var s = 1.0lf;
                if n < 0.0lf {
                    s = l / (l - n);
                }
                if x > 1.0lf {
                    s = min(s, (1.0lf - l) / (x - l));
                }
                o = vec3(l, l, l) + (o - vec3(l, l, l)) * s;
            }
        }
        case 10u: {
            // D-443, grade::add_grain. k: the cell across and down, the depth, softness,
            // saturation, the three channel intensities, shadows, midtones, highlights, midpoint,
            // 0.1 intensity and the blend (0 Film, 1 Add, 2 Overlay). `flag` is Monochromatic.
            let q = vec3((f64(i32(id.x) - F.ox) + 0.5lf) / k[0], (f64(i32(id.y) - F.oy) + 0.5lf) / k[1], k[2]);
            var g = vec3(0.0lf);
            let n = select(3u, 1u, F.flag == 1u);
            for (var c = 0u; c < n; c++) {
                if k[3] == 0.0lf {
                    g[c] = cell_noise_look(c, q, true, 0);
                } else if k[3] == 1.0lf {
                    g[c] = cell_noise_look(c, q, false, 0);
                } else {
                    g[c] = (1.0lf - k[3]) * cell_noise_look(c, q, true, 0) + k[3] * cell_noise_look(c, q, false, 0);
                }
            }
            if F.flag == 1u {
                g = vec3(g.x, g.x, g.x);
            } else {
                let m = (g.x + g.y + g.z) / 3.0lf;
                g = vec3(m + k[4] * (g.x - m), m + k[4] * (g.y - m), m + k[4] * (g.z - m));
            }
            let l = 0.2126lf * e.x + 0.7152lf * e.y + 0.0722lf * e.z;
            var wt = k[9] + (k[10] - k[9]) * (l - k[11]) / (1.0lf - k[11]);
            if l <= k[11] {
                wt = k[8] + (k[9] - k[8]) * l / k[11];
            }
            let amount = k[12] * wt;
            for (var c = 0u; c < 3u; c++) {
                let t = amount * (g[c] * k[5u + c]);
                if k[13] == 0.0lf {
                    o[c] = e[c] + t * 4.0lf * e[c] * (1.0lf - e[c]);
                } else if k[13] == 1.0lf {
                    o[c] = e[c] + t;
                } else {
                    let b = clamp(0.5lf + 0.5lf * t, 0.0lf, 1.0lf);
                    o[c] = select(1.0lf - 2.0lf * (1.0lf - e[c]) * (1.0lf - b), 2.0lf * e[c] * b, e[c] <= 0.5lf);
                }
            }
        }
        case 12u: {
            // D-451, grade::noise_hls. k: hue, lightness and saturation as shares, the grain
            // size, the depth and the kind (0 Uniform, 1 Squared, 2 Grain). `base` is seed 0.
            let x = f64(i32(id.x) - F.ox);
            let y = f64(i32(id.y) - F.oy);
            var n = vec3(0.0lf);
            for (var c = 0u; c < 3u; c++) {
                // D-452: an amount of 0 (k[c], hue, lightness, saturation) moves nothing, so
                // its noise is not drawn.
                if k[c] == 0.0lf {
                    continue;
                }
                if k[5] == 2.0lf {
                    n[c] = cell_noise_look(c, vec3((x + 0.5lf) / k[3], (y + 0.5lf) / k[3], k[4]), false, 0);
                } else {
                    n[c] = cell_noise_look(c, vec3(x, y, k[4]), true, 0);
                    if k[5] == 1.0lf {
                        let r = 1.0lf - abs(n[c]);
                        n[c] = select(-(1.0lf - r * r), 1.0lf - r * r, n[c] >= 0.0lf);
                    }
                }
            }
            let v = hsl(e);
            o = from_hsl(vec3(
                rem(v.x + 180.0lf * n.x * k[0], 360.0lf),
                clamp(v.y + n.z * k[2], 0.0lf, 1.0lf),
                clamp(v.z + n.y * k[1], 0.0lf, 1.0lf),
            ));
        }
        default: {
            // k: amount over 200. `flag` is colour.
            let x = i32(id.x) - F.ox;
            let y = i32(id.y) - F.oy;
            let h = splitmix(splitmix(splitmix(F.base ^ wide(x)) ^ wide(y)) ^ wide(F.frame));
            for (var c = 0u; c < 3u; c++) {
                let m = shr64(splitmix(h ^ vec2(select(0u, c, F.flag == 1u), 0u)), 11u);
                let n = f64(m.y) * 4294967296.0lf + f64(m.x);
                o[c] = e[c] + k[0] * (n / 9007199254740992.0lf * 2.0lf - 1.0lf);
            }
        }
    }
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        out[c] = f32(to_linear(clamp(o[c], 0.0lf, 1.0lf)) * a);
    }
    textureStore(output, id.xy, out);
}

// layer_fx::chromatic_aberration. k: the centre, 1 - k and 1 + k. With `flag` 1 (radial) or
// 2 (offset), layer_fx::lens_chromatic_aberration (D-409). k: the centre, the half diagonal,
// amount over it, the falloff's power, the angle's step across and down, the amount, the fringe
// blur, the three scales and the three sample counts.
@compute @workgroup_size(16, 16)
fn aberr(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    if F.flag != 0u {
        let x = f64(id.x) + 0.5lf;
        let y = f64(id.y) + 0.5lf;
        let ldx = x - k[0];
        let ldy = y - k[1];
        var g = 1.0lf;
        if k[4] != 0.0lf {
            let rho = sqrt(ldx * ldx + ldy * ldy) / k[2];
            g = select(0.0lf, f64(pow(f32(rho), f32(k[4]))), rho > 0.0lf);
        }
        var o = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        for (var c = 0u; c < 3u; c++) {
            let s = k[9u + c];
            let n = u32(k[12u + c]);
            var total = vec4<f32>(0.0, 0.0, 0.0, 0.0);
            for (var j = 0u; j < n; j++) {
                var t = 0.0lf;
                if n > 1u {
                    t = f64(j) / f64(n - 1u) - 0.5lf;
                }
                if F.flag == 2u {
                    let m = k[7] * (t * k[8] * abs(s) - s);
                    total += bilinear(input, x + k[5] * m, y + k[6] * m);
                } else {
                    let f = 1.0lf - s * k[3] * g + t * k[8] * abs(s) * k[3] * g;
                    total += bilinear(input, k[0] + ldx * f, k[1] + ldy * f);
                }
            }
            let mean = total / f32(n);
            o[c] = mean[c];
            o.w = max(o.w, mean.w);
        }
        textureStore(output, id.xy, o);
        return;
    }
    let dx = f64(id.x) + 0.5lf - k[0];
    let dy = f64(id.y) + 0.5lf - k[1];
    let red = bilinear(input, k[0] + dx * k[2], k[1] + dy * k[2]);
    let blue = bilinear(input, k[0] + dx * k[3], k[1] + dy * k[3]);
    let p = textureLoad(input, id.xy, 0);
    textureStore(output, id.xy, vec4(red.x, p.y, blue.z, max(max(p.w, red.w), blue.w)));
}

// layer_fx::drop_shadow's last step: `other` is the blurred covering, grown by `r`, and the
// output is the drawing grown by `g`. k: the move across and down, the opacity, the colour.
@compute @workgroup_size(16, 16)
fn shadow(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = i32(id.x) - F.g;
    let y = i32(id.y) - F.g;
    let a = f64(bilinear(other, f64(x + F.r) + 0.5lf - k[0], f64(y + F.r) + 0.5lf - k[1]).w) * k[2];
    let d = at(input, vec2(x, y));
    let rest = a * (1.0lf - f64(d.w));
    let c = vec3<f64>(vec3<f64>(d.xyz) + vec3(k[3], k[4], k[5]) * rest);
    textureStore(output, id.xy, vec4(vec3<f32>(c), f32(f64(d.w) + rest)));
}

// layer_fx::rim_light's last step: `other` is the blurred covering, grown by `r`. k: the reach
// across and down, the intensity, the colour; `blend` the mix.
@compute @workgroup_size(16, 16)
fn rim(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let x = f64(id.x) + f64(F.r) + 0.5lf;
    let y = f64(id.y) + f64(F.r) + 0.5lf;
    let off = f64(bilinear(other, x + k[0], y + k[1]).w);
    let shine = clamp(1.0lf - off, 0.0lf, 1.0lf) * k[2];
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let b = f64(p[c]) / a;
        out[c] = f32((b + shine * (mixed(b, k[3u + c]) - b)) * a);
    }
    textureStore(output, id.xy, out);
}

// layer_fx::outline's band, the drawing `n` wider on every side: `row` is each row's greatest
// covering within the current half-width, `band` the greatest over the disc so far. B-107: with
// `flag`, Simple Choker's shrink, the least.
fn pick(a: f32, b: f32) -> f32 {
    return select(max(a, b), min(a, b), F.flag == 1u);
}

@compute @workgroup_size(16, 16)
fn rows(@builtin(global_invocation_id) id: vec3<u32>) {
    let h = textureDimensions(input).y;
    let bw = textureDimensions(input).x + 2u * F.n;
    if id.x >= bw || id.y >= h {
        return;
    }
    let x = i32(id.x) - i32(F.n);
    let y = i32(id.y);
    let i = id.y * bw + id.x;
    let s = i32(F.count);
    if s == 0 {
        row[i] = at(input, vec2(x, y)).w;
    } else {
        row[i] = pick(pick(row[i], at(input, vec2(x - s, y)).w), at(input, vec2(x + s, y)).w);
    }
}

// Each disc row whose half-width is the current one, its offsets in `k`, maxed into the band; a
// row outside the layer is empty. B-107: `mode` 1 is the first, the band starting from infinity.
@compute @workgroup_size(16, 16)
fn bands(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    let bw = size.x + 2u * F.n;
    let bh = size.y + 2u * F.n;
    if id.x >= bw || id.y >= bh {
        return;
    }
    let i = id.y * bw + id.x;
    var v = band[i];
    if F.mode == 1u {
        v = bitcast<f32>(0x7f800000u);
    }
    for (var j = 0u; j < F.count; j++) {
        let sy = i32(id.y) - i32(F.n) + i32(k[j]);
        if sy < 0 || sy >= i32(size.y) {
            v = pick(v, 0.0);
            continue;
        }
        v = pick(v, row[u32(sy) * bw + id.x]);
    }
    band[i] = v;
}

// The band as a covering, to be blurred.
@compute @workgroup_size(16, 16)
fn lift(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    textureStore(output, id.xy, vec4(0.0, 0.0, 0.0, band[id.y * size.x + id.x]));
}

// layer_fx::outline's last step: `other` the blurred band, the drawing laid over it `g` in.
// k: the opacity, the colour.
@compute @workgroup_size(16, 16)
fn ring(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let d = at(input, vec2<i32>(id.xy) - vec2(F.g));
    let rest = f64(textureLoad(other, id.xy, 0).w) * k[0] * (1.0lf - f64(d.w));
    let c = vec3<f64>(d.xyz) + vec3(k[1], k[2], k[3]) * rest;
    textureStore(output, id.xy, vec4(vec3<f32>(c), f32(f64(d.w) + rest)));
}

// layer_fx::lens_blur's highlights: with `flag`, a pixel whose brightest straight channel reaches
// the threshold k[1] has its colour times k[2]. The quotient is taken in double precision and
// rounded, which is the CPU's single-precision quotient.
fn lit(p: vec4<f32>) -> vec4<f32> {
    if F.flag == 1u && p.w > 0.0 && f32(f64(max(max(p.x, p.y), p.z)) / f64(p.w)) >= f32(k[1]) {
        return vec4(p.xyz * f32(k[2]), p.w);
    }
    return p;
}

// Each row's running totals in double precision, one row a thread.
@compute @workgroup_size(64)
fn prefix(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.y {
        return;
    }
    let base = id.x * (size.x + 1u);
    var s = vec4<f64>(0.0lf);
    sums[base] = s;
    for (var x = 0u; x < size.x; x++) {
        s += vec4<f64>(lit(textureLoad(input, vec2(x, id.x), 0)));
        sums[base + x + 1u] = s;
    }
}

// layer_fx::lens_blur: each pixel adds the iris's rows in the CPU's order, from the running
// totals. k: the iris's pixel count, the threshold and gain, then each row's offset, first and
// last. `mode` is Repeat Edge Pixels.
@compute @workgroup_size(16, 16)
fn gather(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let w = i32(textureDimensions(input).x);
    let h = i32(textureDimensions(input).y);
    let x = i32(id.x) - F.g;
    let y = i32(id.y) - F.g;
    var acc = vec4<f64>(0.0lf);
    for (var j = 0u; j < F.count; j++) {
        let dy = i32(k[3u + 3u * j]);
        let lo = i32(k[4u + 3u * j]);
        let hi = i32(k[5u + 3u * j]);
        var sy = y - dy;
        if F.mode == 1u {
            sy = clamp(sy, 0, h - 1);
        } else if sy < 0 || sy >= h {
            continue;
        }
        let a = x - hi;
        let b = x - lo;
        let l = max(a, 0);
        let r = min(b, w - 1);
        let base = u32(sy * (w + 1));
        if l <= r {
            acc += sums[base + u32(r + 1)] - sums[base + u32(l)];
        }
        if F.mode == 1u {
            let left = f64(max(min(b, -1) - a + 1, 0));
            let right = f64(max(b - max(a, w) + 1, 0));
            let first = vec4<f64>(lit(textureLoad(input, vec2(0, sy), 0)));
            let last = vec4<f64>(lit(textureLoad(input, vec2(w - 1, sy), 0)));
            acc += left * first + right * last;
        }
    }
    if F.flag == 1u {
        acc = vec4(min(acc.xyz, vec3(acc.w)), acc.w);
    }
    textureStore(output, id.xy, vec4<f32>(acc / k[0]));
}

// D-359, layer_fx::lens_blur_map: one level's mean at the drawing's pixel (x, y), its runs at
// k[start..], summed as `gather` sums them.
fn lmean(start: u32, count: u32, n: f64, x: i32, y: i32, w: i32, h: i32) -> vec4<f64> {
    var acc = vec4<f64>(0.0lf);
    for (var j = 0u; j < count; j++) {
        let dy = i32(k[start + 3u * j]);
        let lo = i32(k[start + 1u + 3u * j]);
        let hi = i32(k[start + 2u + 3u * j]);
        var sy = y - dy;
        if F.mode == 1u {
            sy = clamp(sy, 0, h - 1);
        } else if sy < 0 || sy >= h {
            continue;
        }
        let a = x - hi;
        let b = x - lo;
        let l = max(a, 0);
        let r = min(b, w - 1);
        let base = u32(sy * (w + 1));
        if l <= r {
            acc += sums[base + u32(r + 1)] - sums[base + u32(l)];
        }
        if F.mode == 1u {
            let left = f64(max(min(b, -1) - a + 1, 0));
            let right = f64(max(b - max(a, w) + 1, 0));
            let first = vec4<f64>(lit(textureLoad(input, vec2(0, sy), 0)));
            let last = vec4<f64>(lit(textureLoad(input, vec2(w - 1, sy), 0)));
            acc += left * first + right * last;
        }
    }
    return acc / n;
}

// D-359, layer_fx::lens_blur_map: Lens Blur with `other` the blur map. k: J, the threshold and
// gain, the focus, alpha, invert, the drawing's corner (ox, oy); then each level's count, where
// its runs start in k and how many; then the runs.
@compute @workgroup_size(16, 16)
fn lmgather(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let w = i32(textureDimensions(input).x);
    let h = i32(textureDimensions(input).y);
    let ms = vec2<i32>(textureDimensions(other));
    let x = i32(id.x) - F.g;
    let y = i32(id.y) - F.g;
    // One channel at a time: the card's whole-vector widening reads this map as nothing.
    let q = textureLoad(other, clamp(vec2(x - i32(k[6]), y - i32(k[7])), vec2(0), ms - 1), 0);
    let m = vec4(f64(q.x), f64(q.y), f64(q.z), f64(q.w));
    var v = m.w;
    if k[4] == 0.0lf {
        v = to_srgb(clamp(luma(m.xyz), 0.0lf, 1.0lf));
    }
    if k[5] == 1.0lf {
        v = 1.0lf - v;
    }
    let big = u32(k[0]);
    let s = abs(v - k[3]) * k[0];
    let j = min(u32(floor(s)), big);
    let t = s - f64(j);
    var acc = lmean(u32(k[9u + 3u * j]), u32(k[10u + 3u * j]), k[8u + 3u * j], x, y, w, h);
    if t > 0.0lf {
        let next = lmean(u32(k[12u + 3u * j]), u32(k[13u + 3u * j]), k[11u + 3u * j], x, y, w, h);
        acc += t * (next - acc);
    }
    if F.flag == 1u {
        acc = vec4(min(acc.xyz, vec3(acc.w)), acc.w);
    }
    // B-222: one channel at a time.
    textureStore(output, id.xy, vec4(f32(acc.x), f32(acc.y), f32(acc.z), f32(acc.w)));
}

// B-76, layer_fx::turbulent_displace: `output` is the drawing grown by `g`. k: the amount, the
// size, the depth, the drawing's corner in the output. `count` is the octaves, `flag` Repeat
// Edge Pixels.
@compute @workgroup_size(16, 16)
fn turb(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let p = vec3<f32>(vec3((x - k[3]) / k[1], (y - k[4]) / k[1], k[2]));
    var sx = x + k[0] * f64(fractal32(0u, p, F.count));
    var sy = y + k[0] * f64(fractal32(1u, p, F.count));
    if F.flag == 1u {
        sx = clamp(sx, 0.5lf, f64(size.x) - 0.5lf);
        sy = clamp(sy, 0.5lf, f64(size.y) - 0.5lf);
    }
    textureStore(output, id.xy, bilinear(input, sx - f64(F.g), sy - f64(F.g)));
}

// In single precision: a vec4<f64> handed back from a function reads as zero on the card B-76
// was measured on.
fn around(p: vec2<i32>) -> vec4<f32> {
    let size = vec2<i32>(textureDimensions(input));
    return textureLoad(input, ((p % size) + size) % size, 0);
}

// B-76, layer_fx::offset: `ox` and `oy` are the whole part of the move back, already brought
// inside the drawing, k its fractions.
@compute @workgroup_size(16, 16)
fn slide(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let q = vec2<i32>(id.xy) + vec2(F.ox, F.oy);
    let a = vec4<f64>(around(q));
    let b = vec4<f64>(around(q + vec2(1, 0)));
    let c = vec4<f64>(around(q + vec2(0, 1)));
    let d = vec4<f64>(around(q + vec2(1, 1)));
    let top = (1.0lf - k[0]) * a + k[0] * b;
    let bottom = (1.0lf - k[0]) * c + k[0] * d;
    textureStore(output, id.xy, vec4<f32>((1.0lf - k[1]) * top + k[1] * bottom));
}

// B-76, layer_fx::light_rays' last step (layer_fx::add_rays): `other` is the bright pixels
// zoomed out. k: the intensity, the colour in linear light, (D-422, Light Burst's Set Color) 1
// when the rays' covering stands for their colour, else 0, and (D-423) the mode, 0 D-124's add.
@compute @workgroup_size(16, 16)
fn rays(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let r = textureLoad(other, id.xy, 0);
    let lit = select(r, vec4(r.w), k[4] == 1.0lf);
    let mode = u32(k[5]);
    let a = min(k[0] * f64(r.w), 1.0lf);
    var out: vec4<f32>;
    for (var c = 0u; c < 3u; c++) {
        let q = k[0] * k[1u + c] * f64(lit[c]);
        let o = f64(p[c]);
        if mode == 1u {
            out[c] = f32(q + o * (1.0lf - a));
        } else if mode == 2u {
            out[c] = f32(o + q - o * q);
        } else if mode == 3u {
            out[c] = f32(max(o, q));
        } else {
            out[c] = f32(f64(p[c]) + k[0] * k[1u + c] * f64(lit[c]));
        }
    }
    let o = f64(p.w);
    if mode == 1u {
        out.w = f32(a + o * (1.0lf - a));
    } else if mode == 2u {
        out.w = f32(o + a - o * a);
    } else if mode == 3u {
        out.w = f32(max(o, a));
    } else {
        out.w = f32(min(f64(p.w) + k[0] * f64(r.w), 1.0lf));
    }
    textureStore(output, id.xy, out);
}

// D-123: the drawing inside a ring of edge one wide, `(x, y)` in the ring's pixels; covered is
// inside and at least half covered.
fn covered(x: i32, y: i32, size: vec2<i32>) -> bool {
    return x >= 1 && y >= 1 && x <= size.x && y <= size.y && textureLoad(input, vec2(x - 1, y - 1), 0).w >= 0.5;
}

// B-76, layer_fx::distance_gradation's columns, one a thread: each place's squared distance to
// the nearest place not covered, whole numbers, exact, into `dist`.
@compute @workgroup_size(64)
fn cols(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(textureDimensions(input));
    let pw = size.x + 2;
    let ph = size.y + 2;
    let x = i32(id.x);
    if x >= pw {
        return;
    }
    // The ring's first and last rows are not covered, so both sweeps meet one.
    var last = 0.0lf;
    for (var y = 0; y < ph; y++) {
        if !covered(x, y, size) {
            last = f64(y);
        }
        dist[u32(y * pw + x)] = f64(y) - last;
    }
    var next = f64(ph - 1);
    for (var y = ph - 1; y >= 0; y--) {
        if !covered(x, y, size) {
            next = f64(y);
        }
        let d = min(dist[u32(y * pw + x)], next - f64(y));
        dist[u32(y * pw + x)] = d * d;
    }
}

// Where the parabolas rooted at `a` and `b` (a > b) along a row cross, as a numerator and a
// positive denominator, so comparing two is exact where the CPU's quotients are.
fn crossing(fo: u32, a: u32, b: u32) -> vec2<f64> {
    let fa = dist[fo + a] + f64(a) * f64(a);
    let fb = dist[fo + b] + f64(b) * f64(b);
    return vec2(fa - fb, 2.0lf * f64(a - b));
}

// layer_fx::distance_gradation's rows, one a thread: the lower envelope of the columns'
// parabolas, its roots kept past the columns in `dist`, then each pixel shaded. k: the width,
// the opacity, invert, the colour in linear light; `blend` the mix.
@compute @workgroup_size(64)
fn edt(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    let y = id.x;
    if y >= size.y {
        return;
    }
    let pw = size.x + 2u;
    let fo = (y + 1u) * pw;
    let vo = pw * (size.y + 2u) + y * pw;
    var top = 0u;
    dist[vo] = 0.0lf;
    for (var q = 1u; q < pw; q++) {
        loop {
            let v = u32(dist[vo + top]);
            if top > 0u {
                let s = crossing(fo, q, v);
                let z = crossing(fo, v, u32(dist[vo + top - 1u]));
                if s.x * z.y <= z.x * s.y {
                    top--;
                    continue;
                }
            }
            top++;
            dist[vo + top] = f64(q);
            break;
        }
    }
    var j = 0u;
    for (var x = 0u; x < size.x; x++) {
        let q = x + 1u;
        while j < top {
            let z = crossing(fo, u32(dist[vo + j + 1u]), u32(dist[vo + j]));
            if z.x >= f64(q) * z.y {
                break;
            }
            j++;
        }
        let v = u32(dist[vo + j]);
        let d = f64(q) - f64(v);
        let sq = d * d + dist[fo + v];
        let p = textureLoad(input, vec2(x, y), 0);
        let a = f64(p.w);
        if a <= 0.0lf {
            textureStore(output, vec2(x, y), p);
            continue;
        }
        let t = clamp(1.0lf - (sqrt(sq) - 0.5lf) / k[0], 0.0lf, 1.0lf);
        let o = select(t, 1.0lf - t, k[2] == 1.0lf) * k[1] / 100.0lf;
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            let b = f64(p[c]) / a;
            out[c] = f32((b + o * (mixed(b, k[3u + c]) - b)) * a);
        }
        textureStore(output, vec2(x, y), out);
    }
}

// B-76, render::light_wrap: `input` is the layer placed on the frame, `other` its light grown
// by `r`. k: the intensity as a share; `flag` is Add.
@compute @workgroup_size(16, 16)
fn lightwrap(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let m = textureLoad(other, vec2<i32>(id.xy) + vec2(F.r), 0);
    let reach = k[0] * (1.0lf - f64(m.w));
    let lit = vec3(reach * f64(m.x), reach * f64(m.y), reach * f64(m.z));
    if all(lit == vec3(0.0lf)) {
        textureStore(output, id.xy, p);
        return;
    }
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let v = f64(p[c]) / a;
        var n = 1.0lf - (1.0lf - v) * (1.0lf - clamp(lit[c], 0.0lf, 1.0lf));
        if F.flag == 1u {
            n = v + lit[c];
        }
        out[c] = f32(n * a);
    }
    textureStore(output, id.xy, out);
}

// B-107: sine and cosine in double precision. Whole quarter turns come off in three parts
// (fdlibm's), exact for fewer than 2^20 of them, and the rest, within an eighth of a turn, is
// summed as its series.
// ponytail: past 2^20 quarter turns (a wave phase of about 1.6 million radians) the reduction
// loses digits; a longer reduction if phases that large are ever used.
fn quarters(x: f64) -> f64 {
    return floor(x * 0.6366197723675814lf + 0.5lf);
}

fn reduced(x: f64, n: f64) -> f64 {
    return ((x - n * 1.57079632673412561417lf) - n * 6.07710050630396597660e-11lf) - n * 2.02226624879595063154e-21lf;
}

fn sin_series(r: f64) -> f64 {
    let s = r * r;
    return r * (1.0lf + s * (-0.16666666666666666lf + s * (0.008333333333333333lf + s * (-0.0001984126984126984lf
        + s * (2.7557319223985893e-06lf + s * (-2.505210838544172e-08lf + s * (1.6059043836821613e-10lf
        + s * (-7.647163731819816e-13lf + s * 2.8114572543455206e-15lf))))))));
}

fn cos_series(r: f64) -> f64 {
    let s = r * r;
    return 1.0lf + s * (-0.5lf + s * (0.041666666666666664lf + s * (-0.001388888888888889lf + s * (2.48015873015873e-05lf
        + s * (-2.755731922398589e-07lf + s * (2.08767569878681e-09lf + s * (-1.1470745597729725e-11lf
        + s * (4.779477332387385e-14lf - s * 1.5619206968586225e-16lf))))))));
}

fn turns(n: f64) -> i32 {
    return ((i32(n) % 4) + 4) % 4;
}

fn sin64(x: f64) -> f64 {
    let n = quarters(x);
    let r = reduced(x, n);
    switch turns(n) {
        case 0: { return sin_series(r); }
        case 1: { return cos_series(r); }
        case 2: { return -sin_series(r); }
        default: { return -cos_series(r); }
    }
}

fn cos64(x: f64) -> f64 {
    let n = quarters(x);
    let r = reduced(x, n);
    switch turns(n) {
        case 0: { return cos_series(r); }
        case 1: { return -sin_series(r); }
        case 2: { return -cos_series(r); }
        default: { return sin_series(r); }
    }
}

// asin(sin(x)), the triangle wave, folded from the angle.
fn folded(x: f64) -> f64 {
    let n = quarters(x);
    let r = reduced(x, n);
    switch turns(n) {
        case 0: { return r; }
        case 1: { return 1.5707963267948966lf - abs(r); }
        case 2: { return -r; }
        default: { return abs(r) - 1.5707963267948966lf; }
    }
}

// atan2 in double precision: the straight and diagonal directions exact, the rest a single
// precision guess taken to double by Newton's steps on x sin t = y cos t. Both zero is 0.
fn atan2_64(y: f64, x: f64) -> f64 {
    if y == 0.0lf {
        return select(0.0lf, 3.141592653589793lf, x < 0.0lf);
    }
    if x == 0.0lf {
        return select(-1.5707963267948966lf, 1.5707963267948966lf, y > 0.0lf);
    }
    if abs(x) == abs(y) {
        let t = select(2.356194490192345lf, 0.7853981633974483lf, x > 0.0lf);
        return select(-t, t, y > 0.0lf);
    }
    var t = f64(atan2(f32(y), f32(x)));
    for (var i = 0; i < 2; i++) {
        let s = sin64(t);
        let c = cos64(t);
        t = t - (s * x - c * y) / (c * x + s * y);
    }
    return t;
}

// f64::rem_euclid for m > 0: the remainder worked exactly, as `%` is, then made positive.
fn euclid(a: f64, m: f64) -> f64 {
    var r = a - m * trunc(a / m);
    if a >= 0.0lf && r < 0.0lf {
        r += m;
    } else if a < 0.0lf && r > 0.0lf {
        r -= m;
    }
    return select(r, r + m, r < 0.0lf);
}

fn luma(e: vec3<f64>) -> f64 {
    return 0.2126lf * e.x + 0.7152lf * e.y + 0.0722lf * e.z;
}

// grade::hsv_hue, -1 for a grey.
fn hsv_hue(e: vec3<f64>) -> f64 {
    let mx = max(max(e.x, e.y), e.z);
    let mn = min(min(e.x, e.y), e.z);
    if mx == mn {
        return -1.0lf;
    }
    let d = mx - mn;
    if e.x == mx {
        return euclid(60.0lf * ((e.y - e.z) / d), 360.0lf);
    }
    if e.y == mx {
        return 60.0lf * ((e.z - e.x) / d + 2.0lf);
    }
    return 60.0lf * ((e.x - e.y) / d + 4.0lf);
}

// B-123, lut::Cube::lookup of the encoded colour `e`. k: 1 for a 3-D table, its size, the
// domain's low ends and high ends, and the table, red changing fastest.
fn lookup(e: vec3<f64>) -> vec3<f64> {
    let n = u32(k[1]);
    var i: vec3<u32>;
    var f: vec3<f64>;
    for (var c = 0u; c < 3u; c++) {
        let u = (clamp(e[c], k[2u + c], k[5u + c]) - k[2u + c]) / (k[5u + c] - k[2u + c]) * f64(n - 1u);
        let fl = min(max(floor(u), 0.0lf), f64(n - 2u));
        i[c] = u32(fl);
        f[c] = u - fl;
    }
    var out = vec3(0.0lf);
    if k[0] == 0.0lf {
        for (var c = 0u; c < 3u; c++) {
            out[c] = k[8u + 3u * i[c] + c] * (1.0lf - f[c]) + k[11u + 3u * i[c] + c] * f[c];
        }
        return out;
    }
    for (var db = 0u; db < 2u; db++) {
        for (var dg = 0u; dg < 2u; dg++) {
            for (var dr = 0u; dr < 2u; dr++) {
                let w = select(1.0lf - f.x, f.x, dr == 1u) * select(1.0lf - f.y, f.y, dg == 1u) * select(1.0lf - f.z, f.z, db == 1u);
                let j = 8u + 3u * ((i.x + dr) + (i.y + dg) * n + (i.z + db) * n * n);
                out += w * vec3(k[j], k[j + 1u], k[j + 2u]);
            }
        }
    }
    return out;
}

// B-123, hsv_key's 8-bit step of the straight channel `v` over `a`, as color::linear_to_srgb
// and quantise_u8 work it in single precision, each operation rounded once as the CPU rounds
// it. The power is `root`'s, moved by k[7], the single-precision 1/2.4 less the true one, times
// the logarithm. k[8] is a 0 for `product`.
fn level8(v: f32, a: f64) -> f64 {
    return step8(srgb32(v, a));
}

// B-223: `level8`'s color::linear_to_srgb of `v` over `a`, before its rounding.
fn srgb32(v: f32, a: f64) -> f32 {
    let c = f32(f64(v) / a);
    if c <= 0.0031308 {
        return f32(f64(12.92f) * f64(c));
    }
    let y = f32(root(f64(c)) * (1.0lf + k[7] * f64(log(c))));
    return f32(f64(f32(f64(1.055f) * f64(y))) - f64(0.055f));
}

// B-223: `level8`'s color::quantise_u8 of `s`.
fn step8(s: f32) -> f64 {
    let t = f32(f64(clamp(s, 0.0, 1.0)) * 255.0lf);
    return floor(f64(f32(f64(t) + 0.5lf)));
}

// B-123, hsv_key's windows. k: the hue and its range, the saturation and its range, the value and
// its range.
fn hsv_inside(p: vec4<f32>, a: f64) -> bool {
    let r = quotient(level8(p.x, a), 255.0lf);
    let g = quotient(level8(p.y, a), 255.0lf);
    let b = quotient(level8(p.z, a), 255.0lf);
    let hi = max(max(r, g), b);
    let lo = min(min(r, g), b);
    var s = 0.0lf;
    if hi != 0.0lf {
        s = quotient(hi - lo, hi);
    }
    var h = 0.0lf;
    if s != 0.0lf {
        let d = hi - lo;
        if r == hi {
            h = quotient(g - b, d);
        } else if g == hi {
            h = 2.0lf + quotient(b - r, d);
        } else {
            h = 4.0lf + quotient(r - g, d);
        }
        h = product(h, 60.0lf, k[8]);
        if h < 0.0lf {
            h = h + 360.0lf;
        }
    }
    let dh = abs(h - k[0]);
    return k[1] - min(dh, 360.0lf - dh) >= 0.0lf
        && k[3] - abs(product(100.0lf, s, k[8]) - k[2]) >= 0.0lf
        && k[5] - abs(product(100.0lf, hi, k[8]) - k[4]) >= 0.0lf;
}

// B-222, cel_fx's 8-bit red, green and blue of the pixel, as `level8` works each.
fn q8(p: vec4<f32>, a: f64) -> vec3<f64> {
    return vec3(level8(p.x, a), level8(p.y, a), level8(p.z, a));
}

// B-222, cel_fx::hue of 8-bit `q`, -1 for a grey, each step rounded as the CPU rounds it.
fn hue8(q: vec3<f64>) -> f64 {
    let hi = max(max(q.x, q.y), q.z);
    let c = hi - min(min(q.x, q.y), q.z);
    if c == 0.0lf {
        return -1.0lf;
    }
    if hi == q.x {
        return product(60.0lf, euclid(quotient(q.y - q.z, c), 6.0lf), k[8]);
    }
    if hi == q.y {
        return product(60.0lf, quotient(q.z - q.x, c) + 2.0lf, k[8]);
    }
    return product(60.0lf, quotient(q.x - q.y, c) + 4.0lf, k[8]);
}

// B-222, selective_blur::chosen of a pixel that shows: its 8-bit `q` within the tolerance k[1]
// of one of the k[0] targets, from k[9] three a target.
fn picked(q: vec3<f64>) -> bool {
    for (var i = 0u; i < u32(k[0]); i++) {
        let t = vec3(k[9u + 3u * i], k[10u + 3u * i], k[11u + 3u * i]);
        if all(abs(q - t) <= vec3(k[1])) {
            return true;
        }
    }
    return false;
}

// B-222, grade::hls: the HSV hue (-1 for a grey), the lightness and the saturation of `e`.
// D-468/D-469, grade::channel_threshold's decision on `v` for channel `c`: k[c] the level, k[4 + c]
// 1 to turn it over.
fn th_decide(v: f64, c: u32) -> f64 {
    return select(0.0lf, 1.0lf, (255.0lf * v + 1e-4lf >= k[c]) != (k[4u + c] != 0.0lf));
}

fn hls(e: vec3<f64>) -> vec3<f64> {
    let mx = max(max(e.x, e.y), e.z);
    let mn = min(min(e.x, e.y), e.z);
    let l = (mx + mn) / 2.0lf;
    var s = 0.0lf;
    if mx != mn {
        s = (mx - mn) / (1.0lf - abs(2.0lf * l - 1.0lf));
    }
    return vec3(hsv_hue(e), l, s);
}

// D-459, grade::cdk_mattes' hue window: 0 within 60 degrees of `centre`, 1 from 120.
fn cdk_window(h: f64, centre: f64) -> f64 {
    let d = euclid(h - centre, 360.0lf);
    return clamp((min(d, 360.0lf - d) - 60.0lf) / 60.0lf, 0.0lf, 1.0lf);
}

// D-459, grade::cdk_level on the five levels at k[i]; the power in single precision, as
// Levels'.
fn cdk_level(t: f64, i: u32) -> f64 {
    let x = 255.0lf * t;
    var v = select(0.0lf, 1.0lf, x >= k[i]);
    if k[i + 1u] != k[i] {
        v = clamp((x - k[i]) / (k[i + 1u] - k[i]), 0.0lf, 1.0lf);
    }
    var g = 0.0lf;
    if v > 0.0lf {
        g = f64(pow(f32(v), f32(1.0lf / k[i + 2u])));
    }
    return (k[i + 3u] + g * (k[i + 4u] - k[i + 3u])) / 255.0lf;
}

// D-370, grade::change_distance: how far `e` is from `c`, by the match's place in
// CHANGE_MATCHES; 1 by hue when either is a grey.
fn match_distance(e: vec3<f64>, c: vec3<f64>, kind: f64) -> f64 {
    if kind == 0.0lf {
        let v = e - c;
        return sqrt((v.x * v.x + v.y * v.y + v.z * v.z) / 3.0lf);
    }
    if kind == 2.0lf {
        let ye = luma(e);
        let yc = luma(c);
        let db = (e.z - ye) / 1.8556lf - (c.z - yc) / 1.8556lf;
        let dr = (e.x - ye) / 1.5748lf - (c.x - yc) / 1.5748lf;
        return sqrt(db * db + dr * dr);
    }
    let he = hsv_hue(e);
    let hc = hsv_hue(c);
    if he < 0.0lf || hc < 0.0lf {
        return 1.0lf;
    }
    let dd = abs(he - hc);
    return min(dd, 360.0lf - dd) / 180.0lf;
}

// D-370, grade::nearness.
fn nearness(d: f64, t: f64, w: f64) -> f64 {
    if d <= t {
        return 1.0lf;
    }
    if w == 0.0lf || d >= t + w {
        return 0.0lf;
    }
    return 1.0lf - (d - t) / w;
}

// B-222, change_to_color's share of nearness `d` inside tolerance `t`, softness `w` beyond it.
fn part(d: f64, t: f64, w: f64) -> f64 {
    if d <= t {
        return 1.0lf;
    }
    if w == 0.0lf || t == 0.0lf || d >= t * (1.0lf + w) {
        return 0.0lf;
    }
    return 1.0lf - (d - t) / (t * w);
}

// D-398, grade::key_band: 1 from `lo` to `hi`, falling to 0 over `soft` outside them, eased.
fn key_band(x: f64, lo: f64, hi: f64, soft: f64) -> f64 {
    var d = 0.0lf;
    if x < lo {
        d = lo - x;
    } else if x > hi {
        d = x - hi;
    }
    if d <= 0.0lf {
        return 1.0lf;
    }
    if soft == 0.0lf {
        return 0.0lf;
    }
    let t = clamp(d / soft, 0.0lf, 1.0lf);
    return 1.0lf - t * t * (3.0lf - 2.0lf * t);
}

// D-571, grade::hue_curve (`circle`) or grade::line_curve at `x0`. k: the count, the first
// numbers, then the second.
fn grade_curve(x0: f64, circle: bool) -> f64 {
    let n = u32(k[0]);
    if !circle {
        if x0 <= k[1] {
            return k[1u + n];
        }
        if x0 >= k[n] {
            return k[2u * n];
        }
    }
    for (var i = 0u; i < n && n > 1u; i++) {
        let j = (i + 1u) % n;
        if j == 0u && !circle {
            break;
        }
        let h0 = k[1u + i];
        var h1 = k[1u + j];
        if j == 0u {
            h1 += 360.0lf;
        }
        var x = x0;
        if x < h0 && circle {
            x += 360.0lf;
        }
        if h0 <= x && x < h1 {
            let u = (x - h0) / (h1 - h0);
            return k[1u + n + i] + (k[1u + n + j] - k[1u + n + i]) * u * u * (3.0lf - 2.0lf * u);
        }
    }
    return k[select(1u + n, 2u * n, !circle)];
}

// grade::from_hls: an encoded colour from its hue in degrees, lightness and saturation.
fn from_hls(hue: f64, l: f64, s: f64) -> vec3<f64> {
    let chroma = (1.0lf - abs(2.0lf * l - 1.0lf)) * s;
    let hh = hue / 60.0lf;
    let x = chroma * (1.0lf - abs(euclid(hh, 2.0lf) - 1.0lf));
    var rgb: vec3<f64>;
    switch min(u32(floor(hh)), 5u) {
        case 0u: { rgb = vec3(chroma, x, 0.0lf); }
        case 1u: { rgb = vec3(x, chroma, 0.0lf); }
        case 2u: { rgb = vec3(0.0lf, chroma, x); }
        case 3u: { rgb = vec3(0.0lf, x, chroma); }
        case 4u: { rgb = vec3(x, 0.0lf, chroma); }
        default: { rgb = vec3(chroma, 0.0lf, x); }
    }
    return rgb + vec3(l - chroma / 2.0lf);
}

// B-107, the third batch's colour effects, a pixel a thread: grade::invert (mode 0, `count` the
// channel, 3 all), invert_alpha (1), brightness_contrast (2), black_white (3), posterize (4),
// threshold (5), channel_mixer (6), vibrance (7), leave_color (8), solarize (9) and halftone
// (10); and (B-123) color_lookup (11), hsv_key (12) and paraffin (13); B-222: exposure with an
// offset or gamma (14), tint (15), shift_channels (16), solid_composite (17), change_to_color
// (18), color_key (19), select_color (20), line_recolor (21), colorama (22) and extract (23);
// broadcast_safe (24), color_neutralizer (25), color_offset (26); D-369/D-370: toner (27) and
// change_color (28); D-374/D-375: color_balance_hls (29) and color link (30); D-382:
// gamma_pedestal_gain (31).
@compute @workgroup_size(16, 16)
fn tone(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if F.mode == 1u {
        // k: the amount as a share.
        var b = vec3(0.0lf);
        if a > 0.0lf {
            b = vec3<f64>(p.xyz) / a;
        }
        let n = a + k[0] * (1.0lf - 2.0lf * a);
        textureStore(output, id.xy, vec4(vec3<f32>(b * n), f32(n)));
        return;
    }
    if F.mode == 14u {
        // k: the gain, the offset, one over the gamma. The power in single precision, as Levels'.
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            if a <= 0.0lf {
                out[c] = p[c] * f32(k[0]);
                continue;
            }
            let u = f64(max(p[c] / p.w, 0.0)) * k[0] + k[1];
            var v = 0.0lf;
            if u != 0.0lf {
                v = sign(u) * f64(pow(f32(abs(u)), f32(k[2])));
            }
            out[c] = f32(v) * p.w;
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 16u {
        // k: where red, green, blue and alpha are taken from, each its place in SHIFT_CHANNELS_FROM.
        var e = vec3(0.0lf);
        if a > 0.0lf {
            for (var c = 0u; c < 3u; c++) {
                e[c] = to_srgb(clamp(f64(p[c]) / a, 0.0lf, 1.0lf));
            }
        }
        let mx = max(max(e.x, e.y), e.z);
        let mn = min(min(e.x, e.y), e.z);
        let l = (mx + mn) / 2.0lf;
        var o: vec4<f64>;
        for (var c = 0u; c < 4u; c++) {
            let w = u32(k[c]);
            var v = 0.0lf;
            switch w {
                case 0u: { v = a; }
                case 1u, 2u, 3u: { v = e[w - 1u]; }
                case 4u: { v = luma(e); }
                case 5u: { v = max(hsv_hue(e), 0.0lf) / 360.0lf; }
                case 6u: { v = l; }
                case 7u: {
                    if mx > mn {
                        v = (mx - mn) / (1.0lf - abs(2.0lf * l - 1.0lf));
                    }
                }
                case 8u: { v = 1.0lf; }
                case 9u: { v = 0.5lf; }
                default: {}
            }
            o[c] = clamp(v, 0.0lf, 1.0lf);
        }
        textureStore(output, id.xy, vec4(f32(to_linear(o.x) * o.w), f32(to_linear(o.y) * o.w), f32(to_linear(o.z) * o.w), f32(o.w)));
        return;
    }
    if F.mode == 17u {
        // k: the layer's share and the solid's opacity, then the solid in linear light times
        // it, all single precision as the CPU's; `blend` 1 multiply, 2 screen, 3 add.
        let s = p * f32(k[0]);
        let d = vec4(f32(k[2]), f32(k[3]), f32(k[4]), f32(k[1]));
        if F.blend == 0u {
            textureStore(output, id.xy, s + d * (1.0 - s.w));
            return;
        }
        var cs = vec3(0.0);
        if s.w != 0.0 {
            cs = s.xyz / s.w;
        }
        var cd = vec3(0.0);
        if d.w != 0.0 {
            cd = d.xyz / d.w;
        }
        var b = min(cs + cd, vec3(1.0));
        if F.blend == 1u {
            b = cs * cd;
        } else if F.blend == 2u {
            b = cs + cd - cs * cd;
        }
        let rgb = (1.0 - s.w) * d.xyz + (1.0 - d.w) * s.xyz + s.w * d.w * b;
        textureStore(output, id.xy, vec4(rgb, s.w + d.w - s.w * d.w));
        return;
    }
    if F.mode == 20u {
        // k: as `picked` reads it, 1 at k[3] to keep the chosen, and level8's at k[7].
        let chosen = a > 0.0lf && picked(q8(p, a));
        textureStore(output, id.xy, select(p, vec4(0.0), chosen != (k[3] != 0.0lf)));
        return;
    }
    if F.mode == 30u {
        // D-375, as frame_stats::link. k: the colour encoded, the opacity as a share, 1 for the
        // stencil; `blend` the mode as grade::mixer. Without the stencil the empty pixels take
        // the colour too.
        if k[4] != 0.0lf && a <= 0.0lf {
            textureStore(output, id.xy, p);
            return;
        }
        var e = vec3(0.0lf);
        if a > 0.0lf {
            for (var c = 0u; c < 3u; c++) {
                e[c] = to_srgb(clamp(f64(p[c]) / a, 0.0lf, 1.0lf));
            }
        }
        let o = k[3];
        var a2 = a;
        if k[4] == 0.0lf {
            a2 = o + a * (1.0lf - o);
        }
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            let f = mixed(e[c], k[c]);
            var r = e[c] + o * (f - e[c]);
            if k[4] == 0.0lf {
                r = (o * (1.0lf - a) * k[c] + o * a * f + (1.0lf - o) * a * e[c]) / a2;
            }
            out[c] = f32(to_linear(clamp(r, 0.0lf, 1.0lf)) * a2);
        }
        out.w = f32(a2);
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 22u {
        // D-316 and D-381, as grade::colorama without its layers, which keep it on the processor.
        // k: the phase read (its place in COLORAMA_GET), the repetitions, the shift in turns,
        // the colours in the ring, the blend as a share; the ring encoded in five places, then
        // its five opacities as shares; the modify's place in COLORAMA_MODIFY, 1 each for
        // modify alpha, change empty and interpolate; the match's place in CHANGE_MATCHES or
        // -1, its colour encoded, tolerance and softness as shares; 1 to composite.
        if a <= 0.0lf && (k[26] == 0.0lf || k[27] == 0.0lf) {
            textureStore(output, id.xy, p);
            return;
        }
        var b = vec3(0.0lf);
        var e = vec3(0.0lf);
        if a > 0.0lf {
            b = vec3<f64>(p.xyz) / a;
            for (var c = 0u; c < 3u; c++) {
                e[c] = to_srgb(clamp(b[c], 0.0lf, 1.0lf));
            }
        }
        var ph = (e.x + e.y + e.z) / 3.0lf;
        switch u32(k[0]) {
            case 1u: { ph = to_srgb(luma(clamp(b, vec3(0.0lf), vec3(1.0lf)))); }
            case 2u, 3u, 4u: { ph = e[u32(k[0]) - 2u]; }
            case 5u: { ph = a; }
            case 6u: { ph = hsl(e).x / 360.0lf; }
            case 7u: { ph = hsl(e).z; }
            case 8u: { ph = hsl(e).y; }
            case 9u: { ph = max(max(e.x, e.y), e.z); }
            case 10u: { ph = 0.0lf; }
            default: {}
        }
        let t = ph * k[1] + k[2];
        let q = (t - floor(t)) * k[3];
        let n = u32(k[3]);
        let i = u32(floor(q)) % n;
        let j = (i + 1u) % n;
        var w = q - floor(q);
        if k[28] == 0.0lf {
            w = 0.0lf;
        }
        var m: vec3<f64>;
        for (var c = 0u; c < 3u; c++) {
            let lo = k[5u + 3u * i + c];
            m[c] = lo + w * (k[5u + 3u * j + c] - lo);
        }
        var wt = 1.0lf;
        if k[29] >= 0.0lf {
            wt = nearness(match_distance(e, vec3(k[30], k[31], k[32]), k[29]), k[33], k[34]);
        }
        var out = p;
        if k[25] == 0.0lf && k[26] == 0.0lf && wt == 1.0lf {
            for (var c = 0u; c < 3u; c++) {
                let g = to_linear(m[c]);
                out[c] = f32((g + k[4] * (b[c] - g)) * a);
            }
            textureStore(output, id.xy, out);
            return;
        }
        var g = m;
        let mo = u32(k[25]);
        if mo >= 1u && mo <= 3u {
            var v = hsl(e);
            let hm = hsl(m);
            if mo == 1u {
                v.x = hm.x;
            } else if mo == 2u {
                v.z = hm.z;
            } else {
                v.y = hm.y;
            }
            g = from_hsl(v);
        } else if mo >= 4u {
            g = e;
            if mo <= 6u {
                g[mo - 4u] = m[mo - 4u];
            }
        }
        var a2 = a;
        if k[26] != 0.0lf {
            a2 = k[20u + i] + w * (k[20u + j] - k[20u + i]);
        }
        var r: vec4<f64>;
        for (var c = 0u; c < 3u; c++) {
            r[c] = to_linear(clamp(g[c], 0.0lf, 1.0lf)) * a2;
        }
        r.w = a2;
        for (var c = 0u; c < 4u; c++) {
            let was = f64(p[c]);
            var laid = wt * r[c];
            if k[35] != 0.0lf {
                laid = was + wt * (r[c] - was);
            }
            out[c] = f32(laid + k[4] * (was - laid));
        }
        textureStore(output, id.xy, out);
        return;
    }
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    if F.mode == 110u {
        // D-562, grade::cc_vignette. k: the centre, tan(angle of view / 2) over half the
        // drawing's diagonal, the amount and the pin highlights as shares.
        let dx = f64(id.x) + 0.5lf - k[0];
        let dy = f64(id.y) + 0.5lf - k[1];
        let s2 = (dx * dx + dy * dy) * k[2] * k[2];
        let g = f64(pow(f32(1.0lf + s2), f32(-2.0lf * k[3])));
        var b: vec3<f64>;
        for (var c = 0u; c < 3u; c++) {
            b[c] = f64(p[c]) / a;
        }
        let e = vec3(to_srgb(clamp(b.x, 0.0lf, 1.0lf)), to_srgb(clamp(b.y, 0.0lf, 1.0lf)), to_srgb(clamp(b.z, 0.0lf, 1.0lf)));
        let gain = g + (1.0lf - g) * luma(e) * k[4];
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(min(b[c] * gain, max(b[c], 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 100u {
        // D-468/D-469, grade::channel_threshold. k: the three levels, the channel (0 luminance,
        // 1 rgb, 2 saturation, 3 alpha), 1 to turn each of the three over, and the share of the
        // decision kept (1 less the blend over 100).
        var b: vec3<f64>;
        for (var c = 0u; c < 3u; c++) {
            b[c] = clamp(f64(p[c]) / a, 0.0lf, 1.0lf);
        }
        if k[3] == 3.0lf {
            let n = a + k[7] * (th_decide(a, 0u) - a);
            textureStore(output, id.xy, vec4(f32(b.x * n), f32(b.y * n), f32(b.z * n), f32(n)));
            return;
        }
        let e = vec3(to_srgb(b.x), to_srgb(b.y), to_srgb(b.z));
        var m = vec3(th_decide(luma(e), 0u));
        if k[3] == 1.0lf {
            m = vec3(th_decide(e.x, 0u), th_decide(e.y, 1u), th_decide(e.z, 2u));
        } else if k[3] == 2.0lf {
            m = vec3(th_decide(hls(e).z, 0u));
        }
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32((b[c] + k[7] * (m[c] - b[c])) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 12u {
        // k: as hsv_inside and level8 read it, and 1 to invert at k[6].
        textureStore(output, id.xy, select(p, vec4(0.0), hsv_inside(p, a) != (k[6] != 0.0lf)));
        return;
    }
    if F.mode == 15u {
        // k: the amount, the colour; single precision as the CPU's.
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            let s = p[c] / p.w;
            out[c] = (s + (f32(k[1u + c]) - s) * f32(k[0])) * p.w;
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 19u {
        // k: as `picked` reads it, the softness at k[2], 1 at k[3] to match by hue, and level8's
        // at k[7].
        let q = q8(p, a);
        let h = hue8(q);
        var d = 1e300lf;
        for (var i = 0u; i < u32(k[0]); i++) {
            let t = vec3(k[9u + 3u * i], k[10u + 3u * i], k[11u + 3u * i]);
            var di = 255.0lf;
            if k[3] == 0.0lf {
                let g = abs(q - t);
                di = max(max(g.x, g.y), g.z);
            } else {
                let th = hue8(t);
                if h >= 0.0lf && th >= 0.0lf {
                    let dh = abs(h - th);
                    di = quotient(product(min(dh, 360.0lf - dh), 255.0lf, k[8]), 180.0lf);
                }
            }
            d = min(d, di);
        }
        var keep = 1.0lf;
        if d <= k[1] {
            keep = 0.0lf;
        } else if k[2] != 0.0lf {
            keep = min((d - k[1]) / k[2], 1.0lf);
        }
        textureStore(output, id.xy, select(p, p * f32(keep), keep < 1.0lf));
        return;
    }
    if F.mode == 21u {
        // k: as `picked` reads it, the new colour in linear light, single precision, at k[4],
        // and level8's at k[7].
        if picked(q8(p, a)) {
            textureStore(output, id.xy, vec4(f32(k[4]) * p.w, f32(k[5]) * p.w, f32(k[6]) * p.w, p.w));
            return;
        }
        textureStore(output, id.xy, p);
        return;
    }
    let px = vec3<f64>(p.xyz);
    var e: vec3<f64>;
    for (var c = 0u; c < 3u; c++) {
        e[c] = to_srgb(clamp(px[c] / a, 0.0lf, 1.0lf));
    }
    var out = p;
    if F.mode == 18u {
        // k: from and to encoded, the hue, lightness and saturation tolerances and the softness
        // as shares, then 1 each for transforming, changing lightness, saturation, and the matte.
        let f = hls(vec3(k[0], k[1], k[2]));
        let g = hls(vec3(k[3], k[4], k[5]));
        let h = hls(e);
        var dh = 1.0lf;
        if h.x >= 0.0lf && f.x >= 0.0lf {
            let d = abs(h.x - f.x);
            dh = min(d, 360.0lf - d) / 180.0lf;
        }
        let m = min(min(part(dh, k[6], k[9]), part(abs(h.y - f.y), k[7], k[9])), part(abs(h.z - f.z), k[8], k[9]));
        if k[13] != 0.0lf {
            let v = f32(to_linear(m) * a);
            textureStore(output, id.xy, vec4(v, v, v, p.w));
            return;
        }
        if m <= 0.0lf {
            textureStore(output, id.xy, p);
            return;
        }
        let hf = max(f.x, 0.0lf);
        let ht = max(g.x, 0.0lf);
        var hue = ht;
        var l = h.y;
        var s = h.z;
        if k[10] != 0.0lf {
            hue = euclid(max(h.x, 0.0lf) + ht - hf, 360.0lf);
            if k[11] != 0.0lf {
                l = clamp(l + g.y - f.y, 0.0lf, 1.0lf);
            }
            if k[12] != 0.0lf {
                s = clamp(s + g.z - f.z, 0.0lf, 1.0lf);
            }
        } else {
            if k[11] != 0.0lf {
                l = g.y;
            }
            if k[12] != 0.0lf {
                s = g.z;
            }
        }
        let rgb = from_hls(hue, l, s);
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(to_linear(clamp(e[c] + m * (rgb[c] - e[c]), 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }

    if F.mode == 23u {
        // k: the channel (0 red, 1 green, 2 blue, 3 alpha, 4 luminance), the black and white
        // points and their softnesses, 1 to invert.
        var v = 255.0lf * luma(e);
        if k[0] == 3.0lf {
            v = 255.0lf * a;
        } else if k[0] < 3.0lf {
            v = 255.0lf * e[u32(k[0])];
        }
        var low = select(0.0lf, 1.0lf, v + 1e-4lf >= k[1]);
        if k[3] > 0.0lf {
            low = clamp((v - k[1]) / k[3], 0.0lf, 1.0lf);
        }
        var high = select(0.0lf, 1.0lf, v - 1e-4lf <= k[2]);
        if k[4] > 0.0lf {
            high = clamp((k[2] - v) / k[4], 0.0lf, 1.0lf);
        }
        var m = low * high;
        if k[5] != 0.0lf {
            m = 1.0lf - m;
        }
        // A channel at a time: a whole vec4<f64> turned to f32 came out wrong on this card.
        for (var c = 0u; c < 4u; c++) {
            out[c] = f32(f64(p[c]) * m);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 70u {
        // D-459, as grade::color_difference_key; k: grade::cdk_settings.
        if k[0] == 1.0lf {
            textureStore(output, id.xy, p);
            return;
        }
        let h = hls(e);
        let hue = max(h.x, 0.0lf);
        let ta = h.z * cdk_window(hue, k[1] - 60.0lf) + (1.0lf - h.z) / 2.0lf;
        let tb = h.z * cdk_window(hue, k[1]) + (1.0lf - h.z) / 2.0lf;
        var m: array<f64, 6>;
        if h.y < 0.5lf {
            m[0] = 2.0lf * ta * h.y;
            m[2] = 1.0lf - 2.0lf * (1.0lf - tb) * h.y;
        } else {
            m[0] = 1.0lf - 2.0lf * (1.0lf - ta) * (1.0lf - h.y);
            m[2] = 2.0lf * tb * (1.0lf - h.y);
        }
        m[1] = cdk_level(m[0], 2u);
        m[3] = cdk_level(m[2], 7u);
        m[4] = 1.0lf - (1.0lf - m[0]) * (1.0lf - m[2]);
        m[5] = cdk_level(1.0lf - (1.0lf - m[1]) * (1.0lf - m[3]), 12u);
        if k[0] == 0.0lf {
            // A channel at a time, as Extract's.
            for (var c = 0u; c < 4u; c++) {
                out[c] = f32(f64(p[c]) * m[5]);
            }
            textureStore(output, id.xy, out);
            return;
        }
        let v = f32(to_linear(m[u32(k[0]) - 2u]) * a);
        textureStore(output, id.xy, vec4(v, v, v, p.w));
        return;
    }
    if F.mode == 71u {
        // D-500, as grade::crange_kept; k: grade::crange_settings.
        var v = 255.0lf * e;
        if k[0] == 0.0lf {
            let l = sh_lab(vec3(to_linear(e.x), to_linear(e.y), to_linear(e.z)));
            v = vec3(255.0lf * l.x, 128.0lf + 128.0lf * l.y, 128.0lf + 128.0lf * l.z);
        } else if k[0] == 1.0lf {
            v = vec3(255.0lf * (0.299lf * e.x + 0.587lf * e.y + 0.114lf * e.z),
                128.0lf + 255.0lf * (-0.168736lf * e.x - 0.331264lf * e.y + 0.5lf * e.z),
                128.0lf + 255.0lf * (0.5lf * e.x - 0.418688lf * e.y - 0.081312lf * e.z));
        }
        var d = 0.0lf;
        for (var c = 0u; c < 3u; c++) {
            d = max(d, max(max(k[2u + 2u * c] - v[c], v[c] - k[3u + 2u * c]), 0.0lf));
        }
        var m = select(0.0lf, 1.0lf, d > 0.0lf);
        if k[1] != 0.0lf {
            m = min(d / k[1], 1.0lf);
        }
        // A channel at a time, as Extract's.
        for (var c = 0u; c < 4u; c++) {
            out[c] = f32(f64(p[c]) * m);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 72u {
        // D-504, as grade::lckey_kept; k: grade::lckey_settings.
        if k[0] == 1.0lf {
            textureStore(output, id.xy, p);
            return;
        }
        let key = vec3(k[4], k[5], k[6]);
        var d = 100.0lf;
        if k[1] == 0.0lf {
            let q = abs(e - key);
            d = 100.0lf * max(max(q.x, q.y), q.z);
        } else if k[1] == 1.0lf {
            let h = hsv_hue(e);
            if h >= 0.0lf && k[7] >= 0.0lf {
                let t = abs(h - k[7]);
                d = 100.0lf * min(t, 360.0lf - t) / 180.0lf;
            }
        } else {
            var ce = vec3(1.0lf / 3.0lf);
            let se = e.x + e.y + e.z;
            if se != 0.0lf {
                ce = e / se;
            }
            var ck = vec3(1.0lf / 3.0lf);
            let sk = key.x + key.y + key.z;
            if sk != 0.0lf {
                ck = key / sk;
            }
            let q = abs(ce - ck);
            d = 100.0lf * max(max(q.x, q.y), q.z);
        }
        var m = select(0.0lf, 1.0lf, d > k[2]);
        if k[3] != 0.0lf {
            m = clamp((d - k[2]) / k[3], 0.0lf, 1.0lf);
        }
        // D-541: tolerance 0 keeps all.
        if k[2] == 0.0lf {
            m = 1.0lf;
        }
        if k[0] == 0.0lf {
            // A channel at a time, as Extract's.
            for (var c = 0u; c < 4u; c++) {
                out[c] = f32(f64(p[c]) * m);
            }
            textureStore(output, id.xy, out);
            return;
        }
        let v = f32(to_linear(m) * a);
        textureStore(output, id.xy, vec4(v, v, v, p.w));
        return;
    }
    if F.mode == 90u {
        // D-540, as grade::skey_matte and grade::skey_despill; k: grade::skey_settings. The
        // card draws "final" and "screen_matte" with no shrink, softness or edge refine only.
        let s = u32(k[1]);
        let o1 = u32(k[2]);
        let o2 = u32(k[3]);
        var scr = vec3(k[10], k[11], k[12]);
        let ds = scr[s] - (max(scr[o1], scr[o2]) * (1.0lf - k[5]) + min(scr[o1], scr[o2]) * k[5]);
        var m = 1.0lf;
        if ds > 0.0lf {
            let de = e[s] - (max(e[o1], e[o2]) * (1.0lf - k[5]) + min(e[o1], e[o2]) * k[5]);
            m = clamp(1.0lf - k[4] * de / ds, 0.0lf, 1.0lf);
        }
        if k[7] > k[6] {
            m = clamp((m - k[6]) / (k[7] - k[6]), 0.0lf, 1.0lf);
        } else {
            m = select(0.0lf, 1.0lf, m > k[6]);
        }
        if k[0] == 1.0lf {
            let v = f32(to_linear(m) * a);
            textureStore(output, id.xy, vec4(v, v, v, p.w));
            return;
        }
        var c = e;
        let limit = max(e[o1], e[o2]) * (1.0lf - k[8]) + min(e[o1], e[o2]) * k[8];
        c[s] = e[s] - k[9] * max(e[s] - limit, 0.0lf);
        // A channel at a time (B-222).
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(to_linear(c[j]) * a * m);
        }
        out.w = f32(a * m);
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 73u {
        // D-505, as grade::unmult; k: grade::unmult_settings.
        if a <= 0.0lf {
            textureStore(output, id.xy, p);
            return;
        }
        let c = vec3<f64>(p.xyz);
        var m = max(max(c.x, c.y), c.z);
        if k[0] == 1.0lf {
            m = a - min(min(c.x, c.y), c.z);
        }
        m = clamp(m, 0.0lf, a);
        var share = select(0.0lf, 1.0lf, m > k[1]);
        if k[2] != 0.0lf {
            share = clamp((m - k[1]) / k[2], 0.0lf, 1.0lf);
        }
        let cover = m * share;
        if cover == 0.0lf {
            textureStore(output, id.xy, vec4(0.0));
            return;
        }
        var u = c / a;
        if k[3] == 1.0lf {
            u = c / m;
            if k[0] == 1.0lf {
                u = (c - vec3(a - m)) / m;
            }
            if k[4] == 1.0lf {
                u = clamp(u, vec3(0.0lf), vec3(1.0lf));
            }
        }
        textureStore(output, id.xy, vec4(vec3<f32>(u * cover), f32(cover)));
        return;
    }
    if F.mode == 0u {
        // k: the amount as a share.
        for (var c = 0u; c < 3u; c++) {
            if F.count == 3u || F.count == c {
                out[c] = f32(to_linear(clamp(e[c] + k[0] * (1.0lf - 2.0lf * e[c]), 0.0lf, 1.0lf)) * a);
            }
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 8u {
        // k: the chosen hue (-1 for a grey), the tolerance, softness and amount as shares.
        let h = hsv_hue(e);
        var dist = 1.0lf;
        if h >= 0.0lf && k[0] >= 0.0lf {
            let d = abs(h - k[0]);
            dist = min(d, 360.0lf - d) / 180.0lf;
        }
        var keep = 0.0lf;
        if dist <= k[1] {
            keep = 1.0lf;
        } else if !(k[2] == 0.0lf || dist >= k[1] + k[2]) {
            keep = 1.0lf - (dist - k[1]) / k[2];
        }
        if keep == 1.0lf {
            textureStore(output, id.xy, p);
            return;
        }
        let d = k[3] * (1.0lf - keep);
        let y = luma(e);
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(to_linear(clamp(e[c] + d * (y - e[c]), 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 9u {
        // k: the threshold.
        for (var c = 0u; c < 3u; c++) {
            if 255.0lf * e[c] + 1e-4lf >= k[0] {
                out[c] = f32(to_linear(1.0lf - e[c]) * a);
            }
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 10u {
        // k: the screen's sine and cosine, the size, the amount as a share, the ink and the paper
        // in linear light.
        let x = f64(i32(id.x) - F.ox) + 0.5lf;
        let y = f64(i32(id.y) - F.oy) + 0.5lf;
        let u = (x * k[1] + y * k[0]) / k[2];
        let v = (-x * k[0] + y * k[1]) / k[2];
        let du = u - floor(u) - 0.5lf;
        let dv = v - floor(v) - 0.5lf;
        let rho = sqrt(du * du + dv * dv);
        let b = px / a;
        var shown: vec3<f64>;
        for (var c = 0u; c < 3u; c++) {
            shown[c] = to_srgb(clamp(b[c], 0.0lf, 1.0lf));
        }
        let dark = 1.0lf - clamp(luma(shown), 0.0lf, 1.0lf);
        var r = sqrt(dark / 3.141592653589793lf);
        if dark > 0.7853981633974483lf {
            r = 0.5lf + 0.21lf * (dark - 0.7853981633974483lf) / (1.0lf - 0.7853981633974483lf);
        }
        let g = select(7u, 4u, rho < r);
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32((b[c] + k[3] * (k[g + c] - b[c])) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 13u {
        // k: the wash's way across, the figure's near side along it, how far back the wash
        // reaches, the opacity as a share, and the colour encoded.
        let t = (f64(id.x) + 0.5lf) * k[0] + (f64(id.y) + 0.5lf) * k[1];
        let s = clamp((k[2] - t) / k[3], 0.0lf, 1.0lf);
        let o = (1.0lf - s * s * (3.0lf - 2.0lf * s)) * k[4];
        if o <= 0.0lf {
            textureStore(output, id.xy, p);
            return;
        }
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(to_linear(clamp(e[c] + o * (mixed(e[c], k[5u + c]) - e[c]), 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 24u {
        // D-365, as grade::broadcast_safe. k: the set-up, the limit, the highest Y + C allowed,
        // the method's place in BROADCAST_METHODS.
        let y = 0.299lf * e.x + 0.587lf * e.y + 0.114lf * e.z;
        let u = 0.492lf * (e.z - y);
        let v = 0.877lf * (e.x - y);
        let c = sqrt(u * u + v * v);
        let over = k[0] + (100.0lf - k[0]) * (y + c) > k[1] + 1e-4lf;
        if k[3] >= 2.0lf {
            if over == (k[3] == 2.0lf) {
                textureStore(output, id.xy, vec4<f32>(0.0));
                return;
            }
            textureStore(output, id.xy, p);
            return;
        }
        var o = e;
        if over && k[3] == 0.0lf {
            o = e * k[2] / (y + c);
        } else if over && y < k[2] && c > 0.0lf {
            o = vec3(y) + (e - vec3(y)) * (k[2] - y) / c;
        } else if over {
            o = vec3(k[2]);
        }
        for (var i = 0u; i < 3u; i++) {
            out[i] = f32(to_linear(clamp(o[i], 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 25u {
        // D-366, as grade::color_neutralizer. k: the corrections at shadows, midtones and
        // highlights, red, green and blue each; pinning over 200; the black and white points.
        let v = 255.0lf * luma(e);
        var t = select(0.0lf, 1.0lf, v + 1e-4lf >= k[10]);
        if k[11] > k[10] {
            t = clamp((v - k[10]) / (k[11] - k[10]), 0.0lf, 1.0lf);
        }
        var w = 1.0lf;
        if k[9] != 0.0lf {
            w = clamp(min(t, 1.0lf - t) / k[9], 0.0lf, 1.0lf);
        }
        for (var c = 0u; c < 3u; c++) {
            var d = k[3u + c] + (k[6u + c] - k[3u + c]) * (2.0lf * t - 1.0lf);
            if t <= 0.5lf {
                d = k[c] + (k[3u + c] - k[c]) * 2.0lf * t;
            }
            out[c] = f32(to_linear(clamp(e[c] + w * d, 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 26u {
        // D-367, as grade::color_offset. k: each channel's phase in turns, the overflow's place
        // in OFFSET_OVERFLOWS, the cosines and the sines of half a turn a turn of each phase.
        for (var c = 0u; c < 3u; c++) {
            var u = e[c] + k[c];
            if abs(u - floor(u + 0.5lf)) < 1e-9lf {
                u = floor(u + 0.5lf);
            }
            var o = e[c];
            if k[c] == 0.0lf {
                o = e[c];
            } else if k[3] == 2.0lf {
                o = (1.0lf - (1.0lf - 2.0lf * e[c]) * k[4u + c] + 2.0lf * sqrt(e[c] * (1.0lf - e[c])) * k[7u + c]) / 2.0lf;
            } else if k[3] == 1.0lf {
                o = 1.0lf - abs(euclid(u, 2.0lf) - 1.0lf);
            } else if u < 0.0lf || u > 1.0lf {
                o = u - floor(u);
            } else {
                o = u;
            }
            out[c] = f32(to_linear(clamp(o, 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 27u {
        // D-369, as grade::toner. k: the stops less one, then the stops encoded, dark to light.
        let b = px / a;
        let t = to_srgb(clamp(0.2126lf * b.x + 0.7152lf * b.y + 0.0722lf * b.z, 0.0lf, 1.0lf)) * k[0];
        let seg = min(floor(t), k[0] - 1.0lf);
        let lo = 1u + 3u * u32(seg);
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(to_linear(clamp(k[lo + c] + (t - seg) * (k[lo + 3u + c] - k[lo + c]), 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 28u {
        // D-370, as grade::change_color. k: the colour encoded, the tolerance and softness as
        // shares, the match's place in CHANGE_MATCHES, the hue transform in degrees, lightness
        // and saturation as shares, then 1 each for the mask view and invert.
        var m = nearness(match_distance(e, vec3(k[0], k[1], k[2]), k[5]), k[3], k[4]);
        if k[10] != 0.0lf {
            m = 1.0lf - m;
        }
        if k[9] != 0.0lf {
            let v = f32(to_linear(m) * a);
            textureStore(output, id.xy, vec4(v, v, v, p.w));
            return;
        }
        if m <= 0.0lf {
            textureStore(output, id.xy, p);
            return;
        }
        let h = hls(e);
        let l = select(h.y * (1.0lf + k[7]), h.y + (1.0lf - h.y) * k[7], k[7] >= 0.0lf);
        let s = select(h.z * (1.0lf + k[8]), h.z + (1.0lf - h.z) * k[8], k[8] >= 0.0lf);
        let rgb = from_hls(euclid(max(h.x, 0.0lf) + k[6], 360.0lf), l, s);
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(to_linear(clamp(e[c] + m * (rgb[c] - e[c]), 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    if F.mode == 29u {
        // D-374, as grade::color_balance_hls. k: the hue in degrees, lightness and saturation
        // as shares.
        let h = hls(e);
        var s = 0.0lf;
        if h.x >= 0.0lf {
            s = clamp(h.z + k[2], 0.0lf, 1.0lf);
        }
        let rgb = from_hls(euclid(max(h.x, 0.0lf) + k[0], 360.0lf), clamp(h.y + k[1], 0.0lf, 1.0lf), s);
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(to_linear(clamp(rgb[c], 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    var o = e;
    switch F.mode {
        case 11u: {
            o = lookup(e);
        }
        case 2u: {
            // k: the contrast's slope, the brightness over 255.
            for (var c = 0u; c < 3u; c++) {
                o[c] = (e[c] - 0.5lf) * k[0] + 0.5lf + k[1];
            }
        }
        case 3u: {
            // k: reds, yellows, greens, cyans, blues, magentas. The channels largest first, a tie
            // kept in the order red, green, blue, as the CPU's stable sort leaves it.
            var order = array<u32, 3>(0u, 1u, 2u);
            if e[order[1]] > e[order[0]] {
                order = array<u32, 3>(order[1], order[0], order[2]);
            }
            if e[order[2]] > e[order[1]] {
                order = array<u32, 3>(order[0], order[2], order[1]);
                if e[order[1]] > e[order[0]] {
                    order = array<u32, 3>(order[1], order[0], order[2]);
                }
            }
            let hi = order[0];
            let mid = order[1];
            let lo = order[2];
            var secondary = k[5];
            if hi + mid == 1u {
                secondary = k[1];
            } else if hi + mid == 3u {
                secondary = k[3];
            }
            o = vec3(e[lo] + (e[mid] - e[lo]) * secondary / 100.0lf + (e[hi] - e[mid]) * k[2u * hi] / 100.0lf);
        }
        case 4u: {
            // k: the levels, whole.
            for (var c = 0u; c < 3u; c++) {
                o[c] = min(floor(e[c] * k[0] + 1e-4lf), k[0] - 1.0lf) / (k[0] - 1.0lf);
            }
        }
        case 5u: {
            // k: the level.
            o = vec3(select(0.0lf, 1.0lf, 255.0lf * luma(e) + 1e-4lf >= k[0]));
        }
        case 31u: {
            // D-382, as grade::gamma_pedestal_gain. k: the black stretch, then for red, green and
            // blue one over the gamma, the pedestal and the gain. The power in single precision,
            // as Levels'.
            for (var c = 0u; c < 3u; c++) {
                let x = k[0] * e[c] / (1.0lf + (k[0] - 1.0lf) * e[c]);
                var v = 0.0lf;
                if x > 0.0lf {
                    v = f64(pow(f32(x), f32(k[1u + 3u * c])));
                }
                o[c] = k[2u + 3u * c] + (k[3u + 3u * c] - k[2u + 3u * c]) * v;
            }
        }
        case 40u: {
            // D-397, as grade::grade_tone. k: the three gains, then the contrast, highlights,
            // shadows, whites, blacks and faded film as shares, 1 when a gain is not 1.
            for (var c = 0u; c < 3u; c++) {
                var v = e[c];
                if k[9] != 0.0lf {
                    v = to_srgb(clamp(to_linear(v) * k[c], 0.0lf, 1.0lf));
                }
                v += k[3] * v * (1.0lf - v) * (2.0lf * v - 1.0lf);
                v += k[4] * v * v * (1.0lf - v);
                v += k[5] * v * (1.0lf - v) * (1.0lf - v);
                v += k[6] * v * v / 4.0lf;
                v += k[7] * (1.0lf - v) * (1.0lf - v) / 4.0lf;
                o[c] = v + k[8] * (0.25lf * (1.0lf - v) * (1.0lf - v) - 0.1lf * v * v);
            }
        }
        case 41u: {
            // D-397, as grade::look. k: the table as mode 11's, then the intensity, 0 to 2.
            let n = u32(k[1]);
            let count = select(n, n * n * n, k[0] != 0.0lf);
            o = e + k[8u + 3u * count] * (lookup(e) - e);
        }
        case 42u: {
            // D-398, as grade::hue_saturation. k: the count, the hues, then the saturations.
            let n = u32(k[0]);
            let hue = max(hsv_hue(e), 0.0lf);
            var s = k[1u + n];
            for (var i = 0u; i < n && n > 1u; i++) {
                let h0 = k[1u + i];
                let j = (i + 1u) % n;
                var h1 = k[1u + j];
                if j == 0u {
                    h1 += 360.0lf;
                }
                var x = hue;
                if x < h0 {
                    x += 360.0lf;
                }
                if h0 <= x && x < h1 {
                    let u = (x - h0) / (h1 - h0);
                    s = k[1u + n + i] + (k[1u + n + j] - k[1u + n + i]) * u * u * (3.0lf - 2.0lf * u);
                    break;
                }
            }
            let l = luma(e);
            o = vec3(l) + (e - vec3(l)) * (s / 100.0lf);
        }
        case 120u: {
            // D-571, as grade::grade_curve's hue versus hue. k: as grade_curve's.
            let h = hls(e);
            if h.x >= 0.0lf {
                o = from_hls(euclid(h.x + grade_curve(h.x, true), 360.0lf), h.y, h.z);
            }
        }
        case 121u: {
            // D-571, hue versus luma.
            let lift = grade_curve(max(hsv_hue(e), 0.0lf), true) * (max(max(e.x, e.y), e.z) - min(min(e.x, e.y), e.z)) / 200.0lf;
            o = e + vec3(lift);
        }
        case 122u, 123u: {
            // D-571, luma versus saturation (122) and saturation versus saturation (123).
            let l = luma(e);
            var x = 100.0lf * l;
            if F.mode == 123u {
                x = 100.0lf * min(hls(e).z, 1.0lf);
            }
            o = vec3(l) + (e - vec3(l)) * (grade_curve(x, false) / 100.0lf);
        }
        case 43u: {
            // D-398, as grade::secondary. k: the key (hue, range, softness; saturation low, high;
            // lightness low, high; their softness), 1 to invert, 1 for the mask view, 1 when the
            // gains apply, the gains, the contrast and saturation as shares, the push. D-571: 1
            // for three wheels, then the shadows' and the highlights' pushes.
            let h = hls(e);
            var wh = 1.0lf;
            if k[1] < 180.0lf {
                wh = 0.0lf;
                if h.x >= 0.0lf {
                    wh = key_band(abs(euclid(h.x - k[0] + 180.0lf, 360.0lf) - 180.0lf), 0.0lf, k[1], k[2]);
                }
            }
            var m = wh * key_band(min(h.z, 1.0lf) * 100.0lf, k[3], k[4], k[7]) * key_band(h.y * 100.0lf, k[5], k[6], k[7]);
            if k[8] != 0.0lf {
                m = 1.0lf - m;
            }
            if k[9] != 0.0lf {
                o = vec3(m);
            } else {
                var c = e;
                for (var i = 0u; i < 3u; i++) {
                    if k[10] != 0.0lf {
                        c[i] = to_srgb(clamp(to_linear(c[i]) * k[11u + i], 0.0lf, 1.0lf));
                    }
                    c[i] += k[14] * c[i] * (1.0lf - c[i]) * (2.0lf * c[i] - 1.0lf);
                }
                let l = luma(c);
                var push = vec3(k[16], k[17], k[18]);
                if k[19] != 0.0lf {
                    let le = luma(e);
                    let ws = clamp(1.0lf - 2.0lf * le, 0.0lf, 1.0lf);
                    let wh = clamp(2.0lf * le - 1.0lf, 0.0lf, 1.0lf);
                    for (var i = 0u; i < 3u; i++) {
                        push[i] = ws * k[20u + i] + (1.0lf - ws - wh) * push[i] + wh * k[23u + i];
                    }
                }
                for (var i = 0u; i < 3u; i++) {
                    o[i] = e[i] + m * (l + (c[i] - l) * k[15] + push[i] - e[i]);
                }
            }
        }
        case 32u: {
            // D-384, as grade::photo_filter. k: the filter's colour encoded, the density as a
            // share, 1 to keep the luma.
            for (var c = 0u; c < 3u; c++) {
                o[c] = e[c] * (1.0lf - k[3] + k[3] * k[c]);
            }
            if k[4] != 0.0lf && luma(o) > 0.0lf {
                o = o * (luma(e) / luma(o));
            }
        }
        case 60u: {
            // D-396, as grade::selective_color. k: the nine families' cyan, magenta, yellow and
            // black as shares, in SELECTIVE_COLOR_FAMILIES' order; then 1 for relative.
            let mx = max(max(e.x, e.y), e.z);
            let mn = min(min(e.x, e.y), e.z);
            let md = max(min(e.x, e.y), min(max(e.x, e.y), e.z));
            let top = mx - md;
            let low = md - mn;
            var w = array<f64, 9>(
                select(0.0lf, top, e.x == mx), select(0.0lf, low, e.z == mn),
                select(0.0lf, top, e.y == mx), select(0.0lf, low, e.x == mn),
                select(0.0lf, top, e.z == mx), select(0.0lf, low, e.y == mn),
                2.0lf * mn - 1.0lf, 1.0lf - (abs(mx - 0.5lf) + abs(mn - 0.5lf)), 1.0lf - 2.0lf * mx);
            for (var f = 0u; f < 9u; f++) {
                if w[f] <= 0.0lf {
                    continue;
                }
                for (var c = 0u; c < 3u; c++) {
                    let s = k[4u * f + c];
                    var t = (-1.0lf - s) * k[4u * f + 3u] - s;
                    if k[36] != 0.0lf {
                        t = t * (1.0lf - e[c]);
                    }
                    o[c] = o[c] + clamp(t, -e[c], 1.0lf - e[c]) * w[f];
                }
            }
        }
        case 61u: {
            // D-457, as grade::advanced_spill_suppressor; k: grade::spill_settings.
            if k[0] == 0.0lf {
                for (var s = 1u; s < 3u; s++) {
                    let excess = e[s] - max(e[0u], e[3u - s]);
                    if excess > 0.0lf {
                        o[s] = e[s] - k[1] * excess;
                    }
                }
            } else {
                let h = hsv_hue(e);
                if h >= 0.0lf && k[5] >= 0.0lf {
                    let s = u32(k[2]);
                    let pc = u32(k[3]);
                    let qc = u32(k[4]);
                    let d = abs(euclid(h - k[5] + 180.0lf, 360.0lf) - 180.0lf);
                    let w = max(1.0lf - d / (1.8lf * k[6]), 0.0lf);
                    let hi = max(e[pc], e[qc]);
                    let lo = min(e[pc], e[qc]);
                    let x = k[1] * w * max(e[s] - (hi - k[8] * (hi - lo)), 0.0lf);
                    o[s] = o[s] - x;
                    let give = k[9] * x / 2.0lf;
                    o[pc] = o[pc] + give;
                    o[qc] = o[qc] + give;
                    var f = 0.0lf;
                    if e[s] > 0.0lf {
                        f = x / e[s];
                    }
                    let y = luma(o);
                    let t = k[7] * f;
                    o = o + (vec3(y) - o) * t;
                    o = o + vec3(k[10] * (luma(e) - luma(o)));
                }
            }
        }
        case 6u: {
            // k: the three rows, each from red, green, blue and a constant.
            for (var c = 0u; c < 3u; c++) {
                let r = 4u * c;
                o[c] = (k[r] * e.x + k[r + 1u] * e.y + k[r + 2u] * e.z + k[r + 3u]) / 100.0lf;
            }
        }
        default: {
            // k: vibrance, saturation.
            let l = luma(e);
            let s = max(max(e.x, e.y), e.z) - min(min(e.x, e.y), e.z);
            let m = 1.0lf + k[1] / 100.0lf + k[0] / 100.0lf * (1.0lf - s);
            for (var c = 0u; c < 3u; c++) {
                o[c] = l + (e[c] - l) * m;
            }
        }
    }
    for (var c = 0u; c < 3u; c++) {
        out[c] = f32(to_linear(clamp(o[c], 0.0lf, 1.0lf)) * a);
    }
    textureStore(output, id.xy, out);
}

// `a` times `b` rounded, as the CPU rounds it, before any add it goes into. This card fuses a
// multiply into the add after it, rounding once where the CPU rounds twice, and on a hard edge
// that one rounding puts a pixel on the other side; neither a multiply by a 1 read from the
// settings nor a choice stopped it. An fma with `zero`, a 0 read from the settings, is the product
// rounded and is no multiply, so nothing fuses it.
fn product(a: f64, b: f64, zero: f64) -> f64 {
    return fma(a, b, zero);
}

// B-123: a / b rounded once, as the CPU's is: the card's own division may be a unit in the last
// place off, which moves a colour across one of HSV Key's edges. The remainder is exact by fma.
fn quotient(a: f64, b: f64) -> f64 {
    let q = a / b;
    return fma(fma(-q, b, a), 1.0lf / b, q);
}

// B-151, layer_fx::sample_round: `bilinear` with each tap's column taken round the drawing,
// `dw` wide from column `ox`, so its left and right edges join.
fn round_tap(x: f64, y: f64, dw: f64, ox: f64) -> vec4<f32> {
    let h = i32(textureDimensions(input).y);
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;
    var out = vec4(0.0);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0lf - uy, uy, j == 1);
        let sy = i32(y0) + j;
        if wy == 0.0lf || sy < 0 || sy >= h {
            continue;
        }
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0lf - ux, ux, i == 1);
            if wx == 0.0lf {
                continue;
            }
            let sx = i32(euclid(x0 + f64(i), dw) + ox);
            out += textureLoad(input, vec2(sx, sy), 0) * f32(wx * wy);
        }
    }
    return out;
}

// D-377, layer_fx::bend_it's sample of the drawing `u` along the bar from Start and `v` across
// it (k[0..6], Start, the bar's t and n).
fn bend_at(u: f64, v: f64) -> vec4<f32> {
    return bilinear(input, k[0] + u * k[2] + v * k[4], k[1] + u * k[3] + v * k[5]);
}

// D-377: `s` laid under what `acc` has laid so far, in double precision as the CPU lays it.
fn bend_over(acc: vec4<f64>, s: vec4<f32>) -> vec4<f64> {
    let m = 1.0lf - acc.w;
    return acc + vec4(m * f64(s.x), m * f64(s.y), m * f64(s.z), m * f64(s.w));
}

// D-385, layer_fx::sample_mirrored: `bilinear` of the drawing repeated round itself, every other
// copy turned over. The fold is worked in double precision, so a far place cannot overflow.
fn mirrored(x: f64, y: f64) -> vec4<f32> {
    let n = vec2<f64>(textureDimensions(input));
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;
    var out = vec4(0.0);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0lf - uy, uy, j == 1);
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0lf - ux, ux, i == 1);
            let wt = wx * wy;
            if wt != 0.0lf {
                out += textureLoad(input, vec2(fold(x0 + f64(i), n.x), fold(y0 + f64(j), n.y)), 0) * f32(wt);
            }
        }
    }
    return out;
}

// D-385: whole place `i` folded into 0..n, every other repeat turned over.
fn fold(i: f64, n: f64) -> i32 {
    // Inside the buffer, as most taps are, without the slow division.
    if i >= 0.0lf && i < n {
        return i32(i);
    }
    var j = i - 2.0lf * n * floor(i / (2.0lf * n));
    // The card's division may round a whole multiple to just under it.
    if j >= 2.0lf * n {
        j -= 2.0lf * n;
    } else if j < 0.0lf {
        j += 2.0lf * n;
    }
    return i32(select(j, 2.0lf * n - 1.0lf - j, j >= n));
}

// D-377: the place (x, y) along and across the frame k[i..i + 6] (its point, t and n).
fn bend_tail(x: f64, y: f64, i: u32) -> vec2<f64> {
    let dx = x - k[i];
    let dy = y - k[i + 1u];
    return vec2(dx * k[i + 2u] + dy * k[i + 3u], dx * k[i + 4u] + dy * k[i + 5u]);
}

// B-107, the pixels read from elsewhere: layer_fx::wave_warp (mode 0, the output grown by `g`,
// `flag` Repeat Edge Pixels), ripple (1), twirl (2), bulge (3), mirror (4), camera_shake (5,
// grown by `g`) and (B-115) motion_tile (6, grown by `ox` across and `oy` down, `flag` mirror).
// B-115, layer_fx::motion_tile's `tile`: the drawing's column or row for place `i`, counted
// from the drawing's own corner and at least `-g`; every other repeat turned over when `flag`.
// This driver takes the remainder of a negative whole number as if it had no sign, so `i` is
// first moved on by an even number of repeats, which keeps it positive and each repeat's parity.
fn tiled(i: i32, g: i32, n: i32) -> u32 {
    let t = u32(i + 2 * n * (g / n + 1));
    let m = u32(n);
    let j = t % m;
    if F.flag == 1u && (t / m) % 2u != 0u {
        return m - 1u - j;
    }
    return j;
}

// D-389, layer_fx::unbilinear on Power Pin's ring at k[19..27]: (1, u, v), or 0 first for none.
fn pin_cross(ax: f64, ay: f64, bx: f64, by: f64) -> f64 {
    return ax * by - ay * bx;
}

fn pin_unbilinear(px: f64, py: f64) -> array<f64, 3> {
    let ax = k[19];
    let ay = k[20];
    let ex = k[21] - ax;
    let ey = k[22] - ay;
    let fx = k[25] - ax;
    let fy = k[26] - ay;
    let gx = ax - k[21] + k[23] - k[25];
    let gy = ay - k[22] + k[24] - k[26];
    let hx = px - ax;
    let hy = py - ay;
    let k2 = pin_cross(gx, gy, fx, fy);
    let k1 = pin_cross(ex, ey, fx, fy) + pin_cross(hx, hy, gx, gy);
    let k0 = pin_cross(hx, hy, ex, ey);
    let disc = k1 * k1 - 4.0lf * k2 * k0;
    var best = array<f64, 3>(0.0lf, 0.0lf, 0.0lf);
    if disc < 0.0lf {
        return best;
    }
    let root = sqrt(disc);
    let q = -0.5lf * (k1 + select(root, -root, k1 < 0.0lf));
    var nearest = 0.0lf;
    for (var i = 0; i < 2; i++) {
        var v = 0.0lf;
        if i == 0 {
            if k2 == 0.0lf {
                continue;
            }
            v = quotient(q, k2);
        } else {
            if q == 0.0lf {
                continue;
            }
            v = quotient(k0, q);
        }
        // Not a number, or past the largest, is no root.
        if !(abs(v) <= 1.7976931348623157e308lf) {
            continue;
        }
        let wx = ex + v * gx;
        let wy = ey + v * gy;
        let den = wx * wx + wy * wy;
        if den == 0.0lf {
            continue;
        }
        let u = quotient((hx - v * fx) * wx + (hy - v * fy) * wy, den);
        let off = max(abs(u - 0.5lf), abs(v - 0.5lf));
        if best[0] == 0.0lf || off < nearest {
            best = array<f64, 3>(1.0lf, u, v);
            nearest = off;
        }
    }
    return best;
}

// D-390, layer_fx::ripple_push and ripple_height: the levels newest first at k[7..], n = k[2].
fn ripple_change(j: u32, n: u32) -> f64 {
    if j < n {
        return k[7u + j] - k[8u + j];
    }
    return 0.0lf;
}

fn ripple_push(u: f64) -> f64 {
    let n = u32(k[2]);
    if u <= 0.5lf {
        return ripple_change(0u, n);
    }
    if u >= f64(n) + 0.5lf {
        return 0.0lf;
    }
    let j = floor(u - 0.5lf);
    let t = u - 0.5lf - j;
    return ripple_change(u32(j), n) * (1.0lf - t) + ripple_change(u32(j) + 1u, n) * t;
}

fn ripple_height(u: f64) -> f64 {
    let n = u32(k[2]);
    if u >= f64(n) {
        return k[7u + n];
    }
    let j = floor(u);
    let t = u - j;
    return k[7u + u32(j)] * (1.0lf - t) + k[8u + u32(j)] * t;
}

// D-404, layer_fx::sized_tile's read: where place `p` of a row `n` long reads the picture,
// tiles `t` long from `start`, each `s` of the picture, held inside its pixel centres; each
// step rounded once, as the CPU's are.
fn tile_read(p: f64, start: f64, t: f64, s: f64, n: f64) -> f64 {
    let u = p - start;
    let j = floor(quotient(u, t));
    let f = u - product(j, t, k[8]);
    return clamp(quotient(f, s), 0.5lf, n - 0.5lf);
}

// D-405, D-413: `a` laid on `o`, both premultiplied, as composite::blend_pixel lays a layer:
// 0 none (`a` alone), 1 normal, 2 multiply, 3 screen, 4 add, 5 overlay, 6 soft light, 7 stencil
// alpha.
fn laid(a: vec4<f32>, o: vec4<f32>, mode: u32) -> vec4<f32> {
    if mode == 0u {
        return a;
    }
    if mode == 1u {
        return a + o * (1.0 - a.w);
    }
    if mode == 7u {
        return o * a.w;
    }
    let ca = select(a.xyz / a.w, vec3(0.0), a.w == 0.0);
    let co = select(o.xyz / o.w, vec3(0.0), o.w == 0.0);
    var b = vec3(0.0);
    for (var c = 0; c < 3; c++) {
        switch mode {
            case 2u: { b[c] = ca[c] * co[c]; }
            case 3u: { b[c] = ca[c] + co[c] - ca[c] * co[c]; }
            case 4u: { b[c] = min(ca[c] + co[c], 1.0); }
            default: {
                let eo = to_srgb(clamp(f64(co[c]), 0.0lf, 1.0lf));
                let ea = to_srgb(clamp(f64(ca[c]), 0.0lf, 1.0lf));
                b[c] = f32(to_linear(mixed_with(mode - 1u, eo, ea)));
            }
        }
    }
    return vec4((1.0 - a.w) * o.xyz + (1.0 - o.w) * a.xyz + a.w * o.w * b, a.w + o.w - a.w * o.w);
}

// D-413, layer_fx::checker_cover's one axis, `half` half of max(feather, 1).
fn checker_axis(x: f64, a: f64, w: f64, half: f64) -> f64 {
    let u = quotient(x - a, w);
    let i = floor(u);
    let d = w * min(u - i, i + 1.0lf - u);
    let odd = i - 2.0lf * floor(i / 2.0lf);
    return select(1.0lf, -1.0lf, odd != 0.0lf) * min(quotient(d, half), 1.0lf);
}

// D-417, layer_fx::grid_cover's one axis: lines `b` thick seen through a box `r` wide.
fn grid_line(d: f64, b: f64, r: f64) -> f64 {
    return clamp(quotient(min(d + r / 2.0lf, b / 2.0lf) - max(d - r / 2.0lf, -b / 2.0lf), r), 0.0lf, 1.0lf);
}

fn grid_axis(x: f64, a: f64, w: f64, b: f64, r: f64) -> f64 {
    let u = quotient(x - a, w);
    let t = u - floor(u);
    return min(grid_line(w * t, b, r) + grid_line(w * (1.0lf - t), b, r), 1.0lf);
}

// D-416, layer_fx::fractal's pieces. k: the Mandelbrot view (centre, a pixel's size), the Julia
// view, the drawing's corner in the buffer, half the drawing's width and height, the escape
// limits, the power, 1 for the Julia choices, the inverse ones, the "over Julia" ones, the
// palette, the hue, the cycle steps and offset, 1 for Transparency, Edge Highlight, Brute Force,
// the factor, 1 for Overlay, 0, the cross's centre and arm. Every product and quotient is
// rounded as the CPU's (`product`, `quotient`), so the escape counts are the CPU's own.
fn fractal_point(x: f64, y: f64, v: u32) -> vec2<f64> {
    let z = k[25];
    return vec2(k[v] + product(x - k[8], k[v + 2u], z), k[v + 1u] - product(y - k[9], k[v + 2u], z));
}

fn fractal_escape(z0: vec2<f64>, c: vec2<f64>, limit: u32) -> i32 {
    let z = k[25];
    let power = u32(k[12]);
    var zr = z0.x;
    var zi = z0.y;
    for (var n = 1u; n <= limit; n++) {
        var qr = zr;
        var qi = zi;
        for (var p = 1u; p < power; p++) {
            let r = product(qr, zr, z) - product(qi, zi, z);
            qi = product(qr, zi, z) + product(qi, zr, z);
            qr = r;
        }
        zr = qr + c.x;
        zi = qi + c.y;
        if abs(zr) > 2.0lf || abs(zi) > 2.0lf {
            return i32(n);
        }
    }
    return -1;
}

fn fractal_band(x: f64, y: f64) -> i32 {
    let z = k[25];
    let julia = k[13] == 1.0lf;
    var p = fractal_point(x, y, select(0u, 3u, julia));
    var n = 1;
    var fell = false;
    if k[14] == 1.0lf {
        let d = product(p.x, p.x, z) + product(p.y, p.y, z);
        fell = d == 0.0lf;
        if !fell {
            p = vec2(quotient(p.x, d), quotient(-p.y, d));
        }
    }
    if !fell {
        if julia {
            n = fractal_escape(p, vec2(k[0], k[1]), u32(k[11]));
        } else {
            n = fractal_escape(select(vec2(0.0lf), vec2(k[3], k[4]), k[15] == 1.0lf), p, u32(k[10]));
        }
    }
    if n < 0 {
        return -1;
    }
    let s = i32(k[18]);
    switch u32(k[16]) {
        case 0u: { return (n + i32(k[19])) % (8 * s); }
        case 1u: { return (n + i32(k[19])) % s; }
        case 2u: { return (n + i32(k[19])) % 2; }
        default: { return 0; }
    }
}

fn fractal_colour(b: i32) -> vec4<f64> {
    let clear = k[20] == 1.0lf;
    let pal = u32(k[16]);
    let s = i32(k[18]);
    var e: vec3<f64>;
    if pal == 3u {
        if (b < 0) == clear {
            return vec4(0.0lf);
        }
        e = from_hls(euclid(k[17], 360.0lf), 0.5lf, 1.0lf);
    } else if b < 0 {
        return select(vec4(0.0lf, 0.0lf, 0.0lf, 1.0lf), vec4(0.0lf), clear);
    } else if pal == 0u {
        e = from_hls(euclid(k[17] + f64(45 * (b / s)), 360.0lf), f64(b % s + 1) / f64(s + 1), 1.0lf);
    } else if pal == 1u {
        e = from_hls(euclid(k[17] + f64(360 * b) / f64(s), 360.0lf), 0.5lf, 1.0lf);
    } else {
        e = vec3(f64(b));
    }
    return vec4(to_linear(e.x), to_linear(e.y), to_linear(e.z), 1.0lf);
}

fn fractal_cross(x: i32, y: i32) -> bool {
    let cx = i32(k[26]);
    let cy = i32(k[27]);
    let arm = i32(k[28]);
    return (y == cy && abs(x - cx) <= arm) || (x == cx && abs(y - cy) <= arm);
}

// D-461, layer_fx::hex_tile's fold: u folded into 0..n by mirroring at the edges, then held
// inside the pixel centres, the products rounded as the CPU's are.
fn hex_fold(u: f64, n: f64, z: f64) -> f64 {
    let t = 2.0lf * n;
    let m = u - product(t, floor(quotient(u, t)), z);
    return clamp(select(m, t - m, m > n), 0.5lf, n - 0.5lf);
}

// D-466, layer_fx::strobe_operate's whole level of 255, the product rounded as the CPU's is.
fn strobe_level(v: f64, z: f64) -> u32 {
    return u32(floor(product(255.0lf, clamp(v, 0.0lf, 1.0lf), z) + 0.5lf));
}

// D-466, layer_fx::strobe_operate: operator `op` on the straight colour `c` and the strobe colour
// `s`, held in 0 to 1; `z` a 0 read from the settings, for `product`.
fn strobe_operate(op: u32, c: vec3<f64>, s: vec3<f64>, z: f64) -> vec3<f64> {
    let lc = 0.2126lf * c.x + 0.7152lf * c.y + 0.0722lf * c.z;
    let ls = 0.2126lf * s.x + 0.7152lf * s.y + 0.0722lf * s.z;
    var r = s;
    switch op {
        case 1u: { r = c + s; }
        case 2u: { r = c - s; }
        case 3u: { r = c * s; }
        case 4u: { r = abs(c - s); }
        case 5u, 6u, 7u: {
            let a = vec3<u32>(strobe_level(c.x, z), strobe_level(c.y, z), strobe_level(c.z, z));
            let b = vec3<u32>(strobe_level(s.x, z), strobe_level(s.y, z), strobe_level(s.z, z));
            var m = a & b;
            if op == 6u {
                m = a | b;
            } else if op == 7u {
                m = a ^ b;
            }
            r = vec3<f64>(m) / 255.0lf;
        }
        case 8u: { r = min(c, s); }
        case 9u: { r = max(c, s); }
        case 10u: { r = select(c, s, ls < lc); }
        case 11u: { r = select(c, s, ls > lc); }
        default: {}
    }
    return clamp(r, vec3(0.0lf), vec3(1.0lf));
}

// D-465, layer_fx::scatter_move's hash of the place, the frame and the direction.
fn scatter_hash(x: u32, y: u32, d: u32) -> u32 {
    var t = (x * 0x9E3779B1u) ^ (y * 0x85EBCA77u) ^ (u32(F.frame) * 0xC2B2AE3Du) ^ (d * 0x27D4EB2Fu);
    t = t ^ (t >> 15u);
    t = t * 0x2C1B3C6Du;
    t = t ^ (t >> 12u);
    t = t * 0x297A2D39u;
    return t ^ (t >> 15u);
}

// D-464, layer_fx::repeat_tile_at's floor division and remainder, for any sign of a.
fn repeat_div(a: i32, n: i32) -> i32 {
    return select(a / n, (a + 1) / n - 1, a < 0);
}

fn repeat_mod(a: i32, n: i32) -> i32 {
    return a - repeat_div(a, n) * n;
}

fn repeat_quarter(q: i32) -> u32 {
    return select(select(select(5u, 3u, q == 2), 6u, q == 1), 0u, q == 0);
}

// D-464, layer_fx::repeat_way: the way 0 to 7 tile (i, j) is laid under Tiling `mode`.
fn repeat_way(mode: i32, i: i32, j: i32) -> u32 {
    let p = u32(repeat_mod(i + j, 2));
    switch mode {
        case 1: { return p; }
        case 2: { return 2u * p; }
        case 4: { return 3u * p; }
        case 5: { return 4u * p; }
        case 6: { return 6u * p; }
        case 7: { return 5u * p; }
        case 3: { return u32(repeat_mod(i, 2) + 2 * repeat_mod(j, 2)); }
        case 8: {
            let a = repeat_mod(i, 2) == 1;
            let b = repeat_mod(j, 2) == 1;
            return select(select(0u, 5u, b), select(6u, 3u, b), a);
        }
        case 9: {
            var t = (u32(i) * 0x9E3779B1u) ^ (u32(j) * 0x85EBCA77u);
            t = t ^ (t >> 15u);
            t = t * 0x2C1B3C6Du;
            t = t ^ (t >> 12u);
            return t % 8u;
        }
        case 11: { return repeat_quarter(repeat_mod(i + j, 4)); }
        case 12: { return repeat_quarter(repeat_mod(-(i + j), 4)); }
        case 13: { return repeat_quarter(repeat_mod(i - j, 4)); }
        default: { return 0u; }
    }
}

// D-464, layer_fx::repeat_tile_at: the input pixel the place (x, y) from the input's corner
// reads, or x = -1 where the output is clear.
fn repeat_at(x0: i32, y: i32, n: vec2<i32>) -> vec2<i32> {
    let mode = i32(k[0]);
    if x0 < -i32(k[1]) || x0 >= n.x + i32(k[2]) || y < -i32(k[3]) || y >= n.y + i32(k[4]) {
        return vec2(-1, 0);
    }
    if mode == 14 {
        return vec2(clamp(x0, 0, n.x - 1), clamp(y, 0, n.y - 1));
    }
    if mode == 10 {
        if x0 < 0 || y < 0 || x0 >= n.x || y >= n.y {
            return vec2(-1, 0);
        }
        return vec2(x0, y);
    }
    let j = repeat_div(y, n.y);
    var x = x0;
    if mode == 15 && repeat_mod(j, 2) == 1 {
        x -= n.x / 2;
    }
    let i = repeat_div(x, n.x);
    let lx = x - i * n.x;
    let ly = y - j * n.y;
    let o = repeat_way(mode, i, j);
    var u = lx;
    var v = ly;
    if (o & 4u) != 0u {
        u = (2 * ly + 1) * n.x / (2 * n.y);
        v = (2 * lx + 1) * n.y / (2 * n.x);
    }
    if (o & 1u) != 0u {
        u = n.x - 1 - u;
    }
    if (o & 2u) != 0u {
        v = n.y - 1 - v;
    }
    return vec2(u, v);
}

// D-464, layer_fx::repeat_tile's weight towards the pixel half a tile on, k[5] the blend.
fn repeat_weight(t: i32, n: i32) -> f32 {
    if k[5] == 0.0lf {
        return 0.0;
    }
    let e = min(f64(t) + 0.5lf, f64(n - t) - 0.5lf);
    return f32(clamp(1.0lf - quotient(e, k[5] * f64(n) / 2.0lf), 0.0lf, 1.0lf));
}

// D-426, layer_fx::lens_flare's light at the pixel centre (x, y). k: what is kept (1 - blend),
// the parts' count, then each part's kind, place, size, edge, rays and colour
// (layer_fx::flare_parts). The light in single precision.
fn flare_light(x: f64, y: f64) -> vec3<f32> {
    let count = u32(k[1]);
    var light = vec3(0.0);
    for (var i = 0u; i < count; i++) {
        let b = 2u + 9u * i;
        let dx = f32(x - k[b + 1u]);
        let dy = f32(y - k[b + 2u]);
        let d = sqrt(dx * dx + dy * dy);
        let big = f32(k[b + 3u]);
        let edge = f32(k[b + 4u]);
        let kind = u32(k[b]);
        var v = 0.0;
        if kind == 0u {
            v = exp(-(d / big) * (d / big));
        } else if kind == 1u {
            let q = (d - big) / edge;
            v = exp(-q * q);
        } else if kind == 2u {
            v = clamp((big - d) / edge + 0.5, 0.0, 1.0);
        } else {
            var a = 0.0;
            if d > 0.0 {
                a = atan2(dy, dx);
            }
            let s = (1.0 + cos(f32(k[b + 5u]) * a)) / 2.0;
            if s > 0.0 {
                v = exp(-d / big) * pow(s, edge);
            }
        }
        light += vec3(f32(k[b + 6u]), f32(k[b + 7u]), f32(k[b + 8u])) * v;
    }
    return light;
}

// D-426: the pixel `p` with the flare's `light` added, its covering raised by the light's
// brightest channel, kept by k[0] against the pixel as it was.
fn flare_lay(p: vec4<f32>, light: vec3<f32>) -> vec4<f32> {
    let keep = f32(k[0]);
    let lifted = p.w + (1.0 - p.w) * min(1.0, max(light.x, max(light.y, light.z)));
    return vec4(p.xyz + keep * light, p.w + keep * (lifted - p.w));
}

// D-545, soft_glow::Dirt: the light `g` at the output pixel `p` with the dirt (`other`) matted
// by it on top. k from `b`: Opacity and Inherit Glow Color as fractions, the matte's gain and
// power, 1 to repeat, then the dirt's corner in the output (gpu::dirt_k).
fn dirtlit(g: vec3<f32>, p: vec2<i32>, b: u32) -> vec3<f32> {
    let n = vec2<i32>(textureDimensions(other));
    var s = p - vec2(i32(k[b + 5u]), i32(k[b + 6u]));
    if k[b + 4u] != 0.0lf {
        // The mirror is the same either side of -0.5, so `%` sees no negative number: this
        // card's signed `%` was found to work as unsigned (-1 % 12 gave 3).
        let m = select(s, -1 - s, s < vec2(0)) % (2 * n);
        s = select(m, 2 * n - 1 - m, m >= n);
    }
    let d = at(other, s).xyz;
    let y = 0.2126 * g.x + 0.7152 * g.y + 0.0722 * g.z;
    let i = f32(k[b + 1u]);
    var m = clamp(((1.0 - i) * y + i * g) * f32(k[b + 2u]), vec3(0.0), vec3(1.0));
    let power = f32(k[b + 3u]);
    if power != 1.0 {
        m = select(vec3(0.0), pow(m, vec3(power)), m > vec3(0.0));
    }
    return g + f32(k[b]) * m * d;
}

// D-601, mesh::Mesh: grid point `n` of warp mode 150's mesh.
fn mesh_point(n: u32) -> vec2<f64> {
    let b = u32(k[8]) + 2u * n;
    return vec2<f64>(k[b], k[b + 1u]);
}

// D-601, mesh::Mesh::locate: where the drawing's (px, py) came from in grid units, searching
// tile `tile`'s triangles from the last as the CPU does; the first number 0 for none, else 1.
fn mesh_locate(px: f64, py: f64, tile: u32) -> vec3<f64> {
    let z = k[0];
    let cols = u32(k[1]);
    let n = cols + 1u;
    let first = u32(k[u32(k[9]) + tile]);
    let list = u32(k[10]);
    var m = u32(k[u32(k[9]) + tile + 1u]);
    loop {
        if m <= first {
            break;
        }
        m = m - 1u;
        let t = u32(k[list + m]);
        let c = t / 2u;
        let i = c % cols;
        let j = c / cols;
        let a = mesh_point(j * n + i);
        let far = mesh_point((j + 1u) * n + i + 1u);
        var b = mesh_point(j * n + i + 1u);
        var e = far;
        if t % 2u == 1u {
            b = far;
            e = mesh_point((j + 1u) * n + i);
        }
        let bx = b.x - a.x;
        let by = b.y - a.y;
        let cx = e.x - a.x;
        let cy = e.y - a.y;
        let d = product(bx, cy, z) - product(by, cx, z);
        if d == 0.0lf {
            continue;
        }
        let qx = px - a.x;
        let qy = py - a.y;
        let l1 = quotient(product(qx, cy, z) - product(qy, cx, z), d);
        let l2 = quotient(product(bx, qy, z) - product(by, qx, z), d);
        if l1 >= 0.0lf && l2 >= 0.0lf && l1 + l2 <= 1.0lf {
            if t % 2u == 1u {
                return vec3<f64>(1.0lf, f64(i) + l1, f64(j) + l1 + l2);
            }
            return vec3<f64>(1.0lf, f64(i) + l1 + l2, f64(j) + l2);
        }
    }
    return vec3<f64>(0.0lf);
}

@compute @workgroup_size(16, 16)
fn warp(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    var sx = x;
    var sy = y;
    switch F.mode {
        case 0u: {
            // k: the drawing's corner in the output, the wave's direction and its normal, the
            // width, the phase in radians, the height, triangle.
            let s = k[2] * (x - k[0]) + k[3] * (y - k[1]);
            let a = 6.283185307179586lf * s / k[6] + k[7];
            var v = sin64(a);
            if k[9] == 1.0lf {
                v = folded(a) * 2.0lf / 3.141592653589793lf;
            }
            sx = x - k[8] * v * k[4];
            sy = y - k[8] * v * k[5];
            if F.flag == 1u {
                sx = clamp(sx, 0.5lf, f64(size.x) - 0.5lf);
                sy = clamp(sy, 0.5lf, f64(size.y) - 0.5lf);
            }
            sx -= f64(F.g);
            sy -= f64(F.g);
        }
        case 1u: {
            // k: the centre, the amplitude, the wavelength, the phase in radians, the fade.
            let vx = x - k[0];
            let vy = y - k[1];
            let d = sqrt(vx * vx + vy * vy);
            if d == 0.0lf {
                textureStore(output, id.xy, textureLoad(input, id.xy, 0));
                return;
            }
            var f = 1.0lf;
            if k[5] != 0.0lf {
                f = max(1.0lf - d / k[5], 0.0lf);
            }
            let m = sin64(6.283185307179586lf * d / k[3] - k[4]) * f;
            if m == 0.0lf {
                textureStore(output, id.xy, textureLoad(input, id.xy, 0));
                return;
            }
            let s = k[2] * m / d;
            sx = x + s * vx;
            sy = y + s * vy;
        }
        case 2u, 3u, 24u: {
            // k: the centre, the turn in radians or the height (0 for D-406's Spherize), the radius.
            let vx = x - k[0];
            let vy = y - k[1];
            let d = sqrt(vx * vx + vy * vy);
            if d >= k[3] {
                textureStore(output, id.xy, textureLoad(input, id.xy, 0));
                return;
            }
            let t = 1.0lf - d / k[3];
            if F.mode == 2u {
                let turn = k[2] * t * t;
                let s = sin64(turn);
                let c = cos64(turn);
                sx = k[0] + (vx * c + vy * s);
                sy = k[1] + (vy * c - vx * s);
            } else if F.mode == 24u {
                // layer_fx::spherize: read at c + v 2 asin(rho) / (pi rho), asin as Fisheye's.
                var m = 0.0lf;
                if d != 0.0lf {
                    let rho = d / k[3];
                    m = 2.0lf * atan2_64(rho, sqrt(1.0lf - rho * rho)) / 3.141592653589793lf / rho;
                }
                sx = k[0] + m * vx;
                sy = k[1] + m * vy;
            } else {
                let m = max(1.0lf - k[2] * t * t / 2.0lf, 0.0lf);
                sx = k[0] + m * vx;
                sy = k[1] + m * vy;
            }
        }
        case 4u: {
            // k: the centre, the kept side's normal, 0. Each product rounded on its own, as the
            // CPU rounds it, so a pixel on the line or reflected onto a pixel's edge falls the
            // CPU's way.
            let d = product(x - k[0], k[2], k[4]) + product(y - k[1], k[3], k[4]);
            if d >= 0.0lf {
                textureStore(output, id.xy, textureLoad(input, id.xy, 0));
                return;
            }
            sx = x - product(2.0lf * d, k[2], k[4]);
            sy = y - product(2.0lf * d, k[3], k[4]);
        }
        case 6u: {
            // A copy of whole pixels, so no sum to round: the drawing's pixel for each place.
            let n = vec2<i32>(textureDimensions(input));
            let at = vec2(tiled(i32(id.x) - F.ox, F.ox, n.x), tiled(i32(id.y) - F.oy, F.oy, n.y));
            textureStore(output, id.xy, textureLoad(input, at, 0));
            return;
        }
        case 8u: {
            // B-151, layer_fx::polar_coordinates. k: the share of the way, to polar, the
            // drawing's size and its corner; D-320's half-axes and their shares of its half.
            let px = x - k[4];
            let py = y - k[5];
            var qx: f64;
            var qy: f64;
            if k[1] == 1.0lf {
                let nx = quotient(px - k[2] / 2.0lf, k[6]);
                let ny = quotient(py - k[3] / 2.0lf, k[7]);
                // The CPU's atan2 of 0 and -0 is a half turn.
                var phi = 3.141592653589793lf;
                if nx != 0.0lf || ny != 0.0lf {
                    phi = atan2_64(nx, -ny);
                }
                if phi < 0.0lf {
                    phi += 6.283185307179586lf;
                }
                qx = quotient(phi, 6.283185307179586lf) * k[2];
                qy = sqrt(nx * nx + ny * ny) * k[3];
            } else {
                let a = quotient(6.283185307179586lf * px, k[2]);
                let v = quotient(py, k[3]);
                qx = k[2] / 2.0lf * (1.0lf + v * sin64(a) * k[8]);
                qy = k[3] / 2.0lf * (1.0lf - v * cos64(a) * k[9]);
            }
            qx = px + k[0] * (qx - px);
            qy = py + k[0] * (qy - py);
            if k[1] == 1.0lf {
                textureStore(output, id.xy, round_tap(qx, qy + k[5], k[2], k[4]));
                return;
            }
            sx = qx + k[4];
            sy = qy + k[5];
        }
        case 9u: {
            // B-151, layer_fx::optics_compensation. k: the centre, the radius, half the field of
            // view in radians, reverse, 0.
            let dx = x - k[0];
            let dy = y - k[1];
            let d = sqrt(product(dx, dx, k[5]) + product(dy, dy, k[5]));
            var m = 0.0lf;
            if d != 0.0lf {
                let a = product(quotient(d, k[2]), k[3], k[5]);
                if k[4] == 1.0lf {
                    m = quotient(quotient(k[2] * atan2_64(a, 1.0lf), k[3]), d);
                } else if a >= 1.5707963267948966lf {
                    textureStore(output, id.xy, vec4(0.0));
                    return;
                } else {
                    m = quotient(quotient(k[2] * quotient(sin64(a), cos64(a)), k[3]), d);
                }
            }
            sx = k[0] + m * dx;
            sy = k[1] + m * dy;
        }
        case 10u: {
            // B-151, layer_fx::corner_pin. k: the map back into the drawing by rows, its
            // determinant (0 for crossed corners), the drawing's size and corner, the drawing's
            // corner in the output, 0.
            let px = x - k[14];
            let py = y - k[15];
            let u = product(k[0], px, k[16]) + product(k[1], py, k[16]) + k[2];
            let v = product(k[3], px, k[16]) + product(k[4], py, k[16]) + k[5];
            let t = product(k[6], px, k[16]) + product(k[7], py, k[16]) + k[8];
            if !(t * k[9] > 0.0lf) {
                textureStore(output, id.xy, vec4(0.0));
                return;
            }
            sx = product(quotient(u, t), k[10], k[16]) + k[12];
            sy = product(quotient(v, t), k[11], k[16]) + k[13];
        }
        case 11u: {
            // B-151, layer_fx::radial_shadow's cast. k: the light, the scale, the growth.
            sx = k[0] + quotient(x - k[3] - k[0], k[2]);
            sy = k[1] + quotient(y - k[4] - k[1], k[2]);
        }
        case 12u: {
            // D-377, layer_fx::bend_it. k: Start, the bar's t and n, its length, the turn in
            // radians (0 straight), the radius, the centre, the turn a pixel along, the first u
            // kept, the prestart (none, static, bend, mirror), extended, then the frames at End
            // and at -L, each its point, t and n.
            let a = (x - k[0]) * k[2] + (y - k[1]) * k[3];
            let b = (x - k[0]) * k[4] + (y - k[1]) * k[5];
            let length = k[6];
            let pre = u32(k[13]);
            var acc = vec4(0.0lf, 0.0lf, 0.0lf, 0.0lf);
            if k[7] == 0.0lf {
                if (a >= 0.0lf && a <= length) || (a > length && k[14] == 1.0lf) || (a < 0.0lf && (pre == 1u || pre == 2u)) {
                    acc = bend_over(acc, bilinear(input, x, y));
                } else if a >= -length && a < 0.0lf && pre == 3u {
                    acc = bend_over(acc, bend_at(-a, b));
                }
            } else {
                if k[14] == 1.0lf {
                    let e = bend_tail(x, y, 15u);
                    if e.x > 0.0lf {
                        acc = bend_over(acc, bend_at(length + e.x, e.y));
                    }
                }
                let qx = x - k[9];
                let qy = y - k[10];
                let first = atan2_64(-(qx * k[2] + qy * k[3]), qx * k[4] + qy * k[5]);
                let size = sqrt(qx * qx + qy * qy);
                let kk = k[11];
                let lo = k[12];
                var lo_phi = kk * lo;
                var hi_phi = kk * length;
                if kk < 0.0lf {
                    lo_phi = kk * length;
                    hi_phi = kk * lo;
                }
                let pi = 3.141592653589793lf;
                let j0 = i32(floor((lo_phi - first) / pi)) - 1;
                let j1 = i32(ceil((hi_phi - first) / pi)) + 1;
                // Farthest along first: u rises with j when kk is above 0.
                var j = j1;
                var step = -1;
                if kk < 0.0lf {
                    j = j0;
                    step = 1;
                }
                for (; j >= j0 && j <= j1; j += step) {
                    let u = quotient(first + f64(j) * pi, kk);
                    var v = k[8] - size;
                    if j % 2 == 0 {
                        v = k[8] + size;
                    }
                    if lo <= u && u <= length {
                        if u < 0.0lf && pre == 3u {
                            acc = bend_over(acc, bend_at(-u, v));
                        } else {
                            acc = bend_over(acc, bend_at(u, v));
                        }
                    }
                }
                if pre == 1u && a < 0.0lf {
                    acc = bend_over(acc, bilinear(input, x, y));
                } else if pre == 2u {
                    let s = bend_tail(x, y, 21u);
                    if s.x < 0.0lf {
                        acc = bend_over(acc, bend_at(-length + s.x, s.y));
                    }
                }
            }
            textureStore(output, id.xy, vec4(f32(acc.x), f32(acc.y), f32(acc.z), f32(acc.w)));
            return;
        }
        case 13u: {
            // D-378, layer_fx::bender. k: the Base, the axis's direction and its normal, its
            // length, the push in pixels, the style (bend, marilyn, sharp, boxer).
            let s = quotient((x - k[0]) * k[2] + (y - k[1]) * k[3], k[6]);
            let a = k[7];
            var d = 0.0lf;
            switch u32(k[8]) {
                case 0u: {
                    if s > 1.0lf {
                        d = a * (2.0lf * s - 1.0lf);
                    } else if s >= 0.0lf {
                        d = a * s * s;
                    }
                }
                case 3u: {
                    if s > 1.0lf {
                        d = a;
                    } else if s >= 0.0lf {
                        d = a * (3.0lf * s * s - 2.0lf * s * s * s);
                    }
                }
                case 1u: {
                    if s >= 0.0lf && s <= 1.0lf {
                        let v = sin64(3.141592653589793lf * s);
                        d = a * (v * v);
                    }
                }
                default: {
                    if s >= 0.0lf && s <= 1.0lf {
                        d = a * (1.0lf - abs(2.0lf * s - 1.0lf));
                    }
                }
            }
            if d == 0.0lf {
                textureStore(output, id.xy, textureLoad(input, id.xy, 0));
                return;
            }
            sx = x - d * k[4];
            sy = y - d * k[5];
        }
        case 14u: {
            // D-385, layer_fx::flow_motion. k: knot 1 and its strength, knot 2 and its strength,
            // sigma squared, tile edges, the points a side.
            // n is 1, 2 or 4, so multiplying by 1 / n and 1 / n^2 is exact.
            let n = u32(k[8]);
            let step = 1.0lf / f64(n);
            var sum = vec4(0.0lf, 0.0lf, 0.0lf, 0.0lf);
            for (var j = 0u; j < n; j++) {
                for (var i = 0u; i < n; i++) {
                    let px = f64(id.x) + (f64(i) + 0.5lf) * step;
                    let py = f64(id.y) + (f64(j) + 0.5lf) * step;
                    var qx = px;
                    var qy = py;
                    for (var e = 0u; e < 6u; e += 3u) {
                        let a = k[e + 2u];
                        if a != 0.0lf {
                            let dx = px - k[e];
                            let dy = py - k[e + 1u];
                            let g = quotient(k[6], k[6] + dx * dx + dy * dy);
                            var m = 1.0lf + a * g;
                            if a < 0.0lf {
                                m = quotient(1.0lf, 1.0lf - a * g);
                            }
                            qx += dx * (m - 1.0lf);
                            qy += dy * (m - 1.0lf);
                        }
                    }
                    var s: vec4<f32>;
                    if k[7] == 1.0lf {
                        s = mirrored(qx, qy);
                    } else {
                        s = bilinear(input, qx, qy);
                    }
                    sum += vec4(f64(s.x), f64(s.y), f64(s.z), f64(s.w));
                }
            }
            let mean = sum * (step * step);
            textureStore(output, id.xy, vec4(f32(mean.x), f32(mean.y), f32(mean.z), f32(mean.w)));
            return;
        }
        case 15u: {
            // D-386, layer_fx::griddler. k: the drawing's corner, the tile, the turn's sine and
            // cosine, the scales across and down, cut tiles, 0.
            if k[5] == 0.0lf || k[6] == 0.0lf {
                textureStore(output, id.xy, vec4(0.0));
                return;
            }
            let px = x - k[0];
            let py = y - k[1];
            let t = k[2];
            let cx = (floor(quotient(px, t)) + 0.5lf) * t;
            let cy = (floor(quotient(py, t)) + 0.5lf) * t;
            let qx = px - cx;
            let qy = py - cy;
            let ux = quotient(product(qx, k[4], k[8]) + product(qy, k[3], k[8]), k[5]);
            let uy = quotient(product(qy, k[4], k[8]) - product(qx, k[3], k[8]), k[6]);
            if k[7] == 1.0lf && (abs(ux) > t / 2.0lf || abs(uy) > t / 2.0lf) {
                textureStore(output, id.xy, vec4(0.0));
                return;
            }
            sx = k[0] + cx + ux;
            sy = k[1] + cy + uy;
        }
        case 16u: {
            // D-387, layer_fx::fisheye. k: the centre, the radius, the convergence over 100, 0.
            let pi = 3.141592653589793lf;
            let dx = x - k[0];
            let dy = y - k[1];
            let r = sqrt(product(dx, dx, k[4]) + product(dy, dy, k[4]));
            let cover = clamp(k[2] - r + 0.5lf, 0.0lf, 1.0lf);
            if k[2] <= 0.0lf || cover == 0.0lf {
                textureStore(output, id.xy, vec4(0.0));
                return;
            }
            var m = 0.0lf;
            if r != 0.0lf {
                let rho = min(quotient(r, k[2]), 1.0lf);
                let c = k[3];
                var f = rho - c * (sin64(pi * rho / 2.0lf) - rho);
                if c >= 0.0lf {
                    f = rho + c * (2.0lf * atan2_64(rho, sqrt(1.0lf - rho * rho)) / pi - rho);
                }
                m = quotient(f, rho);
            }
            let s = bilinear(input, k[0] + dx * m, k[1] + dy * m);
            textureStore(output, id.xy, vec4(f32(f64(s.x) * cover), f32(f64(s.y) * cover), f32(f64(s.z) * cover), f32(f64(s.w) * cover)));
            return;
        }
        case 17u: {
            // D-389, layer_fx::power_pin. k: the map back into the square by rows, its
            // determinant, the map out of it by rows, the target ring, the drawing's size and
            // corner, the drawing's corner in the output, the perspective (0 to 1), unstretch, 0,
            // and 1 for a ring.
            let z = k[35];
            let p = k[33];
            if k[36] == 0.0lf {
                textureStore(output, id.xy, vec4(0.0));
                return;
            }
            if k[34] == 1.0lf {
                let u = quotient(x - k[29], k[27]);
                let v = quotient(y - k[30], k[28]);
                let wa = (1.0lf - u) * (1.0lf - v);
                let wb = u * (1.0lf - v);
                let wc = u * v;
                let wd = (1.0lf - u) * v;
                var px = wa * k[19] + wb * k[21] + wc * k[23] + wd * k[25];
                var py = wa * k[20] + wb * k[22] + wc * k[24] + wd * k[26];
                if p > 0.0lf {
                    let t = product(k[16], u, z) + product(k[17], v, z) + k[18];
                    if t <= 0.0lf {
                        textureStore(output, id.xy, vec4(0.0));
                        return;
                    }
                    let hx = quotient(product(k[10], u, z) + product(k[11], v, z) + k[12], t);
                    let hy = quotient(product(k[13], u, z) + product(k[14], v, z) + k[15], t);
                    px = p * hx + (1.0lf - p) * px;
                    py = p * hy + (1.0lf - p) * py;
                }
                sx = px + k[29];
                sy = py + k[30];
            } else {
                let px = x - k[31];
                let py = y - k[32];
                var u = 0.0lf;
                var v = 0.0lf;
                if p > 0.0lf {
                    let uu = product(k[0], px, z) + product(k[1], py, z) + k[2];
                    let vv = product(k[3], px, z) + product(k[4], py, z) + k[5];
                    let t = product(k[6], px, z) + product(k[7], py, z) + k[8];
                    if !(t * k[9] > 0.0lf) {
                        textureStore(output, id.xy, vec4(0.0));
                        return;
                    }
                    u = p * quotient(uu, t);
                    v = p * quotient(vv, t);
                }
                if p < 1.0lf {
                    let b = pin_unbilinear(px, py);
                    if b[0] == 0.0lf {
                        textureStore(output, id.xy, vec4(0.0));
                        return;
                    }
                    u = u + (1.0lf - p) * b[1];
                    v = v + (1.0lf - p) * b[2];
                }
                sx = product(u, k[27], z) + k[29];
                sy = product(v, k[28], z) + k[30];
            }
        }
        case 18u: {
            // D-390, layer_fx::ripple_pulse. k: the centre, n (the levels less one), the reach of
            // the whole history, the amplitude, 1 for the bump, 0, then the levels newest first.
            let dx = x - k[0];
            let dy = y - k[1];
            let r = sqrt(product(dx, dx, k[6]) + product(dy, dy, k[6]));
            let u = quotient(k[2] * r, k[3]);
            if k[5] == 1.0lf {
                let h = ripple_height(u) - k[7u + u32(k[2])];
                let v = f32(clamp(0.5lf + quotient(k[4] * h, 2000.0lf), 0.0lf, 1.0lf));
                textureStore(output, id.xy, vec4(v, v, v, 1.0));
                return;
            }
            if r == 0.0lf {
                textureStore(output, id.xy, textureLoad(input, id.xy, 0));
                return;
            }
            let s = k[4] / 10.0lf * ripple_push(u);
            sx = x - quotient(s * dx, r);
            sy = y - quotient(s * dy, r);
        }
        case 19u: {
            // D-391, layer_fx::slant. k: the floor line's height, the slant's tangent, the
            // height scale, 1 for a colour, the colour in linear light, 0.
            if k[2] <= 0.0lf {
                textureStore(output, id.xy, vec4(0.0));
                return;
            }
            let v = y - k[0];
            sx = x + product(v, k[1], k[7]);
            sy = k[0] + quotient(v, k[2]);
            if k[3] == 1.0lf {
                let a = f64(bilinear(input, sx, sy).w);
                textureStore(output, id.xy, vec4(f32(k[4] * a), f32(k[5] * a), f32(k[6] * a), f32(a)));
                return;
            }
        }
        case 20u: {
            // D-392, layer_fx::smear. k: the from point, the drag, the radius, 0.
            let z = k[5];
            let v2 = product(k[2], k[2], z) + product(k[3], k[3], z);
            let s = clamp(quotient(product(x - k[0], k[2], z) + product(y - k[1], k[3], z), v2), 0.0lf, 1.0lf);
            let ex = x - k[0] - s * k[2];
            let ey = y - k[1] - s * k[3];
            let rho = sqrt(product(ex, ex, z) + product(ey, ey, z));
            if rho < k[4] {
                let q = quotient(rho, k[4]);
                let m = (1.0lf - q * q) * (1.0lf - q * q) * s;
                sx = x - m * k[2];
                sy = y - m * k[3];
            }
        }
        case 21u: {
            // D-393, layer_fx::split. k: point A, the unit step to B, the length, the split on
            // the right walking from A to B, 0, and (D-394) the split on the left.
            let z = k[6];
            let px = x - k[0];
            let py = y - k[1];
            let along = product(px, k[2], z) + product(py, k[3], z);
            let d = product(py, k[2], z) - product(px, k[3], z);
            let t = quotient(along, k[4]);
            var wave = 0.0lf;
            if t >= 0.0lf && t <= 1.0lf {
                wave = sin64(3.141592653589793lf * t);
            }
            let g1 = k[7] / 2.0lf * wave;
            let g2 = k[5] / 2.0lf * wave;
            let cover = 1.0lf - max(min(d + 0.5lf, g2) - max(d - 0.5lf, -g1), 0.0lf);
            let g = select(g1, g2, d >= 0.0lf);
            let m = abs(d);
            let far = g + k[4] / 2.0lf;
            if m < far {
                var q = 0.0lf;
                if m >= g {
                    q = select(-1.0lf, 1.0lf, d >= 0.0lf) * (m - g) * quotient(far, far - g);
                }
                sx = k[0] + along * k[2] - q * k[3];
                sy = k[1] + along * k[3] + q * k[2];
            }
            let s = bilinear(input, sx, sy);
            textureStore(output, id.xy, vec4(f32(f64(s.x) * cover), f32(f64(s.y) * cover), f32(f64(s.z) * cover), f32(f64(s.w) * cover)));
            return;
        }
        case 22u: {
            // D-408, layer_fx::transform, one moment. k: the inverse map a to f, the opacity, 0,
            // and 1 for bicubic sampling.
            let z = k[7];
            sx = product(k[0], x, z) + product(k[1], y, z) + k[2];
            sy = product(k[3], x, z) + product(k[4], y, z) + k[5];
            var s = bilinear(input, sx, sy);
            if k[8] == 1.0lf {
                s = bicubic(input, sx, sy);
            }
            textureStore(output, id.xy, s * f32(k[6]));
            return;
        }
        case 25u: {
            // D-404, layer_fx::tiles. k: where a tile starts across, the tile's width, the
            // scale, where one starts down, its height, the points across and down, the blend
            // (0 to 1), 0. Places counted from the pixel's corner, as the CPU's; the points summed in the CPU's order, single precision as its are.
            let n = textureDimensions(input);
            let mx = u32(k[5]);
            let my = u32(k[6]);
            var sum = vec4(0.0);
            for (var b = 0u; b < my; b++) {
                let ry = tile_read(f64(id.y) + quotient(f64(b) + 0.5lf, k[6]), k[3], k[4], k[2], f64(n.y));
                for (var a = 0u; a < mx; a++) {
                    let rx = tile_read(f64(id.x) + quotient(f64(a) + 0.5lf, k[5]), k[0], k[1], k[2], f64(n.x));
                    sum += bilinear(input, rx, ry);
                }
            }
            let t = sum / f32(mx * my);
            let bl = f32(k[7]);
            textureStore(output, id.xy, t * (1.0 - bl) + textureLoad(input, id.xy, 0) * bl);
            return;
        }
        case 29u: {
            // D-407, layer_fx::detail_upscale's Lanczos sum, the output the grown layer. `input`
            // is the picture, grown by F.r when softened first. k: 0, then for each output
            // column and then each output row its first tap and six weights. A tap outside the
            // picture is clear; summed across, then down, in the CPU's order and precision. F.flag
            // 1: the covering held to 0..1 and each colour to 0..covering, with no Detail after.
            let n = vec2<i32>(textureDimensions(input)) - vec2(2 * F.r);
            let cx = 1u + 7u * id.x;
            let cy = 1u + 7u * (size.x + id.y);
            let fx = i32(k[cx]);
            let fy = i32(k[cy]);
            var sum = vec4(0.0);
            for (var j = 0; j < 6; j++) {
                let yy = fy + j;
                if yy < 0 || yy >= n.y {
                    continue;
                }
                var across = vec4(0.0);
                for (var i = 0; i < 6; i++) {
                    let xx = fx + i;
                    if xx < 0 || xx >= n.x {
                        continue;
                    }
                    across += f32(k[cx + 1u + u32(i)]) * textureLoad(input, vec2(xx, yy) + vec2(F.r), 0);
                }
                sum += f32(k[cy + 1u + u32(j)]) * across;
            }
            if F.flag == 1u {
                let al = min(max(sum.w, 0.0), 1.0);
                sum = vec4(min(max(sum.xyz, vec3(0.0)), vec3(al)), al);
            }
            textureStore(output, id.xy, sum);
            return;
        }
        case 23u: {
            // D-405, layer_fx::magnify, the output grown by F.g on every side. k: the centre,
            // the magnification (a share), the radius, the feather, the opacity (a share), the
            // scatter's reach, square, the scaling (0 standard, 1 soft, 2 scatter), the blend
            // (0 none, 1 normal, 2 multiply, 3 screen, 4 add, 5 overlay, 6 soft light), the
            // drawing's corner, 0. Each product rounded once, as the CPU's are.
            let px = vec2<i32>(id.xy) - vec2(F.g);
            let vx = f64(px.x) + 0.5lf - k[0];
            let vy = f64(px.y) + 0.5lf - k[1];
            var d = max(abs(vx), abs(vy));
            var inside = d <= k[3];
            if k[7] == 0.0lf {
                let s = product(vx, vx, k[12]) + product(vy, vy, k[12]);
                d = sqrt(s);
                inside = s <= product(k[3], k[3], k[12]);
            }
            var cover = select(0.0lf, 1.0lf, inside);
            if k[4] != 0.0lf {
                cover = clamp(quotient(k[3] - d, k[4]), 0.0lf, 1.0lf);
            }
            let o = at(input, px);
            var a = vec4(0.0);
            if cover > 0.0lf {
                var qx = k[0] + quotient(vx, k[2]);
                var qy = k[1] + quotient(vy, k[2]);
                var p: vec4<f32>;
                if k[8] == 1.0lf {
                    p = bilinear(input, qx, qy);
                } else {
                    if k[8] == 2.0lf {
                        let hx = px.x - i32(k[10]);
                        let hy = px.y - i32(k[11]);
                        qx += product(k[6], hashed(hx, hy, 0, 0u), k[12]);
                        qy += product(k[6], hashed(hx, hy, 0, 1u), k[12]);
                    }
                    p = at(input, vec2<i32>(i32(floor(qx)), i32(floor(qy))));
                }
                a = p * f32(product(cover, k[5], k[12]));
            }
            textureStore(output, id.xy, laid(a, o, u32(k[9])));
            return;
        }
        case 26u: {
            // D-413, layer_fx::checker_cover laid as layer_fx::lay_pattern lays it. k: the
            // anchor, the cell's width and height, the ramps (half of max(feather, 1)) across and
            // down, the colour (linear), the opacity (a share), the blend (as `laid`'s).
            let cover = (1.0lf + checker_axis(x, k[0], k[2], k[4]) * checker_axis(y, k[1], k[3], k[5])) / 2.0lf * k[9];
            let s = vec4(f32(k[6] * cover), f32(k[7] * cover), f32(k[8] * cover), f32(cover));
            textureStore(output, id.xy, laid(s, textureLoad(input, id.xy, 0), u32(k[10])));
            return;
        }
        case 27u: {
            // D-414, layer_fx::circle_cover laid as Checkerboard's. k: the centre, the outer and
            // inner radii, their ramps, the colour (linear), the opacity (a share), the blend (as
            // `laid`'s), 1 to invert.
            let d = sqrt((x - k[0]) * (x - k[0]) + (y - k[1]) * (y - k[1]));
            var c = clamp(quotient(k[2] - d, k[4]) + 0.5lf, 0.0lf, 1.0lf);
            if k[3] > 0.0lf {
                c = c * clamp(quotient(d - k[3], k[5]) + 0.5lf, 0.0lf, 1.0lf);
            }
            let cover = select(c, 1.0lf - c, k[11] != 0.0lf) * k[9];
            let s = vec4(f32(k[6] * cover), f32(k[7] * cover), f32(k[8] * cover), f32(cover));
            textureStore(output, id.xy, laid(s, textureLoad(input, id.xy, 0), u32(k[10])));
            return;
        }
        case 28u: {
            // D-415, layer_fx::ellipse. k: the centre, the half axes, half the thickness, the
            // softness, 1 alone, the inside and outside colours (linear).
            let u = x - k[0];
            let v = y - k[1];
            let g = sqrt(u * u / (k[2] * k[2] * k[2] * k[2]) + v * v / (k[3] * k[3] * k[3] * k[3]));
            var d = min(k[2], k[3]);
            if g != 0.0lf {
                let e = sqrt(u * u / (k[2] * k[2]) + v * v / (k[3] * k[3]));
                d = abs(e - 1.0lf) * e / g;
            }
            let r = k[4];
            let sw = max(2.0lf * r * k[5] / 100.0lf, 1.0lf);
            let c = clamp((min(d + sw / 2.0lf, r) - max(d - sw / 2.0lf, -r)) / sw, 0.0lf, 1.0lf);
            var q = 1.0lf;
            if r != 0.0lf {
                q = min(d / r, 1.0lf);
            }
            let px = textureLoad(input, id.xy, 0);
            let keep = select(1.0lf - c, 0.0lf, k[6] == 1.0lf);
            var out = px;
            for (var j = 0u; j < 3u; j++) {
                out[j] = f32(f64(px[j]) * keep + ((1.0lf - q) * k[7u + j] + q * k[10u + j]) * c);
            }
            out.w = f32(f64(px.w) * keep + c);
            textureStore(output, id.xy, out);
            return;
        }
        case 31u: {
            // D-417, layer_fx::grid_cover laid as Checkerboard's. k: the anchor, the cell's width
            // and height, the border, the boxes (max(feather, 1)) across and down, the colour
            // (linear), the opacity (a share), the blend (as `laid`'s), 1 to invert.
            let gx = grid_axis(x, k[0], k[2], k[4], k[5]);
            let gy = grid_axis(y, k[1], k[3], k[4], k[6]);
            let c = gx + gy - gx * gy;
            let cover = select(c, 1.0lf - c, k[12] != 0.0lf) * k[10];
            let s = vec4(f32(k[7] * cover), f32(k[8] * cover), f32(k[9] * cover), f32(cover));
            textureStore(output, id.xy, laid(s, textureLoad(input, id.xy, 0), u32(k[11])));
            return;
        }
        case 30u: {
            // D-416, layer_fx::fractal. k: as `fractal_point`'s.
            let dx = x - k[6];
            let dy = y - k[7];
            let b = fractal_band(dx, dy);
            let n0 = fractal_band(dx - 1.0lf, dy);
            let n2 = fractal_band(dx, dy - 1.0lf);
            let edge = n0 != b || n2 != b || fractal_band(dx + 1.0lf, dy) != b || fractal_band(dx, dy + 1.0lf) != b;
            let f = u32(k[23]);
            var p = fractal_colour(b);
            if f > 1u && (k[22] == 1.0lf || edge) {
                var total = vec4(0.0lf);
                for (var bb = 0u; bb < f; bb++) {
                    let sy = dy - 0.5lf + quotient(f64(bb) + 0.5lf, k[23]);
                    for (var aa = 0u; aa < f; aa++) {
                        total += fractal_colour(fractal_band(dx - 0.5lf + quotient(f64(aa) + 0.5lf, k[23]), sy));
                    }
                }
                p = total / f64(f * f);
            } else if f == 1u && k[21] == 1.0lf && (n0 != b || n2 != b) {
                p = vec4(1.0lf);
            }
            if k[24] == 1.0lf {
                var inside = false;
                if k[13] == 1.0lf {
                    inside = fractal_escape(vec2(0.0lf), fractal_point(dx, dy, 0u), u32(k[10])) < 0;
                } else {
                    inside = fractal_escape(fractal_point(dx, dy, 3u), vec2(k[0], k[1]), u32(k[11])) < 0;
                }
                if inside {
                    p = (p + vec4(1.0lf)) / 2.0lf;
                }
                let x0 = i32(id.x) - i32(k[6]);
                let y0 = i32(id.y) - i32(k[7]);
                if fractal_cross(x0, y0) {
                    p = vec4(1.0lf);
                } else if fractal_cross(x0 - 1, y0 - 1) {
                    p = vec4(0.0lf, 0.0lf, 0.0lf, 1.0lf);
                }
            }
            textureStore(output, id.xy, vec4(f32(p.x), f32(p.y), f32(p.z), f32(p.w)));
            return;
        }
        case 33u: {
            // D-423, layer_fx::light_share: the drawing times its share of Light Rays' source.
            // k: the centre, the radius, 1 for a square, its turn's sine and cosine.
            let dx = x - k[0];
            let dy = y - k[1];
            var d = sqrt(dx * dx + dy * dy);
            if k[3] == 1.0lf {
                d = max(abs(dx * k[5] + dy * k[4]), abs(-dx * k[4] + dy * k[5]));
            }
            let m = clamp(k[2] + 0.5lf - d, 0.0lf, 1.0lf);
            let p = textureLoad(input, id.xy, 0);
            textureStore(output, id.xy, vec4(f32(f64(p.x) * m), f32(f64(p.y) * m), f32(f64(p.z) * m), f32(f64(p.w) * m)));
            return;
        }
        case 34u: {
            // D-424, layer_fx::glue_gun. k: r, R, T0, the look's length, 1 for a point light, its
            // place and height, the distant light's direction, the light in linear light, the
            // intensity, ambient, diffuse, specular, roughness, metal, the blobs' count, then each
            // blob's centre, then for each band of 16 rows where its list of blobs starts and how
            // many, then the lists. The highlight's power is single precision.
            let r = k[0];
            let nb = u32(k[20]);
            let band = 21u + 2u * nb + 2u * (id.y / 16u);
            let first = u32(k[band]);
            let count = u32(k[band + 1u]);
            var f = 0.0lf;
            var gx = 0.0lf;
            var gy = 0.0lf;
            for (var i = 0u; i < count; i++) {
                let o = 21u + 2u * u32(k[first + i]);
                let dx = x - k[o];
                let dy = y - k[o + 1u];
                let q = (dx * dx + dy * dy) / (k[1] * k[1]);
                if q < 1.0lf {
                    f += (1.0lf - q) * (1.0lf - q);
                    gx += -4.0lf * (1.0lf - q) * dx / (k[1] * k[1]);
                    gy += -4.0lf * (1.0lf - q) * dy / (k[1] * k[1]);
                }
            }
            let p = textureLoad(input, id.xy, 0);
            let g = sqrt(gx * gx + gy * gy);
            var a = 0.0lf;
            var n = vec3(0.0lf, 0.0lf, 1.0lf);
            if g > 0.0lf {
                let s = (f - k[2]) / g;
                let m = clamp(1.0lf - s / r, 0.0lf, 1.0lf);
                a = clamp(s + 0.5lf, 0.0lf, 1.0lf);
                n = vec3(-m * gx / g, -m * gy / g, sqrt(1.0lf - m * m));
            } else if f > k[2] {
                a = 1.0lf;
            }
            if a == 0.0lf {
                textureStore(output, id.xy, p);
                return;
            }
            let seen = bilinear(input, x - k[3] * n.x, y - k[3] * n.y);
            var col = vec3(0.0lf, 0.0lf, 0.0lf);
            if seen.w > 0.0 {
                col = vec3(f64(seen.x) / f64(seen.w), f64(seen.y) / f64(seen.w), f64(seen.z) / f64(seen.w));
            }
            var l = vec3(k[8], k[9], k[10]);
            if k[4] == 1.0lf {
                l = blob_unit(vec3(k[5] - x, k[6] - y, k[7]));
            }
            let nl = n.x * l.x + n.y * l.y + n.z * l.z;
            let rz = 2.0lf * nl * n.z - l.z;
            var shine = 0.0lf;
            if nl > 0.0lf && rz > 0.0lf {
                shine = f64(pow(f32(rz), f32(1.0lf / k[18])));
            }
            var out = vec4(0.0);
            for (var c = 0u; c < 3u; c++) {
                let cc = col[c];
                let lc = k[11u + c];
                let lit = cc * (k[15] + k[16] * k[14] * lc * max(nl, 0.0lf)) + k[17] * k[14] * (lc + (cc - lc) * k[19]) * shine;
                out[c] = f32(lit * a + (1.0lf - a) * f64(p[c]));
            }
            out.w = f32(a + (1.0lf - a) * f64(p.w));
            textureStore(output, id.xy, out);
            return;
        }
        case 35u: {
            // D-425, layer_fx::threads. k: the centre, the turn's sine and cosine, the spacing
            // across and down, the overlaps, coverage, shadowing, texture.
            let dx = x - k[0];
            let dy = y - k[1];
            let u = dx * k[3] + dy * k[2];
            let v = -dx * k[2] + dy * k[3];
            let ti = floor(u / k[4]);
            let tj = floor(v / k[5]);
            var e = vec2(u - (ti + 0.5lf) * k[4], v - (tj + 0.5lf) * k[5]);
            var hf = vec2(k[7] * k[4] / 2.0lf, k[7] * k[5] / 2.0lf);
            let q = ti + tj;
            if q - 2.0lf * k[6] * floor(q / (2.0lf * k[6])) >= k[6] {
                e = e.yx;
                hf = hf.yx;
            }
            var m = vec2(0.0lf);
            var sh = vec2(1.0lf);
            for (var t = 0u; t < 2u; t++) {
                m[t] = max(min(e[t] + 0.5lf, hf[t]) - max(e[t] - 0.5lf, -hf[t]), 0.0lf);
                if hf[t] > 0.0lf {
                    sh[t] = 1.0lf - k[10] * min(e[t] * e[t] / (hf[t] * hf[t]), 1.0lf);
                }
            }
            var g = 0.0lf;
            if hf.x > 0.0lf {
                g = clamp(1.0lf - (abs(e.x) - hf.x) / hf.x, 0.0lf, 1.0lf);
            }
            let kk = m.x * sh.x + m.y * sh.y * (1.0lf - k[8] * (1.0lf + g) / 2.0lf) * (1.0lf - m.x);
            let a = m.x + m.y * (1.0lf - m.x);
            let p = textureLoad(input, id.xy, 0);
            textureStore(output, id.xy, vec4(f32(f64(p.x) * kk), f32(f64(p.y) * kk), f32(f64(p.z) * kk), f32(f64(p.w) * a)));
            return;
        }
        case 36u: {
            textureStore(output, id.xy, flare_lay(textureLoad(input, id.xy, 0), flare_light(x, y)));
            return;
        }
        case 100u: {
            // D-461, layer_fx::hex_tile_at then hex_tile's fold: k as hex_tile_setup lays it,
            // every product rounded before the add it goes into, as the CPU's are.
            let z = k[26];
            let dx = x - k[2];
            let dy = y - k[3];
            let c = k[4];
            let s = k[5];
            let u = product(c, dx, z) + product(s, dy, z);
            let v = product(c, dy, z) - product(s, dx, z);
            let qf = product(u, k[6], z);
            let rf = product(u, k[7], z) + product(v, k[8], z);
            let t = array<f64, 3>(qf, -qf - rf, rf);
            var r = array<f64, 3>(floor(t[0] + 0.5lf), floor(t[1] + 0.5lf), floor(t[2] + 0.5lf));
            let g0 = abs(r[0] - t[0]);
            let g1 = abs(r[1] - t[1]);
            let g2 = abs(r[2] - t[2]);
            if g0 > g1 && g0 > g2 {
                r[0] = -r[1] - r[2];
            } else if g1 > g2 {
                r[1] = -r[0] - r[2];
            } else {
                r[2] = -r[0] - r[1];
            }
            let mu = product(k[9], r[0], z);
            let mv = product(k[10], r[2] + r[0] * 0.5lf, z);
            var lu = u - mu;
            var lv = v - mv;
            if k[12] != 0.0lf {
                let h = k[13];
                let e1 = lv * 0.5lf - product(h, lu, z);
                let e2 = -lv * 0.5lf - product(h, lu, z);
                var n = 5u;
                if lv >= 0.0lf {
                    n = select(select(2u, 1u, e2 < 0.0lf), 0u, e1 < 0.0lf);
                } else if e1 > 0.0lf {
                    n = 3u;
                } else if e2 > 0.0lf {
                    n = 4u;
                }
                let ck = k[14u + 2u * n];
                let sk = k[15u + 2u * n];
                let a = product(ck, lu, z) + product(sk, lv, z);
                let b = product(ck, lv, z) - product(sk, lu, z);
                lu = a;
                lv = b;
                if k[12] == 2.0lf && n % 2u == 1u {
                    let a2 = lu * 0.5lf + product(h, lv, z);
                    let b2 = product(h, lu, z) - lv * 0.5lf;
                    lu = a2;
                    lv = b2;
                }
            }
            let m = k[11];
            sx = k[0] + product(c, lu, z) - product(s, lv, z) + product(m, product(c, mu, z) - product(s, mv, z), z);
            sy = k[1] + product(s, lu, z) + product(c, lv, z) + product(m, product(s, mu, z) + product(c, mv, z), z);
            sx = hex_fold(sx, f64(size.x), z);
            sy = hex_fold(sy, f64(size.y), z);
        }
        case 151u: {
            // D-601, mesh::draw_field, for D-605's Liquify. k as mesh::field_numbers lays it: a
            // 0, the cells across and down, the drawing's size and corner, the share, the
            // offset, 0, 0; then each grid corner's step, row by row. Every product rounded once,
            // as the CPU's are.
            let z = k[0];
            let gx = k[1];
            let gy = k[2];
            let px = x - k[5];
            let py = y - k[6];
            let fx = clamp(product(quotient(px - k[8], k[3]), gx, z), 0.0lf, gx);
            let fy = clamp(product(quotient(py - k[9], k[4]), gy, z), 0.0lf, gy);
            let i = min(floor(fx), gx - 1.0lf);
            let j = min(floor(fy), gy - 1.0lf);
            let u = fx - i;
            let v = fy - j;
            let n = u32(gx) + 1u;
            let at = 12u + 2u * (u32(j) * n + u32(i));
            let below = at + 2u * n;
            let dx = product(1.0lf - v, product(1.0lf - u, k[at], z) + product(u, k[at + 2u], z), z)
                + product(v, product(1.0lf - u, k[below], z) + product(u, k[below + 2u], z), z);
            let dy = product(1.0lf - v, product(1.0lf - u, k[at + 1u], z) + product(u, k[at + 3u], z), z)
                + product(v, product(1.0lf - u, k[below + 1u], z) + product(u, k[below + 3u], z), z);
            sx = px + product(k[7], dx, z) + k[5];
            sy = py + product(k[7], dy, z) + k[6];
        }
        case 150u: {
            // D-601, mesh::draw, for D-602's Bezier Warp and the forward meshes after it. k as
            // mesh::mesh_numbers lays it: a 0, the columns and rows, the drawing's size and
            // corner, the tiles across, where the points, the offsets and the list start.
            let z = k[0];
            let at = mesh_locate(x - k[5], y - k[6], (id.y / 16u) * u32(k[7]) + id.x / 16u);
            if at.x == 0.0lf {
                textureStore(output, id.xy, vec4(0.0));
                return;
            }
            sx = product(quotient(at.y, k[1]), k[3], z) + k[5];
            sy = product(quotient(at.z, k[2]), k[4], z) + k[6];
        }
        case 103u: {
            // D-466, layer_fx::strobe_light while it is on: k as strobe_setup lays it, then a 0
            // for `product`; worked in double precision as the CPU's is.
            let p = textureLoad(input, vec2<i32>(id.xy), 0);
            let b = k[3];
            if k[5] != 0.0lf {
                // Each channel on its own: vec4<f64>(p) * b came back as nothing on this card.
            textureStore(output, id.xy, vec4<f32>(f32(f64(p.x) * b), f32(f64(p.y) * b), f32(f64(p.z) * b), f32(f64(p.a) * b)));
                return;
            }
            let a = f64(p.a);
            if a <= 0.0lf {
                textureStore(output, id.xy, p);
                return;
            }
            let c = vec3<f64>(quotient(f64(p.x), a), quotient(f64(p.y), a), quotient(f64(p.z), a));
            let r = strobe_operate(u32(k[4]), c, vec3<f64>(k[0], k[1], k[2]), k[6]);
            textureStore(output, id.xy, vec4<f32>(vec3<f32>(((1.0lf - b) * r + b * c) * a), p.a));
            return;
        }
        case 102u: {
            // D-465, layer_fx::scatter: k[0] the reach, k[1] the Grain; the move worked in
            // whole numbers as the CPU's is.
            let n = vec2<i32>(textureDimensions(input));
            let r = u32(k[0]);
            let span = 2u * r + 1u;
            var d = vec2<i32>(0);
            if k[1] != 2.0lf {
                d.x = i32(scatter_hash(id.x, id.y, 0u) % span) - i32(r);
            }
            if k[1] != 1.0lf {
                d.y = i32(scatter_hash(id.x, id.y, 1u) % span) - i32(r);
            }
            let q = vec2<i32>(id.xy) + d;
            if q.x < 0 || q.y < 0 || q.x >= n.x || q.y >= n.y {
                textureStore(output, id.xy, vec4<f32>(0.0));
            } else {
                textureStore(output, id.xy, textureLoad(input, q, 0));
            }
            return;
        }
        case 101u: {
            // D-464, layer_fx::repeat_tile: k as repeat_tile_setup lays it, every choice made in
            // whole numbers as the CPU's are; F.ox and F.oy are the growth.
            let n = vec2<i32>(textureDimensions(input));
            let px = i32(id.x) - F.ox;
            let py = i32(id.y) - F.oy;
            let at = repeat_at(px, py, n);
            if at.x < 0 {
                textureStore(output, id.xy, vec4<f32>(0.0));
                return;
            }
            let uv = vec2<i32>(at.x, at.y);
            let p = textureLoad(input, uv, 0);
            let wx = repeat_weight(uv.x, n.x);
            let wy = repeat_weight(uv.y, n.y);
            if wx == 0.0 && wy == 0.0 {
                textureStore(output, id.xy, p);
                return;
            }
            let u2 = (uv.x + n.x / 2) % n.x;
            let v2 = (uv.y + n.y / 2) % n.y;
            let b = textureLoad(input, vec2(u2, uv.y), 0);
            let c = textureLoad(input, vec2(uv.x, v2), 0);
            let e = textureLoad(input, vec2(u2, v2), 0);
            textureStore(output, id.xy, (1.0 - wy) * ((1.0 - wx) * p + wx * b) + wy * ((1.0 - wx) * c + wx * e));
            return;
        }
        default: {
            // k: the centre, the jolt across and down, the turn's sine and cosine.
            let vx = x - f64(F.g) - k[0] - k[2];
            let vy = y - f64(F.g) - k[1] - k[3];
            sx = k[0] + vx * k[5] + vy * k[4];
            sy = k[1] - vx * k[4] + vy * k[5];
        }
    }
    textureStore(output, id.xy, bilinear(input, sx, sy));
}

// D-388, layer_fx::page_turn. k: the fold's step n across and down, its offset, the radius,
// the light along n, the back's opacity (0 to 1), the paper in linear light, the render (0 full,
// 1 front, 2 back), 1 when `other` is the back (a map lying on the drawing), the drawing's
// corner, 0. Each channel in double precision on its own.
fn page_at(x0: f64, x1: f64, d: f64, s: f64) -> array<f64, 4> {
    let p = bilinear(input, x0 + (s - d) * k[0] + k[11], x1 + (s - d) * k[1] + k[12]);
    return array<f64, 4>(f64(p.x), f64(p.y), f64(p.z), f64(p.w));
}

fn page_turned(x0: f64, x1: f64, d: f64, s: f64, shade: f64) -> array<f64, 4> {
    var f = page_at(x0, x1, d, s);
    var m = array<f64, 4>(k[6], k[7], k[8], 1.0lf);
    if k[10] == 1.0lf {
        let b = bilinear(other, x0 + (s - d) * k[0], x1 + (s - d) * k[1]);
        m = array<f64, 4>(f64(b.x), f64(b.y), f64(b.z), f64(b.w));
    }
    let o = k[5];
    var out = array<f64, 4>(0.0lf, 0.0lf, 0.0lf, f[3]);
    for (var c = 0; c < 3; c++) {
        out[c] = (f[3] * o * m[c] + (1.0lf - o * m[3]) * f[c]) * shade;
    }
    return out;
}

@compute @workgroup_size(16, 16)
fn pageturn(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let pi = 3.141592653589793lf;
    let x0 = f64(id.x) + 0.5lf - k[11];
    let x1 = f64(id.y) + 0.5lf - k[12];
    let r = k[3];
    let d = product(x0, k[0], k[13]) + product(x1, k[1], k[13]) - k[2];
    var top = array<f64, 4>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    var under = array<f64, 4>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    if !(d > r || (r == 0.0lf && d >= 0.0lf)) {
        if d >= 0.0lf {
            let t = min(quotient(d, r), 1.0lf);
            let c = sqrt(1.0lf - t * t);
            let shade = clamp(k[4] * t + c, 0.0lf, 1.0lf);
            let a = atan2_64(t, c);
            top = page_turned(x0, x1, d, r * (pi - a), shade);
            under = page_at(x0, x1, d, r * a);
        } else {
            top = page_turned(x0, x1, d, pi * r - d, 1.0lf);
            under = page_at(x0, x1, d, d);
        }
    }
    var out = array<f64, 4>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    for (var c = 0; c < 4; c++) {
        if k[9] == 1.0lf {
            out[c] = under[c];
        } else if k[9] == 2.0lf {
            out[c] = top[c];
        } else {
            out[c] = top[c] + (1.0lf - top[3]) * under[c];
        }
    }
    textureStore(output, id.xy, vec4(f32(out[0]), f32(out[1]), f32(out[2]), f32(out[3])));
}

// B-107, the four wipes: layer_fx::linear_wipe (mode 0), radial_wipe (1, `count` the way round),
// venetian_blinds (2) and iris_wipe (3); D-510's layer_fx::tile_wipe (4); D-511's
// layer_fx::teeth_wipe (5), which moves the two parts and returns early; D-513's
// layer_fx::glow_wipe (6), which adds its light and returns early; D-514's
// layer_fx::stripe_sweep (7); D-515's layer_fx::push_iris (8) and D-516's
// layer_fx::stretch_wipe (9), which read between pixels and return early. `flag` is complete,
// every pixel gone.
@compute @workgroup_size(16, 16)
fn wipe(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    if F.flag == 1u {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    // What is kept runs from 0 to 1 over the feather, `s` the place and `edge` where it is half
    // kept; feather 0 is a hard step.
    var s: f64;
    var edge: f64;
    var feather: f64;
    switch F.mode {
        case 0u: {
            // k: the direction across, the edge, the feather, 0, each product rounded on its own
            // as the CPU rounds it.
            s = product(k[0], x, k[4]) + product(k[1], y, k[4]);
            edge = k[2];
            feather = k[3];
        }
        case 1u: {
            // k: the centre, the start angle, the edge and the feather, in degrees.
            let vx = x - k[0];
            let vy = y - k[1];
            var screen = 0.0lf;
            if !(vx == 0.0lf && vy == 0.0lf) {
                screen = euclid(atan2_64(vx, -vy) * 57.29577951308232lf, 360.0lf);
                screen = select(screen, 0.0lf, screen >= 360.0lf);
            }
            var a = euclid(screen - k[2], 360.0lf);
            a = select(a, 0.0lf, a >= 360.0lf);
            s = a;
            if F.count == 1u {
                s = select(0.0lf, 360.0lf - a, a > 0.0lf);
            } else if F.count == 2u {
                s = 2.0lf * min(a, 360.0lf - a);
            }
            edge = k[3];
            feather = k[4];
        }
        case 2u: {
            // k: the direction across, the slats' width, the edge, the feather, 0, each product
            // rounded on its own as the CPU rounds it.
            let along = product(k[0], x, k[5]) + product(k[1], y, k[5]);
            s = along - product(k[2], floor(along / k[2]), k[5]);
            edge = k[3];
            feather = k[4];
        }
        case 4u: {
            // D-510, layer_fx::tile_keep: k as layer_fx::tile_numbers; the kept share runs over
            // the border from the shrinking tile's edge.
            let dx = x - k[0];
            let dy = y - k[1];
            let qx = dx * k[2] + dy * k[3];
            let qy = -dx * k[3] + dy * k[2];
            let tx = floor(qx / k[4] + 0.5lf) * k[4];
            let ty = floor(qy / k[4] + 0.5lf) * k[4];
            let m = max(abs(qx - tx), abs(qy - ty));
            var d = abs(tx);
            if k[7] == 1.0lf {
                d = sqrt(tx * tx + ty * ty);
            } else if k[7] == 2.0lf {
                d = max(abs(tx), abs(ty));
            }
            var u = 0.0lf;
            if k[5] > 0.0lf {
                u = min(d / k[5], 1.0lf);
            }
            if k[8] == 1.0lf {
                u = 1.0lf - u;
            }
            let p = clamp((k[6] - u * 0.75lf) / 0.25lf, 0.0lf, 1.0lf);
            let a = (1.0lf - p) * (k[4] / 2.0lf + k[9]) - k[9] / 2.0lf;
            // Border 0: kept where m < a, strictly, as the CPU keeps it.
            s = select(a - m, select(-1.0lf, 1.0lf, m < a), k[9] == 0.0lf);
            edge = 0.0lf;
            feather = k[9];
        }
        case 5u: {
            // D-511, layer_fx::teeth_side: k as layer_fx::teeth_numbers; each part read from
            // across the cut, the gap clear.
            let dx = x - k[0];
            let dy = y - k[1];
            let v = dx * k[2] + dy * k[3];
            let w = -dx * k[3] + dy * k[2];
            let f = v / k[5];
            let t = abs(2.0lf * (f - floor(f)) - 1.0lf);
            var sh = t;
            if k[7] == 1.0lf {
                sh = clamp(2.0lf * t - 0.5lf, 0.0lf, 1.0lf);
            } else if k[7] == 2.0lf {
                sh = select(0.0lf, 1.0lf, t > 0.5lf);
            }
            let g = k[4] * (sh - 0.5lf);
            var m = 0.0lf;
            if w + k[6] < g {
                m = k[6];
            } else if w - k[6] >= g {
                m = -k[6];
            } else {
                textureStore(output, id.xy, vec4(0.0));
                return;
            }
            let bx = f64(id.x) + 0.5lf;
            let by = f64(id.y) + 0.5lf;
            textureStore(output, id.xy, bilinear(input, bx + m * -k[3], by + m * k[2]));
            return;
        }
        case 9u: {
            // D-516, layer_fx::stretch_from: k as layer_fx::stretch_numbers; behind the line the
            // pixel itself, past it read between pixels.
            let dx = x - k[0];
            let dy = y - k[1];
            let u = dx * k[2] + dy * k[3];
            let v = -dx * k[3] + dy * k[2];
            if u <= 0.0lf {
                textureStore(output, id.xy, textureLoad(input, id.xy, 0));
                return;
            }
            let a = u / k[4];
            let tx = k[0] + a * k[2] - v * k[3];
            let ty = k[1] + a * k[3] + v * k[2];
            textureStore(output, id.xy, bilinear(input, tx + f64(F.ox), ty + f64(F.oy)));
            return;
        }
        case 8u: {
            // D-515, layer_fx::push_from: k as layer_fx::push_numbers; read between pixels, the
            // hole (or outside the iris) clear.
            let dx = x - k[0];
            let dy = y - k[1];
            let r = sqrt(dx * dx + dy * dy);
            var tx = k[0] + k[3];
            var ty = k[1];
            if k[4] == 1.0lf {
                if r >= k[2] - k[3] {
                    textureStore(output, id.xy, vec4(0.0));
                    return;
                }
                if r != 0.0lf {
                    let f = (r + k[3]) / r;
                    tx = k[0] + dx * f;
                    ty = k[1] + dy * f;
                }
            } else {
                if r < k[3] {
                    textureStore(output, id.xy, vec4(0.0));
                    return;
                }
                let f = (r - k[3]) / r;
                tx = k[0] + dx * f;
                ty = k[1] + dy * f;
            }
            textureStore(output, id.xy, bilinear(input, tx + f64(F.ox), ty + f64(F.oy)));
            return;
        }
        case 7u: {
            // D-514, layer_fx::stripe_place: k as layer_fx::stripe_numbers; kept where the place
            // along the line is not below how far the line has gone.
            let dx = x - k[0];
            let dy = y - k[1];
            let u = dx * k[2] + dy * k[3];
            let v = -dx * k[3] + dy * k[2];
            let i = min(floor((v + k[5]) / k[6]), k[7] - 1.0lf);
            var t = (u + k[4]) / (2.0lf * k[4]);
            if k[11] == 1.0lf {
                t = 1.0lf - t;
            }
            s = t;
            edge = clamp((k[10] - i * k[9] * (1.0lf - k[8])) / k[9], 0.0lf, 1.0lf);
            feather = 0.0lf;
        }
        case 6u: {
            // D-513, layer_fx::glow_pixel: k as layer_fx::glow_numbers; the layer kept or
            // cleared, the band's light added. Scalars, not a vec4<f64>.
            let dx = x - k[0];
            let dy = y - k[1];
            let u = dx * k[2] + dy * k[3];
            let v = -dx * k[3] + dy * k[2];
            var d = abs(u);
            if k[4] == 1.0lf {
                d = sqrt(u * u + v * v);
            } else if k[4] == 2.0lf {
                d = max(abs(u), abs(v));
            }
            let s = select(d, k[5] - d, k[9] == 1.0lf);
            let keep = select(0.0lf, 1.0lf, s >= k[7]);
            let g = max(0.0lf, 1.0lf - abs(s - k[7]) / k[6]) * k[8];
            let p = textureLoad(input, id.xy, 0);
            let pr = f64(p.x);
            let pg = f64(p.y);
            let pb = f64(p.z);
            let pa = f64(p.w);
            var lr = k[11] * g;
            var lg = k[12] * g;
            var lb = k[13] * g;
            var cover = min(g, 1.0lf);
            if k[10] == 1.0lf {
                lr = pr * g;
                lg = pg * g;
                lb = pb * g;
                cover = min(g * pa, 1.0lf);
            }
            let qa = pa * keep;
            textureStore(output, id.xy, vec4(f32(pr * keep + lr), f32(pg * keep + lg), f32(pb * keep + lb), f32(qa + (1.0lf - qa) * cover)));
            return;
        }
        default: {
            // k: the centre, the radius, the feather, invert.
            let dx = x - k[0];
            let dy = y - k[1];
            let d = sqrt(dx * dx + dy * dy);
            s = select(k[2] - d, d - k[2], k[4] == 1.0lf);
            edge = 0.0lf;
            feather = k[3];
        }
    }
    var kept = select(0.0lf, 1.0lf, s >= edge);
    if feather > 0.0lf {
        kept = clamp((s - edge) / feather + 0.5lf, 0.0lf, 1.0lf);
    }
    textureStore(output, id.xy, textureLoad(input, id.xy, 0) * f32(kept));
}

// B-107, layer_fx::mosaic's blocks, one a thread, each block's mean into `row`, summed in the
// CPU's order. k: the columns of blocks, the rows, then where each column starts, the end last,
// then where each row starts, the end last.
// ponytail: one thread sums a whole block, slow for blocks of many thousands of pixels; a
// reduction in steps if Mosaic's big sizes are ever slow on the card.
@compute @workgroup_size(16, 16)
fn blocks(@builtin(global_invocation_id) id: vec3<u32>) {
    let nc = u32(k[0]);
    let nr = u32(k[1]);
    if id.x >= nc || id.y >= nr {
        return;
    }
    let x0 = u32(k[2u + id.x]);
    let x1 = u32(k[3u + id.x]);
    let y0 = u32(k[3u + nc + id.y]);
    let y1 = u32(k[4u + nc + id.y]);
    // Four scalars, not a vec4<f64>, which this driver reads back as zero here.
    var r = 0.0lf;
    var g = 0.0lf;
    var b = 0.0lf;
    var a = 0.0lf;
    for (var y = y0; y < y1; y++) {
        for (var x = x0; x < x1; x++) {
            let p = textureLoad(input, vec2(x, y), 0);
            r += f64(p.x);
            g += f64(p.y);
            b += f64(p.z);
            a += f64(p.w);
        }
    }
    let n = f64((y1 - y0) * (x1 - x0));
    let i = 4u * (id.y * nc + id.x);
    row[i] = f32(r / n);
    row[i + 1u] = f32(g / n);
    row[i + 2u] = f32(b / n);
    row[i + 3u] = f32(a / n);
}

// D-449: the two blocks, of the `n` whose starts are k[base..], either side of pixel centre `p`
// in block `c`, and how far between their centres; the edge block alone past the end ones.
struct TilePair {
    a: u32,
    b: u32,
    t: f64,
}

fn tile_pair(base: u32, n: u32, c: u32, p: f64) -> TilePair {
    var a = c;
    var b = c + 1u;
    if p < (k[base + c] + k[base + c + 1u]) / 2.0lf {
        if c == 0u {
            return TilePair(0u, 0u, 0.0lf);
        }
        a = c - 1u;
        b = c;
    } else if b >= n {
        return TilePair(c, c, 0.0lf);
    }
    let ca = (k[base + a] + k[base + a + 1u]) / 2.0lf;
    let cb = (k[base + b] + k[base + b + 1u]) / 2.0lf;
    return TilePair(a, b, (p - ca) / (cb - ca));
}

// Each pixel its block's mean. k as for `blocks`, then each column's block, then each row's.
// D-449: mode 1 blends the means between the blocks' centres, as layer_fx::block_picture.
@compute @workgroup_size(16, 16)
fn tiles(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let nc = u32(k[0]);
    let nr = u32(k[1]);
    let start = 4u + nc + nr;
    let col = u32(k[start + id.x]);
    let rw = u32(k[start + size.x + id.y]);
    if F.mode == 1u {
        let x = tile_pair(2u, nc, col, f64(id.x) + 0.5lf);
        let y = tile_pair(3u + nc, nr, rw, f64(id.y) + 0.5lf);
        var out = vec4(0.0);
        for (var c = 0u; c < 4u; c++) {
            let top = (1.0lf - x.t) * f64(row[4u * (y.a * nc + x.a) + c]) + x.t * f64(row[4u * (y.a * nc + x.b) + c]);
            let bottom = (1.0lf - x.t) * f64(row[4u * (y.b * nc + x.a) + c]) + x.t * f64(row[4u * (y.b * nc + x.b) + c]);
            out[c] = f32((1.0lf - y.t) * top + y.t * bottom);
        }
        textureStore(output, id.xy, out);
        return;
    }
    let i = 4u * (rw * nc + col);
    textureStore(output, id.xy, vec4(row[i], row[i + 1u], row[i + 2u], row[i + 3u]));
}

// D-449, layer_fx::block_load: rows above k[0] from `input`, the pass under way; the rest from
// `other`, the pass before, or (mode 1, Start Cleared's first pass) empty.
@compute @workgroup_size(16, 16)
fn blockload(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var p = textureLoad(input, id.xy, 0);
    if f64(id.y) >= k[0] {
        p = select(textureLoad(other, id.xy, 0), vec4(0.0), F.mode == 1u);
    }
    textureStore(output, id.xy, p);
}

// D-460, layer_fx::burn_film: k the burn front, the diagonal, then the seven burn points' x, y
// and delay; F.base the seed's mix.
@compute @workgroup_size(16, 16)
fn burnfilm(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let d = k[1];
    var near = sqrt((x - k[2]) * (x - k[2]) + (y - k[3]) * (y - k[3])) / d + k[4];
    for (var i = 1u; i < 7u; i++) {
        let q = 2u + 3u * i;
        near = min(near, sqrt((x - k[q]) * (x - k[q]) + (y - k[q + 1u]) * (y - k[q + 1u])) / d + k[q + 2u]);
    }
    let n = fractal(0u, vec3(x / (d / 10.0lf), y / (d / 10.0lf), 0.0lf), 6u);
    let g = near + 0.12lf * (n + 1.0lf) / 2.0lf - k[0];
    var out = p;
    if g >= 0.06lf {
        out = p;
    } else if g >= 0.0lf {
        let t = g / 0.06lf;
        out = vec4(f32(f64(p.x) * t), f32(f64(p.y) * t), f32(f64(p.z) * t), p.w);
    } else if g > -0.02lf {
        out = vec4(0.0, 0.0, 0.0, f32(f64(p.w) * (1.0lf + g / 0.02lf)));
    } else {
        out = vec4(0.0);
    }
    textureStore(output, id.xy, out);
}

// D-462, layer_fx::mr_smoothie: k the phase over 360, the colour loop (ab, ba, aba, bab), Sample
// A's x and y, Sample B's, and a 0 for the rounded products; `other` the height, its corner F.r
// pixels out, as `vblur`'s.
@compute @workgroup_size(16, 16)
fn smoothie(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let a = f64(textureLoad(input, id.xy, 0).w);
    if a <= 0.0lf {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let z = k[6];
    let v = clamp(height(vec2<i32>(id.xy)), 0.0lf, 1.0lf);
    var t = v + k[0];
    if t < 0.0lf || t > 1.0lf {
        t = t - floor(t);
    }
    let lp = u32(k[1]);
    var s = t;
    if lp == 1u {
        s = 1.0lf - t;
    } else if lp == 2u {
        s = 1.0lf - abs(2.0lf * t - 1.0lf);
    } else if lp == 3u {
        s = abs(2.0lf * t - 1.0lf);
    }
    let px = clamp(k[2] + product(s, k[4] - k[2], z), 0.5lf, f64(size.x) - 0.5lf);
    let py = clamp(k[3] + product(s, k[5] - k[3], z), 0.5lf, f64(size.y) - 0.5lf);
    let q = bilinear(input, px, py);
    let qa = f64(q.w);
    var out = vec4(0.0, 0.0, 0.0, f32(a));
    if qa > 0.0lf {
        out = vec4(f32(quotient(f64(q.x), qa) * a), f32(quotient(f64(q.y), qa) * a), f32(quotient(f64(q.z), qa) * a), f32(a));
    }
    textureStore(output, id.xy, out);
}

// D-467, layer_fx::texturize: the drawing `input` with the relief of the texture's luminance
// (`other`'s alpha, the drawing's size) one pixel ahead and behind, each sample held inside the
// pixel centres. k: the step along the light across and down, the contrast over 2.
@compute @workgroup_size(16, 16)
fn texturize(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let fw = f64(size.x);
    let fh = f64(size.y);
    let ahead = f64(bilinear(other, min(max(x + k[0], 0.5lf), fw - 0.5lf), min(max(y + k[1], 0.5lf), fh - 0.5lf)).w);
    let behind = f64(bilinear(other, min(max(x - k[0], 0.5lf), fw - 0.5lf), min(max(y - k[1], 0.5lf), fh - 0.5lf)).w);
    let v = (ahead - behind) * k[2];
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let e = to_srgb(clamp(f64(p[c]) / a, 0.0lf, 1.0lf)) + v;
        out[c] = f32(to_linear(clamp(e, 0.0lf, 1.0lf)) * a);
    }
    textureStore(output, id.xy, out);
}

// D-470, layer_fx::contour_flow's luminance held inside the picture: `input`'s alpha at `p`.
fn cp_lum(p: vec2<i32>) -> f64 {
    let size = vec2<i32>(textureDimensions(input));
    return f64(textureLoad(input, clamp(p, vec2(0), size - vec2(1)), 0).w);
}

// D-470, layer_fx::contour_flow's tensor: Sobel's slopes over 8 of the luminance (`input`'s
// alpha), each neighbour held inside, as (gx², gx gy, gy²).
@compute @workgroup_size(16, 16)
fn cptensor(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let c = vec2<i32>(id.xy);
    let gx = (cp_lum(c + vec2(1, -1)) + 2.0lf * cp_lum(c + vec2(1, 0)) + cp_lum(c + vec2(1, 1)) - cp_lum(c + vec2(-1, -1))
        - 2.0lf * cp_lum(c + vec2(-1, 0)) - cp_lum(c + vec2(-1, 1))) * 0.125lf;
    let gy = (cp_lum(c + vec2(-1, 1)) + 2.0lf * cp_lum(c + vec2(0, 1)) + cp_lum(c + vec2(1, 1)) - cp_lum(c + vec2(-1, -1))
        - 2.0lf * cp_lum(c + vec2(0, -1)) - cp_lum(c + vec2(1, -1))) * 0.125lf;
    textureStore(output, id.xy, vec4(f32(gx * gx), f32(gx * gy), f32(gy * gy), 0.0));
}

// D-470, layer_fx::contour_tangent on the blurred tensor `input`: the flow across and down and
// its strength. k: the turn's cosine and sine, then F.n directions across and down.
@compute @workgroup_size(16, 16)
fn cptangent(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let t = textureLoad(input, id.xy, 0);
    let a = f64(t.x);
    let b = f64(t.y);
    let c = f64(t.z);
    let r = sqrt((a - c) * (a - c) + 4.0lf * b * b);
    var tx = b;
    var ty = -(a - c + r) * 0.5lf;
    if a < c {
        tx = -(c - a + r) * 0.5lf;
        ty = b;
    }
    let n = sqrt(tx * tx + ty * ty);
    let e = min(4.0lf * sqrt(r), 1.0lf);
    if n == 0.0lf {
        textureStore(output, id.xy, vec4(0.0, 0.0, f32(e), 0.0));
        return;
    }
    let ux = quotient(tx, n);
    let uy = quotient(ty, n);
    let rx = ux * k[0] - uy * k[1];
    let ry = ux * k[1] + uy * k[0];
    var ox = rx;
    var oy = ry;
    var best = -1.0lf;
    for (var j = 0u; j < F.n; j++) {
        let wx = k[2u + 2u * j];
        let wy = k[3u + 2u * j];
        let d = abs(rx * wx + ry * wy);
        if d > best {
            best = d;
            ox = wx;
            oy = wy;
        }
    }
    textureStore(output, id.xy, vec4(f32(ox), f32(oy), f32(e), 0.0));
}

// D-470, layer_fx::contour_way: the flow (`other`) at (x, y), each of the four pixels round it
// held inside and turned to face (dx, dy) first.
struct CpWay {
    x: f64,
    y: f64,
}

fn cp_way(x: f64, y: f64, dx: f64, dy: f64) -> CpWay {
    let size = vec2<i32>(textureDimensions(other));
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;
    var out = CpWay(0.0lf, 0.0lf);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0lf - uy, uy, j == 1);
        let qy = clamp(i32(y0) + j, 0, size.y - 1);
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0lf - ux, ux, i == 1);
            let qx = clamp(i32(x0) + i, 0, size.x - 1);
            let t = textureLoad(other, vec2(qx, qy), 0);
            let tx = f64(t.x);
            let ty = f64(t.y);
            let sgn = select(1.0lf, -1.0lf, tx * dx + ty * dy < 0.0lf);
            out.x += sgn * tx * wx * wy;
            out.y += sgn * ty * wx * wy;
        }
    }
    return out;
}

// D-470, layer_fx::contour_smear: one pass of the smear of `input` along the flow `other`. k: the
// F.n step weights; F.flag 1 keeps the covering.
@compute @workgroup_size(16, 16)
fn cpsmear(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let f = textureLoad(other, id.xy, 0);
    let tx = f64(f.x);
    let ty = f64(f.y);
    let e = f64(f.z);
    if tx == 0.0lf && ty == 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let fw = f64(size.x);
    let fh = f64(size.y);
    var acc = array<f64, 4>(f64(p.x), f64(p.y), f64(p.z), f64(p.w));
    var total = 1.0lf;
    for (var side = 0; side < 2; side++) {
        let s = select(1.0lf, -1.0lf, side == 1);
        var x = f64(id.x) + 0.5lf;
        var y = f64(id.y) + 0.5lf;
        var dx = s * tx;
        var dy = s * ty;
        for (var j = 0u; j < F.n; j++) {
            x = clamp(x + dx, 0.5lf, fw - 0.5lf);
            y = clamp(y + dy, 0.5lf, fh - 0.5lf);
            let q = bilinear(input, x, y);
            let wt = k[j];
            acc[0] += f64(q.x) * wt;
            acc[1] += f64(q.y) * wt;
            acc[2] += f64(q.z) * wt;
            acc[3] += f64(q.w) * wt;
            total += wt;
            let m = cp_way(x, y, dx, dy);
            let ln = sqrt(m.x * m.x + m.y * m.y);
            if ln <= 0.000001lf {
                break;
            }
            dx = quotient(m.x, ln);
            dy = quotient(m.y, ln);
        }
    }
    var res = array<f64, 4>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    for (var c = 0; c < 4; c++) {
        let o = f64(p[c]);
        res[c] = o + e * (quotient(acc[c], total) - o);
    }
    var out = vec4(f32(res[0]), f32(res[1]), f32(res[2]), f32(res[3]));
    if F.flag == 1u {
        out.w = p.w;
        for (var c = 0; c < 3; c++) {
            out[c] = select(0.0, f32(quotient(res[c], res[3]) * f64(p.w)), res[3] > 0.0lf);
        }
    }
    textureStore(output, id.xy, out);
}

// D-470, layer_fx::contour_fill: `input` filled from the palette where the flow strength
// (`other`'s blue) times 100 lies from k[0] to k[1]. k[2] the palette's n colours, then each
// encoded, then each linear; F.flag bit 1 clears elsewhere. D-584: with bit 2 each is matched in
// CIE L*a*b* (layer_fx::contour_lab), the n keys given in it.
@compute @workgroup_size(16, 16)
fn cpfill(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    let v = 100.0lf * f64(textureLoad(other, id.xy, 0).z);
    if a > 0.0lf && k[0] <= v && v <= k[1] {
        let n = u32(k[2]);
        let straight = vec3(clamp(quotient(f64(p.x), a), 0.0lf, 1.0lf), clamp(quotient(f64(p.y), a), 0.0lf, 1.0lf), clamp(quotient(f64(p.z), a), 0.0lf, 1.0lf));
        var r = to_srgb(straight.x);
        var g = to_srgb(straight.y);
        var b = to_srgb(straight.z);
        if (F.flag & 2u) != 0u {
            let l = sh_lab(straight);
            r = 100.0lf * l.x;
            g = 128.0lf * l.y;
            b = 128.0lf * l.z;
        }
        var best = 0u;
        var near = 0.0lf;
        for (var j = 0u; j < n; j++) {
            let dr = k[3u + 3u * j] - r;
            let dg = k[4u + 3u * j] - g;
            let db = k[5u + 3u * j] - b;
            let d = dr * dr + dg * dg + db * db;
            if j == 0u || d < near {
                near = d;
                best = j;
            }
        }
        let base = 3u + 3u * n + 3u * best;
        textureStore(output, id.xy, vec4(f32(k[base] * a), f32(k[base + 1u] * a), f32(k[base + 2u] * a), p.w));
    } else if (F.flag & 1u) != 0u {
        textureStore(output, id.xy, vec4(0.0));
    } else {
        textureStore(output, id.xy, p);
    }
}

// D-470, layer_fx::contour_edges: the Edge Map view of the flow `input`. k: the cell, the pixel,
// (D-585) 1 for the heads.
@compute @workgroup_size(16, 16)
fn cpedge(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let cell = k[0];
    let px = k[1];
    let g = to_linear(f64(textureLoad(input, id.xy, 0).z));
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let cx = (floor(quotient(x, cell)) + 0.5lf) * cell;
    let cy = (floor(quotient(y, cell)) + 0.5lf) * cell;
    let q = vec2(min(u32(floor(cx)), size.x - 1u), min(u32(floor(cy)), size.y - 1u));
    let t = textureLoad(input, q, 0);
    let tx = f64(t.x);
    let ty = f64(t.y);
    var cov = 0.0lf;
    if tx != 0.0lf || ty != 0.0lf {
        let reach = 0.4lf * cell;
        let vx = x - cx;
        let vy = y - cy;
        let along = clamp(vx * tx + vy * ty, -reach, reach);
        let ax = vx - along * tx;
        let ay = vy - along * ty;
        var d = sqrt(ax * ax + ay * ay);
        if k[2] == 1.0lf {
            let wx = vx - reach * tx;
            let wy = vy - reach * ty;
            for (var i = 0; i < 2; i++) {
                let sg = select(-1.0lf, 1.0lf, i == 0);
                let bx = -tx * 0.8660254037844386lf + ty * sg * 0.5lf;
                let by = -tx * sg * 0.5lf - ty * 0.8660254037844386lf;
                let a = clamp(wx * bx + wy * by, 0.0lf, 0.2lf * cell);
                let hx = wx - a * bx;
                let hy = wy - a * by;
                d = min(d, sqrt(hx * hx + hy * hy));
            }
        }
        cov = f64(t.z) * clamp(1.0lf - quotient(d, px), 0.0lf, 1.0lf);
    }
    textureStore(output, id.xy, vec4(f32(g + cov * (0.0lf - g)), f32(g + cov * (1.0lf - g)), f32(g + cov * (1.0lf - g)), 1.0));
}

// D-463, layer_fx::plastic's cut of the bump `v`: k[0] lo, k[1] hi.
fn plastic_cut(v: f64) -> f64 {
    if k[1] == k[0] {
        return select(0.0lf, 1.0lf, v >= k[0]);
    }
    return clamp(quotient(v - k[0], k[1] - k[0]), 0.0lf, 1.0lf);
}

// D-463, layer_fx::plastic: the drawing `input` lit on the cut bump (`other`'s alpha, grown by
// F.r), its own colour and covering kept. k: lo, hi, the slope's scale, 1 for a point light, its
// place and the height, the distant light's direction, the light and the ambient light in linear
// light, the intensity, ambient, diffuse, dust, specular, roughness, metal. The highlight's power
// is single precision, as `blobby`'s.
@compute @workgroup_size(16, 16)
fn plastic(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let o = textureLoad(input, id.xy, 0);
    let a = f64(o.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let x = i32(id.x) + F.r;
    let y = i32(id.y) + F.r;
    let gx = (plastic_cut(f64(at(other, vec2(x + 1, y)).w)) - plastic_cut(f64(at(other, vec2(x - 1, y)).w))) / 2.0lf;
    let gy = (plastic_cut(f64(at(other, vec2(x, y + 1)).w)) - plastic_cut(f64(at(other, vec2(x, y - 1)).w))) / 2.0lf;
    let n = blob_unit(vec3(-k[2] * gx, -k[2] * gy, 1.0lf));
    var l = vec3(k[7], k[8], k[9]);
    if k[3] == 1.0lf {
        l = blob_unit(vec3(k[4] - (f64(id.x) + 0.5lf), k[5] - (f64(id.y) + 0.5lf), k[6]));
    }
    let nl = n.x * l.x + n.y * l.y + n.z * l.z;
    var dusty = 0.0lf;
    if nl > 0.0lf {
        dusty = quotient(nl, nl + n.z);
    }
    let rz = 2.0lf * nl * n.z - l.z;
    var shine = 0.0lf;
    if nl > 0.0lf && rz > 0.0lf {
        shine = f64(pow(f32(rz), f32(1.0lf / k[21])));
    }
    var out = vec4(0.0, 0.0, 0.0, o.w);
    for (var c = 0u; c < 3u; c++) {
        let cc = quotient(f64(o[c]), a);
        let lc = k[10u + c];
        let lit = cc * (k[17] * k[13u + c] + k[18] * k[16] * lc * max(nl, 0.0lf) + k[19] * k[16] * lc * dusty) + k[20] * k[16] * (lc + (cc - lc) * k[22]) * shine;
        out[c] = f32(lit * a);
    }
    textureStore(output, id.xy, out);
}

// layer_fx::picture_luma.
fn picture_luma(p: vec4<f32>) -> f64 {
    return to_srgb(clamp(0.2126lf * f64(p.x) + 0.7152lf * f64(p.y) + 0.0722lf * f64(p.z), 0.0lf, 1.0lf));
}

// B-107, layer_fx::emboss (mode 0; k: the relief across and down, the contrast as a share,
// colour) and find_edges (mode 1; k: the amount as a share, invert); D-368, layer_fx::kernel
// (mode 2; k: the nine numbers row by row, the divider, absolute values).
@compute @workgroup_size(16, 16)
fn relief(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    var out = p;
    if F.mode == 0u {
        let x = f64(id.x) + 0.5lf;
        let y = f64(id.y) + 0.5lf;
        let fw = f64(size.x);
        let fh = f64(size.y);
        let ahead = picture_luma(bilinear(input, min(max(x + k[0], 0.5lf), fw - 0.5lf), min(max(y + k[1], 0.5lf), fh - 0.5lf)));
        let behind = picture_luma(bilinear(input, min(max(x - k[0], 0.5lf), fw - 0.5lf), min(max(y - k[1], 0.5lf), fh - 0.5lf)));
        let v = 0.5lf + (ahead - behind) * k[2];
        for (var c = 0u; c < 3u; c++) {
            var e = v;
            if k[3] == 1.0lf {
                e = to_srgb(clamp(f64(p[c]) / a, 0.0lf, 1.0lf)) + v - 0.5lf;
            }
            out[c] = f32(to_linear(clamp(e, 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    let most = vec2<i32>(size) - vec2(1);
    let q = vec2<i32>(id.xy);
    if F.mode == 2u {
        var sum = vec3(0.0lf);
        for (var j = 0u; j < 3u; j++) {
            for (var i = 0u; i < 3u; i++) {
                let n = textureLoad(input, clamp(q + vec2(i32(i) - 1, i32(j) - 1), vec2(0), most), 0);
                if n.w > 0.0 {
                    let na = f64(n.w);
                    for (var c = 0u; c < 3u; c++) {
                        sum[c] += k[j * 3u + i] * to_srgb(clamp(f64(n[c]) / na, 0.0lf, 1.0lf));
                    }
                }
            }
        }
        for (var c = 0u; c < 3u; c++) {
            var u = sum[c] / k[9];
            if k[10] != 0.0lf {
                u = abs(u);
            }
            out[c] = f32(to_linear(clamp(u, 0.0lf, 1.0lf)) * a);
        }
        textureStore(output, id.xy, out);
        return;
    }
    // l[j * 3 + i] is the pixel i - 1 across and j - 1 down, held inside the layer.
    var l: array<f64, 9>;
    for (var j = 0; j < 3; j++) {
        for (var i = 0; i < 3; i++) {
            l[j * 3 + i] = picture_luma(textureLoad(input, clamp(q + vec2(i - 1, j - 1), vec2(0), most), 0));
        }
    }
    let gx = l[2] + 2.0lf * l[5] + l[8] - l[0] - 2.0lf * l[3] - l[6];
    let gy = l[6] + 2.0lf * l[7] + l[8] - l[0] - 2.0lf * l[1] - l[2];
    let m = min(sqrt(gx * gx + gy * gy) / 2.0lf, 1.0lf);
    let v = select(1.0lf - m, m, k[1] == 1.0lf);
    for (var c = 0u; c < 3u; c++) {
        let e = to_srgb(clamp(f64(p[c]) / a, 0.0lf, 1.0lf));
        out[c] = f32(to_linear(clamp(e + k[0] * (v - e), 0.0lf, 1.0lf)) * a);
    }
    textureStore(output, id.xy, out);
}

// B-107, layer_fx::sharpen (mode 0) and diffusion (mode 1, `blend` 0 normal, 1 screen, 2
// lighten): `other` is the picture blurred, grown by `r`. k: the amount as a share; for
// diffusion (D-364) also the second amount as a share and its mixer, 4 overlay, 5 soft light.
@compute @workgroup_size(16, 16)
fn sharp(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    // D-407, layer_fx::detail_upscale's Detail (mode 2), on the premultiplied numbers as they
    // are: p + k[0] (p - blurred), then the covering held to 0..1 and each colour to 0..covering.
    if F.mode == 2u {
        let g = textureLoad(other, vec2<i32>(id.xy) + vec2(F.r), 0);
        let u = p + f32(k[0]) * (p - g);
        let al = min(max(u.w, 0.0), 1.0);
        textureStore(output, id.xy, vec4(min(max(u.xyz, vec3(0.0)), vec3(al)), al));
        return;
    }
    let a = f64(p.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let g = textureLoad(other, vec2<i32>(id.xy) + vec2(F.r), 0);
    let ga = f64(g.w);
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        if F.mode == 0u {
            let e = to_srgb(clamp(f64(p[c]) / a, 0.0lf, 1.0lf));
            var eb = e;
            if ga > 0.0lf {
                eb = to_srgb(clamp(f64(g[c]) / ga, 0.0lf, 1.0lf));
            }
            out[c] = f32(to_linear(clamp(e + k[0] * (e - eb), 0.0lf, 1.0lf)) * a);
        } else {
            let b = f64(p[c]) / a;
            var s = b;
            if ga > 0.0lf {
                s = f64(g[c]) / ga;
            }
            var f = s;
            if F.blend == 1u {
                f = 1.0lf - (1.0lf - b) * (1.0lf - s);
            } else if F.blend == 2u {
                f = max(b, s);
            }
            var r = b + k[0] * (f - b);
            if k[1] > 0.0lf {
                let m = mixed_with(u32(k[2]), to_srgb(clamp(r, 0.0lf, 1.0lf)), to_srgb(clamp(s, 0.0lf, 1.0lf)));
                r = r + k[1] * (to_linear(m) - r);
            }
            out[c] = f32(r * a);
        }
    }
    textureStore(output, id.xy, out);
}

// D-400, grade::shadow_highlight_lab: straight linear colour to CIE L*a*b* on sRGB's own white,
// scaled as darktable scales it. The cube root starts single precision and takes two Newton steps.
fn sh_f(t: f64) -> f64 {
    let d = 6.0lf / 29.0lf;
    if t > d * d * d {
        var y = f64(pow(f32(t), 1.0 / 3.0));
        for (var i = 0; i < 2; i++) {
            y = y - (y * y * y - t) / (3.0lf * y * y);
        }
        return y;
    }
    return t / (3.0lf * d * d) + 4.0lf / 29.0lf;
}

fn sh_lab(b: vec3<f64>) -> vec3<f64> {
    let x = sh_f((0.4124lf * b.x + 0.3576lf * b.y + 0.1805lf * b.z) / 0.9505lf);
    let y = sh_f((0.2126lf * b.x + 0.7152lf * b.y + 0.0722lf * b.z) / 1.0lf);
    let z = sh_f((0.0193lf * b.x + 0.1192lf * b.y + 0.9505lf * b.z) / 1.089lf);
    return vec3((116.0lf * y - 16.0lf) / 100.0lf, 500.0lf * (x - y) / 128.0lf, 200.0lf * (y - z) / 128.0lf);
}

// D-400, grade::shadow_highlight_unlab.
fn sh_finv(u: f64) -> f64 {
    let d = 6.0lf / 29.0lf;
    if u > d {
        return u * u * u;
    }
    return 3.0lf * d * d * (u - 4.0lf / 29.0lf);
}

fn sh_unlab(v: vec3<f64>) -> vec3<f64> {
    let fy = (100.0lf * v.x + 16.0lf) / 116.0lf;
    let x = 0.9505lf * sh_finv(fy + 128.0lf * v.y / 500.0lf);
    let y = 1.0lf * sh_finv(fy);
    let z = 1.089lf * sh_finv(fy - 128.0lf * v.z / 200.0lf);
    return vec3(
        3.2406254773200533lf * x + -1.5372079722103187lf * y + -0.4986285986982479lf * z,
        -0.9689307147293194lf * x + 1.875756060885241lf * y + 0.04151752384295394lf * z,
        0.05571012044551061lf * x + -0.2040210505984867lf * y + 1.0569959422543882lf * z);
}

fn sh_straight(p: vec4<f32>, a: f64) -> vec3<f64> {
    return vec3(clamp(f64(p.x) / a, 0.0lf, 1.0lf), clamp(f64(p.y) / a, 0.0lf, 1.0lf), clamp(f64(p.z) / a, 0.0lf, 1.0lf));
}

fn sh_recip(v: f64) -> f64 {
    if abs(v) > 1e-6lf {
        return 1.0lf / v;
    }
    return select(1e6lf, -1e6lf, v < 0.0lf);
}

// D-400, grade::shadow_highlight's first step: each pixel's lightness (L* / 100) in red, green
// and blue, covering 1; 0 where the drawing does not show.
@compute @workgroup_size(16, 16)
fn shprep(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    var l = 0.0;
    if a > 0.0lf {
        l = f32(sh_lab(sh_straight(p, a)).x);
    }
    textureStore(output, id.xy, vec4(l, l, l, 1.0));
}

// D-400, grade::shadow_highlight_pixel: `other` the lightness blurred at the shadow radius (red)
// and at the highlight radius (green). k: the shadow and highlight amounts as 2 x / 100, the
// shadow and highlight compressions, the colour correction as a share, (D-563) the midtone
// contrast as a share.
@compute @workgroup_size(16, 16)
fn shhi(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let g = textureLoad(other, id.xy, 0);
    var v = sh_lab(sh_straight(p, a));
    var l = v.x;
    var pa = v.y;
    var qb = v.z;
    for (var side = 0u; side < 2u; side++) {
        let shadows = side == 1u;
        let t = select(1.0lf - f64(g.y), 1.0lf - f64(g.x), shadows);
        var x = 1.0lf - t / (1.0lf - k[3]);
        if shadows {
            x = t / (1.0lf - k[2]) - k[2] / (1.0lf - k[2]);
        }
        x = clamp(x, 0.0lf, 1.0lf);
        let near = select(1.0lf - k[4], k[4], shadows);
        let far = select(k[4], 1.0lf - k[4], shadows);
        let amount = select(k[1], k[0], shadows);
        var n = amount * amount;
        while n > 0.0lf {
            let la = l;
            let lb = (t - 0.5lf) * select(1.0lf, -1.0lf, 1.0lf - la < 0.0lf) + 0.5lf;
            let o = min(n, 1.0lf) * x;
            n = n - 1.0lf;
            var laid = 2.0lf * la * lb;
            if la > 0.5lf {
                laid = 1.0lf - (1.0lf - 2.0lf * (la - 0.5lf)) * (1.0lf - lb);
            }
            l = la * (1.0lf - o) + laid * o;
            let f = l * sh_recip(la) * near + (1.0lf - l) * sh_recip(1.0lf - la) * far;
            pa = pa * (1.0lf - o) + pa * f * o;
            qb = qb * (1.0lf - o) + qb * f * o;
        }
    }
    // D-563, grade::shadow_highlight_follow: the midtone contrast, the chroma following it.
    let l2 = l + k[5] * l * (1.0lf - l) * (2.0lf * l - 1.0lf);
    var ck = 1.0lf;
    if l2 < l {
        ck = l2 / l;
    } else if l2 > l {
        ck = (1.0lf - l2) / (1.0lf - l);
    }
    l = l2;
    pa = pa * ck;
    qb = qb * ck;
    let o = sh_unlab(vec3(l, pa, qb));
    var out = p;
    out.x = f32(clamp(o.x, 0.0lf, 1.0lf) * a);
    out.y = f32(clamp(o.y, 0.0lf, 1.0lf) * a);
    out.z = f32(clamp(o.z, 0.0lf, 1.0lf) * a);
    textureStore(output, id.xy, out);
}

// B-107, layer_fx::simple_choker's last step: `band` is the least (`flag`, shrinking) or the
// greatest covering within reach, the output the drawing grown by `g`; a spread's empty pixels
// take the average within reach from each row's running totals in `sums`. k: each disc row's
// offset and half-width; `count` the rows.
@compute @workgroup_size(16, 16)
fn choke(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let w = i32(textureDimensions(input).x);
    let h = i32(textureDimensions(input).y);
    let x = i32(id.x) - F.g;
    let y = i32(id.y) - F.g;
    let a = band[id.y * size.x + id.x];
    let p = at(input, vec2(x, y));
    if p.w > 0.0 {
        let m = f64(a) / f64(p.w);
        textureStore(output, id.xy, vec4(vec3<f32>(vec3<f64>(p.xyz) * m), a));
        return;
    }
    if F.flag == 1u {
        textureStore(output, id.xy, p);
        return;
    }
    if a == 0.0 {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    var sum = vec4<f64>(0.0lf);
    for (var j = 0u; j < F.count; j++) {
        let sy = y + i32(k[2u * j]);
        let hw = i32(k[2u * j + 1u]);
        let x0 = max(x - hw, 0);
        let x1 = min(x + hw + 1, w);
        if sy < 0 || sy >= h || x0 >= x1 {
            continue;
        }
        let base = u32(sy * (w + 1));
        sum += sums[base + u32(x1)] - sums[base + u32(x0)];
    }
    textureStore(output, id.xy, vec4(vec3<f32>(sum.xyz / sum.w * f64(a)), a));
}

// B-107, layer_fx::speed_lines. k: the centre, the opacity as a share, the colour in linear
// light, the widest half-width, the count, then each line's angle, half-width and inner edge, in
// angle order.
@compute @workgroup_size(16, 16)
fn lines(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    if p.w == 0.0 {
        textureStore(output, id.xy, p);
        return;
    }
    let vx = f64(id.x) + 0.5lf - k[0];
    let vy = f64(id.y) + 0.5lf - k[1];
    let d = sqrt(vx * vx + vy * vy);
    let alpha = euclid(atan2_64(vx, -vy) * 57.29577951308232lf, 360.0lf);
    let n = u32(k[7]);
    // A line more than `reach` degrees round gives nothing here; at the centre every line is in
    // reach.
    var reach = 1e300lf;
    if d > 0.0lf {
        reach = k[6] + 90.0lf / (3.141592653589793lf * d) + 1e-9lf;
    }
    // The first line at or past the pixel's angle.
    var lo = 0u;
    var hi = n;
    while lo < hi {
        let mid = (lo + hi) / 2u;
        if k[8u + 3u * mid] < alpha {
            lo = mid + 1u;
        } else {
            hi = mid;
        }
    }
    var q = 0.0lf;
    // Forward from `lo`, then back from the line before it, each way stopping out of reach.
    for (var way = 0u; way < 2u; way++) {
        for (var j = 0u; j < n; j++) {
            var l = (lo + j) % n;
            var gap = euclid(k[8u + 3u * l] - alpha, 360.0lf);
            if way == 1u {
                l = (lo + 2u * n - j - 1u) % n;
                gap = euclid(alpha - k[8u + 3u * l], 360.0lf);
            }
            if gap > reach {
                break;
            }
            let turn = euclid(alpha - k[8u + 3u * l], 360.0lf);
            let one = clamp((k[9u + 3u * l] - min(turn, 360.0lf - turn)) * 3.141592653589793lf / 180.0lf * d + 0.5lf, 0.0lf, 1.0lf)
                * clamp(d - k[10u + 3u * l] + 0.5lf, 0.0lf, 1.0lf);
            q = max(q, one);
        }
    }
    let op = q * k[2];
    let a = f64(p.w);
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let v = f64(p[c]);
        out[c] = f32(v + op * (k[3u + c] * a - v));
    }
    textureStore(output, id.xy, out);
}

// B-107, layer_fx::cross_glare's last step: `other` is the bright pixels, the output the drawing
// grown by `g`, the length. k: the intensity, the colour in linear light, the arms' count, each
// arm's direction and D-546 tint (red, green, blue, covering; effects::Glint::tints), then each
// step's distance and weight.
// ponytail: every sample is taken, where the CPU skips the dark ones; a lit-pixel count like the
// CPU's if long glares on big layers are ever slow on the card.
@compute @workgroup_size(16, 16)
fn glare(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = i32(id.x) - F.g;
    let y = i32(id.y) - F.g;
    let cx = f64(x) + 0.5lf;
    let cy = f64(y) + 0.5lf;
    let arms = u32(k[4]);
    let steps = 5u + 6u * arms;
    var g = vec4<f32>(0.0);
    for (var j = 0u; j < arms; j++) {
        let a = 5u + 6u * j;
        let vx = f32(k[a]);
        let vy = f32(k[a + 1u]);
        let tint = vec4(f32(k[a + 2u]), f32(k[a + 3u]), f32(k[a + 4u]), f32(k[a + 5u]));
        for (var t = 0u; t < u32(F.g); t++) {
            let d = f32(k[steps + 2u * t]);
            g += f32(k[steps + 2u * t + 1u]) * (bilinear32(other, f32(cx) - d * vx, f32(cy) - d * vy) * tint);
        }
    }
    let o = at(input, vec2(x, y));
    var out: vec4<f32>;
    for (var c = 0u; c < 3u; c++) {
        out[c] = f32(f64(o[c]) + k[0] * k[1u + c] * f64(g[c]));
    }
    out.w = f32(min(f64(o.w) + k[0] * f64(g.w), 1.0lf));
    textureStore(output, id.xy, out);
}

// B-107, layer_fx::rain: the seed in `base`, the drawing's corner at `ox`, `oy`. k: the fall's
// direction and its normal, the streak's reach across, half its length, the fall so far, the
// spacing, the density as a share, half the width, the opacity, the colour in linear light.
@compute @workgroup_size(16, 16)
fn rain(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a == 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    let fa = k[2] * x + k[3] * y;
    let fb = k[0] * x + k[1] * y - k[6];
    let r = k[4];
    let half = k[5];
    let sp = k[7];
    var q = 0.0lf;
    for (var ci = i32(floor((fa - r) / sp)); ci <= i32(floor((fa + r) / sp)); ci++) {
        for (var cj = i32(floor((fb - half - r) / sp)); cj <= i32(floor((fb + half + r) / sp)); cj++) {
            if (hashed(ci, cj, 0, 0u) + 1.0lf) / 2.0lf >= k[8] {
                continue;
            }
            let cx = sp * (f64(ci) + (hashed(ci, cj, 0, 1u) + 1.0lf) / 2.0lf);
            let cy = sp * (f64(cj) + (hashed(ci, cj, 0, 2u) + 1.0lf) / 2.0lf);
            let beta = 0.5lf + 0.25lf * (hashed(ci, cj, 0, 3u) + 1.0lf);
            let dx = fa - cx;
            let dy = max(abs(fb - cy) - half, 0.0lf);
            let delta = sqrt(dx * dx + dy * dy);
            q = max(q, clamp(k[9] - delta + 0.5lf, 0.0lf, 1.0lf) * beta);
        }
    }
    let op = q * k[10] / 100.0lf;
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let b = f64(p[c]) / a;
        out[c] = f32((b + op * (k[11u + c] - b)) * a);
    }
    textureStore(output, id.xy, out);
}

// line_blur's covering and ink of a pixel. B-157 (G6): in single precision, as the rest of the
// pass: the slopes, the line's way and the taps along it each round far below a level of 255.
fn cover_ink(p: vec4<f32>) -> vec2<f32> {
    return vec2(p.w, clamp(p.w - 0.2126 * p.x - 0.7152 * p.y - 0.0722 * p.z, 0.0, 1.0));
}

// line_blur's `sample`: the drawing at pixel position (x, y), its corners on the pixels.
fn tap(x: f32, y: f32) -> vec4<f32> {
    let x0 = floor(x);
    let y0 = floor(y);
    let fx = x - x0;
    let fy = y - y0;
    let i = i32(x0);
    let j = i32(y0);
    let p00 = at(input, vec2(i, j));
    let p10 = at(input, vec2(i + 1, j));
    let p01 = at(input, vec2(i, j + 1));
    let p11 = at(input, vec2(i + 1, j + 1));
    return (p00 * (1.0 - fx) + p10 * fx) * (1.0 - fy) + (p01 * (1.0 - fx) + p11 * fx) * fy;
}

// `tap`'s covering in double precision, for the rare tap near the threshold that ends a line.
fn tap_cover(x: f64, y: f64) -> f64 {
    let x0 = floor(x);
    let y0 = floor(y);
    let fx = x - x0;
    let fy = y - y0;
    let i = i32(x0);
    let j = i32(y0);
    let p00 = f64(at(input, vec2(i, j)).w);
    let p10 = f64(at(input, vec2(i + 1, j)).w);
    let p01 = f64(at(input, vec2(i, j + 1)).w);
    let p11 = f64(at(input, vec2(i + 1, j + 1)).w);
    return (p00 * (1.0lf - fx) + p10 * fx) * (1.0lf - fy) + (p01 * (1.0lf - fx) + p11 * fx) * fy;
}

// B-123, line_blur::line_blur, a pixel a thread, the output grown by `g`: the covering's and the
// ink's slopes smoothed round the pixel, the line's way from them, and the taps along it. k: the
// strength as a share, 1 for lines only, and the `count` taps' weights.
@compute @workgroup_size(16, 16)
fn lineblur(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let lx = i32(id.x) - F.g;
    let ly = i32(id.y) - F.g;
    let mine = at(input, vec2(lx, ly));
    if k[0] == 0.0lf {
        textureStore(output, id.xy, mine);
        return;
    }
    var weights = array<f32, 5>(1.0, 4.0, 6.0, 4.0, 1.0);
    var t = vec3(0.0);
    for (var j = 0; j < 5; j++) {
        for (var i = 0; i < 5; i++) {
            let x = lx + i - 2;
            let y = ly + j - 2;
            let l = cover_ink(at(input, vec2(x - 1, y)));
            let r = cover_ink(at(input, vec2(x + 1, y)));
            let u = cover_ink(at(input, vec2(x, y - 1)));
            let d = cover_ink(at(input, vec2(x, y + 1)));
            let gx = (r - l) / 2.0;
            let gy = (d - u) / 2.0;
            let raw = vec3(gx.x * gx.x + gx.y * gx.y, gx.x * gy.x + gx.y * gy.y, gy.x * gy.x + gy.y * gy.y);
            t += weights[i] * weights[j] / 256.0 * raw;
        }
    }
    let a = f64(t.x);
    let b = f64(t.y);
    let c = f64(t.z);
    let r = sqrt((a - c) * (a - c) + 4.0lf * b * b);
    if r == 0.0lf {
        textureStore(output, id.xy, mine);
        return;
    }
    var tx = -(c - a + r) / 2.0lf;
    var ty = b;
    if a >= c {
        tx = b;
        ty = -(a - c + r) / 2.0lf;
    }
    let norm = sqrt(tx * tx + ty * ty);
    tx = tx / norm;
    ty = ty / norm;
    let ux = f32(tx);
    let uy = f32(ty);
    var acc = mine;
    var total = 1.0;
    for (var side = 0; side < 2; side++) {
        let way = select(1.0, -1.0, side == 1);
        for (var q = 0u; q < F.count; q++) {
            let out = way * f32(q + 1u);
            let s = tap(f32(lx) + out * ux, f32(ly) + out * uy);
            if abs(s.w - 1.0 / 256.0) < 1e-4 {
                if tap_cover(f64(lx) + f64(out) * tx, f64(ly) + f64(out) * ty) < 1.0lf / 256.0lf {
                    break;
                }
            } else if s.w < 1.0 / 256.0 {
                break;
            }
            acc += f32(k[2u + q]) * s;
            total += f32(k[2u + q]);
        }
    }
    var amount = k[0] * r / (a + c);
    if k[1] != 0.0lf {
        amount *= f64(cover_ink(mine).y);
    }
    textureStore(output, id.xy, mine + (acc / total - mine) * f32(amount));
}

// B-123, layer_fx::kira_kira's light at (px, py) from the star at k[s]: its centre, size and beat.
// k: the arms' count at 6 and each arm's way and length as a share from 7.
fn star_light(s: u32, px: f64, py: f64) -> f64 {
    let r = k[s + 2u];
    let dx = px - k[s];
    let dy = py - k[s + 1u];
    let h = 0.5lf + r / 32.0lf;
    var best = 0.0lf;
    for (var m = 0u; m < u32(k[6]); m++) {
        let vx = k[7u + 3u * m];
        let vy = k[8u + 3u * m];
        let a = abs(dx * vx + dy * vy);
        let b = abs(dx * vy - dy * vx);
        let l = r * k[9u + 3u * m];
        if a < l && b < h {
            let q = 1.0lf - a / l;
            best = max(best, q * q * (1.0lf - b / h));
        }
    }
    let d = sqrt(dx * dx + dy * dy);
    if d < r / 4.0lf {
        let q = 1.0lf - d / (r / 4.0lf);
        best = max(best, q * q);
    }
    return k[s + 3u] * best;
}

// B-123, kira_kira's stars, a star a workgroup: each lights its patch of the output, grown by
// `g`, keeping the strongest light at each pixel of `lit`. k: the drawing's corner in the output
// across and down, and from 19 each of `count` stars.
@compute @workgroup_size(64)
fn stars(@builtin(workgroup_id) wg: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let n = wg.x + wg.y * 65535u;
    if n >= F.count {
        return;
    }
    let s = 19u + 4u * n;
    let w = textureDimensions(input).x + 2u * u32(F.g);
    let h = textureDimensions(input).y + 2u * u32(F.g);
    let e = k[s + 2u] + 0.5lf + k[s + 2u] / 32.0lf;
    let x0 = u32(max(floor(k[s] + k[0] - 0.5lf - e), 0.0lf));
    let x1 = u32(clamp(ceil(k[s] + k[0] - 0.5lf + e) + 1.0lf, 0.0lf, f64(w)));
    let y0 = u32(max(floor(k[s + 1u] + k[1] - 0.5lf - e), 0.0lf));
    let y1 = u32(clamp(ceil(k[s + 1u] + k[1] - 0.5lf + e) + 1.0lf, 0.0lf, f64(h)));
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    let bw = x1 - x0;
    for (var i = li; i < bw * (y1 - y0); i += 64u) {
        let x = x0 + i % bw;
        let y = y0 + i / bw;
        let py = f64(y) - k[1] + 0.5lf;
        if abs(py - k[s + 1u]) >= e {
            continue;
        }
        let v = star_light(s, f64(x) - k[0] + 0.5lf, py);
        if v > 0.0lf {
            atomicMax(&glints[y * w + x], bitcast<u32>(f32(v)));
        }
    }
}

// B-123, kira_kira's light laid on the drawing, a pixel a thread. k: the opacity as a share at 2
// and the colour in linear light from 3.
@compute @workgroup_size(16, 16)
fn kira(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let o = vec4<f64>(at(input, vec2(i32(id.x) - F.g, i32(id.y) - F.g)));
    let s = k[2] * f64(bitcast<f32>(atomicLoad(&glints[id.y * size.x + id.x])));
    textureStore(output, id.xy, vec4<f32>(vec4(o.xyz + s * vec3(k[3], k[4], k[5]), o.w + s * (1.0lf - o.w))));
}

// D-551, layer_fx::ball_cover: how much ball b covers the own point (x, y).
fn ball_cover(b: u32, x: f64, y: f64) -> f64 {
    let e = sqrt((x - k[b]) * (x - k[b]) + (y - k[b + 1u]) * (y - k[b + 1u]));
    return clamp(k[b + 2u] + 0.5lf - e, 0.0lf, 1.0lf) * min(2.0lf * k[b + 2u], 1.0lf);
}

// D-551, layer_fx::ball_shade: ball b lit at the own point (x, y).
fn ball_shade(b: u32, x: f64, y: f64) -> vec3<f64> {
    let dx = (x - k[b]) / k[b + 2u];
    let dy = (y - k[b + 1u]) / k[b + 2u];
    let q = dx * dx + dy * dy;
    var n = vec3(dx, dy, -sqrt(1.0lf - q));
    if q >= 1.0lf {
        n = vec3(dx / sqrt(q), dy / sqrt(q), 0.0lf);
    }
    let dif = max(n.x * k[0] + n.y * k[1] + n.z * k[2], 0.0lf);
    var sp = max(n.x * k[3] + n.y * k[4] + n.z * k[5], 0.0lf);
    for (var i = 0; i < 5; i++) {
        sp *= sp;
    }
    return vec3(k[b + 3u], k[b + 4u], k[b + 5u]) * (0.25lf + 0.75lf * dif) + 0.5lf * sp;
}

// D-551, cc_ball_action's balls laid, a ball a workgroup, as layer_fx::ball_numbers puts them in
// k (from 6, six each, farthest first), the drawing's corner at (F.ox, F.oy). Mode 0 keeps each
// pixel's highest numbered ball (1 up) in glints; mode 1, after it, the highest below that in
// glints from w h: its first and second balls.
@compute @workgroup_size(64)
fn balls(@builtin(workgroup_id) wg: vec3<u32>, @builtin(local_invocation_index) li: u32) {
    let n = wg.x + wg.y * 65535u;
    if n >= F.count {
        return;
    }
    let b = 6u + 6u * n;
    let size = textureDimensions(input);
    let o = vec2<f64>(f64(F.ox), f64(F.oy));
    let x0 = u32(clamp(floor(k[b] + o.x - k[b + 2u] - 1.5lf), 0.0lf, f64(size.x)));
    let x1 = u32(clamp(ceil(k[b] + o.x + k[b + 2u] + 1.5lf), 0.0lf, f64(size.x)));
    let y0 = u32(clamp(floor(k[b + 1u] + o.y - k[b + 2u] - 1.5lf), 0.0lf, f64(size.y)));
    let y1 = u32(clamp(ceil(k[b + 1u] + o.y + k[b + 2u] + 1.5lf), 0.0lf, f64(size.y)));
    if x0 >= x1 || y0 >= y1 {
        return;
    }
    let bw = x1 - x0;
    let key = n + 1u;
    for (var i = li; i < bw * (y1 - y0); i += 64u) {
        let x = x0 + i % bw;
        let y = y0 + i / bw;
        if ball_cover(b, f64(x) - o.x + 0.5lf, f64(y) - o.y + 0.5lf) > 0.0lf {
            let p = y * size.x + x;
            if F.mode == 0u {
                atomicMax(&glints[p], key);
            } else if key < atomicLoad(&glints[p]) {
                atomicMax(&glints[size.x * size.y + p], key);
            }
        }
    }
}

// D-551, layer_fx::ball_pixel, a pixel a thread, from its two balls in glints.
@compute @workgroup_size(16, 16)
fn ballshade(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = id.y * size.x + id.x;
    let first = atomicLoad(&glints[p]);
    if first == 0u {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let second = atomicLoad(&glints[size.x * size.y + p]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    let b1 = 6u * first;
    let c1 = ball_cover(b1, x, y);
    var out = vec4(c1 * ball_shade(b1, x, y), c1);
    if second != 0u {
        let b2 = 6u * second;
        let t = (1.0lf - c1) * ball_cover(b2, x, y);
        out = vec4(out.xyz + t * ball_shade(b2, x, y), c1 + t);
    }
    textureStore(output, id.xy, vec4<f32>(out));
}

// D-611, particles::levels: level j of the texture's atlas (other) as x, y, w, h: level 0, F.n
// by F.r, at the corner, each next half as big (rounded up) stacked down its right.
fn tex_level(j: u32) -> vec4<u32> {
    var l = vec4(0u, 0u, F.n, u32(F.r));
    for (var i = 0u; i < j; i++) {
        if i == 0u {
            l = vec4(F.n, 0u, l.z, l.w);
        } else {
            l.y += l.w;
        }
        l.z = (l.z + 1u) / 2u;
        l.w = (l.w + 1u) / 2u;
    }
    return l;
}

// D-611: the last level's number.
fn tex_top() -> u32 {
    var w = F.n;
    var h = u32(F.r);
    var t = 0u;
    while w > 1u || h > 1u {
        w = (w + 1u) / 2u;
        h = (h + 1u) / 2u;
        t++;
    }
    return t;
}

// D-611, particles::texel's held: document 21's bilinear on level l at (s w, u h), each tap
// held to the level's edge pixels.
fn tex_held(l: vec4<u32>, s: f64, u: f64) -> vec4<f64> {
    let fx = s * f64(l.z) - 0.5lf;
    let fy = u * f64(l.w) - 0.5lf;
    let bx = floor(fx);
    let by = floor(fy);
    let ux = fx - bx;
    let uy = fy - by;
    var out = vec4(0.0lf);
    for (var dy = 0; dy < 2; dy++) {
        for (var dx = 0; dx < 2; dx++) {
            let px = clamp(i32(bx) + dx, 0, i32(l.z) - 1);
            let py = clamp(i32(by) + dy, 0, i32(l.w) - 1);
            // Each part widened on its own: the whole read widened as one comes back wrong on
            // this card's driver (B-491).
            let t = textureLoad(other, vec2(i32(l.x) + px, i32(l.y) + py), 0);
            let q = vec4(f64(t.x), f64(t.y), f64(t.z), f64(t.w));
            var wx = 1.0lf - ux;
            if dx == 1 {
                wx = ux;
            }
            var wy = 1.0lf - uy;
            if dy == 1 {
                wy = uy;
            }
            out += q * (wx * wy);
        }
    }
    return out;
}

// D-611, particles::cover's textured kinds (5 square, 6 disc, 7 faded disc) from k[s]: turned,
// never lit; the texture read when F.flag is 1, at level log2(size / 2 R) mixed with the next.
fn textured_cover(s: u32, x: f64, y: f64) -> vec4<f64> {
    let kind = k[s];
    let r = k[s + 3u];
    let dx = x - k[s + 1u];
    let dy = y - k[s + 2u];
    let a = dx * k[s + 4u] + dy * k[s + 5u];
    let c = -dx * k[s + 5u] + dy * k[s + 4u];
    let e = sqrt(dx * dx + dy * dy);
    var m = 0.0lf;
    if kind == 5.0lf {
        m = clamp(r - abs(a) + 0.5lf, 0.0lf, 1.0lf) * clamp(r - abs(c) + 0.5lf, 0.0lf, 1.0lf);
    } else if kind == 6.0lf {
        m = clamp(r - e + 0.5lf, 0.0lf, 1.0lf);
    } else {
        let t = max(1.0lf - (e / r) * (e / r), 0.0lf);
        m = t * t;
    }
    if m <= 0.0lf {
        return vec4(0.0lf);
    }
    let aa = k[s + 6u] * m;
    let col = vec3(k[s + 7u], k[s + 8u], k[s + 9u]);
    if F.flag == 0u {
        return vec4(col * aa, aa);
    }
    let top = tex_top();
    let lam = clamp(f64(log2(f32(max(F.n, u32(F.r))) / f32(2.0lf * r))), 0.0lf, f64(top));
    let j = u32(floor(lam));
    let su = a / (2.0lf * r) + 0.5lf;
    let tu = c / (2.0lf * r) + 0.5lf;
    var q = tex_held(tex_level(j), su, tu);
    if j < top {
        q += (tex_held(tex_level(j + 1u), su, tu) - q) * (lam - f64(j));
    }
    let fd = k[s + 10u];
    return vec4(aa * (fd * q.xyz + (1.0lf - fd) * col * q.w), aa * q.w);
}

// The numbers a sprite takes in every sprite list, `particles::STRIDE` (pinned by the test
// `the_card_reads_sprites_by_the_processors_stride`).
const SPRITE_STRIDE: u32 = 11u;

// D-612, particles::star_cover: the star (kind 8) from k[s], never lit: 1/2 + e inside by the
// even-odd rule, 1/2 - e outside, e the distance to the nearest of its ten edges.
fn star_cover(s: u32, x: f64, y: f64) -> vec4<f64> {
    var sx = array<f64, 10>(0.0lf, 0.23511410091698925lf, 0.9510565162951535lf, 0.3804226065180615lf,
        0.5877852522924732lf, 4.898587196589413e-17lf, -0.587785252292473lf, -0.3804226065180614lf,
        -0.9510565162951536lf, -0.23511410091698937lf);
    var sy = array<f64, 10>(-1.0lf, -0.323606797749979lf, -0.30901699437494745lf, 0.12360679774997894lf,
        0.8090169943749473lf, 0.4lf, 0.8090169943749475lf, 0.12360679774997903lf,
        -0.30901699437494723lf, -0.32360679774997897lf);
    let r = k[s + 3u];
    let dx = x - k[s + 1u];
    let dy = y - k[s + 2u];
    var e = 1.0e300lf;
    var inside = false;
    for (var j = 0u; j < 10u; j++) {
        let ax = sx[j] * r;
        let ay = sy[j] * r;
        let bx = sx[(j + 1u) % 10u] * r;
        let by = sy[(j + 1u) % 10u] * r;
        let ex = bx - ax;
        let ey = by - ay;
        let t = clamp(((dx - ax) * ex + (dy - ay) * ey) / (ex * ex + ey * ey), 0.0lf, 1.0lf);
        let qx = dx - ax - t * ex;
        let qy = dy - ay - t * ey;
        e = min(e, sqrt(qx * qx + qy * qy));
        if (ay > dy) != (by > dy) && dx < ax + (bx - ax) * (dy - ay) / (by - ay) {
            inside = !inside;
        }
    }
    var cv = clamp(0.5lf - e, 0.0lf, 1.0lf);
    if inside {
        cv = clamp(0.5lf + e, 0.0lf, 1.0lf);
    }
    if cv <= 0.0lf {
        return vec4(0.0lf);
    }
    let a = k[s + 6u] * cv;
    return vec4(k[s + 7u] * a, k[s + 8u] * a, k[s + 9u] * a, a);
}

// D-617, particles::cover's Particle Emitter spheres from k[s], never lit: the soft sphere (9),
// feathered over k[s + 4], and the glowing sphere (10), a core of half the radius feathered over
// k[s + 4] and a glow out to the radius.
fn soft_cover(s: u32, x: f64, y: f64) -> vec4<f64> {
    let r = k[s + 3u];
    let w = k[s + 4u];
    let dx = x - k[s + 1u];
    let dy = y - k[s + 2u];
    let e = sqrt(dx * dx + dy * dy);
    var cv = clamp((r - e) / w + 0.5lf, 0.0lf, 1.0lf);
    if k[s] == 10.0lf {
        let core = clamp((r / 2.0lf - e) / w + 0.5lf, 0.0lf, 1.0lf);
        let g = max(1.0lf - e / r, 0.0lf);
        cv = core + (1.0lf - core) * g * g / 2.0lf;
    }
    if cv <= 0.0lf {
        return vec4(0.0lf);
    }
    let a = k[s + 6u] * cv;
    return vec4(k[s + 7u] * a, k[s + 8u] * a, k[s + 9u] * a, a);
}

// D-619, render::sample_bilinear on the part of the atlas (other) at (l.x, l.y), l.z by l.w, at
// the part's own point (x, y): taps outside the part count nothing.
fn part_read(l: vec4<u32>, x: f64, y: f64) -> vec4<f64> {
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    let bx = floor(fx);
    let by = floor(fy);
    let ux = fx - bx;
    let uy = fy - by;
    var out = vec4(0.0lf);
    for (var dy = 0; dy < 2; dy++) {
        for (var dx = 0; dx < 2; dx++) {
            let px = i32(bx) + dx;
            let py = i32(by) + dy;
            if px < 0 || py < 0 || px >= i32(l.z) || py >= i32(l.w) {
                continue;
            }
            // Each part widened on its own (B-491).
            let t = textureLoad(other, vec2(i32(l.x) + px, i32(l.y) + py), 0);
            let q = vec4(f64(t.x), f64(t.y), f64(t.z), f64(t.w));
            var wx = 1.0lf - ux;
            if dx == 1 {
                wx = ux;
            }
            var wy = 1.0lf - uy;
            if dy == 1 {
                wy = uy;
            }
            out += q * (wx * wy);
        }
    }
    return out;
}

// D-619, playground::particle_playground's looks from k[s], premultiplied: a dot (11), a piece of
// the Explode layer (12, at the atlas's corner, F.n by F.r) and a copy of the Layer Map (13, right
// of it, F.base), each turned by its cosine and sine. D-713: a dot of two radii is an ellipse
// turned by its turn (its cosine and sine in single precision); a piece's and a copy's scale
// along each of its own axes; a piece's origin worked out from its number.
fn playground_cover(s: u32, x: f64, y: f64) -> vec4<f64> {
    let dx = x - k[s + 1u];
    let dy = y - k[s + 2u];
    if k[s] == 11.0lf {
        let rx = k[s + 4u];
        let ry = k[s + 5u];
        var e = rx - sqrt(dx * dx + dy * dy);
        if rx != ry {
            let o = f32(k[s + 6u]);
            let c = f64(cos(o));
            let n = f64(sin(o));
            let ex = (dx * c + dy * n) / rx;
            let ey = (-dx * n + dy * c) / ry;
            e = min(rx, ry) * (1.0lf - sqrt(ex * ex + ey * ey));
        }
        let cv = clamp(e + 0.5lf, 0.0lf, 1.0lf);
        return vec4(k[s + 7u], k[s + 8u], k[s + 9u], k[s + 10u]) * cv;
    }
    let cs = k[s + 4u];
    let sn = k[s + 5u];
    let lx = dx * cs + dy * sn;
    let ly = -dx * sn + dy * cs;
    if k[s] == 12.0lf {
        let side = k[s + 7u];
        let sx = k[s + 9u];
        let sy = k[s + 10u];
        let cv = clamp(side * sx / 2.0lf - abs(lx) + 0.5lf, 0.0lf, 1.0lf) * clamp(side * sy / 2.0lf - abs(ly) + 0.5lf, 0.0lf, 1.0lf);
        if cv <= 0.0lf {
            return vec4(0.0lf);
        }
        let hold = side / 2.0lf - 0.5lf;
        let across = ceil(f64(F.n) / side);
        let j = k[s + 8u];
        let row = floor(j / across);
        let ox = (j - row * across + 0.5lf) * side;
        let oy = (row + 0.5lf) * side;
        let t = part_read(vec4(0u, 0u, F.n, u32(F.r)), ox + clamp(lx / sx, -hold, hold), oy + clamp(ly / sy, -hold, hold));
        return t * (cv * k[s + 6u]);
    }
    let t = part_read(vec4(F.n, 0u, F.base.x, F.base.y), k[s + 8u] + lx / k[s + 7u], k[s + 9u] + ly / k[s + 10u]);
    if t.w <= 0.0lf {
        return vec4(0.0lf);
    }
    return t * k[s + 6u];
}

// D-610, particles::cover: sprite n of k (from 7, eleven each) at the own point (x, y),
// premultiplied; nothing where it does not reach.
fn sprite_cover(n: u32, x: f64, y: f64) -> vec4<f64> {
    let s = 7u + SPRITE_STRIDE * n;
    let kind = k[s];
    if kind >= 11.0lf {
        return playground_cover(s, x, y);
    }
    if kind == 8.0lf {
        return star_cover(s, x, y);
    }
    if kind == 9.0lf || kind == 10.0lf {
        return soft_cover(s, x, y);
    }
    if kind >= 5.0lf {
        return textured_cover(s, x, y);
    }
    var cv = 0.0lf;
    var shade = 1.0lf;
    if kind == 0.0lf {
        let ax = k[s + 1u];
        let ay = k[s + 2u];
        let dx = k[s + 3u] - ax;
        let dy = k[s + 4u] - ay;
        let ll = dx * dx + dy * dy;
        var t = 0.0lf;
        if ll > 0.0lf {
            t = clamp(((x - ax) * dx + (y - ay) * dy) / ll, 0.0lf, 1.0lf);
        }
        let ex = x - ax - t * dx;
        let ey = y - ay - t * dy;
        cv = clamp(k[s + 5u] / 2.0lf - sqrt(ex * ex + ey * ey) + 0.5lf, 0.0lf, 1.0lf);
    } else {
        let r = k[s + 3u];
        let ex = x - k[s + 1u];
        let ey = y - k[s + 2u];
        let e = sqrt(ex * ex + ey * ey);
        if kind == 1.0lf {
            let t = max(1.0lf - (e / r) * (e / r), 0.0lf);
            cv = t * t;
        } else {
            cv = clamp(r - e + 0.5lf, 0.0lf, 1.0lf);
            if kind == 4.0lf {
                let m = min(e / r, 1.0lf);
                cv *= 0.25lf + 0.75lf * (m * m) * (m * m);
            }
        }
        var v = k[0];
        if kind == 2.0lf || kind == 4.0lf {
            v = 1.0lf;
        }
        if v > 0.0lf {
            let dx = ex / r;
            let dy = ey / r;
            let q = dx * dx + dy * dy;
            var nv = vec3(dx, dy, -sqrt(max(1.0lf - q, 0.0lf)));
            if q >= 1.0lf {
                nv = vec3(dx / sqrt(q), dy / sqrt(q), 0.0lf);
            }
            shade = 1.0lf - v + v * (0.2lf + 0.8lf * max(nv.x * k[1] + nv.y * k[2] + nv.z * k[3], 0.0lf));
        }
    }
    if cv <= 0.0lf {
        return vec4(0.0lf);
    }
    let a = k[s + 6u] * cv;
    return vec4(k[s + 7u] * shade * a, k[s + 8u] * shade * a, k[s + 9u] * shade * a, a);
}

// D-610, particles::particle_space, a pixel a thread: the sprites of its 16 by 16 tile (after
// the sprites, each tile's start in the list and the end, then the list) laid farthest first by
// the transfer mode, over the drawing with Composite With Original on or else over nothing.
@compute @workgroup_size(16, 16)
fn particles(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var d = vec4(0.0lf);
    if k[5] > 0.5lf {
        d = vec4<f64>(at(input, vec2<i32>(id.xy)));
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + SPRITE_STRIDE * u32(k[6]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
        let s = sprite_cover(u32(k[o + nt + 1u + j]), x, y);
        if s.w <= 0.0lf {
            continue;
        }
        if k[4] == 0.0lf {
            d = s + (1.0lf - s.w) * d;
        } else if k[4] == 1.0lf {
            d = s + d - s * d;
        } else {
            d = vec4(s.xyz + d.xyz, min(s.w + d.w, 1.0lf));
        }
    }
    textureStore(output, id.xy, vec4<f32>(d));
}

// D-671, simulate::bubbles's bilinear read of the drawing (k[2] by k[3], its corner at the
// origin) at the own point (x, y), the last row and column repeated.
fn bubble_read(x: f64, y: f64) -> vec4<f64> {
    let fx = clamp(x - 0.5lf, 0.0lf, k[2] - 1.0lf);
    let fy = clamp(y - 0.5lf, 0.0lf, k[3] - 1.0lf);
    let x0 = i32(fx);
    let y0 = i32(fy);
    let x1 = min(x0 + 1, i32(k[2]) - 1);
    let y1 = min(y0 + 1, i32(k[3]) - 1);
    let tx = fx - f64(x0);
    let ty = fy - f64(y0);
    let o = vec2(F.ox, F.oy);
    let a = vec4<f64>(at(input, vec2(x0, y0) + o));
    let b = vec4<f64>(at(input, vec2(x1, y0) + o));
    let c = vec4<f64>(at(input, vec2(x0, y1) + o));
    let d = vec4<f64>(at(input, vec2(x1, y1) + o));
    return (a * (1.0lf - tx) + b * tx) * (1.0lf - ty) + (c * (1.0lf - tx) + d * tx) * ty;
}

// D-671, simulate::bubbles, a pixel a thread: the bubbles of its 16 by 16 tile (k as the
// particles pass has it: k[0] the reflection's sign, k[1] the shading, k[2] and k[3] the
// drawing's size, k[6] the count, the bubbles from 7, SPRITE_STRIDE each, then each tile's start and the
// lists), each read through itself, shaded, laid in order onto clear.
@compute @workgroup_size(16, 16)
fn bubbles(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + SPRITE_STRIDE * u32(k[6]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    var d = vec4(0.0lf);
    for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
        let s = 7u + SPRITE_STRIDE * u32(k[o + nt + 1u + j]);
        let bx = k[s + 1u];
        let by = k[s + 2u];
        let rho = k[s + 3u];
        let dx = x - bx;
        let dy = y - by;
        let dd = sqrt(dx * dx + dy * dy);
        let cover = clamp(rho - dd + 0.5lf, 0.0lf, 1.0lf);
        if cover == 0.0lf {
            continue;
        }
        var e = vec2(dx, dy) / rho;
        if dd > rho {
            e = vec2(dx, dy) / dd;
        }
        let t2 = min(e.x * e.x + e.y * e.y, 1.0lf);
        let r = 2.0lf * rho / (1.0lf + sqrt(1.0lf - t2));
        var c = bubble_read(bx + k[0] * r * e.x, by + k[0] * r * e.y);
        if k[1] == 1.0lf {
            c = c * (1.0lf - t2) + vec4(t2);
        } else if k[1] == 2.0lf {
            c = vec4(c.xyz * (1.0lf - t2), c.w * (1.0lf - t2) + t2);
        } else if k[1] == 3.0lf {
            c = c * t2;
        } else if k[1] == 4.0lf {
            c = c * (1.0lf - t2);
        }
        d = c * cover + d * (1.0lf - c.w * cover);
    }
    textureStore(output, id.xy, vec4<f32>(d));
}

// D-672, simulate::drizzle, a pixel a thread: the drops of its 16 by 16 tile (k: k[0] where the
// light's numbers start, k[6] the count, the drops from 7, SPRITE_STRIDE each, [3, x, y, R, A, ..], then
// each tile's start and the lists, then simulate::drizzle_light's fifteen numbers), the water's
// slope summed over them, the layer read through it, clamped to the buffer, and lit by
// Blobbylize's rule. The highlight's power is single precision, as Glue Gun's.
@compute @workgroup_size(16, 16)
fn drizzle(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + SPRITE_STRIDE * u32(k[6]);
    let g = u32(k[0]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    let tau = 2.0lf * 3.141592653589793lf * k[g];
    var zx = 0.0lf;
    var zy = 0.0lf;
    for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
        let s = 7u + SPRITE_STRIDE * u32(k[o + nt + 1u + j]);
        let dx = x - k[s + 1u];
        let dy = y - k[s + 2u];
        let r = k[s + 3u];
        let d2 = dx * dx + dy * dy;
        if d2 >= r * r {
            continue;
        }
        let q = sqrt(d2) / r;
        let one = 1.0lf - q * q;
        let ph = tau * q;
        var sq = tau * tau;
        if q != 0.0lf {
            sq = tau * sin64(ph) / q;
        }
        let gg = k[s + 4u] * (-4.0lf * one * cos64(ph) - one * one * sq);
        zx += gg * dx / (r * r);
        zy += gg * dy / (r * r);
    }
    let fx = clamp(x - k[g + 1u] * zx + f64(F.ox) - 0.5lf, 0.0lf, f64(size.x) - 1.0lf);
    let fy = clamp(y - k[g + 1u] * zy + f64(F.oy) - 0.5lf, 0.0lf, f64(size.y) - 1.0lf);
    let x0 = i32(fx);
    let y0 = i32(fy);
    let x1 = min(x0 + 1, i32(size.x) - 1);
    let y1 = min(y0 + 1, i32(size.y) - 1);
    let tx = fx - f64(x0);
    let ty = fy - f64(y0);
    let c = (vec4<f64>(at(input, vec2(x0, y0))) * (1.0lf - tx) + vec4<f64>(at(input, vec2(x1, y0))) * tx) * (1.0lf - ty)
        + (vec4<f64>(at(input, vec2(x0, y1))) * (1.0lf - tx) + vec4<f64>(at(input, vec2(x1, y1))) * tx) * ty;
    var col = vec3(0.0lf);
    if c.w > 0.0lf {
        col = c.xyz / c.w;
    }
    let nv = vec3(-zx, -zy, 1.0lf);
    let n = nv / sqrt(nv.x * nv.x + nv.y * nv.y + nv.z * nv.z);
    var lv = vec3(k[g + 12u], k[g + 13u], k[g + 14u]);
    if k[g + 11u] == 1.0lf {
        lv = vec3(k[g + 12u] - x, k[g + 13u] - y, k[g + 14u]);
    }
    let ll = sqrt(lv.x * lv.x + lv.y * lv.y + lv.z * lv.z);
    var l = vec3(0.0lf);
    if ll > 0.0lf {
        l = lv / ll;
    }
    let nl = n.x * l.x + n.y * l.y + n.z * l.z;
    let rz = 2.0lf * nl * n.z - l.z;
    var shine = 0.0lf;
    if nl > 0.0lf && rz > 0.0lf {
        shine = f64(pow(f32(rz), f32(1.0lf / k[g + 10u])));
    }
    var out = vec4(0.0lf, 0.0lf, 0.0lf, c.w);
    for (var ch = 0u; ch < 3u; ch++) {
        let lc = k[g + 2u + ch];
        let lit = col[ch] * (k[g + 6u] + k[g + 7u] * k[g + 5u] * lc * max(nl, 0.0lf)) + k[g + 8u] * k[g + 5u] * (lc + (col[ch] - lc) * k[g + 9u]) * shine;
        out[ch] = lit * c.w;
    }
    textureStore(output, id.xy, vec4<f32>(out));
}

// D-673, D-674, the layer at a strand's root or a drop's birthplace, at (x, y) in the buffer's
// own pixels: the bilinear read clamped to the buffer, premultiplied.
fn hair_root(x: f64, y: f64) -> vec4<f64> {
    let size = textureDimensions(input);
    let fx = clamp(x - 0.5lf, 0.0lf, f64(size.x) - 1.0lf);
    let fy = clamp(y - 0.5lf, 0.0lf, f64(size.y) - 1.0lf);
    let x0 = i32(fx);
    let y0 = i32(fy);
    let x1 = min(x0 + 1, i32(size.x) - 1);
    let y1 = min(y0 + 1, i32(size.y) - 1);
    let tx = fx - f64(x0);
    let ty = fy - f64(y0);
    return (vec4<f64>(at(input, vec2(x0, y0))) * (1.0lf - tx) + vec4<f64>(at(input, vec2(x1, y0))) * tx) * (1.0lf - ty)
        + (vec4<f64>(at(input, vec2(x0, y1))) * (1.0lf - tx) + vec4<f64>(at(input, vec2(x1, y1))) * tx) * ty;
}

// D-673, simulate::hair, a pixel a thread: the strands of its 16 by 16 tile in order (k: k[0]
// where the shared numbers start, k[6] the count, the strands from 7, SPRITE_STRIDE each, [3, x, y, reach,
// d_x, d_y, d_z, L, b, u_2], then each tile's start and the lists, then simulate::hair_light's
// fifteen numbers), each strand's coverage over its eight pieces, its root read for whether it
// grows and its colour, lit as hair and laid over. The shine's power is single precision, as
// Glue Gun's. Audit P2 (after B-157): each strand's pieces and its light in single precision,
// worked from its root so the numbers stay small; its root's read and the laying over stay double.
@compute @workgroup_size(16, 16)
fn hair(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + SPRITE_STRIDE * u32(k[6]);
    let g = u32(k[0]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    let th = f32(k[g + 5u]);
    let l = vec3(f32(k[g + 7u]), f32(k[g + 8u]), f32(k[g + 9u]));
    var res = vec4<f64>(at(input, vec2<i32>(id.xy)));
    for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
        let s = 7u + SPRITE_STRIDE * u32(k[o + nt + 1u + j]);
        let rx = k[s + 1u];
        let ry = k[s + 2u];
        let qx = f32(x - rx);
        let qy = f32(y - ry);
        let d = vec3(f32(k[s + 4u]), f32(k[s + 5u]), f32(k[s + 6u]));
        let sl = f32(k[s + 7u]);
        let b = f32(k[s + 8u]);
        var best = 0.0;
        var sat = 0.0;
        var a = vec2(0.0, 0.0);
        for (var n = 0u; n < 8u; n++) {
            let q = f32(n + 1u) / 8.0;
            let e2 = vec2(sl * d.x * q, sl * (d.y * q + b * q * q));
            let ex = e2.x - a.x;
            let ey = e2.y - a.y;
            let ee = ex * ex + ey * ey;
            var sp = 0.0;
            if ee != 0.0 {
                sp = clamp(((qx - a.x) * ex + (qy - a.y) * ey) / ee, 0.0, 1.0);
            }
            let px = qx - (a.x + sp * ex);
            let py = qy - (a.y + sp * ey);
            let at_s = (f32(n) + sp) / 8.0;
            let cov = clamp(th / 2.0 * (1.0 - 0.75 * at_s) - sqrt(px * px + py * py) + 0.5, 0.0, 1.0);
            if cov > best {
                best = cov;
                sat = at_s;
            }
            a = e2;
        }
        if best == 0.0 {
            continue;
        }
        let c = hair_root(rx + f64(F.ox), ry + f64(F.oy));
        if !(k[s + 9u] < c.w) {
            continue;
        }
        let dv0 = vec3(d.x, d.y + 2.0 * b * sat, d.z);
        let dv = dv0 / sqrt(dv0.x * dv0.x + dv0.y * dv0.y + dv0.z * dv0.z);
        let al = dv.x * l.x + dv.y * l.y + dv.z * l.z;
        let kd = sqrt(max(1.0 - al * al, 0.0));
        let br = kd * sqrt(max(1.0 - dv.z * dv.z, 0.0)) - al * dv.z;
        var ks = 0.0lf;
        if br > 0.0 {
            ks = f64(pow(br, f32(1.0lf / k[g + 13u])));
        }
        let cover = f64(best) * k[g + 4u];
        for (var ch = 0u; ch < 3u; ch++) {
            let col = k[g + ch] * (1.0lf - k[g + 3u]) + c[ch] / c.w * k[g + 3u];
            let lit = col * (k[g + 10u] + k[g + 11u] * k[g + 6u] * f64(kd)) + k[g + 12u] * k[g + 6u] * (1.0lf + (col - 1.0lf) * k[g + 14u]) * ks;
            res[ch] = lit * cover + res[ch] * (1.0lf - cover);
        }
        res.w = cover + res.w * (1.0lf - cover);
    }
    textureStore(output, id.xy, vec4<f32>(res));
}

// D-674, simulate::mercury, a pixel a thread: the drops of its 16 by 16 tile in order (k: k[0]
// where the shared numbers start, k[6] the count, the drops from 7, SPRITE_STRIDE each, [3, x, y, rho, r,
// x_0, y_0, u_2, ..], then each tile's start and the lists, then simulate::mercury_light's
// fifteen numbers), each live drop's birthplace read for whether it lives and its colour, the
// metaball field, its slope and the colour summed, the surface's edge a pixel soft and lit by
// Drizzle's light. Where no drop covers, clear. The highlight's power is single precision, as
// Glue Gun's.
@compute @workgroup_size(16, 16)
fn mercury(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + SPRITE_STRIDE * u32(k[6]);
    let g = u32(k[0]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    var field = 0.0lf;
    var fx = 0.0lf;
    var fy = 0.0lf;
    var wt = 0.0lf;
    var cs = vec3(0.0lf);
    for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
        let s = 7u + SPRITE_STRIDE * u32(k[o + nt + 1u + j]);
        let dx = x - k[s + 1u];
        let dy = y - k[s + 2u];
        let rho = k[s + 3u];
        let q2 = (dx * dx + dy * dy) / (rho * rho);
        if q2 >= 1.0lf {
            continue;
        }
        let c = hair_root(k[s + 5u] + f64(F.ox), k[s + 6u] + f64(F.oy));
        if !(k[s + 7u] < c.w) {
            continue;
        }
        let one = 1.0lf - q2;
        let ff = one * one;
        field += ff;
        fx += -4.0lf * one * dx / (rho * rho);
        fy += -4.0lf * one * dy / (rho * rho);
        wt += ff * rho;
        cs += ff * (c.xyz / c.w);
    }
    let edge = k[g + 1u];
    let gl = sqrt(fx * fx + fy * fy);
    var cov = 0.0lf;
    if field > 0.0lf {
        if gl > 0.0lf {
            cov = clamp((field - edge) / gl + 0.5lf, 0.0lf, 1.0lf);
        } else if field > edge {
            cov = 1.0lf;
        }
    }
    if cov == 0.0lf {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let col = cs / field;
    let cc = wt / field;
    let nv = vec3(-cc * fx, -cc * fy, 2.0lf * sqrt(max(field - edge, 0.0lf)));
    let nlen = sqrt(nv.x * nv.x + nv.y * nv.y + nv.z * nv.z);
    var n = vec3(0.0lf, 0.0lf, 1.0lf);
    if nlen > 0.0lf {
        n = nv / nlen;
    }
    var lv = vec3(k[g + 12u], k[g + 13u], k[g + 14u]);
    if k[g + 11u] == 1.0lf {
        lv = vec3(k[g + 12u] - x, k[g + 13u] - y, k[g + 14u]);
    }
    let ll = sqrt(lv.x * lv.x + lv.y * lv.y + lv.z * lv.z);
    var l = vec3(0.0lf);
    if ll > 0.0lf {
        l = lv / ll;
    }
    let nl = n.x * l.x + n.y * l.y + n.z * l.z;
    let rz = 2.0lf * nl * n.z - l.z;
    var shine = 0.0lf;
    if nl > 0.0lf && rz > 0.0lf {
        shine = f64(pow(f32(rz), f32(1.0lf / k[g + 10u])));
    }
    var out = vec4(0.0lf, 0.0lf, 0.0lf, cov);
    for (var ch = 0u; ch < 3u; ch++) {
        let lc = k[g + 2u + ch];
        let lit = col[ch] * (k[g + 6u] + k[g + 7u] * k[g + 5u] * lc * max(nl, 0.0lf)) + k[g + 8u] * k[g + 5u] * (lc + (col[ch] - lc) * k[g + 9u]) * shine;
        out[ch] = lit * cov;
    }
    textureStore(output, id.xy, vec4<f32>(out));
}

// D-675, simulate::polygon_shards, a pixel a thread: the shards of its 16 by 16 tile in the order
// they are drawn (k: k[0] where the shared numbers start, k[2] and k[3] the drawing's size, k[6]
// the count, the shards from 7, SPRITE_STRIDE each, [3, x, y, reach, part, r, g, b, a, 0], then each tile's
// start and the lists, then the seven shared numbers, the eye (a_x, a_y, A_l), the drawing's
// size, the grid spacing and 1 when textured, then sixteen numbers a shard, P, M by rows, c and
// the cell's corner). The ray from the eye through the pixel meets the shard's plane, the point
// there of the drawing found, its distance inside the shard and the drawing a pixel soft, and the
// shard's colour or the drawing there (Bubbles' read) laid over what is below. Clear where no
// shard covers.
@compute @workgroup_size(16, 16)
fn shards(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + SPRITE_STRIDE * u32(k[6]);
    let g = u32(k[0]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    let ax = k[g];
    let ay = k[g + 1u];
    let al = k[g + 2u];
    let s = k[g + 5u];
    let rx = x - ax;
    let ry = y - ay;
    let rl = sqrt(rx * rx + ry * ry + al * al);
    var acc = vec4(0.0lf);
    for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
        let n = u32(k[o + nt + 1u + j]);
        let sp = 7u + SPRITE_STRIDE * n;
        let b = g + 7u + n * 16u;
        let den = k[b + 5u] * rx + k[b + 8u] * ry + k[b + 11u] * al;
        if !(abs(den) > 1e-9lf * rl) {
            continue;
        }
        let lam = (k[b + 5u] * (k[b] - ax) + k[b + 8u] * (k[b + 1u] - ay) + k[b + 11u] * (k[b + 2u] + al)) / den;
        if !(lam > 0.0lf) {
            continue;
        }
        let qz = -al + lam * al;
        let dx = ax + lam * rx - k[b];
        let dy = ay + lam * ry - k[b + 1u];
        let dz = qz - k[b + 2u];
        let x0 = k[b + 12u] + (k[b + 3u] * dx + k[b + 6u] * dy + k[b + 9u] * dz);
        let y0 = k[b + 13u] + (k[b + 4u] * dx + k[b + 7u] * dy + k[b + 10u] * dz);
        let a = x0 - k[b + 14u];
        let bb = y0 - k[b + 15u];
        var inside = min(min(a, bb), min(s - a, s - bb));
        if k[sp + 4u] == 0.0lf {
            inside = min(min(bb, s - a), (a - bb) / 1.4142135623730951lf);
        } else if k[sp + 4u] == 1.0lf {
            inside = min(min(a, s - bb), (bb - a) / 1.4142135623730951lf);
        }
        inside = min(min(inside, min(x0, y0)), min(k[g + 3u] - x0, k[g + 4u] - y0));
        let cov = clamp(inside * al / (al + qz) + 0.5lf, 0.0lf, 1.0lf);
        if cov == 0.0lf {
            continue;
        }
        var col = vec4(k[sp + 5u], k[sp + 6u], k[sp + 7u], k[sp + 8u]);
        if k[g + 6u] == 1.0lf {
            col = bubble_read(x0, y0);
        }
        acc = col * cov + acc * (1.0lf - col.w * cov);
    }
    textureStore(output, id.xy, vec4<f32>(acc));
}

// D-678, simulate::foam, a pixel a thread (k: k[0] where the shared numbers start, k[2] and k[3]
// the drawing's size, k[6] the count, the bubbles from 7, SPRITE_STRIDE each, [3, x, y, reach, R, cos
// theta, sin theta, cos 2 alpha, sin 2 alpha, 0], in the order laid down, then each tile's start
// and the lists, then simulate::foam_list's twelve shared numbers): in the Draft view the
// producer's red ring, then each bubble's blue ring; Rendered, each bubble's texture or the
// drawing through it (Bubbles' read), with the reflection, screened or laid over. Clear where
// nothing covers.
@compute @workgroup_size(16, 16)
fn foam(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + SPRITE_STRIDE * u32(k[6]);
    let g = u32(k[0]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    let w = k[2];
    let h = k[3];
    let draft = k[g] == 0.0lf;
    var acc = vec4(0.0lf);
    if draft {
        let px = x - k[g + 6u];
        let py = y - k[g + 7u];
        let lx = px * k[g + 10u] + py * k[g + 11u];
        let ly = -px * k[g + 11u] + py * k[g + 10u];
        let a2 = lx / k[g + 8u];
        let b2 = ly / k[g + 9u];
        let kk = sqrt(a2 * a2 + b2 * b2);
        let a3 = lx / (k[g + 8u] * k[g + 8u]);
        let b3 = ly / (k[g + 9u] * k[g + 9u]);
        let g2 = sqrt(a3 * a3 + b3 * b3);
        if g2 > 0.0lf {
            let cov = clamp(1.0lf - abs((kk * kk - kk) / g2), 0.0lf, 1.0lf);
            acc = vec4(1.0lf, 0.15lf, 0.1lf, 1.0lf) * cov + acc * (1.0lf - cov);
        }
    }
    let tex = k[g + 2u];
    let fr = k[g + 4u];
    let cv = k[g + 5u];
    for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
        let s = 7u + SPRITE_STRIDE * u32(k[o + nt + 1u + j]);
        let dx = x - k[s + 1u];
        let dy = y - k[s + 2u];
        let e2 = dx * dx + dy * dy;
        let e = sqrt(e2);
        var rw = k[s + 4u];
        if e2 > 0.0lf {
            rw = k[s + 4u] * (1.0lf + k[g + 3u] * (k[s + 7u] * (dx * dx - dy * dy) + k[s + 8u] * 2.0lf * dx * dy) / e2);
        }
        if draft {
            let cov = clamp(1.0lf - abs(e - rw), 0.0lf, 1.0lf);
            if cov > 0.0lf {
                acc = vec4(0.2lf, 0.5lf, 1.0lf, 1.0lf) * cov + acc * (1.0lf - cov);
            }
            continue;
        }
        let cov = clamp(rw - e + 0.5lf, 0.0lf, 1.0lf);
        if cov == 0.0lf || !(rw > 0.0lf) {
            continue;
        }
        let q0 = e / rw;
        let cut = max(q0, 1.0lf);
        let ux = (dx * k[s + 5u] + dy * k[s + 6u]) / rw / cut;
        let uy = (-dx * k[s + 6u] + dy * k[s + 5u]) / rw / cut;
        let q = min(q0, 1.0lf);
        var col: vec4<f64>;
        if tex == 4.0lf {
            col = bubble_read((ux + 1.0lf) * w / 2.0lf, (uy + 1.0lf) * h / 2.0lf);
        } else {
            let hx = ux + 0.35lf;
            let hy = uy + 0.35lf;
            let h0 = max(0.0lf, 1.0lf - (hx * hx + hy * hy) / 0.0625lf);
            let hl = h0 * h0;
            var a = 0.3lf + 0.45lf * q * q * q;
            var c = vec3(0.72lf, 0.68lf, 0.58lf);
            if tex == 0.0lf {
                a = 0.12lf + 0.68lf * q * q * q * q;
                c = vec3(0.85lf + 0.15lf * q * q, 0.92lf, 1.0lf - 0.15lf * q * q);
            } else if tex == 1.0lf {
                a = 0.45lf + 0.45lf * q * q;
                c = vec3(0.95lf, 0.95lf, 0.93lf);
            } else if tex == 2.0lf {
                let lit = 0.5lf + 0.5lf * max(0.0lf, -0.4lf * ux - 0.4lf * uy + 0.824lf * sqrt(max(0.0lf, 1.0lf - q * q)));
                a = 0.95lf;
                c = vec3(1.0lf, 0.45lf, 0.7lf) * lit;
            }
            a = a + (1.0lf - a) * 0.85lf * hl;
            c = c + (vec3(1.0lf) - c) * hl;
            col = vec4(c * a, a);
        }
        if fr > 0.0lf {
            let ex = (1.0lf - cv) * (k[s + 1u] + 2.0lf * dx / cut) + cv * ((ux + 1.0lf) * w / 2.0lf);
            let ey = (1.0lf - cv) * (k[s + 2u] + 2.0lf * dy / cut) + cv * ((uy + 1.0lf) * h / 2.0lf);
            let r = bubble_read(ex, ey);
            col = vec4(col.xyz * (1.0lf - fr) + r.xyz * col.w * fr, col.w);
        }
        if k[g + 1u] == 0.0lf {
            acc = acc + col * cov - acc * col * cov;
        } else {
            acc = col * cov + acc * (1.0lf - col.w * cov);
        }
    }
    textureStore(output, id.xy, vec4<f32>(acc));
}

// D-676, simulate::scatterize, a pixel a thread (k: k[1] the transfer mode, k[2] and k[3] the
// drawing's size, k[6] the count, the dots from 7, three each and not sprites, [x, y, n] in the
// drawing's own pixels, n the pixel j w + i the dot came from, farthest first, then each tile's
// start and the lists): every dot within 1 of the pixel centre, its colour the drawing's at n,
// laid onto clear in order by the transfer mode. Clear outside the drawing.
@compute @workgroup_size(16, 16)
fn scatterize(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + u32(k[6]) * 3u;
    let px = i32(id.x) - F.ox;
    let py = i32(id.y) - F.oy;
    var d = vec4(0.0lf);
    if px >= 0 && py >= 0 && f64(px) < k[2] && f64(py) < k[3] {
        let w0 = u32(k[2]);
        let x = f64(px) + 0.5lf;
        let y = f64(py) + 0.5lf;
        let mode = k[1];
        for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
            let s = 7u + u32(k[o + nt + 1u + j]) * 3u;
            let dx = x - k[s];
            let dy = y - k[s + 1u];
            let c = clamp(1.0lf - sqrt(dx * dx + dy * dy), 0.0lf, 1.0lf);
            if c <= 0.0lf {
                continue;
            }
            let n = u32(k[s + 2u]);
            let col = vec4<f64>(at(input, vec2<i32>(i32(n % w0) + F.ox, i32(n / w0) + F.oy)));
            if col.w <= 0.0lf {
                continue;
            }
            let sc = col * c;
            if mode == 1.0lf {
                d = sc + d - sc * d;
            } else if mode == 2.0lf {
                d = vec4(sc.xyz + d.xyz, sc.w + d.w * (1.0lf - sc.w));
            } else if mode == 3.0lf {
                d = vec4(sc.xyz + d.xyz, min(1.0lf, sc.w + d.w));
            } else {
                d = sc + d * (1.0lf - sc.w);
            }
        }
    }
    textureStore(output, id.xy, vec4<f32>(d));
}

// D-677, simulate::star_burst, a pixel a thread (k as simulate::star_list has it: k[0] the share
// of the picture mixed back, k[2] and k[3] the drawing's size, k[6] the count, the stars from 7,
// SPRITE_STRIDE each, [3, x, y, r, gx, gy, opacity, ..], then each tile's start and the lists):
// inside the drawing, the stars of its tile, each the drawing read at its grid point by Bubbles'
// bilinear read, laid farthest first onto clear; then mixed with the picture. Audit P2 (after
// B-157): each star's edge, read and laying over in single precision, the distance taken from the
// star's centre in double first so it stays small; the mix with the picture stays double.
@compute @workgroup_size(16, 16)
fn star_burst(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let tw = (size.x + 15u) / 16u;
    let nt = tw * ((size.y + 15u) / 16u);
    let t = (id.y / 16u) * tw + id.x / 16u;
    let o = 7u + SPRITE_STRIDE * u32(k[6]);
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    var d = vec4(0.0);
    if x > 0.0lf && y > 0.0lf && x < k[2] && y < k[3] {
        for (var j = u32(k[o + t]); j < u32(k[o + t + 1u]); j++) {
            let s = 7u + SPRITE_STRIDE * u32(k[o + nt + 1u + j]);
            let ex = f32(x - k[s + 1u]);
            let ey = f32(y - k[s + 2u]);
            let cv = clamp(f32(k[s + 3u]) - sqrt(ex * ex + ey * ey) + 0.5, 0.0, 1.0);
            if cv <= 0.0 {
                continue;
            }
            // Bubbles' read, its fraction single.
            let fx = clamp(k[s + 4u] - 0.5lf, 0.0lf, k[2] - 1.0lf);
            let fy = clamp(k[s + 5u] - 0.5lf, 0.0lf, k[3] - 1.0lf);
            let x0 = i32(fx);
            let y0 = i32(fy);
            let x1 = min(x0 + 1, i32(k[2]) - 1);
            let y1 = min(y0 + 1, i32(k[3]) - 1);
            let tx = f32(fx - f64(x0));
            let ty = f32(fy - f64(y0));
            let p = vec2(F.ox, F.oy);
            let r = (at(input, vec2(x0, y0) + p) * (1.0 - tx) + at(input, vec2(x1, y0) + p) * tx) * (1.0 - ty)
                + (at(input, vec2(x0, y1) + p) * (1.0 - tx) + at(input, vec2(x1, y1) + p) * tx) * ty;
            let c = r * (f32(k[s + 6u]) * cv);
            d = c + d * (1.0 - c.w);
        }
    }
    let m = k[0];
    textureStore(output, id.xy, vec4<f32>(vec4<f64>(d) * (1.0lf - m) + vec4<f64>(at(input, vec2<i32>(id.xy))) * m));
}

// B-151, median.rs's straight colour of a pixel, each channel over the covering in single
// precision as the CPU divides it, or nothing where nothing shows.
fn straight(p: vec4<f32>) -> vec4<f32> {
    if p.w <= 0.0 {
        return vec4(0.0);
    }
    let a = f64(p.w);
    return vec4(f32(quotient(f64(p.x), a)), f32(quotient(f64(p.y), a)), f32(quotient(f64(p.z), a)), p.w);
}

// f32::total_cmp's order, as whole numbers.
fn ordered(v: f32) -> u32 {
    let b = bitcast<u32>(v);
    return select(b | 0x80000000u, ~b, (b & 0x80000000u) != 0u);
}

fn unordered(u: u32) -> f32 {
    return bitcast<f32>(select(~u, u & 0x7fffffffu, (u & 0x80000000u) != 0u));
}

// B-151, median::median, a pixel a thread. k from `n`: each row of the disc's half-width, rows
// -r to r. `flag` is operate on alpha. Each channel's middle is found a bit at a time among the
// taps' keys, the value with as many below it as select_nth_unstable_by puts there; colours
// count only the taps that show, the covering every tap.
@compute @workgroup_size(16, 16)
fn median(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(textureDimensions(input));
    let p = vec2<i32>(id.xy);
    if p.x >= size.x || p.y >= size.y {
        return;
    }
    let own = straight(textureLoad(input, p, 0));
    // At radius 10, the largest, the disc has 317 taps.
    var keys: array<vec4<u32>, 320>;
    var n = 0u;
    var shows = 0u;
    var flat = true;
    for (var dy = -F.r; dy <= F.r; dy++) {
        let hw = i32(k[F.n + u32(dy + F.r)]);
        for (var dx = -hw; dx <= hw; dx++) {
            let q = p + vec2(dx, dy);
            var t = vec4(0.0);
            if all(q >= vec2(0)) && all(q < size) {
                t = straight(textureLoad(input, q, 0));
                flat = flat && all(t == own);
            } else {
                flat = false;
            }
            keys[n] = vec4(ordered(t.x), ordered(t.y), ordered(t.z), ordered(t.w));
            n++;
            shows += select(0u, 1u, t.w > 0.0);
        }
    }
    // A disc all of one colour and covering is its own median, as the CPU takes it.
    if flat {
        textureStore(output, id.xy, select(vec4(0.0), vec4(own.xyz * own.w, own.w), own.w > 0.0));
        return;
    }
    let total = vec4(shows, shows, shows, n);
    let want = total / 2u;
    let zero = ordered(0.0);
    var m = vec4(0u);
    for (var b = 32u; b > 0u; b--) {
        let t = m | vec4(1u << (b - 1u));
        var below = vec4(0u);
        for (var i = 0u; i < n; i++) {
            let key = keys[i];
            let counts = select(vec4(0u, 0u, 0u, 1u), vec4(1u), key.w > zero);
            below += select(vec4(0u), counts, key < t);
        }
        m = select(m, t, below <= want);
    }
    // For an even count, the mean of the middle and the greatest below it.
    var below = vec4(0u);
    var lower = vec4(0u);
    for (var i = 0u; i < n; i++) {
        let key = keys[i];
        let under = (key < m) & select(vec4(false, false, false, true), vec4(true), key.w > zero);
        below += select(vec4(0u), vec4(1u), under);
        lower = select(lower, max(lower, key), under);
    }
    lower = select(m, lower, below == want);
    var mid: vec4<f32>;
    for (var c = 0; c < 4; c++) {
        mid[c] = unordered(m[c]);
        if total[c] % 2u == 0u {
            mid[c] = (unordered(lower[c]) + mid[c]) / 2.0;
        }
    }
    var a = select(own.w, mid.w, F.flag == 1u);
    // D-453, mode 1, Dust & Scratches: a channel, or with `flag` the covering, whose 8-bit
    // value is within k[0] of the median's keeps its own.
    if F.mode == 1u && F.flag == 1u && abs(step8(mid.w) - step8(own.w)) <= k[0] {
        a = own.w;
    }
    if a <= 0.0 || shows == 0u {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    if F.mode == 1u && own.w > 0.0 {
        for (var c = 0; c < 3; c++) {
            if abs(level8(own[c], 1.0lf) - level8(mid[c], 1.0lf)) <= k[0] {
                mid[c] = own[c];
            }
        }
    }
    textureStore(output, id.xy, vec4(mid.xyz * a, a));
}

// B-151, median::smart_blur, in two passes. Mode 0: each pixel that shows its 8-bit straight
// colour and covering. Mode 1, `other` those: each pixel that shows the mean of the taps that
// show within the disc whose four are each within the threshold of its own. k: the threshold,
// then level8's numbers at 7 and 8, then from `n` the disc as `median` has it.
@compute @workgroup_size(16, 16)
fn smart(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(textureDimensions(input));
    let p = vec2<i32>(id.xy);
    if p.x >= size.x || p.y >= size.y {
        return;
    }
    let s = textureLoad(input, p, 0);
    if s.w <= 0.0 {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    if F.mode == 0u {
        let a = f64(s.w);
        let t = f32(f64(clamp(s.w, 0.0, 1.0)) * 255.0lf);
        let cover = f32(floor(f64(f32(f64(t) + 0.5lf))));
        textureStore(output, id.xy, vec4(f32(level8(s.x, a)), f32(level8(s.y, a)), f32(level8(s.z, a)), cover));
        return;
    }
    let own = textureLoad(other, p, 0);
    var s0 = 0.0lf;
    var s1 = 0.0lf;
    var s2 = 0.0lf;
    var s3 = 0.0lf;
    var n = 0u;
    for (var dy = -F.r; dy <= F.r; dy++) {
        let hw = i32(k[F.n + u32(dy + F.r)]);
        for (var dx = -hw; dx <= hw; dx++) {
            let q = p + vec2(dx, dy);
            if any(q < vec2(0)) || any(q >= size) {
                continue;
            }
            let t = textureLoad(input, q, 0);
            if t.w <= 0.0 {
                continue;
            }
            let l = textureLoad(other, q, 0);
            if abs(f64(l.x) - f64(own.x)) <= k[0] && abs(f64(l.y) - f64(own.y)) <= k[0]
                && abs(f64(l.z) - f64(own.z)) <= k[0] && abs(f64(l.w) - f64(own.w)) <= k[0] {
                s0 += f64(t.x);
                s1 += f64(t.y);
                s2 += f64(t.z);
                s3 += f64(t.w);
                n++;
            }
        }
    }
    let m = f64(n);
    textureStore(output, id.xy, vec4(f32(quotient(s0, m)), f32(quotient(s1, m)), f32(quotient(s2, m)), f32(quotient(s3, m))));
}

// B-237 (D-358), median::bilateral_blur, in two passes. Mode 0: each pixel that shows the levels
// of its straight colour, 255 times the sRGB curve, three of them, or with `flag` 0 the one of its
// luminance. Mode 1, `other` those: each pixel that shows the mean of the taps that show within
// the disc, each weighed by its covering and two bells, one on its distance and one on how far its
// level is from the pixel's own. k: the likeness's 1 / 2T^2, the distance's 1 / 2s^2, then from
// `n` the disc's rows. The taps are summed in single precision: double sums made it about ten
// times slower, and single stays well within a level of 255 of the CPU's double even over Radius
// 50's 7,850 taps (D-358 table).
@compute @workgroup_size(16, 16)
fn bilat(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(textureDimensions(input));
    let p = vec2<i32>(id.xy);
    if p.x >= size.x || p.y >= size.y {
        return;
    }
    let s = textureLoad(input, p, 0);
    if s.w <= 0.0 {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let a = f64(s.w);
    let r = f64(s.x) / a;
    let g = f64(s.y) / a;
    let b = f64(s.z) / a;
    if F.mode == 0u {
        if F.flag == 1u {
            textureStore(output, id.xy, vec4(f32(255.0lf * to_srgb(r)), f32(255.0lf * to_srgb(g)), f32(255.0lf * to_srgb(b)), 0.0));
        } else {
            let y = 0.2126lf * r + 0.7152lf * g + 0.0722lf * b;
            textureStore(output, id.xy, vec4(f32(255.0lf * to_srgb(y)), 0.0, 0.0, 0.0));
        }
        return;
    }
    let own = textureLoad(other, p, 0);
    let k0 = f32(k[0]);
    let k1 = f32(k[1]);
    var s0 = 0.0;
    var s1 = 0.0;
    var s2 = 0.0;
    var w0 = 0.0;
    var w1 = 0.0;
    var w2 = 0.0;
    for (var dy = -F.r; dy <= F.r; dy++) {
        let hw = i32(k[F.n + u32(dy + F.r)]);
        for (var dx = -hw; dx <= hw; dx++) {
            let q = p + vec2(dx, dy);
            if any(q < vec2(0)) || any(q >= size) {
                continue;
            }
            let t = textureLoad(input, q, 0);
            if t.w <= 0.0 {
                continue;
            }
            let l = textureLoad(other, q, 0);
            let near = exp(-f32(dx * dx + dy * dy) * k1) * t.w;
            let c0 = t.x / t.w;
            let c1 = t.y / t.w;
            let c2 = t.z / t.w;
            let d0 = l.x - own.x;
            let v0 = select(0.2126 * c0 + 0.7152 * c1 + 0.0722 * c2, c0, F.flag == 1u);
            let x0 = near * exp(-d0 * d0 * k0);
            s0 += x0 * v0;
            w0 += x0;
            if F.flag == 1u {
                let d1 = l.y - own.y;
                let d2 = l.z - own.z;
                let x1 = near * exp(-d1 * d1 * k0);
                let x2 = near * exp(-d2 * d2 * k0);
                s1 += x1 * c1;
                w1 += x1;
                s2 += x2 * c2;
                w2 += x2;
            }
        }
    }
    let o0 = quotient(f64(s0), f64(w0));
    var o1 = o0;
    var o2 = o0;
    if F.flag == 1u {
        o1 = quotient(f64(s1), f64(w1));
        o2 = quotient(f64(s2), f64(w2));
    }
    textureStore(output, id.xy, vec4(f32(o0 * a), f32(o1 * a), f32(o2 * a), s.w));
}

// B-335 (D-455), grade::remove_grain's denoise. Mode 0: each pixel that shows its straight colour
// through the sRGB curve, held in 0 to 1, and its covering. Mode 1, on those, pass k: each pixel
// that shows the mean of the taps that show within the disc, each weighed by its covering, a bell
// on its distance and one on its colour difference, with `flag` 1 one bell on the mean of the
// three squared differences, else one per channel. k: each channel's 1 / 2h^2, below 0 keeping
// the channel; the distance's 1 / 2s^2; then from `n` the disc's rows. Mode 2: back through the
// curve, times the covering. Summed in single precision, as Bilateral Blur's.
@compute @workgroup_size(16, 16)
fn degrain(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(textureDimensions(input));
    let p = vec2<i32>(id.xy);
    if p.x >= size.x || p.y >= size.y {
        return;
    }
    let s = textureLoad(input, p, 0);
    if s.w <= 0.0 {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let a = f64(s.w);
    if F.mode == 0u {
        let e = vec3(to_srgb(clamp(f64(s.x) / a, 0.0lf, 1.0lf)), to_srgb(clamp(f64(s.y) / a, 0.0lf, 1.0lf)), to_srgb(clamp(f64(s.z) / a, 0.0lf, 1.0lf)));
        textureStore(output, id.xy, vec4(f32(e.x), f32(e.y), f32(e.z), s.w));
        return;
    }
    if F.mode == 2u {
        textureStore(output, id.xy, vec4(f32(to_linear(f64(s.x)) * a), f32(to_linear(f64(s.y)) * a), f32(to_linear(f64(s.z)) * a), s.w));
        return;
    }
    let inv = vec3(f32(k[0]), f32(k[1]), f32(k[2]));
    let bell = max(inv, vec3(0.0));
    let k3 = f32(k[3]);
    var sum = vec3(0.0);
    var weight = vec3(0.0);
    for (var dy = -F.r; dy <= F.r; dy++) {
        let hw = i32(k[F.n + u32(dy + F.r)]);
        for (var dx = -hw; dx <= hw; dx++) {
            let q = p + vec2(dx, dy);
            if any(q < vec2(0)) || any(q >= size) {
                continue;
            }
            let t = textureLoad(input, q, 0);
            if t.w <= 0.0 {
                continue;
            }
            let near = exp(-f32(dx * dx + dy * dy) * k3) * t.w;
            let d = t.xyz - s.xyz;
            var x = near * exp(-d * d * bell);
            if F.flag == 1u {
                x = vec3(near * exp(-dot(d, d) / 3.0 * bell.x));
            }
            sum += x * t.xyz;
            weight += x;
        }
    }
    var out = s;
    for (var c = 0u; c < 3u; c++) {
        if inv[c] >= 0.0 {
            out[c] = f32(quotient(f64(sum[c]), f64(weight[c])));
        }
    }
    textureStore(output, id.xy, out);
}

// B-336 (D-456), a pixel's straight colour through the sRGB curve, held in 0 to 1.
fn db_enc(s: vec4<f32>) -> vec3<f64> {
    let a = f64(s.w);
    return vec3(to_srgb(clamp(f64(s.x) / a, 0.0lf, 1.0lf)), to_srgb(clamp(f64(s.y) / a, 0.0lf, 1.0lf)), to_srgb(clamp(f64(s.z) / a, 0.0lf, 1.0lf)));
}

// B-336, the edge weight of the pixel at `p` of `t`, which shows: its largest step in levels to its
// four neighbours that show, against the threshold and the softness, in double precision.
fn db_edge(t: texture_2d<f32>, p: vec2<i32>) -> f64 {
    let e = db_enc(textureLoad(t, p, 0));
    var g = 0.0lf;
    var n = array(vec2(-1, 0), vec2(1, 0), vec2(0, -1), vec2(0, 1));
    for (var i = 0; i < 4; i++) {
        let q = at(t, p + n[i]);
        if q.w > 0.0 {
            let d = abs(db_enc(q) - e) * 255.0lf;
            g = max(g, max(max(d.x, d.y), d.z));
        }
    }
    var edge = select(0.0lf, 1.0lf, g > k[0]);
    if k[1] > 0.0lf {
        edge = clamp((g - k[0]) / k[1], 0.0lf, 1.0lf);
    }
    return edge;
}

// B-336, round half away from 0.
fn db_away(v: f64) -> i32 {
    let r = i32(floor(abs(v) + 0.5lf));
    return select(r, -r, v < 0.0lf);
}

// B-336, what the pixel at `q` lends the blur across: its colour times a (1 - E) and that
// weight, from the drawing in `input` and mode 0's in `other`. B-430 (D-550): single precision.
fn db_lent(q: vec2<i32>) -> vec4<f32> {
    let t = textureLoad(input, q, 0);
    if t.w <= 0.0 {
        return vec4(0.0);
    }
    let ew = textureLoad(other, q, 0);
    let wt = t.w * ew.w;
    return vec4(ew.xyz * wt, wt);
}

// B-430 (D-550): the sums across at pixel index `i`, four to a pixel in `row`.
fn db_sum(i: i32) -> vec4<f32> {
    let j = u32(i) * 4u;
    return vec4(row[j], row[j + 1u], row[j + 2u], row[j + 3u]);
}

// B-336 (D-456), grade::deband. k: the threshold and the softness, the Edge Blur Reduction as a
// fraction, the diffusion amount and its reduction, the noise amount over 255 and its
// reduction, the view (0 Normal, 1 Edge Only, 2 Selected Edge), then from 8 the blur's taps from
// the middle out, `r` past it. Mode 0, on the drawing: each pixel's colour through the curve and
// 1 - E, its edge weight's complement, which single precision keeps however near 0 it is. Mode
// 2, on the drawing with mode 0's in `other`: the sums across of colour times weight and of
// weight, each tap weighed a (1 - E) and stopped by the edges between, into `row` in single
// precision (B-430, D-550: the double-precision sums of B-336 held within the same 1 level and
// cost most of the effect's time). Mode 3, on the drawing with mode 0's in
// `other`: the same down from `sums`, then the blurred colour pulled back on edges, and 1 - E.
// Mode 5, on mode 3's (mode 0's without a blur) with the drawing in `other`: the diffusion at
// step `frame`, the dither at step `g`, the view, back through the curve at the covering.
@compute @workgroup_size(16, 16)
fn deband(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(textureDimensions(input));
    let p = vec2<i32>(id.xy);
    if p.x >= size.x || p.y >= size.y {
        return;
    }
    let s = textureLoad(input, p, 0);
    if F.mode == 0u {
        // An empty pixel is no edge, so the blur passes it.
        if s.w <= 0.0 {
            textureStore(output, id.xy, vec4(0.0, 0.0, 0.0, 1.0));
            return;
        }
        let e = db_enc(s);
        let edge = db_edge(input, p);
        textureStore(output, id.xy, vec4(f32(e.x), f32(e.y), f32(e.z), f32(1.0lf - edge)));
        return;
    }
    if F.mode == 2u || F.mode == 3u {
        let across = F.mode == 2u;
        let dir = select(vec2(0, 1), vec2(1, 0), across);
        var total: vec4<f32>;
        if across {
            total = db_lent(p) * f32(k[8]);
        } else {
            total = db_sum(p.y * size.x + p.x) * f32(k[8]);
        }
        for (var sg = -1; sg <= 1; sg += 2) {
            var through = 1.0;
            for (var d = 1; d <= F.r; d++) {
                let q = p + dir * (sg * d);
                if any(q < vec2(0)) || any(q >= size) {
                    break;
                }
                var lent: vec4<f32>;
                if across {
                    lent = db_lent(q);
                } else {
                    lent = db_sum(q.y * size.x + q.x);
                }
                total += lent * (f32(k[8u + u32(d)]) * through);
                through *= textureLoad(other, q, 0).w;
            }
        }
        if across {
            let j = u32(p.y * size.x + p.x) * 4u;
            row[j] = total.x;
            row[j + 1u] = total.y;
            row[j + 2u] = total.z;
            row[j + 3u] = total.w;
            return;
        }
        let ew = textureLoad(other, p, 0);
        var m = ew.xyz;
        if s.w > 0.0 && total.w > 0.0 {
            m += (total.xyz / total.w - m) * (1.0 - (1.0 - ew.w) * f32(k[2]));
        }
        textureStore(output, id.xy, vec4(m, ew.w));
        return;
    }
    let still = textureLoad(other, p, 0);
    if still.w <= 0.0 {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    // The edge weight again in double precision: the single one kept beside the colour can move
    // a diffusion offset across a half pixel.
    let edge = db_edge(other, p);
    let own = vec3<f64>(s.xyz);
    var d = own;
    let lx = p.x - F.ox;
    let ly = p.y - F.oy;
    if k[3] > 0.0lf {
        let kk = k[3] * (1.0lf - edge * k[4]);
        let q = p + vec2(db_away(kk * hashed(lx, ly, F.frame, 0u)), db_away(kk * hashed(lx, ly, F.frame, 1u)));
        if all(q >= vec2(0)) && all(q < size) && textureLoad(other, q, 0).w > 0.0 {
            let t = textureLoad(input, q, 0);
            let lend = f64(t.w);
            d = vec3<f64>(t.xyz) * lend + own * (1.0lf - lend);
        }
    }
    let tri = (hashed(lx, ly, F.g, 2u) + hashed(lx, ly, F.g, 3u)) / 2.0lf;
    let n = tri * k[5] * (1.0lf - edge * k[6]);
    var o = clamp(d + vec3(n), vec3(0.0lf), vec3(1.0lf));
    if k[7] == 1.0lf {
        o = vec3(edge);
    } else if k[7] == 2.0lf {
        o = o * (1.0lf - edge) + vec3(edge, 0.0lf, 0.0lf);
    }
    let a = f64(still.w);
    textureStore(output, id.xy, vec4(f32(to_linear(o.x) * a), f32(to_linear(o.y) * a), f32(to_linear(o.z) * a), still.w));
}

// B-151, layer_fx::roughen_edges' least covering at `e` either way across and down.
fn least(x: f64, y: f64, e: f64) -> f64 {
    let c = f64(bilinear(input, x, y).w);
    let r = f64(bilinear(input, x + e, y).w);
    let l = f64(bilinear(input, x - e, y).w);
    let d = f64(bilinear(input, x, y + e).w);
    let u = f64(bilinear(input, x, y - e).w);
    return min(min(min(min(c, r), l), d), u);
}

// B-151, layer_fx::roughen_edges: the seed in `base`, the drawing's corner at `ox`, `oy`, the
// octaves in `count`. k: the border, the size, the depth, colour, the colour in linear light.
@compute @workgroup_size(16, 16)
fn rough(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let p = f64(px.w);
    if p == 0.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let f = fractal(0u, vec3(quotient(x - f64(F.ox), k[1]), quotient(y - f64(F.oy), k[1]), k[2]), F.count);
    let e = k[0] * clamp(0.5lf + f, 0.0lf, 1.0lf);
    let m = least(x, y, e);
    let s = quotient(m, p);
    var t = 0.0lf;
    if k[3] == 1.0lf && m > 0.0lf {
        t = 1.0lf - quotient(min(least(x, y, 2.0lf * e), m), m);
    }
    var out: vec4<f32>;
    for (var c = 0u; c < 3u; c++) {
        out[c] = f32((1.0lf - t) * f64(px[c]) * s + t * k[4u + c] * m);
    }
    out.w = f32(m);
    textureStore(output, id.xy, out);
}

// B-151, layer_fx::radial_shadow's shadow from the blurred cast (`input`) laid behind the
// drawing (`other`, its corner at `ox`, `oy`). k: the opacity and the influence as shares,
// glass, shadow only, the colour in linear light.
@compute @workgroup_size(16, 16)
fn rshadow(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let s = textureLoad(input, id.xy, 0);
    let a = f64(s.w) * k[0];
    var d = vec4(0.0);
    if k[3] == 0.0lf {
        d = at(other, vec2(i32(id.x) - F.ox, i32(id.y) - F.oy));
    }
    let behind = 1.0lf - f64(d.w);
    var out: vec4<f32>;
    for (var c = 0u; c < 3u; c++) {
        var shadow = k[4u + c] * a;
        if k[2] == 1.0lf {
            shadow = ((1.0lf - k[1]) * k[4u + c] * f64(s.w) + k[1] * f64(s[c])) * k[0];
        }
        out[c] = f32(f64(d[c]) + shadow * behind);
    }
    out.w = f32(f64(d.w) + a * behind);
    textureStore(output, id.xy, out);
}

// B-151, layer_fx::bevel_alpha: `other` the covering blurred, grown by `r`. k: the thickness,
// the light's direction, the intensity, the light in linear light.
@compute @workgroup_size(16, 16)
fn bevel(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let a = f64(px.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    let x = i32(id.x) + F.r;
    let y = i32(id.y) + F.r;
    let gx = (f64(at(other, vec2(x + 1, y)).w) - f64(at(other, vec2(x - 1, y)).w)) / 2.0lf;
    let gy = (f64(at(other, vec2(x, y + 1)).w) - f64(at(other, vec2(x, y - 1)).w)) / 2.0lf;
    let s = clamp(-1.25lf * k[0] * (gx * k[1] + gy * k[2]), -1.0lf, 1.0lf);
    var out = px;
    for (var c = 0u; c < 3u; c++) {
        let p = f64(px[c]);
        if s > 0.0lf {
            out[c] = f32(p + (k[4u + c] * a - p) * k[3] * s);
        } else {
            out[c] = f32(p * (1.0lf + k[3] * s));
        }
    }
    textureStore(output, id.xy, out);
}

// B-151, layer_fx::snowfall: the seed in `base`, the drawing's corner at `ox`, `oy`. k: the
// density as a share, the opacity, the period, the frame, each plane's cell, largest radius,
// sway, drift and fall from 4, the colour in linear light from 19.
@compute @workgroup_size(16, 16)
fn snow(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a == 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let x = f64(i32(id.x) - F.ox) + 0.5lf;
    let y = f64(i32(id.y) - F.oy) + 0.5lf;
    let beat = quotient(k[3], k[2]);
    var q = 0.0lf;
    for (var l = 0; l < 3; l++) {
        let o = 4u + 5u * u32(l);
        let s = k[o];
        let rmax = k[o + 1u];
        let wz = k[o + 2u];
        let fa = x - k[o + 3u];
        let fb = y - k[o + 4u];
        let reach = rmax + 0.5lf;
        for (var ci = i32(floor(quotient(fa - reach - wz, s))); ci <= i32(floor(quotient(fa + reach + wz, s))); ci++) {
            for (var cj = i32(floor(quotient(fb - reach, s))); cj <= i32(floor(quotient(fb + reach, s))); cj++) {
                if (hashed(ci, cj, l, 0u) + 1.0lf) / 2.0lf >= k[0] {
                    continue;
                }
                let sway = wz * sin64(6.283185307179586lf * (beat + (hashed(ci, cj, l, 4u) + 1.0lf) / 2.0lf));
                let cx = s * (f64(ci) + (hashed(ci, cj, l, 1u) + 1.0lf) / 2.0lf) + sway;
                let cy = s * (f64(cj) + (hashed(ci, cj, l, 2u) + 1.0lf) / 2.0lf);
                let r = rmax * (0.5lf + 0.5lf * (hashed(ci, cj, l, 3u) + 1.0lf) / 2.0lf);
                let dx = fa - cx;
                let dy = fb - cy;
                let d = sqrt(dx * dx + dy * dy);
                q = max(q, clamp(r + 0.5lf - d, 0.0lf, 1.0lf) * min(2.0lf * r, 1.0lf));
            }
        }
    }
    if q == 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let op = quotient(q * k[1], 100.0lf);
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let b = quotient(f64(p[c]), a);
        out[c] = f32((b + op * (k[19u + c] - b)) * a);
    }
    textureStore(output, id.xy, out);
}

// B-151, grade::cell_pattern: the drawing's corner at `ox`, `oy`. k: the size, the contrast, the
// opacity as a share, the pattern (bubbles, crystals, plates, else the cell's grey), invert, the
// first cell across and down, the cells across, the dark and light colours encoded, 0, then
// each cell's point and grey from 15, as grade::cell_points has them.
@compute @workgroup_size(16, 16)
fn cells(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let x = quotient(f64(i32(id.x) - F.ox) + 0.5lf, k[0]);
    let y = quotient(f64(i32(id.y) - F.oy) + 0.5lf, k[0]);
    let ci = i32(floor(x)) - 2 - i32(k[5]);
    let cj = i32(floor(y)) - 2 - i32(k[6]);
    let cols = i32(k[7]);
    // The nearest and second nearest, squared, the first found keeping a tie.
    var f1 = 1e300lf;
    var f2 = 1e300lf;
    var g = 0.0lf;
    for (var n = cj; n < cj + 5; n++) {
        for (var m = ci; m < ci + 5; m++) {
            let o = 15u + 3u * u32(n * cols + m);
            let d = product(x - k[o], x - k[o], k[14]) + product(y - k[o + 1u], y - k[o + 1u], k[14]);
            if d < f1 {
                f2 = f1;
                f1 = d;
                g = k[o + 2u];
            } else if d < f2 {
                f2 = d;
            }
        }
    }
    let r1 = sqrt(f1);
    let r2 = sqrt(f2);
    var s = 0.0lf;
    if r1 + r2 > 0.0lf {
        s = quotient(2.0lf * r1, r1 + r2);
    }
    var v = g;
    switch u32(k[3]) {
        case 0u: { v = sqrt(1.0lf - s * s); }
        case 1u: { v = 1.0lf - s; }
        case 2u: { v = min(4.0lf * (1.0lf - s), 1.0lf); }
        default: {}
    }
    if k[4] == 1.0lf {
        v = 1.0lf - v;
    }
    v = clamp(0.5lf + (v - 0.5lf) * k[1] / 100.0lf, 0.0lf, 1.0lf);
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let color = to_linear(k[8u + c] + v * (k[11u + c] - k[8u + c]));
        let b = quotient(f64(p[c]), a);
        out[c] = f32((b + k[2] * (mixed(b, color) - b)) * a);
    }
    textureStore(output, id.xy, out);
}

// B-223, effects::channel_blur: channel F.count of each pixel the colour of its own blur, `other`,
// whose corner is F.g pixels out, over that blur's covering and times the pixel's.
@compute @workgroup_size(16, 16)
fn chanmix(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var p = textureLoad(input, id.xy, 0);
    let q = at(other, vec2<i32>(id.xy) + vec2(F.g));
    if q.w > 0.0 {
        let c = F.count;
        p[c] = f32(f64(f32(quotient(f64(q[c]), f64(q.w)))) * f64(p.w));
    }
    textureStore(output, id.xy, p);
}

// D-360, blurs::cross_mix: `input`, the blur across, laid on `other`, the blur down, the same
// size, by Cross Blur's mode F.mode: blend their mean, every other document 21's rule.
@compute @workgroup_size(16, 16)
fn crossmix(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let s = textureLoad(input, id.xy, 0);
    let d = textureLoad(other, id.xy, 0);
    if F.mode == 0u {
        textureStore(output, id.xy, (s + d) / 2.0);
        return;
    }
    var cs = vec3(0.0);
    var cd = vec3(0.0);
    if s.w != 0.0 {
        cs = s.xyz / s.w;
    }
    if d.w != 0.0 {
        cd = d.xyz / d.w;
    }
    var b = min(cs, cd);
    switch F.mode {
        case 1u: { b = min(cs + cd, vec3(1.0)); }
        case 2u: { b = cs + cd - cs * cd; }
        case 3u: { b = cs * cd; }
        case 4u: { b = max(cs, cd); }
        default: {}
    }
    let c = (1.0 - s.w) * d.xyz + (1.0 - d.w) * s.xyz + s.w * d.w * b;
    textureStore(output, id.xy, vec4(c, s.w + d.w - s.w * d.w));
}

// B-223, selective_blur::selective_color_blur: each pixel's straight colour encoded in single
// precision, and 1 where it is chosen (k as `picked` and `srgb32` read it); clear where it shows
// nothing.
@compute @workgroup_size(16, 16)
fn selprep(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    if p.w <= 0.0 {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let a = f64(p.w);
    let e = vec3(srgb32(p.x, a), srgb32(p.y, a), srgb32(p.z, a));
    let on = picked(vec3(step8(e.x), step8(e.y), step8(e.z)));
    textureStore(output, id.xy, vec4(e, select(0.0, 1.0, on)));
}

// B-223, selective_blur::pass: each chosen pixel the weighted mean of itself and the chosen
// pixels either side along rows (F.flag 1) or columns, out to F.n, each side stopping at the
// first not chosen or the edge. k: the weights from the middle out.
@compute @workgroup_size(16, 16)
fn selpass(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(textureDimensions(input));
    if i32(id.x) >= size.x || i32(id.y) >= size.y {
        return;
    }
    let own = textureLoad(input, id.xy, 0);
    if own.w == 0.0 {
        textureStore(output, id.xy, own);
        return;
    }
    let step = select(vec2(0, 1), vec2(1, 0), F.flag == 1u);
    var got = vec3(0.0);
    var total = f32(k[0]);
    for (var side = -1; side <= 1; side += 2) {
        for (var j = 1; j <= i32(F.n); j++) {
            let m = vec2<i32>(id.xy) + step * (side * j);
            if any(m < vec2(0)) || any(m >= size) {
                break;
            }
            let o = textureLoad(input, m, 0);
            if o.w == 0.0 {
                break;
            }
            // B-224: u32, as B-172's run reads k from its own offset, a u32.
            let wk = f32(k[u32(j)]);
            got += wk * (o.xyz - own.xyz);
            total += wk;
        }
    }
    textureStore(output, id.xy, vec4(own.xyz + got / total, 1.0));
}

// B-223: each chosen pixel whose blurred colour (`other`) is not its own, that colour decoded and
// times its covering. k as `selprep`'s.
@compute @workgroup_size(16, 16)
fn selfinish(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let o = textureLoad(other, id.xy, 0);
    var out = p;
    if o.w == 1.0 && p.w > 0.0 {
        let a = f64(p.w);
        let e = vec3(srgb32(p.x, a), srgb32(p.y, a), srgb32(p.z, a));
        if any(o.xyz != e) {
            for (var c = 0u; c < 3u; c++) {
                out[c] = f32(to_linear(f64(o[c])) * a);
            }
        }
    }
    textureStore(output, id.xy, out);
}

// B-223, grade::phase_of: the phase `kind` (red, green, blue, alpha, luminance, lightness, hue,
// saturation; B-224, 8 intensity) of `q`.
fn phase(kind: u32, q: vec4<f32>) -> f64 {
    let a = f64(q.w);
    if kind == 3u {
        return a;
    }
    if a <= 0.0lf {
        return 0.0lf;
    }
    let b = vec3(
        clamp(quotient(f64(q.x), a), 0.0lf, 1.0lf),
        clamp(quotient(f64(q.y), a), 0.0lf, 1.0lf),
        clamp(quotient(f64(q.z), a), 0.0lf, 1.0lf),
    );
    let c = vec3(to_srgb(b.x), to_srgb(b.y), to_srgb(b.z));
    switch kind {
        case 0u: { return c.x; }
        case 1u: { return c.y; }
        case 2u: { return c.z; }
        case 4u: { return to_srgb(luma(b)); }
        case 5u: { return hsl(c).z; }
        case 6u: { return quotient(hsl(c).x, 360.0lf); }
        case 8u: { return (c.x + c.y + c.z) / 3.0lf; }
        default: { return hsl(c).y; }
    }
}

// B-223, layer_fx::vector_blur's height (B-224, and layer_fx::glass's bump), as the alpha of the
// output (the drawing's size): the `phase` k[0] of `input`, the drawing or a map whose corner is
// F.ox, F.oy in, times its covering unless alpha.
@compute @workgroup_size(16, 16)
fn vheight(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let q = at(input, vec2<i32>(id.xy) - vec2(F.ox, F.oy));
    let kind = u32(k[0]);
    var ph = phase(kind, q);
    if kind != 3u {
        ph = ph * f64(q.w);
    }
    textureStore(output, id.xy, vec4(0.0, 0.0, 0.0, f32(ph)));
}

// B-223: `vblur`'s height at `p`, `other`'s alpha, its corner F.r pixels out.
fn height(p: vec2<i32>) -> f64 {
    return f64(at(other, p + vec2(F.r)).w);
}

// B-223, layer_fx::vector_blur: each pixel the weighted mean of bilinear samples of the drawing
// along its vector. k: the amount, the angle, the ridge, ceil(amount), the type (natural,
// constant, perpendicular, direction_center, direction_fading), the turn's cosine and sine.
@compute @workgroup_size(16, 16)
fn vblur(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = i32(id.x);
    let y = i32(id.y);
    let kind = u32(k[4]);
    var ux: f64;
    var uy: f64;
    var len = k[0];
    if kind >= 3u {
        // blurs::along, its quarter turns exact.
        let d = k[1] + 360.0lf * k[2] * height(vec2(x, y));
        let q = rem(d, 360.0lf);
        ux = sin64(d * 0.017453292519943295lf);
        uy = -cos64(d * 0.017453292519943295lf);
        if q == 0.0lf {
            ux = 0.0lf;
            uy = -1.0lf;
        } else if q == 90.0lf {
            ux = 1.0lf;
            uy = 0.0lf;
        } else if q == 180.0lf {
            ux = 0.0lf;
            uy = 1.0lf;
        } else if q == 270.0lf {
            ux = -1.0lf;
            uy = 0.0lf;
        }
    } else {
        let gx = (height(vec2(x + 1, y)) - height(vec2(x - 1, y))) / 2.0lf;
        let gy = (height(vec2(x, y + 1)) - height(vec2(x, y - 1))) / 2.0lf;
        let g = sqrt(gx * gx + gy * gy);
        if g < 1e-4lf {
            textureStore(output, id.xy, textureLoad(input, id.xy, 0));
            return;
        }
        let vx = quotient(gx, g);
        let vy = quotient(gy, g);
        ux = vx * k[5] - vy * k[6];
        uy = vx * k[6] + vy * k[5];
        if kind != 1u && k[2] != 0.0lf {
            let m = 100.0lf * g;
            len = quotient(k[0] * 100.0lf * g, sqrt(m * m + k[2] * k[2]));
        }
    }
    let n = i32(k[3]);
    let fading = kind == 0u || kind == 2u || kind == 4u;
    var acc = array<f64, 4>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    var total = 0.0lf;
    let cx = f64(x) + 0.5lf;
    let cy = f64(y) + 0.5lf;
    for (var j = select(-n, 0, kind == 4u); j <= n; j++) {
        let t = quotient(f64(j), f64(n));
        var wt = 1.0lf;
        if fading {
            wt = 1.0lf - abs(t);
        }
        if wt == 0.0lf {
            continue;
        }
        let s = bilinear(input, cx + t * len * ux, cy + t * len * uy);
        for (var c = 0; c < 4; c++) {
            acc[c] += wt * f64(s[c]);
        }
        total += wt;
    }
    var out = vec4(0.0);
    for (var c = 0; c < 4; c++) {
        out[c] = f32(quotient(acc[c], total));
    }
    textureStore(output, id.xy, out);
}

// B-223, layer_fx::compound_blur's sigmas into `dist`: each pixel of the drawing, F.n wide and
// F.count tall, k[0] times the brightness of the map (`input`, its corner F.ox, F.oy in), or with
// k[1] one less that.
@compute @workgroup_size(16, 16)
fn csigma(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= F.n || id.y >= F.count {
        return;
    }
    let m = vec2<i32>(id.xy) - vec2(F.ox, F.oy);
    var v = 0.0lf;
    if all(m >= vec2(0)) && all(m < vec2<i32>(textureDimensions(input))) {
        let p = textureLoad(input, m, 0);
        v = to_srgb(clamp(luma(vec3(f64(p.x), f64(p.y), f64(p.z))), 0.0lf, 1.0lf));
    }
    if k[1] == 1.0lf {
        v = 1.0lf - v;
    }
    dist[id.y * F.n + id.x] = v * k[0];
}

// B-223: layer_fx::compound_blur's mix: each pixel whose sigma (`dist`) falls in segment F.n of
// the levels k, between `input` (its corner F.g pixels out) and `other` (F.r out); the rest left
// as the output has them.
@compute @workgroup_size(16, 16)
fn cmix(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let s = dist[id.y * size.x + id.x];
    var seg = 0u;
    for (var j = 4; j >= 0; j--) {
        if k[u32(j)] <= s {
            seg = u32(j);
            break;
        }
    }
    if seg != F.n {
        return;
    }
    let lo = k[seg];
    let hi = k[seg + 1u];
    let t = quotient(s - lo, hi - lo);
    let a = at(input, vec2<i32>(id.xy) + vec2(F.g));
    let b = at(other, vec2<i32>(id.xy) + vec2(F.r));
    var out = vec4(0.0);
    for (var c = 0; c < 4; c++) {
        let x = f64(a[c]);
        out[c] = f32(x + t * (f64(b[c]) - x));
    }
    textureStore(output, id.xy, out);
}

// B-224, layer_fx::displacement_map's `value` of the map pixel `m`: the `phase` `kind`, 9 full,
// 10 off, mid grey where the map shows nothing, drawn toward mid grey by the map's covering.
fn push(kind: u32, m: vec4<f32>) -> f64 {
    let a = f64(m.w);
    if kind == 9u {
        return 1.0lf;
    }
    if kind == 10u {
        return 0.5lf;
    }
    if kind == 3u {
        return a;
    }
    if a <= 0.0lf {
        return 0.5lf;
    }
    return 0.5lf + a * (phase(kind, m) - 0.5lf);
}

// B-224, layer_fx::wrapped: `bilinear` with each of the four pixels' places taken round `t`.
fn wrapped(t: texture_2d<f32>, x: f64, y: f64) -> vec4<f32> {
    let size = vec2<f64>(textureDimensions(t));
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;
    var out = vec4(0.0);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0lf - uy, uy, j == 1);
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0lf - ux, ux, i == 1);
            let p = vec2(i32(euclid(x0 + f64(i), size.x)), i32(euclid(y0 + f64(j), size.y)));
            out += textureLoad(t, p, 0) * f32(wx * wy);
        }
    }
    return out;
}

// D-458, layer_fx::wire_removal: F.mode the style (0 fade, 1 frame offset, 2 displace, 3 along
// the rows); `other` the other frame's picture for 1. k: the wire's start, its unit step, its
// length, h, r, E, the mirror's share, a 0.
fn wire_held(x: f64, y: f64) -> vec4<f32> {
    let size = vec2<f64>(textureDimensions(input));
    return bilinear(input, clamp(x, 0.5lf, size.x - 0.5lf), clamp(y, 0.5lf, size.y - 0.5lf));
}

fn wire_at(j: f64, along: f64, d: f64, cx: f64, cy: f64) -> vec4<f32> {
    let z = k[9];
    if F.mode == 2u {
        return wire_held(k[0] + product(along, k[2], z) + product(j, -k[3], z), k[1] + product(along, k[3], z) + product(j, k[2], z));
    }
    return wire_held(cx + quotient(j - d, -k[3]), cy);
}

@compute @workgroup_size(16, 16)
fn wire(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let z = k[9];
    let c = textureLoad(input, vec2<i32>(id.xy), 0);
    let cx = f64(id.x) + 0.5lf;
    let cy = f64(id.y) + 0.5lf;
    let x = cx - k[0];
    let y = cy - k[1];
    let along = product(x, k[2], z) + product(y, k[3], z);
    let d = product(x, -k[3], z) + product(y, k[2], z);
    let t = quotient(along, k[4]);
    let q = abs(d);
    let e = k[7];
    if t < 0.0lf || t > 1.0lf || q >= e {
        textureStore(output, vec2<i32>(id.xy), c);
        return;
    }
    var weight = 1.0lf;
    if q > k[5] {
        weight = 1.0lf - quotient(q - k[5], k[6]);
    }
    var rr = vec4<f64>(0.0lf);
    if F.mode == 1u {
        rr = vec4<f64>(at(other, vec2<i32>(id.xy)));
    } else if F.mode >= 2u {
        var s = -1.0lf;
        if d >= 0.0lf {
            s = 1.0lf;
        }
        let lo = vec4<f64>(wire_at(-e, along, d, cx, cy));
        let hi = vec4<f64>(wire_at(e, along, d, cx, cy));
        let mir = vec4<f64>(wire_at(s * (2.0lf * e - q), along, d, cx, cy));
        let f = quotient(d + e, 2.0lf * e);
        for (var ch = 0; ch < 4; ch++) {
            let inter = lo[ch] + product(hi[ch] - lo[ch], f, z);
            rr[ch] = inter + product(mir[ch] - inter, k[8], z);
        }
    }
    var out = vec4(0.0);
    for (var ch = 0; ch < 4; ch++) {
        out[ch] = f32(product(f64(c[ch]), 1.0lf - weight, z) + product(rr[ch], weight, z));
    }
    textureStore(output, vec2<i32>(id.xy), out);
}

// B-224, layer_fx::displacement_map: each pixel of the output, the drawing (`input`) grown by F.g,
// the drawing read bilinearly where the map (`other`, its corner F.ox, F.oy in the drawing) moves
// it; past the map's edge, when grown, the map's nearest edge pixel, else clear. k: across's and
// down's `push` kind, their most, 1 to wrap round the drawing; D-412: the spectrum's samples (0
// for none) and the three colours' shares, read as layer_fx::chromatic_samples reads them.
@compute @workgroup_size(16, 16)
fn dmap(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let ms = vec2<i32>(textureDimensions(other));
    let mp = vec2<i32>(id.xy) - vec2(F.ox + F.g, F.oy + F.g);
    var m = vec4(0.0);
    if all(mp >= vec2(0)) && all(mp < ms) {
        m = textureLoad(other, mp, 0);
    } else if F.g > 0 {
        m = textureLoad(other, clamp(mp, vec2(0), ms - vec2(1)), 0);
    }
    let g = f64(F.g);
    let dx = (2.0lf * push(u32(k[0]), m) - 1.0lf) * k[2];
    let dy = (2.0lf * push(u32(k[1]), m) - 1.0lf) * k[3];
    let n = u32(k[5]);
    if n >= 2u {
        let cx = f64(id.x) + 0.5lf - g;
        let cy = f64(id.y) + 0.5lf - g;
        var sum = array<f64, 3>(0.0lf, 0.0lf, 0.0lf);
        var total = array<f64, 3>(0.0lf, 0.0lf, 0.0lf);
        var cover = 0.0;
        for (var i = 0u; i < n; i++) {
            let t = f64(i) / f64(n - 1u);
            var a = k[7] + (k[8] - k[7]) * (2.0lf * t - 1.0lf);
            if t <= 0.5lf {
                a = k[6] + (k[7] - k[6]) * 2.0lf * t;
            }
            let wt = array<f64, 3>(max(0.0lf, 1.0lf - 2.0lf * t), 1.0lf - abs(2.0lf * t - 1.0lf), max(0.0lf, 2.0lf * t - 1.0lf));
            var p = bilinear(input, cx + a * dx, cy + a * dy);
            if k[4] == 1.0lf {
                p = wrapped(input, cx + a * dx, cy + a * dy);
            }
            for (var c = 0; c < 3; c++) {
                sum[c] += wt[c] * f64(p[c]);
                total[c] += wt[c];
            }
            cover = max(cover, p.w);
        }
        textureStore(output, id.xy, vec4(f32(sum[0] / total[0]), f32(sum[1] / total[1]), f32(sum[2] / total[2]), cover));
        return;
    }
    let sx = f64(id.x) + 0.5lf + dx - g;
    let sy = f64(id.y) + 0.5lf + dy - g;
    if k[4] == 1.0lf {
        textureStore(output, id.xy, wrapped(input, sx, sy));
    } else {
        textureStore(output, id.xy, bilinear(input, sx, sy));
    }
}

// B-224, layer_fx::glass: the bump's slope (`other`, grown by F.r) by central differences times
// k[1]; each pixel the drawing read bilinearly k[0] pixels along it, then lit as `bevel` lights
// it, its slope toward the light (k[2], k[3]) held to -1..1. k[4] the intensity, then the light
// in linear light. D-561: k[8] 0 for that classic light, 1 distant, 2 point, lit then as
// `plastic` lights it without the dust or an ambient colour: k[9], k[10] the point and k[11] the
// height, k[12..15] the distant light's direction, then ambient, diffuse, specular, roughness,
// metal. The highlight's power is single precision, as `plastic`'s.
@compute @workgroup_size(16, 16)
fn glass(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = i32(id.x) + F.r;
    let y = i32(id.y) + F.r;
    let nx = (f64(at(other, vec2(x + 1, y)).w) - f64(at(other, vec2(x - 1, y)).w)) / 2.0lf * k[1];
    let ny = (f64(at(other, vec2(x, y + 1)).w) - f64(at(other, vec2(x, y - 1)).w)) / 2.0lf * k[1];
    var px = textureLoad(input, id.xy, 0);
    if k[0] != 0.0lf {
        px = bilinear(input, f64(id.x) + 0.5lf + k[0] * nx, f64(id.y) + 0.5lf + k[0] * ny);
    }
    let a = f64(px.w);
    if k[8] != 0.0lf {
        if a <= 0.0lf {
            textureStore(output, id.xy, vec4(0.0));
            return;
        }
        let n = blob_unit(vec3(-nx, -ny, 1.0lf));
        var l = vec3(k[12], k[13], k[14]);
        if k[8] == 2.0lf {
            l = blob_unit(vec3(k[9] - (f64(id.x) + 0.5lf), k[10] - (f64(id.y) + 0.5lf), k[11]));
        }
        let nl = n.x * l.x + n.y * l.y + n.z * l.z;
        let rz = 2.0lf * nl * n.z - l.z;
        var shine = 0.0lf;
        if nl > 0.0lf && rz > 0.0lf {
            shine = f64(pow(f32(rz), f32(1.0lf / k[18])));
        }
        for (var c = 0u; c < 3u; c++) {
            let cc = quotient(f64(px[c]), a);
            let lc = k[5u + c];
            let lit = cc * (k[15] + k[16] * k[4] * lc * max(nl, 0.0lf)) + k[17] * k[4] * (lc + (cc - lc) * k[19]) * shine;
            px[c] = f32(lit * a);
        }
        textureStore(output, id.xy, px);
        return;
    }
    if k[4] > 0.0lf && a > 0.0lf {
        let s = clamp(-(nx * k[2] + ny * k[3]), -1.0lf, 1.0lf);
        for (var c = 0u; c < 3u; c++) {
            let p = f64(px[c]);
            if s > 0.0lf {
                px[c] = f32(p + (k[5u + c] * a - p) * k[4] * s);
            } else {
                px[c] = f32(p * (1.0lf + k[4] * s));
            }
        }
    }
    textureStore(output, id.xy, px);
}

// D-379, layer_fx::blobbylize's colour: the drawing over its own blur (`other`, grown by F.r),
// straight.
@compute @workgroup_size(16, 16)
fn blobover(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let o = textureLoad(input, id.xy, 0);
    let s = textureLoad(other, vec2<i32>(id.xy) + vec2(F.r), 0);
    let m = 1.0lf - f64(o.w);
    let ka = f64(o.w) + m * f64(s.w);
    var out = vec4(0.0);
    if ka > 0.0lf {
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(quotient(f64(o[c]) + m * f64(s[c]), ka));
        }
    }
    textureStore(output, id.xy, out);
}

// D-379: `v` made a unit long, or 0.
fn blob_unit(v: vec3<f64>) -> vec3<f64> {
    let l = sqrt(v.x * v.x + v.y * v.y + v.z * v.z);
    if l > 0.0lf {
        return vec3(v.x / l, v.y / l, v.z / l);
    }
    return vec3(0.0lf, 0.0lf, 0.0lf);
}

// D-379, layer_fx::blobbylize: the covering from the blob's height (`other`'s alpha, grown by
// F.r) less the cut, the surface lit by Phong's rule on the straight colour `input` (blobover's).
// k: the cut, the slope's scale, 1 for a point light, its place and the height, the distant
// light's direction, the light in linear light, the intensity, ambient, diffuse, specular,
// roughness, metal. The highlight's power is single precision.
@compute @workgroup_size(16, 16)
fn blobby(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = i32(id.x) + F.r;
    let y = i32(id.y) + F.r;
    var a = 0.0lf;
    if k[0] < 1.0lf {
        a = clamp(quotient(f64(at(other, vec2(x, y)).w) - k[0], 1.0lf - k[0]), 0.0lf, 1.0lf);
    }
    if a == 0.0lf {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let gx = (f64(at(other, vec2(x + 1, y)).w) - f64(at(other, vec2(x - 1, y)).w)) / 2.0lf;
    let gy = (f64(at(other, vec2(x, y + 1)).w) - f64(at(other, vec2(x, y - 1)).w)) / 2.0lf;
    let n = blob_unit(vec3(-k[1] * gx, -k[1] * gy, 1.0lf));
    var l = vec3(k[6], k[7], k[8]);
    if k[2] == 1.0lf {
        l = blob_unit(vec3(k[3] - (f64(id.x) + 0.5lf), k[4] - (f64(id.y) + 0.5lf), k[5]));
    }
    let nl = n.x * l.x + n.y * l.y + n.z * l.z;
    let rz = 2.0lf * nl * n.z - l.z;
    var shine = 0.0lf;
    if nl > 0.0lf && rz > 0.0lf {
        shine = f64(pow(f32(rz), f32(1.0lf / k[16])));
    }
    let col = textureLoad(input, id.xy, 0);
    var out = vec4(0.0);
    for (var c = 0u; c < 3u; c++) {
        let cc = f64(col[c]);
        let lc = k[9u + c];
        let lit = cc * (k[13] + k[14] * k[12] * lc * max(nl, 0.0lf)) + k[15] * k[12] * (lc + (cc - lc) * k[17]) * shine;
        out[c] = f32(lit * a);
    }
    out.w = f32(a);
    textureStore(output, id.xy, out);
}

// B-225, layer_fx::beam: k the line's stretch start (0, 1) and run (2, 3), its squared length, the
// start's share `a` and the length's `l`, the two thicknesses, the softness, 1 alone, then the
// inside and outside colours in linear light.
@compute @workgroup_size(16, 16)
fn beam(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let x = f64(id.x) + 0.5lf - k[0];
    let y = f64(id.y) + 0.5lf - k[1];
    var t = 0.0lf;
    if k[4] != 0.0lf {
        t = clamp((x * k[2] + y * k[3]) / k[4], 0.0lf, 1.0lf);
    }
    let ex = x - t * k[2];
    let ey = y - t * k[3];
    let d = sqrt(ex * ex + ey * ey);
    let r = (k[7] + (k[5] + t * k[6]) * (k[8] - k[7])) / 2.0lf;
    let sw = max(2.0lf * r * k[9] / 100.0lf, 1.0lf);
    let c = clamp((min(d + sw / 2.0lf, r) - max(d - sw / 2.0lf, -r)) / sw, 0.0lf, 1.0lf);
    var q = 1.0lf;
    if r != 0.0lf {
        q = min(d / r, 1.0lf);
    }
    let keep = select(1.0lf - c, 0.0lf, k[10] == 1.0lf);
    var out = px;
    for (var j = 0u; j < 3u; j++) {
        out[j] = f32(f64(px[j]) * keep + ((1.0lf - q) * k[11u + j] + q * k[14u + j]) * c);
    }
    out.w = f32(f64(px.w) * keep + c);
    textureStore(output, id.xy, out);
}

// B-225, grade::four_color_gradient: k the four points (0..8), the power 100 / blend, the opacity
// as a share, then the four colours encoded; laid on by F.blend.
@compute @workgroup_size(16, 16)
fn gradient4(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let a = f64(px.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    var d: array<f64, 4>;
    var m = 1e300lf;
    for (var j = 0u; j < 4u; j++) {
        let dx = x - k[2u * j];
        let dy = y - k[2u * j + 1u];
        d[j] = dx * dx + dy * dy;
        m = min(m, d[j]);
    }
    // ponytail: the power in single precision, its rounding far below a level of 255 once the
    // weights are summed and divided.
    var wk: array<f64, 4>;
    var sum = 0.0lf;
    for (var j = 0u; j < 4u; j++) {
        if m == 0.0lf {
            wk[j] = select(0.0lf, 1.0lf, d[j] == 0.0lf);
        } else {
            wk[j] = f64(pow(f32(m / d[j]), f32(k[8])));
        }
        sum += wk[j];
    }
    var out = px;
    for (var c = 0u; c < 3u; c++) {
        var acc = 0.0lf;
        for (var j = 0u; j < 4u; j++) {
            acc += wk[j] * k[10u + 3u * j + c];
        }
        let g = to_linear(acc / sum);
        let b = f64(px[c]) / a;
        out[c] = f32((b + k[9] * (mixed(b, g) - b)) * a);
    }
    textureStore(output, id.xy, out);
}

// B-225, layer_fx::least_covering's first half: each pixel's least covering along its row, F.r
// either side, into `row`; 0 where that reaches past the row's ends.
@compute @workgroup_size(16, 16)
fn sweepmin(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = i32(id.x);
    var m = 0.0;
    if x >= F.r && x < i32(size.x) - F.r {
        m = 3.4e38;
        for (var i = x - F.r; i <= x + F.r; i++) {
            m = min(m, textureLoad(input, vec2(i, i32(id.y)), 0).w);
        }
    }
    row[id.y * size.x + id.x] = m;
}

// B-225, layer_fx::light_sweep: k the centre (0, 1), the band's normal (2, 3), its half width, the
// sweep and edge intensities as shares, the shape (0 the band's hard edge, 1 linear, 2 smooth),
// the reception (0 add, 1 composite, 2 cutout), 1 when the edge is lit from `row` (F.r deep),
// then the light in linear light.
@compute @workgroup_size(16, 16)
fn sweep(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let a = f64(px.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    let d = abs(k[2] * (f64(id.x) + 0.5lf - k[0]) + k[3] * (f64(id.y) + 0.5lf - k[1]));
    let r = k[4];
    var p = 0.0lf;
    if r == 0.0lf {
        p = 0.0lf;
    } else if k[7] == 1.0lf {
        p = max(1.0lf - d / r, 0.0lf);
    } else if k[7] == 2.0lf {
        let t = d / r;
        if t < 1.0lf {
            p = 1.0lf - t * t * (3.0lf - 2.0lf * t);
        }
    } else {
        p = clamp(r + 0.5lf - d, 0.0lf, 1.0lf);
    }
    var edge = 0.0lf;
    if p > 0.0lf && k[9] == 1.0lf {
        let y = i32(id.y);
        var least = 0.0;
        if y >= F.r && y + F.r < i32(size.y) {
            least = 3.4e38;
            for (var v = y - F.r; v <= y + F.r; v++) {
                least = min(least, row[u32(v) * size.x + id.x]);
            }
        }
        edge = k[6] * (a - f64(least)) / a;
    }
    let l = p * (k[5] + edge);
    if l == 0.0lf && k[8] != 2.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    let m = min(l, 1.0lf);
    var out = px;
    for (var j = 0u; j < 3u; j++) {
        let v = f64(px[j]);
        if k[8] == 0.0lf {
            out[j] = f32(v + l * a * k[10u + j]);
        } else if k[8] == 1.0lf {
            out[j] = f32(v + m * (a * k[10u + j] - v));
        } else {
            out[j] = f32(m * a * k[10u + j]);
        }
    }
    if k[8] == 2.0lf {
        out.w = f32(m * a);
    }
    textureStore(output, id.xy, out);
}

// B-225, D-345, layer_fx::radio_waves: k the corner step in degrees (0 the sides), the apothem's share,
// the profile (0 square, 1 triangle, 2 sine), the colour in linear light, the waves' count, then
// from 8 each wave as `radio_wave_list` gives it.
@compute @workgroup_size(16, 16)
fn waves(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let ap = k[2];
    var t = 1.0lf;
    let n = u32(k[7]);
    for (var j = 0u; j < n; j++) {
        let o = 8u + 6u * j;
        let qx = x - k[o];
        let qy = y - k[o + 1u];
        let q = sqrt(qx * qx + qy * qy);
        let r = k[o + 2u];
        let h = k[o + 4u];
        if q < r * ap - h - 0.5lf || q * ap > r * ap + h + 0.5lf {
            continue;
        }
        let phi = atan2_64(qx, -qy) * 57.29577951308232lf;
        let delta = phi - k[o + 3u] - k[1] * (floor((phi - k[o + 3u]) / k[1]) + 0.5lf);
        let dd = abs(q * cos64(delta * 0.017453292519943295lf) - r * ap);
        var p = 0.0lf;
        if k[3] == 0.0lf {
            p = clamp(min(dd + 0.5lf, h) - max(dd - 0.5lf, -h), 0.0lf, 1.0lf);
        } else if k[3] == 1.0lf {
            p = max(1.0lf - dd / h, 0.0lf);
        } else if dd < h {
            p = cos64(90.0lf * dd / h * 0.017453292519943295lf);
        }
        t *= 1.0lf - k[o + 5u] * p;
    }
    if t == 1.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    var out = px;
    for (var j = 0u; j < 3u; j++) {
        out[j] = f32(t * f64(px[j]) + (1.0lf - t) * k[4u + j]);
    }
    out.w = f32(t * f64(px.w) + 1.0lf - t);
    textureStore(output, id.xy, out);
}


// layer_fx::bolt_light's soft core: the fall from 0 to u, odd in u.
fn rise(u: f64, half: f64) -> f64 {
    let a = min(abs(u), half);
    return select(-1.0lf, 1.0lf, u >= 0.0lf) * (a - a * a / (2.0lf * half));
}

// B-225, layer_fx::lightning_bolt: k the width, the glow, 1 soft, the opacity as a share, the core
// and glow colours in linear light, 1 to clear the layer first (Composite on Original off), the
// segments' count, then from 12 each segment as `bolt_list` gives it and its box.
@compute @workgroup_size(16, 16)
fn bolt(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var px = textureLoad(input, id.xy, 0);
    if k[10] == 1.0lf {
        px = vec4(0.0);
    }
    let xi = f64(id.x);
    let x = xi + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    var c = 0.0lf;
    var g = 0.0lf;
    let n = u32(k[11]);
    for (var j = 0u; j < n; j++) {
        let o = 12u + 10u * j;
        if y <= k[o + 8u] || y >= k[o + 9u] || xi < floor(k[o + 6u] - 0.5lf) || xi > ceil(k[o + 7u] - 0.5lf) {
            continue;
        }
        let sx = k[o];
        let sy = k[o + 1u];
        let dx = k[o + 2u] - sx;
        let dy = k[o + 3u] - sy;
        let l2 = dx * dx + dy * dy;
        var t = 0.0lf;
        if l2 != 0.0lf {
            t = clamp(((x - sx) * dx + (y - sy) * dy) / l2, 0.0lf, 1.0lf);
        }
        let ex = x - sx - t * dx;
        let ey = y - sy - t * dy;
        let d = sqrt(ex * ex + ey * ey);
        let w = k[o + 4u] + t * (k[o + 5u] - k[o + 4u]);
        let half = w * k[0] / 2.0lf;
        var core = 0.0lf;
        if k[2] == 1.0lf {
            if half > 0.0lf {
                core = rise(d + 0.5lf, half) - rise(d - 0.5lf, half);
            }
        } else {
            core = clamp(min(d + 0.5lf, half) - max(d - 0.5lf, -half), 0.0lf, 1.0lf);
        }
        let r = w * k[1];
        var lit = 0.0lf;
        if d < r {
            let u = 1.0lf - d / r;
            lit = w * (u * u);
        }
        c = max(c, core);
        g = max(g, lit);
    }
    if c == 0.0lf && g == 0.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    var out = px;
    for (var j = 0u; j < 3u; j++) {
        out[j] = f32(f64(px[j]) + k[3] * (c * k[4u + j] + (1.0lf - c) * g * k[7u + j]));
    }
    out.w = f32(f64(px.w) + k[3] * (c + (1.0lf - c) * g) * (1.0lf - f64(px.w)));
    textureStore(output, id.xy, out);
}

// B-235 (D-356), along::path_stroke: k the brush size, hardness and opacity, the colour in linear
// light, the paint style (0 on the original, 1 on transparent, 2 reveal), the runs' count, then
// from 8 each run as `along::stroke_runs` gives it and its box, then for each band of 16 rows
// where its list of runs starts and how many, then the lists. A pixel looks only at its band's
// runs; the nearest of them is the nearest of all, since the others' boxes miss the band. The
// nearest dab of a run is the one nearest along it, as `along::distance` finds it.
@compute @workgroup_size(16, 16)
fn stroke(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let xi = f64(id.x);
    let x = xi + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let r = k[0] / 2.0lf;
    let reach = r + 0.5lf;
    var d = reach;
    let band = 8u + 9u * u32(k[7]) + 2u * (id.y / 16u);
    let first = u32(k[band]);
    let n = u32(k[band + 1u]);
    for (var i = 0u; i < n; i++) {
        let o = 8u + 9u * u32(k[first + i]);
        if y <= k[o + 7u] || y >= k[o + 8u] || xi < ceil(k[o + 5u] - 0.5lf) || xi > floor(k[o + 6u] - 0.5lf) {
            continue;
        }
        let sx = k[o];
        let sy = k[o + 1u];
        let dx = k[o + 2u] - sx;
        let dy = k[o + 3u] - sy;
        let l2 = dx * dx + dy * dy;
        var t = 0.0lf;
        if l2 != 0.0lf {
            t = clamp(((x - sx) * dx + (y - sy) * dy) / l2, 0.0lf, 1.0lf);
        }
        let steps = k[o + 4u];
        if steps > 0.0lf {
            t = floor(t * steps + 0.5lf) / steps;
        }
        let ex = x - sx - t * dx;
        let ey = y - sy - t * dy;
        d = min(d, sqrt(ex * ex + ey * ey));
    }
    let style = k[6];
    if style == 0.0lf && d >= reach {
        textureStore(output, id.xy, px);
        return;
    }
    let soft = max(r * (1.0lf - k[1] / 100.0lf), 1.0lf);
    let u = clamp((reach - d) / soft, 0.0lf, 1.0lf);
    let c = k[2] / 100.0lf * u * u * (3.0lf - 2.0lf * u);
    var out = px;
    if style == 0.0lf {
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(f64(px[j]) * (1.0lf - c) + k[3u + j] * c);
        }
        out.w = f32(f64(px.w) * (1.0lf - c) + c);
    } else if style == 1.0lf {
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(k[3u + j] * c);
        }
        out.w = f32(c);
    } else {
        for (var j = 0u; j < 4u; j++) {
            out[j] = f32(f64(px[j]) * c);
        }
    }
    textureStore(output, id.xy, out);
}

// D-444, along::vegas: k half the width, the softness, the start, mid-point and end opacities and the
// mid-point's place (effects::vegas_fade), the blend mode (0 over, 1 under, 2 transparent, 3
// stencil), the colour in linear light, the bits' count, then from 11 each bit as
// along::vegas_bits gives it and its box, then for each band of 16 rows where its list of bits
// starts and how many, then the lists. A pixel looks only at its band's bits; the others' boxes
// miss the band. Its own entry point, as Path Stroke's is.
@compute @workgroup_size(16, 16)
fn vegas(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let count = u32(k[10]);
    let xi = f64(id.x);
    var c = 0.0lf;
    let band = 11u + 10u * count + 2u * (id.y / 16u);
    let first = u32(k[band]);
    let n = u32(k[band + 1u]);
    for (var i = 0u; i < n; i++) {
        let o = 11u + 10u * u32(k[first + i]);
        if y <= k[o + 8u] || y >= k[o + 9u] || xi < ceil(k[o + 6u] - 0.5lf) || xi > floor(k[o + 7u] - 0.5lf) {
            continue;
        }
        let bx = k[o];
        let by = k[o + 1u];
        let dx = k[o + 2u] - bx;
        let dy = k[o + 3u] - by;
        let l2 = dx * dx + dy * dy;
        var t = 0.0lf;
        if l2 != 0.0lf {
            t = clamp(((x - bx) * dx + (y - by) * dy) / l2, 0.0lf, 1.0lf);
        }
        let ex = x - bx - t * dx;
        let ey = y - by - t * dy;
        let v = clamp((k[0] + 0.5lf - sqrt(ex * ex + ey * ey)) / k[1], 0.0lf, 1.0lf);
        let s = k[o + 4u] + t * (k[o + 5u] - k[o + 4u]);
        var fade = k[3];
        if s < k[5] {
            fade = k[2] + (k[3] - k[2]) * s / k[5];
        } else if k[5] < 1.0lf {
            fade = k[3] + (k[4] - k[3]) * (s - k[5]) / (1.0lf - k[5]);
        }
        c = max(c, v * v * (3.0lf - 2.0lf * v) * fade);
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    var out = p;
    if k[6] == 0.0lf {
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(f64(p[j]) * (1.0lf - c) + k[7u + j] * c);
        }
        out.w = f32(a * (1.0lf - c) + c);
    } else if k[6] == 1.0lf {
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(f64(p[j]) + (1.0lf - a) * k[7u + j] * c);
        }
        out.w = f32(a + (1.0lf - a) * c);
    } else if k[6] == 2.0lf {
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(k[7u + j] * c);
        }
        out.w = f32(c);
    } else {
        for (var j = 0u; j < 4u; j++) {
            out[j] = f32(f64(p[j]) * c);
        }
    }
    textureStore(output, id.xy, out);
}

// D-446, grade::value_slope: the smooth value noise for the seed in `base` and its slope across and
// down, over all eight corners. Audit P2 (after B-157): the corner and the point's place in it
// found in double, the rest in single precision; the noise only steers the flow, by far less than
// a pixel's rounding.
fn value_slope(ch: u32, p: vec3<f64>) -> vec3<f32> {
    let c = floor(p);
    let t = vec3<f32>(p - c);
    let s = vec3(fade32(t.x), fade32(t.y), fade32(t.z));
    let d = vec2(30.0 * t.x * t.x * (1.0 - t.x) * (1.0 - t.x), 30.0 * t.y * t.y * (1.0 - t.y) * (1.0 - t.y));
    let i = vec3<i32>(c);
    var out = vec3(0.0);
    for (var corner = 0u; corner < 8u; corner++) {
        let e = vec3(corner & 1u, (corner >> 1u) & 1u, corner >> 2u);
        let w = select(vec3(1.0) - s, s, e == vec3(1u));
        let n = hashed32(i.x + i32(e.x), i.y + i32(e.y), i.z + i32(e.z), ch);
        out.x += w.x * w.y * w.z * n;
        out.y += select(-d.x, d.x, e.x == 1u) * w.y * w.z * n;
        out.z += w.x * select(-d.y, d.y, e.y == 1u) * w.z * n;
    }
    return out;
}

// D-583, grade::CurlPlane::get: the plane `input`'s alpha at pixel (i, j), 0 outside, or with
// k[15] 1 mirrored back in (no `%` on a negative).
fn curl_plane_at(i: i32, j: i32) -> f64 {
    let size = vec2<i32>(textureDimensions(input));
    var p = vec2(i, j);
    if k[15] == 1.0lf {
        let n = 2 * size;
        p = p - n * (p / n);
        p = select(p, p + n, p < vec2(0));
        p = select(p, n - vec2(1) - p, p >= size);
    } else if any(p < vec2(0)) || any(p >= size) {
        return 0.0lf;
    }
    return f64(textureLoad(input, p, 0).w);
}

// D-583, grade::CurlPlane::read: the plane between pixels, pixel centres at halves.
fn curl_plane(x: f64, y: f64) -> f64 {
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    let i = floor(fx);
    let j = floor(fy);
    let ux = fx - i;
    let uy = fy - j;
    let a = vec2<i32>(i32(i), i32(j));
    let top = (1.0lf - ux) * curl_plane_at(a.x, a.y) + ux * curl_plane_at(a.x + 1, a.y);
    let bottom = (1.0lf - ux) * curl_plane_at(a.x, a.y + 1) + ux * curl_plane_at(a.x + 1, a.y + 1);
    return (1.0lf - uy) * top + uy * bottom;
}

// D-583, effects::held_blur exactly as the CPU sums the plane `input`'s alpha, along k[0] (0
// across, 1 down): k[1] the radius, k[2..] the taps, edges held. Each product is rounded to
// single precision on its own (through double, where it is exact), so the card cannot fuse it
// into the add and the plane comes out the CPU's to the bit, which the flow lines need.
@compute @workgroup_size(16, 16)
fn curlsoft(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let r = i32(k[1]);
    let step = select(vec2(1, 0), vec2(0, 1), k[0] == 1.0lf);
    let last = vec2<i32>(textureDimensions(input)) - vec2(1);
    var o = 0.0;
    for (var t = 0; t <= 2 * r; t++) {
        let v = textureLoad(input, clamp(vec2<i32>(id.xy) + step * (t - r), vec2(0), last), 0).w;
        o += f32(f64(v) * k[2u + u32(t)]);
    }
    textureStore(output, id.xy, vec4(0.0, 0.0, 0.0, o));
}

// D-446, grade::Curl::at over the field's buffer, the drawing's size and F.g round it: k the
// field's size, its slide across and down, its depth, the four octaves' weights, the across and
// down bias, the share of blocks in the seed and the swirl; F.base the seed nought's mix. D-583:
// k[12] 1 reads the noise from the plane `input` (grade::CurlPlane), the drawing's corner at
// k[13], k[14] on it, k[15] 1 to mirror at its edges. D-646: k[16] 1 turns the noise round
// (k[20], k[21]) by cosine k[17], sine k[18] and 100 / Scale k[19]; k[22] 1 slides each finer
// octave half as far (grade::Curl::point).
fn curl_point(px: f64, py: f64, o: u32) -> vec2<f64> {
    var w = 1.0lf;
    if k[22] == 1.0lf {
        w = 1.0lf / f64(1u << o);
    }
    let x = px - w * k[1];
    let y = py - w * k[2];
    if k[16] != 1.0lf {
        return vec2(x, y);
    }
    let dx = x - k[20];
    let dy = y - k[21];
    return vec2(k[20] + (dx * k[17] + dy * k[18]) * k[19], k[21] + (-dx * k[18] + dy * k[17]) * k[19]);
}

@compute @workgroup_size(16, 16)
fn curlfield(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = f64(i32(id.x) - F.g - F.ox) + 0.5lf;
    let py = f64(i32(id.y) - F.g - F.oy) + 0.5lf;
    let x = px - k[1];
    let y = py - k[2];
    let s = k[0];
    var n = 0.0lf;
    var gx = 0.0lf;
    var gy = 0.0lf;
    if k[12] == 1.0lf {
        let bx = x + k[13];
        let by = y + k[14];
        gx = curl_plane(bx + 1.0lf, by) - curl_plane(bx - 1.0lf, by);
        gy = curl_plane(bx, by + 1.0lf) - curl_plane(bx, by - 1.0lf);
        if sqrt(gx * gx + gy * gy) < 1e-5lf {
            gx = 0.0lf;
            gy = 0.0lf;
        }
        n = 2.0lf * curl_plane(bx, by) - 1.0lf;
    } else {
        var total = 0.0lf;
        var fine = 1.0lf;
        for (var o = 0u; o < 4u; o++) {
            let amp = k[4u + o];
            let q = curl_point(px, py, o);
            let v = vec3<f64>(value_slope(8u * o, vec3(q.x / s * fine, q.y / s * fine, k[3] * fine)));
            n += amp * v.x;
            gx += amp * fine * v.y;
            gy += amp * fine * v.z;
            total += amp;
            fine *= 2.0lf;
        }
        n = n / total;
        gx = gx / total / s;
        gy = gy / total / s;
        if k[16] == 1.0lf {
            let tx = (k[17] * gx - k[18] * gy) * k[19];
            gy = (k[18] * gx + k[17] * gy) * k[19];
            gx = tx;
        }
    }
    let t = (-90.0lf + k[11] * n) * 0.017453292519943295lf;
    let c = cos64(t);
    let sn = sin64(t);
    let vx = (c * gx - sn * gy) * k[8];
    let vy = (sn * gx + c * gy) * k[9];
    // D-680: at most one long, shorter where the slope falls under one per noise size.
    let length = max(sqrt(vx * vx + vy * vy), 1.0lf / s);
    let d = vec2(vx / length, vy / length);
    let cell = s / 50.0lf;
    let at = curl_point(px, py, 0u);
    let q = vec3(at.x / cell, at.y / cell, k[3]);
    let e = k[10];
    var seed = 0.0lf;
    if e == 1.0lf {
        seed = cell_noise_look(7u, q, true, 0);
    } else if e == 0.0lf {
        seed = cell_noise_look(7u, q, false, 0);
    } else {
        seed = e * cell_noise_look(7u, q, true, 0) + (1.0lf - e) * cell_noise_look(7u, q, false, 0);
    }
    textureStore(output, id.xy, vec4(f32(d.x), f32(d.y), f32(n), f32(seed)));
}

// D-583: render::sample_bilinear exactly as the CPU sums it: each product rounded to single
// precision on its own (through double, where a single product is exact), so the card cannot
// fuse it into the add. Kept for a picture's plane (Audit P2), whose flow lines part on a hair.
fn curl_sample(t: texture_2d<f32>, x: f64, y: f64) -> vec4<f32> {
    let size = vec2<i32>(textureDimensions(t));
    let fx = x - 0.5lf;
    let fy = y - 0.5lf;
    if !(fx > -1.0lf && fy > -1.0lf && fx < f64(size.x) && fy < f64(size.y)) {
        return vec4(0.0);
    }
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;
    var out = vec4(0.0);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0lf - uy, uy, j == 1);
        let sy = i32(y0) + j;
        if wy == 0.0lf || sy < 0 || sy >= size.y {
            continue;
        }
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0lf - ux, ux, i == 1);
            let sx = i32(x0) + i;
            if wx == 0.0lf || sx < 0 || sx >= size.x {
                continue;
            }
            let wt = f64(f32(wx * wy));
            let v = textureLoad(t, vec2(sx, sy), 0);
            out += vec4(f32(f64(v.x) * wt), f32(f64(v.y) * wt), f32(f64(v.z) * wt), f32(f64(v.w) * wt));
        }
    }
    return out;
}

// D-583: render::sample_bilinear over the field at the start pixel `base`'s centre plus `r`.
// Audit P2 (after B-157): the place found from the start pixel, so the numbers stay small, and
// the sum in single precision.
fn curl_sample32(t: texture_2d<f32>, base: vec2<i32>, r: vec2<f32>) -> vec4<f32> {
    let size = vec2<i32>(textureDimensions(t));
    let c = floor(r);
    let u = r - c;
    let o = base + vec2<i32>(c);
    var out = vec4(0.0);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0 - u.y, u.y, j == 1);
        let sy = o.y + j;
        if wy == 0.0 || sy < 0 || sy >= size.y {
            continue;
        }
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0 - u.x, u.x, i == 1);
            let sx = o.x + i;
            if wx == 0.0 || sx < 0 || sx >= size.x {
                continue;
            }
            out += textureLoad(t, vec2(sx, sy), 0) * (wx * wy);
        }
    }
    return out;
}

// D-446, grade::curl_noise after the field (`other`, F.g round the drawing): k the sample radius,
// the steps, Flow Softness and Flow Falloff out of 1, the view (0 Final Render, 1 Input Noise, 2
// Curl Generation), Contrast, Brightness, 1 to hold at white, the channel (0 RGB, 1 to 3 red to
// blue, 4 alpha), 1 where the field is a picture's plane (D-583).
@compute @workgroup_size(16, 16)
fn curlflow(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let a = f64(p.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    let m = u32(F.g);
    let fp = textureLoad(other, id.xy + vec2(m), 0);
    let dx = f64(fp.x);
    let dy = f64(fp.y);
    let n = f64(fp.z);
    var o = vec3(0.5lf + 0.5lf * n);
    let view = u32(k[4]);
    if view == 2u {
        let length = sqrt(dx * dx + dy * dy);
        var d = vec2(0.0lf);
        if length > 0.0lf {
            d = vec2(dx / length, dy / length);
        }
        o = vec3(0.5lf + 0.5lf * d.x, 0.5lf + 0.5lf * d.y, 0.5lf + 0.5lf * n);
    } else if view == 0u {
        let radius = k[0];
        let steps = u32(k[1]);
        let h = radius / k[1];
        let subs = max(ceil(h), 1.0lf);
        var sw = 1.0lf;
        var sws = 1.0lf;
        // Audit P2 (after B-157): each step's length and stride stay double; for the noise, the
        // samples between, their weights and sums are single precision, read from the start
        // pixel. A picture's plane (k[9] 1) keeps the CPU's reads to the bit, as D-583.
        let exact = k[9] == 1.0lf;
        let base = vec2<i32>(id.xy + vec2(m));
        let start = vec2<f64>(base) + 0.5lf;
        let subs32 = f32(subs);
        let last32 = f32(k[1] + 1.0lf);
        var ss = f64(fp.w);
        var tw = 1.0lf;
        var ss32 = 0.0;
        var tw32 = 0.0;
        var walked = 0.0lf;
        for (var way = 0u; way < 2u; way++) {
            let sign = select(-1.0lf, 1.0lf, way == 0u);
            var qx = start.x;
            var qy = start.y;
            var ex = dx;
            var ey = dy;
            for (var i = 1u; i <= steps; i++) {
                let length = sqrt(ex * ex + ey * ey);
                if length < 1e-6lf {
                    break;
                }
                walked += h * length;
                let mx = sign * h * ex;
                let my = sign * h * ey;
                let r = vec2<f32>(vec2(qx, qy) - start);
                let r1 = vec2<f32>(vec2(mx, my));
                for (var j = 1u; j <= u32(subs); j++) {
                    var q: vec4<f32>;
                    if exact {
                        let f = f64(j) / subs;
                        q = curl_sample(other, qx + f * mx, qy + f * my);
                        let wj = 1.0lf - (f64(i - 1u) + f) / (k[1] + 1.0lf);
                        ss += wj * f64(q.w);
                        tw += wj;
                    } else {
                        let f = f32(j) / subs32;
                        q = curl_sample32(other, base, r + f * r1);
                        let wj = 1.0 - (f32(i - 1u) + f) / last32;
                        ss32 += wj * q.w;
                        tw32 += wj;
                    }
                    if j == u32(subs) {
                        ex = f64(q.x);
                        ey = f64(q.y);
                    }
                }
                qx += mx;
                qy += my;
                let wt = 1.0lf - f64(i) / (k[1] + 1.0lf);
                sw += wt;
                sws += wt * wt;
            }
        }
        let gain = 1.0lf + (sw / sqrt(sws) - 1.0lf) * min(walked / max(2.0lf * radius, 1e-9lf), 1.0lf);
        let lines = (ss + f64(ss32)) / (tw + f64(tw32)) * gain;
        let mixed = n + (1.0lf - k[2]) * min(1.0lf, radius / 8.0lf) * (lines - n);
        o = vec3(0.5lf + 0.5lf * (mixed * (1.0lf - k[3] * (0.5lf - 0.5lf * n))));
    }
    for (var j = 0u; j < 3u; j++) {
        let v = 0.5lf + (o[j] - 0.5lf) * k[5] / 100.0lf + k[6] / 100.0lf;
        o[j] = select(max(v, 0.0lf), clamp(v, 0.0lf, 1.0lf), k[7] == 1.0lf);
    }
    var out = p;
    let channel = u32(k[8]);
    if channel == 0u {
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(to_linear(o[j]) * a);
        }
    } else if channel == 4u {
        let t = a * clamp((o.x + o.y + o.z) / 3.0lf, 0.0lf, 1.0lf);
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(f64(p[j]) / a * t);
        }
        out.w = f32(t);
    } else {
        out[channel - 1u] = f32(to_linear(o[channel - 1u]) * a);
    }
    textureStore(output, id.xy, out);
}

// D-448, median::cartoon_k's numbers in k; median::cartoon's edges: each pixel's edge from Sobel
// on the smoothed picture luma, the border repeated, into red.
@compute @workgroup_size(16, 16)
fn cartoonedge(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let most = vec2<i32>(size) - vec2(1);
    let q = vec2<i32>(id.xy);
    var l: array<f64, 9>;
    for (var j = 0; j < 3; j++) {
        for (var i = 0; i < 3; i++) {
            l[j * 3 + i] = picture_luma(textureLoad(input, clamp(q + vec2(i - 1, j - 1), vec2(0), most), 0));
        }
    }
    let gx = l[2] + 2.0lf * l[5] + l[8] - l[0] - 2.0lf * l[3] - l[6];
    let gy = l[6] + 2.0lf * l[7] + l[8] - l[0] - 2.0lf * l[1] - l[2];
    let e = clamp((10.0lf * sqrt(gx * gx + gy * gy) / 2.0lf - k[0]) / k[1], 0.0lf, 1.0lf);
    textureStore(output, id.xy, vec4(f32(e), 0.0, 0.0, 0.0));
}

// D-448, median::cartoon_fill: a smoothed pixel's fill, encoded, with its covering in w (all 0
// where it does not show).
fn cartoon_fill(p: vec4<f32>) -> vec4<f64> {
    let a = f64(p.w);
    if a <= 0.0lf {
        return vec4(0.0lf);
    }
    let c = vec3(f64(p.x) / a, f64(p.y) / a, f64(p.z) / a);
    let l = to_srgb(clamp(0.2126lf * c.x + 0.7152lf * c.y + 0.0722lf * c.z, 0.0lf, 1.0lf));
    let q = l * k[5];
    let n = floor(q);
    let t = q - n;
    var ramp = select(0.0lf, 1.0lf, t >= 0.5lf);
    if k[6] != 0.0lf {
        ramp = clamp((t - 0.5lf) / k[6] + 0.5lf, 0.0lf, 1.0lf);
    }
    let shade = (n + ramp) / k[5];
    var f = vec4(0.0lf, 0.0lf, 0.0lf, a);
    for (var i = 0u; i < 3u; i++) {
        f[i] = clamp(to_srgb(clamp(c[i], 0.0lf, 1.0lf)) + shade - l, 0.0lf, 1.0lf);
    }
    return f;
}

// D-448, median::cartoon: `input` the smoothed layer, `other` its edges (cartoonedge).
@compute @workgroup_size(16, 16)
fn cartoon(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var f = cartoon_fill(textureLoad(input, id.xy, 0));
    let a = f.w;
    if a <= 0.0lf {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let most = vec2<i32>(size) - vec2(1);
    let q = vec2<i32>(id.xy);
    if k[7] != 0.0lf {
        var m = vec3(0.0lf);
        var count = 0.0lf;
        for (var dy = -1; dy <= 1; dy++) {
            for (var dx = -1; dx <= 1; dx++) {
                let g = cartoon_fill(textureLoad(input, clamp(q + vec2(dx, dy), vec2(0), most), 0));
                if g.w > 0.0lf {
                    m += g.xyz;
                    count += 1.0lf;
                }
            }
        }
        let e = f64(textureLoad(other, id.xy, 0).x);
        for (var c = 0u; c < 3u; c++) {
            f[c] = clamp(f[c] + k[7] * e * (f[c] - m[c] / count), 0.0lf, 1.0lf);
        }
    }
    let r = i32(k[10]);
    var ink = 0.0lf;
    for (var ty = max(q.y - r, 0); ty <= min(q.y + r, most.y); ty++) {
        for (var tx = max(q.x - r, 0); tx <= min(q.x + r, most.x); tx++) {
            let dx = tx - q.x;
            let dy = ty - q.y;
            let wt = clamp((k[2] + 0.5lf - sqrt(f64(dx * dx + dy * dy))) / (2.0lf * k[3]) + 0.5lf, 0.0lf, 1.0lf);
            ink = max(ink, f64(textureLoad(other, vec2(tx, ty), 0).x) * wt);
        }
    }
    ink *= k[4];
    let b = k[8];
    var c = f.xyz;
    if k[9] == 1.0lf {
        c = vec3((1.0lf - b) * (1.0lf - ink) + b * ink);
    } else if k[9] == 2.0lf {
        c = f.xyz * (1.0lf - ink) + vec3(b * ink);
    }
    textureStore(output, id.xy, vec4(f32(to_linear(c.x) * a), f32(to_linear(c.y) * a), f32(to_linear(c.z) * a), f32(a)));
}

// D-447: grade::brush_strokes' number k of 0 to 5 for the cell whose hash so far is `g`.
fn brush_h(g: vec2<u32>, k: u32) -> f64 {
    let h = shr64(splitmix(g ^ vec2(k, 0u)), 11u);
    return (f64(h.y) * 4294967296.0lf + f64(h.x)) / 9007199254740992.0lf;
}

// D-447, grade::brush_strokes: k the stroke direction (two numbers), the cells' length and
// width, the stroke length, the brush size, min(randomness, 1), the largest slope and its sine,
// the reach, the surface (0 original, 1 transparent, 2 white, 3 black) and the blend out of 1;
// the seed's hash in `base`, the frame in `frame`, the drawing's corner in (ox, oy).
@compute @workgroup_size(16, 16)
fn brush(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let pf = textureLoad(input, id.xy, 0);
    let p = vec4(f64(pf.x), f64(pf.y), f64(pf.z), f64(pf.w));
    let d = vec2(k[0], k[1]);
    let n = vec2(-d.y, d.x);
    let su = k[2];
    let sv = k[3];
    let len0 = k[4];
    let r = k[5];
    let m = k[6];
    let slope = k[7];
    let s = k[8];
    let reach = k[9];
    let qx = f64(id.x) + 0.5lf - f64(F.ox);
    let qy = f64(id.y) + 0.5lf - f64(F.oy);
    let pu = qx * d.x + qy * d.y;
    let pv = qx * n.x + qy * n.y;
    var p0 = -1.0lf;
    var c0 = vec4(0.0lf);
    var v0 = 0.0lf;
    var p1 = -1.0lf;
    var c1 = vec4(0.0lf);
    var v1 = 0.0lf;
    let i0 = i32(floor((pu - len0 - reach) / su)) - 1;
    let i1 = i32(floor((pu + reach) / su));
    let j0 = i32(floor((pv - reach - len0 * s) / sv)) - 1;
    let j1 = i32(floor((pv + reach + len0 * s) / sv));
    let fw = wide(F.frame);
    for (var i = i0; i <= i1; i++) {
        let gi = splitmix(F.base ^ wide(i));
        for (var j = j0; j <= j1; j++) {
            let g = splitmix(splitmix(gi ^ wide(j)) ^ fw);
            let cu = (f64(i) + 0.5lf + (brush_h(g, 0u) - 0.5lf) * m) * su;
            let cv = (f64(j) + 0.5lf + (brush_h(g, 1u) - 0.5lf) * m) * sv;
            let t = (2.0lf * brush_h(g, 2u) - 1.0lf) * slope;
            let el = sqrt(1.0lf + t * t);
            let eu = 1.0lf / el;
            let ev = t / el;
            let len = len0 * (1.0lf - 0.5lf * m * brush_h(g, 3u));
            let rho = r * (1.0lf + 0.25lf * m * (2.0lf * brush_h(g, 4u) - 1.0lf));
            let wu = pu - cu;
            let wv = pv - cv;
            let a = min(max(wu * eu + wv * ev, 0.0lf), len);
            let du = wu - a * eu;
            let dv = wv - a * ev;
            let cov = clamp(rho + 0.5lf - sqrt(du * du + dv * dv), 0.0lf, 1.0lf);
            if cov <= 0.0lf {
                continue;
            }
            let pile = brush_h(g, 5u);
            if pile <= p1 {
                continue;
            }
            let sx = min(u32(max(floor(cu * d.x + cv * n.x + f64(F.ox)), 0.0lf)), size.x - 1u);
            let sy = min(u32(max(floor(cu * d.y + cv * n.y + f64(F.oy)), 0.0lf)), size.y - 1u);
            let cf = textureLoad(input, vec2(sx, sy), 0);
            let col = vec4(f64(cf.x), f64(cf.y), f64(cf.z), f64(cf.w));
            if pile > p0 {
                p1 = p0;
                c1 = c0;
                v1 = v0;
                p0 = pile;
                c0 = col;
                v0 = cov;
            } else {
                p1 = pile;
                c1 = col;
                v1 = cov;
            }
        }
    }
    var o = p;
    let surface = u32(k[10]);
    if surface == 1u {
        o = vec4(0.0lf);
    } else if surface == 2u {
        o = vec4(1.0lf);
    } else if surface == 3u {
        o = vec4(0.0lf, 0.0lf, 0.0lf, 1.0lf);
    }
    if p1 >= 0.0lf {
        o = c1 * v1 + o * (1.0lf - v1 * c1.w);
    }
    if p0 >= 0.0lf {
        o = c0 * v0 + o * (1.0lf - v0 * c0.w);
    }
    let out = o + k[11] * (p - o);
    textureStore(output, id.xy, vec4(f32(out.x), f32(out.y), f32(out.z), f32(out.w)));
}

// D-420, layer_fx::draw_marks: k half the thickness, the softness, how (0 over, 1 alone, 2 add),
// 1 to blend, the pieces' count, then from 5 each piece (its ends, its mark, its inside and
// outside colours in linear light) and its box, then for each band of 16 rows where its list of
// pieces starts and how many, then the lists. A pixel looks only at its band's pieces within
// their boxes; out of them a mark covers nothing.
@compute @workgroup_size(16, 16)
fn marks(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let x = f64(id.x) + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    let r = k[0];
    let sw = max(2.0lf * r * k[1] / 100.0lf, 1.0lf);
    let blend = k[3] != 0.0lf;
    let band = 5u + 15u * u32(k[4]) + 2u * (id.y / 16u);
    let first = u32(k[band]);
    let n = u32(k[band + 1u]);
    var acc = array<f64, 4>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    var total = array<f64, 3>(0.0lf, 0.0lf, 0.0lf);
    var weight = 0.0lf;
    var clear = 1.0lf;
    var best = 0u;
    var bd = 0.0lf;
    for (var i = 0u; i <= n; i++) {
        var o = 0u;
        var hit = false;
        if i < n {
            o = 5u + 15u * u32(k[first + i]);
            hit = x > k[o + 11u] && x < k[o + 12u] && y > k[o + 13u] && y < k[o + 14u];
        }
        if best != 0u && (i == n || (hit && k[o + 4u] != k[best + 4u])) {
            let c = clamp((min(bd + sw / 2.0lf, r) - max(bd - sw / 2.0lf, -r)) / sw, 0.0lf, 1.0lf);
            var q = 1.0lf;
            if r != 0.0lf {
                q = min(bd / r, 1.0lf);
            }
            for (var j = 0u; j < 3u; j++) {
                let l = (1.0lf - q) * k[best + 5u + j] + q * k[best + 8u + j];
                if blend {
                    total[j] += l * c;
                } else {
                    acc[j] = l * c + acc[j] * (1.0lf - c);
                }
            }
            if blend {
                weight += c;
                clear *= 1.0lf - c;
            } else {
                acc[3] = c + acc[3] * (1.0lf - c);
            }
            best = 0u;
        }
        if hit {
            let dx = k[o + 2u] - k[o];
            let dy = k[o + 3u] - k[o + 1u];
            let l2 = dx * dx + dy * dy;
            var t = 0.0lf;
            if l2 != 0.0lf {
                t = clamp(((x - k[o]) * dx + (y - k[o + 1u]) * dy) / l2, 0.0lf, 1.0lf);
            }
            let ex = x - k[o] - t * dx;
            let ey = y - k[o + 1u] - t * dy;
            let d = sqrt(ex * ex + ey * ey);
            if best == 0u || d < bd {
                best = o;
                bd = d;
            }
        }
    }
    if blend {
        acc[3] = 1.0lf - clear;
        for (var j = 0u; j < 3u; j++) {
            acc[j] = select(0.0lf, total[j] / weight * acc[3], weight > 0.0lf);
        }
    }
    var out = px;
    let how = k[2];
    for (var j = 0u; j < 4u; j++) {
        let p = f64(px[j]);
        if how == 1.0lf {
            out[j] = f32(acc[j]);
        } else if how == 2.0lf {
            if j == 3u {
                out[j] = f32(min(p + acc[j], 1.0lf));
            } else {
                out[j] = f32(p + acc[j]);
            }
        } else {
            out[j] = f32(acc[j] + p * (1.0lf - acc[3]));
        }
    }
    textureStore(output, id.xy, out);
}

// D-508, along::energy_stroke: k as along::energy_numbers (twelve), the bits' count, D-565 the
// shaped ends' count and from 14 each end (six), then each bit as `along::energy_bits` gives it
// and its box, then for each band of 16 rows where its
// list of bits starts and how many, then the lists, as Path Stroke's. The core and the glow are
// each the largest over the bits; the rings' exponentials are worked in single precision, which
// has no double exp. B-222: the pixel read and written one channel at a time.
@compute @workgroup_size(16, 16)
fn energy(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let xi = f64(id.x);
    let x = xi + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    var c = 0.0lf;
    var g = 0.0lf;
    let ends = u32(k[13]);
    let at = 14u + 6u * ends;
    let band = at + 10u * u32(k[12]) + 2u * (id.y / 16u);
    let first = u32(k[band]);
    let n = u32(k[band + 1u]);
    for (var i = 0u; i < n; i++) {
        let o = at + 10u * u32(k[first + i]);
        if y <= k[o + 8u] || y >= k[o + 9u] || xi < ceil(k[o + 6u] - 0.5lf) || xi > floor(k[o + 7u] - 0.5lf) {
            continue;
        }
        let sx = k[o];
        let sy = k[o + 1u];
        let dx = k[o + 2u] - sx;
        let dy = k[o + 3u] - sy;
        let l2 = dx * dx + dy * dy;
        var t = 0.0lf;
        if l2 != 0.0lf {
            t = clamp(((x - sx) * dx + (y - sy) * dy) / l2, 0.0lf, 1.0lf);
        }
        let ex = x - sx - t * dx;
        let ey = y - sy - t * dy;
        let d2 = ex * ex + ey * ey;
        let d = sqrt(d2);
        let w = k[o + 4u] + (k[o + 5u] - k[o + 4u]) * t;
        let r = k[0] * w / 2.0lf;
        if r > 0.0lf {
            let soft = max(r * k[1], 1.0lf);
            let u = clamp((r + 0.5lf - d) / soft, 0.0lf, 1.0lf);
            c = max(c, u * u * (3.0lf - 2.0lf * u));
        }
        if k[2] > 0.0lf {
            var s = 0.0lf;
            for (var j = 3u; j < 7u; j++) {
                s += f64(exp(f32(-d2 / (2.0lf * k[j] * k[j]))));
            }
            g = max(g, w * s / 4.0lf);
        }
    }
    // D-565: along::energy_ends.
    for (var e = 0u; e < ends; e++) {
        let q0 = 14u + 6u * e;
        let fx = x - k[q0];
        let fy = y - k[q0 + 1u];
        let along = fx * k[q0 + 2u] + fy * k[q0 + 3u];
        let r = k[q0 + 4u];
        let q = k[q0 + 5u];
        if along <= 0.0lf || sqrt(fx * fx + fy * fy) >= r + 0.5lf {
            continue;
        }
        var cap = 0.0lf;
        if q > 0.0lf {
            let across = fx * k[q0 + 3u] - fy * k[q0 + 2u];
            let ql = along / q;
            let u = clamp((r + 0.5lf - sqrt(across * across + ql * ql)) / max(r * k[1], 1.0lf), 0.0lf, 1.0lf);
            cap = u * u * (3.0lf - 2.0lf * u);
        }
        c = min(c, cap);
    }
    let mode = u32(k[11]);
    if mode == 0u && c == 0.0lf && g == 0.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    var light = vec3<f64>(0.0lf);
    for (var j = 0u; j < 3u; j++) {
        light[j] = k[10] * c + (1.0lf - c) * g * k[7u + j];
    }
    let a = min(max(max(light.x, light.y), light.z), 1.0lf);
    var out = vec4<f32>(f32(light.x), f32(light.y), f32(light.z), f32(a));
    if mode == 0u {
        out = vec4<f32>(f32(f64(px.x) + light.x), f32(f64(px.y) + light.y), f32(f64(px.z) + light.z), f32(f64(px.w) + (1.0lf - f64(px.w)) * a));
    } else if mode == 2u {
        out.w = 1.0;
    }
    textureStore(output, id.xy, out);
}

// D-441, along::write_on: k the paint style (0 on the original, 1 on transparent, 2 reveal), the
// colour in linear light, the marks' count, then from 5 each mark's centre, radius, soft edge,
// covering at most and box, then for each band of 16 rows where its list of marks starts and how
// many, then the lists, as Path Stroke's. The mark covering most wins.
@compute @workgroup_size(16, 16)
fn writeon(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let xi = f64(id.x);
    let x = xi + 0.5lf;
    let y = f64(id.y) + 0.5lf;
    var c = 0.0lf;
    let band = 5u + 9u * u32(k[4]) + 2u * (id.y / 16u);
    let first = u32(k[band]);
    let n = u32(k[band + 1u]);
    for (var i = 0u; i < n; i++) {
        let o = 5u + 9u * u32(k[first + i]);
        if y <= k[o + 7u] || y >= k[o + 8u] || xi < ceil(k[o + 5u] - 0.5lf) || xi > floor(k[o + 6u] - 0.5lf) {
            continue;
        }
        let ex = x - k[o];
        let ey = y - k[o + 1u];
        let u = clamp((k[o + 2u] + 0.5lf - sqrt(ex * ex + ey * ey)) / k[o + 3u], 0.0lf, 1.0lf);
        c = max(c, k[o + 4u] * u * u * (3.0lf - 2.0lf * u));
    }
    let style = k[0];
    if style == 0.0lf && c <= 0.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    var out = px;
    if style == 0.0lf {
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(f64(px[j]) * (1.0lf - c) + k[1u + j] * c);
        }
        out.w = f32(f64(px.w) * (1.0lf - c) + c);
    } else if style == 1.0lf {
        for (var j = 0u; j < 3u; j++) {
            out[j] = f32(k[1u + j] * c);
        }
        out.w = f32(c);
    } else {
        for (var j = 0u; j < 4u; j++) {
            out[j] = f32(f64(px[j]) * c);
        }
    }
    textureStore(output, id.xy, out);
}

// B-225, layer_fx::bevel_edges: k the thickness in pixels, the light's direction (1, 2), the
// intensity, then the light in linear light. A pixel nearer than the thickness to the buffer's
// nearest side, the first of left, top, right and bottom among equals, takes that side's slope.
@compute @workgroup_size(16, 16)
fn edges(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let px = textureLoad(input, id.xy, 0);
    let a = f64(px.w);
    if a <= 0.0lf {
        textureStore(output, id.xy, px);
        return;
    }
    let cx = f64(id.x) + 0.5lf;
    let cy = f64(id.y) + 0.5lf;
    var near = cx;
    var s = -k[1];
    if cy < near {
        near = cy;
        s = -k[2];
    }
    if f64(size.x) - cx < near {
        near = f64(size.x) - cx;
        s = k[1];
    }
    if f64(size.y) - cy < near {
        near = f64(size.y) - cy;
        s = k[2];
    }
    if !(near < k[0]) {
        s = 0.0lf;
    }
    var out = px;
    for (var c = 0u; c < 3u; c++) {
        let p = f64(px[c]);
        if s > 0.0lf {
            out[c] = f32(p + (k[4u + c] * a - p) * k[3] * s);
        } else {
            out[c] = f32(p * (1.0lf + k[3] * s));
        }
    }
    textureStore(output, id.xy, out);
}

// B-226, layer_fx::block_dissolve: k[0] 1 with a feather. Without one, k[1] the blocks across,
// then each column's block (from 2), each row's (from 2 + width) and whether each block is kept
// (from 2 + width + height), all worked out by the CPU. With one, k[1] the length of each
// column's running sums, the first row of blocks, the blocks' height, half the feather, the
// feather squared and the corner's row, then from 7 each column's running sums of the area kept
// across each row of blocks (layer_fx::dissolve_down).
fn dissolve_upto(c: u32, n: u32, t: f64) -> f64 {
    let s = t / k[3] - k[2];
    let a = u32(min(max(floor(s), 0.0lf), f64(n - 2u)));
    return (k[c + a] + (s - f64(a)) * (k[c + a + 1u] - k[c + a])) * k[3];
}

@compute @workgroup_size(16, 16)
fn dissolve(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var kept: f64;
    if k[0] == 0.0lf {
        let ix = u32(k[2u + id.x]);
        let jy = u32(k[2u + size.x + id.y]);
        kept = k[2u + size.x + size.y + jy * u32(k[1]) + ix];
    } else {
        let n = u32(k[1]);
        let cy = f64(id.y) - k[6] + 0.5lf;
        let c = 7u + id.x * n;
        kept = clamp((dissolve_upto(c, n, cy + k[4]) - dissolve_upto(c, n, cy - k[4])) / k[5], 0.0lf, 1.0lf);
    }
    textureStore(output, id.xy, textureLoad(input, id.xy, 0) * f32(kept));
}

// D-512, layer_fx::wipe_reading: the reading `kind` (VECTOR_BLUR_PROPERTIES' order) of `q`.
fn wipe_reading(kind: u32, q: vec4<f32>) -> f64 {
    if kind == 3u {
        return f64(q.w);
    }
    if kind == 4u {
        return picture_luma(q);
    }
    let c = vec3(
        to_srgb(clamp(f64(q.x), 0.0lf, 1.0lf)),
        to_srgb(clamp(f64(q.y), 0.0lf, 1.0lf)),
        to_srgb(clamp(f64(q.z), 0.0lf, 1.0lf)),
    );
    switch kind {
        case 0u: { return c.x; }
        case 1u: { return c.y; }
        case 2u: { return c.z; }
        case 5u: { return hsl(c).z; }
        case 6u: { return hsl(c).x / 360.0lf; }
        default: { return hsl(c).y; }
    }
}

// B-226, layer_fx::gradient_wipe: the map (`other`) lying on the drawing at (F.ox, F.oy); k the
// edge, the softness as a share, invert, and 1 to clear the drawing. D-512: k[4] the reading's
// kind (Gradient Wipe's 4, luminance); k[5] 0 to read the map, 2 to store the reading as the
// output's alpha (to be blurred), 1 to read that blurred reading, `other`'s alpha.
@compute @workgroup_size(16, 16)
fn gwipe(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    if k[3] == 1.0lf {
        textureStore(output, id.xy, vec4(0.0));
        return;
    }
    let q = at(other, vec2<i32>(id.xy) - vec2(F.ox, F.oy));
    var v = f64(q.w);
    if k[5] != 1.0lf {
        v = wipe_reading(u32(k[4]), q);
    }
    if k[5] == 2.0lf {
        textureStore(output, id.xy, vec4(0.0, 0.0, 0.0, f32(v)));
        return;
    }
    if k[2] == 1.0lf {
        v = 1.0lf - v;
    }
    var kept = select(0.0lf, 1.0lf, v >= k[0]);
    if k[1] > 0.0lf {
        kept = clamp((v - k[0]) / k[1] + 0.5lf, 0.0lf, 1.0lf);
    }
    var out = p;
    for (var c = 0u; c < 4u; c++) {
        out[c] = f32(f64(p[c]) * kept);
    }
    textureStore(output, id.xy, out);
}

// D-403, layer_fx::aerial_haze: k[0] the amount as a share, k[1] to k[3] the haze colour in linear
// light, k[4] 1 with a matte (`other`, lying on the drawing at (F.ox, F.oy)) and 0 without.
@compute @workgroup_size(16, 16)
fn haze(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    var v = 1.0lf;
    if k[4] == 1.0lf {
        v = picture_luma(at(other, vec2<i32>(id.xy) - vec2(F.ox, F.oy)));
    }
    let m = k[0] * v;
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let q = f64(p[c]);
        out[c] = f32(q + m * (k[1u + c] * f64(p.w) - q));
    }
    textureStore(output, id.xy, out);
}

// D-524, layer_fx::fog_3d: k[0] to k[2] the fog colour in linear light; `other` the shares, F
// then B, lying on the drawing at (F.ox, F.oy), transparent outside it.
@compute @workgroup_size(16, 16)
fn fog(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let s = at(other, vec2<i32>(id.xy) - vec2(F.ox, F.oy));
    let a = f64(p.w);
    let f = f64(s.x);
    let b = f64(s.y);
    var out = p;
    for (var c = 0u; c < 3u; c++) {
        let q = f64(p[c]);
        out[c] = f32(q + f * (k[c] * a - q) + (1.0lf - a) * b * k[c]);
    }
    out.w = f32(a + (1.0lf - a) * b);
    textureStore(output, id.xy, out);
}

// D-501, layer_fx::difference_matte: a pixel as straight encoded colour and its alpha.
fn dm_straight(p: vec4<f32>) -> vec4<f64> {
    let a = f64(p.w);
    if a <= 0.0lf {
        return vec4<f64>(0.0lf);
    }
    let c = clamp(vec3<f64>(f64(p.x), f64(p.y), f64(p.z)) / a, vec3(0.0lf), vec3(1.0lf));
    return vec4<f64>(to_srgb(c.x), to_srgb(c.y), to_srgb(c.z), min(a, 1.0lf));
}

// D-501, layer_fx::difference_matte: k the tolerance, the softness, and 1 for the matte view.
// Mode 0 writes the share kept to `output` from the (blurred) drawing `input`, read F.r in, and
// the (blurred) plate `other` lying at (F.ox, F.oy); mode 1 does the same unblurred and applies
// it to `input`; mode 2 applies the shares `other` to `input`.
@compute @workgroup_size(16, 16)
fn dmatte(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let q = vec2<i32>(id.xy);
    var kept: f64;
    if F.mode == 2u {
        kept = f64(textureLoad(other, id.xy, 0).x);
    } else {
        let d4 = abs(dm_straight(at(input, q + vec2(F.r))) - dm_straight(at(other, q - vec2(F.ox, F.oy))));
        let d = 100.0lf * max(max(d4.x, d4.y), max(d4.z, d4.w));
        kept = select(1.0lf, 0.0lf, d <= k[0]);
        if k[1] > 0.0lf {
            kept = clamp((d - k[0]) / k[1], 0.0lf, 1.0lf);
        }
        if F.mode == 0u {
            textureStore(output, id.xy, vec4(f32(kept)));
            return;
        }
    }
    let p = textureLoad(input, id.xy, 0);
    if k[2] == 1.0lf {
        let g = f32(to_linear(kept) * f64(p.w));
        textureStore(output, id.xy, vec4(g, g, g, p.w));
        return;
    }
    var out = p;
    for (var c = 0u; c < 4u; c++) {
        out[c] = f32(f64(p[c]) * kept);
    }
    textureStore(output, id.xy, out);
}

// D-506, layer_fx::three_d_glasses: the left view `input`, its drawing's corner at (k[6], k[7]),
// and the right view `other`, its corner at (k[8], k[9]), already swapped; the output's corner at
// (F.ox, F.oy). k[0] the view, an index into effects::GLASSES_VIEWS; k[1] and k[2] the
// convergence and alignment in pixels; k[3] and k[4] the drawing's own size; k[5] the balance.
fn glasses_read(side: u32, x: f64, y: f64) -> vec4<f64> {
    if side == 0u {
        return vec4<f64>(bilinear(input, x + k[6], y + k[7]));
    }
    return vec4<f64>(bilinear(other, x + k[8], y + k[9]));
}

fn glasses_luma(p: vec4<f64>) -> f64 {
    return 0.2126lf * p.x + 0.7152lf * p.y + 0.0722lf * p.z;
}

@compute @workgroup_size(16, 16)
fn glasses(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let view = u32(k[0]);
    let x = f64(i32(id.x) - F.ox);
    let y = f64(i32(id.y) - F.oy);
    var out: vec4<f64>;
    if view == 0u {
        if 2.0lf * x < k[3] {
            out = glasses_read(0u, 2.0lf * x + 1.0lf, y + 0.5lf);
        } else {
            out = glasses_read(1u, 2.0lf * x + 1.0lf - k[3], y + 0.5lf);
        }
    } else if view == 1u {
        if 2.0lf * y < k[4] {
            out = glasses_read(0u, x + 0.5lf, 2.0lf * y + 1.0lf);
        } else {
            out = glasses_read(1u, x + 0.5lf, 2.0lf * y + 1.0lf - k[4]);
        }
    } else {
        var l = glasses_read(0u, x + 0.5lf - k[1] / 2.0lf, y + 0.5lf + k[2] / 2.0lf);
        var r = glasses_read(1u, x + 0.5lf + k[1] / 2.0lf, y + 0.5lf - k[2] / 2.0lf);
        if view == 2u {
            out = select(r, l, (i32(id.y) - F.oy) % 2 == 0);
        } else {
            if view >= 5u {
                let m = (l + r) / 2.0lf;
                l = m + k[5] * (l - m);
                r = m + k[5] * (r - m);
            }
            let a = clamp(max(l.w, r.w), 0.0lf, 1.0lf);
            var rgb = vec3(glasses_luma(r), glasses_luma(l), glasses_luma(l));
            if view == 3u || view == 5u {
                rgb.z = 0.0lf;
            } else if view == 7u {
                rgb = vec3(r.x, l.y, l.z);
            }
            out = vec4(max(rgb, vec3(0.0lf)), a);
        }
    }
    textureStore(output, id.xy, vec4<f32>(out));
}

// D-518, layer_fx::warp_blend's reactor as the alpha of the output (the buffer's size): k[0] its
// place in effects::WARP_BLEND_REACTORS; `other` the reveal lying on the drawing at (F.ox, F.oy)
// when k[1] is 1, clear when 0.
@compute @workgroup_size(16, 16)
fn wreact(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let a = textureLoad(input, vec2<i32>(id.xy), 0);
    var b = vec4<f32>(0.0);
    if k[1] == 1.0lf {
        b = at(other, vec2<i32>(id.xy) - vec2(F.ox, F.oy));
    }
    let la = phase(4u, a) * f64(a.w);
    let lb = phase(4u, b) * f64(b.w);
    var r = 0.0lf;
    switch u32(k[0]) {
        case 0u: { r = (la + lb) / 2.0lf; }
        case 1u: { r = abs(abs(la - 0.5lf) - abs(lb - 0.5lf)); }
        case 2u: { r = abs(la - lb); }
        default: {
            for (var c = 0u; c < 3u; c++) {
                r = max(r, abs(phase(c, a) * f64(a.w) - phase(c, b) * f64(b.w)));
            }
        }
    }
    textureStore(output, id.xy, vec4(0.0, 0.0, 0.0, f32(r)));
}

// D-518, layer_fx::warp_blend: `input` the layer, `other` the reveal lying on the drawing at
// (F.ox, F.oy) when k[3] is 1, `sums` from gmeltk (the pull in y and z); k[0] the completion as
// a share, k[1] the direction's place (joint, opposing, twisting), k[2] the reveal's share.
@compute @workgroup_size(16, 16)
fn warpb(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let s = sums[id.y * size.x + id.x];
    var dx = s.y;
    var dy = s.z;
    if k[1] == 2.0lf {
        dx = -s.z;
        dy = s.y;
    }
    var t = -1.0lf;
    if k[1] == 1.0lf {
        t = 1.0lf;
    }
    let c = k[0];
    let sx = f64(id.x) + 0.5lf;
    let sy = f64(id.y) + 0.5lf;
    let a = vec4<f64>(bilinear(input, sx + c * dx, sy + c * dy));
    var b = vec4<f64>(0.0lf);
    if k[3] == 1.0lf && k[2] > 0.0lf {
        b = vec4<f64>(bilinear(other, sx + t * (1.0lf - c) * dx - f64(F.ox), sy + t * (1.0lf - c) * dy - f64(F.oy)));
    }
    textureStore(output, id.xy, vec4<f32>((1.0lf - k[2]) * a + k[2] * b));
}

// D-517, layer_fx::twister: k as layer_fx::twist_numbers; `other` the back lying on the drawing
// at its corner when k[7] is 1, both read between pixels; the strip edge on clear.
@compute @workgroup_size(16, 16)
fn twist(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let dx = f64(i32(id.x) - F.ox) + 0.5lf - k[0];
    let dy = f64(i32(id.y) - F.oy) + 0.5lf - k[1];
    let u = dx * k[2] + dy * k[3];
    let v = -dx * k[3] + dy * k[2];
    let t = (u + k[4]) / (2.0lf * k[4]);
    let c = cos64(3.141592653589793lf * clamp(k[5] - t, 0.0lf, 1.0lf));
    let s = abs(c);
    var p = vec4<f64>(0.0lf);
    if s != 0.0lf {
        let b = v / s;
        let x = k[0] + u * k[2] - b * k[3];
        let y = k[1] + u * k[3] + b * k[2];
        if c > 0.0lf {
            p = vec4<f64>(bilinear(input, x + f64(F.ox), y + f64(F.oy)));
        } else if k[7] == 1.0lf {
            p = vec4<f64>(bilinear(other, x, y));
        }
        if k[6] == 1.0lf {
            p = vec4<f64>(p.xyz * s, p.w);
        }
    }
    textureStore(output, id.xy, vec4<f32>(p));
}

// D-509, layer_fx::glass_melt's share and bend into `sums` as (k, dx, dy, 0): `input` the bump
// in its alpha, the neighbours held inside; k[0] the amount, k[1] the gain, k[2] the edge, k[3]
// the band.
fn gmelt_h(p: vec2<i32>) -> f64 {
    let size = vec2<i32>(textureDimensions(input));
    return f64(textureLoad(input, clamp(p, vec2(0), size - vec2(1)), 0).w);
}

@compute @workgroup_size(16, 16)
fn gmeltk(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = vec2<i32>(id.xy);
    let dx = k[0] * (gmelt_h(p + vec2(1, 0)) - gmelt_h(p - vec2(1, 0))) / 2.0lf * k[1];
    let dy = k[0] * (gmelt_h(p + vec2(0, 1)) - gmelt_h(p - vec2(0, 1))) / 2.0lf * k[1];
    let kk = clamp((gmelt_h(p) - k[2]) / k[3] + 0.5lf, 0.0lf, 1.0lf);
    sums[id.y * size.x + id.x] = vec4(kk, dx, dy, 0.0lf);
}

// D-509, layer_fx::glass_melt: `input` the layer, `other` the layer to reveal lying on the drawing
// at (F.ox, F.oy) when k[0] is 1 (nothing revealed when 0), `sums` from gmeltk.
@compute @workgroup_size(16, 16)
fn gmelt(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let s = sums[id.y * size.x + id.x];
    let sx = f64(id.x) + 0.5lf;
    let sy = f64(id.y) + 0.5lf;
    let o = vec4<f64>(bilinear(input, sx + (1.0lf - s.x) * s.y, sy + (1.0lf - s.x) * s.z));
    var r = vec4<f64>(0.0lf);
    if k[0] == 1.0lf && s.x < 1.0lf {
        r = vec4<f64>(bilinear(other, sx - s.x * s.y - f64(F.ox), sy - s.x * s.z - f64(F.oy)));
    }
    textureStore(output, id.xy, vec4<f32>(s.x * o + (1.0lf - s.x) * r));
}

// D-507, layer_fx::cc_spotlight: k as layer_fx::spotlight_numbers; the drawing's corner at
// (F.ox, F.oy). The angle is worked in single precision, which has no double atan2. D-519: with
// k[14] 1, `other` the gel lying on the drawing at its corner, read between pixels.
@compute @workgroup_size(16, 16)
fn spotlight(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    // B-222: one channel at a time.
    let t = textureLoad(input, id.xy, 0);
    let p = vec4<f64>(f64(t.x), f64(t.y), f64(t.z), f64(t.w));
    var f = 0.0lf;
    if k[13] != 0.0lf {
        let d = vec3<f64>(f64(i32(id.x) - F.ox) + 0.5lf - k[1], f64(i32(id.y) - F.oy) + 0.5lf - k[2], -k[3]);
        let a = vec3<f64>(k[4], k[5], k[6]);
        let c64 = vec3<f64>(a.y * d.z - a.z * d.y, a.z * d.x - a.x * d.z, a.x * d.y - a.y * d.x);
        let cr = vec3<f32>(f32(c64.x), f32(c64.y), f32(c64.z));
        let th = f64(atan2(length(cr), f32(dot(a, d))));
        if th <= k[8] {
            f = 1.0lf;
        } else if th < k[7] {
            let t = (k[7] - th) / (k[7] - k[8]);
            f = t * t * (3.0lf - 2.0lf * t);
        }
    }
    let q = f * k[9];
    var c = vec3<f64>(k[10], k[11], k[12]);
    if k[14] == 1.0lf && f > 0.0lf {
        let d = vec3<f64>(f64(i32(id.x) - F.ox) + 0.5lf - k[1], f64(i32(id.y) - F.oy) + 0.5lf - k[2], -k[3]);
        let s = dot(d, vec3<f64>(k[15], k[16], k[17])) * k[24];
        let gx = k[26] + dot(d, vec3<f64>(k[18], k[19], k[20])) / s * k[25];
        let gy = k[27] + dot(d, vec3<f64>(k[21], k[22], k[23])) / s * k[25];
        let g = vec4<f64>(bilinear(other, gx, gy));
        c = c * (vec3(1.0lf - g.w) + g.xyz);
    }
    let mode = u32(k[0]);
    var out: vec4<f64>;
    if mode == 0u {
        out = vec4(c * q, min(q, 1.0lf));
    } else if mode == 1u {
        out = vec4(p.xyz + c * q * p.w, p.w);
    } else if mode == 2u {
        out = vec4(p.xyz + c * q, p.w + (1.0lf - p.w) * min(q, 1.0lf));
    } else {
        let dark = (1.0lf - f) * (1.0lf - min(k[9], 1.0lf));
        out = vec4(p.xyz * (vec3(dark) + f * c), p.w);
    }
    textureStore(output, id.xy, vec4<f32>(f32(out.x), f32(out.y), f32(out.z), f32(out.w)));
}

// D-526: a picture (`other`, as big as `input`) laid over the drawing when k[0] is 1, alone when 0.
@compute @workgroup_size(16, 16)
fn over(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let q = textureLoad(other, id.xy, 0);
    let p = textureLoad(input, id.xy, 0) * f32(k[0]);
    textureStore(output, id.xy, q + p * (1.0 - q.w));
}

// D-531, layer_fx::environment_tap: `other` (the environment) at (u, v), rows held, columns
// taken round with `wrap`, else held.
fn env_tap(u: f64, v: f64, wrap: bool) -> vec4<f64> {
    let size = vec2<i32>(textureDimensions(other));
    let fx = u - 0.5lf;
    let fy = v - 0.5lf;
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;
    var out = vec4<f64>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    for (var j = 0; j < 2; j++) {
        let wy = select(1.0lf - uy, uy, j == 1);
        if wy == 0.0lf {
            continue;
        }
        let sy = clamp(i32(y0) + j, 0, size.y - 1);
        for (var i = 0; i < 2; i++) {
            let wx = select(1.0lf - ux, ux, i == 1);
            if wx == 0.0lf {
                continue;
            }
            var sx = i32(x0) + i;
            if wrap {
                sx = ((sx % size.x) + size.x) % size.x;
            } else {
                sx = clamp(sx, 0, size.x - 1);
            }
            let p = textureLoad(other, vec2(sx, sy), 0);
            out += vec4<f64>(f64(p.x), f64(p.y), f64(p.z), f64(p.w)) * (wx * wy);
        }
    }
    return out;
}

// D-531, layer_fx::environment_look: k as layer_fx::environment_numbers.
fn env_look(x: f64, y: f64) -> vec4<f64> {
    let c = vec3<f64>(x - k[11], y - k[12], k[13]);
    let d0 = vec3<f64>(
        k[2] * c.x + k[3] * c.y + k[4] * c.z,
        k[5] * c.x + k[6] * c.y + k[7] * c.z,
        k[8] * c.x + k[9] * c.y + k[10] * c.z,
    );
    let d = d0 / sqrt(dot(d0, d0));
    let ew = k[14];
    let eh = k[15];
    let mode = u32(k[0]);
    if mode == 0u {
        let s = clamp(-d.y, -1.0lf, 1.0lf);
        let u = ew * (0.5lf + atan2_64(d.x, d.z) / 6.283185307179586lf);
        let v = eh * (0.5lf - atan2_64(s, sqrt(1.0lf - s * s)) / 3.141592653589793lf);
        return env_tap(u, v, true);
    }
    if mode == 1u {
        let s = sqrt(d.x * d.x + d.y * d.y);
        var q = 0.0lf;
        if s != 0.0lf {
            let z = clamp(d.z, -1.0lf, 1.0lf);
            q = atan2_64(sqrt(1.0lf - z * z), z) / (3.141592653589793lf * s);
        }
        return env_tap(ew * (1.0lf + d.x * q) / 2.0lf, eh * (1.0lf + d.y * q) / 2.0lf, false);
    }
    let a = abs(d);
    var f = vec4<f64>(1.0lf, 3.0lf, d.x / a.z, -d.y / a.z);
    if a.x >= a.y && a.x >= a.z {
        f = select(vec4<f64>(0.0lf, 1.0lf, d.z / a.x, d.y / a.x), vec4<f64>(2.0lf, 1.0lf, -d.z / d.x, d.y / d.x), d.x > 0.0lf);
    } else if a.y >= a.z {
        f = select(vec4<f64>(1.0lf, 2.0lf, d.x / d.y, -d.z / d.y), vec4<f64>(1.0lf, 0.0lf, d.x / a.y, d.z / a.y), d.y < 0.0lf);
    } else if d.z > 0.0lf {
        f = vec4<f64>(1.0lf, 1.0lf, d.x / d.z, d.y / d.z);
    }
    let fw = ew / 3.0lf;
    let fh = eh / 4.0lf;
    let u = min(max(fw * (f.x + (f.z + 1.0lf) / 2.0lf), f.x * fw + 0.5lf), (f.x + 1.0lf) * fw - 0.5lf);
    let v = min(max(fh * (f.y + (f.w + 1.0lf) / 2.0lf), f.y * fh + 0.5lf), (f.y + 1.0lf) * fh - 0.5lf);
    return env_tap(u, v, false);
}

// D-531, layer_fx::cc_environment: the holder `input`, its drawing's corner at (F.ox, F.oy); the
// environment `other` (one clear pixel for an empty one).
@compute @workgroup_size(16, 16)
fn environment(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let a = textureLoad(input, vec2<i32>(id.xy), 0).w;
    var got = vec4<f64>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    if a != 0.0 {
        let x = f64(i32(id.x) - F.ox);
        let y = f64(i32(id.y) - F.oy);
        if k[1] == 1.0lf {
            for (var j = 0; j < 2; j++) {
                for (var i = 0; i < 2; i++) {
                    got += env_look(x + select(0.25lf, 0.75lf, i == 1), y + select(0.25lf, 0.75lf, j == 1));
                }
            }
            got = got / 4.0lf;
        } else {
            got = env_look(x + 0.5lf, y + 0.5lf);
        }
        got = got * f64(a);
    }
    textureStore(output, id.xy, vec4<f32>(f32(got.x), f32(got.y), f32(got.z), f32(got.w)));
}


// D-580, layer_fx::card_wipe: k as layer_fx::card_wipe_numbers, the back picture `other` (one
// clear pixel when none was read); the drawing's corner at (F.ox, F.oy). Divisions rounded once
// as the CPU's are; the highlight's power is taken in single precision.
fn card_span(lo: f64, hi: f64, size: f64, half_page: f64, reach: f64, n: f64) -> vec2<i32> {
    let first = floor(quotient(lo + half_page - reach, size) - 0.5lf) - 1.0lf;
    let last = ceil(quotient(hi + half_page + reach, size) - 0.5lf) + 1.0lf;
    return vec2<i32>(i32(clamp(first, 0.0lf, n)), i32(clamp(last + 1.0lf, 0.0lf, n)));
}

fn card_unit(p: vec3<f64>) -> vec3<f64> {
    let n = sqrt(dot(p, p));
    if n == 0.0lf {
        return vec3<f64>(0.0lf, 0.0lf, 0.0lf);
    }
    return vec3<f64>(quotient(p.x, n), quotient(p.y, n), quotient(p.z, n));
}

@compute @workgroup_size(16, 16)
fn cardwipe(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let cols = k[0];
    let rows = k[1];
    let cw = k[2];
    let ch = k[3];
    let hx = k[4];
    let hy = k[5];
    let w0 = k[6];
    let h0 = k[7];
    let sc = k[34];
    var got = vec4<f64>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    let d = vec3<f64>(f64(i32(id.x) - F.ox) + 0.5lf - w0 / 2.0lf, f64(i32(id.y) - F.oy) + 0.5lf - h0 / 2.0lf, k[8]);
    let v = vec3<f64>(
        k[12] * d.x + k[15] * d.y + k[18] * d.z,
        k[13] * d.x + k[16] * d.y + k[19] * d.z,
        k[14] * d.x + k[17] * d.y + k[20] * d.z,
    );
    let o = vec3<f64>(k[9], k[10], k[11]);
    let zm = k[35] * 1.001lf + 1e-9lf;
    var sa = vec2<i32>(0, 0);
    var sb = vec2<i32>(0, 0);
    if sc != 0.0lf {
        if v.z != 0.0lf {
            let ta = quotient(-zm - o.z, v.z);
            let tb = quotient(zm - o.z, v.z);
            let t0 = max(min(ta, tb), 0.0lf);
            let t1 = max(ta, tb);
            if t1 >= t0 {
                let xa = o.x + t0 * v.x;
                let xb = o.x + t1 * v.x;
                let ya = o.y + t0 * v.y;
                let yb = o.y + t1 * v.y;
                sa = card_span(min(xa, xb), max(xa, xb), cw, w0 / 2.0lf, hx, cols);
                sb = card_span(min(ya, yb), max(ya, yb), ch, h0 / 2.0lf, hy, rows);
            }
        } else if abs(o.z) <= zm {
            sa = vec2<i32>(0, i32(cols));
            sb = vec2<i32>(0, i32(rows));
        }
    }
    // The nearest hit, the first card in rows then columns where two tie.
    var found = false;
    var bt = 0.0lf;
    var bat = 0u;
    var bl = vec2<f64>(0.0lf, 0.0lf);
    var bfront = false;
    var bc = vec2<f64>(0.0lf, 0.0lf);
    for (var b = sb.x; b < sb.y; b++) {
        for (var a = sa.x; a < sa.y; a++) {
            let ci = u32(36 + 3 * (b * i32(cols) + a));
            let axis = k[ci];
            let s = k[ci + 1];
            let c = k[ci + 2];
            let cx = (f64(a) + 0.5lf) * cw;
            let cy = (f64(b) + 0.5lf) * ch;
            let q = vec3<f64>(o.x - (cx - w0 / 2.0lf), o.y - (cy - h0 / 2.0lf), o.z);
            var o2 = vec3<f64>(c * q.x - s * q.z, q.y, s * q.x + c * q.z);
            var d2 = vec3<f64>(c * v.x - s * v.z, v.y, s * v.x + c * v.z);
            if axis == 0.0lf {
                o2 = vec3<f64>(q.x, c * q.y + s * q.z, -s * q.y + c * q.z);
                d2 = vec3<f64>(v.x, c * v.y + s * v.z, -s * v.y + c * v.z);
            }
            if d2.z == 0.0lf {
                continue;
            }
            let t = quotient(-o2.z, d2.z);
            if t <= 0.0lf || (found && t >= bt) {
                continue;
            }
            let lx = o2.x + t * d2.x;
            let ly = o2.y + t * d2.y;
            if abs(lx) <= hx && abs(ly) <= hy {
                found = true;
                bt = t;
                bat = ci;
                bl = vec2<f64>(lx, ly);
                bfront = d2.z > 0.0lf;
                bc = vec2<f64>(cx, cy);
            }
        }
    }
    if found {
        let axis = k[bat];
        let s = k[bat + 1];
        let c = k[bat + 2];
        let ux = quotient(bl.x, sc);
        let uy = quotient(bl.y, sc);
        var p32 = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        if bfront {
            p32 = bilinear(input, bc.x + ux + f64(F.ox), bc.y + uy + f64(F.oy));
        } else if k[33] != 0.0lf {
            if axis == 1.0lf {
                p32 = bilinear(other, bc.x - ux, bc.y + uy);
            } else {
                p32 = bilinear(other, bc.x + ux, bc.y - uy);
            }
        }
        let p = vec4<f64>(f64(p32.x), f64(p32.y), f64(p32.z), f64(p32.w));
        let pl = vec3<f64>(k[22], k[23], k[24]);
        let hit = o + bt * v;
        var lv = card_unit(pl);
        if k[21] != 0.0lf {
            lv = card_unit(pl - hit);
        }
        var n = vec3<f64>(-s, 0.0lf, -c);
        if axis == 0.0lf {
            n = vec3<f64>(0.0lf, s, -c);
        }
        if !bfront {
            n = -n;
        }
        let vv = card_unit(-v);
        let hv = lv + vv;
        let nl = dot(n, lv);
        let hn = sqrt(dot(hv, hv));
        var sp = 0.0lf;
        if nl > 0.0lf && hn > 0.0lf {
            sp = f64(pow(f32(max(quotient(dot(n, hv), hn), 0.0lf)), f32(k[32])));
        }
        let lc = vec3<f64>(k[26], k[27], k[28]);
        let lit = p.xyz * (vec3<f64>(k[29], k[29], k[29]) + k[30] * k[25] * lc * max(nl, 0.0lf)) + k[31] * k[25] * sp * lc * p.w;
        got = vec4<f64>(lit, p.w);
    }
    textureStore(output, id.xy, vec4<f32>(f32(got.x), f32(got.y), f32(got.z), f32(got.w)));
}

// D-532, layer_fx::cc_sphere: k as layer_fx::cylinder_numbers; the drawing's corner at
// (F.ox, F.oy). The highlight's power is taken in single precision.
@compute @workgroup_size(16, 16)
fn sphere(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let d = vec3<f64>(f64(i32(id.x) - F.ox) + 0.5lf - k[1], f64(i32(id.y) - F.oy) + 0.5lf - k[2], -k[3]);
    let v = vec3<f64>(
        k[4] * d.x + k[7] * d.y + k[10] * d.z,
        k[5] * d.x + k[8] * d.y + k[11] * d.z,
        k[6] * d.x + k[9] * d.y + k[12] * d.z,
    );
    let o = vec3<f64>(k[13], k[14], k[15]);
    let r = k[16];
    let a = dot(v, v);
    let b = dot(o, v);
    let disc = b * b - a * (dot(o, o) - r * r);
    var got = vec4<f64>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    if r != 0.0lf && disc >= 0.0lf {
        let sq = sqrt(disc);
        let mode = u32(k[0]);
        let lv = sqrt(dot(v, v));
        let l = vec3<f64>(k[17], k[18], k[19]);
        let hv = l - v / lv;
        let hl = sqrt(dot(hv, hv));
        let lc = vec3<f64>(k[21], k[22], k[23]);
        // The inside first, the outside laid over it.
        for (var j = 0; j < 2; j++) {
            let inside = j == 0;
            let t = select((-b - sq) / a, (-b + sq) / a, inside);
            if (inside && mode == 1u) || (!inside && mode == 2u) || !(t > 0.0lf) {
                continue;
            }
            let hit = o + t * v;
            let phi = atan2_64(hit.x, -hit.z);
            let lat = atan2_64(hit.y, sqrt(hit.x * hit.x + hit.z * hit.z));
            // The taps' rows held to the drawing: the same as the row held half a pixel in.
            let y = min(max(k[30] / 2.0lf + lat / 3.141592653589793lf * k[30], 0.5lf), k[30] - 0.5lf);
            let s = round_tap(k[29] / 2.0lf + phi / 6.283185307179586lf * k[29], y + f64(F.oy), k[29], f64(F.ox));
            let p = vec4<f64>(f64(s.x), f64(s.y), f64(s.z), f64(s.w));
            let sg = select(1.0lf, -1.0lf, inside);
            let n = hit * (sg / r);
            let nl = dot(n, l);
            var sp = 0.0lf;
            if k[28] > 0.0lf && nl > 0.0lf && hl > 0.0lf {
                sp = f64(pow(f32(max(dot(n, hv) / hl, 0.0lf)), f32(k[28])));
            }
            let diff = k[25] * k[20] * max(nl, 0.0lf);
            let spec = k[26] * k[20] * sp;
            let lit = p.xyz * (vec3<f64>(k[24], k[24], k[24]) + diff * lc) + spec * (k[27] * p.xyz + (1.0lf - k[27]) * lc * p.w);
            got = vec4<f64>(lit, p.w) + (1.0lf - p.w) * got;
        }
    }
    textureStore(output, id.xy, vec4<f32>(f32(got.x), f32(got.y), f32(got.z), f32(got.w)));
}

// D-530, layer_fx::cc_cylinder: k as layer_fx::cylinder_numbers; the drawing's corner at
// (F.ox, F.oy). The highlight's power is taken in single precision.
@compute @workgroup_size(16, 16)
fn cylinder(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let d = vec3<f64>(f64(i32(id.x) - F.ox) + 0.5lf - k[1], f64(i32(id.y) - F.oy) + 0.5lf - k[2], -k[3]);
    let v = vec3<f64>(
        k[4] * d.x + k[7] * d.y + k[10] * d.z,
        k[5] * d.x + k[8] * d.y + k[11] * d.z,
        k[6] * d.x + k[9] * d.y + k[12] * d.z,
    );
    let o = vec3<f64>(k[13], k[14], k[15]);
    let r = k[16];
    let a = v.x * v.x + v.z * v.z;
    let b = o.x * v.x + o.z * v.z;
    let disc = b * b - a * (o.x * o.x + o.z * o.z - r * r);
    var got = vec4<f64>(0.0lf, 0.0lf, 0.0lf, 0.0lf);
    if r != 0.0lf && a != 0.0lf && disc >= 0.0lf {
        let sq = sqrt(disc);
        let mode = u32(k[0]);
        let lv = sqrt(dot(v, v));
        let l = vec3<f64>(k[17], k[18], k[19]);
        let hv = l - v / lv;
        let hl = sqrt(dot(hv, hv));
        let lc = vec3<f64>(k[21], k[22], k[23]);
        // The inside first, the outside laid over it.
        for (var j = 0; j < 2; j++) {
            let inside = j == 0;
            let t = select((-b - sq) / a, (-b + sq) / a, inside);
            let hy = o.y + t * v.y;
            if (inside && mode == 1u) || (!inside && mode == 2u) || !(t > 0.0lf && abs(hy) <= k[30] / 2.0lf) {
                continue;
            }
            let hx = o.x + t * v.x;
            let hz = o.z + t * v.z;
            let phi = atan2_64(hx, -hz);
            let s = round_tap(k[29] / 2.0lf + phi / 6.283185307179586lf * k[29], hy + k[30] / 2.0lf + f64(F.oy), k[29], f64(F.ox));
            let p = vec4<f64>(f64(s.x), f64(s.y), f64(s.z), f64(s.w));
            let sg = select(1.0lf, -1.0lf, inside);
            let n = vec3<f64>(sg * hx / r, 0.0lf, sg * hz / r);
            let nl = dot(n, l);
            var sp = 0.0lf;
            if k[28] > 0.0lf && nl > 0.0lf && hl > 0.0lf {
                sp = f64(pow(f32(max(dot(n, hv) / hl, 0.0lf)), f32(k[28])));
            }
            let diff = k[25] * k[20] * max(nl, 0.0lf);
            let spec = k[26] * k[20] * sp;
            let lit = p.xyz * (vec3<f64>(k[24], k[24], k[24]) + diff * lc) + spec * (k[27] * p.xyz + (1.0lf - k[27]) * lc * p.w);
            got = vec4<f64>(lit, p.w) + (1.0lf - p.w) * got;
        }
    }
    textureStore(output, id.xy, vec4<f32>(f32(got.x), f32(got.y), f32(got.z), f32(got.w)));
}

// B-228, layer_fx::pass_extract: the pass (`other`) lying on the drawing at (F.ox, F.oy), alpha 1
// on it and 0 outside it; k black, white, invert, clamp. D-552: the pass's alpha (1 unless a named
// pass picks one) held to 0..1, each value divided by it first when k[4] (unmult), the result
// laid over it; an alpha of 0 or below is clear, as outside.
@compute @workgroup_size(16, 16)
fn passx(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let q = at(other, vec2<i32>(id.xy) - vec2(F.ox, F.oy));
    var out = vec4(0.0);
    if q.w > 0.0 {
        let a = min(f64(q.w), 1.0lf);
        for (var c = 0u; c < 3u; c++) {
            var v = f64(q[c]);
            if k[4] == 1.0lf {
                v = v / a;
            }
            var o = select(0.0lf, 1.0lf, v >= k[1]);
            if k[1] != k[0] {
                o = (v - k[0]) / (k[1] - k[0]);
            }
            if k[2] == 1.0lf {
                o = 1.0lf - o;
            }
            if k[3] == 1.0lf {
                o = clamp(o, 0.0lf, 1.0lf);
            }
            out[c] = f32(o * a);
        }
        out.w = f32(a);
    }
    textureStore(output, id.xy, out);
}

// B-228, layer_fx::depth_key: the pass as Pass Extract's; k depth, feather, invert.
@compute @workgroup_size(16, 16)
fn dkey(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let q = at(other, vec2<i32>(id.xy) - vec2(F.ox, F.oy));
    var kept = 0.0lf;
    if q.w > 0.0 {
        let z = f64(q.x);
        kept = select(0.0lf, 1.0lf, z >= k[0]);
        if k[1] > 0.0lf {
            kept = clamp((z - k[0]) / k[1] + 0.5lf, 0.0lf, 1.0lf);
        }
        if k[2] == 1.0lf {
            kept = 1.0lf - kept;
        }
    }
    var out = p;
    for (var c = 0u; c < 4u; c++) {
        out[c] = f32(f64(p[c]) * kept);
    }
    textureStore(output, id.xy, out);
}

// D-418, layer_fx::fill. F.mode 0: the covering before its feathers (`row`, the buffer grown by
// F.g on every side, layer_fx::fill_cover) into `output`. F.mode 1: the drawing (`input`) with the
// colour laid on as the covering (`other`, grown by F.g) says; k: the colour (linear), the
// opacity (a share), 1 to invert, 1 for the whole layer (no covering).
@compute @workgroup_size(16, 16)
fn fillfx(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    if F.mode == 0u {
        textureStore(output, id.xy, vec4(row[id.y * size.x + id.x], 0.0, 0.0, 0.0));
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    var m = 1.0lf;
    if k[5] == 0.0lf {
        m = f64(textureLoad(other, vec2<i32>(id.xy) + vec2(F.g), 0).x);
    }
    let c = select(m, 1.0lf - m, k[4] != 0.0lf) * k[3];
    let a = f64(p.w);
    var out = p;
    for (var i = 0u; i < 3u; i++) {
        out[i] = f32(f64(p[i]) * (1.0lf - c) + k[i] * a * c);
    }
    textureStore(output, id.xy, out);
}

// D-419, layer_fx::eyedropper_colour and eyedropper_fill. k: the area's pixel count, the
// averaging (0 Skip Empty, 1 All, 2 All Premultiplied, 3 Including Alpha), 1 to keep each
// pixel's alpha, Blend With Original (a share), then F.count rows of the buffer as row, first
// column, last column. `eyerow`: each row summed into `sums` (premultiplied, then the straight
// colours of the pixels that show and their count), one row a thread, in the CPU's order.
@compute @workgroup_size(64)
fn eyerow(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= F.count {
        return;
    }
    let y = u32(k[4u + 3u * id.x]);
    var p = vec4<f64>(0.0lf);
    var s = vec4<f64>(0.0lf);
    let x0 = u32(k[5u + 3u * id.x]);
    let x1 = u32(k[6u + 3u * id.x]);
    for (var x = x0; x <= x1; x++) {
        let q = eyewide(textureLoad(input, vec2(x, y), 0));
        p += q;
        if q.w > 0.0lf {
            s += vec4(q.xyz / q.w, 1.0lf);
        }
    }
    sums[2u * id.x] = p;
    sums[2u * id.x + 1u] = s;
}

// The texel in double precision a channel at a time: on the RTX 4070 Ti SUPER (Vulkan) the
// whole-vector `vec4<f64>(textureLoad(..))` came back as garbage here.
fn eyewide(t: vec4<f32>) -> vec4<f64> {
    return vec4(f64(t.x), f64(t.y), f64(t.z), f64(t.w));
}

// `eyesum`: the rows added in order and the sampled colour, premultiplied, after them in `sums`.
@compute @workgroup_size(1)
fn eyesum(@builtin(global_invocation_id) id: vec3<u32>) {
    var p = vec4<f64>(0.0lf);
    var s = vec4<f64>(0.0lf);
    for (var j = 0u; j < F.count; j++) {
        p += sums[2u * j];
        s += sums[2u * j + 1u];
    }
    let n = k[0];
    var c = vec3<f64>(0.0lf);
    var a = 1.0lf;
    if k[1] == 0.0lf {
        if s.w > 0.0lf {
            c = s.xyz / s.w;
        }
    } else if k[1] == 1.0lf {
        c = s.xyz / n;
    } else if k[1] == 2.0lf {
        c = p.xyz / n;
    } else {
        if p.w > 0.0lf {
            c = p.xyz / p.w;
        }
        a = p.w / n;
    }
    sums[2u * F.count] = vec4(c * a, a);
}

// `eyefill`: every pixel the sampled colour (times its own alpha when k[2] is 1), blended back.
@compute @workgroup_size(16, 16)
fn eyefill(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = eyewide(textureLoad(input, id.xy, 0));
    let m = select(1.0lf, p.w, k[2] == 1.0lf);
    let b = k[3];
    textureStore(output, id.xy, vec4<f32>(sums[2u * F.count] * m * (1.0lf - b) + p * b));
}

// D-440, layer_fx::paint_bucket. `other`: the area, blurred (or, to view the threshold, the
// matches). k: the colour (linear), the opacity (a share), the blend (as `laid`'s, Fill Only
// as none), 1 for straight_color (times the pixel's own alpha), 1 to view the threshold.
@compute @workgroup_size(16, 16)
fn bucket(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let m = textureLoad(other, id.xy, 0).x;
    if k[6] == 1.0lf {
        textureStore(output, id.xy, vec4(m, m, m, 1.0));
        return;
    }
    let c = f64(m) * select(1.0lf, f64(p.w), k[5] == 1.0lf) * k[3];
    let s = vec4(f32(k[0] * c), f32(k[1] * c), f32(k[2] * c), f32(c));
    textureStore(output, id.xy, laid(s, p, u32(k[4])));
}

// B-229, layer_fx::id_key. k[2] 0: the matte of the ids (`other`, as big as `input`), 1 where
// the id is within a half of k[0], turned over where k[1] is 1. k[2] 1: the drawing (`input`)
// times the matte (`other`, blurred) lying at (F.ox, F.oy), 0 outside it.
@compute @workgroup_size(16, 16)
fn idkey(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    if k[2] == 0.0lf {
        let hit = abs(f64(textureLoad(other, id.xy, 0).x) - k[0]) < 0.5lf;
        textureStore(output, id.xy, vec4(select(0.0, 1.0, hit != (k[1] == 1.0lf))));
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let m = f64(at(other, vec2<i32>(id.xy) - vec2(F.ox, F.oy)).x);
    var out = p;
    for (var c = 0u; c < 4u; c++) {
        out[c] = f32(f64(p[c]) * m);
    }
    textureStore(output, id.xy, out);
}

// B-226, line_width::line_width: the drawing (`input`) grown by F.g. k[0] 0 to thicken by shape,
// 1 to thin by shape, 2 by colour; k[1] 1 to thicken; k[2] how many offsets, nearest first, then
// each across and down from 3 (line_width::disc). `row` 1 where a pixel is chosen, as
// selective_blur::chosen chose it on the CPU.
fn lw_picked(p: vec2<i32>, size: vec2<i32>) -> bool {
    return all(p >= vec2(0)) && all(p < size) && row[u32(p.y * size.x + p.x)] != 0.0;
}

@compute @workgroup_size(16, 16)
fn lwidth(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let s = vec2<i32>(textureDimensions(input));
    let l = vec2<i32>(id.xy) - vec2(F.g);
    let own = at(input, l);
    let n = u32(k[2]);
    var value = own;
    if k[0] == 0.0lf {
        for (var i = 0u; i < n; i++) {
            if value.w >= 1.0 {
                break;
            }
            let p = at(input, l + vec2(i32(k[3u + 2u * i]), i32(k[4u + 2u * i])));
            if p.w > value.w {
                value = p;
            }
        }
    } else if k[0] == 1.0lf {
        var least = own.w;
        for (var i = 0u; i < n; i++) {
            if least <= 0.0 {
                break;
            }
            least = min(least, at(input, l + vec2(i32(k[3u + 2u * i]), i32(k[4u + 2u * i]))).w);
        }
        if own.w > 0.0 {
            let q = least / own.w;
            value = vec4(own.x * q, own.y * q, own.z * q, least);
        }
    } else {
        let wanted = k[1] == 1.0lf;
        if lw_picked(l, s) != wanted {
            for (var i = 0u; i < n; i++) {
                let o = l + vec2(i32(k[3u + 2u * i]), i32(k[4u + 2u * i]));
                if lw_picked(o, s) == wanted {
                    value = at(input, o);
                    break;
                }
            }
        }
    }
    textureStore(output, id.xy, value);
}

// B-226, line_smooth::Smooth::line: one pair of rows, or of columns, a thread, as the CPU works
// each. `band` holds the encoded picture (line_smooth::encoded, worked out by the CPU), four
// numbers a pixel; k[0] the slope, k[1] the threshold. Each mix is written to `row`, eight a
// pixel in the order the CPU makes them: rows then columns, the pair the pixel ends then the
// pair it begins, a run's left end then its right. An area of 0 is written as -0, so a mixed
// pixel is told from one not.
var<private> ls_r: i32;
var<private> ls_lx: i32;
var<private> ls_ly: i32;
var<private> ls_off: i32;
var<private> ls_dx: i32;
var<private> ls_dy: i32;
var<private> ls_lrow: i32;
var<private> ls_lend: i32;
var<private> ls_do1: bool;
var<private> ls_dir: u32;

fn ls_px(i: i32) -> vec4<f32> {
    let j = 4u * u32(i);
    return vec4(band[j], band[j + 1u], band[j + 2u], band[j + 3u]);
}

// D-86: `<=` where OpenToonz has `<`.
fn ls_eq(a: i32, b: i32) -> bool {
    let t = f32(k[1]);
    let d = abs(ls_px(a) - ls_px(b));
    return d.x <= t && d.y <= t && d.z <= t && d.w <= t;
}

fn ls_count(pix: i32, a: i32, b: i32) -> vec2<i32> {
    let one = vec2(select(0, 1, ls_eq(pix - ls_dx, a)) + select(0, 1, ls_eq(pix - ls_dx, b)), 0);
    return one + vec2(0, select(0, 1, ls_eq(pix, a)) + select(0, 1, ls_eq(pix, b)));
}

fn ls_neighbourhood(x: i32, y: i32, pix: i32) -> bool {
    let dx = ls_dx;
    let dy = ls_dy;
    var c = vec2(0);
    if y > 1 {
        c += ls_count(pix, pix - 2 * dy, pix - 2 * dy - dx);
    }
    if y < ls_ly - 1 {
        c += ls_count(pix, pix + dy, pix + dy - dx);
    }
    if x > 1 {
        c += ls_count(pix, pix - 2 * dx, pix - 2 * dx - dy);
    }
    if x < ls_lx - 1 {
        c += ls_count(pix, pix + dx, pix + dx - dy);
    }
    return c.x > c.y;
}

fn ls_mix(out: i32, area: f64, lower: bool, side: u32) {
    let a = f32(area);
    row[8u * u32(out) + 4u * ls_dir + select(0u, 2u, lower) + side] = select(bitcast<f32>(0x80000000u), a, a != 0.0);
}

fn ls_filter(start: i32, ll: i32, step: i32, slope: f64, lower: bool, side: u32) {
    var out = start;
    var h0 = 0.5lf;
    let base = h0 / slope;
    let end = i32(min(floor(base), f64(ll)));
    for (var i = 0; i < end; i++) {
        let h1 = h0 - slope;
        ls_mix(out, 0.5lf * (h0 + h1), lower, side);
        out += step;
        h0 = h1;
    }
    if end < ll {
        ls_mix(out, 0.5lf * (base - f64(end)) * h0, lower, side);
    }
}

fn ls_crossing(al: i32, bl: i32, au: i32, bu: i32) -> i32 {
    var n = 0;
    for (var side = 0; side < 2; side++) {
        var a = au;
        var b = bu;
        var step = ls_dy;
        var room = ls_ly - 1 - ls_r;
        if side == 1 {
            a = al;
            b = bl;
            step = -ls_dy;
            room = ls_r - 1;
        }
        if ls_eq(a, b) {
            continue;
        }
        n += 1;
        var qa = a + step;
        var qb = b + step;
        while room > 0 && ls_eq(qa, a) && ls_eq(qb, b) {
            n += 1;
            room -= 1;
            qa += step;
            qb += step;
        }
    }
    return n;
}

fn ls_corner(len: i32, al: i32, bl: i32, au: i32, bu: i32) -> bool {
    return len >= 4 && ls_crossing(al, bl, au, bu) >= 4;
}

fn ls_check_length(len: i32, l1: i32, u1: i32, l2: i32, u2: i32, unite_u: bool) -> bool {
    let dy = ls_dy;
    return (len > 1) || (ls_do1 && ((unite_u && ls_r > 1 && !(ls_eq(l1, l1 - dy) && ls_eq(l2, l2 - dy))) || (ls_r < ls_ly - 1 && !(ls_eq(u1, u1 + dy) && ls_eq(u2, u2 + dy)))));
}

fn ls_right(ll: i32, lr: i32, whole: bool) {
    let ur = lr + ls_off;
    let len = (lr - ll) / ls_dx;
    let l1 = lr - ls_dx;
    let u1 = ur - ls_dx;
    let x = (l1 - ls_lrow) / ls_dx;
    if ls_corner(len, l1, lr, u1, ur) {
        return;
    }
    var unite_u = ls_eq(u1, lr);
    let unite_l = ls_eq(l1, ur);
    if unite_u || unite_l {
        if unite_u && unite_l {
            unite_u = !ls_neighbourhood(x + 1, ls_r, ur);
        }
        if ls_check_length(len, l1, u1, lr, ur, unite_u) {
            ls_filter(select(u1, l1, unite_u), len, -ls_dx, k[0] / (f64(len) * select(1.0lf, 2.0lf, whole)), unite_u, 1u);
        }
    }
}

fn ls_left(ll: i32, lr: i32, whole: bool) {
    let ul = ll + ls_off;
    let len = (lr - ll) / ls_dx;
    let l0 = ll - ls_dx;
    let u0 = ul - ls_dx;
    let x = (ll - ls_lrow) / ls_dx;
    if ls_corner(len, l0, ll, u0, ul) {
        return;
    }
    var unite_u = ls_eq(ul, l0);
    let unite_l = ls_eq(ll, u0);
    if unite_u || unite_l {
        if unite_u && unite_l {
            unite_u = ls_neighbourhood(x, ls_r, ul);
        }
        if ls_check_length(len, l0, u0, ll, ul, unite_u) {
            ls_filter(select(ul, ll, unite_u), len, ls_dx, k[0] / (f64(len) * select(1.0lf, 2.0lf, whole)), unite_u, 0u);
        }
    }
}

fn ls_same(i: i32) -> bool {
    return ls_eq(i, i + ls_off);
}

// Where the run starting at `i` ends: both lines keep their colours up to there.
fn ls_run_from(i: i32) -> i32 {
    var j = i + ls_dx;
    while j != ls_lend && ls_eq(i, j) && ls_eq(i + ls_off, j + ls_off) {
        j += ls_dx;
    }
    return j;
}

@compute @workgroup_size(64)
fn smoothscan(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = vec2<i32>(textureDimensions(input));
    let w = size.x;
    let h = size.y;
    let t = i32(id.x);
    if t < h - 1 {
        ls_r = t + 1;
        ls_lx = w;
        ls_ly = h;
        ls_lrow = t * w;
        ls_off = w;
        ls_dx = 1;
        ls_dy = w;
        ls_do1 = true;
        ls_dir = 0u;
    } else if t < h - 1 + w - 1 {
        let x = t - (h - 1);
        ls_r = x + 1;
        ls_lx = h;
        ls_ly = w;
        ls_lrow = x;
        ls_off = 1;
        ls_dx = w;
        ls_dy = 1;
        ls_do1 = false;
        ls_dir = 1u;
    } else {
        return;
    }
    ls_lend = ls_lrow + ls_lx * ls_dx;
    var ll = ls_lrow;
    var lr = ls_lend;
    if !ls_same(ll) {
        lr = ls_run_from(ll);
        if lr != ls_lend {
            ls_right(ll, lr, true);
        }
        ll = lr;
    }
    while ll != ls_lend && ls_same(ll) {
        ll += ls_dx;
    }
    while ll != ls_lend {
        lr = ls_run_from(ll);
        if lr == ls_lend {
            break;
        }
        ls_left(ll, lr, false);
        ls_right(ll, lr, false);
        ll = lr;
        while ll != ls_lend && ls_same(ll) {
            ll += ls_dx;
        }
    }
    if ll != ls_lend {
        ls_left(ll, lr, true);
    }
}

// B-226, line_smooth::line_smooth's mixes, made on each pixel in the CPU's order (`smoothscan`);
// a mixed pixel back through the curve, any other as it was.
@compute @workgroup_size(16, 16)
fn smoothmix(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let i = i32(id.y * size.x + id.x);
    var o = ls_px(i);
    var touched = false;
    for (var s = 0u; s < 8u; s++) {
        let v = row[8u * u32(i) + s];
        if bitcast<u32>(v) == 0u {
            continue;
        }
        touched = true;
        let off = select(1, i32(size.x), s < 4u);
        let b = ls_px(select(i - off, i + off, (s & 2u) != 0u));
        for (var c = 0u; c < 4u; c++) {
            o[c] = o[c] * (1.0 - v) + b[c] * v;
        }
    }
    if !touched {
        textureStore(output, id.xy, textureLoad(input, id.xy, 0));
        return;
    }
    var out = vec4(0.0);
    let a = o.w;
    if a > 0.0 {
        for (var c = 0u; c < 3u; c++) {
            out[c] = f32(to_linear(f64(o[c] / a))) * a;
        }
        out.w = a;
    }
    textureStore(output, id.xy, out);
}

@group(0) @binding(10) var effected: texture_2d<f32>;
@group(0) @binding(11) var<storage, read_write> frame_sum: array<vec4<f32>>;

// B-156, render::adjust_frame (D-66): the frame through the adjustment layer's stack, `effected`,
// mixed back into `frame_sum` by what the layer covers: its shape (`input`) through k[0..6], the
// inverse of its transform, by its opacity k[12], by its matte (`other`) through k[6..12]. At full
// cover the effected pixel exactly (D-90); otherwise `b + c * (e - b)` in single precision, the
// product rounded once as the CPU's is.
@compute @workgroup_size(16, 16)
fn adjust(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= F.base.x || id.y >= F.base.y {
        return;
    }
    let dx = f64(id.x) + 0.5lf;
    let dy = f64(id.y) + 0.5lf;
    var c = bilinear(input, k[0] * dx + k[2] * dy + k[4], k[1] * dx + k[3] * dy + k[5]).w * f32(k[12]);
    if F.flag == 1u {
        c *= bilinear(other, k[6] * dx + k[8] * dy + k[10], k[7] * dx + k[9] * dy + k[11]).w;
    }
    if c == 0.0 {
        return;
    }
    let at = id.y * F.base.x + id.x;
    let e = textureLoad(effected, vec2(i32(id.x) + F.ox, i32(id.y) + F.oy), 0);
    if c == 1.0 {
        frame_sum[at] = e;
        return;
    }
    let b = frame_sum[at];
    frame_sum[at] = b + vec4<f32>(f64(c) * vec4<f64>(e - b));
}

// B-221, effects::mix_back (D-202): an effect's result `input` laid over what the effect was given,
// `other`, placed at (F.ox, F.oy) in it and clear outside: `b + m (e - b)`, m = k[0] in single
// precision, the product rounded once as the CPU's is.
@compute @workgroup_size(16, 16)
fn mixback(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let e = textureLoad(input, id.xy, 0);
    let b = at(other, vec2<i32>(id.xy) - vec2(F.ox, F.oy));
    textureStore(output, id.xy, b + vec4<f32>(f64(f32(k[0])) * vec4<f64>(e - b)));
}
// D-352, matte_refine::choker_stage: each pixel's covering the average over the disc from each
// row's running totals in `sums` (outside counting as clear), through the ramp; its colour its
// own, or where it had none the disc's. k: the disc's pixel count, the ramp's centre and width,
// then each disc row's offset and half-width; `count` the rows.
@compute @workgroup_size(16, 16)
fn mchoke(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let w = i32(size.x);
    let h = i32(size.y);
    let x = i32(id.x);
    let y = i32(id.y);
    var t = vec4<f64>(0.0lf);
    for (var j = 0u; j < F.count; j++) {
        let sy = y + i32(k[3u + 2u * j]);
        let hw = i32(k[4u + 2u * j]);
        let x0 = max(x - hw, 0);
        let x1 = min(x + hw + 1, w);
        if sy < 0 || sy >= h || x0 >= x1 {
            continue;
        }
        let base = u32(sy * (w + 1));
        t += sums[base + u32(x1)] - sums[base + u32(x0)];
    }
    let c = k[1];
    let lo = min(max(c - k[2] / 2.0lf, 0.0lf), 1.0lf);
    let hi = min(max(c + k[2] / 2.0lf, 0.0lf), 1.0lf);
    let m = t.w / k[0];
    var a = 0.0lf;
    if hi > lo {
        a = min(max((m - lo) / (hi - lo), 0.0lf), 1.0lf);
    } else if m > c {
        a = 1.0lf;
    }
    let p = textureLoad(input, id.xy, 0);
    var s = vec3<f64>(0.0lf);
    if p.w > 0.0 {
        s = vec3<f64>(f64(p.x), f64(p.y), f64(p.z)) / f64(p.w);
    } else if t.w > 0.0lf {
        s = t.xyz / t.w;
    }
    textureStore(output, id.xy, vec4(f32(s.x * a), f32(s.y * a), f32(s.z * a), f32(a)));
}

// D-353, soft_glow::light in single precision as the CPU's: the light each pixel gives. k: the
// threshold, saturation bias and smooth as fractions, and 1 for luminance.
@compute @workgroup_size(16, 16)
fn sglight(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var rgb = textureLoad(input, id.xy, 0).xyz;
    let t = f32(k[0]);
    if t != 0.0 {
        let hi = max(max(rgb.x, rgb.y), rgb.z);
        let lo = min(min(rgb.x, rgb.y), rgb.z);
        var sat = 0.0;
        if hi > 0.0 {
            sat = (hi - lo) / hi;
        }
        var tested = rgb;
        if k[3] != 0.0lf {
            tested = vec3(0.2126 * rgb.x + 0.7152 * rgb.y + 0.0722 * rgb.z);
        }
        let b = f32(k[1]);
        let m = f32(k[2]);
        for (var c = 0; c < 3; c++) {
            var v = tested[c];
            if b > 0.0 {
                v = (1.0 - b) * v + b * sat;
            } else if b < 0.0 {
                v = (1.0 + b) * v - b * (1.0 - sat);
            }
            var w = 0.0;
            if m == 0.0 {
                w = select(0.0, 1.0, v >= t);
            } else {
                w = clamp((v - t * (1.0 - m)) / (t * m), 0.0, 1.0);
            }
            rgb[c] *= w;
        }
    }
    textureStore(output, id.xy, vec4(rgb, 0.0));
}

// D-353, soft_glow::spread's cells: the light averaged over each d by d block, in double
// precision, into a plane whose first cell is at (rx, ry). k: d, rx, ry, the plane's size.
@compute @workgroup_size(16, 16)
fn sgcells(@builtin(global_invocation_id) id: vec3<u32>) {
    if f64(id.x) >= k[3] || f64(id.y) >= k[4] {
        return;
    }
    let size = vec2<i32>(textureDimensions(input));
    let d = i32(k[0]);
    let ci = i32(id.x) - i32(k[1]);
    let cj = i32(id.y) - i32(k[2]);
    var s = vec3<f64>(0.0lf);
    if ci >= 0 && cj >= 0 && ci < (size.x + d - 1) / d && cj < (size.y + d - 1) / d {
        for (var y = cj * d; y < min((cj + 1) * d, size.y); y++) {
            for (var x = ci * d; x < min((ci + 1) * d, size.x); x++) {
                let p = textureLoad(input, vec2(x, y), 0);
                s += vec3<f64>(f64(p.x), f64(p.y), f64(p.z));
            }
        }
    }
    let area = f64(d * d);
    textureStore(output, id.xy, vec4(f32(s.x / area), f32(s.y / area), f32(s.z / area), 0.0));
}

// D-353, soft_glow::pass: one way through the plane, outside it clear. k: the plane's size, 1
// when down, then each tap's offset, cell aside and two weights; `count` the taps.
@compute @workgroup_size(16, 16)
fn sgpass(@builtin(global_invocation_id) id: vec3<u32>) {
    let pw = i32(k[0]);
    let ph = i32(k[1]);
    let x = i32(id.x);
    let y = i32(id.y);
    if x >= pw || y >= ph {
        return;
    }
    let down = k[2] != 0.0lf;
    var acc = vec3(0.0);
    for (var j = 0u; j < F.count; j++) {
        let t = i32(k[3u + 4u * j]);
        let lo = i32(k[4u + 4u * j]);
        let w0 = f32(k[5u + 4u * j]);
        let w1 = f32(k[6u + 4u * j]);
        var a = vec2(x + t, y + lo);
        var b = vec2(x + t, y + lo + 1);
        if down {
            a = vec2(x + lo, y + t);
            b = vec2(x + lo + 1, y + t);
        }
        if all(a >= vec2(0)) && a.x < pw && a.y < ph {
            acc += w0 * textureLoad(input, a, 0).xyz;
        }
        if w1 != 0.0 && all(b >= vec2(0)) && b.x < pw && b.y < ph {
            acc += w1 * textureLoad(input, b, 0).xyz;
        }
    }
    textureStore(output, id.xy, vec4(acc, 0.0));
}

// D-542, soft_glow::tinted.
fn sgtint(p: vec3<f32>, c: vec3<f32>, a: f32) -> vec3<f32> {
    let m = max(max(p.x, p.y), p.z);
    return (1.0 - a) * p + a * m * c;
}

// D-353, soft_glow::soft_glow's sum: the glow so far (`other`; on the first level, `flag`, the
// light at the layer's own place times its share) and, unless `count` is 0, one level's plane
// read between its cells. k: the level's weight, d, rx and ry, the grow, the light's own share
// and the plane's size; D-542: then 1 when tinted, Tint Amount, the level's colour and the inner
// colour (the own share's), linear; D-543: then the channels this level gives (1 or 0 each) and
// the light's own share for each channel (its chain's).
@compute @workgroup_size(16, 16)
fn sgadd(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let g = i32(k[4]);
    let x = i32(id.x) - g;
    let y = i32(id.y) - g;
    var sum = vec3(0.0);
    let tint = k[8] != 0.0lf;
    let amount = f32(k[9]);
    let mask = vec3(f32(k[16]), f32(k[17]), f32(k[18]));
    if F.flag == 1u {
        let own = vec3(f32(k[19]), f32(k[20]), f32(k[21]));
        if max(max(own.x, own.y), own.z) > 0.0 {
            var l = at(other, vec2(x, y)).xyz;
            if tint {
                l = sgtint(l, vec3(f32(k[13]), f32(k[14]), f32(k[15])), amount);
            }
            sum = own * l;
        }
    } else {
        sum = textureLoad(other, id.xy, 0).xyz;
    }
    if F.count > 0u {
        let d = f32(k[1]);
        let rx = i32(k[2]);
        let ry = i32(k[3]);
        let pw = i32(k[6]);
        let ph = i32(k[7]);
        let weight = f32(k[0]);
        let scale = select(weight, 1.0, tint);
        var lvl = vec3(0.0);
        let u = (f32(x) + 0.5) / d - 0.5;
        let v = (f32(y) + 0.5) / d - 0.5;
        let u0 = floor(u);
        let v0 = floor(v);
        let fu = u - u0;
        let fv = v - v0;
        for (var dj = 0; dj < 2; dj++) {
            let wj = select(1.0 - fv, fv, dj == 1);
            let cy = i32(v0) + dj + ry;
            if wj == 0.0 || cy < 0 || cy >= ph {
                continue;
            }
            for (var di = 0; di < 2; di++) {
                let wi = select(1.0 - fu, fu, di == 1);
                let cx = i32(u0) + di + rx;
                if wi == 0.0 || cx < 0 || cx >= pw {
                    continue;
                }
                let s = scale * wj * wi * textureLoad(input, vec2(cx, cy), 0).xyz;
                if tint {
                    lvl += s;
                } else {
                    sum += mask * s;
                }
            }
        }
        if tint {
            sum += mask * (weight * sgtint(lvl, vec3(f32(k[10]), f32(k[11]), f32(k[12])), amount));
        }
    }
    textureStore(output, id.xy, vec4(sum, 0.0));
}

// D-353, soft_glow::finish: the glow times Exposure laid with the untouched layer (`other`, at
// the grow). k: the grow, Exposure, Source Opacity as a fraction, 1 for screen, 1 for unmult.
@compute @workgroup_size(16, 16)
fn sgfinish(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let screen = k[3] != 0.0lf;
    let o = f32(k[2]);
    var g = textureLoad(input, id.xy, 0).xyz * f32(k[1]);
    if screen {
        g = min(g, vec3(1.0));
    }
    var ga = 1.0;
    if k[4] != 0.0lf {
        ga = min(max(max(g.x, g.y), g.z), 1.0);
    }
    let p = at(other, vec2<i32>(id.xy) - vec2(i32(k[0])));
    var v = g + o * p.xyz;
    if screen {
        v = min(v, vec3(1.0));
    }
    textureStore(output, id.xy, vec4(v, min(ga + o * p.w, 1.0)));
}

// D-544, soft_glow::soft_glow's halation: the spread's Rec. 709 brightness past the light's own
// at the pixel (`other`, at the grow; 0 outside the layer), in Halation's colour. k: the grow,
// then the colour (linear).
@compute @workgroup_size(16, 16)
fn sghalo(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let s = textureLoad(input, id.xy, 0).xyz;
    let l = at(other, vec2<i32>(id.xy) - vec2(i32(k[0]))).xyz;
    let h = max((0.2126 * s.x + 0.7152 * s.y + 0.0722 * s.z) - (0.2126 * l.x + 0.7152 * l.y + 0.0722 * l.z), 0.0);
    textureStore(output, id.xy, vec4(h * vec3(f32(k[1]), f32(k[2]), f32(k[3])), 0.0));
}

// D-545, soft_glow::finish's dirt: the glow times Exposure with the dirt on top, for sgfinish at
// Exposure 1. k: Exposure, then the dirt's (dirtlit).
@compute @workgroup_size(16, 16)
fn sgdirt(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let g = textureLoad(input, id.xy, 0).xyz * f32(k[0]);
    textureStore(output, id.xy, vec4(dirtlit(g, vec2<i32>(id.xy), 1u), 0.0));
}

// D-545, layer_fx::lens_flare with dirt: the flare's light with the dirt on top, then laid as
// case 36 lays it. k: case 36's, then the dirt's (dirtlit) after the parts.
@compute @workgroup_size(16, 16)
fn flaredirt(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(output);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let light = dirtlit(flare_light(f64(id.x) + 0.5lf, f64(id.y) + 0.5lf), vec2<i32>(id.xy), 2u + 9u * u32(k[1]));
    textureStore(output, id.xy, flare_lay(textureLoad(input, id.xy, 0), light));
}
"#;

/// D-545: Lens Dirt's numbers for FX_SHADER's `dirtlit`, the layer `w` by `h` grown by `g` on
/// every side, the dirt centred on the layer as `soft_glow::Dirt::at` places it.
fn dirt_k(d: &crate::soft_glow::Dirt, (w, h): (usize, usize), g: usize) -> [f64; 7] {
    let corner =
        |n: usize, dn: usize| (g as isize + (n as isize - dn as isize).div_euclid(2)) as f64;
    [
        f64::from(d.opacity),
        f64::from(d.inherit),
        f64::from(d.gain),
        f64::from(d.power),
        f64::from(u8::from(d.repeat)),
        corner(w, d.picture.width()),
        corner(h, d.picture.height()),
    ]
}

/// B-65: [`FX_SHADER`]'s numbers, laid out as its `Fx`; each pass reads what it needs.
#[repr(C)]
#[derive(Clone, Copy, Default, bytemuck::Pod, bytemuck::Zeroable)]
struct FxParams {
    mode: u32,
    blend: u32,
    count: u32,
    flag: u32,
    g: i32,
    r: i32,
    n: u32,
    frame: i32,
    base: [u32; 2],
    ox: i32,
    oy: i32,
}

/// B-65: [`FX_SHADER`]'s passes.
struct FxPasses {
    grade: Pass,
    aberr: Pass,
    shadow: Pass,
    rim: Pass,
    rows: Pass,
    bands: Pass,
    lift: Pass,
    ring: Pass,
    prefix: Pass,
    gather: Pass,
    /// D-359.
    lmgather: Pass,
    /// B-76.
    turb: Pass,
    slide: Pass,
    rays: Pass,
    cols: Pass,
    edt: Pass,
    wrap: Pass,
    /// B-107.
    tone: Pass,
    warp: Pass,
    wipe: Pass,
    blocks: Pass,
    tiles: Pass,
    relief: Pass,
    sharp: Pass,
    /// D-400.
    shprep: Pass,
    shhi: Pass,
    choke: Pass,
    lines: Pass,
    glare: Pass,
    rain: Pass,
    /// B-123.
    line_blur: Pass,
    stars: Pass,
    kira: Pass,
    /// D-551.
    balls: Pass,
    ballshade: Pass,
    /// D-610.
    particles: Pass,
    /// D-671.
    bubbles: Pass,
    /// D-672.
    drizzle: Pass,
    /// D-673.
    hair: Pass,
    /// D-674.
    mercury: Pass,
    /// D-675.
    shards: Pass,
    /// D-678.
    foam: Pass,
    /// D-676.
    scatterize: Pass,
    /// D-677.
    star_burst: Pass,
    /// B-151.
    median: Pass,
    smart: Pass,
    /// B-237.
    bilat: Pass,
    degrain: Pass,
    /// B-336.
    deband: Pass,
    rough: Pass,
    rshadow: Pass,
    bevel: Pass,
    snow: Pass,
    cells: Pass,
    /// B-156.
    adjust: Pass,
    /// B-172: [`chain_shader`]'s one pass.
    chain: Pass,
    /// B-221.
    mix_back: Pass,
    /// B-223.
    chanmix: Pass,
    crossmix: Pass,
    selprep: Pass,
    selpass: Pass,
    selfinish: Pass,
    vheight: Pass,
    vblur: Pass,
    csigma: Pass,
    cmix: Pass,
    /// B-224.
    dmap: Pass,
    /// D-458.
    wire: Pass,
    glass: Pass,
    /// D-379.
    blobover: Pass,
    blobby: Pass,
    /// B-225.
    beam: Pass,
    gradient4: Pass,
    sweepmin: Pass,
    sweep: Pass,
    /// D-345.
    waves: Pass,
    bolt: Pass,
    /// B-235.
    stroke: Pass,
    /// D-420.
    marks: Pass,
    /// D-444.
    vegas: Pass,
    /// D-446.
    curlfield: Pass,
    curlflow: Pass,
    /// D-583.
    curlsoft: Pass,
    /// D-447.
    brush: Pass,
    /// D-448.
    cartoonedge: Pass,
    cartoon: Pass,
    /// D-508.
    energy: Pass,
    /// D-449.
    blockload: Pass,
    /// D-509.
    gmeltk: Pass,
    gmelt: Pass,
    /// D-460.
    burnfilm: Pass,
    /// D-462.
    smoothie: Pass,
    /// D-463.
    plastic: Pass,
    /// D-467.
    texturize: Pass,
    /// D-470.
    cptensor: Pass,
    cptangent: Pass,
    cpsmear: Pass,
    cpfill: Pass,
    cpedge: Pass,
    /// D-517.
    twist: Pass,
    /// D-518.
    wreact: Pass,
    warpb: Pass,
    /// D-441.
    writeon: Pass,
    edges: Pass,
    /// B-226.
    dissolve: Pass,
    gwipe: Pass,
    /// D-403.
    haze: Pass,
    /// D-524.
    fog: Pass,
    /// D-526.
    over: Pass,
    /// D-501.
    dmatte: Pass,
    /// D-506.
    glasses: Pass,
    /// D-507.
    spotlight: Pass,
    /// D-530.
    cylinder: Pass,
    /// D-532.
    sphere: Pass,
    /// D-580.
    cardwipe: Pass,
    /// D-531.
    environment: Pass,
    lwidth: Pass,
    /// B-228.
    passx: Pass,
    dkey: Pass,
    /// B-229.
    idkey: Pass,
    fill: Pass,
    eyerow: Pass,
    eyesum: Pass,
    eyefill: Pass,
    /// D-440.
    bucket: Pass,
    smoothscan: Pass,
    smoothmix: Pass,
    /// D-352.
    mchoke: Pass,
    /// D-353.
    sglight: Pass,
    sgcells: Pass,
    sgpass: Pass,
    sgadd: Pass,
    sgfinish: Pass,
    /// D-544.
    sghalo: Pass,
    /// D-545.
    sgdirt: Pass,
    flaredirt: Pass,
    /// D-388.
    pageturn: Pass,
}

/// B-172: one colour effect of a run the card draws in one pass: `grade` (0) or `tone` (1), its
/// numbers and its settings; B-221, and its Mix, 0 to 1.
type Staged = (u32, FxParams, Vec<f64>, f32);

/// B-172: [`FX_SHADER`] with `grade` and `tone` made functions of one pixel, which read their
/// numbers from `F` and their settings from `k` at `KO`, both set for each effect of a run; and
/// `chain`, which runs a run's effects on each pixel one after another, handing each the pixel
/// the one before it made, as the texture between them held it. The same maths in the same order.
fn chain_shader() -> String {
    let s = FX_SHADER.replace(
        "@group(0) @binding(0) var<uniform> F: Fx;",
        "@group(0) @binding(0) var<uniform> RUN: Fx;\nvar<private> F: Fx;\nvar<private> KO: u32;",
    );
    // Every setting read from where this effect's begin.
    let parts: Vec<&str> = s.split("k[").collect();
    let mut s = parts[0].to_string();
    for (before, part) in parts.iter().zip(&parts[1..]) {
        let word = before
            .chars()
            .last()
            .is_some_and(|c| c.is_alphanumeric() || c == '_');
        s.push_str(if word { "k[" } else { "k[KO + " });
        s.push_str(part);
    }
    for name in ["grade", "tone"] {
        let head = format!(
            "@compute @workgroup_size(16, 16)\nfn {name}(@builtin(global_invocation_id) id: vec3<u32>) {{\n    let size = textureDimensions(input);\n    if id.x >= size.x || id.y >= size.y {{\n        return;\n    }}\n    let p = textureLoad(input, id.xy, 0);\n"
        );
        let start = s.find(&head).expect("B-172: the pass begins as it did");
        let end = start + s[start..].find("\n}\n").expect("B-172: the pass ends") + 3;
        // Each store of the pixel, and the return after it, a return of the pixel.
        let mut f = format!("fn {name}_at(id: vec3<u32>, p: vec4<f32>) -> vec4<f32> {{\n");
        let mut rest = &s[start + head.len()..end];
        let store = "textureStore(output, id.xy, ";
        while let Some(i) = rest.find(store) {
            f.push_str(&rest[..i]);
            rest = &rest[i + store.len()..];
            let j = rest.find(");\n").expect("B-172: a store ends its line");
            f.push_str(&format!("return {};", &rest[..j]));
            rest = &rest[j + 2..];
            if let Some(after) = rest.trim_start().strip_prefix("return;") {
                rest = after;
            }
        }
        f.push_str(rest);
        assert!(
            !f.contains("texture"),
            "B-172: {name} reads or writes a texture other than its pixel"
        );
        s.replace_range(start..end, &f);
    }
    s + "
struct Stage {
    f: Fx,
    which: u32,
    ko: u32,
    mix: f32,
}

@group(0) @binding(12) var<storage, read> stages: array<Stage>;

@compute @workgroup_size(16, 16)
fn chain(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    var p = textureLoad(input, id.xy, 0);
    for (var i = 0u; i < RUN.count; i++) {
        F = stages[i].f;
        KO = stages[i].ko;
        let was = p;
        if stages[i].which == 0u {
            p = grade_at(id, p);
        } else {
            p = tone_at(id, p);
        }
        // B-221: mix_back's `b + m (e - b)`, the product rounded once.
        if stages[i].mix < 1.0 {
            p = was + vec4<f32>(f64(stages[i].mix) * vec4<f64>(p - was));
        }
    }
    textureStore(output, id.xy, p);
}
"
}

/// B-172: whether the card draws `effect` from its own pixel alone, through `grade` or `tone`, so
/// that a run of them can be drawn in one pass.
fn one_pixel(effect: &crate::effects::Effect) -> bool {
    use crate::effects::Effect as E;
    matches!(
        effect,
        E::Curves { .. }
            | E::Levels { .. }
            | E::ChannelLevels { .. }
            | E::HueSaturation { .. }
            | E::Gradient { .. }
            | E::Noise { .. }
            | E::AddGrain { .. }
            | E::NoiseAlpha { .. }
            | E::NoiseHls { .. }
            | E::NoiseHlsAuto { .. }
            | E::ExposureFlicker { .. }
            | E::ColorBalance { .. }
            | E::GradientMap { .. }
            | E::TintMap { .. }
            | E::Tritone { .. }
            | E::Vignette { .. }
            | E::FractalNoise { .. }
            | E::TurbulentNoise { .. }
            | E::Invert { .. }
            | E::BrightnessContrast { .. }
            | E::BlackWhite { .. }
            | E::Posterize { .. }
            | E::Threshold { .. }
            | E::ChannelMixer { .. }
            | E::Vibrance { .. }
            | E::LeaveColor { .. }
            | E::Solarize { .. }
            | E::Halftone { .. }
            | E::ColorLookup { .. }
            | E::ArbitraryMap { .. }
            | E::HsvKey { .. }
            | E::Paraffin { .. }
            // B-222.
            | E::Exposure { .. }
            | E::Tint { .. }
            | E::ShiftChannels { .. }
            | E::SolidComposite { .. }
            | E::ChangeToColor { .. }
            | E::ColorKey { .. }
            | E::SelectColor { .. }
            | E::LineRecolor { .. }
            | E::Colorama { .. }
            | E::Extract { .. }
            // D-365..D-367.
            | E::BroadcastSafe { .. }
            | E::ColorNeutralizer { .. }
            | E::ColorOffset { .. }
            // D-369/D-370.
            | E::Toner { .. }
            | E::ChangeColor { .. }
            // D-374/D-375.
            | E::ColorBalanceHls { .. }
            | E::ColorLink { .. }
            // D-382.
            | E::GammaPedestalGain { .. }
            // D-384.
            | E::PhotoFilter { .. }
            // D-396.
            | E::SelectiveColor { .. }
            // D-457.
            | E::AdvancedSpillSuppressor { .. }
            // D-459.
            | E::ColorDifferenceKey { .. }
            // D-500.
            | E::ColorRange { .. }
            // D-504.
            | E::LinearColorKey { .. }
            // D-505.
            | E::Unmult { .. }
            // D-540.
            | E::ScreenKey { .. }
            // D-548.
            | E::SpillSuppressor { .. }
    )
}

/// B-76: Distance Gradation's work for a drawing `w` by `h`: the columns' distances over the
/// drawing inside its ring of edge, then each row's roots.
fn dist_bytes(w: usize, h: usize) -> usize {
    ((w + 2) * (h + 2) + h * (w + 2)) * 8
}

/// B-76: the layer's Light Wraps that run, as `render::wrap_layer` runs them: width, intensity
/// and Add.
fn wraps(layer: &crate::render::LayerDraw) -> Vec<(f64, f64, bool)> {
    layer
        .wrap
        .iter()
        .filter(|i| i.enabled && i.is_valid())
        .filter_map(|i| match &i.effect {
            crate::effects::Effect::LightWrap {
                width,
                intensity,
                blend,
            } => Some((*width, *intensity, blend == "add")),
            _ => None,
        })
        .collect()
}

/// B-45: the window's pixels the page leaves see-through, painted as the page would have.
const SCREEN_SHADER: &str = r#"
struct Paint {
    rect: vec4<f32>,
    origin: vec4<f32>,
    colour: vec4<f32>,
    colour2: vec4<f32>,
    kind: u32,
    square: f32,
    pad0: u32,
    pad1: u32,
}

struct Screen {
    count: u32,
    width: u32,
    height: u32,
    alpha_only: u32,
}

@group(0) @binding(0) var<uniform> V: Screen;
@group(0) @binding(1) var<storage, read> paints: array<Paint>;
@group(0) @binding(2) var<storage, read> picture: array<u32>;

// One triangle that covers the window.
@vertex
fn corner(@builtin(vertex_index) i: u32) -> @builtin(position) vec4<f32> {
    let x = f32((i << 1u) & 2u);
    let y = f32(i & 2u);
    return vec4<f32>(x * 2.0 - 1.0, 1.0 - y * 2.0, 0.0, 1.0);
}

// `at` is the pixel's centre, as the page's own pixels are. Each paint covers what is under it,
// and the picture is blended over it as a canvas is: eight-bit straight alpha, on sRGB numbers.
@fragment
fn paint(@builtin(position) at: vec4<f32>) -> @location(0) vec4<f32> {
    let p = at.xy;
    var c = vec3<f32>(0.0);
    for (var i = 0u; i < V.count; i++) {
        let it = paints[i];
        if p.x < it.rect.x || p.y < it.rect.y || p.x >= it.rect.z || p.y >= it.rect.w {
            continue;
        }
        if it.kind == 0u {
            c = it.colour.rgb;
        } else if it.kind == 1u {
            // The checkerboard: `colour2` where the squares' row and column add up to odd.
            let q = vec2<i32>(floor((p - it.origin.xy) / it.square));
            c = select(it.colour.rgb, it.colour2.rgb, ((q.x + q.y) & 1) == 1);
        } else {
            // The nearest picture pixel, as `image-rendering: pixelated` picks it. A window pixel
            // whose centre falls exactly between two is given the first, as the page gives it:
            // measured at a fit of 456 for 480, where every nineteenth column is such a tie.
            let box = it.origin.zw - it.origin.xy;
            let f = floor((p - it.origin.xy) * vec2<f32>(f32(V.width), f32(V.height)) / box - 0.001);
            let t = vec2<u32>(clamp(f, vec2<f32>(0.0), vec2<f32>(f32(V.width - 1u), f32(V.height - 1u))));
            let b = picture[t.y * V.width + t.x];
            var s = vec4<f32>(f32(b & 255u), f32((b >> 8u) & 255u), f32((b >> 16u) & 255u), f32(b >> 24u)) / 255.0;
            if V.alpha_only != 0u {
                s = vec4<f32>(s.a, s.a, s.a, 1.0);
            }
            c = s.rgb * s.a + c * (1.0 - s.a);
        }
    }
    return vec4<f32>(c, 1.0);
}
"#;

/// B-45: one box of the window the card paints, in window pixels, as the page would paint it.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Paint {
    /// Left, top, right, bottom of what shows: the box cut to whatever scrolls or hides it.
    pub rect: [f32; 4],
    /// The whole box, uncut: where the checkerboard's squares start and where the picture lies.
    pub origin: [f32; 4],
    /// Red, green, blue, 0 to 1, as the page's colour numbers are: sRGB, not linear.
    pub colour: [f32; 4],
    /// The checkerboard's other colour.
    pub colour2: [f32; 4],
    /// [`Paint::COLOUR`], [`Paint::CHECKERBOARD`] or [`Paint::PICTURE`].
    pub kind: u32,
    /// The checkerboard's square, in window pixels.
    pub square: f32,
    pub pad: [u32; 2],
}

impl Paint {
    pub const COLOUR: u32 = 0;
    pub const CHECKERBOARD: u32 = 1;
    pub const PICTURE: u32 = 2;
}

/// B-45: the window the card paints into.
struct Screen {
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    layout: wgpu::BindGroupLayout,
}

/// One drawing on the card.
struct Stored {
    held: Weak<WorkingBuffer>,
    /// The CPU cache's name for it, which outlives `held` (B-44b).
    name: Option<Name>,
    view: wgpu::TextureView,
    bytes: usize,
    /// The frame that last drew with it.
    used: u64,
    /// B-46, B-47: the last effects the card ran on it (B-155: a run of them), kept while the
    /// settings stay the same, with the picture after each, so a run whose last effect alone
    /// changes starts again from the one before it.
    /// B-172: none after an effect a run drawn in one pass went on from.
    applied: Option<(
        Vec<OnCard>,
        Vec<Option<(wgpu::TextureView, (usize, usize))>>,
    )>,
    /// B-173 (D-246): the last picture of each run drawn on it before `applied`, with the frame
    /// that last used it, so a run whose settings move from frame to frame is not drawn again
    /// when a frame comes round again, as the CPU's effect cache keeps each frame's.
    before: Vec<(Vec<OnCard>, wgpu::TextureView, (usize, usize), u64)>,
    /// B-47: held in 32-bit floats, for a Bloom.
    wide: bool,
}

/// B-153: the memory drawings are sent to the card through, kept from frame to frame rather than
/// made for each drawing. Making it anew, and the card taking in memory it has not seen before,
/// cost more than the copy.
#[derive(Default)]
struct Sending {
    buffer: Option<wgpu::Buffer>,
    /// Bytes this frame has put in it.
    filled: u64,
    /// Open for the processor to write.
    open: bool,
    /// The most one frame has sent, which the next buffer is made to hold.
    most: u64,
    /// The last frame that copied from it, which must finish before it is written again.
    after: Option<wgpu::SubmissionIndex>,
}

/// The buffers one frame size needs, kept while frames stay that size.
struct Target {
    width: usize,
    height: usize,
    sum: wgpu::Buffer,
    bytes: wgpu::Buffer,
    readback: wgpu::Buffer,
    size: wgpu::Buffer,
}

pub struct Gpu {
    instance: wgpu::Instance,
    adapter: wgpu::Adapter,
    /// B-45: the window, once the card paints into it.
    screen: Option<Screen>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    about: String,
    limits: wgpu::Limits,
    layer: wgpu::ComputePipeline,
    /// B-156b: one moment of a motion-blurred layer, added up.
    moment: wgpu::ComputePipeline,
    encode: wgpu::ComputePipeline,
    layer_layout: wgpu::BindGroupLayout,
    encode_layout: wgpu::BindGroupLayout,
    /// B-46.
    radial: wgpu::ComputePipeline,
    radial_layout: wgpu::BindGroupLayout,
    /// B-76: a layer with a Light Wrap, laid alone, and its light.
    unpack: wgpu::ComputePipeline,
    unpack_layout: wgpu::BindGroupLayout,
    /// B-156: an adjustment layer's frame taken for its stack to run on.
    take: Pass,
    /// B-156: the last opaque shape an adjustment layer had, kept so it is sent once.
    ones: Option<Arc<WorkingBuffer>>,
    /// B-47: `None` on a card without double precision.
    bloom: Option<BloomPasses>,
    /// B-65: `None` on a card without double precision.
    fx: Option<FxPasses>,
    /// Bound as the matte of a layer that has none. Never read.
    no_matte: wgpu::TextureView,
    store: Vec<Stored>,
    frame: u64,
    target: Option<Target>,
    /// Bytes of drawings the card may hold; public so the B-44 test can squeeze it, and the
    /// window can set it (B-48, D-105).
    pub budget: usize,
    /// The card's own memory, when Windows says.
    memory: Option<u64>,
    /// Drawings sent to the card since it was opened.
    sent: u64,
    /// Frames the card failed since it was opened (B-48).
    failures: u64,
    /// B-153.
    sending: Sending,
    /// B-153: textures of drawings nothing can ask for again, oldest first, kept within the
    /// budget for new drawings of the same size.
    spare: Vec<wgpu::Texture>,
    /// B-153: textures and sending memory made since the card was opened.
    made: Cell<u64>,
    /// B-164: blurs worked small and enlarged since the card was opened.
    shrunk: Cell<u64>,
    /// B-172: passes run since the card was opened, and whether a run of colour effects is drawn
    /// in one.
    dispatched: u64,
    fused: bool,
    /// B-153: the passes' working textures, each with the last frame that used it, kept for the
    /// next frame's passes of the same size, when the card can clear them to what a new one holds.
    working: Option<RefCell<Vec<(wgpu::Texture, u64)>>>,
    /// B-153: working textures lent again this frame, cleared before the passes run.
    to_clear: RefCell<Vec<wgpu::Texture>>,
    /// Audit P9: Curl Noise's fields worked from its own noise, by the numbers that made them,
    /// each with the last frame that used it. The field does not depend on the picture, so it
    /// is drawn again only when those numbers (its settings, or the time when it moves) change.
    fields: RefCell<Vec<(Vec<u64>, wgpu::TextureView, u64)>>,
}

fn entry(binding: u32, ty: wgpu::BindingType) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::COMPUTE,
        ty,
        count: None,
    }
}

fn uniform() -> wgpu::BindingType {
    wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Uniform,
        has_dynamic_offset: false,
        min_binding_size: None,
    }
}

fn storage(read_only: bool) -> wgpu::BindingType {
    wgpu::BindingType::Buffer {
        ty: wgpu::BufferBindingType::Storage { read_only },
        has_dynamic_offset: false,
        min_binding_size: None,
    }
}

fn storage_texture() -> wgpu::BindingType {
    wgpu::BindingType::StorageTexture {
        access: wgpu::StorageTextureAccess::WriteOnly,
        format: wgpu::TextureFormat::Rgba32Float,
        view_dimension: wgpu::TextureViewDimension::D2,
    }
}

fn texture() -> wgpu::BindingType {
    wgpu::BindingType::Texture {
        sample_type: wgpu::TextureSampleType::Float { filterable: false },
        view_dimension: wgpu::TextureViewDimension::D2,
        multisampled: false,
    }
}

/// The frame the viewer asked the card for goes to the CPU, and why.
fn on_cpu(severity: Severity, message: String, detail: String) -> Diagnostic {
    Diagnostic::new(DiagnosticId::GpuPreviewOnCpu, severity, message, detail)
}

/// The effects `Gpu::fx` never draws: those the card's own passes draw (Radial, Spin & Zoom,
/// Fast Zoom, Directional and Gaussian Blur, Bloom, Glow), those drawn as another (`drawn_as`)
/// and those compose never hands the card. Named, not left to a `_`, so a new effect is either
/// drawn by `fx` or put here before this builds.
macro_rules! not_fx {
    () => {
        crate::effects::Effect::AutoTone { .. }
            | crate::effects::Effect::Bloom { .. }
            | crate::effects::Effect::CardDance { .. }
            | crate::effects::Effect::Caustics { .. }
            | crate::effects::Effect::CcRainfall { .. }
            | crate::effects::Effect::CcWideTime { .. }
            | crate::effects::Effect::ColorStabilizer { .. }
            | crate::effects::Effect::Compressor { .. }
            | crate::effects::Effect::DepthOfField { .. }
            | crate::effects::Effect::DirectionalBlur { .. }
            | crate::effects::Effect::Distortion { .. }
            | crate::effects::Effect::Echo { .. }
            | crate::effects::Effect::FastZoomBlur { .. }
            | crate::effects::Effect::ForceMotionBlur { .. }
            | crate::effects::Effect::GaussianBlur { .. }
            | crate::effects::Effect::GaussianBlurLegacy { .. }
            | crate::effects::Effect::Glow { .. }
            | crate::effects::Effect::InnerOuterKey { .. }
            | crate::effects::Effect::Kaleidoscope { .. }
            | crate::effects::Effect::KeyCleaner { .. }
            | crate::effects::Effect::LightWrap { .. }
            | crate::effects::Effect::MomentMap { .. }
            | crate::effects::Effect::NoiseGate { .. }
            | crate::effects::Effect::PixelMotionBlur { .. }
            | crate::effects::Effect::PosterizeTime { .. }
            | crate::effects::Effect::RadialBlur { .. }
            | crate::effects::Effect::ReduceInterlaceFlicker { .. }
            | crate::effects::Effect::RefineMatte { .. }
            | crate::effects::Effect::RollingShutterRepair { .. }
            | crate::effects::Effect::Shatter { .. }
            | crate::effects::Effect::SpillSuppressor { .. }
            | crate::effects::Effect::SpinZoomBlur { .. }
            | crate::effects::Effect::SpreadTones { .. }
            | crate::effects::Effect::TextAnimator { .. }
            | crate::effects::Effect::TimeDifference { .. }
            | crate::effects::Effect::Timewarp { .. }
            | crate::effects::Effect::TurbulentNoise { .. }
            | crate::effects::Effect::Unsupported { .. }
            | crate::effects::Effect::WarpStabilizer { .. }
            | crate::effects::Effect::WaveWorld { .. }
    };
}

/// The effect `Gpu::fx` draws `effect` as, when it is another's. D-445: Turbulent Noise is
/// Fractal Noise's pass. D-454: a Match Grain is drawn as the Add Grain it lays; compose left out
/// one that lays none. D-523: Depth of Field as its Lens Blur. D-548: Spill Suppressor as its
/// Screen Key.
fn drawn_as(effect: &crate::effects::Effect) -> Option<crate::effects::Effect> {
    crate::effects::turbulent_fractal(effect)
        .or_else(|| crate::grade::matched_grain(effect))
        .or_else(|| crate::effects::depth_lens(effect))
        .or_else(|| crate::effects::second_name(effect))
}

/// Whether `Gpu::fx` draws `effect`, for the app's test that every effect compose hands the card
/// is one the card draws.
#[doc(hidden)]
pub fn fx_draws(effect: &crate::effects::Effect) -> bool {
    !matches!(drawn_as(effect).as_ref().unwrap_or(effect), not_fx!())
}

impl Gpu {
    /// The fastest Direct3D 12 or Vulkan card on the machine, or why there is none.
    pub fn new() -> Result<Gpu, String> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
            backends: wgpu::Backends::DX12 | wgpu::Backends::VULKAN,
            ..Default::default()
        });
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        }))
        .map_err(|e| format!("no Direct3D 12 or Vulkan graphics card answered ({e})"))?;
        let info = adapter.get_info();
        if info.device_type == wgpu::DeviceType::Cpu {
            return Err(format!(
                "the only one found, {}, is the processor pretending to be a card",
                info.name
            ));
        }
        let limits = adapter.limits();
        // B-47: Bloom's streaks need double precision. Vulkan offers it on this machine's card;
        // Direct3D 12 does not.
        let f64 = adapter.features() & wgpu::Features::SHADER_F64;
        // B-153: clearing a working texture, to lend it again.
        let clear = adapter.features().contains(wgpu::Features::CLEAR_TEXTURE);
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("B-44 preview"),
            required_features: f64 | (adapter.features() & wgpu::Features::CLEAR_TEXTURE),
            required_limits: limits.clone(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .map_err(|e| format!("{} would not start ({e})", info.name))?;
        // Every call below runs inside error scopes, so this should never be reached. The
        // default would end the program.
        device.on_uncaptured_error(Box::new(|e| eprintln!("B-44: the card reported {e}")));

        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("B-44"),
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layer_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("B-44 layer"),
            entries: &[
                entry(0, uniform()),
                entry(1, texture()),
                entry(2, texture()),
                entry(3, storage(false)),
            ],
        });
        let encode_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("B-44 encode"),
            entries: &[
                entry(0, uniform()),
                entry(1, storage(true)),
                entry(2, storage(false)),
            ],
        });
        let unpack_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("B-76 unpack"),
            entries: &[
                entry(0, uniform()),
                entry(1, storage(true)),
                entry(2, storage(true)),
                entry(3, storage_texture()),
                entry(4, storage_texture()),
            ],
        });
        let radial_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("B-46 radial"),
            entries: &[
                entry(0, uniform()),
                entry(1, texture()),
                entry(2, storage_texture()),
                entry(3, storage(true)),
            ],
        });
        let pipeline_in =
            |module: &wgpu::ShaderModule, layout: &wgpu::BindGroupLayout, entry_point: &str| {
                device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                    label: Some(entry_point),
                    layout: Some(
                        &device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                            label: None,
                            bind_group_layouts: &[layout],
                            push_constant_ranges: &[],
                        }),
                    ),
                    module,
                    entry_point: Some(entry_point),
                    compilation_options: Default::default(),
                    cache: None,
                })
            };
        let pipeline = |layout: &wgpu::BindGroupLayout, entry_point: &str| {
            pipeline_in(&module, layout, entry_point)
        };
        let bloom = (!f64.is_empty()).then(|| {
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("B-47"),
                source: wgpu::ShaderSource::Wgsl(BLOOM_SHADER.into()),
            });
            // Each pass's layout holds only the bindings it reads, so none needs a stand-in.
            let pass = |entry_point: &str, bindings: &[u32]| {
                let entries: Vec<_> = bindings
                    .iter()
                    .map(|&b| {
                        entry(
                            b,
                            [
                                uniform(),
                                texture(),
                                storage_texture(),
                                storage(false),
                                storage(true),
                            ][b as usize],
                        )
                    })
                    .collect();
                let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(entry_point),
                    entries: &entries,
                });
                (pipeline_in(&module, &layout, entry_point), layout)
            };
            BloomPasses {
                bright: pass("bright", &[0, 1, 2, 4]),
                gauss: pass("gauss", &[0, 1, 2, 4]),
                add: pass("add", &[0, 1, 3]),
                streak: pass("streak", &[0, 1, 2, 4]),
                mix: pass("mix", &[0, 1, 3]),
                lay: pass("lay", &[0, 1, 2]),
                combine: pass("combine", &[0, 1, 2, 3]),
                shrink: pass("shrink", &[0, 1, 2]),
                enlarge: pass("enlarge", &[0, 1, 2]),
            }
        });
        let fx = (!f64.is_empty()).then(|| {
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("B-65"),
                source: wgpu::ShaderSource::Wgsl(FX_SHADER.into()),
            });
            let pass = |entry_point: &str, bindings: &[u32]| {
                let types = [
                    uniform(),
                    texture(),
                    storage_texture(),
                    storage(true),
                    texture(),
                    storage(false),
                    storage(false),
                    storage(false),
                    storage(false),
                    storage(false),
                    texture(),
                    storage(false),
                ];
                let entries: Vec<_> = bindings
                    .iter()
                    .map(|&b| entry(b, types[b as usize]))
                    .collect();
                let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some(entry_point),
                    entries: &entries,
                });
                (pipeline_in(&module, &layout, entry_point), layout)
            };
            FxPasses {
                grade: pass("grade", &[0, 1, 2, 3]),
                aberr: pass("aberr", &[0, 1, 2, 3]),
                shadow: pass("shadow", &[0, 1, 2, 3, 4]),
                rim: pass("rim", &[0, 1, 2, 3, 4]),
                rows: pass("rows", &[0, 1, 5]),
                bands: pass("bands", &[0, 1, 3, 5, 6]),
                lift: pass("lift", &[0, 2, 6]),
                ring: pass("ring", &[0, 1, 2, 3, 4]),
                prefix: pass("prefix", &[0, 1, 3, 7]),
                gather: pass("gather", &[0, 1, 2, 3, 7]),
                lmgather: pass("lmgather", &[0, 1, 2, 3, 4, 7]),
                turb: pass("turb", &[0, 1, 2, 3]),
                slide: pass("slide", &[0, 1, 2, 3]),
                rays: pass("rays", &[0, 1, 2, 3, 4]),
                cols: pass("cols", &[0, 1, 8]),
                edt: pass("edt", &[0, 1, 2, 3, 8]),
                wrap: pass("lightwrap", &[0, 1, 2, 3, 4]),
                tone: pass("tone", &[0, 1, 2, 3]),
                warp: pass("warp", &[0, 1, 2, 3]),
                wipe: pass("wipe", &[0, 1, 2, 3]),
                blocks: pass("blocks", &[0, 1, 3, 5]),
                tiles: pass("tiles", &[0, 1, 2, 3, 5]),
                relief: pass("relief", &[0, 1, 2, 3]),
                sharp: pass("sharp", &[0, 1, 2, 3, 4]),
                shprep: pass("shprep", &[0, 1, 2]),
                shhi: pass("shhi", &[0, 1, 2, 3, 4]),
                choke: pass("choke", &[0, 1, 2, 3, 6, 7]),
                lines: pass("lines", &[0, 1, 2, 3]),
                glare: pass("glare", &[0, 1, 2, 3, 4]),
                rain: pass("rain", &[0, 1, 2, 3]),
                line_blur: pass("lineblur", &[0, 1, 2, 3]),
                stars: pass("stars", &[0, 1, 3, 9]),
                kira: pass("kira", &[0, 1, 2, 3, 9]),
                balls: pass("balls", &[0, 1, 3, 9]),
                ballshade: pass("ballshade", &[0, 2, 3, 9]),
                particles: pass("particles", &[0, 1, 2, 3, 4]),
                bubbles: pass("bubbles", &[0, 1, 2, 3]),
                drizzle: pass("drizzle", &[0, 1, 2, 3]),
                hair: pass("hair", &[0, 1, 2, 3]),
                mercury: pass("mercury", &[0, 1, 2, 3]),
                shards: pass("shards", &[0, 1, 2, 3]),
                foam: pass("foam", &[0, 1, 2, 3]),
                scatterize: pass("scatterize", &[0, 1, 2, 3]),
                star_burst: pass("star_burst", &[0, 1, 2, 3]),
                median: pass("median", &[0, 1, 2, 3]),
                smart: pass("smart", &[0, 1, 2, 3, 4]),
                bilat: pass("bilat", &[0, 1, 2, 3, 4]),
                degrain: pass("degrain", &[0, 1, 2, 3]),
                deband: pass("deband", &[0, 1, 2, 3, 4, 5]),
                rough: pass("rough", &[0, 1, 2, 3]),
                rshadow: pass("rshadow", &[0, 1, 2, 3, 4]),
                bevel: pass("bevel", &[0, 1, 2, 3, 4]),
                snow: pass("snow", &[0, 1, 2, 3]),
                cells: pass("cells", &[0, 1, 2, 3]),
                adjust: pass("adjust", &[0, 1, 3, 4, 10, 11]),
                mix_back: pass("mixback", &[0, 1, 2, 3, 4]),
                chanmix: pass("chanmix", &[0, 1, 2, 4]),
                crossmix: pass("crossmix", &[0, 1, 2, 4]),
                selprep: pass("selprep", &[0, 1, 2, 3]),
                selpass: pass("selpass", &[0, 1, 2, 3]),
                selfinish: pass("selfinish", &[0, 1, 2, 3, 4]),
                vheight: pass("vheight", &[0, 1, 2, 3]),
                vblur: pass("vblur", &[0, 1, 2, 3, 4]),
                csigma: pass("csigma", &[0, 1, 3, 8]),
                cmix: pass("cmix", &[0, 1, 2, 3, 4, 8]),
                dmap: pass("dmap", &[0, 1, 2, 3, 4]),
                wire: pass("wire", &[0, 1, 2, 3, 4]),
                glass: pass("glass", &[0, 1, 2, 3, 4]),
                blobover: pass("blobover", &[0, 1, 2, 4]),
                blobby: pass("blobby", &[0, 1, 2, 3, 4]),
                beam: pass("beam", &[0, 1, 2, 3]),
                gradient4: pass("gradient4", &[0, 1, 2, 3]),
                sweepmin: pass("sweepmin", &[0, 1, 5]),
                sweep: pass("sweep", &[0, 1, 2, 3, 5]),
                waves: pass("waves", &[0, 1, 2, 3]),
                bolt: pass("bolt", &[0, 1, 2, 3]),
                stroke: pass("stroke", &[0, 1, 2, 3]),
                marks: pass("marks", &[0, 1, 2, 3]),
                vegas: pass("vegas", &[0, 1, 2, 3]),
                curlfield: pass("curlfield", &[0, 1, 2, 3]),
                curlsoft: pass("curlsoft", &[0, 1, 2, 3]),
                curlflow: pass("curlflow", &[0, 1, 2, 3, 4]),
                brush: pass("brush", &[0, 1, 2, 3]),
                cartoonedge: pass("cartoonedge", &[0, 1, 2, 3]),
                cartoon: pass("cartoon", &[0, 1, 2, 3, 4]),
                energy: pass("energy", &[0, 1, 2, 3]),
                blockload: pass("blockload", &[0, 1, 2, 3, 4]),
                gmeltk: pass("gmeltk", &[0, 1, 3, 7]),
                gmelt: pass("gmelt", &[0, 1, 2, 3, 4, 7]),
                burnfilm: pass("burnfilm", &[0, 1, 2, 3]),
                smoothie: pass("smoothie", &[0, 1, 2, 3, 4]),
                plastic: pass("plastic", &[0, 1, 2, 3, 4]),
                texturize: pass("texturize", &[0, 1, 2, 3, 4]),
                cptensor: pass("cptensor", &[0, 1, 2]),
                cptangent: pass("cptangent", &[0, 1, 2, 3]),
                cpsmear: pass("cpsmear", &[0, 1, 2, 3, 4]),
                cpfill: pass("cpfill", &[0, 1, 2, 3, 4]),
                cpedge: pass("cpedge", &[0, 1, 2, 3]),
                twist: pass("twist", &[0, 1, 2, 3, 4]),
                wreact: pass("wreact", &[0, 1, 2, 3, 4]),
                warpb: pass("warpb", &[0, 1, 2, 3, 4, 7]),
                writeon: pass("writeon", &[0, 1, 2, 3]),
                edges: pass("edges", &[0, 1, 2, 3]),
                dissolve: pass("dissolve", &[0, 1, 2, 3]),
                gwipe: pass("gwipe", &[0, 1, 2, 3, 4]),
                haze: pass("haze", &[0, 1, 2, 3, 4]),
                fog: pass("fog", &[0, 1, 2, 3, 4]),
                over: pass("over", &[0, 1, 2, 3, 4]),
                dmatte: pass("dmatte", &[0, 1, 2, 3, 4]),
                glasses: pass("glasses", &[0, 1, 2, 3, 4]),
                spotlight: pass("spotlight", &[0, 1, 2, 3, 4]),
                cylinder: pass("cylinder", &[0, 1, 2, 3]),
                sphere: pass("sphere", &[0, 1, 2, 3]),
                cardwipe: pass("cardwipe", &[0, 1, 2, 3, 4]),
                environment: pass("environment", &[0, 1, 2, 3, 4]),
                passx: pass("passx", &[0, 1, 2, 3, 4]),
                dkey: pass("dkey", &[0, 1, 2, 3, 4]),
                idkey: pass("idkey", &[0, 1, 2, 3, 4]),
                fill: pass("fillfx", &[0, 1, 2, 3, 4, 5]),
                eyerow: pass("eyerow", &[0, 1, 3, 7]),
                eyesum: pass("eyesum", &[0, 3, 7]),
                eyefill: pass("eyefill", &[0, 1, 2, 3, 7]),
                bucket: pass("bucket", &[0, 1, 2, 3, 4]),
                lwidth: pass("lwidth", &[0, 1, 2, 3, 5]),
                smoothscan: pass("smoothscan", &[0, 1, 3, 5, 6]),
                smoothmix: pass("smoothmix", &[0, 1, 2, 5, 6]),
                mchoke: pass("mchoke", &[0, 1, 2, 3, 7]),
                sglight: pass("sglight", &[0, 1, 2, 3]),
                sgcells: pass("sgcells", &[0, 1, 2, 3]),
                sgpass: pass("sgpass", &[0, 1, 2, 3]),
                sgadd: pass("sgadd", &[0, 1, 2, 3, 4]),
                sgfinish: pass("sgfinish", &[0, 1, 2, 3, 4]),
                sghalo: pass("sghalo", &[0, 1, 2, 3, 4]),
                sgdirt: pass("sgdirt", &[0, 1, 2, 3, 4]),
                flaredirt: pass("flaredirt", &[0, 1, 2, 3, 4]),
                pageturn: pass("pageturn", &[0, 1, 2, 3, 4]),
                chain: {
                    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                        label: Some("B-172"),
                        source: wgpu::ShaderSource::Wgsl(chain_shader().into()),
                    });
                    let entries = [
                        entry(0, uniform()),
                        entry(1, texture()),
                        entry(2, storage_texture()),
                        entry(3, storage(true)),
                        entry(12, storage(true)),
                    ];
                    let layout =
                        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                            label: Some("chain"),
                            entries: &entries,
                        });
                    (pipeline_in(&module, &layout, "chain"), layout)
                },
            }
        });
        let (layer, encode) = (
            pipeline(&layer_layout, "layer"),
            pipeline(&encode_layout, "encode"),
        );
        let radial = pipeline(&radial_layout, "radial");
        let moment = pipeline(&layer_layout, "moment");
        let unpack = pipeline(&unpack_layout, "unpack");
        let own = |entry_point: &str, entries: &[wgpu::BindGroupLayoutEntry]| {
            let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some(entry_point),
                entries,
            });
            (pipeline(&layout, entry_point), layout)
        };
        let take = own(
            "take",
            &[
                entry(0, uniform()),
                entry(1, storage(true)),
                entry(3, storage_texture()),
            ],
        );
        let no_matte = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("B-44 no matte"),
                size: wgpu::Extent3d {
                    width: 1,
                    height: 1,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default());
        let memory = card_memory(info.vendor, info.device);
        let about = format!(
            "{} ({:?}), driver {} {}, {:?}, {}",
            info.name,
            info.device_type,
            info.driver,
            info.driver_info,
            info.backend,
            memory.map_or("its memory not reported".into(), |m| format!(
                "{:.1} GB of its own memory",
                m as f64 / 1e9
            ))
        );
        Ok(Gpu {
            instance,
            adapter,
            screen: None,
            device,
            queue,
            about,
            limits,
            layer,
            moment,
            encode,
            layer_layout,
            encode_layout,
            radial,
            radial_layout,
            unpack,
            unpack_layout,
            take,
            ones: None,
            bloom,
            fx,
            no_matte,
            store: Vec::new(),
            frame: 0,
            target: None,
            // Half, so the frame's own buffers, the window and every other program keep room.
            budget: memory.map_or(DEFAULT_BUDGET_BYTES, |m| (m / 2) as usize),
            memory,
            sent: 0,
            failures: 0,
            sending: Sending::default(),
            spare: Vec::new(),
            made: Cell::new(0),
            shrunk: Cell::new(0),
            dispatched: 0,
            fused: true,
            working: clear.then(|| RefCell::new(Vec::new())),
            to_clear: RefCell::new(Vec::new()),
            fields: RefCell::new(Vec::new()),
        })
    }

    /// The card's own memory in bytes, when Windows says (B-48).
    pub fn memory(&self) -> Option<u64> {
        self.memory
    }

    /// D-105: what the card may hold when the memory setting is Automatic: half its memory, as
    /// B-44b set it.
    pub fn automatic_budget(&self) -> usize {
        self.memory
            .map_or(DEFAULT_BUDGET_BYTES, |m| (m / 2) as usize)
    }

    /// D-105: the most a Custom setting may give the card: 85% of its memory, so the frame's own
    /// buffers, the window and every other program keep the rest.
    pub fn largest_budget(&self) -> usize {
        self.memory
            .map_or(DEFAULT_BUDGET_BYTES, |m| (m / 20 * 17) as usize)
            .max(DEFAULT_BUDGET_BYTES)
    }

    /// How many frames the card has failed since it was opened (B-48): not the ones it refused,
    /// which the CPU draws because the card does not draw them yet, but the ones it tried and could not.
    pub fn failures(&self) -> u64 {
        self.failures
    }

    /// Bytes of drawings the card holds now (B-48), and of textures kept for new ones (B-153).
    pub fn held(&self) -> usize {
        let spare = |t: &wgpu::Texture| {
            (t.width() * t.height()) as usize
                * if t.format() == wgpu::TextureFormat::Rgba32Float {
                    16
                } else {
                    BYTES_PER_PIXEL
                }
        };
        self.store.iter().map(|s| s.bytes).sum::<usize>()
            + self.spare.iter().map(spare).sum::<usize>()
    }

    /// How many drawings have been sent to the card since it was opened.
    pub fn sent(&self) -> u64 {
        self.sent
    }

    /// B-153: how many textures and sending buffers the card has made for drawings since it was
    /// opened. Once frames repeat, a frame whose drawings are new reuses the last frame's.
    pub fn made(&self) -> u64 {
        self.made.get()
    }

    /// B-164 (D-235): how many blurs the card has worked small and enlarged since it was opened.
    pub fn shrunk(&self) -> u64 {
        self.shrunk.get()
    }

    /// B-172: the passes the card has run since it was opened, the frame's encoding aside.
    pub fn dispatched(&self) -> u64 {
        self.dispatched
    }

    /// B-172: whether a run of colour effects next to each other is drawn in one pass, as it is
    /// unless this says not; off only for the check that both draw the same.
    pub fn fuse(&mut self, on: bool) {
        self.fused = on;
    }

    /// The card, its driver and the backend, for tables and the switch.
    pub fn about(&self) -> &str {
        &self.about
    }

    /// Let go of every drawing on the card.
    pub fn forget(&mut self) {
        self.store.clear();
        self.spare.clear();
        self.fields.borrow_mut().clear();
    }

    /// `source` on the card, sending it first if it is not there. The view itself, not a place
    /// in the store, since sending one drawing can evict another this frame has not yet drawn.
    fn resident(
        &mut self,
        uploads: &mut wgpu::CommandEncoder,
        source: &Arc<WorkingBuffer>,
        name: Option<Name>,
        wide: bool,
    ) -> wgpu::TextureView {
        // A drawing the CPU still holds is found by its address; one it has let go of, by the
        // CPU cache's name for it.
        let found = self.store.iter_mut().find(|s| {
            s.wide == wide
                && ((s.held.strong_count() > 0 && s.held.as_ptr() == Arc::as_ptr(source))
                    || (name.is_some() && s.name == name))
        });
        if let Some(s) = found {
            s.held = Arc::downgrade(source);
            s.used = self.frame;
            return s.view.clone();
        }
        // A drawing nothing holds and nothing names can never be asked for again. B-153: its
        // texture is kept, within the budget, for a new drawing of its size.
        let (gone, kept): (Vec<_>, Vec<_>) = std::mem::take(&mut self.store)
            .into_iter()
            .partition(|s| s.name.is_none() && s.held.strong_count() == 0);
        self.store = kept;
        self.spare
            .extend(gone.into_iter().map(|s| s.view.texture().clone()));
        let (width, height) = (source.width(), source.height());
        let per_pixel = if wide { 16 } else { BYTES_PER_PIXEL };
        let bytes = width * height * per_pixel;
        let size = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };
        let format = if wide {
            wgpu::TextureFormat::Rgba32Float
        } else {
            wgpu::TextureFormat::Rgba16Float
        };
        let spare = self
            .spare
            .iter()
            .position(|t| t.size() == size && t.format() == format)
            .map(|i| self.spare.remove(i));
        // Spare textures first, the oldest first; then drawings, least recently used first,
        // never one this frame draws with. A frame that needs more than the budget still gets
        // every drawing it needs.
        while self.held() + bytes > self.budget {
            if !self.spare.is_empty() {
                self.spare.remove(0);
                continue;
            }
            let Some(oldest) = (0..self.store.len())
                .filter(|&i| self.store[i].used < self.frame)
                .min_by_key(|&i| self.store[i].used)
            else {
                break;
            };
            self.store.swap_remove(oldest);
        }
        let texture = match spare {
            // The copy below fills every pixel, so nothing of the drawing before stays.
            Some(t) => t,
            None => {
                self.made.set(self.made.get() + 1);
                self.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some("B-44 drawing"),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                    view_formats: &[],
                })
            }
        };
        // Every thread converts rows straight into memory the card copies from.
        let row = (width * per_pixel).next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize);
        let need = (row * height) as u64;
        let (staging, offset, own) = match self.room(need) {
            Some((buffer, at)) => (buffer, at, false),
            None => {
                self.made.set(self.made.get() + 1);
                let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("B-44 sending"),
                    size: need,
                    usage: wgpu::BufferUsages::MAP_WRITE | wgpu::BufferUsages::COPY_SRC,
                    mapped_at_creation: true,
                });
                (buffer, 0, true)
            }
        };
        {
            let mut mapped = staging.slice(offset..offset + need).get_mapped_range_mut();
            if wide {
                let floats: &mut [f32] = bytemuck::cast_slice_mut(&mut mapped);
                floats
                    .par_chunks_mut(row / 4)
                    .zip(source.data().par_chunks(width * 4))
                    .for_each(|(to, from)| to[..width * 4].copy_from_slice(from));
            } else {
                let halves: &mut [half::f16] = bytemuck::cast_slice_mut(&mut mapped);
                halves
                    .par_chunks_mut(row / 2)
                    .zip(source.data().par_chunks(width * 4))
                    .for_each(|(to, from)| to[..width * 4].convert_from_f32_slice(from));
            }
        }
        if own {
            staging.unmap();
        }
        uploads.copy_buffer_to_texture(
            wgpu::TexelCopyBufferInfo {
                buffer: &staging,
                layout: wgpu::TexelCopyBufferLayout {
                    offset,
                    bytes_per_row: Some(row as u32),
                    rows_per_image: None,
                },
            },
            texture.as_image_copy(),
            size,
        );
        self.sent += 1;
        let view = texture.create_view(&Default::default());
        self.store.push(Stored {
            held: Arc::downgrade(source),
            name,
            view: view.clone(),
            bytes,
            used: self.frame,
            applied: None,
            before: Vec::new(),
            wide,
        });
        view
    }

    /// B-153: where `need` bytes go in the kept sending memory this frame, open for writing, or
    /// `None` when it has no room left and they need memory of their own.
    fn room(&mut self, need: u64) -> Option<(wgpu::Buffer, u64)> {
        let s = &mut self.sending;
        let at = s.filled;
        s.filled += need;
        s.most = s.most.max(s.filled);
        let buffer = s.buffer.clone().filter(|b| b.size() >= s.filled)?;
        if !s.open {
            // Only once the frame that last copied from it is done.
            let (tell, told) = std::sync::mpsc::channel();
            buffer
                .slice(..)
                .map_async(wgpu::MapMode::Write, move |r| drop(tell.send(r)));
            let wait = s
                .after
                .clone()
                .map_or(wgpu::PollType::Wait, wgpu::PollType::WaitForSubmissionIndex);
            let opened = self.device.poll(wait).is_ok() && told.recv().is_ok_and(|r| r.is_ok());
            if !opened {
                // Never written again: the next frame makes another.
                s.buffer = None;
                return None;
            }
            s.open = true;
        }
        Some((buffer, at))
    }

    /// B-46, B-47, B-49, B-50, B-51, B-65: `effects` run on `source`, which [`Gpu::resident`] has just put on the card as
    /// `still`, one after another (B-155), each on the texture the one before it wrote. A result
    /// already made of it with the same settings is reused, and so is the kept picture after the
    /// effects a run begins with unchanged; what to dispatch for the rest is added to `steps`, to
    /// run before the layers.
    fn applied(
        &mut self,
        steps: &mut Vec<Step>,
        source: &Arc<WorkingBuffer>,
        still: &wgpu::TextureView,
        effects: &[OnCard],
    ) -> wgpu::TextureView {
        let stored = self
            .store
            .iter()
            .position(|s| s.used == self.frame && &s.view == still);
        let (mut views, mut unchanged) = (Vec::new(), 0);
        if let Some((kept, after)) = stored.and_then(|i| self.store[i].applied.as_ref()) {
            unchanged = kept.iter().zip(effects).take_while(|(a, b)| a == b).count();
            // B-172: back to the last picture kept, when a run drawn in one pass left none.
            let same = after[..unchanged]
                .iter()
                .rposition(Option::is_some)
                .map_or(0, |i| i + 1);
            views.extend_from_slice(&after[..same]);
        }
        if let Some(Some((last, _))) = views.last().filter(|_| views.len() == effects.len()) {
            return last.clone();
        }
        // B-173: the whole run drawn on it before with these very settings.
        let frame = self.frame;
        if let Some(b) =
            stored.and_then(|i| self.store[i].before.iter_mut().find(|b| b.0 == effects))
        {
            b.3 = frame;
            return b.1.clone();
        }
        let from = views
            .last()
            .cloned()
            .flatten()
            .unwrap_or((still.clone(), (source.width(), source.height())));
        let done = views.len();
        self.run(
            steps,
            source,
            from,
            &effects[done..],
            unchanged.saturating_sub(done),
            &mut views,
        );
        let moved = views
            .last()
            .cloned()
            .flatten()
            .expect("the last effect's picture")
            .0;
        if let Some(i) = stored {
            let s = &mut self.store[i];
            if let Some((old, after)) = s.applied.take() {
                if let Some(Some((view, size))) = after.last() {
                    s.before.push((old, view.clone(), *size, frame)); // Out of the working textures while kept, so the passes do not look it over
                                                                      // every time they want one.
                    if let Some(working) = &self.working {
                        working.borrow_mut().retain(|(t, _)| t != view.texture());
                    }
                }
            }
            s.applied = Some((effects.to_vec(), views));
            self.forget_before(i);
        }
        moved
    }

    /// B-173: the drawing at `i`'s bytes on the card counted again, after the oldest runs kept
    /// before are let go of, while all of them together hold more than a quarter of the budget.
    fn forget_before(&mut self, i: usize) {
        let bytes = |(w, h): (usize, usize)| w * h * 16;
        let mut total: usize = self
            .store
            .iter()
            .flat_map(|s| &s.before)
            .map(|b| bytes(b.2))
            .sum();
        while total > self.budget / 4 {
            let oldest = self
                .store
                .iter()
                .enumerate()
                .flat_map(|(i, s)| s.before.iter().enumerate().map(move |(j, b)| (b.3, i, j)))
                .filter(|&(used, ..)| used < self.frame)
                .min();
            let Some((_, i, j)) = oldest else { break };
            let (_, view, size, _) = self.store[i].before.remove(j);
            total -= bytes(size);
            // Back among the working textures for the next frame's passes, unless another run
            // still shows it.
            let t = view.texture();
            let shown = self.keeps(t)
                || self
                    .store
                    .iter()
                    .any(|s| s.before.iter().any(|b| b.1.texture() == t));
            if let Some(working) = self
                .working
                .as_ref()
                .filter(|_| !shown && t.usage().contains(wgpu::TextureUsages::STORAGE_BINDING))
            {
                working.borrow_mut().push((t.clone(), self.frame));
            }
            self.count(i);
        }
        self.count(i);
    }

    /// The bytes on the card of the drawing at `i`: its own, and the pictures kept after its
    /// effects.
    fn count(&mut self, i: usize) {
        let s = &mut self.store[i];
        let kept: usize = s
            .applied
            .iter()
            .flat_map(|(_, after)| after.iter().flatten())
            .map(|(_, (w, h))| w * h * 16)
            .sum();
        let before: usize = s.before.iter().map(|b| b.2 .0 * b.2 .1 * 16).sum();
        let own = s.view.texture();
        s.bytes = (own.width() * own.height()) as usize * if s.wide { 16 } else { BYTES_PER_PIXEL }
            + kept
            + before;
    }

    /// `effects` run one after another from `moved`, a drawing `size`, each on what the one before
    /// it wrote; each result is added to `views`. `source` is the drawing on the CPU. B-172: a run
    /// of two or more colour effects that each read only their own pixel is one pass, and adds
    /// none to `views` but for its last. A run is cut before `split`, the first effect changed
    /// since the last frame, so the part before it is kept and the next change draws only the rest.
    fn run(
        &self,
        steps: &mut Vec<Step>,
        source: &WorkingBuffer,
        (mut moved, mut size): (wgpu::TextureView, (usize, usize)),
        effects: &[OnCard],
        split: usize,
        views: &mut Vec<Option<(wgpu::TextureView, (usize, usize))>>,
    ) {
        let one = |e: &OnCard| {
            self.fused && matches!(e.unmixed(), OnCard::Fx(f) if one_pixel(&f.instance.effect))
        };
        let mut i = 0;
        while i < effects.len() {
            let mut n = effects[i..].iter().take_while(|e| one(e)).count();
            if i < split && split < i + n {
                n = split - i;
            }
            if n >= 2 {
                let staged = RefCell::new(Some(Vec::new()));
                for effect in &effects[i..i + n] {
                    let OnCard::Fx(f) = effect.unmixed() else {
                        unreachable!("a run is of colour effects")
                    };
                    let from = staged.borrow().as_ref().map_or(0, Vec::len);
                    self.fx(steps, source, size, &moved, f, &staged);
                    // B-221: its Mix, on what it staged.
                    if let OnCard::Mix(_, mix) = effect {
                        for stage in
                            &mut staged.borrow_mut().as_mut().expect("a run's stages")[from..]
                        {
                            stage.3 = (mix / 100.0) as f32;
                        }
                    }
                    views.push(None);
                }
                moved = self.chain(
                    steps,
                    &moved,
                    size,
                    &staged.into_inner().unwrap_or_default(),
                );
                *views.last_mut().expect("the run's last") = Some((moved.clone(), size));
                i += n;
                continue;
            }
            let given = (moved.clone(), size);
            (moved, size) = match effects[i].unmixed() {
                OnCard::Radial(r) => (self.blur(steps, &moved, size, *r), size),
                OnCard::Bloom(b) => self.bloom(steps, &moved, size, *b),
                OnCard::Directional(d) => self.directional(steps, &moved, size, *d),
                OnCard::Gaussian(g) => self.gaussian(steps, &moved, size, *g),
                OnCard::Glow(g) => self.glow(steps, &moved, size, *g),
                OnCard::Fx(f) => self.fx(steps, source, size, &moved, f, &RefCell::new(None)),
                OnCard::Mix(..) => unreachable!("a Mix holds an effect"),
            };
            // B-221 (D-340): what it was given laid back by its Mix, where it was, as `mix_back`.
            if let OnCard::Mix(_, mix) = &effects[i] {
                let passes = self.fx.as_ref().expect("a Mix only with the passes");
                let at = FxParams {
                    ox: ((size.0 - given.1 .0) / 2) as i32,
                    oy: ((size.1 - given.1 .1) / 2) as i32,
                    ..Default::default()
                };
                let out = self.scratch("B-221", size.0, size.1);
                let tiles = ((size.0 as u32).div_ceil(16), (size.1 as u32).div_ceil(16));
                self.fx_step(
                    steps,
                    &passes.mix_back,
                    at,
                    Some(&moved),
                    Some(&out),
                    Some(&[mix / 100.0]),
                    Some(&given.0),
                    [None; 4],
                    tiles,
                );
                moved = out;
            }
            views.push(Some((moved.clone(), size)));
            i += 1;
        }
    }

    /// B-172: a run of colour effects, `staged` by [`Gpu::fx`], drawn on `still`, a drawing `w` by
    /// `h`, in one pass into a texture of its own, which is returned.
    fn chain(
        &self,
        steps: &mut Vec<Step>,
        still: &wgpu::TextureView,
        (w, h): (usize, usize),
        staged: &[Staged],
    ) -> wgpu::TextureView {
        let (pipeline, layout) = &self.fx.as_ref().expect("a run only with the passes").chain;
        let (mut table, mut k) = (Vec::<u32>::new(), Vec::<f64>::new());
        for (which, p, settings, mix) in staged {
            table.extend(bytemuck::cast::<FxParams, [u32; 12]>(*p));
            // B-221: and the Mix, the stage padded to 64 bytes, its stride.
            table.extend([*which, k.len() as u32, mix.to_bits(), 0]);
            k.extend(settings);
        }
        let init = |label, contents: &[u8], usage| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        let run = FxParams {
            count: staged.len() as u32,
            ..Default::default()
        };
        let numbers = init(
            "B-172 run",
            bytemuck::bytes_of(&run),
            wgpu::BufferUsages::UNIFORM,
        );
        let k = init(
            "B-172 settings",
            bytemuck::cast_slice(&k),
            wgpu::BufferUsages::STORAGE,
        );
        let table = init(
            "B-172 effects",
            bytemuck::cast_slice(&table),
            wgpu::BufferUsages::STORAGE,
        );
        let out = self.scratch("B-172", w, h);
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: numbers.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(still),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&out),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: k.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 12,
                    resource: table.as_entire_binding(),
                },
            ],
        });
        steps.push((
            pipeline.clone(),
            group,
            ((w as u32).div_ceil(16), (h as u32).div_ceil(16)),
        ));
        out
    }

    /// A texture the passes write and the layers read, `width` by `height`. B-153: one an earlier
    /// frame used and nothing kept, cleared first, when there is one.
    fn scratch(&self, label: &str, width: usize, height: usize) -> wgpu::TextureView {
        let size = wgpu::Extent3d {
            width: width as u32,
            height: height as u32,
            depth_or_array_layers: 1,
        };
        if let Some(working) = &self.working {
            let mut working = working.borrow_mut();
            let free = working
                .iter_mut()
                .find(|(t, used)| *used < self.frame && t.size() == size && !self.keeps(t));
            if let Some((texture, used)) = free {
                *used = self.frame;
                self.to_clear.borrow_mut().push(texture.clone());
                return texture.create_view(&Default::default());
            }
        }
        self.made.set(self.made.get() + 1);
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba32Float,
            usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        if let Some(working) = &self.working {
            working.borrow_mut().push((texture.clone(), self.frame));
        }
        texture.create_view(&Default::default())
    }

    /// B-153: whether `texture` holds an effect's result a drawing keeps for later frames.
    fn keeps(&self, texture: &wgpu::Texture) -> bool {
        self.store.iter().any(|s| {
            s.applied.as_ref().is_some_and(|(_, after)| {
                after.iter().flatten().any(|(v, _)| v.texture() == texture)
            })
        })
    }

    /// `blurs::radial_blur` of `still`, `width` by `height`, into a texture of its own, which is
    /// returned. What to dispatch is added to `steps`, to run before the layers.
    fn blur(
        &self,
        steps: &mut Vec<Step>,
        still: &wgpu::TextureView,
        (width, height): (usize, usize),
        r: Radial,
    ) -> wgpu::TextureView {
        let moved = self.scratch("B-46 radial", width, height);
        let turns: Vec<[f32; 2]> =
            crate::blurs::radial_turns(r.spin, r.amount, r.sweep.map(|s| s.from))
                .into_iter()
                .flatten()
                .map(|(a, b)| [a as f32, b as f32])
                .collect();
        let init = |label, contents: &[u8], usage| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        let f = |v: f64| (v as f32).to_bits();
        // D-361/D-362: the path is as long whichever way it runs.
        let (weigh, density) = r.sweep.map_or((0, 1.0), |s| (s.weigh as u32, s.density));
        let settings = [
            f(r.center.0),
            f(r.center.1),
            f(r.amount.abs()),
            r.spin as u32,
            r.repeat as u32,
            weigh,
            f(density),
            0,
        ];
        let settings = init(
            "B-46 settings",
            bytemuck::cast_slice(&settings),
            wgpu::BufferUsages::UNIFORM,
        );
        let turns = init(
            "B-46 turns",
            bytemuck::cast_slice(&turns),
            wgpu::BufferUsages::STORAGE,
        );
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.radial_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: settings.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(still),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&moved),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: turns.as_entire_binding(),
                },
            ],
        });
        steps.push((
            self.radial.clone(),
            group,
            ((width as u32).div_ceil(16), (height as u32).div_ceil(16)),
        ));
        moved
    }

    /// One pass of [`BLOOM_SHADER`], added to `steps`: its numbers, `input`, and whichever of
    /// `output`, the halo and the weights the pass takes, over `width` by `height` threads.
    #[allow(clippy::too_many_arguments)]
    fn step(
        &self,
        steps: &mut Vec<Step>,
        (pipeline, layout): &Pass,
        p: Params,
        input: &wgpu::TextureView,
        output: Option<&wgpu::TextureView>,
        halo: Option<&wgpu::Buffer>,
        weights: Option<&wgpu::Buffer>,
        groups: (u32, u32),
    ) {
        let numbers = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("B-47 numbers"),
                contents: bytemuck::bytes_of(&p),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let mut entries = vec![
            wgpu::BindGroupEntry {
                binding: 0,
                resource: numbers.as_entire_binding(),
            },
            wgpu::BindGroupEntry {
                binding: 1,
                resource: wgpu::BindingResource::TextureView(input),
            },
        ];
        if let Some(o) = output {
            entries.push(wgpu::BindGroupEntry {
                binding: 2,
                resource: wgpu::BindingResource::TextureView(o),
            });
        }
        if let Some(h) = halo {
            entries.push(wgpu::BindGroupEntry {
                binding: 3,
                resource: h.as_entire_binding(),
            });
        }
        if let Some(w) = weights {
            entries.push(wgpu::BindGroupEntry {
                binding: 4,
                resource: w.as_entire_binding(),
            });
        }
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &entries,
        });
        steps.push((pipeline.clone(), group, groups));
    }

    /// B-47: `bloom::bloom` of `still`, `width` by `height`, into a texture of its own, grown by
    /// the bloom's reach, which is returned with its size. What to dispatch is added to `steps`.
    fn bloom(
        &self,
        steps: &mut Vec<Step>,
        still: &wgpu::TextureView,
        (w, h): (usize, usize),
        b: Bloom,
    ) -> (wgpu::TextureView, (usize, usize)) {
        let passes = self
            .bloom
            .as_ref()
            .expect("a Bloom is refused without the passes");
        let grow = crate::bloom::reach(b.radius, b.lines, b.length);
        let (gw, gh) = (w + 2 * grow, h + 2 * grow);
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        // The bright pass reads colours and the streak pass end taps, which a Bloom has none
        // of; a buffer cannot be empty.
        let none = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("B-47 none"),
                contents: bytemuck::cast_slice(&[0.0f32; 3]),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let light = self.scratch("B-47 light", w, h);
        let p = Params {
            level: crate::bloom::bright_level(b.threshold),
            ..Default::default()
        };
        self.step(
            steps,
            &passes.bright,
            p,
            still,
            Some(&light),
            None,
            Some(&none),
            tiles(w, h),
        );
        // Made empty: wgpu clears every buffer it makes.
        let halo = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("B-47 halo"),
            size: (gw * gh * 16) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let add = Params {
            width: gw as u32,
            height: gh as u32,
            ..Default::default()
        };
        for s in crate::bloom::SCALES {
            let sigma = b.radius / 3.0 * s;
            let r = crate::effects::kernel_radius(sigma);
            let placed = Params {
                g: (grow - r) as i32,
                weight: 0.25,
                ..add
            };
            if r == 0 {
                self.step(
                    steps,
                    &passes.add,
                    placed,
                    &light,
                    None,
                    Some(&halo),
                    None,
                    tiles(gw, gh),
                );
                continue;
            }
            let tall = self.blurred(steps, passes, "B-47", &light, (w, h), sigma, false);
            self.step(
                steps,
                &passes.add,
                placed,
                &tall,
                None,
                Some(&halo),
                None,
                tiles(gw, gh),
            );
        }
        let share = 1.0 / b.lines as f32;
        for j in 0..b.lines {
            let u = crate::blurs::along(b.angle + j as f64 * 180.0 / b.lines as f64);
            if b.length == 0.0 {
                let p = Params {
                    g: grow as i32,
                    weight: share,
                    ..add
                };
                self.step(
                    steps,
                    &passes.add,
                    p,
                    &light,
                    None,
                    Some(&halo),
                    None,
                    tiles(gw, gh),
                );
                continue;
            }
            let wt = crate::bloom::streak_weights(u, b.length);
            let f = crate::blurs::line_frame(u, w, h, grow);
            let count = (f.k1 - f.k0 + 1) as usize;
            let lines = self.scratch("B-47 lines", f.ow, count);
            let p = Params {
                s: f.s,
                a: wt.a,
                b: wt.b,
                k0: f.k0 as i32,
                n: wt.inner as u32,
                count: count as u32,
                g: grow as i32,
                down: f.down as u32,
                width: f.ow as u32,
                ..Default::default()
            };
            let p = Params {
                reach: wt.inner as u32,
                ..p
            };
            self.step(
                steps,
                &passes.streak,
                p,
                &light,
                Some(&lines),
                None,
                Some(&none),
                ((count as u32).div_ceil(32), 1),
            );
            let p = Params {
                weight: share,
                width: gw as u32,
                height: gh as u32,
                ..p
            };
            self.step(
                steps,
                &passes.mix,
                p,
                &lines,
                None,
                Some(&halo),
                None,
                tiles(gw, gh),
            );
        }
        let out = self.scratch("B-47 bloom", gw, gh);
        let p = Params {
            g: grow as i32,
            weight: b.intensity as f32,
            ..Default::default()
        };
        self.step(
            steps,
            &passes.combine,
            p,
            still,
            Some(&out),
            Some(&halo),
            None,
            tiles(gw, gh),
        );
        (out, (gw, gh))
    }

    /// B-49: `blurs::directional_blur` of `still`, `width` by `height`, into a texture of its own,
    /// grown by half the length (D-109: or not at all), which is returned with its size. What to
    /// dispatch is added to `steps`.
    fn directional(
        &self,
        steps: &mut Vec<Step>,
        still: &wgpu::TextureView,
        (w, h): (usize, usize),
        d: Directional,
    ) -> (wgpu::TextureView, (usize, usize)) {
        let passes = self
            .bloom
            .as_ref()
            .expect("a Directional Blur is refused without the passes");
        let grow = d.grow();
        let (gw, gh) = (w + 2 * grow, h + 2 * grow);
        let u = crate::blurs::along(d.direction);
        let wt = crate::blurs::directional_weights(u, d.length);
        let f = crate::blurs::line_frame(u, w, h, grow);
        let count = (f.k1 - f.k0 + 1) as usize;
        let lines = self.scratch("B-49 lines", f.ow, count);
        let ends: Vec<f32> = wt
            .ends
            .iter()
            .flat_map(|&(j, w)| [j as f32, w as f32])
            .collect();
        let ends = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("B-49 ends"),
                contents: bytemuck::cast_slice(if ends.is_empty() {
                    &[0.0f32; 2]
                } else {
                    &ends[..]
                }),
                usage: wgpu::BufferUsages::STORAGE,
            });
        let p = Params {
            s: f.s,
            a: wt.a,
            b: wt.b,
            k0: f.k0 as i32,
            n: wt.inner as u32,
            count: count as u32,
            g: grow as i32,
            down: f.down as u32,
            width: f.ow as u32,
            reach: wt.reach() as u32,
            ends: wt.ends.len() as u32,
            held: d.repeat as u32,
            ..Default::default()
        };
        self.step(
            steps,
            &passes.streak,
            p,
            still,
            Some(&lines),
            None,
            Some(&ends),
            ((count as u32).div_ceil(32), 1),
        );
        let out = self.scratch("B-49 directional", gw, gh);
        let p = Params {
            width: gw as u32,
            height: gh as u32,
            ..p
        };
        self.step(
            steps,
            &passes.lay,
            p,
            &lines,
            Some(&out),
            None,
            None,
            ((gw as u32).div_ceil(16), (gh as u32).div_ceil(16)),
        );
        (out, (gw, gh))
    }

    /// B-50: `effects::blur` of `still`, `width` by `height`, across and then down with Bloom's
    /// `gauss` pass, into a texture grown by the kernel's radius (D-109: or not at all), which is
    /// returned with its size.
    fn gaussian(
        &self,
        steps: &mut Vec<Step>,
        still: &wgpu::TextureView,
        (w, h): (usize, usize),
        g: Gaussian,
    ) -> (wgpu::TextureView, (usize, usize)) {
        let passes = self
            .bloom
            .as_ref()
            .expect("a Gaussian Blur is refused without the passes");
        let e = g.grow();
        // B-164: D-235's shortcut blurs transparent edges; held ones stay exact.
        let tall = if g.repeat {
            let taps = crate::effects::reach_weights(g.sigma, g.long);
            self.gauss(
                steps,
                passes,
                "B-50",
                still,
                (w, h),
                [&taps, &taps],
                e,
                true,
            )
        } else {
            self.blurred(steps, passes, "B-50", still, (w, h), g.sigma, g.long)
        };
        (tall, (w + 2 * e, h + 2 * e))
    }

    /// `effects::convolve` across and then down with Bloom's `gauss` pass, into a texture grown
    /// by `e` (the kernel's radius; D-109: none with `held`), which is returned. B-223: `taps`
    /// across and down, of one length, as `effects::blur_axes` takes them.
    #[allow(clippy::too_many_arguments)]
    fn gauss(
        &self,
        steps: &mut Vec<Step>,
        passes: &BloomPasses,
        label: &str,
        input: &wgpu::TextureView,
        (w, h): (usize, usize),
        taps: [&[f32]; 2],
        e: usize,
        held: bool,
    ) -> wgpu::TextureView {
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        let weights = |taps: &[f32]| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(&format!("{label} weights")),
                    contents: bytemuck::cast_slice(taps),
                    usage: wgpu::BufferUsages::STORAGE,
                })
        };
        let first = weights(taps[0]);
        let second = (taps[1] != taps[0]).then(|| weights(taps[1]));
        let (wide, tall) = (
            self.scratch(&format!("{label} wide"), w + 2 * e, h),
            self.scratch(&format!("{label} tall"), w + 2 * e, h + 2 * e),
        );
        let across = Params {
            count: (taps[0].len() / 2) as u32,
            axis: 0,
            held: held as u32,
            ..Default::default()
        };
        self.step(
            steps,
            &passes.gauss,
            across,
            input,
            Some(&wide),
            None,
            Some(&first),
            tiles(w + 2 * e, h),
        );
        let down = Params { axis: 1, ..across };
        self.step(
            steps,
            &passes.gauss,
            down,
            &wide,
            Some(&tall),
            None,
            Some(second.as_ref().unwrap_or(&first)),
            tiles(w + 2 * e, h + 2 * e),
        );
        tall
    }

    /// B-223: `picture` (a map another layer made) as a texture of its own, for a pass to read.
    fn map_texture(&self, picture: &WorkingBuffer) -> wgpu::TextureView {
        let texture = self.device.create_texture_with_data(
            &self.queue,
            &wgpu::TextureDescriptor {
                label: Some("B-223 map"),
                size: wgpu::Extent3d {
                    width: picture.width() as u32,
                    height: picture.height() as u32,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float,
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                view_formats: &[],
            },
            wgpu::util::TextureDataOrder::LayerMajor,
            bytemuck::cast_slice(picture.data()),
        );
        texture.create_view(&Default::default())
    }

    /// `effects::blur` of `input`, `width` by `height`, transparent outside it, into a texture
    /// grown by the kernel's radius, which is returned. B-164 (D-235, the viewer only): a big
    /// blur is worked on the picture shrunk by [`shrink_factor`] and enlarged back, which stays
    /// within 1 level of the exact blur.
    #[allow(clippy::too_many_arguments)]
    fn blurred(
        &self,
        steps: &mut Vec<Step>,
        passes: &BloomPasses,
        label: &str,
        input: &wgpu::TextureView,
        (w, h): (usize, usize),
        sigma: f64,
        long: bool,
    ) -> wgpu::TextureView {
        let r = crate::effects::reach_radius(sigma, long);
        let f = shrink_factor(sigma);
        if f == 1 {
            let taps = crate::effects::reach_weights(sigma, long);
            return self.gauss(
                steps,
                passes,
                label,
                input,
                (w, h),
                [&taps, &taps],
                r,
                false,
            );
        }
        self.shrunk.set(self.shrunk.get() + 1);
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        let taps = small_taps(sigma, f, long);
        let rs = taps.len() / 2;
        // Room round the small picture, so the enlarged blur reaches the exact one's edge and
        // a pixel past it.
        let m = (r.div_ceil(f) + 1).saturating_sub(rs);
        let (sw, sh) = (w.div_ceil(f) + 2 * m, h.div_ceil(f) + 2 * m);
        let small = self.scratch(&format!("{label} small"), sw, sh);
        let p = Params {
            n: f as u32,
            g: m as i32,
            ..Default::default()
        };
        self.step(
            steps,
            &passes.shrink,
            p,
            input,
            Some(&small),
            None,
            None,
            tiles(sw, sh),
        );
        let blurred = self.gauss(
            steps,
            passes,
            &format!("{label} small"),
            &small,
            (sw, sh),
            [&taps, &taps],
            rs,
            false,
        );
        let out = self.scratch(&format!("{label} tall"), w + 2 * r, h + 2 * r);
        let p = Params {
            n: f as u32,
            g: r as i32,
            count: (rs + m) as u32,
            ..Default::default()
        };
        self.step(
            steps,
            &passes.enlarge,
            p,
            &blurred,
            Some(&out),
            None,
            None,
            tiles(w + 2 * r, h + 2 * r),
        );
        out
    }

    /// One pass of [`FX_SHADER`], added to `steps`: its numbers, and whichever of `input`,
    /// `output`, the numbers `k`, the `other` texture and the `row`, `band`, `sums`, (B-76)
    /// `dist` and (B-123) `lit` buffers it takes, over `groups`.
    #[allow(clippy::too_many_arguments)]
    fn fx_step<const N: usize>(
        &self,
        steps: &mut Vec<Step>,
        (pipeline, layout): &Pass,
        p: FxParams,
        input: Option<&wgpu::TextureView>,
        output: Option<&wgpu::TextureView>,
        k: Option<&[f64]>,
        other: Option<&wgpu::TextureView>,
        work: [Option<&wgpu::Buffer>; N],
        groups: (u32, u32),
    ) {
        let init = |label, contents: &[u8], usage| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents,
                    usage,
                })
        };
        let numbers = init(
            "B-65 numbers",
            bytemuck::bytes_of(&p),
            wgpu::BufferUsages::UNIFORM,
        );
        let k = k.map(|k| {
            init(
                "B-65 settings",
                bytemuck::cast_slice(k),
                wgpu::BufferUsages::STORAGE,
            )
        });
        let mut entries = vec![wgpu::BindGroupEntry {
            binding: 0,
            resource: numbers.as_entire_binding(),
        }];
        let textures = [(1, input), (2, output), (4, other)];
        entries.extend(textures.iter().filter_map(|&(binding, t)| {
            t.map(|t| wgpu::BindGroupEntry {
                binding,
                resource: wgpu::BindingResource::TextureView(t),
            })
        }));
        entries.extend(k.as_ref().map(|k| wgpu::BindGroupEntry {
            binding: 3,
            resource: k.as_entire_binding(),
        }));
        entries.extend((5..).zip(work).filter_map(|(binding, b)| {
            b.map(|b| wgpu::BindGroupEntry {
                binding,
                resource: b.as_entire_binding(),
            })
        }));
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout,
            entries: &entries,
        });
        steps.push((pipeline.clone(), group, groups));
    }

    /// B-65: one of the batch of ten, `apply_stack_at` of `f` on `still`, `w` by `h`, into a
    /// texture of its own, returned with its size. What to dispatch is added to `steps`.
    /// B-155: `source` is the drawing the layer sent, which Paraffin and Kira-kira read here;
    /// they only begin a run, so it is what they are given.
    fn fx(
        &self,
        steps: &mut Vec<Step>,
        source: &WorkingBuffer,
        (w, h): (usize, usize),
        still: &wgpu::TextureView,
        f: &Fx,
        staged: &RefCell<Option<Vec<Staged>>>,
    ) -> (wgpu::TextureView, (usize, usize)) {
        use crate::effects::Effect as E;
        let passes = self
            .fx
            .as_ref()
            .expect("the batch of ten is refused without the passes");
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        let none = [None; 4];
        let blend = |b: &str| match b {
            "multiply" => 1,
            "screen" => 2,
            "add" => 3,
            "overlay" => 4,
            "soft_light" => 5,
            _ => 0,
        };
        let linear = |c: &str| crate::effects::encoded(c).map(crate::grade::to_linear);
        // D-413: a blending mode as the shader's `laid` takes it.
        let laid = |b: &str| {
            [
                "none",
                "normal",
                "multiply",
                "screen",
                "add",
                "overlay",
                "soft_light",
                "stencil_alpha",
            ]
            .iter()
            .position(|m| *m == b)
            .unwrap_or(1) as f64
        };
        let buffer = |bytes: usize| {
            self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("B-65 work"),
                size: bytes.max(4) as u64,
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            })
        };
        // A pass the size of the drawing, on `still`. B-172: in a run, only noted for its one pass.
        let same = |steps: &mut Vec<Step>,
                    pass: &Pass,
                    p: FxParams,
                    k: &[f64],
                    other: Option<&wgpu::TextureView>| {
            if let Some(run) = staged.borrow_mut().as_mut() {
                let which = [&passes.grade, &passes.tone]
                    .iter()
                    .position(|q| std::ptr::eq(*q, pass))
                    .filter(|_| other.is_none());
                run.push((
                    which.expect("B-172: a run holds only colour effects of their own pixel")
                        as u32,
                    p,
                    k.to_vec(),
                    1.0,
                ));
                return (still.clone(), (w, h));
            }
            let out = self.scratch("B-65", w, h);
            self.fx_step(
                steps,
                pass,
                p,
                Some(still),
                Some(&out),
                Some(k),
                other,
                none,
                tiles(w, h),
            );
            (out, (w, h))
        };
        // The drawing's covering blurred at `sigma`, as `effects::blur`, and the blur's radius.
        let covering = |steps: &mut Vec<Step>, input: &wgpu::TextureView, size, sigma: f64| {
            let (view, _) = self.gaussian(
                steps,
                input,
                size,
                Gaussian {
                    sigma,
                    repeat: false,
                    long: false,
                },
            );
            (view, crate::effects::kernel_radius(sigma))
        };
        // D-358's Bilateral Blur on `still`, its two passes. B-237: Radius below 1 or Threshold 0
        // counts only the pixel itself, a disc of one with both bells flat.
        let bilateral = |steps: &mut Vec<Step>, radius: f64, threshold: f64, colorize: bool| {
            let alone = radius < 1.0 || threshold <= 0.0;
            let (r, k) = if alone {
                (0, vec![0.0, 0.0, 0.0])
            } else {
                let s = radius / 2.0;
                let mut k = vec![1.0 / (2.0 * threshold * threshold), 1.0 / (2.0 * s * s)];
                k.extend(
                    crate::layer_fx::disc_runs(radius)
                        .iter()
                        .map(|&(_, hw)| hw as f64),
                );
                (radius.floor() as i32, k)
            };
            let p = FxParams {
                r,
                n: 2,
                flag: colorize as u32,
                ..Default::default()
            };
            let (levels, _) = same(steps, &passes.bilat, p, &k, Some(still));
            same(
                steps,
                &passes.bilat,
                FxParams { mode: 1, ..p },
                &k,
                Some(&levels),
            )
        };
        let (ox, oy) = f.origin;
        // D-144's Mosaic on `still`, blocks `size` wide from the drawing's corner, or (D-449)
        // blended between their centres. Size 1 or less is `still` as it is.
        let mosaic = |steps: &mut Vec<Step>, size: f64, bilinear: bool| {
            if size <= 1.0 {
                return still.clone();
            }
            // Where each run of columns, or of rows, in one block starts, the end last, and
            // each column's or row's run, as layer_fx::block_runs cuts them.
            let runs = |n: usize, o: usize| {
                let runs = crate::layer_fx::block_runs(n, o, size);
                let mut of = Vec::with_capacity(n);
                for (i, r) in runs.iter().enumerate() {
                    of.extend(r.clone().map(|_| i as f64));
                }
                let mut starts: Vec<f64> = runs.iter().map(|r| r.start as f64).collect();
                starts.push(n as f64);
                (starts, of)
            };
            let ((cs, col_of), (rs, row_of)) = (runs(w, ox), runs(h, oy));
            let (nc, nr) = (cs.len() - 1, rs.len() - 1);
            let mut k = vec![nc as f64, nr as f64];
            k.extend(cs.into_iter().chain(rs).chain(col_of).chain(row_of));
            let means = buffer(16 * nc * nr);
            self.fx_step(
                steps,
                &passes.blocks,
                FxParams::default(),
                Some(still),
                None,
                Some(&k),
                None,
                [Some(&means), None, None, None],
                tiles(nc, nr),
            );
            let out = self.scratch("B-107 mosaic", w, h);
            let p = FxParams {
                mode: bilinear as u32,
                ..Default::default()
            };
            self.fx_step(
                steps,
                &passes.tiles,
                p,
                Some(still),
                Some(&out),
                Some(&k),
                None,
                [Some(&means), None, None, None],
                tiles(w, h),
            );
            out
        };
        let turned = drawn_as(&f.instance.effect);
        match turned.as_ref().unwrap_or(&f.instance.effect) {
            E::MatchGrain { .. } => (still.clone(), (w, h)),
            E::RemoveGrain {
                passes: count,
                mode,
                unsharp_amount,
                unsharp_radius,
                ..
            } => {
                // D-455, as grade::remove_grain: the colour encoded, each pass, back, then
                // Sharpen's pass; compose sends one only with its noise and no threshold.
                let mut pic = still.clone();
                if let Some(spread) = crate::grade::grain_spreads(&f.instance.effect) {
                    let mut cur = self.scratch("D-455 encoded", w, h);
                    self.fx_step(
                        steps,
                        &passes.degrain,
                        FxParams::default(),
                        Some(still),
                        Some(&cur),
                        Some(&[0.0]),
                        None,
                        none,
                        tiles(w, h),
                    );
                    for k in 1..=count.floor() as usize {
                        let mut kk: Vec<f64> = spread
                            .iter()
                            .map(|&s| if s > 0.0 { 1.0 / (2.0 * s * s) } else { -1.0 })
                            .collect();
                        kk.push(1.0 / (2.0 * (k * k) as f64));
                        kk.extend(
                            crate::layer_fx::disc_runs((2 * k) as f64)
                                .iter()
                                .map(|&(_, hw)| hw as f64),
                        );
                        let p = FxParams {
                            mode: 1,
                            flag: (mode == "multichannel") as u32,
                            r: 2 * k as i32,
                            n: 4,
                            ..Default::default()
                        };
                        let next = self.scratch("D-455 pass", w, h);
                        self.fx_step(
                            steps,
                            &passes.degrain,
                            p,
                            Some(&cur),
                            Some(&next),
                            Some(&kk),
                            None,
                            none,
                            tiles(w, h),
                        );
                        cur = next;
                    }
                    let back = self.scratch("D-455 decoded", w, h);
                    let p = FxParams {
                        mode: 2,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.degrain,
                        p,
                        Some(&cur),
                        Some(&back),
                        Some(&[0.0]),
                        None,
                        none,
                        tiles(w, h),
                    );
                    pic = back;
                }
                if *unsharp_amount > 0.0 && *unsharp_radius > 0.0 {
                    let (blurred, r) = covering(steps, &pic, (w, h), *unsharp_radius);
                    let out = self.scratch("D-455 sharpened", w, h);
                    let p = FxParams {
                        r: r as i32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.sharp,
                        p,
                        Some(&pic),
                        Some(&out),
                        Some(&[unsharp_amount / 100.0]),
                        Some(&blurred),
                        none,
                        tiles(w, h),
                    );
                    pic = out;
                }
                (pic, (w, h))
            }
            E::Deband {
                view,
                edge_threshold,
                edge_softness,
                blur_amount,
                edge_blur_reduction,
                diffusion_amount,
                edge_diffusion_reduction,
                diffusion_reseed_interval,
                noise_amount,
                edge_noise_reduction,
                noise_reseed_interval,
                frame,
            } => {
                // D-456, as grade::deband: the edges, the blur across then down and pulled back,
                // then the diffusion, the dither and the view.
                let taps = crate::effects::gaussian_weights(blur_amount / 3.0);
                let r = taps.len() / 2;
                let shown = crate::effects::DEBAND_VIEWS
                    .iter()
                    .position(|v| v == view)
                    .unwrap_or(0) as f64;
                let mut k = vec![
                    *edge_threshold,
                    *edge_softness,
                    edge_blur_reduction / 100.0,
                    *diffusion_amount,
                    edge_diffusion_reduction / 100.0,
                    noise_amount / 255.0,
                    edge_noise_reduction / 100.0,
                    shown,
                ];
                k.extend(taps[r..].iter().map(|&v| v as f64));
                let base = crate::grade::mix(0);
                let p = FxParams {
                    r: r as i32,
                    frame: crate::effects::turbulent_seed(0.0, *diffusion_reseed_interval, *frame)
                        as i64 as i32,
                    g: crate::effects::turbulent_seed(0.0, *noise_reseed_interval, *frame) as i64
                        as i32,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                // The sums across, in single precision (B-430, D-550), which every pass binds.
                let sums = buffer(if r > 0 { w * h * 16 } else { 16 });
                let mut run = |mode: u32,
                               input: &wgpu::TextureView,
                               out: &wgpu::TextureView,
                               other: &wgpu::TextureView| {
                    let work = [Some(&sums), None, None, None];
                    self.fx_step(
                        steps,
                        &passes.deband,
                        FxParams { mode, ..p },
                        Some(input),
                        Some(out),
                        Some(&k),
                        Some(other),
                        work,
                        tiles(w, h),
                    );
                };
                let edges = self.scratch("D-456 edges", w, h);
                run(0, still, &edges, still);
                let mut m = edges.clone();
                if r > 0 {
                    m = self.scratch("D-456 blurred", w, h);
                    run(2, still, &m, &edges);
                    run(3, still, &m, &edges);
                }
                let out = self.scratch("D-456 out", w, h);
                run(5, &m, &out, still);
                (out, (w, h))
            }
            E::Curves {
                master,
                red,
                green,
                blue,
                ..
            } => {
                // Where each curve starts, then each curve's count, ins, outs and second derivatives.
                let mut k = vec![0.0; 4];
                for (i, c) in [red, green, blue, master].into_iter().enumerate() {
                    k[i] = k.len() as f64;
                    let (xs, ys, m) = crate::grade::knots(c);
                    k.push(xs.len() as f64);
                    k.extend(xs.into_iter().chain(ys).chain(m));
                }
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 0,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Levels {
                input_black,
                input_white,
                gamma,
                output_black,
                output_white,
            } => {
                let plain = crate::effects::LEVELS_PLAIN;
                let sets = [
                    [
                        *input_black,
                        *input_white,
                        *gamma,
                        *output_black,
                        *output_white,
                    ],
                    plain,
                    plain,
                    plain,
                ];
                let k: Vec<f64> = sets
                    .iter()
                    .flat_map(|s| [s[0], s[1], 1.0 / s[2], s[3], s[4]])
                    .collect();
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 1,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-383: the alpha set is drawn on the CPU (`compose::card_can`).
            E::ChannelLevels { sets, .. } => {
                let k: Vec<f64> = sets[..4]
                    .iter()
                    .flat_map(|s| [s[0], s[1], 1.0 / s[2], s[3], s[4]])
                    .collect();
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 1,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::HueSaturation {
                hue,
                saturation,
                lightness,
                ..
            } => same(
                steps,
                &passes.grade,
                FxParams {
                    mode: 2,
                    ..Default::default()
                },
                &[*hue, *saturation, *lightness],
                None,
            ),
            E::Gradient {
                shape,
                start,
                end,
                start_color,
                end_color,
                start_opacity,
                end_opacity,
                blend: b,
            } => {
                let (sx, sy) = crate::effects::radial_center(*start, (w, h), f.origin);
                let (ex, ey) = crate::effects::radial_center(*end, (w, h), f.origin);
                let (ex, ey) = (ex - sx, ey - sy);
                let ll = ex * ex + ey * ey;
                let mut k = vec![
                    sx,
                    sy,
                    ex,
                    ey,
                    ll,
                    ll.sqrt(),
                    (shape == "radial") as u8 as f64,
                    *start_opacity,
                    *end_opacity,
                ];
                k.extend(crate::effects::encoded(start_color));
                k.extend(crate::effects::encoded(end_color));
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 3,
                        blend: blend(b),
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Noise {
                amount,
                mode,
                seed,
                frame,
                ..
            } => {
                let base = crate::grade::mix(seed.floor() as u64);
                let p = FxParams {
                    mode: 4,
                    flag: (mode == "color") as u32,
                    frame: *frame,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.grade, p, &[amount / 200.0], None)
            }
            // D-443: grade::add_grain as grade mode 10.
            E::AddGrain {
                intensity,
                size,
                softness,
                aspect_ratio,
                red_intensity,
                green_intensity,
                blue_intensity,
                monochromatic,
                saturation,
                blending_mode,
                shadows,
                midtones,
                highlights,
                midpoint,
                animation_speed,
                animate_smoothly,
                random_seed,
                frame,
            } => {
                let base = crate::grade::mix(random_seed.floor() as u64);
                let phase = *frame as f64 * animation_speed;
                let z = if animate_smoothly == "on" {
                    phase
                } else {
                    phase.floor()
                };
                let blend = crate::effects::ADD_GRAIN_BLENDS
                    .iter()
                    .position(|b| b == blending_mode)
                    .unwrap_or(0) as f64;
                let k = [
                    size * aspect_ratio,
                    *size,
                    z,
                    *softness,
                    *saturation,
                    *red_intensity,
                    *green_intensity,
                    *blue_intensity,
                    *shadows,
                    *midtones,
                    *highlights,
                    *midpoint,
                    0.1 * intensity,
                    blend,
                ];
                let p = FxParams {
                    mode: 10,
                    flag: (monochromatic == "on") as u32,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.grade, p, &k, None)
            }
            // D-450: grade::noise_alpha as grade mode 11.
            E::NoiseAlpha {
                noise,
                amount,
                original_alpha,
                overflow,
                random_seed,
                noise_phase,
                cycle_noise,
                cycle,
            } => {
                let animated = noise.ends_with("_animation");
                let base = crate::grade::mix(if animated {
                    0
                } else {
                    random_seed.floor() as u64
                });
                let at =
                    |list: &[&str], v: &str| list.iter().position(|o| *o == v).unwrap_or(0) as f64;
                let k = [
                    amount / 100.0,
                    if animated { noise_phase / 360.0 } else { 0.0 },
                    if cycle_noise == "on" {
                        cycle.floor()
                    } else {
                        0.0
                    },
                    noise.starts_with("squared") as u8 as f64,
                    at(&crate::effects::NOISE_ALPHA_ORIGINALS, original_alpha),
                    at(&crate::effects::NOISE_ALPHA_OVERFLOWS, overflow),
                ];
                let p = FxParams {
                    mode: 11,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.grade, p, &k, None)
            }
            // D-451: grade::noise_hls as grade mode 12.
            E::NoiseHls {
                noise,
                hue,
                lightness,
                saturation,
                grain_size,
                noise_phase,
            } => {
                let base = crate::grade::mix(0);
                let kind = crate::effects::NOISE_HLS_KINDS
                    .iter()
                    .position(|o| o == noise)
                    .unwrap_or(0) as f64;
                let k = [
                    hue / 100.0,
                    lightness / 100.0,
                    saturation / 100.0,
                    *grain_size,
                    noise_phase / 360.0,
                    kind,
                ];
                let p = FxParams {
                    mode: 12,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.grade, p, &k, None)
            }
            // D-452: the same with the depth the frame times the speed.
            E::NoiseHlsAuto {
                noise,
                hue,
                lightness,
                saturation,
                grain_size,
                animation_speed,
                frame,
            } => {
                let base = crate::grade::mix(0);
                let kind = crate::effects::NOISE_HLS_KINDS
                    .iter()
                    .position(|o| o == noise)
                    .unwrap_or(0) as f64;
                let k = [
                    hue / 100.0,
                    lightness / 100.0,
                    saturation / 100.0,
                    *grain_size,
                    *frame as f64 * animation_speed,
                    kind,
                ];
                let p = FxParams {
                    mode: 12,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.grade, p, &k, None)
            }
            E::ChromaticAberration { amount, center } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let (w0, h0) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let k = amount / ((w0 * w0 + h0 * h0).sqrt() / 2.0);
                same(
                    steps,
                    &passes.aberr,
                    FxParams::default(),
                    &[cx, cy, 1.0 - k, 1.0 + k],
                    None,
                )
            }
            E::LensChromaticAberration {
                mode,
                amount,
                center,
                angle,
                falloff,
                red_scale,
                green_scale,
                blue_scale,
                fringe_blur,
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let (w0, h0) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let rc = (w0 * w0 + h0 * h0).sqrt() / 2.0;
                let (ux, uy) = crate::blurs::along(*angle);
                let scales = [
                    *red_scale / 100.0,
                    *green_scale / 100.0,
                    *blue_scale / 100.0,
                ];
                let b = *fringe_blur / 100.0;
                let n = crate::effects::lens_aberration_counts(*amount, scales, b);
                let p = FxParams {
                    flag: if mode == "offset" { 2 } else { 1 },
                    ..Default::default()
                };
                let k = [
                    cx,
                    cy,
                    rc,
                    amount / rc,
                    2.0 * falloff / 100.0,
                    ux,
                    uy,
                    *amount,
                    b,
                    scales[0],
                    scales[1],
                    scales[2],
                    n[0] as f64,
                    n[1] as f64,
                    n[2] as f64,
                ];
                same(steps, &passes.aberr, p, &k, None)
            }
            E::RimLight {
                color,
                direction,
                width,
                softness,
                intensity,
                blend: b,
            } => {
                let (blurred, r) = covering(steps, still, (w, h), softness / 3.0);
                let (ux, uy) = crate::blurs::along(*direction);
                let mut k = vec![width * ux, width * uy, intensity / 100.0];
                k.extend(linear(color));
                let p = FxParams {
                    r: r as i32,
                    blend: blend(b),
                    ..Default::default()
                };
                same(steps, &passes.rim, p, &k, Some(&blurred))
            }
            E::DropShadow {
                color,
                opacity,
                direction,
                distance,
                softness,
            } => {
                let (blurred, r) = covering(steps, still, (w, h), softness / 3.0);
                let g = distance.ceil() as usize + r;
                let (ux, uy) = crate::blurs::along(*direction);
                let mut k = vec![distance * ux, distance * uy, opacity / 100.0];
                k.extend(linear(color));
                let out = self.scratch("B-65 shadow", w + 2 * g, h + 2 * g);
                let p = FxParams {
                    g: g as i32,
                    r: r as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.shadow,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&blurred),
                    none,
                    tiles(w + 2 * g, h + 2 * g),
                );
                (out, (w + 2 * g, h + 2 * g))
            }
            E::Outline {
                color,
                width,
                softness,
                opacity,
            } => {
                let n = width.ceil() as usize;
                let (bw, bh) = (w + 2 * n, h + 2 * n);
                let runs = crate::layer_fx::disc_runs(*width);
                let (row, band) = (buffer(h * bw * 4), buffer(bw * bh * 4));
                let top = runs.iter().map(|&(_, hw)| hw).max().unwrap_or(0);
                for k in 0..=top {
                    // Each row's greatest covering within k, widened from within k - 1.
                    let p = FxParams {
                        n: n as u32,
                        count: k as u32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.rows,
                        p,
                        Some(still),
                        None,
                        None,
                        None,
                        [Some(&row), None, None, None],
                        tiles(bw, h),
                    );
                    let dys: Vec<f64> = runs
                        .iter()
                        .filter(|&&(_, hw)| hw == k)
                        .map(|&(dy, _)| dy as f64)
                        .collect();
                    if !dys.is_empty() {
                        let p = FxParams {
                            n: n as u32,
                            count: dys.len() as u32,
                            ..Default::default()
                        };
                        self.fx_step(
                            steps,
                            &passes.bands,
                            p,
                            Some(still),
                            None,
                            Some(&dys),
                            None,
                            [Some(&row), Some(&band), None, None],
                            tiles(bw, bh),
                        );
                    }
                }
                let lifted = self.scratch("B-65 band", bw, bh);
                self.fx_step(
                    steps,
                    &passes.lift,
                    FxParams::default(),
                    None,
                    Some(&lifted),
                    None,
                    None,
                    [None, Some(&band), None, None],
                    tiles(bw, bh),
                );
                let (blurred, r) = covering(steps, &lifted, (bw, bh), softness / 3.0);
                let g = n + r;
                let mut k = vec![opacity / 100.0];
                k.extend(linear(color));
                let out = self.scratch("B-65 outline", w + 2 * g, h + 2 * g);
                let p = FxParams {
                    g: g as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.ring,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&blurred),
                    none,
                    tiles(w + 2 * g, h + 2 * g),
                );
                (out, (w + 2 * g, h + 2 * g))
            }
            E::LensBlur {
                radius,
                edges,
                iris,
                roundness,
                rotation,
                aspect,
                highlight_gain,
                highlight_threshold,
                channel,
                focal_distance,
                invert,
                map,
                ..
            } => {
                let repeat = edges == "repeat";
                let g = if repeat {
                    0
                } else {
                    crate::layer_fx::lens_reach(*radius, *aspect).ceil() as usize
                };
                let blades = crate::layer_fx::blades(iris).unwrap_or(0);
                // The threshold and the gain as the CPU rounds them to single precision.
                let (at, m) = (
                    (highlight_threshold / 100.0) as f32 as f64,
                    (1.0 + highlight_gain) as f32 as f64,
                );
                let p = FxParams {
                    mode: repeat as u32,
                    flag: (*highlight_gain > 0.0) as u32,
                    g: g as i32,
                    ..Default::default()
                };
                let sums = buffer((w + 1) * h * 32);
                let out = self.scratch("B-65 lens", w + 2 * g, h + 2 * g);
                if let Some(map) = map {
                    // D-359: a header, each level's count and runs, then the runs; compose leaves
                    // a named map that was not read to draw the layer without the effect.
                    let shape = crate::layer_fx::Iris {
                        blades,
                        roundness: *roundness,
                        rotation: *rotation,
                        aspect: *aspect,
                        gain: *highlight_gain,
                        threshold: *highlight_threshold,
                    };
                    let levels = crate::layer_fx::lens_levels(*radius, &shape);
                    let (alpha, turned) = (
                        (channel == "alpha") as u8 as f64,
                        (invert == "on") as u8 as f64,
                    );
                    let mut k = vec![
                        (levels.len() - 1) as f64,
                        at,
                        m,
                        focal_distance / 255.0,
                        alpha,
                        turned,
                        ox as f64,
                        oy as f64,
                    ];
                    let mut start = 8 + 3 * levels.len();
                    for runs in &levels {
                        k.push(runs.iter().map(|&[_, lo, hi]| hi - lo + 1).sum::<isize>() as f64);
                        k.extend([start as f64, runs.len() as f64]);
                        start += 3 * runs.len();
                    }
                    k.extend(levels.iter().flatten().flatten().map(|&v| v as f64));
                    self.fx_step(
                        steps,
                        &passes.prefix,
                        p,
                        Some(still),
                        None,
                        Some(&k),
                        None,
                        [None, None, Some(&sums), None],
                        ((h as u32).div_ceil(64), 1),
                    );
                    let picture = self.map_texture(&map.0);
                    self.fx_step(
                        steps,
                        &passes.lmgather,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&k),
                        Some(&picture),
                        [None, None, Some(&sums), None],
                        tiles(w + 2 * g, h + 2 * g),
                    );
                } else {
                    let runs =
                        crate::layer_fx::iris_runs(*radius, blades, *roundness, *rotation, *aspect);
                    let n: isize = runs.iter().map(|&[_, lo, hi]| hi - lo + 1).sum();
                    let mut k = vec![n as f64, at, m];
                    k.extend(runs.iter().flatten().map(|&v| v as f64));
                    let p = FxParams {
                        count: runs.len() as u32,
                        ..p
                    };
                    self.fx_step(
                        steps,
                        &passes.prefix,
                        p,
                        Some(still),
                        None,
                        Some(&k),
                        None,
                        [None, None, Some(&sums), None],
                        ((h as u32).div_ceil(64), 1),
                    );
                    self.fx_step(
                        steps,
                        &passes.gather,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&k),
                        None,
                        [None, None, Some(&sums), None],
                        tiles(w + 2 * g, h + 2 * g),
                    );
                }
                (out, (w + 2 * g, h + 2 * g))
            }
            // B-76: the second batch.
            E::ExposureFlicker {
                amount,
                hold,
                seed,
                frame,
            } => {
                let gain =
                    2f64.powf(crate::effects::flicker_stops(*amount, *hold, *seed, *frame)) as f32;
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 5,
                        ..Default::default()
                    },
                    &[gain as f64],
                    None,
                )
            }
            E::ColorBalance {
                shadows,
                midtones,
                highlights,
                preserve_luminosity,
            } => {
                let mut k: Vec<f64> = [shadows, midtones, highlights]
                    .iter()
                    .flat_map(|t| t[..3].to_vec())
                    .collect();
                k.push(if preserve_luminosity == "on" {
                    1.0
                } else {
                    0.0
                });
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 6,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::GradientMap {
                shadow_color,
                midtone_color,
                highlight_color,
                midpoint,
                amount,
            } => {
                let mut k = vec![midpoint / 100.0, amount / 100.0];
                for c in [shadow_color, midtone_color, highlight_color] {
                    k.extend(crate::effects::encoded(c));
                }
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 7,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-401: Gradient Map's pass, the midtone halfway.
            E::TintMap {
                map_black_to,
                map_white_to,
                amount_to_tint,
            } => {
                let mut k = vec![0.5, amount_to_tint / 100.0];
                k.extend(crate::effects::tint_ramp(map_black_to, map_white_to).concat());
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 7,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-402: Gradient Map's pass, the midpoint at the middle.
            E::Tritone {
                highlights,
                midtones,
                shadows,
                blend_with_original,
            } => {
                let mut k = vec![0.5, 1.0 - blend_with_original / 100.0];
                k.extend(crate::effects::tritone_ramp(highlights, midtones, shadows).concat());
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 7,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Vignette {
                amount,
                color,
                size,
                roundness,
                softness,
                center,
            } => {
                let v = crate::effects::vignette_settings(
                    *amount,
                    color,
                    [*size, *roundness, *softness],
                    *center,
                    (w, h),
                    f.origin,
                );
                let mut k = vec![
                    v.center.0,
                    v.center.1,
                    v.radii.0,
                    v.radii.1,
                    v.inner,
                    v.outer,
                    v.amount,
                    2f64.sqrt(),
                ];
                k.extend(v.color.map(crate::grade::to_linear));
                k.push(v.norm);
                same(
                    steps,
                    &passes.grade,
                    FxParams {
                        mode: 8,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::FractalNoise {
                size,
                complexity,
                contrast,
                brightness,
                evolution,
                speed,
                seed,
                dark_color,
                light_color,
                opacity,
                blend: b,
                fractal_type,
                noise_type,
                invert,
                offset,
                scale_width,
                scale_height,
                cycle,
                frame,
                float: _,
                more,
            } => {
                let base = crate::grade::mix(seed.floor() as u64);
                let mut k = vec![
                    size * (scale_width / 100.0),
                    crate::effects::depth(*evolution, *speed, *frame),
                    *contrast,
                    *brightness,
                    opacity / 100.0,
                ];
                k.extend(crate::effects::encoded(dark_color));
                k.extend(crate::effects::encoded(light_color));
                // D-299.
                let look = crate::effects::fractal_look(fractal_type, noise_type, *cycle);
                let on = |b: bool| b as u8 as f64;
                k.extend([
                    size * (scale_height / 100.0),
                    offset[0],
                    offset[1],
                    on(look.kind == 1),
                    on(look.noise == 1),
                    look.cycle as f64,
                    on(invert == "on"),
                ]);
                // D-567: the types past basic, turbulent, smooth and block, mode 111.
                let wide = look.kind > 1 || look.noise > 1;
                k.extend([look.kind as f64, look.noise as f64]);
                // D-647: the Overflow, then the Transform and Sub Settings, mode 112.
                k.push(
                    crate::effects::OVERFLOWS
                        .iter()
                        .position(|m| *m == more.overflow)
                        .unwrap_or(0) as f64,
                );
                let warp = crate::effects::noise_warp(
                    more,
                    *offset,
                    complexity.floor() as usize,
                    (w - 2 * ox, h - 2 * oy),
                );
                if let Some(g) = &warp {
                    k.extend(g.middle);
                    k.extend(g.sub_offset);
                    k.extend([on(g.perspective), g.influence, g.sub_scaling, on(g.center)]);
                    k.extend(g.turns.iter().flatten());
                }
                let noise_blend = match b.as_str() {
                    "hard_light" => 6,
                    "darken" => 7,
                    "lighten" => 8,
                    "difference" => 9,
                    // D-730.
                    "none" => 10,
                    "stencil_alpha" => 11,
                    "stencil_luma" => 12,
                    "saturation" => 13,
                    "color" => 14,
                    "luminosity" => 15,
                    // D-731; Overlay is grade::mixer's, code 4 from `blend`.
                    "hue" => 16,
                    _ => blend(b),
                };
                let p = FxParams {
                    mode: if warp.is_some() {
                        112
                    } else if wide {
                        111
                    } else {
                        9
                    },
                    blend: noise_blend,
                    count: complexity.floor() as u32,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.grade, p, &k, None)
            }
            E::TurbulentDisplace {
                amount,
                size,
                complexity,
                evolution,
                speed,
                seed,
                edges,
                frame,
                units,
                new_seed_every,
                drift_direction,
                drift_speed,
                ..
            } => {
                let repeat = edges == "repeat";
                // D-328: the push, which is the amount in classic units.
                let push = crate::effects::turbulent_push(*amount, *size, units);
                let g = if repeat { 0 } else { push.ceil() as usize };
                // D-410: the seed for the frame.
                let base = crate::grade::mix(crate::effects::turbulent_seed(
                    *seed,
                    *new_seed_every,
                    *frame,
                ));
                // D-411: the drift slides the drawing's corner the field is read from.
                let d = crate::effects::turbulent_drift(*drift_direction, *drift_speed, *frame);
                let k = [
                    push,
                    *size,
                    crate::effects::depth(*evolution, *speed, *frame),
                    (ox + g) as f64 + d.0,
                    (oy + g) as f64 + d.1,
                ];
                let p = FxParams {
                    count: complexity.floor() as u32,
                    flag: repeat as u32,
                    g: g as i32,
                    base: [base as u32, (base >> 32) as u32],
                    ..Default::default()
                };
                let out = self.scratch("B-76 turbulent", w + 2 * g, h + 2 * g);
                self.fx_step(
                    steps,
                    &passes.turb,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(w + 2 * g, h + 2 * g),
                );
                (out, (w + 2 * g, h + 2 * g))
            }
            E::Offset { shift } => {
                let (qx, qy) = (-shift[0], -shift[1]);
                let (i0, j0) = (qx.floor(), qy.floor());
                // Whole turns of the drawing change nothing, and keep the sums small.
                let inside = |v: f64, n: usize| (v as i64).rem_euclid(n as i64) as i32;
                let p = FxParams {
                    ox: inside(i0, w),
                    oy: inside(j0, h),
                    ..Default::default()
                };
                same(steps, &passes.slide, p, &[qx - i0, qy - j0], None)
            }
            E::LightRays {
                center,
                length,
                threshold,
                intensity,
                color,
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("Light Rays are refused without the passes");
                let none = self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("B-76 none"),
                        contents: bytemuck::cast_slice(&[0.0f32; 3]),
                        usage: wgpu::BufferUsages::STORAGE,
                    });
                let lit = self.scratch("B-76 bright", w, h);
                let p = Params {
                    level: crate::bloom::bright_level(*threshold),
                    ..Default::default()
                };
                self.step(
                    steps,
                    &bloom.bright,
                    p,
                    still,
                    Some(&lit),
                    None,
                    Some(&none),
                    tiles(w, h),
                );
                let center = crate::effects::radial_center(*center, (w, h), f.origin);
                let rays = self.blur(
                    steps,
                    &lit,
                    (w, h),
                    Radial {
                        spin: false,
                        amount: *length,
                        center,
                        repeat: false,
                        sweep: None,
                    },
                );
                let mut k = vec![*intensity];
                k.extend(linear(color));
                k.extend([0.0, 0.0]);
                same(steps, &passes.rays, FxParams::default(), &k, Some(&rays))
            }
            // D-422: the layer itself is the light, through Spin & Zoom Blur's zoom, then Light
            // Rays' last step.
            E::LightBurst {
                center,
                intensity,
                ray_length,
                burst,
                set_color,
                color,
            } => {
                let center = crate::effects::radial_center(*center, (w, h), f.origin);
                let sweep = Some(crate::effects::burst_sweep(burst, *ray_length));
                let rays = self.blur(
                    steps,
                    still,
                    (w, h),
                    Radial {
                        spin: false,
                        amount: *ray_length,
                        center,
                        repeat: false,
                        sweep,
                    },
                );
                let on = set_color == "on";
                let mut k = vec![intensity / 100.0];
                if on {
                    k.extend(linear(color));
                } else {
                    k.extend([1.0; 3]);
                }
                k.extend([on as u8 as f64, 0.0]);
                same(steps, &passes.rays, FxParams::default(), &k, Some(&rays))
            }
            // D-423: the layer times the source's share, zoomed out and spun, then the rays pass
            // in its transfer mode.
            E::CcLightRays {
                intensity,
                center,
                radius,
                warp_softness,
                shape,
                direction,
                color_from_source,
                allow_brightening,
                color,
                transfer_mode,
            } => {
                let center = crate::effects::radial_center(*center, (w, h), f.origin);
                let (s, c) = direction.to_radians().sin_cos();
                let k = [
                    center.0,
                    center.1,
                    *radius,
                    (shape == "square") as u8 as f64,
                    s,
                    c,
                ];
                let (lit, _) = same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 33,
                        ..Default::default()
                    },
                    &k,
                    None,
                );
                let zoomed = self.blur(
                    steps,
                    &lit,
                    (w, h),
                    Radial {
                        spin: false,
                        amount: 100.0,
                        center,
                        repeat: false,
                        sweep: None,
                    },
                );
                let turn = warp_softness / 10.0;
                let rays = if turn == 0.0 {
                    zoomed
                } else {
                    self.blur(
                        steps,
                        &zoomed,
                        (w, h),
                        Radial {
                            spin: true,
                            amount: turn,
                            center,
                            repeat: false,
                            sweep: None,
                        },
                    )
                };
                let tinted = color_from_source == "off";
                let mut k = vec![if allow_brightening == "on" {
                    intensity / 100.0
                } else {
                    (intensity / 100.0).min(1.0)
                }];
                if tinted {
                    k.extend(linear(color));
                } else {
                    k.extend([1.0; 3]);
                }
                k.extend([
                    tinted as u8 as f64,
                    crate::effects::ray_mode(transfer_mode) as f64,
                ]);
                same(steps, &passes.rays, FxParams::default(), &k, Some(&rays))
            }
            E::DistanceGradation {
                color,
                width,
                opacity,
                invert,
                blend: b,
            } => {
                let dist = buffer(dist_bytes(w, h));
                let mut k = vec![*width, *opacity, (invert == "on") as u8 as f64];
                k.extend(linear(color));
                let p = FxParams {
                    blend: blend(b),
                    ..Default::default()
                };
                let work = [None, None, None, Some(&dist)];
                self.fx_step(
                    steps,
                    &passes.cols,
                    p,
                    Some(still),
                    None,
                    None,
                    None,
                    work,
                    (((w + 2) as u32).div_ceil(64), 1),
                );
                let out = self.scratch("B-76 distance", w, h);
                self.fx_step(
                    steps,
                    &passes.edt,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    work,
                    ((h as u32).div_ceil(64), 1),
                );
                (out, (w, h))
            }
            // B-107: the third batch, Motion Tile aside.
            E::Invert { channel, amount } => {
                let (mode, count) = match channel.as_str() {
                    "alpha" => (1, 0),
                    "red" => (0, 0),
                    "green" => (0, 1),
                    "blue" => (0, 2),
                    _ => (0, 3),
                };
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode,
                        count,
                        ..Default::default()
                    },
                    &[amount / 100.0],
                    None,
                )
            }
            E::BrightnessContrast {
                brightness,
                contrast,
            } => {
                let k = if *contrast <= 0.0 {
                    1.0 + contrast / 100.0
                } else {
                    1.0 / (1.0 - 0.99 * contrast / 100.0)
                };
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 2,
                        ..Default::default()
                    },
                    &[k, brightness / 255.0],
                    None,
                )
            }
            E::BlackWhite {
                reds,
                yellows,
                greens,
                cyans,
                blues,
                magentas,
            } => {
                let k = [*reds, *yellows, *greens, *cyans, *blues, *magentas];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 3,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Posterize { levels } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 4,
                    ..Default::default()
                },
                &[levels.floor()],
                None,
            ),
            E::Threshold { level } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 5,
                    ..Default::default()
                },
                &[*level],
                None,
            ),
            E::ChannelMixer {
                red,
                green,
                blue,
                monochrome,
            } => {
                let rows = if monochrome == "on" {
                    [red, red, red]
                } else {
                    [red, green, blue]
                };
                let k: Vec<f64> = rows.iter().flat_map(|r| r[..4].to_vec()).collect();
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 6,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Vibrance {
                vibrance,
                saturation,
            } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 7,
                    ..Default::default()
                },
                &[*vibrance, *saturation],
                None,
            ),
            E::LeaveColor {
                color,
                tolerance,
                softness,
                amount,
            } => {
                let hc = crate::grade::hsv_hue(crate::effects::encoded(color)).unwrap_or(-1.0);
                let k = [hc, tolerance / 100.0, softness / 100.0, amount / 100.0];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 8,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Solarize { threshold } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 9,
                    ..Default::default()
                },
                &[*threshold],
                None,
            ),
            E::Halftone {
                size,
                angle,
                ink,
                paper,
                amount,
            } => {
                let (sin, cos) = angle.to_radians().sin_cos();
                let mut k = vec![sin, cos, *size, amount / 100.0];
                k.extend(linear(ink));
                k.extend(linear(paper));
                let p = FxParams {
                    mode: 10,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.tone, p, &k, None)
            }
            E::Mosaic { size } => (mosaic(steps, *size, false), (w, h)),
            // D-449: compose left out one with every pass done.
            E::BlockLoad {
                completion,
                scans,
                start_cleared,
                bilinear,
                pixel,
            } => {
                let (e, empty, f) =
                    crate::layer_fx::block_load_pass(*completion, *scans, start_cleared == "on")
                        .expect("compose leaves out a Block Load with every pass done");
                let size = 2f64.powi(e) * pixel;
                let bilinear = bilinear == "on";
                let now = mosaic(steps, size, bilinear);
                let edge = crate::layer_fx::block_load_edge(h, oy, size, f);
                if edge >= h {
                    return (now, (w, h));
                }
                let before = if empty {
                    now.clone()
                } else {
                    mosaic(steps, size * 2.0, bilinear)
                };
                let out = self.scratch("D-449 block load", w, h);
                let p = FxParams {
                    mode: empty as u32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.blockload,
                    p,
                    Some(&now),
                    Some(&out),
                    Some(&[edge as f64]),
                    Some(&before),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-460: compose left out one at Burn 0.
            E::BurnFilm { burn, center, seed } => {
                let (base, k) = crate::layer_fx::burn_film_setup((w, h), *burn, *center, *seed);
                let p = FxParams {
                    base: [base as u32, (base >> 32) as u32],
                    ..Default::default()
                };
                same(steps, &passes.burnfilm, p, &k, None)
            }
            // D-461: one warp pass, mode 100, its sines worked here.
            E::HexTile {
                render,
                radius,
                center,
                lock_center,
                rotate,
                smearing,
            } => {
                let k = crate::layer_fx::hex_tile_setup(
                    render,
                    *radius,
                    *center,
                    lock_center == "on",
                    *rotate,
                    *smearing,
                    (w, h),
                    f.origin,
                );
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 100,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Emboss {
                direction,
                relief,
                contrast,
                mode,
            } => {
                let (ux, uy) = crate::blurs::along(*direction);
                let k = [
                    relief * ux,
                    relief * uy,
                    contrast / 100.0,
                    (mode == "color") as u8 as f64,
                ];
                same(steps, &passes.relief, FxParams::default(), &k, None)
            }
            E::FindEdges { invert, amount } => {
                let k = [amount / 100.0, (invert == "on") as u8 as f64];
                same(
                    steps,
                    &passes.relief,
                    FxParams {
                        mode: 1,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Sharpen { amount, radius, .. } => {
                let (blurred, r) = covering(steps, still, (w, h), *radius);
                same(
                    steps,
                    &passes.sharp,
                    FxParams {
                        r: r as i32,
                        ..Default::default()
                    },
                    &[amount / 100.0],
                    Some(&blurred),
                )
            }
            E::Diffusion {
                radius,
                amount,
                blend: b,
                second_amount,
                second_blend,
            } => {
                let (blurred, r) = covering(steps, still, (w, h), radius / 3.0);
                let blend = match b.as_str() {
                    "screen" => 1,
                    "lighten" => 2,
                    _ => 0,
                };
                let k = [
                    amount / 100.0,
                    second_amount / 100.0,
                    if second_blend == "overlay" { 4.0 } else { 5.0 },
                ];
                same(
                    steps,
                    &passes.sharp,
                    FxParams {
                        mode: 1,
                        blend,
                        r: r as i32,
                        ..Default::default()
                    },
                    &k,
                    Some(&blurred),
                )
            }
            E::ShadowHighlight {
                shadow_amount,
                highlight_amount,
                shadow_tonal_width,
                shadow_radius,
                highlight_tonal_width,
                highlight_radius,
                color_correction,
                midtone_contrast,
                ..
            } => {
                // D-400, as grade::shadow_highlight: the lightness, blurred at each radius (the
                // highlights' blur copied into green, as Channel Blur's channels), then the rule.
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                let plane = self.scratch("D-400 lightness", w, h);
                self.fx_step(
                    steps,
                    &passes.shprep,
                    FxParams::default(),
                    Some(still),
                    Some(&plane),
                    None,
                    None,
                    none,
                    tiles(w, h),
                );
                let blurred = |steps: &mut Vec<Step>, sigma: f64| {
                    let taps = crate::effects::gaussian_weights(sigma);
                    self.gauss(
                        steps,
                        bloom,
                        "D-400 blur",
                        &plane,
                        (w, h),
                        [&taps, &taps],
                        0,
                        true,
                    )
                };
                let mut lit = blurred(steps, *shadow_radius);
                if highlight_radius != shadow_radius {
                    let b = blurred(steps, *highlight_radius);
                    let next = self.scratch("D-400 blurs", w, h);
                    self.fx_step(
                        steps,
                        &passes.chanmix,
                        FxParams {
                            count: 1,
                            ..Default::default()
                        },
                        Some(&lit),
                        Some(&next),
                        None,
                        Some(&b),
                        none,
                        tiles(w, h),
                    );
                    lit = next;
                }
                let k = [
                    2.0 * shadow_amount / 100.0,
                    2.0 * highlight_amount / 100.0,
                    (1.0 - shadow_tonal_width / 100.0).min(0.99),
                    (1.0 - highlight_tonal_width / 100.0).min(0.99),
                    color_correction / 100.0,
                    midtone_contrast / 100.0,
                ];
                same(steps, &passes.shhi, FxParams::default(), &k, Some(&lit))
            }
            E::WaveWarp {
                shape,
                height,
                width,
                direction,
                speed,
                phase,
                edges,
                frame,
            } => {
                let repeat = edges == "repeat";
                let g = if repeat { 0 } else { height.ceil() as usize };
                let (tx, ty) = crate::blurs::along(*direction);
                let phi = (phase + speed * *frame as f64).to_radians();
                let k = [
                    (ox + g) as f64,
                    (oy + g) as f64,
                    tx,
                    ty,
                    -ty,
                    tx,
                    *width,
                    phi,
                    *height,
                    (shape == "triangle") as u8 as f64,
                ];
                let out = self.scratch("B-107 wave", w + 2 * g, h + 2 * g);
                let p = FxParams {
                    flag: repeat as u32,
                    g: g as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.warp,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(w + 2 * g, h + 2 * g),
                );
                (out, (w + 2 * g, h + 2 * g))
            }
            E::Ripple {
                center,
                amplitude,
                wavelength,
                speed,
                phase,
                fade,
                frame,
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let k = [
                    cx,
                    cy,
                    *amplitude,
                    *wavelength,
                    (phase + speed * *frame as f64).to_radians(),
                    *fade,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 1,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Twirl {
                angle,
                radius,
                center,
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 2,
                        ..Default::default()
                    },
                    &[cx, cy, angle.to_radians(), *radius],
                    None,
                )
            }
            E::Bulge {
                center,
                radius,
                height,
                ..
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 3,
                        ..Default::default()
                    },
                    &[cx, cy, *height, *radius],
                    None,
                )
            }
            // D-406: Bulge's branch of the warp pass, mode 24.
            E::Spherize { radius, center } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 24,
                        ..Default::default()
                    },
                    &[cx, cy, 0.0, *radius],
                    None,
                )
            }
            // D-407: softened first as `covering` blurs, the Lanczos sum in the warp pass (mode
            // 29) grown as compose grew it, then Detail in the sharpen pass (mode 2).
            E::DetailUpscale {
                scale,
                reduce_noise,
                detail,
            } => {
                let (gx, gy) = f.grow;
                let (tw, th) = (w + 2 * gx, h + 2 * gy);
                let s = scale / 100.0;
                let (soft, r) = if *reduce_noise > 0.0 {
                    covering(steps, still, (w, h), reduce_noise / 50.0)
                } else {
                    (still.clone(), 0)
                };
                let mut k = vec![0.0];
                for (n, big) in [(w, tw), (h, th)] {
                    for x in 0..big {
                        let (first, weights) = crate::layer_fx::lanczos_taps(n, big, s, x);
                        k.push(first as f64);
                        k.extend(weights);
                    }
                }
                let up = self.scratch("D-407 upscale", tw, th);
                let p = FxParams {
                    mode: 29,
                    r: r as i32,
                    flag: (*detail <= 0.0) as u32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.warp,
                    p,
                    Some(&soft),
                    Some(&up),
                    Some(&k),
                    None,
                    none,
                    tiles(tw, th),
                );
                if *detail <= 0.0 {
                    return (up, (tw, th));
                }
                let (blurred, r2) = covering(steps, &up, (tw, th), s / 2.0);
                let out = self.scratch("D-407 detail", tw, th);
                let p = FxParams {
                    mode: 2,
                    r: r2 as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.sharp,
                    p,
                    Some(&up),
                    Some(&out),
                    Some(&[detail / 50.0]),
                    Some(&blurred),
                    none,
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            // D-377: the frames at End and at -L worked out here as the CPU works them.
            E::BendIt {
                bend,
                start,
                end,
                render_prestart,
                distort,
            } => {
                let (s, e) = (
                    crate::effects::radial_center(*start, (w, h), f.origin),
                    crate::effects::radial_center(*end, (w, h), f.origin),
                );
                let length = (e.0 - s.0).hypot(e.1 - s.1);
                let (tx, ty) = ((e.0 - s.0) / length, (e.1 - s.1) / length);
                let (nx, ny) = (-ty, tx);
                let theta = bend.to_radians();
                let (r, (cx, cy)) = if theta == 0.0 {
                    (0.0, (0.0, 0.0))
                } else {
                    (
                        length / theta,
                        (s.0 + length / theta * nx, s.1 + length / theta * ny),
                    )
                };
                let frame = |phi: f64| {
                    let (sn, c) = phi.sin_cos();
                    let (t, n) = (
                        (tx * c + nx * sn, ty * c + ny * sn),
                        (nx * c - tx * sn, ny * c - ty * sn),
                    );
                    [cx - r * n.0, cy - r * n.1, t.0, t.1, n.0, n.1]
                };
                let pre = crate::effects::BEND_IT_PRESTARTS
                    .iter()
                    .position(|p| p == render_prestart)
                    .expect("compose leaves a valid Bend It");
                let lo = if pre >= 2 { -length } else { 0.0 };
                let mut k = vec![
                    s.0,
                    s.1,
                    tx,
                    ty,
                    nx,
                    ny,
                    length,
                    theta,
                    r,
                    cx,
                    cy,
                    theta / length,
                    lo,
                    pre as f64,
                    (distort == "extended") as u8 as f64,
                ];
                k.extend(frame(theta));
                k.extend(frame(-theta));
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 12,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Bender {
                amount,
                style,
                adjust_to_distance,
                top,
                base,
            } => {
                let (b, t) = (
                    crate::effects::radial_center(*base, (w, h), f.origin),
                    crate::effects::radial_center(*top, (w, h), f.origin),
                );
                let a = crate::effects::bender_amount(*amount, adjust_to_distance, b, t);
                let length = (t.0 - b.0).hypot(t.1 - b.1);
                let (ux, uy) = ((t.0 - b.0) / length, (t.1 - b.1) / length);
                let kind = crate::effects::BENDER_STYLES
                    .iter()
                    .position(|s| s == style)
                    .expect("compose leaves a valid Bender");
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 13,
                        ..Default::default()
                    },
                    &[b.0, b.1, ux, uy, -uy, ux, length, a, kind as f64],
                    None,
                )
            }
            E::FlowMotion {
                knot_1,
                amount_1,
                knot_2,
                amount_2,
                falloff,
                tile_edges,
                finer_controls,
                antialiasing,
            } => {
                let ([((k1x, k1y), a1), ((k2x, k2y), a2)], sigma) =
                    crate::effects::flow_motion_knots(
                        [*knot_1, *knot_2],
                        [*amount_1, *amount_2],
                        *falloff,
                        finer_controls,
                        (w, h),
                        f.origin,
                    );
                let n = 1
                    << crate::effects::FLOW_MOTION_ANTIALIASING
                        .iter()
                        .position(|a| a == antialiasing)
                        .expect("compose leaves a valid Flow Motion");
                let k = [
                    k1x,
                    k1y,
                    a1,
                    k2x,
                    k2y,
                    a2,
                    sigma * sigma,
                    (tile_edges == "on") as u8 as f64,
                    n as f64,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 14,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Griddler {
                horizontal_scale,
                vertical_scale,
                tile_size,
                rotation,
                cut_tiles,
            } => {
                let (sin, cos) = rotation.to_radians().sin_cos();
                let tile = tile_size / 100.0 * (w - 2 * f.origin.0) as f64;
                let k = [
                    f.origin.0 as f64,
                    f.origin.1 as f64,
                    tile,
                    sin,
                    cos,
                    horizontal_scale / 100.0,
                    vertical_scale / 100.0,
                    (cut_tiles == "on") as u8 as f64,
                    0.0,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 15,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Fisheye {
                center,
                size,
                convergence,
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let k = [
                    cx,
                    cy,
                    crate::effects::fisheye_radius(*size, (w, h), f.origin),
                    convergence / 100.0,
                    0.0,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 16,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-388, layer_fx::page_turn; a corner turned onto itself is the drawing as it was.
            E::PageTurn {
                controls,
                fold_position,
                fold_direction,
                fold_radius,
                light_direction,
                render,
                back_opacity,
                paper_color,
                map,
                ..
            } => {
                let (dw, dh) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let Some(((n0, n1), off)) = crate::effects::page_turn_fold(
                    controls,
                    *fold_position,
                    *fold_direction,
                    *fold_radius,
                    (dw, dh),
                ) else {
                    return (still.clone(), (w, h));
                };
                let (lx, ly) = crate::blurs::along(*light_direction);
                let paper = linear(paper_color);
                let r = crate::effects::PAGE_TURN_RENDERS
                    .iter()
                    .position(|x| x == render)
                    .unwrap_or(0);
                let back = map.as_ref().map(|m| self.map_texture(&m.0));
                let k = [
                    n0,
                    n1,
                    off,
                    *fold_radius,
                    lx * n0 + ly * n1,
                    back_opacity / 100.0,
                    paper[0],
                    paper[1],
                    paper[2],
                    r as f64,
                    back.is_some() as u8 as f64,
                    ox as f64,
                    oy as f64,
                    0.0,
                ];
                let out = self.scratch("D-388 page turn", w, h);
                self.fx_step(
                    steps,
                    &passes.pageturn,
                    FxParams::default(),
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(back.as_ref().unwrap_or(still)),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-389, layer_fx::power_pin; no ring leaves the k[36] flag 0, which draws nothing.
            E::PowerPin {
                top_left,
                top_right,
                bottom_left,
                bottom_right,
                perspective,
                unstretch,
                expansion_top,
                expansion_left,
                expansion_right,
                expansion_bottom,
            } => {
                let p = perspective / 100.0;
                let on = unstretch == "on";
                let (gx, gy) = if on { (0, 0) } else { f.grow };
                let pins = [*top_left, *top_right, *bottom_left, *bottom_right];
                let e = [
                    *expansion_top,
                    *expansion_left,
                    *expansion_right,
                    *expansion_bottom,
                ];
                let mut k = [0.0; 37];
                if let Some((q, mq, adj, det, _)) =
                    crate::layer_fx::power_pin_map(pins, p, e, (w, h), f.origin)
                {
                    k[..9].copy_from_slice(adj.as_flattened());
                    k[9] = det;
                    k[10..19].copy_from_slice(mq.as_flattened());
                    for (i, c) in q.iter().enumerate() {
                        k[19 + 2 * i] = c.0;
                        k[20 + 2 * i] = c.1;
                    }
                    k[36] = 1.0;
                }
                k[27..35].copy_from_slice(&[
                    (w - 2 * ox) as f64,
                    (h - 2 * oy) as f64,
                    ox as f64,
                    oy as f64,
                    (ox + gx) as f64,
                    (oy + gy) as f64,
                    p,
                    on as u8 as f64,
                ]);
                let (tw, th) = (w + 2 * gx, h + 2 * gy);
                let out = self.scratch("D-389 power pin", tw, th);
                self.fx_step(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 17,
                        ..Default::default()
                    },
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            // D-607, mesh::smear, drawn the same way.
            E::MaskSmear {
                mask_offset,
                mask_rotation,
                mask_scale,
                percent,
                elasticity,
                paths,
                ..
            } => match paths.as_ref().filter(|_| *percent != 0.0) {
                None => (still.clone(), (w, h)),
                Some(paths) => {
                    let field = crate::mesh::smear(
                        paths,
                        *mask_offset,
                        *mask_rotation,
                        *mask_scale,
                        *percent,
                        elasticity,
                        ((w - 2 * ox) as f64, (h - 2 * oy) as f64),
                    );
                    let k = crate::mesh::field_numbers(&field, 1.0, [0.0, 0.0], (w, h), f.origin);
                    same(
                        steps,
                        &passes.warp,
                        FxParams {
                            mode: 151,
                            ..Default::default()
                        },
                        &k,
                        None,
                    )
                }
            },
            // D-606, mesh::reshape, drawn the same way.
            E::Reshape {
                percent,
                elasticity,
                correspondence_points,
                between,
                paths,
                ..
            } => match paths.as_ref().and_then(|paths| {
                crate::mesh::reshape(
                    paths,
                    *percent,
                    between,
                    correspondence_points,
                    elasticity,
                    ((w - 2 * ox) as f64, (h - 2 * oy) as f64),
                )
            }) {
                None => (still.clone(), (w, h)),
                Some(field) => {
                    let k = crate::mesh::field_numbers(&field, 1.0, [0.0, 0.0], (w, h), f.origin);
                    same(
                        steps,
                        &passes.warp,
                        FxParams {
                            mode: 151,
                            ..Default::default()
                        },
                        &k,
                        None,
                    )
                }
            },
            // D-605, D-700: mesh::liquify, worked on the processor, drawn as mesh::draw_field
            // draws it: warp mode 151.
            liquify @ E::Liquify {
                distortion_percentage,
                distortion_mesh_offset,
                ..
            } => match crate::mesh::liquify(liquify, ((w - 2 * ox) as f64, (h - 2 * oy) as f64)) {
                None => (still.clone(), (w, h)),
                Some(field) => {
                    let k = crate::mesh::field_numbers(
                        &field,
                        *distortion_percentage / 100.0,
                        *distortion_mesh_offset,
                        (w, h),
                        f.origin,
                    );
                    same(
                        steps,
                        &passes.warp,
                        FxParams {
                            mode: 151,
                            ..Default::default()
                        },
                        &k,
                        None,
                    )
                }
            },
            // D-604, mesh::warp_preset_mesh, drawn the same way.
            E::Warp {
                warp_style,
                warp_axis,
                bend,
                horizontal_distortion,
                vertical_distortion,
            } => {
                let size = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                match crate::mesh::warp_preset_mesh(
                    warp_style,
                    warp_axis,
                    *bend,
                    *horizontal_distortion,
                    *vertical_distortion,
                    size,
                ) {
                    Some(m) => same(
                        steps,
                        &passes.warp,
                        FxParams {
                            mode: 150,
                            ..Default::default()
                        },
                        &crate::mesh::mesh_numbers(&m, (w, h), f.origin),
                        None,
                    ),
                    None => (still.clone(), (w, h)),
                }
            }
            // D-603, mesh::warp_mesh, drawn the same way.
            E::MeshWarp {
                rows,
                columns,
                quality,
                distortion_mesh,
                vertex_handles,
            } => {
                match crate::mesh::warp_mesh(
                    *rows,
                    *columns,
                    *quality,
                    distortion_mesh,
                    vertex_handles,
                    ((w - 2 * ox) as f64, (h - 2 * oy) as f64),
                ) {
                    Some(m) => same(
                        steps,
                        &passes.warp,
                        FxParams {
                            mode: 150,
                            ..Default::default()
                        },
                        &crate::mesh::mesh_numbers(&m, (w, h), f.origin),
                        None,
                    ),
                    None => (still.clone(), (w, h)),
                }
            }
            // D-602, mesh::bezier_mesh, drawn as mesh::draw draws it: warp mode 150.
            E::BezierWarp { points, quality } => match crate::mesh::bezier_mesh(
                points,
                *quality,
                ((w - 2 * ox) as f64, (h - 2 * oy) as f64),
            ) {
                Some(m) => same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 150,
                        ..Default::default()
                    },
                    &crate::mesh::mesh_numbers(&m, (w, h), f.origin),
                    None,
                ),
                None => (still.clone(), (w, h)),
            },
            // D-390, layer_fx::ripple_pulse, with the levels compose read for this frame.
            E::RipplePulse {
                center,
                pulse_level,
                amplitude,
                render_bump_map,
                levels,
                ..
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let big = ((w - 2 * ox) as f64).hypot((h - 2 * oy) as f64) / 2.0;
                let now = [*pulse_level];
                let levels = if levels.is_empty() {
                    &now[..]
                } else {
                    &levels[..]
                };
                let mut k = vec![
                    cx,
                    cy,
                    (levels.len() - 1) as f64,
                    big,
                    *amplitude,
                    (render_bump_map == "on") as u8 as f64,
                    0.0,
                ];
                k.extend_from_slice(levels);
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 18,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-391..D-393, layer_fx::slant, smear and split, each a warp; no drag or no gap is
            // the drawing as it was.
            E::Slant {
                slant,
                stretching,
                height,
                floor,
                set_color,
                color,
            } => {
                let (_, fy) = crate::effects::radial_center(*floor, (w, h), f.origin);
                let theta = slant.to_radians();
                let s = height / 100.0 * if stretching == "on" { 1.0 } else { theta.cos() };
                let c = linear(color);
                let k = [
                    fy,
                    theta.tan(),
                    s,
                    (set_color == "on") as u8 as f64,
                    c[0],
                    c[1],
                    c[2],
                    0.0,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 19,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Smear {
                from,
                to,
                reach,
                radius,
            } => {
                let (fx, fy) = crate::effects::radial_center(*from, (w, h), f.origin);
                let (tx, ty) = crate::effects::radial_center(*to, (w, h), f.origin);
                let v = ((tx - fx) * reach / 100.0, (ty - fy) * reach / 100.0);
                if *radius <= 0.0 || v.0 * v.0 + v.1 * v.1 == 0.0 {
                    return (still.clone(), (w, h));
                }
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 20,
                        ..Default::default()
                    },
                    &[fx, fy, v.0, v.1, *radius, 0.0],
                    None,
                )
            }
            E::Split {
                point_a,
                point_b,
                split,
            } => {
                let (ax, ay) = crate::effects::radial_center(*point_a, (w, h), f.origin);
                let (bx, by) = crate::effects::radial_center(*point_b, (w, h), f.origin);
                let length = (bx - ax).hypot(by - ay);
                if length == 0.0 || *split <= 0.0 {
                    return (still.clone(), (w, h));
                }
                let k = [
                    ax,
                    ay,
                    (bx - ax) / length,
                    (by - ay) / length,
                    length,
                    *split,
                    0.0,
                    *split,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 21,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-394: the same pass, each side its own amount.
            E::Split2 {
                point_a,
                point_b,
                split_1,
                split_2,
            } => {
                let (ax, ay) = crate::effects::radial_center(*point_a, (w, h), f.origin);
                let (bx, by) = crate::effects::radial_center(*point_b, (w, h), f.origin);
                let length = (bx - ax).hypot(by - ay);
                if length == 0.0 || (*split_1 <= 0.0 && *split_2 <= 0.0) {
                    return (still.clone(), (w, h));
                }
                let k = [
                    ax,
                    ay,
                    (bx - ax) / length,
                    (by - ay) / length,
                    length,
                    *split_2,
                    0.0,
                    *split_1,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 21,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-458: compose leaves only a wire that changes the layer.
            E::SimpleWireRemoval {
                point_a,
                point_b,
                removal_style,
                thickness,
                slope,
                mirror_blend,
                picture,
                ..
            } => {
                let (ax, ay) = crate::effects::radial_center(*point_a, (w, h), f.origin);
                let (bx, by) = crate::effects::radial_center(*point_b, (w, h), f.origin);
                let length = (bx - ax).hypot(by - ay);
                let mode = crate::effects::WIRE_STYLES
                    .iter()
                    .position(|s| s == removal_style)
                    .expect("compose leaves a valid Simple Wire Removal")
                    as u32;
                let (hh, r) = (thickness / 2.0, thickness * (1.0 - slope / 100.0));
                let k = [
                    ax,
                    ay,
                    (bx - ax) / length,
                    (by - ay) / length,
                    length,
                    hh,
                    r,
                    hh + r,
                    mirror_blend / 100.0,
                    0.0,
                ];
                let other = match picture {
                    Some(m) if mode == 1 => self.map_texture(&m.0),
                    _ => still.clone(),
                };
                same(
                    steps,
                    &passes.wire,
                    FxParams {
                        mode,
                        ..Default::default()
                    },
                    &k,
                    Some(&other),
                )
            }
            // D-404: Motion Tile's sized tile held at the picture's size, as layer_fx::sized_tile
            // works its numbers, then the original mixed back.
            E::Tiles {
                scale,
                center,
                blend,
            } => {
                let s = scale / 100.0;
                let points = (1.0 / s).ceil().clamp(1.0, 16.0);
                let (nx, ny) = (w as f64, h as f64);
                let (tx, ty) = (nx * s, ny * s);
                let k = [
                    center[0] / 100.0 * nx - tx / 2.0,
                    tx,
                    s,
                    center[1] / 100.0 * ny - ty / 2.0,
                    ty,
                    points,
                    points,
                    blend / 100.0,
                    0.0,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 25,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-405: grown as layer_fx::magnify_lens says, as compose's growth for it.
            E::Magnify {
                shape,
                center,
                magnification,
                link,
                size: radius,
                feather,
                opacity,
                scaling,
                blending_mode,
                resize_layer,
            } => {
                let ((cx, cy), r, fe, g) = crate::layer_fx::magnify_lens(
                    *center,
                    *magnification,
                    link,
                    [*radius, *feather],
                    resize_layer == "on",
                    (w, h),
                    f.origin,
                );
                let m = magnification / 100.0;
                let scaling = match scaling.as_str() {
                    "soft" => 1.0,
                    "scatter" => 2.0,
                    _ => 0.0,
                };
                let (ox, oy) = (f.origin.0 as f64, f.origin.1 as f64);
                let k = [
                    cx,
                    cy,
                    m,
                    r,
                    fe,
                    opacity / 100.0,
                    1.0 - 1.0 / m,
                    (shape == "square") as u8 as f64,
                    scaling,
                    laid(blending_mode),
                    ox,
                    oy,
                    0.0,
                ];
                let base = crate::grade::mix(0);
                let (tw, th) = (w + 2 * g, h + 2 * g);
                let out = self.scratch("D-405 magnify", tw, th);
                let p = FxParams {
                    mode: 23,
                    g: g as i32,
                    base: [base as u32, (base >> 32) as u32],
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.warp,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            // D-413: as effects' arm reads it, the layer never grows.
            E::Checkerboard {
                anchor,
                size_from,
                corner,
                width,
                height,
                feather_width,
                feather_height,
                color,
                opacity,
                blending_mode,
            } => {
                let ((ax, ay), (cw, ch)) = crate::layer_fx::checker_cells(
                    *anchor,
                    size_from,
                    *corner,
                    [*width, *height],
                    (w, h),
                    f.origin,
                );
                let c = linear(color);
                let k = [
                    ax,
                    ay,
                    cw,
                    ch,
                    feather_width.max(1.0) / 2.0,
                    feather_height.max(1.0) / 2.0,
                    c[0],
                    c[1],
                    c[2],
                    opacity / 100.0,
                    laid(blending_mode),
                    0.0,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 26,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-414: as effects' arm reads it, the layer never grows.
            E::Circle {
                center,
                radius,
                edge,
                edge_thickness,
                feather_outer,
                feather_inner,
                invert,
                color,
                opacity,
                blending_mode,
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let [ro, ri, fo, fi] = crate::layer_fx::circle_ring(
                    *radius,
                    edge,
                    *edge_thickness,
                    [*feather_outer, *feather_inner],
                );
                let c = linear(color);
                let k = [
                    cx,
                    cy,
                    ro,
                    ri,
                    fo,
                    fi,
                    c[0],
                    c[1],
                    c[2],
                    opacity / 100.0,
                    laid(blending_mode),
                    (invert == "on") as u8 as f64,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 27,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-417: as effects' arm reads it, the layer never grows.
            E::Grid {
                anchor,
                size_from,
                corner,
                width,
                height,
                border,
                feather_width,
                feather_height,
                invert,
                color,
                opacity,
                blending_mode,
            } => {
                let ((ax, ay), (cw, ch)) = crate::layer_fx::checker_cells(
                    *anchor,
                    size_from,
                    *corner,
                    [*width, *height],
                    (w, h),
                    f.origin,
                );
                let c = linear(color);
                let k = [
                    ax,
                    ay,
                    cw,
                    ch,
                    *border,
                    feather_width.max(1.0),
                    feather_height.max(1.0),
                    c[0],
                    c[1],
                    c[2],
                    opacity / 100.0,
                    laid(blending_mode),
                    (invert == "on") as u8 as f64,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 31,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-415: as effects' arm reads it, the layer never grows.
            // D-420: the marks worked out here as the CPU works them, from the levels compose
            // found; with none the card is not asked (`compose::card_effect`).
            // D-421: Audio Waveform's marks on the same pass.
            e @ (E::AudioSpectrum { .. } | E::AudioWaveform { .. }) => {
                let m = crate::layer_fx::spectrum_marks(e, (w, h), f.origin)
                    .or_else(|| crate::layer_fx::waveform_marks(e, (w, h), f.origin))
                    .unwrap_or_default();
                let boxes = m.boxes();
                let bands = crate::layer_fx::Marks::bands(&boxes, h);
                let mut k = vec![
                    m.r,
                    m.softness,
                    m.how as f64,
                    m.blend as u8 as f64,
                    m.pieces.len() as f64,
                ];
                for (p, b) in m.pieces.iter().zip(&boxes) {
                    k.extend(p);
                    k.extend(b);
                }
                let mut first = k.len() + 2 * bands.len();
                for band in &bands {
                    k.extend([first as f64, band.len() as f64]);
                    first += band.len();
                }
                k.extend(bands.concat().into_iter().map(f64::from));
                same(steps, &passes.marks, FxParams::default(), &k, None)
            }
            E::Ellipse {
                center,
                width,
                height,
                thickness,
                softness,
                inside_color,
                outside_color,
                composite,
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let mut k = vec![
                    cx,
                    cy,
                    width / 2.0,
                    height / 2.0,
                    thickness / 2.0,
                    *softness,
                    (composite == "off") as u8 as f64,
                ];
                k.extend(linear(inside_color));
                k.extend(linear(outside_color));
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 28,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-416: as effects' arm reads it, the layer replaced and never grown.
            effect @ E::Fractal { .. } => {
                let (w0, h0) = ((w - 2 * f.origin.0) as f64, (h - 2 * f.origin.1) as f64);
                let v = crate::layer_fx::FractalView::of(effect, h0).expect("a Fractal");
                let on = |b: bool| b as u8 as f64;
                let k = [
                    v.m[0],
                    v.m[1],
                    v.m[2],
                    v.j[0],
                    v.j[1],
                    v.j[2],
                    f.origin.0 as f64,
                    f.origin.1 as f64,
                    w0 / 2.0,
                    h0 / 2.0,
                    v.m_limit as f64,
                    v.j_limit as f64,
                    v.power as f64,
                    on(v.julia),
                    on(v.inverse),
                    on(v.over_julia),
                    v.palette as f64,
                    v.hue,
                    v.steps as f64,
                    v.offset as f64,
                    on(v.transparency),
                    on(v.edge_highlight),
                    on(v.brute),
                    v.factor as f64,
                    on(v.overlay),
                    0.0,
                    (w0 as i64 / 2) as f64,
                    (h0 as i64 / 2) as f64,
                    2.max(h0 as i64 / 20) as f64,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 30,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Mirror { center, angle } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let (nx, ny) = crate::layer_fx::mirror_normal(*angle);
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 4,
                        ..Default::default()
                    },
                    &[cx, cy, nx, ny, 0.0],
                    None,
                )
            }
            E::CameraShake {
                amount,
                rotation,
                hold,
                seed,
                frame,
            } => {
                let ((cx, cy), g) =
                    crate::layer_fx::shake_reach(*amount, *rotation, (w, h), f.origin);
                let (dx, dy, sb, cb) =
                    crate::layer_fx::shake_jolt([*amount, *rotation, *hold, *seed], *frame);
                let out = self.scratch("B-107 shake", w + 2 * g, h + 2 * g);
                let p = FxParams {
                    mode: 5,
                    g: g as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.warp,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&[cx, cy, dx, dy, sb, cb]),
                    None,
                    none,
                    tiles(w + 2 * g, h + 2 * g),
                );
                (out, (w + 2 * g, h + 2 * g))
            }
            E::MotionTile { mirror, .. } => {
                let (gx, gy) = f.grow;
                let (tw, th) = (w + 2 * gx, h + 2 * gy);
                let out = self.scratch("B-115 tile", tw, th);
                let p = FxParams {
                    mode: 6,
                    flag: (mirror == "on") as u32,
                    ox: gx as i32,
                    oy: gy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.warp,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&[0.0]),
                    None,
                    none,
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            // D-464: one warp pass, mode 101, into the grown buffer.
            E::RepeatTile {
                expand_right,
                expand_left,
                expand_down,
                expand_up,
                tiling,
                blend_borders,
            } => {
                let (gx, gy) = f.grow;
                let (tw, th) = (w + 2 * gx, h + 2 * gy);
                let out = self.scratch("D-464 repeat tile", tw, th);
                let k = crate::layer_fx::repeat_tile_setup(
                    [*expand_right, *expand_left, *expand_down, *expand_up],
                    tiling,
                    *blend_borders,
                );
                let p = FxParams {
                    mode: 101,
                    ox: gx as i32,
                    oy: gy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.warp,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            // D-465: one warp pass, mode 102.
            E::Scatter {
                scatter_amount,
                grain,
                frame,
                ..
            } => {
                let k = crate::layer_fx::scatter_setup(*scatter_amount, grain);
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 102,
                        frame: *frame,
                        ..Default::default()
                    },
                    &[k[0] as f64, k[1] as f64],
                    None,
                )
            }
            // D-466: one warp pass, mode 103; the card's plan leaves out a Strobe Light that is off.
            E::StrobeLight {
                strobe_color,
                blend_with_original,
                strobe,
                strobe_operator,
                ..
            } => {
                let k = crate::layer_fx::strobe_setup(
                    linear(strobe_color),
                    *blend_with_original,
                    strobe,
                    strobe_operator,
                );
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 103,
                        ..Default::default()
                    },
                    &[k[0], k[1], k[2], k[3], k[4], k[5], 0.0],
                    None,
                )
            }
            // D-562: tone mode 110; the card's plan leaves out one that changes nothing.
            E::CcVignette {
                amount,
                angle_of_view,
                center,
                pin_highlights,
            } => {
                let k = crate::effects::cc_vignette_settings(
                    *amount,
                    *angle_of_view,
                    *center,
                    *pin_highlights,
                    (w, h),
                    f.origin,
                );
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 110,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-468: tone mode 100, one level and one Invert for all three.
            E::ChannelThreshold {
                threshold,
                channel,
                invert,
                blend_with_original,
            } => {
                let ch = crate::effects::CHANNEL_THRESHOLD_CHANNELS
                    .iter()
                    .position(|c| c == channel)
                    .expect("a valid channel") as f64;
                let i = (invert == "on") as u8 as f64;
                let k = [
                    *threshold,
                    *threshold,
                    *threshold,
                    ch,
                    i,
                    i,
                    i,
                    1.0 - blend_with_original / 100.0,
                ];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 100,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-469: Channel Threshold's tone mode 100 on RGB, a level and an Invert each.
            E::ThresholdRgb {
                red_threshold,
                green_threshold,
                blue_threshold,
                invert_red,
                invert_green,
                invert_blue,
                blend_with_original,
            } => {
                let [i0, i1, i2] =
                    [invert_red, invert_green, invert_blue].map(|v| (v == "on") as u8 as f64);
                let k = [
                    *red_threshold,
                    *green_threshold,
                    *blue_threshold,
                    1.0,
                    i0,
                    i1,
                    i2,
                    1.0 - blend_with_original / 100.0,
                ];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 100,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-470: the flow as CC Vector Blur's luminance, the tensor, D-400's held blur and the
            // tangent; then each smear pass, and the fills from compose's palette.
            E::ContourPaint {
                view,
                stylization,
                cleanliness,
                smear_iterations,
                apply_to_alpha,
                draw_color_fills,
                reduction_method,
                low_strength,
                high_strength,
                offset_flow_angles,
                quantize_flow_angles,
                arrowheads,
                px,
                palette,
                map,
                ..
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                let (sigma, turn, ways) = crate::layer_fx::contour_setup(
                    *cleanliness,
                    *offset_flow_angles,
                    *quantize_flow_angles,
                    *px,
                );
                let taps = crate::effects::gaussian_weights(sigma);
                let place = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| *p == "luminance")
                    .expect("luminance is a property");
                let mut tk = vec![turn.0, turn.1];
                tk.extend(ways.iter().flat_map(|&(x, y)| [x, y]));
                let picture = map.as_ref().map(|m| self.map_texture(&m.0));
                let flow = |steps: &mut Vec<Step>, from: &wgpu::TextureView| {
                    let (from, (mx, my)) = match &picture {
                        Some(p) => (p, (ox, oy)),
                        None => (from, (0, 0)),
                    };
                    let lum = self.scratch("D-470 luminance", w, h);
                    let p = FxParams {
                        ox: mx as i32,
                        oy: my as i32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.vheight,
                        p,
                        Some(from),
                        Some(&lum),
                        Some(&[place as f64]),
                        None,
                        none,
                        tiles(w, h),
                    );
                    let tensor = self.scratch("D-470 tensor", w, h);
                    self.fx_step(
                        steps,
                        &passes.cptensor,
                        FxParams::default(),
                        Some(&lum),
                        Some(&tensor),
                        None,
                        None,
                        none,
                        tiles(w, h),
                    );
                    let blurred = self.gauss(
                        steps,
                        bloom,
                        "D-470 blur",
                        &tensor,
                        (w, h),
                        [&taps, &taps],
                        0,
                        true,
                    );
                    let out = self.scratch("D-470 flow", w, h);
                    let p = FxParams {
                        n: ways.len() as u32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.cptangent,
                        p,
                        Some(&blurred),
                        Some(&out),
                        Some(&tk),
                        None,
                        none,
                        tiles(w, h),
                    );
                    out
                };
                if view == "edge_map" {
                    let field = flow(steps, still);
                    let out = self.scratch("D-470 edges", w, h);
                    self.fx_step(
                        steps,
                        &passes.cpedge,
                        FxParams::default(),
                        Some(&field),
                        Some(&out),
                        Some(&[12.0 * px, *px, (arrowheads == "on") as u8 as f64]),
                        None,
                        none,
                        tiles(w, h),
                    );
                    (out, (w, h))
                } else {
                    let fixed = picture.as_ref().map(|_| flow(steps, still));
                    let weights = crate::layer_fx::contour_weights(*stylization, *px);
                    let keep = FxParams {
                        n: weights.len() as u32,
                        flag: (apply_to_alpha == "off") as u32,
                        ..Default::default()
                    };
                    let mut cur = still.clone();
                    for _ in 0..smear_iterations.floor() as usize {
                        let field = fixed.clone().unwrap_or_else(|| flow(steps, &cur));
                        let out = self.scratch("D-470 smear", w, h);
                        self.fx_step(
                            steps,
                            &passes.cpsmear,
                            keep,
                            Some(&cur),
                            Some(&out),
                            Some(&weights),
                            Some(&field),
                            none,
                            tiles(w, h),
                        );
                        cur = out;
                    }
                    let palette = palette
                        .as_ref()
                        .expect("compose fills Contour Paint's palette");
                    if draw_color_fills != "none" && !palette.is_empty() {
                        let field = fixed.unwrap_or_else(|| flow(steps, &cur));
                        let perceptual = reduction_method == crate::effects::CONTOUR_CUSTOM;
                        let mut k = vec![*low_strength, *high_strength, palette.len() as f64];
                        k.extend(palette.iter().flat_map(|&q| {
                            if perceptual {
                                crate::layer_fx::contour_lab(q)
                            } else {
                                q
                            }
                        }));
                        k.extend(palette.iter().flat_map(|q| q.map(crate::grade::to_linear)));
                        let p = FxParams {
                            flag: (draw_color_fills == "only") as u32 | (perceptual as u32) << 1,
                            ..Default::default()
                        };
                        let out = self.scratch("D-470 fills", w, h);
                        self.fx_step(
                            steps,
                            &passes.cpfill,
                            p,
                            Some(&cur),
                            Some(&out),
                            Some(&k),
                            Some(&field),
                            none,
                            tiles(w, h),
                        );
                        cur = out;
                    }
                    (cur, (w, h))
                }
            }
            // D-467: the texture's luminance as Plastic's bump, then one pass that presses it in.
            E::Texturize {
                light_direction,
                texture_contrast,
                map,
                ..
            } => {
                let picture = self.map_texture(
                    &map.as_ref()
                        .expect("compose leaves a Texturize with a texture")
                        .0,
                );
                let place = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| *p == "luminance")
                    .expect("luminance is a property");
                let raw = self.scratch("D-467 height", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.vheight,
                    p,
                    Some(&picture),
                    Some(&raw),
                    Some(&[place as f64]),
                    None,
                    none,
                    tiles(w, h),
                );
                let (ux, uy) = crate::blurs::along(*light_direction);
                let out = self.scratch("D-467 texturize", w, h);
                self.fx_step(
                    steps,
                    &passes.texturize,
                    FxParams::default(),
                    Some(still),
                    Some(&out),
                    Some(&[ux, uy, texture_contrast / 2.0]),
                    Some(&raw),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            E::LinearWipe {
                completion,
                angle,
                feather,
            } => {
                let ((ux, uy), edge) =
                    crate::layer_fx::linear_edge(*completion, *angle, *feather, (w, h), f.origin);
                let p = FxParams {
                    mode: 0,
                    flag: (*completion == 100.0) as u32,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.wipe, p, &[ux, uy, edge, *feather, 0.0], None)
            }
            E::RadialWipe {
                completion,
                start_angle,
                center,
                wipe,
                feather,
            } => {
                let way = match wipe.as_str() {
                    "counterclockwise" => 1,
                    "both" => 2,
                    _ => 0,
                };
                let (w0, h0) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let edge = completion / 100.0 * (360.0 + feather) - feather / 2.0;
                let k = [
                    center[0] / 100.0 * w0,
                    center[1] / 100.0 * h0,
                    *start_angle,
                    edge,
                    *feather,
                ];
                let p = FxParams {
                    mode: 1,
                    count: way,
                    flag: (*completion == 100.0) as u32,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.wipe, p, &k, None)
            }
            E::VenetianBlinds {
                completion,
                angle,
                width,
                feather,
            } => {
                let (ux, uy) = crate::blurs::along(*angle);
                let edge = completion / 100.0 * (width + feather) - feather / 2.0;
                let p = FxParams {
                    mode: 2,
                    flag: (*completion == 100.0) as u32,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(
                    steps,
                    &passes.wipe,
                    p,
                    &[ux, uy, *width, edge, *feather, 0.0],
                    None,
                )
            }
            E::IrisWipe {
                completion,
                center,
                feather,
                invert,
            } => {
                let invert = invert == "on";
                let ((cx, cy), r) = crate::layer_fx::iris_circle(
                    *completion,
                    *center,
                    *feather,
                    invert,
                    (w, h),
                    f.origin,
                );
                let p = FxParams {
                    mode: 3,
                    flag: (*completion == 100.0) as u32,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(
                    steps,
                    &passes.wipe,
                    p,
                    &[cx, cy, r, *feather, invert as u8 as f64],
                    None,
                )
            }
            e @ E::StretchWipe { .. } => {
                let k =
                    crate::layer_fx::stretch_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let p = FxParams {
                    mode: 9,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.wipe, p, &k, None)
            }
            e @ E::PushIris { .. } => {
                let k = crate::layer_fx::push_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let p = FxParams {
                    mode: 8,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.wipe, p, &k, None)
            }
            e @ E::StripeSweep { .. } => {
                let k =
                    crate::layer_fx::stripe_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let p = FxParams {
                    mode: 7,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.wipe, p, &k, None)
            }
            e @ E::GlowWipe { .. } => {
                let k = crate::layer_fx::glow_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let p = FxParams {
                    mode: 6,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.wipe, p, &k, None)
            }
            e @ E::TeethWipe { .. } => {
                let k = crate::layer_fx::teeth_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let p = FxParams {
                    mode: 5,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.wipe, p, &k, None)
            }
            e @ E::TileWipe { .. } => {
                let k = crate::layer_fx::tile_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let p = FxParams {
                    mode: 4,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.wipe, p, &k, None)
            }
            E::SimpleChoker { choke } => {
                let spread = *choke < 0.0;
                let g = if spread { (-choke).floor() as usize } else { 0 };
                let (bw, bh) = (w + 2 * g, h + 2 * g);
                let runs = crate::layer_fx::disc_runs(choke.abs());
                let (row, band) = (buffer(h * bw * 4), buffer(bw * bh * 4));
                let top = runs.iter().map(|&(_, hw)| hw).max().unwrap_or(0);
                let mut first = true;
                for k in 0..=top {
                    // Each row's least or greatest covering within k, widened from within k - 1.
                    let p = FxParams {
                        n: g as u32,
                        count: k as u32,
                        flag: !spread as u32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.rows,
                        p,
                        Some(still),
                        None,
                        None,
                        None,
                        [Some(&row), None, None, None],
                        tiles(bw, h),
                    );
                    let dys: Vec<f64> = runs
                        .iter()
                        .filter(|&&(_, hw)| hw == k)
                        .map(|&(dy, _)| dy as f64)
                        .collect();
                    if !dys.is_empty() {
                        // A shrink's band starts from infinity, a spread's from the empty buffer.
                        let mode = (!spread && first) as u32;
                        first = false;
                        let p = FxParams {
                            mode,
                            n: g as u32,
                            count: dys.len() as u32,
                            flag: !spread as u32,
                            ..Default::default()
                        };
                        self.fx_step(
                            steps,
                            &passes.bands,
                            p,
                            Some(still),
                            None,
                            Some(&dys),
                            None,
                            [Some(&row), Some(&band), None, None],
                            tiles(bw, bh),
                        );
                    }
                }
                let sums = buffer(if spread { (w + 1) * h * 32 } else { 32 });
                if spread {
                    self.fx_step(
                        steps,
                        &passes.prefix,
                        FxParams::default(),
                        Some(still),
                        None,
                        Some(&[0.0; 3]),
                        None,
                        [None, None, Some(&sums), None],
                        ((h as u32).div_ceil(64), 1),
                    );
                }
                let k: Vec<f64> = runs
                    .iter()
                    .flat_map(|&(dy, hw)| [dy as f64, hw as f64])
                    .collect();
                let out = self.scratch("B-107 choker", bw, bh);
                let p = FxParams {
                    flag: !spread as u32,
                    g: g as i32,
                    count: runs.len() as u32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.choke,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None, Some(&band), Some(&sums), None],
                    tiles(bw, bh),
                );
                (out, (bw, bh))
            }
            // D-352: each stage the running totals of what the stage before made, then the
            // disc's average through the ramp, two textures taken turn about.
            E::MatteChoker {
                geometric_softness_1,
                choke_1,
                gray_level_softness_1,
                geometric_softness_2,
                choke_2,
                gray_level_softness_2,
                iterations,
            } => {
                let sums = buffer((w + 1) * h * 32);
                let pair = [
                    self.scratch("D-352 matte choker", w, h),
                    self.scratch("D-352 matte choker", w, h),
                ];
                let stages = [
                    [*geometric_softness_1, *choke_1, *gray_level_softness_1],
                    [*geometric_softness_2, *choke_2, *gray_level_softness_2],
                ];
                let mut done = 0;
                for _ in 0..iterations.floor() as usize {
                    for [g, c, s] in stages {
                        let from = if done == 0 {
                            still
                        } else {
                            &pair[(done + 1) % 2]
                        };
                        self.fx_step(
                            steps,
                            &passes.prefix,
                            FxParams::default(),
                            Some(from),
                            None,
                            Some(&[0.0; 3]),
                            None,
                            [None, None, Some(&sums), None],
                            ((h as u32).div_ceil(64), 1),
                        );
                        let runs = crate::layer_fx::disc_runs(g);
                        let size: isize = runs.iter().map(|&(_, hw)| 2 * hw + 1).sum();
                        let mut k = vec![size as f64, 0.5 + c / 255.0, s / 100.0];
                        k.extend(runs.iter().flat_map(|&(dy, hw)| [dy as f64, hw as f64]));
                        let p = FxParams {
                            count: runs.len() as u32,
                            ..Default::default()
                        };
                        self.fx_step(
                            steps,
                            &passes.mchoke,
                            p,
                            Some(from),
                            Some(&pair[done % 2]),
                            Some(&k),
                            None,
                            [None, None, Some(&sums), None],
                            tiles(w, h),
                        );
                        done += 1;
                    }
                }
                let [a, b] = pair;
                (if done % 2 == 1 { a } else { b }, (w, h))
            }
            // D-353: the light, then each level's cells spread two ways and added between its
            // cells, then laid with the layer, grown by the plan's reach. The three planes are
            // the largest level's size, each level using its own corner of them.
            // D-544: a Halation is one, with one pass more.
            e @ (E::SoftGlow { .. } | E::Halation { .. }) => {
                let s = crate::effects::soft_glow_settings(e).expect("a soft glow's settings");
                // D-543: one chain of levels, or one a channel at its own radius, each level adding
                // only its chain's channel; the light's own share is each chain's for its channel.
                let chains: Vec<_> = crate::soft_glow::chains(s.radius, s.spectral)
                    .into_iter()
                    .map(|(r, c)| (r, crate::soft_glow::plan(r, s.aspect, s.angle), c))
                    .collect();
                let mut own = [0.0f64; 3];
                for (_, plan, c) in &chains {
                    match c {
                        None => own = [plan.own as f64; 3],
                        Some(c) => own[*c] = plan.own as f64,
                    }
                }
                // D-542: the tint's numbers for each level k at a radius, all 0 when off.
                let tint = &s.tint;
                let tinted = |radius: f64, k: usize| -> [f64; 8] {
                    let Some(t) = &tint else { return [0.0; 8] };
                    let c = t.colour(radius, k);
                    [
                        1.0,
                        t.amount as f64,
                        c[0] as f64,
                        c[1] as f64,
                        c[2] as f64,
                        t.inner[0] as f64,
                        t.inner[1] as f64,
                        t.inner[2] as f64,
                    ]
                };
                let g = chains
                    .iter()
                    .map(|(_, plan, _)| plan.grow)
                    .max()
                    .unwrap_or(0);
                let (ow, oh) = (w + 2 * g, h + 2 * g);
                let single = |v: f64| v as f32 as f64;
                let lit = self.scratch("D-353 light", w, h);
                let k = [
                    single(s.threshold / 100.0),
                    single(s.bias / 100.0),
                    single(s.smooth / 100.0),
                    f64::from(s.luminance),
                ];
                self.fx_step(
                    steps,
                    &passes.sglight,
                    FxParams::default(),
                    Some(still),
                    Some(&lit),
                    Some(&k),
                    None,
                    none,
                    tiles(w, h),
                );
                let size = |lv: &crate::soft_glow::Level| {
                    (
                        w.div_ceil(lv.d) + 2 * lv.reach.0,
                        h.div_ceil(lv.d) + 2 * lv.reach.1,
                    )
                };
                let (mw, mh) = chains
                    .iter()
                    .flat_map(|(_, plan, _)| &plan.levels)
                    .map(size)
                    .fold((1, 1), |(a, b), (x, y)| (a.max(x), b.max(y)));
                let planes = [
                    self.scratch("D-353 plane", mw, mh),
                    self.scratch("D-353 plane", mw, mh),
                    self.scratch("D-353 plane", mw, mh),
                ];
                let sums = [
                    self.scratch("D-353 glow", ow, oh),
                    self.scratch("D-353 glow", ow, oh),
                ];
                let along =
                    |taps: &[f32], down: bool, mu: f64, (pw, ph): (usize, usize)| {
                        let mut k = vec![pw as f64, ph as f64, f64::from(down)];
                        k.extend(crate::soft_glow::steps(taps, mu).into_iter().flat_map(
                            |(t, lo, w0, w1)| [t as f64, lo as f64, w0 as f64, w1 as f64],
                        ));
                        k
                    };
                let mut done = 0;
                let levels = chains.iter().flat_map(|(r, plan, c)| {
                    plan.levels
                        .iter()
                        .enumerate()
                        .map(move |(n, lv)| (*r, n, lv, *c))
                });
                for (r, n, lv, channel) in levels {
                    let (pw, ph) = size(lv);
                    let mask: [f64; 3] = match channel {
                        None => [1.0; 3],
                        Some(c) => std::array::from_fn(|i| f64::from(i == c)),
                    };
                    let (rx, ry) = (lv.reach.0 as f64, lv.reach.1 as f64);
                    let k = [lv.d as f64, rx, ry, pw as f64, ph as f64];
                    self.fx_step(
                        steps,
                        &passes.sgcells,
                        FxParams::default(),
                        Some(&lit),
                        Some(&planes[0]),
                        Some(&k),
                        None,
                        none,
                        tiles(pw, ph),
                    );
                    let p = FxParams {
                        count: lv.first.len() as u32,
                        ..Default::default()
                    };
                    let k = along(&lv.first, lv.first_y, 0.0, (pw, ph));
                    self.fx_step(
                        steps,
                        &passes.sgpass,
                        p,
                        Some(&planes[0]),
                        Some(&planes[1]),
                        Some(&k),
                        None,
                        none,
                        tiles(pw, ph),
                    );
                    let p = FxParams {
                        count: lv.second.len() as u32,
                        ..Default::default()
                    };
                    let k = along(&lv.second, !lv.first_y, lv.mu, (pw, ph));
                    self.fx_step(
                        steps,
                        &passes.sgpass,
                        p,
                        Some(&planes[1]),
                        Some(&planes[2]),
                        Some(&k),
                        None,
                        none,
                        tiles(pw, ph),
                    );
                    let mut k = vec![
                        lv.weight as f64,
                        lv.d as f64,
                        rx,
                        ry,
                        g as f64,
                        own[0],
                        pw as f64,
                        ph as f64,
                    ];
                    k.extend(tinted(r, n));
                    k.extend(mask);
                    k.extend(own);
                    let p = FxParams {
                        flag: (done == 0) as u32,
                        count: 1,
                        ..Default::default()
                    };
                    let before = if done == 0 {
                        &lit
                    } else {
                        &sums[(done + 1) % 2]
                    };
                    self.fx_step(
                        steps,
                        &passes.sgadd,
                        p,
                        Some(&planes[2]),
                        Some(&sums[done % 2]),
                        Some(&k),
                        Some(before),
                        none,
                        tiles(ow, oh),
                    );
                    done += 1;
                }
                if done == 0 {
                    // No level: the light's own share alone.
                    let mut k = vec![0.0, 1.0, 0.0, 0.0, g as f64, own[0], 0.0, 0.0];
                    k.extend(tinted(s.radius, 0));
                    k.extend([1.0; 3]);
                    k.extend(own);
                    let p = FxParams {
                        flag: 1,
                        count: 0,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.sgadd,
                        p,
                        Some(&planes[2]),
                        Some(&sums[0]),
                        Some(&k),
                        Some(&lit),
                        none,
                        tiles(ow, oh),
                    );
                    done = 1;
                }
                if let Some(c) = s.halation {
                    let k = [g as f64, c[0] as f64, c[1] as f64, c[2] as f64];
                    self.fx_step(
                        steps,
                        &passes.sghalo,
                        FxParams::default(),
                        Some(&sums[(done + 1) % 2]),
                        Some(&sums[done % 2]),
                        Some(&k),
                        Some(&lit),
                        none,
                        tiles(ow, oh),
                    );
                    done += 1;
                }
                // D-545: the glow times Exposure with the dirt on top, then laid at Exposure 1.
                let mut exposure = single(s.exposure);
                if let Some(d) = &s.dirt {
                    let mut k = vec![exposure];
                    k.extend(dirt_k(d, (w, h), g));
                    let picture = self.map_texture(&d.picture);
                    self.fx_step(
                        steps,
                        &passes.sgdirt,
                        FxParams::default(),
                        Some(&sums[(done + 1) % 2]),
                        Some(&sums[done % 2]),
                        Some(&k),
                        Some(&picture),
                        none,
                        tiles(ow, oh),
                    );
                    done += 1;
                    exposure = 1.0;
                }
                let out = self.scratch("D-353 soft glow", ow, oh);
                let k = [
                    g as f64,
                    exposure,
                    single(s.opacity / 100.0),
                    f64::from(s.screen),
                    f64::from(s.unmult),
                ];
                self.fx_step(
                    steps,
                    &passes.sgfinish,
                    FxParams::default(),
                    Some(&sums[(done + 1) % 2]),
                    Some(&out),
                    Some(&k),
                    Some(still),
                    none,
                    tiles(ow, oh),
                );
                (out, (ow, oh))
            }
            E::SpeedLines {
                center,
                color,
                count,
                thickness,
                inner,
                inner_jitter,
                angle_jitter,
                seed,
                hold,
                opacity,
                frame,
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let lines = crate::layer_fx::speed_line_list(
                    [
                        *count,
                        *thickness,
                        *inner,
                        *inner_jitter,
                        *angle_jitter,
                        *seed,
                        *hold,
                    ],
                    *frame,
                );
                let widest = lines.iter().map(|l| l.1).fold(0.0, f64::max);
                let mut k = vec![cx, cy, opacity / 100.0];
                k.extend(linear(color));
                k.extend([widest, lines.len() as f64]);
                k.extend(lines.iter().flat_map(|&(t, h, r)| [t, h, r]));
                same(steps, &passes.lines, FxParams::default(), &k, None)
            }
            E::CrossGlare {
                threshold,
                length,
                points,
                angle,
                intensity,
                color,
                glint,
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a Cross Glare is refused without the passes");
                let empty = self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("B-107 none"),
                        contents: bytemuck::cast_slice(&[0.0f32; 3]),
                        usage: wgpu::BufferUsages::STORAGE,
                    });
                let lit = self.scratch("B-107 bright", w, h);
                let p = Params {
                    level: crate::bloom::bright_level(*threshold),
                    ..Default::default()
                };
                self.step(
                    steps,
                    &bloom.bright,
                    p,
                    still,
                    Some(&lit),
                    None,
                    Some(&empty),
                    tiles(w, h),
                );
                let l = length.floor() as usize;
                let n = points.floor();
                // D-546: each arm's tint after its direction.
                let (outer, tints) = glint.tints(n as usize, color);
                let mut k = vec![*intensity];
                k.extend(outer);
                k.push(n);
                k.extend((0..n as usize).flat_map(|j| {
                    let (vx, vy) = crate::blurs::along(angle + 360.0 * j as f64 / n);
                    let [r, g, b, a] = tints[j];
                    [vx, vy, r, g, b, a]
                }));
                let fade: Vec<f64> = (1..=l)
                    .map(|t| (1.0 - t as f64 / (l + 1) as f64).powi(2))
                    .collect();
                let total: f64 = fade.iter().sum();
                k.extend(
                    fade.iter()
                        .enumerate()
                        .flat_map(|(t, f)| [(t + 1) as f64, f / total]),
                );
                let out = self.scratch("B-107 glare", w + 2 * l, h + 2 * l);
                let p = FxParams {
                    g: l as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.glare,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&lit),
                    none,
                    tiles(w + 2 * l, h + 2 * l),
                );
                (out, (w + 2 * l, h + 2 * l))
            }
            E::Rain {
                color,
                density,
                spacing,
                length,
                width,
                direction,
                speed,
                seed,
                opacity,
                frame,
            } => {
                let (tx, ty) = crate::blurs::along(*direction);
                let base = crate::grade::mix(seed.floor() as u64);
                let mut k = vec![
                    tx,
                    ty,
                    -ty,
                    tx,
                    width / 2.0 + 0.5,
                    length / 2.0,
                    speed * *frame as f64,
                    *spacing,
                    density / 100.0,
                    width / 2.0,
                    *opacity,
                ];
                k.extend(linear(color));
                let p = FxParams {
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.rain, p, &k, None)
            }
            E::ColorLookup { table, .. } => {
                let table = table
                    .as_ref()
                    .expect("compose leaves a Color Lookup only with its table");
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 11,
                        ..Default::default()
                    },
                    &table.0.packed(),
                    None,
                )
            }
            // D-395: the colour tables as one 1D lookup, Color Lookup's pass.
            E::ArbitraryMap { phase, table, .. } => {
                let table = table
                    .as_ref()
                    .expect("compose leaves an Arbitrary Map only with its file");
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 11,
                        ..Default::default()
                    },
                    &table.0.colours(*phase).packed(),
                    None,
                )
            }
            E::HsvKey {
                hue,
                saturation,
                value,
                hue_range,
                saturation_range,
                value_range,
                invert,
            } => {
                // The single-precision power 1/2.4 the CPU's encoding uses, less the true one.
                let power = f64::from(1.0f32 / 2.4) - 1.0 / 2.4;
                let k = [
                    *hue,
                    *hue_range,
                    *saturation,
                    *saturation_range,
                    *value,
                    *value_range,
                    (invert == "on") as u8 as f64,
                    power,
                    0.0,
                ];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 12,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // B-222: ten colour effects of their own pixel. Bypass is the 8 bpc and 32 bpc (After
            // Effects) depths' only, which the card is never given (D-330).
            E::Exposure {
                stops,
                offset,
                gamma,
                ..
            } if *offset == 0.0 && *gamma == 1.0 => same(
                steps,
                &passes.grade,
                FxParams {
                    mode: 5,
                    ..Default::default()
                },
                &[2f64.powf(*stops) as f32 as f64],
                None,
            ),
            E::Exposure {
                stops,
                offset,
                gamma,
                ..
            } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 14,
                    ..Default::default()
                },
                &[2f64.powf(*stops), *offset, 1.0 / gamma],
                None,
            ),
            E::Tint { color, amount } => {
                let k = [
                    *amount as f32 as f64,
                    color[0] as f32 as f64,
                    color[1] as f32 as f64,
                    color[2] as f32 as f64,
                ];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 15,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::ShiftChannels {
                take_alpha,
                take_red,
                take_green,
                take_blue,
            } => {
                let place = |w: &String| {
                    crate::effects::SHIFT_CHANNELS_FROM
                        .iter()
                        .position(|f| f == w)
                        .unwrap_or(10) as f64
                };
                let k = [take_red, take_green, take_blue, take_alpha].map(place);
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 16,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::SolidComposite {
                source_opacity,
                color,
                opacity,
                blend: b,
            } => {
                let o = (opacity / 100.0) as f32;
                let lin = crate::effects::encoded(color)
                    .map(|c| (crate::color::srgb_to_linear(c as f32) * o) as f64);
                let k = [
                    (source_opacity / 100.0) as f32 as f64,
                    o as f64,
                    lin[0],
                    lin[1],
                    lin[2],
                ];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 17,
                        blend: blend(b),
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::ChangeToColor {
                from,
                to,
                change,
                change_by,
                hue_tolerance,
                lightness_tolerance,
                saturation_tolerance,
                softness,
                view_matte,
            } => {
                let mut k = crate::effects::encoded(from).to_vec();
                k.extend(crate::effects::encoded(to));
                k.extend([
                    hue_tolerance / 100.0,
                    lightness_tolerance / 100.0,
                    saturation_tolerance / 100.0,
                    softness / 100.0,
                ]);
                let on = |b: bool| b as u8 as f64;
                k.extend([
                    on(change_by == "transforming"),
                    on(change.contains("lightness")),
                    on(change.contains("saturation")),
                    on(view_matte == "on"),
                ]);
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 18,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::ColorKey {
                colors, tolerance, ..
            }
            | E::SelectColor {
                colors, tolerance, ..
            }
            | E::LineRecolor {
                colors, tolerance, ..
            } => {
                // k: the targets' count, the tolerance, two settings and the new colour as each
                // reads them, level8's power and 0, then the targets.
                let targets = crate::selective_blur::targets(colors);
                let power = f64::from(1.0f32 / 2.4) - 1.0 / 2.4;
                let mut k = vec![
                    targets.len() as f64,
                    *tolerance,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    power,
                    0.0,
                ];
                let mode = match &f.instance.effect {
                    E::ColorKey {
                        match_by, softness, ..
                    } => {
                        k[2] = *softness;
                        k[3] = (match_by == "hue") as u8 as f64;
                        19
                    }
                    E::SelectColor { keep, .. } => {
                        k[3] = (keep == "chosen") as u8 as f64;
                        20
                    }
                    E::LineRecolor { new_color, .. } => {
                        let new = crate::selective_blur::parse_hex(new_color)
                            .expect("compose leaves a valid Line Recolour");
                        for c in 0..3 {
                            k[4 + c] = crate::color::srgb_to_linear(new[c] as f32 / 255.0) as f64;
                        }
                        21
                    }
                    _ => unreachable!("matched above"),
                };
                k.extend(targets.iter().flat_map(|t| t.map(f64::from)));
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Colorama {
                get_phase,
                phase_shift,
                cycle_repetitions,
                stops,
                color_1,
                color_2,
                color_3,
                color_4,
                color_5,
                blend_with_original,
                interpolate,
                opacity_1,
                opacity_2,
                opacity_3,
                opacity_4,
                opacity_5,
                modify,
                modify_alpha,
                change_empty,
                matching_mode,
                matching_color,
                matching_tolerance,
                matching_softness,
                composite_over,
                ..
            } => {
                let n = (stops.floor() as usize).clamp(2, 5);
                let place = |list: &[&str], word: &str| {
                    list.iter()
                        .position(|g| *g == word)
                        .map_or(-1.0, |i| i as f64)
                };
                let on = |word: &String| (word == "on") as u8 as f64;
                let mut k = vec![
                    place(&crate::effects::COLORAMA_GET, get_phase).max(0.0),
                    *cycle_repetitions,
                    phase_shift / 360.0,
                    n as f64,
                    blend_with_original / 100.0,
                ];
                k.extend(
                    [color_1, color_2, color_3, color_4, color_5]
                        .iter()
                        .flat_map(|c| crate::effects::encoded(c)),
                );
                k.extend(
                    [opacity_1, opacity_2, opacity_3, opacity_4, opacity_5].map(|o| o / 100.0),
                );
                k.extend([
                    place(&crate::effects::COLORAMA_MODIFY, modify).max(0.0),
                    on(modify_alpha),
                    on(change_empty),
                    on(interpolate),
                ]);
                k.push(place(&crate::effects::CHANGE_MATCHES, matching_mode));
                k.extend(crate::effects::encoded(matching_color));
                k.extend([
                    matching_tolerance / 100.0,
                    matching_softness / 100.0,
                    on(composite_over),
                ]);
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 22,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Extract {
                channel,
                black_point,
                white_point,
                black_softness,
                white_softness,
                invert,
            } => {
                let place = ["red", "green", "blue", "alpha"]
                    .iter()
                    .position(|c| c == channel)
                    .unwrap_or(4) as f64;
                let k = [
                    place,
                    *black_point,
                    *white_point,
                    *black_softness,
                    *white_softness,
                    (invert == "on") as u8 as f64,
                ];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 23,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::BroadcastSafe {
                locale,
                method,
                max_amplitude,
            } => {
                let setup = crate::effects::broadcast_setup(locale);
                let place = crate::effects::BROADCAST_METHODS
                    .iter()
                    .position(|m| m == method)
                    .unwrap_or(0) as f64;
                let k = [
                    setup,
                    *max_amplitude,
                    (max_amplitude - setup) / (100.0 - setup),
                    place,
                ];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 24,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::ColorNeutralizer {
                shadows_unbalance,
                midtones_unbalance,
                highlights_unbalance,
                shadows,
                midtones,
                highlights,
                pinning,
                black_point,
                white_point,
            } => {
                let d = crate::effects::neutral_corrections(
                    [shadows_unbalance, midtones_unbalance, highlights_unbalance],
                    [shadows, midtones, highlights],
                );
                let mut k: Vec<f64> = d.iter().flatten().copied().collect();
                k.extend([pinning / 200.0, *black_point, *white_point]);
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 25,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::ColorOffset {
                red_phase,
                green_phase,
                blue_phase,
                overflow,
            } => {
                let phases = [*red_phase, *green_phase, *blue_phase];
                let turns = phases
                    .map(|p| std::f64::consts::PI * p / 360.0)
                    .map(f64::sin_cos);
                let mut k: Vec<f64> = phases.map(|p| p / 360.0).to_vec();
                k.push(
                    crate::effects::OFFSET_OVERFLOWS
                        .iter()
                        .position(|o| o == overflow)
                        .unwrap_or(0) as f64,
                );
                k.extend(turns.map(|t| t.1));
                k.extend(turns.map(|t| t.0));
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 26,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Kernel {
                line_1,
                line_2,
                line_3,
                divider,
                absolute_values,
            } => {
                let mut k: Vec<f64> = crate::effects::kernel_grid([line_1, line_2, line_3])
                    .iter()
                    .flatten()
                    .copied()
                    .collect();
                k.extend([*divider, (absolute_values == "on") as u8 as f64]);
                same(
                    steps,
                    &passes.relief,
                    FxParams {
                        mode: 2,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Toner {
                tones,
                highlights,
                brights,
                midtones,
                darktones,
                shadows,
            } => {
                let stops = crate::effects::toner_stops(
                    tones,
                    [highlights, brights, midtones, darktones, shadows],
                );
                let mut k = vec![(stops.len() - 1) as f64];
                k.extend(stops.iter().flatten());
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 27,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::ChangeColor {
                view,
                hue_transform,
                lightness_transform,
                saturation_transform,
                color_to_change,
                tolerance,
                softness,
                match_colors,
                invert_mask,
            } => {
                let mut k = crate::effects::encoded(color_to_change).to_vec();
                k.extend([
                    tolerance / 100.0,
                    softness / 100.0,
                    crate::effects::CHANGE_MATCHES
                        .iter()
                        .position(|m| m == match_colors)
                        .unwrap_or(1) as f64,
                    *hue_transform,
                    lightness_transform / 100.0,
                    saturation_transform / 100.0,
                    (view == "mask") as u8 as f64,
                    (invert_mask == "on") as u8 as f64,
                ]);
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 28,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::ColorBalanceHls {
                hue,
                lightness,
                saturation,
            } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 29,
                    ..Default::default()
                },
                &[*hue, lightness / 100.0, saturation / 100.0],
                None,
            ),
            E::GammaPedestalGain {
                black_stretch,
                gamma,
                pedestal,
                gain,
            } => {
                let mut k = vec![*black_stretch];
                for c in 0..3 {
                    k.extend([1.0 / gamma[c], pedestal[c], gain[c]]);
                }
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 31,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-397: its steps drawn in one pass of their own, each a stage of the grade or tone
            // pass. Not one of a run (`one_pixel`): a run's Mix lands on each stage, and the Mix
            // is the whole grade's.
            E::ColorGrade { .. } => {
                use crate::effects::GradeStep as G;
                let stage = |which: u32, mode: u32, k: Vec<f64>| {
                    (
                        which,
                        FxParams {
                            mode,
                            ..Default::default()
                        },
                        k,
                        1.0,
                    )
                };
                let mine: Vec<Staged> =
                    crate::effects::color_grade_steps(&f.instance.effect, (w, h), f.origin)
                        .into_iter()
                        .map(|s| match s {
                            G::Tone(g, t) => {
                                let mut k = g.to_vec();
                                k.extend(t);
                                k.push((g != [1.0; 3]) as u8 as f64);
                                stage(1, 40, k)
                            }
                            G::Vibrance(v, s) => stage(1, 7, vec![v, s]),
                            G::Look(cube, i) => {
                                let mut k = cube.packed();
                                k.push(i);
                                stage(1, 41, k)
                            }
                            G::Tints(sh, hi) => {
                                let mut k = sh.to_vec();
                                k.extend([0.0; 3]);
                                k.extend(hi);
                                k.push(0.0);
                                stage(0, 6, k)
                            }
                            // D-398: Curves' pass, red, green, blue then the master, as E::Curves.
                            G::Curves(c) => {
                                let mut k = vec![0.0; 4];
                                for (i, c) in [&c[1], &c[2], &c[3], &c[0]].into_iter().enumerate() {
                                    k[i] = k.len() as f64;
                                    let (xs, ys, m) = crate::grade::knots(c);
                                    k.push(xs.len() as f64);
                                    k.extend(xs.into_iter().chain(ys).chain(m));
                                }
                                stage(0, 0, k)
                            }
                            G::HueSaturation(points) => {
                                let mut k = vec![points.len() as f64];
                                k.extend(points.iter().map(|p| p[0]));
                                k.extend(points.iter().map(|p| p[1]));
                                stage(1, 42, k)
                            }
                            // D-571: the same count and numbers, modes 120 to 123 in
                            // COLOR_GRADE_MORE_CURVES' order.
                            G::More(which, points) => {
                                let mut k = vec![points.len() as f64];
                                k.extend(points.iter().map(|p| p[0]));
                                k.extend(points.iter().map(|p| p[1]));
                                stage(1, 120 + which as u32, k)
                            }
                            G::Wheels(sh, mid, hi) => {
                                let mut k = sh.to_vec();
                                k.extend(mid);
                                k.extend(hi);
                                k.push(0.0);
                                stage(0, 6, k)
                            }
                            G::Secondary(s) => {
                                let mut k = s.key.to_vec();
                                k.extend([
                                    s.invert as u8 as f64,
                                    s.mask as u8 as f64,
                                    s.gains.is_some() as u8 as f64,
                                ]);
                                k.extend(s.gains.unwrap_or([1.0; 3]));
                                k.extend([s.contrast, s.saturation]);
                                k.extend(s.push);
                                k.push(s.three.is_some() as u8 as f64);
                                k.extend(s.three.unwrap_or([[0.0; 3]; 2]).into_iter().flatten());
                                stage(1, 43, k)
                            }
                            G::Vignette(v) => {
                                let mut k = vec![
                                    v.center.0,
                                    v.center.1,
                                    v.radii.0,
                                    v.radii.1,
                                    v.inner,
                                    v.outer,
                                    v.amount,
                                    2f64.sqrt(),
                                ];
                                k.extend(v.color.map(crate::grade::to_linear));
                                k.push(v.norm);
                                stage(0, 8, k)
                            }
                            G::Sharpen(_) => unreachable!(
                                "D-637: compose keeps a sharpened grade on the processor"
                            ),
                        })
                        .collect();
                if mine.is_empty() {
                    (still.clone(), (w, h))
                } else {
                    (self.chain(steps, still, (w, h), &mine), (w, h))
                }
            }
            E::PhotoFilter {
                filter,
                color,
                density,
                preserve_luminosity,
            } => {
                let mut k = crate::effects::photo_filter_colour(filter, color).to_vec();
                k.extend([density / 100.0, (preserve_luminosity == "on") as u8 as f64]);
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 32,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::SelectiveColor { method, families } => {
                let mut k: Vec<f64> = crate::effects::selective_color_amounts(families)
                    .iter()
                    .flatten()
                    .map(|v| v / 100.0)
                    .collect();
                k.push((method == "relative") as u8 as f64);
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 60,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            spill @ E::AdvancedSpillSuppressor { .. } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 61,
                    ..Default::default()
                },
                &crate::grade::spill_settings(spill),
                None,
            ),
            cdk @ E::ColorDifferenceKey { .. } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 70,
                    ..Default::default()
                },
                &crate::grade::cdk_settings(cdk),
                None,
            ),
            crange @ E::ColorRange { .. } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 71,
                    ..Default::default()
                },
                &crate::grade::crange_settings(crange),
                None,
            ),
            lckey @ E::LinearColorKey { .. } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 72,
                    ..Default::default()
                },
                &crate::grade::lckey_settings(lckey),
                None,
            ),
            u @ E::Unmult { .. } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 73,
                    ..Default::default()
                },
                &crate::grade::unmult_settings(u),
                None,
            ),
            skey @ E::ScreenKey { .. } => same(
                steps,
                &passes.tone,
                FxParams {
                    mode: 90,
                    ..Default::default()
                },
                &crate::grade::skey_settings(skey),
                None,
            ),
            E::ColorLink {
                sample,
                clip,
                stencil,
                opacity,
                blending_mode: b,
                map,
                ..
            } => {
                // card_can leaves only a named layer's picture; its colour is read here, once.
                let map = map
                    .as_ref()
                    .expect("card_can takes Color Link only with its picture");
                let stats =
                    crate::frame_stats::Stats::of(&map.0).expect("compose leaves a colour to link");
                let c = crate::frame_stats::link_colour(&stats, sample, *clip);
                let k = [
                    c[0],
                    c[1],
                    c[2],
                    opacity / 100.0,
                    (stencil == "on") as u8 as f64,
                ];
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 30,
                        blend: blend(b),
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::Paraffin {
                color,
                direction,
                spread,
                opacity,
                blend: b,
            } => {
                // No pixel covered half or more: nothing is washed.
                let (span, share) = match crate::grade::paraffin_span(source, *direction, *spread) {
                    Some(span) => (span, opacity / 100.0),
                    None => ([0.0, 0.0, 0.0, 1.0], 0.0),
                };
                let mut k = span.to_vec();
                k.push(share);
                k.extend(crate::effects::encoded(color));
                same(
                    steps,
                    &passes.tone,
                    FxParams {
                        mode: 13,
                        blend: blend(b),
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::LineBlur {
                length,
                strength,
                lines_only,
            } => {
                let g = crate::line_blur::GROW;
                let weights = crate::line_blur::weights(*length);
                let mut k = vec![strength / 100.0, (lines_only == "on") as u8 as f64];
                k.extend(&weights);
                let out = self.scratch("B-123 line blur", w + 2 * g, h + 2 * g);
                let p = FxParams {
                    g: g as i32,
                    count: weights.len() as u32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.line_blur,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(w + 2 * g, h + 2 * g),
                );
                (out, (w + 2 * g, h + 2 * g))
            }
            E::KiraKira {
                threshold,
                spacing,
                density,
                size,
                shape,
                angle,
                twinkle,
                period,
                seed,
                opacity,
                color,
                frame,
            } => {
                let g = f.grow.0;
                let stars = crate::layer_fx::kira_stars(
                    source,
                    [
                        *threshold, *spacing, *density, *size, *twinkle, *period, *seed,
                    ],
                    *frame,
                    f.origin,
                );
                let arms = crate::layer_fx::kira_arms(*angle, shape == "star");
                let mut k = vec![(g + ox) as f64, (g + oy) as f64, opacity / 100.0];
                k.extend(linear(color));
                k.push(arms.len() as f64);
                k.extend(arms.iter().flat_map(|&((vx, vy), share)| [vx, vy, share]));
                k.resize(19, 0.0);
                k.extend(stars.iter().flatten());
                let (tw, th) = (w + 2 * g, h + 2 * g);
                let lit = buffer(tw * th * 4);
                let n = stars.len() as u32;
                let p = FxParams {
                    g: g as i32,
                    count: n,
                    ..Default::default()
                };
                if n > 0 {
                    self.fx_step(
                        steps,
                        &passes.stars,
                        p,
                        Some(still),
                        None,
                        Some(&k),
                        None,
                        [None, None, None, None, Some(&lit)],
                        (n.min(65535), n.div_ceil(65535)),
                    );
                }
                let out = self.scratch("B-123 kira", tw, th);
                self.fx_step(
                    steps,
                    &passes.kira,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None, None, None, None, Some(&lit)],
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            // D-551: the balls from the drawing the layer sent, as Kira-kira's stars; each pixel's
            // first and second ball laid, then lit.
            e @ E::CcBallAction { .. } => {
                let k = crate::layer_fx::ball_numbers(source, e, f.origin);
                let n = ((k.len() - crate::layer_fx::BALL_HEAD) / 6) as u32;
                let keys = buffer(2 * w * h * 4);
                let p = FxParams {
                    count: n,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                for mode in [0, 1].into_iter().filter(|_| n > 0) {
                    let groups = (n.min(65535), n.div_ceil(65535));
                    self.fx_step(
                        steps,
                        &passes.balls,
                        FxParams { mode, ..p },
                        Some(still),
                        None,
                        Some(&k),
                        None,
                        [None, None, None, None, Some(&keys)],
                        groups,
                    );
                }
                let out = self.scratch("D-551 balls", w, h);
                self.fx_step(
                    steps,
                    &passes.ballshade,
                    p,
                    None,
                    Some(&out),
                    Some(&k),
                    None,
                    [None, None, None, None, Some(&keys)],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-610: the sprites from the drawing the layer sent, binned into the card's tiles,
            // the lists after them. D-611: the texture's levels as the other picture, its size
            // as n by r (a clear 1 by 1 with none read).
            // D-612: Particle Plane's the same way. D-617: Particle Emitter's too. D-618: Particle Grid's.
            // D-619: Particle Playground's particles worked out on the processor, drawn by the
            // particles pass as dots, pieces of the Explode layer and copies of the Layer Map, the
            // two maps side by side in its atlas.
            e @ E::ParticlePlayground { .. } => {
                let (mut k, atlas, [ew, eh, lw, lh]) =
                    crate::playground::sprites(e, ((w - 2 * ox) as f64, (h - 2 * oy) as f64));
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                let atlas = self.map_texture(&atlas);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    n: ew as u32,
                    r: eh as i32,
                    base: [lw as u32, lh as u32],
                    ..Default::default()
                };
                let out = self.scratch("D-619 playground", w, h);
                self.fx_step(
                    steps,
                    &passes.particles,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&atlas),
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            e @ (E::ParticleSpace { .. }
            | E::ParticlePlane { .. }
            | E::ParticleEmitter { .. }
            | E::ParticleGrid { .. }) => {
                let mut k = crate::particles::sprite_numbers(source, e, f.origin);
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                let tex = crate::particles::texture(e);
                let atlas = match tex {
                    Some(t) => self.map_texture(&crate::particles::atlas(t)),
                    None => self.map_texture(&crate::WorkingBuffer::transparent(1, 1)),
                };
                let (tw, th) = tex.map_or((1, 1), |t| (t.width(), t.height()));
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    flag: tex.is_some() as u32,
                    n: tw as u32,
                    r: th as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-610 particles", w, h);
                self.fx_step(
                    steps,
                    &passes.particles,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&atlas),
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-671: the bubbles in the drawing's own space, binned into the card's tiles as the
            // particles are, the lists after them.
            e @ E::Bubbles { words, .. } => {
                let (w0, h0) = (w.saturating_sub(2 * ox), h.saturating_sub(2 * oy));
                let mut k = crate::simulate::bubble_list(e, w0 as f64, h0 as f64);
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                let shading = crate::effects::BUBBLES_WORDS[1]
                    .1
                    .iter()
                    .position(|s| *s == words[1])
                    .unwrap_or(3);
                k[..4].copy_from_slice(&[
                    if words[0] == "world" { 1.0 } else { -1.0 },
                    shading as f64,
                    w0 as f64,
                    h0 as f64,
                ]);
                k[6] = ((k.len() - crate::particles::HEAD) / crate::particles::STRIDE) as f64;
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-671 bubbles", w, h);
                self.fx_step(
                    steps,
                    &passes.bubbles,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-672: the drops in the drawing's own space, binned into the card's tiles as the
            // particles are, the lists after them, then the light.
            e @ E::Drizzle { .. } => {
                let (w0, h0) = (w.saturating_sub(2 * ox), h.saturating_sub(2 * oy));
                let mut k = crate::simulate::drizzle_list(e, w0 as f64, h0 as f64);
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                k[6] = ((k.len() - crate::particles::HEAD) / crate::particles::STRIDE) as f64;
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                k[0] = k.len() as f64;
                k.extend(crate::simulate::drizzle_light(e, w0 as f64, h0 as f64));
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-672 drizzle", w, h);
                self.fx_step(
                    steps,
                    &passes.drizzle,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-673: the strands in the drawing's own space, binned into the card's tiles as the
            // particles are, the lists after them, then the shared numbers.
            e @ E::Hair { .. } => {
                let (w0, h0) = (w.saturating_sub(2 * ox), h.saturating_sub(2 * oy));
                let mut k = crate::simulate::hair_list(e, w0 as f64, h0 as f64);
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                k[6] = ((k.len() - crate::particles::HEAD) / crate::particles::STRIDE) as f64;
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                k[0] = k.len() as f64;
                k.extend(crate::simulate::hair_light(e, h0 as f64));
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-673 hair", w, h);
                self.fx_step(
                    steps,
                    &passes.hair,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-674: the drops in the drawing's own space, binned into the card's tiles as the
            // particles are, the lists after them, then the light.
            e @ E::Mercury { .. } => {
                let (w0, h0) = (w.saturating_sub(2 * ox), h.saturating_sub(2 * oy));
                let mut k = crate::simulate::mercury_list(e, w0 as f64, h0 as f64);
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                k[6] = ((k.len() - crate::particles::HEAD) / crate::particles::STRIDE) as f64;
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                k[0] = k.len() as f64;
                k.extend(crate::simulate::mercury_light(e, w0 as f64, h0 as f64));
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-674 mercury", w, h);
                self.fx_step(
                    steps,
                    &passes.mercury,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-675: the shards worked on the processor from the drawing the layer sent, which
            // they only begin a run with (their own colours are read from it), binned into the
            // card's tiles as the particles are, the lists after them, then the numbers every
            // pixel shares and each shard's sixteen. Before the start time, the drawing as it is.
            e @ E::PolygonShards { .. } => {
                let (sw, sh) = (source.width(), source.height());
                let (w0, h0) = (sw.saturating_sub(2 * ox), sh.saturating_sub(2 * oy));
                let Some((mut k, blocks, shared)) = (w0 > 0 && h0 > 0)
                    .then(|| crate::simulate::shards_list(e, source.data(), sw, f.origin, w0, h0))
                    .flatten()
                else {
                    return (still.clone(), (w, h));
                };
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                k[6] = ((k.len() - crate::particles::HEAD) / crate::particles::STRIDE) as f64;
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                k[0] = k.len() as f64;
                k.extend(shared);
                k.extend(blocks);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-675 shards", w, h);
                self.fx_step(
                    steps,
                    &passes.shards,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-678: the bubbles stepped on the processor and laid out on screen in the drawing's
            // own space, binned into the card's tiles as the particles are, the lists after them,
            // then the shared numbers; the card only draws.
            e @ E::Foam { .. } => {
                let (w0, h0) = (w.saturating_sub(2 * ox), h.saturating_sub(2 * oy));
                if w0 == 0 || h0 == 0 {
                    return (still.clone(), (w, h));
                }
                let (mut k, shared) = crate::simulate::foam_list(e, w0 as f64, h0 as f64);
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                k[2] = w0 as f64;
                k[3] = h0 as f64;
                k[6] = ((k.len() - crate::particles::HEAD) / crate::particles::STRIDE) as f64;
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                k[0] = k.len() as f64;
                k.extend(shared);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-678 foam", w, h);
                self.fx_step(
                    steps,
                    &passes.foam,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-676: the dots worked out and binned into the card's tiles on the processor, three
            // numbers each, not the particle engine's sprites; the card reads each dot's colour
            // from the drawing and lays them in order.
            e @ E::Scatterize { .. } => {
                let (w0, h0) = (w.saturating_sub(2 * ox), h.saturating_sub(2 * oy));
                if w0 == 0 || h0 == 0 {
                    return (still.clone(), (w, h));
                }
                let k = crate::simulate::scatterize_list(e, (w0, h0), (ox, oy), (w, h));
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-676 scatterize", w, h);
                self.fx_step(
                    steps,
                    &passes.scatterize,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-677: the stars in the drawing's own space, binned into the card's tiles as the
            // particles are, the lists after them; the card reads their colours from the drawing.
            e @ E::StarBurst { .. } => {
                let (w0, h0) = (w.saturating_sub(2 * ox), h.saturating_sub(2 * oy));
                if w0 == 0 || h0 == 0 {
                    return (still.clone(), (w, h));
                }
                let mut k = crate::simulate::star_list(e, w0 as f64, h0 as f64);
                let (starts, list) = crate::particles::tile_lists(&k, f.origin, w, h);
                k.extend(starts.iter().chain(&list).map(|&i| i as f64));
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let out = self.scratch("D-677 star burst", w, h);
                self.fx_step(
                    steps,
                    &passes.star_burst,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [None; 5],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // B-151: ten of the fourth batch.
            E::Median {
                radius,
                operate_on_alpha,
            } => {
                let k: Vec<f64> = crate::layer_fx::disc_runs(*radius)
                    .iter()
                    .map(|&(_, hw)| hw as f64)
                    .collect();
                let p = FxParams {
                    r: radius.floor() as i32,
                    flag: (operate_on_alpha == "on") as u32,
                    ..Default::default()
                };
                same(steps, &passes.median, p, &k, None)
            }
            // D-453: the same pass in mode 1, k the threshold, then level8's power and 0, then
            // the disc's rows, as SmartBlur's.
            E::DustScratches {
                radius,
                threshold,
                operate_on_alpha,
            } => {
                let power = f64::from(1.0f32 / 2.4) - 1.0 / 2.4;
                let mut k = vec![*threshold, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, power, 0.0];
                k.extend(
                    crate::layer_fx::disc_runs(*radius)
                        .iter()
                        .map(|&(_, hw)| hw as f64),
                );
                let p = FxParams {
                    r: radius.floor() as i32,
                    n: 9,
                    mode: 1,
                    flag: (operate_on_alpha == "on") as u32,
                    ..Default::default()
                };
                same(steps, &passes.median, p, &k, None)
            }
            E::SmartBlur { radius, threshold } => {
                // The threshold, then level8's power and 0, then the disc's rows.
                let power = f64::from(1.0f32 / 2.4) - 1.0 / 2.4;
                let mut k = vec![*threshold, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, power, 0.0];
                k.extend(
                    crate::layer_fx::disc_runs(*radius)
                        .iter()
                        .map(|&(_, hw)| hw as f64),
                );
                let p = FxParams {
                    r: radius.floor() as i32,
                    n: 9,
                    ..Default::default()
                };
                let (levels, _) = same(steps, &passes.smart, p, &k, Some(still));
                same(
                    steps,
                    &passes.smart,
                    FxParams { mode: 1, ..p },
                    &k,
                    Some(&levels),
                )
            }
            // B-237: alone, only Colorize off reaches here.
            E::BilateralBlur {
                radius,
                threshold,
                colorize,
            } => bilateral(steps, *radius, *threshold, colorize == "on"),
            // D-448: the smoothing as Bilateral Blur's (none when it would count only the pixel
            // itself), then the edges, then the fill and the line.
            e @ E::Cartoon {
                detail_radius,
                detail_threshold,
                ..
            } => {
                let k = crate::median::cartoon_k(e).expect("a Cartoon");
                let smooth = if *detail_radius < 1.0 || *detail_threshold <= 0.0 {
                    still.clone()
                } else {
                    bilateral(steps, *detail_radius, *detail_threshold, true).0
                };
                let edge = self.scratch("D-448 edges", w, h);
                self.fx_step(
                    steps,
                    &passes.cartoonedge,
                    FxParams::default(),
                    Some(&smooth),
                    Some(&edge),
                    Some(&k),
                    None,
                    none,
                    tiles(w, h),
                );
                let out = self.scratch("D-448", w, h);
                self.fx_step(
                    steps,
                    &passes.cartoon,
                    FxParams::default(),
                    Some(&smooth),
                    Some(&out),
                    Some(&k),
                    Some(&edge),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            E::RoughenEdges {
                edge_type,
                edge_color,
                border,
                size,
                complexity,
                evolution,
                speed,
                seed,
                frame,
            } => {
                let base = crate::grade::mix(seed.floor() as u64);
                let mut k = vec![
                    *border,
                    *size,
                    crate::effects::depth(*evolution, *speed, *frame),
                    (edge_type == "roughen_color") as u8 as f64,
                ];
                k.extend(linear(edge_color));
                let p = FxParams {
                    count: complexity.floor() as u32,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.rough, p, &k, None)
            }
            E::RadialShadow {
                color,
                opacity,
                light,
                distance,
                softness,
                render,
                color_influence,
                shadow_only,
            } => {
                // The drawing cast from the light, blurred, then laid behind the drawing.
                let (lx, ly, scale, gx, gy) =
                    crate::layer_fx::radial_cast(*light, *distance, (w, h), f.origin);
                let (cw, ch) = (w + 2 * gx, h + 2 * gy);
                let cast = self.scratch("B-151 cast", cw, ch);
                let k = [lx, ly, scale, gx as f64, gy as f64];
                self.fx_step(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 11,
                        ..Default::default()
                    },
                    Some(still),
                    Some(&cast),
                    Some(&k),
                    None,
                    none,
                    tiles(cw, ch),
                );
                let (blurred, (bw, bh)) = self.gaussian(
                    steps,
                    &cast,
                    (cw, ch),
                    Gaussian {
                        sigma: softness / 3.0,
                        repeat: false,
                        long: false,
                    },
                );
                let r = (bw - cw) / 2;
                let mut k = vec![
                    opacity / 100.0,
                    color_influence / 100.0,
                    (render == "glass_edge") as u8 as f64,
                    (shadow_only == "on") as u8 as f64,
                ];
                k.extend(linear(color));
                let out = self.scratch("B-151 radial shadow", bw, bh);
                let p = FxParams {
                    ox: (gx + r) as i32,
                    oy: (gy + r) as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.rshadow,
                    p,
                    Some(&blurred),
                    Some(&out),
                    Some(&k),
                    Some(still),
                    none,
                    tiles(bw, bh),
                );
                (out, (bw, bh))
            }
            E::BevelAlpha {
                edge_thickness,
                light_angle,
                light_color,
                light_intensity,
            } => {
                let (blurred, r) = covering(steps, still, (w, h), edge_thickness / 2.0);
                let (ux, uy) = crate::blurs::along(*light_angle);
                let mut k = vec![*edge_thickness, ux, uy, *light_intensity];
                k.extend(linear(light_color));
                same(
                    steps,
                    &passes.bevel,
                    FxParams {
                        r: r as i32,
                        ..Default::default()
                    },
                    &k,
                    Some(&blurred),
                )
            }
            E::Snowfall {
                color,
                density,
                spacing,
                size,
                depth,
                speed,
                wind,
                wiggle,
                period,
                seed,
                opacity,
                frame,
            } => {
                let base = crate::grade::mix(seed.floor() as u64);
                let mut k = vec![density / 100.0, *opacity, *period, *frame as f64];
                k.extend(
                    crate::layer_fx::snow_planes(
                        [*spacing, *size, *depth, *speed, *wind, *wiggle],
                        *frame,
                    )
                    .iter()
                    .flatten(),
                );
                k.extend(linear(color));
                let p = FxParams {
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.snow, p, &k, None)
            }
            E::CellPattern {
                pattern,
                invert,
                contrast,
                disperse,
                size,
                evolution,
                seed,
                dark_color,
                light_color,
                opacity,
                blend: b,
            } => {
                let (m0, n0, cols, points) = crate::grade::cell_points(
                    [*disperse, *size, *evolution, *seed],
                    (w, h),
                    f.origin,
                );
                let code = ["bubbles", "crystals", "plates"]
                    .iter()
                    .position(|p| *p == pattern.as_str())
                    .unwrap_or(3);
                let mut k = vec![
                    *size,
                    *contrast,
                    opacity / 100.0,
                    code as f64,
                    (invert == "on") as u8 as f64,
                    m0 as f64,
                    n0 as f64,
                    cols as f64,
                ];
                k.extend(crate::effects::encoded(dark_color));
                k.extend(crate::effects::encoded(light_color));
                k.push(0.0);
                k.extend(points.iter().flat_map(|&(x, y, grey)| [x, y, grey]));
                same(
                    steps,
                    &passes.cells,
                    FxParams {
                        blend: blend(b),
                        ox: ox as i32,
                        oy: oy as i32,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::PolarCoordinates {
                interpolation,
                conversion,
                shape,
            } => {
                let (dw, dh) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let (rx, ry) = if shape == "circle" {
                    (dw.min(dh) / 2.0, dw.min(dh) / 2.0)
                } else {
                    (dw / 2.0, dh / 2.0)
                };
                let k = [
                    interpolation / 100.0,
                    (conversion == "rect_to_polar") as u8 as f64,
                    dw,
                    dh,
                    ox as f64,
                    oy as f64,
                    rx,
                    ry,
                    rx / (dw / 2.0),
                    ry / (dh / 2.0),
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 8,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::OpticsCompensation {
                field_of_view,
                reverse,
                orientation,
                center,
            } => {
                let (dw, dh) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let r = match orientation.as_str() {
                    "vertical" => dh / 2.0,
                    "diagonal" => dw.hypot(dh) / 2.0,
                    _ => dw / 2.0,
                };
                let (cx, cy) = (
                    center[0] / 100.0 * dw + ox as f64,
                    center[1] / 100.0 * dh + oy as f64,
                );
                let k = [
                    cx,
                    cy,
                    r,
                    field_of_view.to_radians() / 2.0,
                    (reverse == "on") as u8 as f64,
                    0.0,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 9,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-408: one moment; a Transform through the shutter stays off the card (card_can).
            // A map that cannot be undone draws nothing, at opacity 0.
            E::Transform { sampling, .. } => {
                let mut k = [0.0; 9];
                let m = crate::effects::transform_moment(&f.instance.effect).expect("a Transform");
                if let Some(map) = crate::layer_fx::transform_map(&m, (w, h), (ox, oy)) {
                    k[..6].copy_from_slice(&map);
                    k[6] = m[9] / 100.0;
                }
                k[8] = (sampling == "bicubic") as u8 as f64;
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 22,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::CornerPin {
                upper_left,
                upper_right,
                lower_left,
                lower_right,
            } => {
                // Crossed corners leave the determinant 0, which draws nothing.
                let (gx, gy) = f.grow;
                let mut k = [0.0; 17];
                if let Some((adj, det, _)) = crate::layer_fx::corner_map(
                    [*upper_left, *upper_right, *lower_left, *lower_right],
                    (w, h),
                    f.origin,
                ) {
                    k[..9].copy_from_slice(adj.as_flattened());
                    k[9] = det;
                }
                k[10..16].copy_from_slice(&[
                    (w - 2 * ox) as f64,
                    (h - 2 * oy) as f64,
                    ox as f64,
                    oy as f64,
                    (ox + gx) as f64,
                    (oy + gy) as f64,
                ]);
                let (tw, th) = (w + 2 * gx, h + 2 * gy);
                let out = self.scratch("B-151 corner pin", tw, th);
                self.fx_step(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 10,
                        ..Default::default()
                    },
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            // B-223 (D-342): five blurs, each as its CPU function.
            E::FastBoxBlur {
                radius,
                iterations,
                edges,
                dimensions,
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                let taps = crate::effects::box_weights(*radius, *iterations);
                let flat = crate::effects::still_weights(taps.len() / 2);
                let pick = |on: bool| if on { &taps[..] } else { &flat[..] };
                let repeat = edges == "repeat";
                let e = if repeat { 0 } else { taps.len() / 2 };
                let out = self.gauss(
                    steps,
                    bloom,
                    "B-223 box",
                    still,
                    (w, h),
                    [
                        pick(dimensions != "vertical"),
                        pick(dimensions != "horizontal"),
                    ],
                    e,
                    repeat,
                );
                (out, (w + 2 * e, h + 2 * e))
            }
            // D-471: Fast Box Blur's passes at its radius and 3 iterations.
            E::FastBlurLegacy {
                blurriness,
                edges,
                dimensions,
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                let taps = crate::effects::box_weights(
                    crate::effects::FAST_BLUR_LEGACY_FACTOR * blurriness,
                    crate::effects::FAST_BLUR_LEGACY_ITERATIONS,
                );
                let flat = crate::effects::still_weights(taps.len() / 2);
                let pick = |on: bool| if on { &taps[..] } else { &flat[..] };
                let repeat = edges == "repeat";
                let e = if repeat { 0 } else { taps.len() / 2 };
                let out = self.gauss(
                    steps,
                    bloom,
                    "D-471 box",
                    still,
                    (w, h),
                    [
                        pick(dimensions != "vertical"),
                        pick(dimensions != "horizontal"),
                    ],
                    e,
                    repeat,
                );
                (out, (w + 2 * e, h + 2 * e))
            }
            // D-360: Fast Box Blur's passes across and, apart, down, then laid together.
            E::CrossBlur {
                radius_x,
                radius_y,
                mode,
                edges,
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                let [across, down] = crate::blurs::cross_taps(*radius_x, *radius_y);
                let flat = crate::effects::still_weights(across.len() / 2);
                let repeat = edges == "repeat";
                let e = if repeat { 0 } else { across.len() / 2 };
                let a = self.gauss(
                    steps,
                    bloom,
                    "D-360 across",
                    still,
                    (w, h),
                    [&across, &flat],
                    e,
                    repeat,
                );
                let d = self.gauss(
                    steps,
                    bloom,
                    "D-360 down",
                    still,
                    (w, h),
                    [&flat, &down],
                    e,
                    repeat,
                );
                let (tw, th) = (w + 2 * e, h + 2 * e);
                let out = self.scratch("D-360 cross", tw, th);
                let mode = crate::effects::CROSS_MODES
                    .iter()
                    .position(|m| m == mode)
                    .unwrap_or(0) as u32;
                self.fx_step(
                    steps,
                    &passes.crossmix,
                    FxParams {
                        mode,
                        ..Default::default()
                    },
                    Some(&a),
                    Some(&out),
                    None,
                    Some(&d),
                    none,
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            E::ChannelBlur {
                red_blurriness,
                green_blurriness,
                blue_blurriness,
                alpha_blurriness,
                edges,
                dimensions,
                units,
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                let repeat = edges == "repeat";
                // D-380: each number as D-321's units take it.
                let blurred = |steps: &mut Vec<Step>, number: f64| {
                    let (sigma, long) = crate::effects::blur_reach(number, units);
                    let taps = crate::effects::reach_weights(sigma, long);
                    let flat = crate::effects::still_weights(taps.len() / 2);
                    let pick = |on: bool| if on { &taps[..] } else { &flat[..] };
                    let e = if repeat { 0 } else { taps.len() / 2 };
                    let axes = [
                        pick(dimensions != "vertical"),
                        pick(dimensions != "horizontal"),
                    ];
                    (
                        self.gauss(
                            steps,
                            bloom,
                            "B-223 channel",
                            still,
                            (w, h),
                            axes,
                            e,
                            repeat,
                        ),
                        e,
                    )
                };
                let sigma = [
                    *red_blurriness,
                    *green_blurriness,
                    *blue_blurriness,
                    *alpha_blurriness,
                ];
                let (mut out, g) = blurred(steps, sigma[3]);
                let (tw, th) = (w + 2 * g, h + 2 * g);
                for c in 0..3 {
                    if sigma[c] == sigma[3] {
                        continue;
                    }
                    let (b, r) = blurred(steps, sigma[c]);
                    let next = self.scratch("B-223 channel mix", tw, th);
                    let p = FxParams {
                        count: c as u32,
                        g: r as i32 - g as i32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.chanmix,
                        p,
                        Some(&out),
                        Some(&next),
                        None,
                        Some(&b),
                        none,
                        tiles(tw, th),
                    );
                    out = next;
                }
                (out, (tw, th))
            }
            E::SelectiveColorBlur {
                blur,
                colors,
                tolerance,
            } => {
                // k: the targets' count, the tolerance, 0s, level8's power and 0, then the targets.
                let r = (blur + 0.5).floor() as usize;
                let targets = crate::selective_blur::targets(colors);
                let power = f64::from(1.0f32 / 2.4) - 1.0 / 2.4;
                let mut k = vec![
                    targets.len() as f64,
                    *tolerance,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    power,
                    0.0,
                ];
                k.extend(targets.iter().flat_map(|t| t.map(f64::from)));
                let mut chosen = self.scratch("B-223 chosen", w, h);
                self.fx_step(
                    steps,
                    &passes.selprep,
                    FxParams::default(),
                    Some(still),
                    Some(&chosen),
                    Some(&k),
                    None,
                    none,
                    tiles(w, h),
                );
                let zone = r as f64 / 3.0;
                let tbl: Vec<f64> = (0..=r)
                    .map(|j| (-((j * j) as f64) / (2.0 * zone * zone)).exp() as f32 as f64)
                    .collect();
                for (reach, across) in [
                    (r, true),
                    (r, false),
                    (r / 4, true),
                    (r / 16, false),
                    (r / 64, true),
                ] {
                    if reach > 0 {
                        let next = self.scratch("B-223 chosen", w, h);
                        let p = FxParams {
                            n: reach as u32,
                            flag: across as u32,
                            ..Default::default()
                        };
                        self.fx_step(
                            steps,
                            &passes.selpass,
                            p,
                            Some(&chosen),
                            Some(&next),
                            Some(&tbl),
                            None,
                            none,
                            tiles(w, h),
                        );
                        chosen = next;
                    }
                }
                let out = self.scratch("B-223 selective", w, h);
                self.fx_step(
                    steps,
                    &passes.selfinish,
                    FxParams::default(),
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&chosen),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            E::VectorBlur {
                kind,
                amount,
                angle_offset,
                ridge_smoothness,
                property,
                map_softness,
                map,
                ..
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                // The height from the map compose read, lying on the drawing at its corner, or the drawing.
                let (picture, (mx, my)) = match map {
                    Some(m) => (self.map_texture(&m.0), (ox, oy)),
                    None => (still.clone(), (0, 0)),
                };
                let place = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| p == property)
                    .expect("compose leaves a valid CC Vector Blur");
                let raw = self.scratch("B-223 height", w, h);
                let p = FxParams {
                    ox: mx as i32,
                    oy: my as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.vheight,
                    p,
                    Some(&picture),
                    Some(&raw),
                    Some(&[place as f64]),
                    None,
                    none,
                    tiles(w, h),
                );
                let (height, r) = if *map_softness > 0.0 {
                    let taps = crate::effects::gaussian_weights(map_softness / 2.0);
                    let r = taps.len() / 2;
                    (
                        self.gauss(
                            steps,
                            bloom,
                            "B-223 height",
                            &raw,
                            (w, h),
                            [&taps, &taps],
                            r,
                            false,
                        ),
                        r,
                    )
                } else {
                    (raw, 0)
                };
                let kind_at = crate::effects::VECTOR_BLUR_TYPES
                    .iter()
                    .position(|t| t == kind)
                    .expect("compose leaves a valid CC Vector Blur");
                let turn =
                    (angle_offset + if kind == "perpendicular" { 90.0 } else { 0.0 }).to_radians();
                let k = [
                    *amount,
                    *angle_offset,
                    *ridge_smoothness,
                    amount.ceil(),
                    kind_at as f64,
                    turn.cos(),
                    turn.sin(),
                ];
                let out = self.scratch("B-223 vector", w, h);
                self.fx_step(
                    steps,
                    &passes.vblur,
                    FxParams {
                        r: r as i32,
                        ..Default::default()
                    },
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&height),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-462: the height as CC Vector Blur's, then one pass that samples the drawing.
            E::MrSmoothie {
                property,
                smoothness,
                sample_a,
                sample_b,
                phase,
                color_loop,
                map,
                ..
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                let (picture, (mx, my)) = match map {
                    Some(m) => (self.map_texture(&m.0), (ox, oy)),
                    None => (still.clone(), (0, 0)),
                };
                let place = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| p == property)
                    .expect("compose leaves a valid Mr. Smoothie");
                let raw = self.scratch("D-462 height", w, h);
                let p = FxParams {
                    ox: mx as i32,
                    oy: my as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.vheight,
                    p,
                    Some(&picture),
                    Some(&raw),
                    Some(&[place as f64]),
                    None,
                    none,
                    tiles(w, h),
                );
                let (height, r) = if *smoothness > 0.0 {
                    let taps = crate::effects::gaussian_weights(smoothness / 2.0);
                    let r = taps.len() / 2;
                    (
                        self.gauss(
                            steps,
                            bloom,
                            "D-462 height",
                            &raw,
                            (w, h),
                            [&taps, &taps],
                            r,
                            false,
                        ),
                        r,
                    )
                } else {
                    (raw, 0)
                };
                let k = crate::layer_fx::mr_smoothie_setup(
                    *phase,
                    color_loop,
                    *sample_a,
                    *sample_b,
                    (w, h),
                    (ox, oy),
                );
                let out = self.scratch("D-462 smoothie", w, h);
                self.fx_step(
                    steps,
                    &passes.smoothie,
                    FxParams {
                        r: r as i32,
                        ..Default::default()
                    },
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&height),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-463: the bump as Blobbylize's, then one pass that lights the drawing on it.
            E::Plastic {
                property,
                softness,
                height,
                cut_min,
                cut_max,
                light_intensity,
                light_color,
                light_type,
                light_height,
                light_position,
                light_direction,
                ambient_light_color,
                ambient,
                diffuse,
                dust,
                specular,
                roughness,
                metal,
                map,
                ..
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                let (picture, (mx, my)) = match map {
                    Some(m) => (self.map_texture(&m.0), (ox, oy)),
                    None => (still.clone(), (0, 0)),
                };
                let place = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| p == property)
                    .expect("compose leaves a valid Plastic");
                let raw = self.scratch("D-463 bump", w, h);
                let p = FxParams {
                    ox: mx as i32,
                    oy: my as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.vheight,
                    p,
                    Some(&picture),
                    Some(&raw),
                    Some(&[place as f64]),
                    None,
                    none,
                    tiles(w, h),
                );
                let (bump, r) = if *softness > 0.0 {
                    let taps = crate::effects::gaussian_weights(softness / 2.0);
                    let r = taps.len() / 2;
                    (
                        self.gauss(
                            steps,
                            bloom,
                            "D-463 bump",
                            &raw,
                            (w, h),
                            [&taps, &taps],
                            r,
                            false,
                        ),
                        r,
                    )
                } else {
                    (raw, 0)
                };
                let (ux, uy) = crate::blurs::along(*light_direction);
                let d = [100.0 * ux, 100.0 * uy, *light_height];
                let dl = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                let d = if dl > 0.0 {
                    d.map(|c| c / dl)
                } else {
                    [0.0; 3]
                };
                let (px, py) = crate::effects::radial_center(*light_position, (w, h), f.origin);
                let l = linear(light_color);
                let al = linear(ambient_light_color);
                let k = [
                    cut_min / 100.0,
                    cut_max / 100.0,
                    height / 100.0 * 1.25 * softness.max(1.0),
                    (light_type == "point") as u8 as f64,
                    px,
                    py,
                    *light_height,
                    d[0],
                    d[1],
                    d[2],
                    l[0],
                    l[1],
                    l[2],
                    al[0],
                    al[1],
                    al[2],
                    light_intensity / 100.0,
                    ambient / 100.0,
                    diffuse / 100.0,
                    dust / 100.0,
                    specular / 100.0,
                    *roughness,
                    metal / 100.0,
                ];
                let out = self.scratch("D-463 plastic", w, h);
                self.fx_step(
                    steps,
                    &passes.plastic,
                    FxParams {
                        r: r as i32,
                        ..Default::default()
                    },
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&bump),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            E::CompoundBlur {
                max_blur,
                invert,
                edges,
                map,
                ..
            } => {
                let map = self.map_texture(
                    &map.as_ref()
                        .expect("compose leaves a Compound Blur with a map")
                        .0,
                );
                let big = max_blur / 3.0;
                let levels = [0.0, big / 16.0, big / 8.0, big / 4.0, big / 2.0, big];
                let dist = buffer(w * h * 8);
                let work = [None, None, None, Some(&dist)];
                let p = FxParams {
                    n: w as u32,
                    count: h as u32,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.csigma,
                    p,
                    Some(&map),
                    None,
                    Some(&[big, (invert == "on") as u8 as f64]),
                    None,
                    work,
                    tiles(w, h),
                );
                // ponytail: all five levels are blurred, where the CPU stops at the map's brightest;
                // find the brightest first if a dark map is slow here.
                let out = self.scratch("B-223 compound", w, h);
                let mut low = (still.clone(), 0);
                for j in 0..5 {
                    let g = Gaussian {
                        sigma: levels[j + 1],
                        repeat: edges == "repeat",
                        long: false,
                    };
                    let (high, _) = self.gaussian(steps, still, (w, h), g);
                    let p = FxParams {
                        n: j as u32,
                        g: low.1 as i32,
                        r: g.grow() as i32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.cmix,
                        p,
                        Some(&low.0),
                        Some(&out),
                        Some(&levels),
                        Some(&high),
                        work,
                        tiles(w, h),
                    );
                    low = (high, g.grow());
                }
                (out, (w, h))
            }
            // B-224 (D-343): two that read a map, each as its CPU function.
            E::DisplacementMap {
                horizontal,
                max_horizontal,
                vertical,
                max_vertical,
                wrap,
                red_amount,
                green_amount,
                blue_amount,
                spectrum,
                map,
                ..
            } => {
                let map = self.map_texture(
                    &map.as_ref()
                        .expect("compose leaves a Displacement Map with a map")
                        .0,
                );
                // `push`'s kinds: `phase`'s, 9 full, 10 off.
                let kind = |word: &str| match word {
                    "full" => 9.0,
                    "off" => 10.0,
                    _ => crate::effects::VECTOR_BLUR_PROPERTIES
                        .iter()
                        .position(|p| *p == word)
                        .expect("compose leaves a valid Displacement Map")
                        as f64,
                };
                let g = f.grow.0;
                // D-412: then the samples' count (0 for none) and the three shares.
                let (most, a) = crate::effects::chromatic_amounts(
                    [*max_horizontal, *max_vertical],
                    [*red_amount, *green_amount, *blue_amount],
                );
                let n = if a.is_some() { spectrum.floor() } else { 0.0 };
                let a = a.unwrap_or([1.0; 3]);
                let k = [
                    kind(horizontal),
                    kind(vertical),
                    most[0],
                    most[1],
                    (wrap == "on") as u8 as f64,
                    n,
                    a[0],
                    a[1],
                    a[2],
                ];
                let (tw, th) = (w + 2 * g, h + 2 * g);
                let out = self.scratch("B-224 displacement", tw, th);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    g: g as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.dmap,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&map),
                    none,
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            E::Glass {
                property,
                softness,
                height,
                displacement,
                light_angle,
                light_color,
                light_intensity,
                light_type,
                light_height,
                light_position,
                ambient,
                diffuse,
                specular,
                roughness,
                metal,
                map,
                ..
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                // The bump from the map compose read, lying on the drawing at its corner, or the drawing.
                let (picture, (mx, my)) = match map {
                    Some(m) => (self.map_texture(&m.0), (ox, oy)),
                    None => (still.clone(), (0, 0)),
                };
                let place = match property.as_str() {
                    "intensity" => 8,
                    p => crate::effects::VECTOR_BLUR_PROPERTIES
                        .iter()
                        .position(|q| *q == p)
                        .expect("compose leaves a valid CC Glass"),
                };
                let raw = self.scratch("B-224 bump", w, h);
                let p = FxParams {
                    ox: mx as i32,
                    oy: my as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.vheight,
                    p,
                    Some(&picture),
                    Some(&raw),
                    Some(&[place as f64]),
                    None,
                    none,
                    tiles(w, h),
                );
                let (bump, r) = if *softness > 0.0 {
                    let taps = crate::effects::gaussian_weights(softness / 2.0);
                    let r = taps.len() / 2;
                    (
                        self.gauss(
                            steps,
                            bloom,
                            "B-224 bump",
                            &raw,
                            (w, h),
                            [&taps, &taps],
                            r,
                            false,
                        ),
                        r,
                    )
                } else {
                    (raw, 0)
                };
                let (ux, uy) = crate::blurs::along(*light_angle);
                let l = linear(light_color);
                let mode = match light_type.as_str() {
                    "distant" => 1.0,
                    "point" => 2.0,
                    _ => 0.0,
                };
                let (px, py) = crate::effects::radial_center(*light_position, (w, h), (ox, oy));
                let d = [100.0 * ux, 100.0 * uy, *light_height];
                let dl = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                let d = if dl > 0.0 {
                    d.map(|c| c / dl)
                } else {
                    [0.0; 3]
                };
                let k = [
                    *displacement,
                    height / 100.0 * 1.25 * softness.max(1.0),
                    ux,
                    uy,
                    light_intensity / 100.0,
                    l[0],
                    l[1],
                    l[2],
                    mode,
                    px,
                    py,
                    *light_height,
                    d[0],
                    d[1],
                    d[2],
                    ambient / 100.0,
                    diffuse / 100.0,
                    specular / 100.0,
                    *roughness,
                    metal / 100.0,
                ];
                let out = self.scratch("B-224 glass", w, h);
                self.fx_step(
                    steps,
                    &passes.glass,
                    FxParams {
                        r: r as i32,
                        ..Default::default()
                    },
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&bump),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            E::Blobbylize {
                property,
                softness,
                cut_away,
                light_intensity,
                light_color,
                light_type,
                light_height,
                light_position,
                light_direction,
                ambient,
                diffuse,
                specular,
                roughness,
                metal,
                map,
                ..
            } => {
                let bloom = self
                    .bloom
                    .as_ref()
                    .expect("a blur is refused without the passes");
                // The blob from the map compose read, lying on the drawing at its corner, or the drawing.
                let (picture, (mx, my)) = match map {
                    Some(m) => (self.map_texture(&m.0), (ox, oy)),
                    None => (still.clone(), (0, 0)),
                };
                let place = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| p == property)
                    .expect("compose leaves a valid Blobbylize");
                let raw = self.scratch("D-379 blob", w, h);
                let p = FxParams {
                    ox: mx as i32,
                    oy: my as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.vheight,
                    p,
                    Some(&picture),
                    Some(&raw),
                    Some(&[place as f64]),
                    None,
                    none,
                    tiles(w, h),
                );
                let (bump, spread, r) = if *softness > 0.0 {
                    let taps = crate::effects::gaussian_weights(softness / 2.0);
                    let r = taps.len() / 2;
                    let bump = self.gauss(
                        steps,
                        bloom,
                        "D-379 blob",
                        &raw,
                        (w, h),
                        [&taps, &taps],
                        r,
                        false,
                    );
                    (
                        bump,
                        self.gauss(
                            steps,
                            bloom,
                            "D-379 spread",
                            still,
                            (w, h),
                            [&taps, &taps],
                            r,
                            false,
                        ),
                        r,
                    )
                } else {
                    (raw, still.clone(), 0)
                };
                let colour = self.scratch("D-379 colour", w, h);
                self.fx_step(
                    steps,
                    &passes.blobover,
                    FxParams {
                        r: r as i32,
                        ..Default::default()
                    },
                    Some(still),
                    Some(&colour),
                    None,
                    Some(&spread),
                    none,
                    tiles(w, h),
                );
                let (ux, uy) = crate::blurs::along(*light_direction);
                let d = [100.0 * ux, 100.0 * uy, *light_height];
                let dl = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                let d = if dl > 0.0 {
                    d.map(|c| c / dl)
                } else {
                    [0.0; 3]
                };
                let (px, py) = crate::effects::radial_center(*light_position, (w, h), f.origin);
                let l = linear(light_color);
                let k = [
                    cut_away / 100.0,
                    1.25 * softness.max(1.0),
                    (light_type == "point") as u8 as f64,
                    px,
                    py,
                    *light_height,
                    d[0],
                    d[1],
                    d[2],
                    l[0],
                    l[1],
                    l[2],
                    light_intensity / 100.0,
                    ambient / 100.0,
                    diffuse / 100.0,
                    specular / 100.0,
                    *roughness,
                    metal / 100.0,
                ];
                let out = self.scratch("D-379 blobby", w, h);
                self.fx_step(
                    steps,
                    &passes.blobby,
                    FxParams {
                        r: r as i32,
                        ..Default::default()
                    },
                    Some(&colour),
                    Some(&out),
                    Some(&k),
                    Some(&bump),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-424: the blobs as the CPU lays them, and the bands of rows each reaches, handed
            // over in `k`; one warp pass.
            E::GlueGun {
                stroke_width,
                density,
                reflection,
                strength,
                paint_style,
                wobble_width,
                wobble_height,
                wobble_speed,
                light_intensity,
                light_color,
                light_type,
                light_height,
                light_position,
                light_direction,
                ambient,
                diffuse,
                specular,
                roughness,
                metal,
                trail,
                clock,
                ..
            } => {
                let trail: Vec<(f64, f64)> = trail
                    .iter()
                    .map(|p| crate::effects::radial_center(*p, (w, h), f.origin))
                    .collect();
                let wobble = (paint_style == "wobbly").then_some((
                    *wobble_width,
                    *wobble_height,
                    *wobble_speed,
                ));
                let blobs = crate::layer_fx::glue_blobs(&trail, *clock, *density, wobble);
                let r = stroke_width / 2.0;
                let big = r * (1.0 + strength / 100.0);
                let t0 = (1.0 - (r / big) * (r / big)) * (1.0 - (r / big) * (r / big));
                let (ux, uy) = crate::blurs::along(*light_direction);
                let d = [100.0 * ux, 100.0 * uy, *light_height];
                let dl = (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
                let d = if dl > 0.0 {
                    d.map(|c| c / dl)
                } else {
                    [0.0; 3]
                };
                let (px, py) = crate::effects::radial_center(*light_position, (w, h), f.origin);
                let l = linear(light_color);
                let mut k = vec![
                    r,
                    big,
                    t0,
                    reflection / 100.0 * 2.0 * r,
                    (light_type == "point") as u8 as f64,
                    px,
                    py,
                    *light_height,
                    d[0],
                    d[1],
                    d[2],
                    l[0],
                    l[1],
                    l[2],
                    light_intensity / 100.0,
                    ambient / 100.0,
                    diffuse / 100.0,
                    specular / 100.0,
                    *roughness,
                    metal / 100.0,
                    blobs.len() as f64,
                ];
                k.extend(blobs.iter().flat_map(|b| [b.0, b.1]));
                let bands = crate::layer_fx::glue_bands(&blobs, big, h);
                let mut first = k.len() + 2 * bands.len();
                for band in &bands {
                    k.extend([first as f64, band.len() as f64]);
                    first += band.len();
                }
                k.extend(bands.concat().into_iter().map(f64::from));
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 34,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-425: one warp pass, the cloth's numbers in `k`.
            E::Threads {
                width,
                height,
                overlaps,
                direction,
                center,
                coverage,
                shadowing,
                texture,
            } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let (s, c) = direction.to_radians().sin_cos();
                let k = [
                    cx,
                    cy,
                    s,
                    c,
                    *width,
                    *height,
                    overlaps.floor(),
                    coverage / 100.0,
                    shadowing / 100.0,
                    0.0,
                    texture / 100.0,
                ];
                same(
                    steps,
                    &passes.warp,
                    FxParams {
                        mode: 35,
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            // D-426: one warp pass, the parts placed on the processor.
            E::LensFlare {
                flare_center,
                flare_brightness,
                lens_type,
                blend_with_original,
                dirt,
            } => {
                let (ox, oy) = f.origin;
                let parts = crate::layer_fx::flare_parts(
                    lens_type,
                    crate::effects::radial_center(*flare_center, (w, h), f.origin),
                    crate::effects::radial_center([50.0, 50.0], (w, h), f.origin),
                    ((w - 2 * ox) as f64).hypot((h - 2 * oy) as f64),
                    flare_brightness / 100.0,
                );
                let mut k = vec![1.0 - blend_with_original / 100.0, parts.len() as f64];
                k.extend(parts.iter().flatten());
                // D-545: with dirt, its own pass reading the dirt picture.
                match dirt.lit() {
                    Some(d) => {
                        k.extend(dirt_k(&d, (w, h), 0));
                        let picture = self.map_texture(&d.picture);
                        same(
                            steps,
                            &passes.flaredirt,
                            FxParams::default(),
                            &k,
                            Some(&picture),
                        )
                    }
                    None => same(
                        steps,
                        &passes.warp,
                        FxParams {
                            mode: 36,
                            ..Default::default()
                        },
                        &k,
                        None,
                    ),
                }
            }
            // D-444: its own pass, the dashes' bits worked out here as the CPU works them, from
            // the paths compose found, in bands as Path Stroke's runs are; with no path the card
            // is not asked (`compose::card_effect`).
            E::Vegas {
                segments,
                length,
                segment_distribution,
                rotation,
                random_phase,
                random_seed,
                blend_mode,
                color,
                width,
                hardness,
                start_opacity,
                mid_point_opacity,
                mid_point_position,
                end_opacity,
                paths,
                ..
            } => {
                let bits = match paths {
                    Some(p) if *width != 0.0 => crate::along::vegas_bits(
                        p,
                        f.origin,
                        [*segments, *length, *rotation, *random_seed],
                        segment_distribution == "even",
                        random_phase == "on",
                    ),
                    _ => Vec::new(),
                };
                let r = width / 2.0;
                let mode = crate::effects::VEGAS_MODES
                    .iter()
                    .position(|m| m == blend_mode)
                    .unwrap_or(0) as f64;
                let mut k = vec![r, (r * (1.0 - hardness)).max(1.0)];
                k.extend(crate::effects::vegas_fade(
                    *start_opacity,
                    *mid_point_opacity,
                    *mid_point_position,
                    *end_opacity,
                ));
                k.push(mode);
                k.extend(linear(color));
                k.push(bits.len() as f64);
                let mut bands = vec![Vec::new(); (h as usize).div_ceil(16)];
                for (j, bit) in bits.iter().enumerate() {
                    let b = crate::along::run_box(&[bit[0], bit[1], bit[2], bit[3], 0.0], r + 0.5);
                    k.extend(bit);
                    k.extend(b);
                    let rows = (b[2] - 0.5).floor().max(0.0) as usize
                        ..=((b[3] - 0.5).ceil().max(0.0) as usize).min(h as usize - 1);
                    for band in bands
                        .iter_mut()
                        .take(rows.end() / 16 + 1)
                        .skip(rows.start() / 16)
                    {
                        band.push(j as f64);
                    }
                }
                let mut first = k.len() + 2 * bands.len();
                for band in &bands {
                    k.extend([first as f64, band.len() as f64]);
                    first += band.len();
                }
                k.extend(bands.concat());
                same(steps, &passes.vegas, FxParams::default(), &k, None)
            }
            // D-446: the field over the drawing and a margin round it in one pass, then the flow
            // lines through it in a second, as grade::curl_noise works them. D-583: This Layer or
            // Other Layer first makes the picture's lightness plane, blurred by Softness, as
            // grade::CurlPlane does.
            e @ E::CurlNoise {
                source,
                softness,
                mirror_at_edge,
                map,
                sample_count,
                sample_radius,
                flow_softness,
                flow_falloff,
                view,
                contrast,
                brightness,
                clip_hdr_results,
                channel,
                float,
                ..
            } => {
                let c = crate::grade::Curl::new(e, (w - 2 * ox, h - 2 * oy)).expect("a Curl Noise");
                let m = crate::grade::curl_margin(*sample_radius);
                let (gw, gh) = (w + 2 * m, h + 2 * m);
                let mut k = vec![c.size, c.shift[0], c.shift[1], c.z];
                k.extend(c.amps);
                k.extend(c.bias);
                k.extend([c.edge, c.swirl]);
                let picture = match source.as_str() {
                    "this_layer" => Some((still.clone(), (w, h), [ox as f64, oy as f64])),
                    "other_layer" => {
                        let m = &map
                            .as_ref()
                            .expect("compose leaves an Other Layer Curl Noise whose layer was read")
                            .0;
                        Some((self.map_texture(m), (m.width(), m.height()), [0.0, 0.0]))
                    }
                    _ => None,
                };
                let plane = picture.map(|(from, (pw, ph), origin)| {
                    let place = crate::effects::VECTOR_BLUR_PROPERTIES
                        .iter()
                        .position(|p| *p == "luminance")
                        .expect("luminance is a property");
                    let raw = self.scratch("D-583 plane", pw, ph);
                    self.fx_step(
                        steps,
                        &passes.vheight,
                        FxParams::default(),
                        Some(&from),
                        Some(&raw),
                        Some(&[place as f64]),
                        None,
                        none,
                        tiles(pw, ph),
                    );
                    k.extend([
                        1.0,
                        origin[0],
                        origin[1],
                        (mirror_at_edge == "on") as u8 as f64,
                    ]);
                    let taps = crate::effects::gaussian_weights(softness / 10.0);
                    if taps.len() > 1 {
                        let kk = |axis: f64| -> Vec<f64> {
                            [axis, (taps.len() / 2) as f64]
                                .into_iter()
                                .chain(taps.iter().map(|&t| t as f64))
                                .collect()
                        };
                        let (across, down) = (
                            self.scratch("D-583 across", pw, ph),
                            self.scratch("D-583 down", pw, ph),
                        );
                        self.fx_step(
                            steps,
                            &passes.curlsoft,
                            FxParams::default(),
                            Some(&raw),
                            Some(&across),
                            Some(&kk(0.0)),
                            None,
                            none,
                            tiles(pw, ph),
                        );
                        self.fx_step(
                            steps,
                            &passes.curlsoft,
                            FxParams::default(),
                            Some(&across),
                            Some(&down),
                            Some(&kk(1.0)),
                            None,
                            none,
                            tiles(pw, ph),
                        );
                        down
                    } else {
                        raw
                    }
                });
                if plane.is_none() {
                    k.extend([0.0; 4]);
                }
                // D-646: k[16] 1 turns the noise by k[17..22] (grade::Curl::turn), k[22] 1 slides
                // each finer octave half as far.
                k.push(c.turn.is_some() as u8 as f64);
                k.extend(c.turn.unwrap_or_default());
                k.push(c.perspective as u8 as f64);
                let base = crate::grade::mix(0);
                let p = FxParams {
                    g: m as i32,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                // Audit P9: a field from the noise alone is kept, and drawn again only when the
                // numbers that make it change; a picture's plane is drawn each time.
                let key: Option<Vec<u64>> = plane.is_none().then(|| {
                    k.iter()
                        .map(|v| v.to_bits())
                        .chain([m, ox, oy, gw, gh].map(|v| v as u64))
                        .collect()
                });
                let kept = key.as_ref().and_then(|key| {
                    let mut fields = self.fields.borrow_mut();
                    let f = fields.iter_mut().find(|f| &f.0 == key)?;
                    f.2 = self.frame;
                    Some(f.1.clone())
                });
                let field = match kept {
                    Some(field) => field,
                    None => {
                        let field = self.scratch("D-446 field", gw, gh);
                        self.fx_step(
                            steps,
                            &passes.curlfield,
                            p,
                            Some(plane.as_ref().unwrap_or(still)),
                            Some(&field),
                            Some(&k),
                            None,
                            none,
                            tiles(gw, gh),
                        );
                        if let Some(key) = key {
                            if let Some(working) = &self.working {
                                working.borrow_mut().retain(|(t, _)| t != field.texture());
                            }
                            self.fields
                                .borrow_mut()
                                .push((key, field.clone(), self.frame));
                        }
                        field
                    }
                };
                let at =
                    |list: &[&str], v: &str| list.iter().position(|o| *o == v).unwrap_or(0) as f64;
                let k = [
                    *sample_radius,
                    sample_count.floor(),
                    flow_softness / 100.0,
                    flow_falloff / 100.0,
                    at(&crate::effects::CURL_VIEWS, view),
                    *contrast,
                    *brightness,
                    !(clip_hdr_results == "off" && *float) as u8 as f64,
                    at(&crate::effects::CURL_CHANNELS, channel),
                    plane.is_some() as u8 as f64,
                ];
                same(
                    steps,
                    &passes.curlflow,
                    FxParams {
                        g: m as i32,
                        ..Default::default()
                    },
                    &k,
                    Some(&field),
                )
            }
            // D-447: one pass, as grade::brush_strokes works it.
            e @ E::BrushStrokes { .. } => {
                let b = crate::grade::Brush::new(e).expect("a Brush Strokes");
                let k = [
                    b.d[0],
                    b.d[1],
                    b.su,
                    b.sv,
                    b.len,
                    b.r,
                    b.m,
                    b.slope,
                    b.s,
                    b.reach,
                    b.surface as f64,
                    b.blend,
                ];
                let p = FxParams {
                    base: [b.base as u32, (b.base >> 32) as u32],
                    frame: b.frame as i32,
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.brush, p, &k, None)
            }
            // B-225 (D-344): five generators, each as its CPU function; the bolt's segments are
            // worked out here, as the CPU works them, and handed over in `k`.
            E::Beam {
                start,
                end,
                length,
                time,
                start_thickness,
                end_thickness,
                softness,
                inside_color,
                outside_color,
                composite,
            } => {
                let (s, e) = (
                    crate::effects::radial_center(*start, (w, h), f.origin),
                    crate::effects::radial_center(*end, (w, h), f.origin),
                );
                let l = length / 100.0;
                let a = time / 100.0 * (1.0 - l);
                let (dx, dy) = (l * (e.0 - s.0), l * (e.1 - s.1));
                let mut k = vec![
                    s.0 + a * (e.0 - s.0),
                    s.1 + a * (e.1 - s.1),
                    dx,
                    dy,
                    dx * dx + dy * dy,
                    a,
                    l,
                ];
                k.extend([
                    *start_thickness,
                    *end_thickness,
                    *softness,
                    (composite == "off") as u8 as f64,
                ]);
                k.extend(linear(inside_color));
                k.extend(linear(outside_color));
                same(steps, &passes.beam, FxParams::default(), &k, None)
            }
            E::FourColorGradient {
                point_1,
                point_2,
                point_3,
                point_4,
                color_1,
                color_2,
                color_3,
                color_4,
                blend: b,
                opacity,
                blending_mode,
            } => {
                let mut k: Vec<f64> = [point_1, point_2, point_3, point_4]
                    .iter()
                    .flat_map(|p| {
                        let (x, y) = crate::effects::radial_center(**p, (w, h), f.origin);
                        [x, y]
                    })
                    .collect();
                k.extend([100.0 / b, opacity / 100.0]);
                for c in [color_1, color_2, color_3, color_4] {
                    k.extend(crate::effects::encoded(c));
                }
                same(
                    steps,
                    &passes.gradient4,
                    FxParams {
                        blend: blend(blending_mode),
                        ..Default::default()
                    },
                    &k,
                    None,
                )
            }
            E::LightSweep {
                center,
                direction,
                shape,
                width,
                sweep_intensity,
                edge_intensity,
                edge_thickness,
                light_color,
                light_reception,
            } => {
                let r = width / 2.0;
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let (nx, ny) = crate::blurs::along(direction + 90.0);
                let least = *edge_intensity > 0.0 && r > 0.0;
                let shape = match shape.as_str() {
                    "linear" => 1.0,
                    "smooth" => 2.0,
                    _ => 0.0,
                };
                let reception = match light_reception.as_str() {
                    "add" => 0.0,
                    "composite" => 1.0,
                    _ => 2.0,
                };
                let mut k = vec![
                    cx,
                    cy,
                    nx,
                    ny,
                    r,
                    sweep_intensity / 100.0,
                    edge_intensity / 100.0,
                    shape,
                    reception,
                    least as u8 as f64,
                ];
                k.extend(linear(light_color));
                let rows = buffer(if least { w * h * 4 } else { 4 });
                let p = FxParams {
                    r: edge_thickness.floor() as i32,
                    ..Default::default()
                };
                if least {
                    self.fx_step(
                        steps,
                        &passes.sweepmin,
                        p,
                        Some(still),
                        None,
                        None,
                        None,
                        [Some(&rows)],
                        tiles(w, h),
                    );
                }
                let out = self.scratch("B-225 sweep", w, h);
                self.fx_step(
                    steps,
                    &passes.sweep,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [Some(&rows)],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-345: Radio Waves, its waves worked out here as the CPU works them.
            E::RadioWaves {
                producer_point,
                sides,
                interval,
                expansion,
                orientation,
                direction,
                velocity,
                spin,
                lifespan,
                opacity,
                fade_in_time,
                fade_out_time,
                start_width,
                end_width,
                profile,
                color,
                frame,
            } => {
                let s = crate::layer_fx::RadioWaves {
                    producer: crate::effects::radial_center(*producer_point, (w, h), f.origin),
                    sides: *sides,
                    interval: *interval,
                    expansion: *expansion,
                    orientation: *orientation,
                    direction: *direction,
                    velocity: *velocity,
                    spin: *spin,
                    lifespan: *lifespan,
                    opacity: *opacity,
                    fade_in: *fade_in_time,
                    fade_out: *fade_out_time,
                    widths: [*start_width, *end_width],
                    profile,
                    color: linear(color),
                    frame: *frame,
                };
                let list = crate::layer_fx::radio_wave_list(&s);
                let n = sides.floor();
                let profile = match profile.as_str() {
                    "square" => 0.0,
                    "triangle" => 1.0,
                    _ => 2.0,
                };
                let mut k = vec![n, 360.0 / n, (std::f64::consts::PI / n).cos(), profile];
                k.extend(s.color);
                k.push(list.len() as f64);
                k.extend(
                    list.iter()
                        .flat_map(|&(x, y, r, t, hw, g)| [x, y, r, t, hw, g]),
                );
                same(steps, &passes.waves, FxParams::default(), &k, None)
            }
            E::LightningBolt {
                start,
                end,
                jagged,
                detail,
                branches,
                width,
                glow,
                opacity,
                hold,
                seed,
                color,
                glow_color,
                composite,
                kind,
                turbulence,
                decay,
                conductivity,
                obstacle,
                path,
                core,
                forks,
                frame,
            } => {
                // D-324: Alpha Obstacle reads the drawing it is given; compose has a bolt with
                // one begin its run, so that is `source`, the CPU's.
                let edge = 1.0 - obstacle / 100.0;
                let blocks: Vec<bool> = match *obstacle {
                    a if a > 0.0 => source
                        .data()
                        .chunks_exact(4)
                        .map(|p| p[3] as f64 > edge)
                        .collect(),
                    a if a < 0.0 => source
                        .data()
                        .chunks_exact(4)
                        .map(|p| (p[3] as f64) < -a / 100.0)
                        .collect(),
                    _ => Vec::new(),
                };
                let segs = if *opacity == 0.0 || (*width == 0.0 && *glow == 0.0) {
                    Vec::new()
                } else {
                    let size = (w, h);
                    let ends = [start, end, &[start[0], 100.0]]
                        .map(|p| crate::effects::radial_center(*p, size, f.origin));
                    let kinds = (
                        kind.as_str(),
                        [*turbulence, *decay, *conductivity],
                        forks.as_str(),
                    );
                    crate::layer_fx::bolt_list(
                        size,
                        ends,
                        [*jagged, *detail, *branches, *hold, *seed],
                        kinds,
                        (&blocks, *obstacle < 0.0, path == "around"),
                        *frame,
                    )
                };
                let mut k = vec![
                    *width,
                    *glow,
                    (core == "soft") as u8 as f64,
                    opacity / 100.0,
                ];
                k.extend(linear(color));
                k.extend(linear(glow_color));
                k.extend([(composite == "off") as u8 as f64, segs.len() as f64]);
                for s in &segs {
                    // Its box, as `lightning_bolt` culls by it.
                    let ww = s[4].max(s[5]);
                    let e = (ww * width / 2.0 + 0.5).max(ww * glow);
                    k.extend(s);
                    k.extend([
                        s[0].min(s[2]) - e,
                        s[0].max(s[2]) + e,
                        s[1].min(s[3]) - e,
                        s[1].max(s[3]) + e,
                    ]);
                }
                same(steps, &passes.bolt, FxParams::default(), &k, None)
            }
            // B-235 (D-356): Path Stroke's runs, worked out here as the CPU works them, from the
            // paths compose found; with none the card is not asked (`compose::card_effect`).
            // D-442: Scribble's lines are drawn by the same pass, found here as the CPU finds them.
            e @ (E::Stroke { .. } | E::Scribble { .. }) => {
                let (runs, brush_size, brush_hardness, opacity, color, paint_style) = match e {
                    E::Stroke {
                        all_masks,
                        stroke_sequentially,
                        color,
                        brush_size,
                        brush_hardness,
                        opacity,
                        start,
                        end,
                        spacing,
                        paint_style,
                        paths,
                        ..
                    } => {
                        let runs = match paths {
                            Some(p) if *brush_size != 0.0 && *opacity != 0.0 => {
                                let sequential = all_masks == "on" && stroke_sequentially == "on";
                                crate::along::stroke_runs(
                                    p,
                                    f.origin,
                                    [*start, *end, *spacing, *brush_size],
                                    sequential,
                                )
                            }
                            _ => Vec::new(),
                        };
                        (
                            runs,
                            brush_size,
                            brush_hardness,
                            opacity,
                            color,
                            paint_style,
                        )
                    }
                    E::Scribble {
                        color,
                        opacity,
                        stroke_width,
                        composite,
                        ..
                    } => {
                        let runs = if *stroke_width != 0.0 && *opacity != 0.0 {
                            crate::along::scribble_runs(e, (w as usize, h as usize), f.origin)
                        } else {
                            Vec::new()
                        };
                        (runs, stroke_width, &100.0, opacity, color, composite)
                    }
                    _ => unreachable!("matched as a Path Stroke or a Scribble"),
                };
                let style = crate::effects::PAINT_STYLES
                    .iter()
                    .position(|s| s == paint_style)
                    .unwrap_or(0) as f64;
                let mut k = vec![*brush_size, *brush_hardness, *opacity];
                k.extend(linear(color));
                k.extend([style, runs.len() as f64]);
                let mut bands = vec![Vec::new(); (h as usize).div_ceil(16)];
                for (j, run) in runs.iter().enumerate() {
                    let b = crate::along::run_box(run, brush_size / 2.0 + 0.5);
                    k.extend(run);
                    k.extend(b);
                    let rows = (b[2] - 0.5).floor().max(0.0) as usize
                        ..=((b[3] - 0.5).ceil().max(0.0) as usize).min(h as usize - 1);
                    for band in bands
                        .iter_mut()
                        .take(rows.end() / 16 + 1)
                        .skip(rows.start() / 16)
                    {
                        band.push(j as f64);
                    }
                }
                let mut first = k.len() + 2 * bands.len();
                for band in &bands {
                    k.extend([first as f64, band.len() as f64]);
                    first += band.len();
                }
                k.extend(bands.concat());
                same(steps, &passes.stroke, FxParams::default(), &k, None)
            }
            // D-508: Energy Stroke's bits and numbers, worked out here as the CPU works them;
            // with no path the card is not asked (`compose::card_effect`).
            e @ E::EnergyStroke { .. } => {
                let n = crate::along::energy_numbers(e);
                let mut k = n.to_vec();
                let (bits, ends) =
                    crate::along::energy_bits(e, (w as usize, h as usize), f.origin, &n);
                k.push(bits.len() as f64);
                // D-565: the shaped ends, before the bits.
                k.push(ends.len() as f64);
                k.extend(ends.concat());
                let mut bands = vec![Vec::new(); (h as usize).div_ceil(16)];
                for (j, (bit, b)) in bits.iter().enumerate() {
                    k.extend(bit);
                    k.extend(b);
                    let rows = (b[2] - 0.5).floor().max(0.0) as usize
                        ..=((b[3] - 0.5).ceil().max(0.0) as usize).min(h as usize - 1);
                    for band in bands
                        .iter_mut()
                        .take(rows.end() / 16 + 1)
                        .skip(rows.start() / 16)
                    {
                        band.push(j as f64);
                    }
                }
                let mut first = k.len() + 2 * bands.len();
                for band in &bands {
                    k.extend([first as f64, band.len() as f64]);
                    first += band.len();
                }
                k.extend(bands.concat());
                same(steps, &passes.energy, FxParams::default(), &k, None)
            }
            // D-441: Write-on's marks, placed here as the CPU places them.
            E::WriteOn {
                color,
                paint_style,
                marks,
                ..
            } => {
                let marks =
                    crate::effects::write_on_marks(marks, (w as usize, h as usize), f.origin);
                let style = crate::effects::PAINT_STYLES
                    .iter()
                    .position(|s| s == paint_style)
                    .unwrap_or(0) as f64;
                let mut k = vec![style];
                k.extend(linear(color));
                let kept: Vec<[f64; 5]> = marks
                    .into_iter()
                    .filter(|m| m[2] != 0.0 && m[4] != 0.0)
                    .collect();
                k.push(kept.len() as f64);
                let mut bands = vec![Vec::new(); (h as usize).div_ceil(16)];
                for (j, &[x, y, size, hardness, opacity]) in kept.iter().enumerate() {
                    let r = size / 2.0;
                    let reach = r + 0.5;
                    let b = [x - reach, x + reach, y - reach, y + reach];
                    k.extend([
                        x,
                        y,
                        r,
                        (r * (1.0 - hardness / 100.0)).max(1.0),
                        opacity / 100.0,
                    ]);
                    k.extend(b);
                    let rows = (b[2] - 0.5).floor().max(0.0) as usize
                        ..=((b[3] - 0.5).ceil().max(0.0) as usize).min(h as usize - 1);
                    for band in bands
                        .iter_mut()
                        .take(rows.end() / 16 + 1)
                        .skip(rows.start() / 16)
                    {
                        band.push(j as f64);
                    }
                }
                let mut first = k.len() + 2 * bands.len();
                for band in &bands {
                    k.extend([first as f64, band.len() as f64]);
                    first += band.len();
                }
                k.extend(bands.concat());
                same(steps, &passes.writeon, FxParams::default(), &k, None)
            }
            E::BevelEdges {
                edge_thickness,
                light_angle,
                light_color,
                light_intensity,
            } => {
                let (ux, uy) = crate::blurs::along(*light_angle);
                let mut k = vec![
                    edge_thickness * (w as f64).min(h as f64),
                    ux,
                    uy,
                    *light_intensity,
                ];
                k.extend(linear(light_color));
                same(steps, &passes.edges, FxParams::default(), &k, None)
            }
            // B-226 (D-346): four more, each as its CPU function. Block Dissolve's blocks and Line
            // Width's offsets are worked out here, as the CPU works them, and handed over in `k`;
            // Line Width's chosen pixels and Line Smooth's encoded picture are the CPU's own, from
            // `source`, as compose has each of those two begin its run.
            E::BlockDissolve {
                completion,
                block_width,
                block_height,
                feather,
            } => {
                let k = if *feather > 0.0 && *completion != 100.0 {
                    let (j0, ny, down) = crate::layer_fx::dissolve_down(
                        (w, h),
                        *completion,
                        (*block_width, *block_height),
                        *feather,
                        (ox, oy),
                    );
                    let mut k = vec![
                        1.0,
                        (ny + 1) as f64,
                        j0 as f64,
                        *block_height,
                        feather / 2.0,
                        feather * feather,
                        oy as f64,
                    ];
                    k.extend(down);
                    k
                } else {
                    let kept = crate::layer_fx::dissolve_kept(*completion);
                    let block = |x: usize, o: usize, size: f64| {
                        ((x as f64 - o as f64 + 0.5) / size).floor() as i64
                    };
                    let ix: Vec<i64> = (0..w).map(|x| block(x, ox, *block_width)).collect();
                    let jy: Vec<i64> = (0..h).map(|y| block(y, oy, *block_height)).collect();
                    let mut k = vec![0.0, (ix[w - 1] - ix[0] + 1) as f64];
                    k.extend(ix.iter().map(|i| (i - ix[0]) as f64));
                    k.extend(jy.iter().map(|j| (j - jy[0]) as f64));
                    for j in jy[0]..=jy[h - 1] {
                        k.extend((ix[0]..=ix[w - 1]).map(|i| {
                            if *completion == 100.0 {
                                0.0
                            } else {
                                kept(i, j)
                            }
                        }));
                    }
                    k
                };
                same(steps, &passes.dissolve, FxParams::default(), &k, None)
            }
            E::GradientWipe {
                completion,
                softness,
                invert,
                map,
                ..
            } => {
                let map = self.map_texture(
                    &map.as_ref()
                        .expect("compose leaves a Gradient Wipe with a map")
                        .0,
                );
                let (c, s) = (completion / 100.0, softness / 100.0);
                let luminance = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| *p == "luminance")
                    .expect("luminance is a reading");
                let k = [
                    -s / 2.0 + c * (1.0 + s),
                    s,
                    (invert == "on") as u8 as f64,
                    (*completion == 100.0) as u8 as f64,
                    luminance as f64,
                    0.0,
                ];
                let out = self.scratch("B-226 wipe", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.gwipe,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&map),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-512: compose leaves the card a Channel Wipe only when it wipes and its layer, if
            // one is named, was read; "" for the layer itself.
            E::ChannelWipe {
                completion,
                softness,
                property,
                blur,
                invert,
                map,
                ..
            } => {
                let (picture, (mx, my)) = match map {
                    Some(m) => (self.map_texture(&m.0), (ox, oy)),
                    None => (still.clone(), (0, 0)),
                };
                let kind = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| *p == property.as_str())
                    .expect("compose leaves a Channel Wipe whose property is a reading");
                let (c, s) = (completion / 100.0, softness / 100.0);
                let mut k = [
                    -s / 2.0 + c * (1.0 + s),
                    s,
                    (invert == "on") as u8 as f64,
                    (*completion == 100.0) as u8 as f64,
                    kind as f64,
                    0.0,
                ];
                let out = self.scratch("D-512 wipe", w, h);
                let p = FxParams {
                    ox: mx as i32,
                    oy: my as i32,
                    ..Default::default()
                };
                if *blur > 0.0 && *completion != 100.0 {
                    let raw = self.scratch("D-512 reading", w, h);
                    k[5] = 2.0;
                    self.fx_step(
                        steps,
                        &passes.gwipe,
                        p,
                        Some(still),
                        Some(&raw),
                        Some(&k),
                        Some(&picture),
                        none,
                        tiles(w, h),
                    );
                    let bloom = self
                        .bloom
                        .as_ref()
                        .expect("a blur is refused without the passes");
                    let taps = crate::effects::gaussian_weights(blur / 2.0);
                    let field = self.gauss(
                        steps,
                        bloom,
                        "D-512 reading",
                        &raw,
                        (w, h),
                        [&taps, &taps],
                        0,
                        true,
                    );
                    k[5] = 1.0;
                    self.fx_step(
                        steps,
                        &passes.gwipe,
                        FxParams::default(),
                        Some(still),
                        Some(&out),
                        Some(&k),
                        Some(&field),
                        none,
                        tiles(w, h),
                    );
                } else {
                    self.fx_step(
                        steps,
                        &passes.gwipe,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&k),
                        Some(&picture),
                        none,
                        tiles(w, h),
                    );
                }
                (out, (w, h))
            }
            // D-403: compose leaves the card a haze only when it is even or its matte was read.
            E::AerialHaze {
                haze_color,
                amount,
                map,
                ..
            } => {
                let mut k = vec![amount / 100.0];
                k.extend(crate::effects::encoded(haze_color).map(crate::grade::to_linear));
                k.push(map.is_some() as u8 as f64);
                let matte = map.as_ref().map(|m| self.map_texture(&m.0));
                let out = self.scratch("D-403 haze", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.haze,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(matte.as_ref().unwrap_or(still)),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-524: compose leaves the card a Fog 3D only when its depth (and gradient) was read.
            // ponytail: the shares are worked on the processor each frame; a shader reading the
            // depth and the gradient together if Fog 3D shows in the card's timings.
            e @ E::Fog3d { .. } => {
                let shares = crate::layer_fx::fog_shares(e)
                    .expect("compose leaves a Fog 3D whose depth was read");
                let shares = self.map_texture(&shares);
                let E::Fog3d { fog_color, .. } = e else {
                    unreachable!()
                };
                let k = crate::effects::encoded(fog_color).map(crate::grade::to_linear);
                let out = self.scratch("D-524 fog", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.fog,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&shares),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-501: compose leaves the card a Difference Matte only when its plate was read. With
            // a blur, the shares are worked from both blurred (grown by r) and then applied.
            E::DifferenceMatte {
                view,
                matching_tolerance,
                matching_softness,
                blur_before_difference,
                map,
                ..
            } => {
                let map = &map
                    .as_ref()
                    .expect("compose leaves a Difference Matte with a plate")
                    .0;
                let plate = self.map_texture(map);
                let k = [
                    *matching_tolerance,
                    *matching_softness,
                    (view == "matte") as u8 as f64,
                ];
                let out = self.scratch("D-501 matte", w, h);
                if *blur_before_difference > 0.0 {
                    let bloom = self
                        .bloom
                        .as_ref()
                        .expect("a blur is refused without the passes");
                    let taps = crate::effects::gaussian_weights(*blur_before_difference);
                    let r = taps.len() / 2;
                    let soft = self.gauss(
                        steps,
                        bloom,
                        "D-501 shot",
                        still,
                        (w, h),
                        [&taps, &taps],
                        r,
                        false,
                    );
                    let soft_plate = self.gauss(
                        steps,
                        bloom,
                        "D-501 plate",
                        &plate,
                        (map.width(), map.height()),
                        [&taps, &taps],
                        r,
                        false,
                    );
                    let shares = self.scratch("D-501 shares", w, h);
                    let p = FxParams {
                        r: r as i32,
                        ox: ox as i32 - r as i32,
                        oy: oy as i32 - r as i32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.dmatte,
                        p,
                        Some(&soft),
                        Some(&shares),
                        Some(&k),
                        Some(&soft_plate),
                        none,
                        tiles(w, h),
                    );
                    let p = FxParams {
                        mode: 2,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.dmatte,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&k),
                        Some(&shares),
                        none,
                        tiles(w, h),
                    );
                } else {
                    let p = FxParams {
                        mode: 1,
                        ox: ox as i32,
                        oy: oy as i32,
                        ..Default::default()
                    };
                    self.fx_step(
                        steps,
                        &passes.dmatte,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&k),
                        Some(&plate),
                        none,
                        tiles(w, h),
                    );
                }
                (out, (w, h))
            }
            // D-531: compose leaves the card a CC Environment only with its environment read.
            e @ E::CcEnvironment { map, .. } => {
                let m = &map
                    .as_ref()
                    .expect("compose leaves the card a CC Environment whose environment was read")
                    .0;
                let k = crate::layer_fx::environment_numbers(
                    e,
                    (w - 2 * ox) as f64,
                    (h - 2 * oy) as f64,
                    (m.width(), m.height()),
                );
                let env = if m.width() == 0 || m.height() == 0 {
                    self.map_texture(&crate::WorkingBuffer::transparent(1, 1))
                } else {
                    self.map_texture(m)
                };
                let out = self.scratch("D-531 environment", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.environment,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&env),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            e @ E::CardWipe { back_map, .. } => {
                let k =
                    crate::layer_fx::card_wipe_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let back = match back_map {
                    Some(m) if m.0.width() > 0 && m.0.height() > 0 => self.map_texture(&m.0),
                    _ => self.map_texture(&crate::WorkingBuffer::transparent(1, 1)),
                };
                let out = self.scratch("D-580 cardwipe", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.cardwipe,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&back),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            e @ E::CcSphere { .. } => {
                let k =
                    crate::layer_fx::cylinder_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let out = self.scratch("D-532 sphere", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.sphere,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            e @ E::CcCylinder { .. } => {
                let k =
                    crate::layer_fx::cylinder_numbers(e, (w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let out = self.scratch("D-530 cylinder", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.cylinder,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-519: compose leaves the card a CC Spotlight only when any gel named was read.
            e @ E::CcSpotlight { .. } => {
                let gel = crate::layer_fx::spot_gel(e)
                    .expect("compose leaves a CC Spotlight whose gel was read");
                let k = crate::layer_fx::spotlight_numbers(
                    e,
                    gel.is_some(),
                    (w - 2 * ox) as f64,
                    (h - 2 * oy) as f64,
                );
                let gel = gel.map(|m| self.map_texture(m));
                let out = self.scratch("D-507 spotlight", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.spotlight,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(gel.as_ref().unwrap_or(still)),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-518: compose leaves the card a Warp Blend only when it has begun and any layer named
            // was read.
            e @ E::WarpBlend { .. } => {
                let g = crate::layer_fx::warp_settings(e)
                    .expect("compose leaves a Warp Blend whose layer was read");
                let reveal = g.reveal.map(|m| self.map_texture(m));
                let raw = self.scratch("D-518 reactor", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let k = [g.reactor as f64, reveal.is_some() as u8 as f64];
                self.fx_step(
                    steps,
                    &passes.wreact,
                    p,
                    Some(still),
                    Some(&raw),
                    Some(&k),
                    Some(reveal.as_ref().unwrap_or(still)),
                    none,
                    tiles(w, h),
                );
                let bump = if g.smoothness > 0.0 {
                    let bloom = self
                        .bloom
                        .as_ref()
                        .expect("a blur is refused without the passes");
                    let taps = crate::effects::gaussian_weights(g.smoothness / 2.0);
                    self.gauss(
                        steps,
                        bloom,
                        "D-518 reactor",
                        &raw,
                        (w, h),
                        [&taps, &taps],
                        0,
                        true,
                    )
                } else {
                    raw
                };
                let sums = buffer(w * h * 32);
                let work = [None, None, Some(&sums), None];
                // Glass Melt's slope pass; its share (k[2], k[3]) is not read here.
                let k = [g.amount, 1.25 * g.smoothness.max(1.0), 0.0, 1.0];
                self.fx_step(
                    steps,
                    &passes.gmeltk,
                    FxParams::default(),
                    Some(&bump),
                    None,
                    Some(&k),
                    None,
                    work,
                    tiles(w, h),
                );
                let out = self.scratch("D-518 warp", w, h);
                let c = g.completion / 100.0;
                let k = [
                    c,
                    g.direction as f64,
                    crate::layer_fx::blend_share(c, g.span),
                    reveal.is_some() as u8 as f64,
                ];
                self.fx_step(
                    steps,
                    &passes.warpb,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(reveal.as_ref().unwrap_or(still)),
                    work,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-517: compose leaves the card a Twister only when it turns and any back named was
            // read.
            e @ E::Twister { .. } => {
                let back = crate::layer_fx::twist_back(e)
                    .expect("compose leaves a Twister whose back was read");
                let k = crate::layer_fx::twist_numbers(
                    e,
                    back.is_some(),
                    (w - 2 * ox) as f64,
                    (h - 2 * oy) as f64,
                );
                let back = back.map(|m| self.map_texture(m));
                let out = self.scratch("D-517 twist", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.twist,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(back.as_ref().unwrap_or(still)),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-509: compose leaves the card a Glass Melt only when it melts and every layer named
            // was read.
            e @ E::GlassMelt { .. } => {
                let g = crate::layer_fx::melt_settings(e)
                    .expect("compose leaves a Glass Melt whose layers were read");
                let (picture, (mx, my)) = match g.gradient {
                    Some(m) => (self.map_texture(m), (ox, oy)),
                    None => (still.clone(), (0, 0)),
                };
                let luminance = crate::effects::VECTOR_BLUR_PROPERTIES
                    .iter()
                    .position(|p| *p == "luminance")
                    .expect("luminance is a phase");
                let raw = self.scratch("D-509 bump", w, h);
                let p = FxParams {
                    ox: mx as i32,
                    oy: my as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.vheight,
                    p,
                    Some(&picture),
                    Some(&raw),
                    Some(&[luminance as f64]),
                    None,
                    none,
                    tiles(w, h),
                );
                let bump = if g.softness > 0.0 {
                    let bloom = self
                        .bloom
                        .as_ref()
                        .expect("a blur is refused without the passes");
                    let taps = crate::effects::gaussian_weights(g.softness / 2.0);
                    self.gauss(
                        steps,
                        bloom,
                        "D-509 bump",
                        &raw,
                        (w, h),
                        [&taps, &taps],
                        0,
                        true,
                    )
                } else {
                    raw
                };
                let sums = buffer(w * h * 32);
                let work = [None, None, Some(&sums), None];
                let edge = -crate::layer_fx::MELT_BAND / 2.0
                    + g.completion / 100.0 * (1.0 + crate::layer_fx::MELT_BAND);
                let k = [
                    g.amount,
                    1.25 * g.softness.max(1.0),
                    edge,
                    crate::layer_fx::MELT_BAND,
                ];
                self.fx_step(
                    steps,
                    &passes.gmeltk,
                    FxParams::default(),
                    Some(&bump),
                    None,
                    Some(&k),
                    None,
                    work,
                    tiles(w, h),
                );
                let reveal = g.reveal.map(|m| self.map_texture(m));
                let out = self.scratch("D-509 melt", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let k = [reveal.is_some() as u8 as f64];
                self.fx_step(
                    steps,
                    &passes.gmelt,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(reveal.as_ref().unwrap_or(still)),
                    work,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-506: compose leaves the card a 3D Glasses only when every view named was read.
            e @ E::ThreeDGlasses { .. } => {
                let g = crate::layer_fx::glasses_settings(e)
                    .expect("compose leaves a 3D Glasses whose views were read");
                let (w0, h0) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let (c, v) = if g.percent {
                    (g.convergence / 100.0 * w0, g.alignment / 100.0 * h0)
                } else {
                    (g.convergence, g.alignment)
                };
                let maps = g.maps.map(|m| m.map(|m| self.map_texture(m)));
                let corner = |m: &Option<wgpu::TextureView>| {
                    if m.is_some() {
                        [0.0, 0.0]
                    } else {
                        [ox as f64, oy as f64]
                    }
                };
                let [l, r] = [corner(&maps[0]), corner(&maps[1])];
                let k = [
                    g.view as f64,
                    c,
                    v,
                    w0,
                    h0,
                    g.balance,
                    l[0],
                    l[1],
                    r[0],
                    r[1],
                ];
                let out = self.scratch("D-506 glasses", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                let [left, right] = [
                    maps[0].as_ref().unwrap_or(still),
                    maps[1].as_ref().unwrap_or(still),
                ];
                self.fx_step(
                    steps,
                    &passes.glasses,
                    p,
                    Some(left),
                    Some(&out),
                    Some(&k),
                    Some(right),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            E::PassExtract {
                channels,
                black_point,
                white_point,
                invert,
                clamp,
                unmult,
                ..
            } => {
                let pass = self.map_texture(
                    &channels
                        .as_ref()
                        .expect("compose leaves a Pass Extract with a pass")
                        .0,
                );
                let on = |w: &String| (w == "on") as u8 as f64;
                let k = [
                    *black_point,
                    *white_point,
                    on(invert),
                    on(clamp),
                    on(unmult),
                ];
                let out = self.scratch("B-228 pass", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.passx,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&pass),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            E::DepthKey {
                channels,
                depth,
                feather,
                invert,
            } => {
                let pass = self.map_texture(
                    &channels
                        .as_ref()
                        .expect("compose leaves a Depth Key with a pass")
                        .0,
                );
                let k = [*depth, *feather, (invert == "on") as u8 as f64];
                let out = self.scratch("B-228 key", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.dkey,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&pass),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-418: the covering worked out as layer_fx::fill_cover works it, reaching the
            // feathers' radius past the layer, sent once, feathered with Bloom's `gauss` (its
            // edges held, the shorter taps padded with zeros), then laid on.
            E::Fill {
                color,
                invert,
                horizontal_feather,
                vertical_feather,
                opacity,
                paths,
                ..
            } => {
                let paths = paths
                    .as_ref()
                    .expect("compose leaves a Fill with its paths");
                let c = linear(color);
                let k = [
                    c[0],
                    c[1],
                    c[2],
                    opacity / 100.0,
                    (invert == "on") as u8 as f64,
                    paths.is_empty() as u8 as f64,
                ];
                let ([tx, ty], r) =
                    crate::layer_fx::fill_taps([*horizontal_feather, *vertical_feather]);
                let (fw, fh) = (w + 2 * r, h + 2 * r);
                let cover = if paths.is_empty() {
                    vec![0.0f32]
                } else {
                    crate::layer_fx::fill_cover(paths, f.origin, (w, h), r)
                };
                let cover = self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("B-297 covering"),
                        contents: bytemuck::cast_slice(&cover),
                        usage: wgpu::BufferUsages::STORAGE,
                    });
                let field = (!paths.is_empty()).then(|| {
                    let field = self.scratch("B-297 covering", fw, fh);
                    self.fx_step(
                        steps,
                        &passes.fill,
                        FxParams::default(),
                        Some(still),
                        Some(&field),
                        Some(&k),
                        Some(still),
                        [Some(&cover)],
                        tiles(fw, fh),
                    );
                    if r == 0 {
                        return field;
                    }
                    let pad = |t: Vec<f32>| {
                        let mut p = vec![0.0f32; r - t.len() / 2];
                        p.extend(&t);
                        p.resize(2 * r + 1, 0.0);
                        p
                    };
                    let bloom = self
                        .bloom
                        .as_ref()
                        .expect("a blur is refused without the passes");
                    self.gauss(
                        steps,
                        bloom,
                        "B-297 feather",
                        &field,
                        (fw, fh),
                        [&pad(tx), &pad(ty)],
                        0,
                        true,
                    )
                });
                let out = self.scratch("B-297 fill", w, h);
                let p = FxParams {
                    mode: 1,
                    g: r as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.fill,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(field.as_ref().unwrap_or(still)),
                    [Some(&cover)],
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-419: the area's rows found as layer_fx::eyedropper_rows finds them, each summed
            // by a thread, added in order by one, then every pixel filled; no warp mode.
            E::EyedropperFill {
                sample_point,
                sample_radius,
                average_pixel_colors,
                maintain_original_alpha,
                blend_with_original,
            } => {
                let point = crate::effects::radial_center(*sample_point, (w, h), f.origin);
                let (n, rows) = crate::layer_fx::eyedropper_rows(point, *sample_radius, (w, h));
                let mode = crate::effects::EYEDROPPER_AVERAGES
                    .iter()
                    .position(|a| a == average_pixel_colors)
                    .unwrap_or(0);
                let mut k = vec![
                    n,
                    mode as f64,
                    (maintain_original_alpha == "on") as u8 as f64,
                    blend_with_original / 100.0,
                ];
                k.extend(rows.iter().flat_map(|r| r.map(|v| v as f64)));
                let sums = buffer((2 * rows.len() + 1) * 32);
                let p = FxParams {
                    count: rows.len() as u32,
                    ..Default::default()
                };
                let work = [None, None, Some(&sums)];
                self.fx_step(
                    steps,
                    &passes.eyerow,
                    p,
                    Some(still),
                    None,
                    Some(&k),
                    None,
                    work,
                    ((rows.len() as u32).div_ceil(64).max(1), 1),
                );
                self.fx_step(
                    steps,
                    &passes.eyesum,
                    p,
                    None,
                    None,
                    Some(&k),
                    None,
                    work,
                    (1, 1),
                );
                let out = self.scratch("B-298 eyedropper fill", w, h);
                self.fx_step(
                    steps,
                    &passes.eyefill,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    work,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-440: the area found on the processor from the drawing compose begins the run with
            // (layer_fx::bucket_area: the flood fill and the distances), sent once, blurred with
            // Bloom's `gauss` (edges held), then laid on; no warp mode.
            E::PaintBucket {
                fill_point,
                fill_selector,
                tolerance,
                view_threshold,
                stroke,
                invert_fill,
                spread_radius,
                stroke_width,
                feather_softness,
                color,
                opacity,
                blending_mode,
            } => {
                let b = crate::layer_fx::Bucket {
                    point: crate::effects::radial_center(*fill_point, (w, h), f.origin),
                    selector: fill_selector,
                    tolerance: tolerance / 100.0,
                    invert: invert_fill == "on",
                    stroke,
                    radius: *spread_radius,
                    width: *stroke_width,
                    softness: *feather_softness,
                };
                let view = view_threshold == "on";
                let area: Vec<f32> = match view {
                    true => crate::layer_fx::bucket_matches(source, &b).map_or_else(
                        || vec![0.0; w * h],
                        |(ok, _)| ok.into_iter().map(|g| g as u8 as f32).collect(),
                    ),
                    false => crate::layer_fx::bucket_area(source, &b),
                };
                let cover = self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("B-320 area"),
                        contents: bytemuck::cast_slice(&area),
                        usage: wgpu::BufferUsages::STORAGE,
                    });
                let field = self.scratch("B-320 area", w, h);
                self.fx_step(
                    steps,
                    &passes.fill,
                    FxParams::default(),
                    Some(still),
                    Some(&field),
                    Some(&[0.0]),
                    Some(still),
                    [Some(&cover)],
                    tiles(w, h),
                );
                let taps = crate::layer_fx::bucket_taps(&b);
                let field = if !view && taps.len() > 1 {
                    let bloom = self
                        .bloom
                        .as_ref()
                        .expect("a blur is refused without the passes");
                    self.gauss(
                        steps,
                        bloom,
                        "B-320 blur",
                        &field,
                        (w, h),
                        [&taps, &taps],
                        0,
                        true,
                    )
                } else {
                    field
                };
                let c = linear(color);
                let mode = if blending_mode == "fill_only" {
                    "none"
                } else {
                    blending_mode.as_str()
                };
                let k = [
                    c[0],
                    c[1],
                    c[2],
                    opacity / 100.0,
                    laid(mode),
                    (fill_selector == "straight_color") as u8 as f64,
                    view as u8 as f64,
                ];
                let out = self.scratch("B-320 paint bucket", w, h);
                self.fx_step(
                    steps,
                    &passes.bucket,
                    FxParams::default(),
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&field),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // B-229: the matte at the ids' size, blurred with its edges held, then laid on.
            E::IdKey {
                channels,
                id,
                feather,
                invert,
                ..
            } => {
                let ids = &channels
                    .as_ref()
                    .expect("compose leaves an ID Key with ids")
                    .0;
                let (pw, ph) = (ids.width(), ids.height());
                let pass = self.map_texture(ids);
                let matte = self.scratch("B-229 matte", pw, ph);
                let mut k = [*id, (invert == "on") as u8 as f64, 0.0];
                self.fx_step(
                    steps,
                    &passes.idkey,
                    FxParams::default(),
                    Some(&pass),
                    Some(&matte),
                    Some(&k),
                    Some(&pass),
                    none,
                    tiles(pw, ph),
                );
                let taps = crate::effects::gaussian_weights(*feather);
                let matte = if taps.len() > 1 {
                    let bloom = self
                        .bloom
                        .as_ref()
                        .expect("a blur is refused without the passes");
                    self.gauss(
                        steps,
                        bloom,
                        "B-229 feather",
                        &matte,
                        (pw, ph),
                        [&taps, &taps],
                        0,
                        true,
                    )
                } else {
                    matte
                };
                k[2] = 1.0;
                let out = self.scratch("B-229 key", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.idkey,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&matte),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-527: the box and the words drawn on the processor, in the bundled font, laid over
            // the drawing or alone.
            e @ E::Timecode {
                composite_on_original,
                ..
            } => {
                let words = crate::layer_fx::timecode_picture(e, (w, h), (ox, oy))
                    .expect("the bundled font is always there");
                let words = self.map_texture(&words);
                let out = self.scratch("D-527 timecode", w, h);
                let k = [(composite_on_original == "on") as u8 as f64];
                self.fx_step(
                    steps,
                    &passes.over,
                    FxParams::default(),
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&words),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-473: as Numbers' words, drawn on the processor.
            e @ E::BasicText {
                composite_on_original,
                ..
            } => {
                let words = crate::layer_fx::basic_text_picture(e, (w, h), (ox, oy))
                    .expect("compose leaves Basic Text a font it can draw in");
                let words = self.map_texture(&words);
                let out = self.scratch("D-473 basic text", w, h);
                let k = [(composite_on_original == "on") as u8 as f64];
                self.fx_step(
                    steps,
                    &passes.over,
                    FxParams::default(),
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&words),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-526: the words drawn on the processor, as compose leaves Numbers only a font it can
            // draw in, laid over the drawing or alone.
            e @ E::Numbers {
                composite_on_original,
                ..
            } => {
                let words = crate::layer_fx::numbers_picture(e, (w, h), (ox, oy))
                    .expect("compose leaves Numbers a font it can draw in");
                let words = self.map_texture(&words);
                let out = self.scratch("D-526 numbers", w, h);
                let k = [(composite_on_original == "on") as u8 as f64];
                self.fx_step(
                    steps,
                    &passes.over,
                    FxParams::default(),
                    Some(still),
                    Some(&out),
                    Some(&k),
                    Some(&words),
                    none,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            // D-525: the picture worked on the processor at the ids' size, then laid on as
            // Cryptomatte's is: the alpha matte as ID Key's matte, the rest as Pass Extract's pass.
            E::Identifier {
                channels,
                id,
                output,
                ..
            } => {
                let ids = &channels
                    .as_ref()
                    .expect("compose leaves an IDentifier with ids")
                    .0;
                let pass = self.map_texture(&crate::layer_fx::identifier(ids, *id, output));
                let out = self.scratch("D-525 identifier", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                if output == "alpha_matte" {
                    self.fx_step(
                        steps,
                        &passes.idkey,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&[0.0, 0.0, 1.0]),
                        Some(&pass),
                        none,
                        tiles(w, h),
                    );
                } else {
                    self.fx_step(
                        steps,
                        &passes.passx,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&[0.0, 1.0, 0.0, 0.0]),
                        Some(&pass),
                        none,
                        tiles(w, h),
                    );
                }
                (out, (w, h))
            }
            // D-522: the colours or the matte as Pass Extract's pass at 0 to 1 unclamped; keyed,
            // the drawing times the matte as ID Key lays its matte on.
            E::Cryptomatte {
                channels, output, ..
            } => {
                let pass = self.map_texture(
                    &channels
                        .as_ref()
                        .expect("compose leaves a Cryptomatte with its matte")
                        .0,
                );
                let out = self.scratch("B-402 cryptomatte", w, h);
                let p = FxParams {
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                if output == "keyed" {
                    self.fx_step(
                        steps,
                        &passes.idkey,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&[0.0, 0.0, 1.0]),
                        Some(&pass),
                        none,
                        tiles(w, h),
                    );
                } else {
                    self.fx_step(
                        steps,
                        &passes.passx,
                        p,
                        Some(still),
                        Some(&out),
                        Some(&[0.0, 1.0, 0.0, 0.0]),
                        Some(&pass),
                        none,
                        tiles(w, h),
                    );
                }
                (out, (w, h))
            }
            E::LineWidth {
                width,
                based_on,
                colors,
                tolerance,
            } => {
                let shape = based_on == "shape";
                let disc = crate::line_width::disc(*width);
                let mode = if !shape {
                    2.0
                } else if *width > 0.0 {
                    0.0
                } else {
                    1.0
                };
                let mut k = vec![mode, (*width > 0.0) as u8 as f64, disc.len() as f64];
                k.extend(disc.iter().flat_map(|&(dx, dy)| [dx as f64, dy as f64]));
                let targets = crate::selective_blur::targets(colors);
                let picked: Vec<f32> = match shape {
                    true => vec![0.0],
                    false => source
                        .data()
                        .par_chunks_exact(4)
                        .map(|p| {
                            crate::selective_blur::chosen(p, &targets, *tolerance) as u8 as f32
                        })
                        .collect(),
                };
                let picked = self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("B-226 chosen"),
                        contents: bytemuck::cast_slice(&picked),
                        usage: wgpu::BufferUsages::STORAGE,
                    });
                let g = f.grow.0;
                let (tw, th) = (w + 2 * g, h + 2 * g);
                let out = self.scratch("B-226 width", tw, th);
                let p = FxParams {
                    g: g as i32,
                    ..Default::default()
                };
                self.fx_step(
                    steps,
                    &passes.lwidth,
                    p,
                    Some(still),
                    Some(&out),
                    Some(&k),
                    None,
                    [Some(&picked)],
                    tiles(tw, th),
                );
                (out, (tw, th))
            }
            E::LineSmooth {
                softness,
                threshold,
            } => {
                let encoded: Vec<[f32; 4]> = source
                    .data()
                    .par_chunks_exact(4)
                    .map(crate::line_smooth::encoded)
                    .collect();
                let band = self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("B-226 encoded"),
                        contents: bytemuck::cast_slice(&encoded),
                        usage: wgpu::BufferUsages::STORAGE,
                    });
                let mixes = buffer(w * h * 32);
                let k = [50.0 / softness, (threshold / 255.0) as f32 as f64];
                let lines = (w + h - 2) as u32;
                let work = [Some(&mixes), Some(&band)];
                self.fx_step(
                    steps,
                    &passes.smoothscan,
                    FxParams::default(),
                    Some(still),
                    None,
                    Some(&k),
                    None,
                    work,
                    (lines.div_ceil(64).max(1), 1),
                );
                let out = self.scratch("B-226 smooth", w, h);
                self.fx_step(
                    steps,
                    &passes.smoothmix,
                    FxParams::default(),
                    Some(still),
                    Some(&out),
                    None,
                    None,
                    work,
                    tiles(w, h),
                );
                (out, (w, h))
            }
            not_fx!() => unreachable!(
                "compose hands the card only the effects `fx` draws; not_fx! lists the rest"
            ),
        }
    }

    /// B-51: `glow::glow` of `still`, `width` by `height`: the light in Bloom's bright pass, blurred
    /// in its gauss pass, and laid on the drawing by its combine pass, into a texture grown by the
    /// blur's radius, which is returned with its size.
    fn glow(
        &self,
        steps: &mut Vec<Step>,
        still: &wgpu::TextureView,
        (w, h): (usize, usize),
        g: Glow,
    ) -> (wgpu::TextureView, (usize, usize)) {
        let passes = self
            .bloom
            .as_ref()
            .expect("a Glow is refused without the passes");
        let (sigma, long) = crate::effects::glow_reach(
            g.radius,
            if g.after_effects {
                "after_effects"
            } else {
                "classic"
            },
        );
        let r = crate::effects::reach_radius(sigma, long);
        let (gw, gh) = (w + 2 * r, h + 2 * r);
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        let init = |label, contents: &[f32]| {
            self.device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some(label),
                    contents: bytemuck::cast_slice(contents),
                    usage: wgpu::BufferUsages::STORAGE,
                })
        };
        // The colours, then the tint in linear light, as the bright pass reads them.
        let mut colours: Vec<f32> = g.targets[..g.count]
            .iter()
            .flatten()
            .map(|&v| v as f32)
            .collect();
        colours.extend(g.tint.map_or([0.0; 3], |t| {
            t.map(|v| crate::color::srgb_to_linear(v as f32 / 255.0))
        }));
        let light = self.scratch("B-51 light", w, h);
        let p = Params {
            // 256 lets nothing through: a Glow on chosen colours has no brightness test.
            level: if g.bright {
                crate::bloom::bright_level(g.threshold)
            } else {
                256
            },
            count: if g.bright { 0 } else { g.count as u32 },
            // The levels are whole numbers, so within a tolerance is within its whole part.
            tolerance: g.tolerance as u32,
            tinted: g.tint.is_some() as u32,
            ..Default::default()
        };
        self.step(
            steps,
            &passes.bright,
            p,
            still,
            Some(&light),
            None,
            Some(&init("B-51 colours", &colours)),
            tiles(w, h),
        );
        // Made empty: wgpu clears every buffer it makes.
        let halo = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("B-51 halo"),
            size: (gw * gh * 16) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let add = Params {
            width: gw as u32,
            height: gh as u32,
            weight: 1.0,
            ..Default::default()
        };
        if r == 0 {
            self.step(
                steps,
                &passes.add,
                add,
                &light,
                None,
                Some(&halo),
                None,
                tiles(gw, gh),
            );
        } else {
            let tall = self.blurred(steps, passes, "B-51", &light, (w, h), sigma, long);
            self.step(
                steps,
                &passes.add,
                add,
                &tall,
                None,
                Some(&halo),
                None,
                tiles(gw, gh),
            );
        }
        let out = self.scratch("B-51 glow", gw, gh);
        // D-331: After Effects' colour at the intensity, the covering over t + 16 (1 - t).
        let t = g.threshold / 100.0;
        let cover = 1.0 / (t + 16.0 * (1.0 - t));
        let p = Params {
            g: r as i32,
            weight: g.intensity as f32,
            screen: g.screen as u32,
            held: g.after_effects as u32,
            cover: cover as f32,
            ..Default::default()
        };
        self.step(
            steps,
            &passes.combine,
            p,
            still,
            Some(&out),
            Some(&halo),
            None,
            tiles(gw, gh),
        );
        (out, (gw, gh))
    }

    fn target(&mut self, width: usize, height: usize) -> &Target {
        if self
            .target
            .as_ref()
            .is_none_or(|t| (t.width, t.height) != (width, height))
        {
            let n = (width * height) as u64;
            let buffer = |label, size, usage| {
                self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some(label),
                    size,
                    usage,
                    mapped_at_creation: false,
                })
            };
            use wgpu::BufferUsages as U;
            self.target = Some(Target {
                width,
                height,
                sum: buffer("B-44 sum", n * 16, U::STORAGE | U::COPY_DST),
                bytes: buffer("B-44 bytes", n * 4, U::STORAGE | U::COPY_SRC | U::COPY_DST),
                readback: buffer("B-44 readback", n * 4, U::MAP_READ | U::COPY_DST),
                size: self
                    .device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some("B-44 size"),
                        contents: bytemuck::cast_slice(&[width as u32, height as u32, 0, 0]),
                        usage: U::UNIFORM,
                    }),
            });
        }
        self.target.as_ref().expect("made above")
    }

    /// Why this plan cannot go to the card, if it cannot.
    fn refuse(&self, plan: &FramePlan) -> Option<Diagnostic> {
        // D-319: Float working depth is the CPU's until the card's blends and noise match it.
        if plan.float {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: its composition works in Float depth.".into(),
                "D-319's Float working depth (Add, Screen and Fractal Noise past white) is drawn on the CPU only.".into(),
            ));
        }
        // B-156 (D-225): an adjustment layer whose every effect the card draws on the frame.
        let frame = (plan.width, plan.height);
        if plan
            .layers
            .iter()
            .filter_map(|l| l.adjust.as_ref())
            .any(|s| self.fx.is_none() || crate::compose::adjust_run(s, frame).is_none())
        {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: its adjustment layer has an effect the GPU does not draw there.".into(),
                "B-156 draws an adjustment layer (D-66) on the card when the card draws each of its effects on the frame (D-225); Bloom, Glow, Paraffin, Kira-kira, HSV Key, Colour Key, Select Colour and Line Recolour stay the CPU's.".into(),
            ));
        }
        // D-301: the four modes after add are the CPU's.
        // ponytail: a frame with one draws on the CPU; give the layer shader them if it is slow.
        if plan.layers.iter().any(|l| {
            !matches!(
                l.blend,
                BlendMode::Normal | BlendMode::Multiply | BlendMode::Screen | BlendMode::Add
            )
        }) {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: a layer has a blend mode the GPU does not draw.".into(),
                "D-301's Overlay, Soft Light, Stencil Alpha and Stencil Luma are drawn on the CPU only.".into(),
            ));
        }
        // D-297: an adjustment layer in a blend mode other than normal is the CPU's.
        if plan
            .layers
            .iter()
            .any(|l| l.adjust.is_some() && l.blend != crate::model::BlendMode::Normal)
        {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: its adjustment layer has a blend mode other than normal.".into(),
                "D-297 lays an adjusted frame on the one beneath in its blend mode on the CPU only.".into(),
            ));
        }
        // B-152: a motion-blurred, frame-mixed or dissolved layer arrives already built by the
        // CPU, and the card lays it like any drawing. B-153b: but sending that new picture costs
        // the card more than the CPU takes to draw the frame whole, so it does so unless the card
        // has an effect of its own to draw (D-218).
        if plan.layers.iter().any(|l| l.motion_blur || l.mixed)
            && !plan
                .layers
                .iter()
                .any(|l| !l.on_card.is_empty() || !wraps(l).is_empty() || l.adjust.is_some())
        {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: it has motion blur or frame blending and no effect for the GPU, which the CPU draws faster.".into(),
                "B-153b hands a blurred or mixed frame (D-218) with no card effect to the CPU whole; measured faster there.".into(),
            ));
        }
        if plan
            .layers
            .iter()
            .flat_map(|l| &l.wrap)
            .any(|i| i.enabled && i.is_valid() && i.mix < 100.0)
        {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: it has a Light Wrap with a Mix below 100, which the GPU does not draw yet.".into(),
                "B-137b draws a mixed Light Wrap (D-202) on the CPU; a card version is a later unit.".into(),
            ));
        }
        // B-76: a Light Wrap blurs the frame beneath, grown by the blur's radius.
        let radii: Vec<usize> = plan
            .layers
            .iter()
            .flat_map(wraps)
            .map(|(width, _, _)| crate::effects::kernel_radius(width / 3.0))
            .collect();
        if let Some(&r) = radii.iter().max() {
            if self.bloom.is_none() || self.fx.is_none() {
                return Some(on_cpu(
                    Severity::Info,
                    "The CPU drew this frame: it has a Light Wrap, and this graphics card cannot do the double-precision sums it needs.".into(),
                    format!("{}: no SHADER_F64. B-76 draws a Light Wrap on the card only where it can add as the CPU does.", self.about),
                ));
            }
            let (gw, gh) = (plan.width + 2 * r, plan.height + 2 * r);
            if gw.max(gh) > self.limits.max_texture_dimension_2d as usize {
                return Some(on_cpu(
                    Severity::Info,
                    format!("The CPU drew this frame: a Light Wrap blurs the frame to {gw} by {gh}, larger than the card allows."),
                    format!("{}: largest texture side {}.", self.about, self.limits.max_texture_dimension_2d),
                ));
            }
        }
        // B-47: a Bloom needs double precision, and room for its grown drawing, its halo and
        // its lines, which are never more than the grown width and height together. B-49: so
        // does a Directional Blur, which has no halo, (B-50) a Gaussian Blur, whose pass is
        // in the same module, and (B-51) a Glow, which has a halo.
        // B-65: so does the batch of ten; Lens Blur keeps its running totals and Outline its band
        // in a buffer.
        for l in &plan.layers {
            // B-155: each effect of a run on the drawing the ones before it grew. B-156: an
            // adjustment layer's on the frame.
            let (run, (mut w, mut h)) = match &l.adjust {
                Some(stack) => (
                    std::borrow::Cow::Owned(
                        crate::compose::adjust_run(stack, frame).unwrap_or_default(),
                    ),
                    frame,
                ),
                None => (
                    std::borrow::Cow::Borrowed(&l.on_card[..]),
                    (l.source.width(), l.source.height()),
                ),
            };
            for card in run.iter() {
                let halo = |g: usize| ((w + 2 * g) * (h + 2 * g) * 16) as u64;
                let (name, grow, bytes) = match card.unmixed() {
                    OnCard::Bloom(b) => {
                        let g = crate::bloom::reach(b.radius, b.lines, b.length);
                        ("a Bloom".to_string(), (g, g), halo(g))
                    }
                    OnCard::Directional(d) => {
                        ("a Directional Blur".into(), (d.grow(), d.grow()), 0)
                    }
                    OnCard::Gaussian(g) => ("a Gaussian Blur".into(), (g.grow(), g.grow()), 0),
                    OnCard::Glow(g) => {
                        let g = crate::effects::kernel_radius(g.radius / 3.0);
                        ("a Glow".into(), (g, g), halo(g))
                    }
                    OnCard::Fx(f) => {
                        let bytes = match &f.instance.effect {
                            crate::effects::Effect::LensBlur { .. }
                            | crate::effects::Effect::DepthOfField { .. } => {
                                ((w + 1) * h * 32) as u64
                            }
                            crate::effects::Effect::Outline { width, .. } => {
                                let n = width.ceil() as usize;
                                ((w + 2 * n) * (h + 2 * n) * 4) as u64
                            }
                            crate::effects::Effect::DistanceGradation { .. } => {
                                dist_bytes(w, h) as u64
                            }
                            // B-107: a Simple Choker's band as Outline's, and a spread's running totals.
                            crate::effects::Effect::SimpleChoker { choke } => {
                                let g = if *choke < 0.0 {
                                    (-choke).floor() as usize
                                } else {
                                    0
                                };
                                let sums = if *choke < 0.0 { (w + 1) * h * 32 } else { 0 };
                                ((w + 2 * g) * (h + 2 * g) * 4).max(sums) as u64
                            }
                            // D-352: a Matte Choker's running totals.
                            crate::effects::Effect::MatteChoker { .. } => ((w + 1) * h * 32) as u64,
                            // Each block's mean.
                            crate::effects::Effect::Mosaic { size } => {
                                (16.0 * (w as f64 / size + 2.0) * (h as f64 / size + 2.0)) as u64
                            }
                            // B-123: a Color Lookup's table rides with its settings; Kira-kira keeps
                            // its light in a buffer the size of the grown drawing, and its stars, at
                            // most one a cell, with its settings.
                            crate::effects::Effect::ColorLookup { table: Some(t), .. } => {
                                ((8 + 3 * t.0.entries()) * 8) as u64
                            }
                            crate::effects::Effect::ArbitraryMap { table: Some(_), .. } => {
                                ((8 + 3 * 256) * 8) as u64
                            }
                            // D-397: a Color Grade's look the same, with its intensity.
                            crate::effects::Effect::ColorGrade { table: Some(t), .. } => {
                                ((9 + 3 * t.0.entries()) * 8) as u64
                            }
                            crate::effects::Effect::KiraKira { spacing, .. } => {
                                let cells = (w as f64 / spacing + 2.0) * (h as f64 / spacing + 2.0);
                                (((w + 2 * f.grow.0) * (h + 2 * f.grow.1) * 4) as f64)
                                    .max((19.0 + 4.0 * cells) * 8.0)
                                    as u64
                            }
                            // D-551: CC Ball Action's two balls a pixel, and its balls, at most one
                            // a cell, with its settings.
                            crate::effects::Effect::CcBallAction { grid_spacing, .. } => {
                                let cells = (w as f64 / grid_spacing.max(1.0))
                                    * (h as f64 / grid_spacing.max(1.0));
                                ((2 * w * h * 4) as f64).max((6.0 + 6.0 * cells) * 8.0) as u64
                            }
                            // D-610: Particle Space's sprites, ten numbers each and a place in a
                            // tile's list or several, and its tiles' starts.
                            // ponytail: reckons a sprite for each particle Birth Rate and
                            // Longevity keep alive now; a keyed rate can run above it.
                            crate::effects::Effect::ParticleSpace { values, .. } => {
                                let alive = 100.0 * values[0] * values[1] + 1.0;
                                ((7.0 + 14.0 * alive + (w * h) as f64 / 256.0 + 1.0) * 8.0) as u64
                            }
                            // D-671: Bubbles' bubbles, eleven numbers each and a place in a tile's
                            // list or several, and its tiles' starts.
                            crate::effects::Effect::Bubbles { values, .. } => {
                                let n = values[0].clamp(0.0, 10000.0).floor();
                                let tiles = ((w / 16 + 1) * (h / 16 + 1)) as f64;
                                let r = values[4].clamp(0.0, 100.0) * 1.5 * h as f64 / 40.0;
                                let reach = ((2.0 * r + 4.0) / 16.0 + 2.0).powi(2).min(tiles);
                                ((7.0
                                    + n * (crate::particles::STRIDE as f64 + reach)
                                    + tiles
                                    + 1.0)
                                    * 8.0) as u64
                            }
                            // D-672: Drizzle's live drops at most (the fastest rate times the
                            // in point's longevity), eleven numbers each and a place in a tile's list
                            // or several, its tiles' starts and the light.
                            crate::effects::Effect::Drizzle {
                                values, history, ..
                            } => {
                                let most = |i: usize| {
                                    history.as_deref().map_or(values[i], |hs| {
                                        hs.iter().fold(values[i], |m, v| m.max(v[i]))
                                    })
                                };
                                let rate = most(0).clamp(0.0, 1000.0);
                                let life = history
                                    .as_deref()
                                    .and_then(|hs| hs.first())
                                    .map_or(values[1], |v| v[1])
                                    .clamp(0.0, 100.0);
                                let n = (rate * life).ceil() + 2.0;
                                let tiles = ((w / 16 + 1) * (h / 16 + 1)) as f64;
                                let r = most(5).clamp(0.0, 1000.0) / 100.0 * h as f64 / 4.0;
                                let reach = ((2.0 * r + 4.0) / 16.0 + 2.0).powi(2).min(tiles);
                                ((7.0
                                    + n * (crate::particles::STRIDE as f64 + reach)
                                    + tiles
                                    + 1.0
                                    + 15.0)
                                    * 8.0) as u64
                            }
                            // D-612: Particle Plane's the same.
                            crate::effects::Effect::ParticlePlane { values, .. } => {
                                let alive = 100.0 * values[0] * values[1] + 1.0;
                                ((7.0 + 14.0 * alive + (w * h) as f64 / 256.0 + 1.0) * 8.0) as u64
                            }
                            // D-617: Particle Emitter's, as Particle Space's: a sprite for each
                            // particle the rate and the longest life keep alive now.
                            // D-711: with the second system, each throws Aux Particles/sec (or
                            // /Bounce) for Aux Life more.
                            // ponytail: one bounce a second assumed; a floor bouncing them faster
                            // can run above it.
                            crate::effects::Effect::ParticleEmitter { values, words, .. } => {
                                let thrown = if words[6] == "off" {
                                    0.0
                                } else {
                                    values[45].max(values[46]) * values[47]
                                };
                                let alive = values[0]
                                    * values[15]
                                    * (1.0 + values[16] / 100.0)
                                    * (1.0 + thrown)
                                    + 1.0;
                                ((7.0 + 14.0 * alive + (w * h) as f64 / 256.0 + 1.0) * 8.0) as u64
                            }
                            // D-619: Particle Playground's, a sprite for each particle the Cannon
                            // shoots by now, each point of the Grid and each piece of the Explode
                            // layer, and its two maps side by side (counted as sprites).
                            // ponytail: the Particle Exploder's children are not counted; a frame
                            // with many bursts can run above it.
                            crate::effects::Effect::ParticlePlayground {
                                values,
                                clock,
                                maps,
                                ..
                            } => {
                                let shot =
                                    values[1] * (clock[0].max(0.0) / clock[1].max(1.0) + 1.0);
                                let grid = values[9] * values[10];
                                let side = (2.0 * values[12]).max(1.0);
                                let px = |m: &Option<crate::layer_map::Map>| {
                                    m.as_ref()
                                        .map_or(0.0, |m| (m.0.width() * m.0.height()) as f64)
                                };
                                let pieces = px(&maps[0]) / (side * side);
                                let n = shot + grid + pieces + 1.0;
                                ((7.0 + 14.0 * n + (w * h) as f64 / 256.0 + 1.0) * 8.0
                                    + 16.0 * (px(&maps[0]) + px(&maps[1])))
                                    as u64
                            }
                            // D-618: Particle Grid's, a sprite for each point of its grid.
                            crate::effects::Effect::ParticleGrid { values, .. } => {
                                let points = values[3] * values[4] * values[5];
                                ((7.0 + 14.0 * points + (w * h) as f64 / 256.0 + 1.0) * 8.0) as u64
                            }
                            // D-673: Hair's strands, eleven numbers each and a place in a tile's
                            // list or several, its tiles' starts and the shared numbers.
                            crate::effects::Effect::Hair { values, .. } => {
                                let n = (values[3].clamp(0.0, 1000.0) / 100.0 * 4000.0 * w as f64
                                    / (h as f64).max(1.0))
                                .floor();
                                let tiles = ((w / 16 + 1) * (h / 16 + 1)) as f64;
                                let r = values[0].clamp(0.0, 1000.0) / 100.0 * h as f64 / 10.0
                                    * (1.0 + values[2].clamp(-500.0, 500.0).abs() / 100.0)
                                    + values[1].clamp(0.0, 100.0) * h as f64 / 2160.0
                                    + 1.0;
                                let reach = ((2.0 * r + 4.0) / 16.0 + 2.0).powi(2).min(tiles);
                                ((7.0
                                    + n * (crate::particles::STRIDE as f64 + reach)
                                    + tiles
                                    + 1.0
                                    + 15.0)
                                    * 8.0) as u64
                            }
                            // D-674: Mercury's live drops at most (the fastest rate times the
                            // in point's longevity), eleven numbers each and a place in a tile's list
                            // or several, its tiles' starts and the light.
                            crate::effects::Effect::Mercury {
                                values, history, ..
                            } => {
                                let most = |i: usize| {
                                    history.as_deref().map_or(values[i], |hs| {
                                        hs.iter().fold(values[i], |m, v| m.max(v[i]))
                                    })
                                };
                                let rate = most(0).clamp(0.0, 1000.0);
                                let life = history
                                    .as_deref()
                                    .and_then(|hs| hs.first())
                                    .map_or(values[1], |v| v[1])
                                    .clamp(0.0, 100.0);
                                let n = (rate * life).ceil() + 2.0;
                                let tiles = ((w / 16 + 1) * (h / 16 + 1)) as f64;
                                let r = most(12).max(most(13)).clamp(0.0, 1000.0) / 100.0
                                    * h as f64
                                    / 20.0
                                    * (1.0 + values[11].clamp(0.0, 400.0) / 100.0);
                                let reach = ((2.0 * r + 4.0) / 16.0 + 2.0).powi(2).min(tiles);
                                ((7.0
                                    + n * (crate::particles::STRIDE as f64 + reach)
                                    + tiles
                                    + 1.0
                                    + 15.0)
                                    * 8.0) as u64
                            }
                            // D-675: Polygon Shards' shards, a cell or two each, ten and sixteen
                            // numbers each and a place in a tile's list or several, its tiles'
                            // starts and the shared numbers.
                            // ponytail: a shard thrown towards the eye covers more tiles than
                            // this allows; count them from the list if the card runs short.
                            crate::effects::Effect::PolygonShards { values, words, .. } => {
                                let s = values[7].clamp(2.0, 1000.0);
                                let per = if words[0].ends_with("polygon") {
                                    2.0
                                } else {
                                    1.0
                                };
                                let n = (w as f64 / s).ceil() * (h as f64 / s).ceil() * per;
                                let tiles = ((w / 16 + 1) * (h / 16 + 1)) as f64;
                                let reach = ((2.0 * s + 8.0) / 16.0 + 2.0).powi(2).min(tiles);
                                ((7.0 + n * (26.0 + reach) + tiles + 1.0 + 7.0) * 8.0) as u64
                            }
                            // D-678: Foam's bubbles at most (the fastest rate over the longest
                            // life), eleven numbers each and a place in a tile's list or several, its
                            // tiles' starts and the shared numbers.
                            crate::effects::Effect::Foam {
                                values, history, ..
                            } => {
                                let most = |i: usize| {
                                    history.as_deref().map_or(values[i], |hs| {
                                        hs.iter().fold(values[i], |m, v| m.max(v[i]))
                                    })
                                };
                                let rate = 30.0 * most(5).clamp(0.0, 100.0);
                                let secs = most(8).clamp(1.0, 3000.0) / 30.0 + 1.0;
                                let n = (rate * secs).ceil() + 2.0;
                                let tiles = ((w / 16 + 1) * (h / 16 + 1)) as f64;
                                let r = most(6).clamp(0.0, 10.0) * 2.0 * h as f64 / 40.0
                                    * most(21).clamp(0.05, 50.0)
                                    * 2.0;
                                let reach = ((2.0 * r + 4.0) / 16.0 + 2.0).powi(2).min(tiles);
                                ((7.0
                                    + n * (crate::particles::STRIDE as f64 + reach)
                                    + tiles
                                    + 1.0
                                    + 12.0)
                                    * 8.0) as u64
                            }
                            // D-676: Scatterize's dots, one a pixel, three numbers each and a
                            // place in up to four tiles' lists, and its tiles' starts.
                            crate::effects::Effect::Scatterize { .. } => {
                                let tiles = ((w / 16 + 1) * (h / 16 + 1)) as f64;
                                ((7.0 + 7.0 * (w * h) as f64 + tiles + 1.0) * 8.0) as u64
                            }
                            // D-677: Star Burst's stars, one a grid point, STRIDE numbers each
                            // and a place in the lists of the tiles the nearest star could
                            // reach, and its tiles' starts.
                            crate::effects::Effect::StarBurst { values, view, .. } => {
                                let (wf, hf) = (w as f64, h as f64);
                                let g = values[3].clamp(0.5, 100.0) * hf / 270.0;
                                let n = (wf / g).ceil() * (hf / g).ceil();
                                let a = if view[0] == 1.0 { view[3] } else { 1.5 * hf };
                                let near = (a / (a - 1.35 * hf).max(hf / 100.0)).abs().max(1.0);
                                let r = values[4].clamp(0.0, 1000.0) / 100.0 * g / 4.0 * near + 1.0;
                                let tiles = ((w / 16 + 1) * (h / 16 + 1)) as f64;
                                let reach = ((2.0 * r + 4.0) / 16.0 + 2.0).powi(2).min(tiles);
                                ((7.0
                                    + n * (crate::particles::STRIDE as f64 + reach)
                                    + tiles
                                    + 1.0)
                                    * 8.0) as u64
                            }
                            // B-151: Cell Pattern's points ride with its settings.
                            crate::effects::Effect::CellPattern { size, .. } => {
                                ((15.0 + 3.0 * (w as f64 / size + 5.0) * (h as f64 / size + 5.0))
                                    * 8.0) as u64
                            }
                            // B-223: a Compound Blur's sigma for each pixel.
                            crate::effects::Effect::CompoundBlur { .. } => (w * h * 8) as u64,
                            // D-407: an Upscale's taps, seven numbers for each grown column and row.
                            crate::effects::Effect::DetailUpscale { .. } => {
                                ((1 + 7 * (w + 2 * f.grow.0 + h + 2 * f.grow.1)) * 8) as u64
                            }
                            // B-226: Line Smooth's mixes, eight a pixel; Block Dissolve's blocks, or
                            // its columns' running sums, with its settings.
                            crate::effects::Effect::LineSmooth { .. } => (w * h * 32) as u64,
                            crate::effects::Effect::BlockDissolve {
                                block_width,
                                block_height,
                                feather,
                                ..
                            } => {
                                let rows = (h as f64 + feather) / block_height + 3.0;
                                (8.0 * ((w as f64 + w as f64 / block_width + 3.0) * rows
                                    + (w + h + 7) as f64)) as u64
                            }
                            _ => 0,
                        };
                        (
                            format!("the effect {}", f.instance.effect.name()),
                            f.grow,
                            bytes,
                        )
                    }
                    OnCard::Radial(_) | OnCard::Mix(..) => continue,
                };
                let (gx, gy) = grow;
                if self.bloom.is_none() || self.fx.is_none() {
                    return Some(on_cpu(
                        Severity::Info,
                        format!("The CPU drew this frame: it has {name}, and this graphics card cannot do the double-precision sums it needs."),
                        format!("{}: no SHADER_F64. B-47 and B-49 draw these on the card only where it can add as the CPU does.", self.about),
                    ));
                }
                let (gw, gh) = (w + 2 * gx, h + 2 * gy);
                // B-223: a Channel Blur's colour can reach further than its alpha, which it grows by.
                let reach = match card.unmixed() {
                    OnCard::Fx(f) => match &f.instance.effect {
                        crate::effects::Effect::ChannelBlur {
                            red_blurriness: r,
                            green_blurriness: g,
                            blue_blurriness: b,
                            edges,
                            units,
                            ..
                        } if edges != "repeat" => [r, g, b]
                            .map(|s| {
                                let (sigma, long) = crate::effects::blur_reach(*s, units);
                                crate::effects::reach_radius(sigma, long)
                            })
                            .into_iter()
                            .max()
                            .unwrap_or(0),
                        _ => 0,
                    },
                    _ => 0,
                };
                if gw + gh + 1 > self.limits.max_texture_dimension_2d as usize
                    || w + h + 4 * reach + 1 > self.limits.max_texture_dimension_2d as usize
                    || bytes > self.limits.max_storage_buffer_binding_size as u64
                    || bytes > self.limits.max_buffer_size
                {
                    return Some(on_cpu(
                        Severity::Info,
                        format!("The CPU drew this frame: {name} grows a drawing to {gw} by {gh}, larger than the card allows."),
                        format!("{}: largest texture side {}.", self.about, self.limits.max_texture_dimension_2d),
                    ));
                }
                (w, h) = (gw, gh);
            }
        }
        let most = self.limits.max_texture_dimension_2d as usize;
        let frame_bytes = (plan.width * plan.height * 16) as u64;
        let too_big = plan
            .layers
            .iter()
            .flat_map(|l| std::iter::once(&l.source).chain(l.matte.as_ref().map(|m| &m.source)))
            .find(|s| {
                let sending = (s.width() * BYTES_PER_PIXEL).next_multiple_of(256) * s.height();
                s.width() > most
                    || s.height() > most
                    || sending as u64 > self.limits.max_buffer_size
            })
            .map(|s| format!("a drawing is {} by {}", s.width(), s.height()))
            .or_else(|| {
                (frame_bytes > self.limits.max_storage_buffer_binding_size as u64
                    || frame_bytes > self.limits.max_buffer_size
                    || plan.width.div_ceil(16)
                        > self.limits.max_compute_workgroups_per_dimension as usize
                    || plan.height.div_ceil(16)
                        > self.limits.max_compute_workgroups_per_dimension as usize)
                    .then(|| format!("the picture is {} by {}", plan.width, plan.height))
            })?;
        Some(on_cpu(
            Severity::Info,
            format!("The CPU drew this frame: {too_big}, larger than the card allows."),
            format!("{}: largest texture side {most}.", self.about),
        ))
    }

    /// The plan as eight-bit straight sRGB RGBA, the bytes `to_srgb8_straight` gives, or why the
    /// CPU must draw it instead.
    /// `cache` names the drawings it holds, so the card can keep them after it lets go.
    pub fn draw(&mut self, plan: &FramePlan, cache: &CelCache) -> Result<Vec<u8>, Diagnostic> {
        self.draw_held(plan, cache)?;
        if plan.width == 0 || plan.height == 0 {
            return Ok(Vec::new());
        }
        self.picture()
    }

    /// B-45: the picture [`Gpu::draw_held`] or [`Gpu::hold`] left on the card, brought back.
    pub fn picture(&mut self) -> Result<Vec<u8>, Diagnostic> {
        perf::time(Stage::GpuDraw, || self.read_back()).map_err(|e| self.failed(e))
    }

    /// B-45: [`Gpu::draw`], with the picture left on the card for [`Gpu::show`].
    /// One layer laid onto `onto`, a frame of `sum`'s size: `numbers` as the shader's `Layer`,
    /// its drawing and its matte's. Added to `steps`.
    fn lay(
        &self,
        steps: &mut Vec<Step>,
        numbers: [u32; 20],
        source: &wgpu::TextureView,
        matte: Option<&wgpu::TextureView>,
        onto: &wgpu::Buffer,
    ) {
        self.lay_with(&self.layer, steps, numbers, source, matte, onto);
    }

    /// [`Gpu::lay`] through `pipeline`, which reads the same numbers.
    fn lay_with(
        &self,
        pipeline: &wgpu::ComputePipeline,
        steps: &mut Vec<Step>,
        numbers: [u32; 20],
        source: &wgpu::TextureView,
        matte: Option<&wgpu::TextureView>,
        onto: &wgpu::Buffer,
    ) {
        let uniform = self
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("B-44 layer"),
                contents: bytemuck::cast_slice(&numbers),
                usage: wgpu::BufferUsages::UNIFORM,
            });
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.layer_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(source),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(matte.unwrap_or(&self.no_matte)),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: onto.as_entire_binding(),
                },
            ],
        });
        steps.push((
            pipeline.clone(),
            group,
            (numbers[14].div_ceil(16), numbers[15].div_ceil(16)),
        ));
    }

    /// B-156b (D-226), render::average: a motion-blurred layer's moments added up on the card, each
    /// at one over their number, as a texture the frame's size that is then laid through the
    /// identity. A moment behind the camera counts and adds nothing, as on the CPU.
    fn averaged(
        &self,
        steps: &mut Vec<Step>,
        layer: &crate::render::LayerDraw,
        source: &wgpu::TextureView,
        (width, height): (usize, usize),
    ) -> wgpu::TextureView {
        let sum = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("B-156b moments"),
            size: (width * height * 16) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let share = (1.0 / layer.moments.len() as f32).to_bits();
        let (sw, sh) = (layer.source.width() as f64, layer.source.height() as f64);
        let f = |v: f64| (v as f32).to_bits();
        for m in layer.moments.iter().flatten() {
            let Some(inverse) = m.invert() else { continue };
            // Outside the drawing grown by a pixel a moment samples only zero.
            let corners = [
                m.apply(-1.0, -1.0),
                m.apply(sw + 1.0, -1.0),
                m.apply(-1.0, sh + 1.0),
                m.apply(sw + 1.0, sh + 1.0),
            ];
            let low = corners.iter().fold((f64::INFINITY, f64::INFINITY), |m, c| {
                (m.0.min(c.0), m.1.min(c.1))
            });
            let high = corners
                .iter()
                .fold((f64::NEG_INFINITY, f64::NEG_INFINITY), |m, c| {
                    (m.0.max(c.0), m.1.max(c.1))
                });
            let (x0, y0, x1, y1) = if [low.0, low.1, high.0, high.1].iter().all(|v| v.is_finite()) {
                (
                    low.0.floor().clamp(0.0, width as f64) as u32,
                    low.1.floor().clamp(0.0, height as f64) as u32,
                    high.0.ceil().clamp(0.0, width as f64) as u32,
                    high.1.ceil().clamp(0.0, height as f64) as u32,
                )
            } else {
                (0, 0, width as u32, height as u32)
            };
            if x1 <= x0 || y1 <= y0 {
                continue;
            }
            let s0 = inverse.apply(x0 as f64 + 0.5, y0 as f64 + 0.5);
            let numbers = [
                f(s0.0),
                f(s0.1),
                f(inverse.a),
                f(inverse.b),
                f(inverse.c),
                f(inverse.d),
                0,
                0,
                0,
                0,
                0,
                0,
                x0,
                y0,
                x1 - x0,
                y1 - y0,
                width as u32,
                0,
                0,
                share,
            ];
            self.lay_with(&self.moment, steps, numbers, source, None, &sum);
        }
        let target = self.target.as_ref().expect("made before any layer");
        let averaged = self.scratch("B-156b averaged", width, height);
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.take.1,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: target.size.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: sum.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::TextureView(&averaged),
                },
            ],
        });
        steps.push((
            self.take.0.clone(),
            group,
            ((width as u32).div_ceil(16), (height as u32).div_ceil(16)),
        ));
        averaged
    }

    pub fn draw_held(&mut self, plan: &FramePlan, cache: &CelCache) -> Result<(), Diagnostic> {
        if let Some(refused) = self.refuse(plan) {
            return Err(refused);
        }
        let (width, height) = (plan.width, plan.height);
        if width == 0 || height == 0 {
            return Ok(());
        }
        self.frame += 1;
        self.device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
        self.device.push_error_scope(wgpu::ErrorFilter::Validation);
        // B-153: sending memory big enough for the most a frame has sent, up to an eighth of the
        // card's budget; a frame that sends more gives the rest memory of its own. B-153b: not
        // a measure of this frame's size, since Draft's frame is half the drawings it sends.
        let most = self.sending.most.min(self.budget as u64 / 8);
        if most > self.sending.buffer.as_ref().map_or(0, |b| b.size()) {
            self.made.set(self.made.get() + 1);
            self.sending.buffer = Some(self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("B-153 sending"),
                size: most,
                usage: wgpu::BufferUsages::MAP_WRITE | wgpu::BufferUsages::COPY_SRC,
                mapped_at_creation: true,
            }));
            self.sending.open = true;
            self.sending.after = None;
        }
        self.sending.filled = 0;

        // Each layer that can show anything, laid in order with what its effects dispatch. The
        // same skips as `render_tile`.
        self.target(width, height);
        let target = self.target.as_ref().expect("made above");
        let (sum, size) = (target.sum.clone(), target.size.clone());
        let mut uploads = self.device.create_command_encoder(&Default::default());
        let mut steps = Vec::new();
        perf::time(Stage::GpuUpload, || {
            for layer in &plan.layers {
                let Some(inverse) = layer.transform.invert() else {
                    continue;
                };
                let matte = match &layer.matte {
                    None => None,
                    Some(m) => match m.transform.invert() {
                        Some(inv) => Some((m, inv)),
                        None => continue,
                    },
                };
                if layer.source.width() == 0
                    || layer.source.height() == 0
                    || matte.is_some_and(|(m, _)| m.source.width() == 0 || m.source.height() == 0)
                {
                    continue;
                }
                // P-05: outside its box a layer samples only zero, which changes no pixel in any
                // of the four modes, so only the box is dispatched. A box that cannot be
                // computed is the whole frame, as it is in `render_tile`.
                // B-156b: a motion-blurred layer's average is the frame's size.
                let (l, t, r, b) = if layer.moments.is_empty() {
                    bounds(layer)
                } else {
                    (0.0, 0.0, width as f64, height as f64)
                };
                let (x0, y0, x1, y1) = if [l, t, r, b].iter().all(|v| v.is_finite()) {
                    (
                        l.floor().clamp(0.0, width as f64) as u32,
                        t.floor().clamp(0.0, height as f64) as u32,
                        r.ceil().clamp(0.0, width as f64) as u32,
                        b.ceil().clamp(0.0, height as f64) as u32,
                    )
                } else {
                    (0, 0, width as u32, height as u32)
                };
                if x1 <= x0 || y1 <= y0 {
                    continue;
                }
                let (ox, oy) = (x0 as f64 + 0.5, y0 as f64 + 0.5);
                let s0 = inverse.apply(ox, oy);
                let m = matte.map_or((0.0, 0.0, 0.0, 0.0, 0.0, 0.0), |(_, i)| {
                    let m0 = i.apply(ox, oy);
                    (m0.0, m0.1, i.a, i.b, i.c, i.d)
                });
                let f = |v: f64| (v as f32).to_bits();
                let blend = match layer.blend {
                    BlendMode::Normal => 0,
                    BlendMode::Multiply => 1,
                    BlendMode::Screen => 2,
                    BlendMode::Add => 3,
                    // D-301: refused above.
                    BlendMode::Overlay
                    | BlendMode::SoftLight
                    | BlendMode::StencilAlpha
                    | BlendMode::StencilLuma => 0,
                };
                let mut numbers = [
                    f(s0.0),
                    f(s0.1),
                    f(inverse.a),
                    f(inverse.b),
                    f(inverse.c),
                    f(inverse.d),
                    f(m.0),
                    f(m.1),
                    f(m.2),
                    f(m.3),
                    f(m.4),
                    f(m.5),
                    x0,
                    y0,
                    x1 - x0,
                    y1 - y0,
                    width as u32,
                    blend,
                    matte.is_some() as u32,
                    layer.opacity.to_bits(),
                ];
                let tiles = ((width as u32).div_ceil(16), (height as u32).div_ceil(16));
                // B-156 (D-225), render::adjust_frame: the frame laid so far through the layer's
                // stack, then mixed back by what the layer's shape, opacity and matte cover.
                if let Some(stack) = &layer.adjust {
                    let run = crate::compose::adjust_run(stack, (width, height))
                        .expect("refused when the CPU runs it");
                    let taken = self.scratch("B-156 frame", width, height);
                    let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &self.take.1,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: size.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: sum.as_entire_binding(),
                            },
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: wgpu::BindingResource::TextureView(&taken),
                            },
                        ],
                    });
                    steps.push((self.take.0.clone(), group, tiles));
                    // Only Paraffin and Kira-kira read the drawing on the CPU, and they stay there.
                    let mut views = Vec::new();
                    self.run(
                        &mut steps,
                        &layer.source,
                        (taken.clone(), (width, height)),
                        &run,
                        0,
                        &mut views,
                    );
                    let (effected, (ew, eh)) =
                        views.pop().flatten().unwrap_or((taken, (width, height)));
                    // Its shape is the composition's size, opaque, unless a mask cut it: that one is
                    // kept on the card, and only the cut ones are sent each frame. Both wide, so the
                    // cover is the CPU's to the bit.
                    let shape = if layer.source.data().par_iter().all(|v| *v == 1.0) {
                        if self.ones.as_ref().is_none_or(|o| {
                            (o.width(), o.height()) != (layer.source.width(), layer.source.height())
                        }) {
                            self.ones = Some(layer.source.clone());
                        }
                        self.ones.clone().expect("kept above")
                    } else {
                        layer.source.clone()
                    };
                    let shape = self.resident(&mut uploads, &shape, None, true);
                    let matte_view = matte.map(|(m, _)| {
                        self.resident(&mut uploads, &m.source, cache.name_of(&m.source), true)
                    });
                    let m = matte.map_or([0.0; 6], |(_, i)| [i.a, i.b, i.c, i.d, i.tx, i.ty]);
                    let k = [
                        inverse.a,
                        inverse.b,
                        inverse.c,
                        inverse.d,
                        inverse.tx,
                        inverse.ty,
                        m[0],
                        m[1],
                        m[2],
                        m[3],
                        m[4],
                        m[5],
                        layer.opacity as f64,
                    ];
                    // Every effect grows the frame alike on both sides.
                    let p = FxParams {
                        flag: matte.is_some() as u32,
                        base: [width as u32, height as u32],
                        ox: ((ew - width) / 2) as i32,
                        oy: ((eh - height) / 2) as i32,
                        ..Default::default()
                    };
                    let init = |label, contents: &[u8], usage| {
                        self.device
                            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some(label),
                                contents,
                                usage,
                            })
                    };
                    let numbers = init(
                        "B-156 numbers",
                        bytemuck::bytes_of(&p),
                        wgpu::BufferUsages::UNIFORM,
                    );
                    let k = init(
                        "B-156 cover",
                        bytemuck::cast_slice(&k),
                        wgpu::BufferUsages::STORAGE,
                    );
                    let passes = self
                        .fx
                        .as_ref()
                        .expect("an adjustment layer is refused without the passes");
                    let texture = |binding, view| wgpu::BindGroupEntry {
                        binding,
                        resource: wgpu::BindingResource::TextureView(view),
                    };
                    let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &passes.adjust.1,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: numbers.as_entire_binding(),
                            },
                            texture(1, &shape),
                            wgpu::BindGroupEntry {
                                binding: 3,
                                resource: k.as_entire_binding(),
                            },
                            texture(4, matte_view.as_ref().unwrap_or(&self.no_matte)),
                            texture(10, &effected),
                            wgpu::BindGroupEntry {
                                binding: 11,
                                resource: sum.as_entire_binding(),
                            },
                        ],
                    });
                    steps.push((passes.adjust.0.clone(), group, tiles));
                    continue;
                }
                let wraps = wraps(layer);
                // B-156: under an adjustment layer every drawing is sent whole, not in half
                // precision: Posterize, Threshold, a wipe's cut and the like turn the smallest
                // difference in the frame beneath into a large one (D-225).
                let wide = plan.layers.iter().any(|l| l.adjust.is_some())
                    || !wraps.is_empty()
                    || matches!(
                        layer.on_card.first().map(OnCard::unmixed),
                        Some(OnCard::Bloom(_) | OnCard::Glow(_) | OnCard::Fx(_))
                    );
                let mut source = self.resident(
                    &mut uploads,
                    &layer.source,
                    cache.name_of(&layer.source),
                    wide,
                );
                if !layer.moments.is_empty() {
                    source = self.averaged(&mut steps, layer, &source, (width, height));
                }
                if !layer.on_card.is_empty() {
                    source = self.applied(&mut steps, &layer.source, &source, &layer.on_card);
                }
                let matte = matte.map(|(m, _)| {
                    self.resident(&mut uploads, &m.source, cache.name_of(&m.source), false)
                });
                if wraps.is_empty() {
                    self.lay(&mut steps, numbers, &source, matte.as_ref(), &sum);
                    continue;
                }
                // B-76, render::wrap_layer: the layer placed alone, its light the frame beneath
                // with its covering, blurred once for each wrap, and the result laid as the layer.
                let placed = self.device.create_buffer(&wgpu::BufferDescriptor {
                    label: Some("B-76 placed"),
                    size: (width * height * 16) as u64,
                    usage: wgpu::BufferUsages::STORAGE,
                    mapped_at_creation: false,
                });
                let (blend, opacity) = (numbers[17], numbers[19]);
                numbers[17] = 0;
                numbers[19] = 1f32.to_bits();
                self.lay(&mut steps, numbers, &source, matte.as_ref(), &placed);
                let mut drawn = self.scratch("B-76 drawn", width, height);
                let light = self.scratch("B-76 light", width, height);
                let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: None,
                    layout: &self.unpack_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: size.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: sum.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 2,
                            resource: placed.as_entire_binding(),
                        },
                        wgpu::BindGroupEntry {
                            binding: 3,
                            resource: wgpu::BindingResource::TextureView(&drawn),
                        },
                        wgpu::BindGroupEntry {
                            binding: 4,
                            resource: wgpu::BindingResource::TextureView(&light),
                        },
                    ],
                });
                steps.push((self.unpack.clone(), group, tiles));
                for (w, intensity, add) in wraps {
                    if intensity == 0.0 {
                        continue;
                    }
                    let sigma = w / 3.0;
                    let r = crate::effects::kernel_radius(sigma);
                    let blurred = if r == 0 {
                        light.clone()
                    } else {
                        self.gaussian(
                            &mut steps,
                            &light,
                            (width, height),
                            Gaussian {
                                sigma,
                                repeat: false,
                                long: false,
                            },
                        )
                        .0
                    };
                    let out = self.scratch("B-76 wrapped", width, height);
                    let p = FxParams {
                        r: r as i32,
                        flag: add as u32,
                        ..Default::default()
                    };
                    let passes = self
                        .fx
                        .as_ref()
                        .expect("a Light Wrap is refused without the passes");
                    self.fx_step(
                        &mut steps,
                        &passes.wrap,
                        p,
                        Some(&drawn),
                        Some(&out),
                        Some(&[intensity / 100.0]),
                        Some(&blurred),
                        [None; 4],
                        tiles,
                    );
                    drawn = out;
                }
                let f = |v: f32| v.to_bits();
                let placed = [
                    f(x0 as f32 + 0.5),
                    f(y0 as f32 + 0.5),
                    f(1.0),
                    f(0.0),
                    f(0.0),
                    f(1.0),
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    x0,
                    y0,
                    x1 - x0,
                    y1 - y0,
                    width as u32,
                    blend,
                    0,
                    opacity,
                ];
                self.lay(&mut steps, placed, &drawn, None, &sum);
            }
        });

        let drawn = perf::time(Stage::GpuDraw, || {
            let target = self.target.as_ref().expect("made above");
            let encode_group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &self.encode_layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: target.size.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: target.sum.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: target.bytes.as_entire_binding(),
                    },
                ],
            });

            let mut encoder = self.device.create_command_encoder(&Default::default());
            encoder.clear_buffer(&target.sum, 0, None);
            // B-153: a working texture lent again holds zero, as a new one does.
            for texture in self.to_clear.borrow_mut().drain(..) {
                encoder.clear_texture(&texture, &Default::default());
            }
            {
                // One after another: wgpu has each dispatch wait for what the one before wrote,
                // so a Light Wrap reads the layers laid before it.
                let mut pass = encoder.begin_compute_pass(&Default::default());
                self.dispatched += steps.len() as u64;
                for (pipeline, group, (x, y)) in &steps {
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, group, &[]);
                    pass.dispatch_workgroups(*x, *y, 1);
                }
                pass.set_pipeline(&self.encode);
                pass.set_bind_group(0, &encode_group, &[]);
                pass.dispatch_workgroups(
                    (width as u32).div_ceil(16),
                    (height as u32).div_ceil(16),
                    1,
                );
            }
            // The drawings arrive before the frame that draws with them: one queue, in order.
            if std::mem::take(&mut self.sending.open) {
                self.sending
                    .buffer
                    .as_ref()
                    .expect("open only while there")
                    .unmap();
            }
            self.sending.after = Some(self.queue.submit([uploads.finish(), encoder.finish()]));
            // Working textures neither this frame nor the one before used, and no drawing keeps,
            // are let go: an effect redrawn each frame frees last frame's result only as it
            // keeps this one.
            if let Some(working) = &self.working {
                working
                    .borrow_mut()
                    .retain(|(t, used)| *used + 1 >= self.frame || self.keeps(t));
            }
            // Audit P9: a Curl Noise field neither frame used goes back to the working textures.
            self.fields.borrow_mut().retain(|(_, view, used)| {
                let kept = *used + 1 >= self.frame;
                if let (false, Some(working)) = (kept, &self.working) {
                    working.borrow_mut().push((view.texture().clone(), *used));
                }
                kept
            });

            let invalid = pollster::block_on(self.device.pop_error_scope());
            let memory = pollster::block_on(self.device.pop_error_scope());
            match memory.or(invalid) {
                Some(e) => Err(e.to_string()),
                None => Ok(()),
            }
        });
        drawn.map_err(|e| self.failed(e))
    }

    /// The picture [`Gpu::draw_held`] left on the card, brought back.
    fn read_back(&self) -> Result<Vec<u8>, String> {
        let target = self.target.as_ref().expect("drawn before it is read");
        let mut encoder = self.device.create_command_encoder(&Default::default());
        encoder.copy_buffer_to_buffer(
            &target.bytes,
            0,
            &target.readback,
            0,
            (target.width * target.height * 4) as u64,
        );
        self.queue.submit([encoder.finish()]);
        let slice = target.readback.slice(..);
        let (tell, told) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| drop(tell.send(r)));
        self.device
            .poll(wgpu::PollType::Wait)
            .map_err(|e| e.to_string())?;
        told.recv()
            .map_err(|e| e.to_string())?
            .map_err(|e| e.to_string())?;
        let pixels = slice.get_mapped_range().to_vec();
        target.readback.unmap();
        Ok(pixels)
    }

    fn failed(&mut self, e: String) -> Diagnostic {
        self.failures += 1;
        // Whatever the card was holding may be what failed; start again from nothing.
        self.store.clear();
        self.spare.clear();
        self.target = None;
        self.sending.buffer = None;
        if let Some(working) = &self.working {
            working.borrow_mut().clear();
        }
        self.to_clear.borrow_mut().clear();
        self.fields.borrow_mut().clear();
        on_cpu(
            Severity::Warning,
            "The CPU drew this frame: the graphics card failed.".into(),
            format!("{}: {e}", self.about),
        )
        .with_remediation("If it keeps happening, switch the preview back to CPU, or to Draft.")
    }

    /// B-45: a picture the CPU drew, eight-bit straight sRGB, put where [`Gpu::show`] reads.
    pub fn hold(&mut self, pixels: &[u8], width: usize, height: usize) {
        if width == 0 || height == 0 {
            return;
        }
        self.target(width, height);
        let target = self.target.as_ref().expect("made above");
        self.queue.write_buffer(&target.bytes, 0, pixels);
    }

    /// B-45: paint into `window` from now on. The page over it must be see-through where the
    /// card paints, which is the caller's business.
    ///
    /// # Safety
    /// `window` must outlive this `Gpu`, or [`Gpu::let_go`] must be called before it goes.
    pub unsafe fn attach(
        &mut self,
        window: &(impl wgpu::rwh::HasWindowHandle + wgpu::rwh::HasDisplayHandle),
    ) -> Result<(), String> {
        let target =
            unsafe { wgpu::SurfaceTargetUnsafe::from_window(window) }.map_err(|e| e.to_string())?;
        let surface =
            unsafe { self.instance.create_surface_unsafe(target) }.map_err(|e| e.to_string())?;
        let caps = surface.get_capabilities(&self.adapter);
        // Plain eight-bit, not sRGB: the page's colour numbers go to the screen as they are.
        let format = [
            wgpu::TextureFormat::Bgra8Unorm,
            wgpu::TextureFormat::Rgba8Unorm,
        ]
        .into_iter()
        .find(|f| caps.formats.contains(f))
        .ok_or_else(|| {
            format!(
                "{} cannot paint this window in eight-bit colour",
                self.about
            )
        })?;
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: 0,
            height: 0,
            present_mode: wgpu::PresentMode::AutoVsync,
            desired_maximum_frame_latency: 1,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
        };
        let module = self
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("B-45"),
                source: wgpu::ShaderSource::Wgsl(SCREEN_SHADER.into()),
            });
        let fragment = |binding, ty| wgpu::BindGroupLayoutEntry {
            visibility: wgpu::ShaderStages::FRAGMENT,
            ..entry(binding, ty)
        };
        let layout = self
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("B-45 screen"),
                entries: &[
                    fragment(0, uniform()),
                    fragment(1, storage(true)),
                    fragment(2, storage(true)),
                ],
            });
        let pipeline = self
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("B-45 screen"),
                layout: Some(&self.device.create_pipeline_layout(
                    &wgpu::PipelineLayoutDescriptor {
                        label: None,
                        bind_group_layouts: &[&layout],
                        push_constant_ranges: &[],
                    },
                )),
                vertex: wgpu::VertexState {
                    module: &module,
                    entry_point: Some("corner"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &module,
                    entry_point: Some("paint"),
                    compilation_options: Default::default(),
                    targets: &[Some(format.into())],
                }),
                primitive: Default::default(),
                depth_stencil: None,
                multisample: Default::default(),
                multiview: None,
                cache: None,
            });
        // The page is a child window covering the whole of this one. Windows leaves a parent's
        // pixels out wherever a child is unless this is cleared, and wry's own example of a
        // card painting under a see-through page clears it too.
        #[cfg(windows)]
        if let Some(wgpu::rwh::RawWindowHandle::Win32(h)) =
            window.window_handle().ok().map(|w| w.as_raw())
        {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{
                GetWindowLongPtrW, SetWindowLongPtrW, GWL_STYLE, WS_CLIPCHILDREN,
            };
            let hwnd = HWND(h.hwnd.get() as *mut _);
            // SAFETY: the handle is the live window the caller vouched for; only its style changes.
            unsafe {
                SetWindowLongPtrW(
                    hwnd,
                    GWL_STYLE,
                    GetWindowLongPtrW(hwnd, GWL_STYLE) & !(WS_CLIPCHILDREN.0 as isize),
                )
            };
        }
        self.screen = Some(Screen {
            surface,
            config,
            pipeline,
            layout,
        });
        Ok(())
    }

    /// Whether the card paints into a window.
    pub fn on_screen(&self) -> bool {
        self.screen.is_some()
    }

    /// Stop painting into the window.
    pub fn let_go(&mut self) {
        self.screen = None;
    }

    /// B-45: paint the window, `size` pixels, with the last picture drawn or held.
    pub fn show(
        &mut self,
        size: (u32, u32),
        paints: &[Paint],
        alpha_only: bool,
    ) -> Result<(), String> {
        let (Some(screen), Some(target)) = (self.screen.as_mut(), self.target.as_ref()) else {
            return Ok(());
        };
        if size.0 == 0 || size.1 == 0 || paints.is_empty() {
            return Ok(());
        }
        perf::time(Stage::GpuShow, || {
            if (screen.config.width, screen.config.height) != size {
                (screen.config.width, screen.config.height) = size;
                screen.surface.configure(&self.device, &screen.config);
            }
            let frame = match screen.surface.get_current_texture() {
                Ok(frame) => frame,
                // The window changed under it: once more, freshly set up.
                Err(wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost) => {
                    screen.surface.configure(&self.device, &screen.config);
                    screen
                        .surface
                        .get_current_texture()
                        .map_err(|e| e.to_string())?
                }
                Err(e) => return Err(e.to_string()),
            };
            self.device.push_error_scope(wgpu::ErrorFilter::Validation);
            let numbers = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("B-45 screen"),
                    contents: bytemuck::cast_slice(&[
                        paints.len() as u32,
                        target.width as u32,
                        target.height as u32,
                        alpha_only as u32,
                    ]),
                    usage: wgpu::BufferUsages::UNIFORM,
                });
            let list = self
                .device
                .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("B-45 paints"),
                    contents: bytemuck::cast_slice(paints),
                    usage: wgpu::BufferUsages::STORAGE,
                });
            let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &screen.layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: numbers.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: list.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: target.bytes.as_entire_binding(),
                    },
                ],
            });
            let view = frame.texture.create_view(&Default::default());
            let mut encoder = self.device.create_command_encoder(&Default::default());
            {
                let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: None,
                    color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                        view: &view,
                        depth_slice: None,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                            store: wgpu::StoreOp::Store,
                        },
                    })],
                    ..Default::default()
                });
                pass.set_pipeline(&screen.pipeline);
                pass.set_bind_group(0, &group, &[]);
                pass.draw(0..3, 0..1);
            }
            self.queue.submit([encoder.finish()]);
            if let Some(e) = pollster::block_on(self.device.pop_error_scope()) {
                return Err(e.to_string());
            }
            frame.present();
            Ok(())
        })
    }
}

#[cfg(test)]
mod stride {
    /// The card reads every sprite list by `particles::STRIDE` from `particles::HEAD`: a change
    /// to either that the shader misses fails here instead of drawing the wrong sprites.
    #[test]
    fn the_card_reads_sprites_by_the_processors_stride() {
        let s = super::FX_SHADER;
        let line = format!("const SPRITE_STRIDE: u32 = {}u;", crate::particles::STRIDE);
        assert!(
            s.contains(&line),
            "the shader's sprite stride is not {line}"
        );
        assert_eq!(
            crate::particles::HEAD,
            7,
            "the shader reads sprites from 7u"
        );
        for (i, _) in s.match_indices("7u + ") {
            let rest = &s[i + 5..];
            let n = rest.chars().take_while(|c| c.is_ascii_digit()).count();
            let read = rest[n..].starts_with("u * n;") || rest[n..].starts_with("u * u32(k[");
            assert!(
                n == 0 || !read,
                "a sprite read by a written number: {}",
                &rest[..40]
            );
        }
    }
}
