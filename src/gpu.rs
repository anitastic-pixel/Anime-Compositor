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
    let n = min(u32(ceil(path)) + 1u, 256u);
    if n == 1u {
        textureStore(moved, id.xy, textureLoad(still, id.xy, 0));
        return;
    }
    let start = n * (n - 1u) / 2u - 1u;
    var total = vec4(0.0);
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
        total += bilinear(still, p);
    }
    textureStore(moved, id.xy, total / f32(n));
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
fn tap(c: i32, y: f64) -> vec4<f32> {
    let f = y - 0.5lf;
    let base = floor(f);
    let u = f - base;
    let r = i32(base);
    var o = vec4(0.0);
    if 1.0lf - u != 0.0lf {
        o += at(select(vec2(c, r), vec2(r, c), P.down == 1u)) * f32(1.0lf - u);
    }
    if u != 0.0lf {
        o += at(select(vec2(c, r + 1), vec2(r + 1, c), P.down == 1u)) * f32(u);
    }
    return o;
}

fn nz(v: vec4<f32>) -> u32 {
    return select(0u, 1u, any(v != vec4(0.0)));
}

// blurs::by_lines for one line a thread: the tent a - b|j| over |j| <= n, from running totals
// kept in double precision, then (B-49) the `ends` taps, pairs of column and weight in
// `weights`. `lp` and `rp` are the samples left and right of the point, `lw` and `rw` the same
// times their distance, and `seen` how many within `reach` are not zero.
@compute @workgroup_size(32)
fn streak(@builtin(global_invocation_id) id: vec3<u32>) {
    if id.x >= P.count {
        return;
    }
    let k = f64(P.k0 + i32(id.x)) + 0.5lf;
    let n = i32(P.n);
    let m = i32(P.reach);
    let x0 = -P.g;
    var lp = vec4<f64>(0.0lf);
    var rp = lp;
    var lw = lp;
    var rw = lp;
    var seen = 0u;
    for (var j = 1; j <= m; j++) {
        let vr = tap(x0 + j, k + f64(x0 + j) * P.s);
        let vl = tap(x0 - j, k + f64(x0 - j) * P.s);
        if j <= n {
            rp += vec4<f64>(vr);
            rw += f64(j) * vec4<f64>(vr);
            lp += vec4<f64>(vl);
            lw += f64(j) * vec4<f64>(vl);
        }
        seen += nz(vr) + nz(vl);
    }
    var vc = tap(x0, k + f64(x0) * P.s);
    seen += nz(vc);
    for (var x = 0u; x < P.width; x++) {
        let c = i32(x) + x0;
        var r = P.a * (lp + vec4<f64>(vc) + rp) - P.b * (rw + lw);
        for (var e = 0u; e < P.ends; e++) {
            let j = c + i32(weights[2u * e]);
            r += f64(weights[2u * e + 1u]) * vec4<f64>(tap(j, k + f64(j) * P.s));
        }
        textureStore(output, vec2(x, id.x), select(vec4<f32>(r), vec4(0.0), seen == 0u));
        let vn = tap(c + n + 1, k + f64(c + n + 1) * P.s);
        let vo = tap(c - n, k + f64(c - n) * P.s);
        let v1 = tap(c + 1, k + f64(c + 1) * P.s);
        rw = rw - rp + f64(n) * vec4<f64>(vn);
        rp = rp - vec4<f64>(v1) + vec4<f64>(vn);
        lw = lw + lp + vec4<f64>(vc) - f64(n + 1) * vec4<f64>(vo);
        lp = lp + vec4<f64>(vc) - vec4<f64>(vo);
        if m == n {
            seen = seen + nz(vn) - nz(vo);
        } else {
            seen = seen + nz(tap(c + m + 1, k + f64(c + m + 1) * P.s)) - nz(tap(c - m, k + f64(c - m) * P.s));
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
    if P.screen == 1u {
        let v = clamp(h * P.weight, vec4(0.0), vec4(1.0));
        textureStore(output, id.xy, p + v - p * v);
        return;
    }
    textureStore(output, id.xy, vec4(p.xyz + h.xyz * P.weight, min(p.w + h.w * P.weight, 1.0)));
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
    (sigma * sigma - (2.0 * f * f + 1.0) / 12.0 - f * f / 6.0).max(0.0).sqrt() / f
}

/// B-164 (D-221): the small blur's weights. Document 21's kernel stops at its radius r, so the
/// small one stops where r + 1/2 falls among the small pixels, its last weights cut by the part
/// of their pixel inside it, and is normalised as the exact one is; a small kernel of its own
/// length leaves the exact one's edge up to f pixels out, which a dark picture shows.
fn small_taps(sigma: f64, f: usize) -> Vec<f32> {
    let s = small_sigma(sigma, f);
    let edge = (crate::effects::kernel_radius(sigma) as f64 + 0.5) / f as f64;
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

// grade::mixer.
fn mixed(b: f64, c: f64) -> f64 {
    switch F.blend {
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

// The batch's colour effects, a pixel a thread: grade::curves (mode 0), levels (1),
// hue_saturation (2), gradient (3) and noise (4); B-76: exposure (5, Exposure Flicker),
// color_balance (6), gradient_map (7), vignette (8) and fractal_noise (9).
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
    let a = f64(p.w);
    if a <= 0.0lf {
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
        // 2, the colour in linear light.
        let dx = (f64(id.x) + 0.5lf - k[0]) / k[2];
        let dy = (f64(id.y) + 0.5lf - k[1]) / k[3];
        let d = sqrt(dx * dx + dy * dy) / k[7];
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
    if F.mode == 9u {
        // k: the size, the depth, the contrast and brightness, the opacity as a share, the dark
        // and light colours encoded. `count` is the octaves.
        let x = (f64(i32(id.x) - F.ox) + 0.5lf) / k[0];
        let y = (f64(i32(id.y) - F.oy) + 0.5lf) / k[0];
        let n = fractal(0u, vec3(x, y, k[1]), F.count);
        let v = clamp(0.5lf + 0.5lf * n * k[2] / 100.0lf + k[3] / 100.0lf, 0.0lf, 1.0lf);
        var out = p;
        for (var c = 0u; c < 3u; c++) {
            let color = to_linear(k[5u + c] + v * (k[8u + c] - k[5u + c]));
            let b = px[c] / a;
            out[c] = f32((b + k[4] * (mixed(b, color) - b)) * a);
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
            // k: input black and white, one over the gamma, output black and white.
            for (var c = 0u; c < 3u; c++) {
                // The power in single precision, the one step here the card cannot do in double;
                // its ends are exact.
                let v = clamp((e[c] * 255.0lf - k[0]) / (k[1] - k[0]), 0.0lf, 1.0lf);
                var bent = f64(pow(f32(v), f32(k[2])));
                if v == 0.0lf || v == 1.0lf {
                    bent = v;
                }
                o[c] = (k[3] + bent * (k[4] - k[3])) / 255.0lf;
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

// layer_fx::chromatic_aberration. k: the centre, 1 - k and 1 + k.
@compute @workgroup_size(16, 16)
fn aberr(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
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
    let p = vec3((x - k[3]) / k[1], (y - k[4]) / k[1], k[2]);
    var sx = x + k[0] * fractal(0u, p, F.count);
    var sy = y + k[0] * fractal(1u, p, F.count);
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

// B-76, layer_fx::light_rays' last step: `other` is the bright pixels zoomed out. k: the
// intensity, the colour in linear light.
@compute @workgroup_size(16, 16)
fn rays(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let p = textureLoad(input, id.xy, 0);
    let r = textureLoad(other, id.xy, 0);
    var out: vec4<f32>;
    for (var c = 0u; c < 3u; c++) {
        out[c] = f32(f64(p[c]) + k[0] * k[1u + c] * f64(r[c]));
    }
    out.w = f32(min(f64(p.w) + k[0] * f64(r.w), 1.0lf));
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
    let c = f32(f64(v) / a);
    var s: f32;
    if c <= 0.0031308 {
        s = f32(f64(12.92f) * f64(c));
    } else {
        let y = f32(root(f64(c)) * (1.0lf + k[7] * f64(log(c))));
        s = f32(f64(f32(f64(1.055f) * f64(y))) - f64(0.055f));
    }
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

// B-107, the third batch's colour effects, a pixel a thread: grade::invert (mode 0, `count` the
// channel, 3 all), invert_alpha (1), brightness_contrast (2), black_white (3), posterize (4),
// threshold (5), channel_mixer (6), vibrance (7), leave_color (8), solarize (9) and halftone
// (10); and (B-123) color_lookup (11), hsv_key (12) and paraffin (13).
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
    if a <= 0.0lf {
        textureStore(output, id.xy, p);
        return;
    }
    if F.mode == 12u {
        // k: as hsv_inside and level8 read it, and 1 to invert at k[6].
        textureStore(output, id.xy, select(p, vec4(0.0), hsv_inside(p, a) != (k[6] != 0.0lf)));
        return;
    }
    let px = vec3<f64>(p.xyz);
    var e: vec3<f64>;
    for (var c = 0u; c < 3u; c++) {
        e[c] = to_srgb(clamp(px[c] / a, 0.0lf, 1.0lf));
    }
    var out = p;
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
        case 2u, 3u: {
            // k: the centre, the turn in radians or the height, the radius.
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
            // drawing's size and its corner.
            let px = x - k[4];
            let py = y - k[5];
            var qx: f64;
            var qy: f64;
            if k[1] == 1.0lf {
                let nx = quotient(px - k[2] / 2.0lf, k[2] / 2.0lf);
                let ny = quotient(py - k[3] / 2.0lf, k[3] / 2.0lf);
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
                qx = k[2] / 2.0lf * (1.0lf + v * sin64(a));
                qy = k[3] / 2.0lf * (1.0lf - v * cos64(a));
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

// B-107, the four wipes: layer_fx::linear_wipe (mode 0), radial_wipe (1, `count` the way round),
// venetian_blinds (2) and iris_wipe (3). `flag` is complete, every pixel gone.
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

// Each pixel its block's mean. k as for `blocks`, then each column's block, then each row's.
@compute @workgroup_size(16, 16)
fn tiles(@builtin(global_invocation_id) id: vec3<u32>) {
    let size = textureDimensions(input);
    if id.x >= size.x || id.y >= size.y {
        return;
    }
    let nc = u32(k[0]);
    let nr = u32(k[1]);
    let start = 4u + nc + nr;
    let i = 4u * (u32(k[start + size.x + id.y]) * nc + u32(k[start + id.x]));
    textureStore(output, id.xy, vec4(row[i], row[i + 1u], row[i + 2u], row[i + 3u]));
}

// layer_fx::picture_luma.
fn picture_luma(p: vec4<f32>) -> f64 {
    return to_srgb(clamp(0.2126lf * f64(p.x) + 0.7152lf * f64(p.y) + 0.0722lf * f64(p.z), 0.0lf, 1.0lf));
}

// B-107, layer_fx::emboss (mode 0; k: the relief across and down, the contrast as a share,
// colour) and find_edges (mode 1; k: the amount as a share, invert).
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
// lighten): `other` is the picture blurred, grown by `r`. k: the amount as a share.
@compute @workgroup_size(16, 16)
fn sharp(@builtin(global_invocation_id) id: vec3<u32>) {
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
            out[c] = f32((b + k[0] * (f - b)) * a);
        }
    }
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
// arm's direction, then each step's distance and weight.
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
    let steps = 5u + 2u * arms;
    var g = vec4<f64>(0.0lf);
    for (var j = 0u; j < arms; j++) {
        let vx = k[5u + 2u * j];
        let vy = k[6u + 2u * j];
        for (var t = 0u; t < u32(F.g); t++) {
            let d = k[steps + 2u * t];
            g += k[steps + 2u * t + 1u] * vec4<f64>(bilinear(other, cx - d * vx, cy - d * vy));
        }
    }
    let o = at(input, vec2(x, y));
    var out: vec4<f32>;
    for (var c = 0u; c < 3u; c++) {
        out[c] = f32(f64(o[c]) + k[0] * k[1u + c] * g[c]);
    }
    out.w = f32(min(f64(o.w) + k[0] * g.w, 1.0lf));
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

// line_blur's covering and ink of a pixel.
fn cover_ink(p: vec4<f32>) -> vec2<f64> {
    let q = vec4<f64>(p);
    return vec2(q.w, clamp(q.w - 0.2126lf * q.x - 0.7152lf * q.y - 0.0722lf * q.z, 0.0lf, 1.0lf));
}

// line_blur's `sample`: the drawing at pixel position (x, y), its corners on the pixels.
fn tap(x: f64, y: f64) -> vec4<f64> {
    let x0 = floor(x);
    let y0 = floor(y);
    let fx = x - x0;
    let fy = y - y0;
    let i = i32(x0);
    let j = i32(y0);
    let p00 = vec4<f64>(at(input, vec2(i, j)));
    let p10 = vec4<f64>(at(input, vec2(i + 1, j)));
    let p01 = vec4<f64>(at(input, vec2(i, j + 1)));
    let p11 = vec4<f64>(at(input, vec2(i + 1, j + 1)));
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
    let own = vec4<f64>(mine);
    if k[0] == 0.0lf {
        textureStore(output, id.xy, mine);
        return;
    }
    var weights = array<f64, 5>(1.0lf, 4.0lf, 6.0lf, 4.0lf, 1.0lf);
    var t = vec3(0.0lf);
    for (var j = 0; j < 5; j++) {
        for (var i = 0; i < 5; i++) {
            let x = lx + i - 2;
            let y = ly + j - 2;
            let l = cover_ink(at(input, vec2(x - 1, y)));
            let r = cover_ink(at(input, vec2(x + 1, y)));
            let u = cover_ink(at(input, vec2(x, y - 1)));
            let d = cover_ink(at(input, vec2(x, y + 1)));
            let gx = (r - l) / 2.0lf;
            let gy = (d - u) / 2.0lf;
            let raw = vec3(gx.x * gx.x + gx.y * gx.y, gx.x * gy.x + gx.y * gy.y, gy.x * gy.x + gy.y * gy.y);
            t += weights[i] * weights[j] / 256.0lf * raw;
        }
    }
    let a = t.x;
    let b = t.y;
    let c = t.z;
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
    var acc = own;
    var total = 1.0lf;
    for (var side = 0; side < 2; side++) {
        let way = select(1.0lf, -1.0lf, side == 1);
        for (var q = 0u; q < F.count; q++) {
            let out = way * f64(q + 1u);
            let s = tap(f64(lx) + out * tx, f64(ly) + out * ty);
            if s.w < 1.0lf / 256.0lf {
                break;
            }
            acc += k[2u + q] * s;
            total += k[2u + q];
        }
    }
    var amount = k[0] * r / (a + c);
    if k[1] != 0.0lf {
        amount *= cover_ink(mine).y;
    }
    textureStore(output, id.xy, vec4<f32>(own + (acc / total - own) * amount));
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
    let a = select(own.w, mid.w, F.flag == 1u);
    if a <= 0.0 || shows == 0u {
        textureStore(output, id.xy, vec4(0.0));
        return;
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
"#;

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
    choke: Pass,
    lines: Pass,
    glare: Pass,
    rain: Pass,
    /// B-123.
    line_blur: Pass,
    stars: Pass,
    kira: Pass,
    /// B-151.
    median: Pass,
    smart: Pass,
    rough: Pass,
    rshadow: Pass,
    bevel: Pass,
    snow: Pass,
    cells: Pass,
    /// B-156.
    adjust: Pass,
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
            crate::effects::Effect::LightWrap { width, intensity, blend } => Some((*width, *intensity, blend == "add")),
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
    applied: Option<(Vec<OnCard>, Vec<(wgpu::TextureView, (usize, usize))>)>,
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
    /// B-153: the passes' working textures, each with the last frame that used it, kept for the
    /// next frame's passes of the same size, when the card can clear them to what a new one holds.
    working: Option<RefCell<Vec<(wgpu::Texture, u64)>>>,
    /// B-153: working textures lent again this frame, cleared before the passes run.
    to_clear: RefCell<Vec<wgpu::Texture>>,
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
            entries: &[entry(0, uniform()), entry(1, texture()), entry(2, texture()), entry(3, storage(false))],
        });
        let encode_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("B-44 encode"),
            entries: &[entry(0, uniform()), entry(1, storage(true)), entry(2, storage(false))],
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
        let pipeline_in = |module: &wgpu::ShaderModule, layout: &wgpu::BindGroupLayout, entry_point: &str| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry_point),
                layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[layout],
                    push_constant_ranges: &[],
                })),
                module,
                entry_point: Some(entry_point),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        let pipeline = |layout: &wgpu::BindGroupLayout, entry_point: &str| pipeline_in(&module, layout, entry_point);
        let bloom = (!f64.is_empty()).then(|| {
            let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("B-47"),
                source: wgpu::ShaderSource::Wgsl(BLOOM_SHADER.into()),
            });
            // Each pass's layout holds only the bindings it reads, so none needs a stand-in.
            let pass = |entry_point: &str, bindings: &[u32]| {
                let entries: Vec<_> = bindings
                    .iter()
                    .map(|&b| entry(b, [uniform(), texture(), storage_texture(), storage(false), storage(true)][b as usize]))
                    .collect();
                let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some(entry_point), entries: &entries });
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
                let types = [uniform(), texture(), storage_texture(), storage(true), texture(), storage(false), storage(false), storage(false), storage(false), storage(false), texture(), storage(false)];
                let entries: Vec<_> = bindings.iter().map(|&b| entry(b, types[b as usize])).collect();
                let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some(entry_point), entries: &entries });
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
                choke: pass("choke", &[0, 1, 2, 3, 6, 7]),
                lines: pass("lines", &[0, 1, 2, 3]),
                glare: pass("glare", &[0, 1, 2, 3, 4]),
                rain: pass("rain", &[0, 1, 2, 3]),
                line_blur: pass("lineblur", &[0, 1, 2, 3]),
                stars: pass("stars", &[0, 1, 3, 9]),
                kira: pass("kira", &[0, 1, 2, 3, 9]),
                median: pass("median", &[0, 1, 2, 3]),
                smart: pass("smart", &[0, 1, 2, 3, 4]),
                rough: pass("rough", &[0, 1, 2, 3]),
                rshadow: pass("rshadow", &[0, 1, 2, 3, 4]),
                bevel: pass("bevel", &[0, 1, 2, 3, 4]),
                snow: pass("snow", &[0, 1, 2, 3]),
                cells: pass("cells", &[0, 1, 2, 3]),
                adjust: pass("adjust", &[0, 1, 3, 4, 10, 11]),
            }
        });
        let (layer, encode) = (pipeline(&layer_layout, "layer"), pipeline(&encode_layout, "encode"));
        let radial = pipeline(&radial_layout, "radial");
        let unpack = pipeline(&unpack_layout, "unpack");
        let own = |entry_point: &str, entries: &[wgpu::BindGroupLayoutEntry]| {
            let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor { label: Some(entry_point), entries });
            (pipeline(&layout, entry_point), layout)
        };
        let take = own("take", &[entry(0, uniform()), entry(1, storage(true)), entry(3, storage_texture())]);
        let no_matte = device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("B-44 no matte"),
                size: wgpu::Extent3d { width: 1, height: 1, depth_or_array_layers: 1 },
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
            memory.map_or("its memory not reported".into(), |m| format!("{:.1} GB of its own memory", m as f64 / 1e9))
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
            working: clear.then(|| RefCell::new(Vec::new())),
            to_clear: RefCell::new(Vec::new()),
        })
    }

    /// The card's own memory in bytes, when Windows says (B-48).
    pub fn memory(&self) -> Option<u64> {
        self.memory
    }

    /// D-105: what the card may hold when the memory setting is Automatic: half its memory, as
    /// B-44b set it.
    pub fn automatic_budget(&self) -> usize {
        self.memory.map_or(DEFAULT_BUDGET_BYTES, |m| (m / 2) as usize)
    }

    /// D-105: the most a Custom setting may give the card: 85% of its memory, so the frame's own
    /// buffers, the window and every other program keep the rest.
    pub fn largest_budget(&self) -> usize {
        self.memory.map_or(DEFAULT_BUDGET_BYTES, |m| (m / 20 * 17) as usize).max(DEFAULT_BUDGET_BYTES)
    }

    /// How many frames the card has failed since it was opened (B-48): not the ones it refused,
    /// which the CPU draws because the card does not draw them yet, but the ones it tried and could not.
    pub fn failures(&self) -> u64 {
        self.failures
    }

    /// Bytes of drawings the card holds now (B-48), and of textures kept for new ones (B-153).
    pub fn held(&self) -> usize {
        let spare = |t: &wgpu::Texture| {
            (t.width() * t.height()) as usize * if t.format() == wgpu::TextureFormat::Rgba32Float { 16 } else { BYTES_PER_PIXEL }
        };
        self.store.iter().map(|s| s.bytes).sum::<usize>() + self.spare.iter().map(spare).sum::<usize>()
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

    /// The card, its driver and the backend, for tables and the switch.
    pub fn about(&self) -> &str {
        &self.about
    }

    /// Let go of every drawing on the card.
    pub fn forget(&mut self) {
        self.store.clear();
        self.spare.clear();
    }

    /// `source` on the card, sending it first if it is not there. The view itself, not a place
    /// in the store, since sending one drawing can evict another this frame has not yet drawn.
    fn resident(&mut self, uploads: &mut wgpu::CommandEncoder, source: &Arc<WorkingBuffer>, name: Option<Name>, wide: bool) -> wgpu::TextureView {
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
        let (gone, kept): (Vec<_>, Vec<_>) =
            std::mem::take(&mut self.store).into_iter().partition(|s| s.name.is_none() && s.held.strong_count() == 0);
        self.store = kept;
        self.spare.extend(gone.into_iter().map(|s| s.view.texture().clone()));
        let (width, height) = (source.width(), source.height());
        let per_pixel = if wide { 16 } else { BYTES_PER_PIXEL };
        let bytes = width * height * per_pixel;
        let size = wgpu::Extent3d { width: width as u32, height: height as u32, depth_or_array_layers: 1 };
        let format = if wide { wgpu::TextureFormat::Rgba32Float } else { wgpu::TextureFormat::Rgba16Float };
        let spare = self.spare.iter().position(|t| t.size() == size && t.format() == format).map(|i| self.spare.remove(i));
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
                layout: wgpu::TexelCopyBufferLayout { offset, bytes_per_row: Some(row as u32), rows_per_image: None },
            },
            texture.as_image_copy(),
            size,
        );
        self.sent += 1;
        let view = texture.create_view(&Default::default());
        self.store.push(Stored { held: Arc::downgrade(source), name, view: view.clone(), bytes, used: self.frame, applied: None, wide });
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
            buffer.slice(..).map_async(wgpu::MapMode::Write, move |r| drop(tell.send(r)));
            let wait = s.after.clone().map_or(wgpu::PollType::Wait, wgpu::PollType::WaitForSubmissionIndex);
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
    fn applied(&mut self, steps: &mut Vec<Step>, source: &Arc<WorkingBuffer>, still: &wgpu::TextureView, effects: &[OnCard]) -> wgpu::TextureView {
        let stored = self.store.iter().position(|s| s.used == self.frame && &s.view == still);
        let mut views = Vec::new();
        if let Some((kept, after)) = stored.and_then(|i| self.store[i].applied.as_ref()) {
            let same = kept.iter().zip(effects).take_while(|(a, b)| a == b).count();
            views.extend_from_slice(&after[..same]);
        }
        if views.len() == effects.len() && !views.is_empty() {
            return views[views.len() - 1].0.clone();
        }
        let from = views.last().cloned().unwrap_or((still.clone(), (source.width(), source.height())));
        let done = views.len();
        self.run(steps, source, from, &effects[done..], &mut views);
        let moved = views[views.len() - 1].0.clone();
        if let Some(i) = stored {
            let s = &mut self.store[i];
            let kept: usize = views.iter().map(|(_, (w, h))| w * h * 16).sum();
            s.bytes = source.width() * source.height() * if s.wide { 16 } else { BYTES_PER_PIXEL } + kept;
            s.applied = Some((effects.to_vec(), views));
        }
        moved
    }

    /// `effects` run one after another from `moved`, a drawing `size`, each on what the one before
    /// it wrote; each result is added to `views`. `source` is the drawing on the CPU.
    fn run(
        &self,
        steps: &mut Vec<Step>,
        source: &WorkingBuffer,
        (mut moved, mut size): (wgpu::TextureView, (usize, usize)),
        effects: &[OnCard],
        views: &mut Vec<(wgpu::TextureView, (usize, usize))>,
    ) {
        for effect in effects {
            (moved, size) = match effect {
                OnCard::Radial(r) => (self.blur(steps, &moved, size, *r), size),
                OnCard::Bloom(b) => self.bloom(steps, &moved, size, *b),
                OnCard::Directional(d) => self.directional(steps, &moved, size, *d),
                OnCard::Gaussian(g) => self.gaussian(steps, &moved, size, *g),
                OnCard::Glow(g) => self.glow(steps, &moved, size, *g),
                OnCard::Fx(f) => self.fx(steps, source, size, &moved, f),
            };
            views.push((moved.clone(), size));
        }
    }

    /// A texture the passes write and the layers read, `width` by `height`. B-153: one an earlier
    /// frame used and nothing kept, cleared first, when there is one.
    fn scratch(&self, label: &str, width: usize, height: usize) -> wgpu::TextureView {
        let size = wgpu::Extent3d { width: width as u32, height: height as u32, depth_or_array_layers: 1 };
        if let Some(working) = &self.working {
            let mut working = working.borrow_mut();
            let free = working.iter_mut().find(|(t, used)| *used < self.frame && t.size() == size && !self.keeps(t));
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
        self.store.iter().any(|s| s.applied.as_ref().is_some_and(|(_, after)| after.iter().any(|(v, _)| v.texture() == texture)))
    }

    /// `blurs::radial_blur` of `still`, `width` by `height`, into a texture of its own, which is
    /// returned. What to dispatch is added to `steps`, to run before the layers.
    fn blur(&self, steps: &mut Vec<Step>, still: &wgpu::TextureView, (width, height): (usize, usize), r: Radial) -> wgpu::TextureView {
        let moved = self.scratch("B-46 radial", width, height);
        let turns: Vec<[f32; 2]> = crate::blurs::radial_turns(r.spin, r.amount)
            .into_iter()
            .flatten()
            .map(|(a, b)| [a as f32, b as f32])
            .collect();
        let init = |label, contents: &[u8], usage| {
            self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(label), contents, usage })
        };
        let f = |v: f64| (v as f32).to_bits();
        let settings = [f(r.center.0), f(r.center.1), f(r.amount), r.spin as u32, r.repeat as u32, 0, 0, 0];
        let settings = init("B-46 settings", bytemuck::cast_slice(&settings), wgpu::BufferUsages::UNIFORM);
        let turns = init("B-46 turns", bytemuck::cast_slice(&turns), wgpu::BufferUsages::STORAGE);
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.radial_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: settings.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(still) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(&moved) },
                wgpu::BindGroupEntry { binding: 3, resource: turns.as_entire_binding() },
            ],
        });
        steps.push((self.radial.clone(), group, ((width as u32).div_ceil(16), (height as u32).div_ceil(16))));
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
        let numbers = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("B-47 numbers"),
            contents: bytemuck::bytes_of(&p),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let mut entries = vec![
            wgpu::BindGroupEntry { binding: 0, resource: numbers.as_entire_binding() },
            wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(input) },
        ];
        if let Some(o) = output {
            entries.push(wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(o) });
        }
        if let Some(h) = halo {
            entries.push(wgpu::BindGroupEntry { binding: 3, resource: h.as_entire_binding() });
        }
        if let Some(w) = weights {
            entries.push(wgpu::BindGroupEntry { binding: 4, resource: w.as_entire_binding() });
        }
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout, entries: &entries });
        steps.push((pipeline.clone(), group, groups));
    }

    /// B-47: `bloom::bloom` of `still`, `width` by `height`, into a texture of its own, grown by
    /// the bloom's reach, which is returned with its size. What to dispatch is added to `steps`.
    fn bloom(&self, steps: &mut Vec<Step>, still: &wgpu::TextureView, (w, h): (usize, usize), b: Bloom) -> (wgpu::TextureView, (usize, usize)) {
        let passes = self.bloom.as_ref().expect("a Bloom is refused without the passes");
        let grow = crate::bloom::reach(b.radius, b.lines, b.length);
        let (gw, gh) = (w + 2 * grow, h + 2 * grow);
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        // The bright pass reads colours and the streak pass end taps, which a Bloom has none
        // of; a buffer cannot be empty.
        let none = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("B-47 none"),
            contents: bytemuck::cast_slice(&[0.0f32; 3]),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let light = self.scratch("B-47 light", w, h);
        let p = Params { level: crate::bloom::bright_level(b.threshold), ..Default::default() };
        self.step(steps, &passes.bright, p, still, Some(&light), None, Some(&none), tiles(w, h));
        // Made empty: wgpu clears every buffer it makes.
        let halo = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("B-47 halo"),
            size: (gw * gh * 16) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let add = Params { width: gw as u32, height: gh as u32, ..Default::default() };
        for s in crate::bloom::SCALES {
            let sigma = b.radius / 3.0 * s;
            let r = crate::effects::kernel_radius(sigma);
            let placed = Params { g: (grow - r) as i32, weight: 0.25, ..add };
            if r == 0 {
                self.step(steps, &passes.add, placed, &light, None, Some(&halo), None, tiles(gw, gh));
                continue;
            }
            let tall = self.blurred(steps, passes, "B-47", &light, (w, h), sigma);
            self.step(steps, &passes.add, placed, &tall, None, Some(&halo), None, tiles(gw, gh));
        }
        let share = 1.0 / b.lines as f32;
        for j in 0..b.lines {
            let u = crate::blurs::along(b.angle + j as f64 * 180.0 / b.lines as f64);
            if b.length == 0.0 {
                let p = Params { g: grow as i32, weight: share, ..add };
                self.step(steps, &passes.add, p, &light, None, Some(&halo), None, tiles(gw, gh));
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
            let p = Params { reach: wt.inner as u32, ..p };
            self.step(steps, &passes.streak, p, &light, Some(&lines), None, Some(&none), ((count as u32).div_ceil(32), 1));
            let p = Params { weight: share, width: gw as u32, height: gh as u32, ..p };
            self.step(steps, &passes.mix, p, &lines, None, Some(&halo), None, tiles(gw, gh));
        }
        let out = self.scratch("B-47 bloom", gw, gh);
        let p = Params { g: grow as i32, weight: b.intensity as f32, ..Default::default() };
        self.step(steps, &passes.combine, p, still, Some(&out), Some(&halo), None, tiles(gw, gh));
        (out, (gw, gh))
    }

    /// B-49: `blurs::directional_blur` of `still`, `width` by `height`, into a texture of its own,
    /// grown by half the length (D-109: or not at all), which is returned with its size. What to
    /// dispatch is added to `steps`.
    fn directional(&self, steps: &mut Vec<Step>, still: &wgpu::TextureView, (w, h): (usize, usize), d: Directional) -> (wgpu::TextureView, (usize, usize)) {
        let passes = self.bloom.as_ref().expect("a Directional Blur is refused without the passes");
        let grow = d.grow();
        let (gw, gh) = (w + 2 * grow, h + 2 * grow);
        let u = crate::blurs::along(d.direction);
        let wt = crate::blurs::directional_weights(u, d.length);
        let f = crate::blurs::line_frame(u, w, h, grow);
        let count = (f.k1 - f.k0 + 1) as usize;
        let lines = self.scratch("B-49 lines", f.ow, count);
        let ends: Vec<f32> = wt.ends.iter().flat_map(|&(j, w)| [j as f32, w as f32]).collect();
        let ends = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("B-49 ends"),
            contents: bytemuck::cast_slice(if ends.is_empty() { &[0.0f32; 2] } else { &ends[..] }),
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
        self.step(steps, &passes.streak, p, still, Some(&lines), None, Some(&ends), ((count as u32).div_ceil(32), 1));
        let out = self.scratch("B-49 directional", gw, gh);
        let p = Params { width: gw as u32, height: gh as u32, ..p };
        self.step(steps, &passes.lay, p, &lines, Some(&out), None, None, ((gw as u32).div_ceil(16), (gh as u32).div_ceil(16)));
        (out, (gw, gh))
    }

    /// B-50: `effects::blur` of `still`, `width` by `height`, across and then down with Bloom's
    /// `gauss` pass, into a texture grown by the kernel's radius (D-109: or not at all), which is
    /// returned with its size.
    fn gaussian(&self, steps: &mut Vec<Step>, still: &wgpu::TextureView, (w, h): (usize, usize), g: Gaussian) -> (wgpu::TextureView, (usize, usize)) {
        let passes = self.bloom.as_ref().expect("a Gaussian Blur is refused without the passes");
        let e = g.grow();
        // B-164: D-235's shortcut blurs transparent edges; held ones stay exact.
        let tall = if g.repeat {
            self.gauss(steps, passes, "B-50", still, (w, h), &crate::effects::gaussian_weights(g.sigma), e, true)
        } else {
            self.blurred(steps, passes, "B-50", still, (w, h), g.sigma)
        };
        (tall, (w + 2 * e, h + 2 * e))
    }

    /// `effects::convolve` across and then down with Bloom's `gauss` pass, into a texture grown
    /// by `e` (the kernel's radius; D-109: none with `held`), which is returned.
    #[allow(clippy::too_many_arguments)]
    fn gauss(&self, steps: &mut Vec<Step>, passes: &BloomPasses, label: &str, input: &wgpu::TextureView, (w, h): (usize, usize), taps: &[f32], e: usize, held: bool) -> wgpu::TextureView {
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        let weights = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some(&format!("{label} weights")),
            contents: bytemuck::cast_slice(taps),
            usage: wgpu::BufferUsages::STORAGE,
        });
        let (wide, tall) = (self.scratch(&format!("{label} wide"), w + 2 * e, h), self.scratch(&format!("{label} tall"), w + 2 * e, h + 2 * e));
        let across = Params { count: (taps.len() / 2) as u32, axis: 0, held: held as u32, ..Default::default() };
        self.step(steps, &passes.gauss, across, input, Some(&wide), None, Some(&weights), tiles(w + 2 * e, h));
        let down = Params { axis: 1, ..across };
        self.step(steps, &passes.gauss, down, &wide, Some(&tall), None, Some(&weights), tiles(w + 2 * e, h + 2 * e));
        tall
    }

    /// `effects::blur` of `input`, `width` by `height`, transparent outside it, into a texture
    /// grown by the kernel's radius, which is returned. B-164 (D-235, the viewer only): a big
    /// blur is worked on the picture shrunk by [`shrink_factor`] and enlarged back, which stays
    /// within 1 level of the exact blur.
    #[allow(clippy::too_many_arguments)]
    fn blurred(&self, steps: &mut Vec<Step>, passes: &BloomPasses, label: &str, input: &wgpu::TextureView, (w, h): (usize, usize), sigma: f64) -> wgpu::TextureView {
        let r = crate::effects::kernel_radius(sigma);
        let f = shrink_factor(sigma);
        if f == 1 {
            return self.gauss(steps, passes, label, input, (w, h), &crate::effects::gaussian_weights(sigma), r, false);
        }
        self.shrunk.set(self.shrunk.get() + 1);
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        let taps = small_taps(sigma, f);
        let rs = taps.len() / 2;
        // Room round the small picture, so the enlarged blur reaches the exact one's edge and
        // a pixel past it.
        let m = (r.div_ceil(f) + 1).saturating_sub(rs);
        let (sw, sh) = (w.div_ceil(f) + 2 * m, h.div_ceil(f) + 2 * m);
        let small = self.scratch(&format!("{label} small"), sw, sh);
        let p = Params { n: f as u32, g: m as i32, ..Default::default() };
        self.step(steps, &passes.shrink, p, input, Some(&small), None, None, tiles(sw, sh));
        let blurred = self.gauss(steps, passes, &format!("{label} small"), &small, (sw, sh), &taps, rs, false);
        let out = self.scratch(&format!("{label} tall"), w + 2 * r, h + 2 * r);
        let p = Params { n: f as u32, g: r as i32, count: (rs + m) as u32, ..Default::default() };
        self.step(steps, &passes.enlarge, p, &blurred, Some(&out), None, None, tiles(w + 2 * r, h + 2 * r));
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
            self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(label), contents, usage })
        };
        let numbers = init("B-65 numbers", bytemuck::bytes_of(&p), wgpu::BufferUsages::UNIFORM);
        let k = k.map(|k| init("B-65 settings", bytemuck::cast_slice(k), wgpu::BufferUsages::STORAGE));
        let mut entries = vec![wgpu::BindGroupEntry { binding: 0, resource: numbers.as_entire_binding() }];
        let textures = [(1, input), (2, output), (4, other)];
        entries.extend(textures.iter().filter_map(|&(binding, t)| {
            t.map(|t| wgpu::BindGroupEntry { binding, resource: wgpu::BindingResource::TextureView(t) })
        }));
        entries.extend(k.as_ref().map(|k| wgpu::BindGroupEntry { binding: 3, resource: k.as_entire_binding() }));
        entries.extend((5..).zip(work).filter_map(|(binding, b)| {
            b.map(|b| wgpu::BindGroupEntry { binding, resource: b.as_entire_binding() })
        }));
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor { label: None, layout, entries: &entries });
        steps.push((pipeline.clone(), group, groups));
    }

    /// B-65: one of the batch of ten, `apply_stack_at` of `f` on `still`, `w` by `h`, into a
    /// texture of its own, returned with its size. What to dispatch is added to `steps`.
    /// B-155: `source` is the drawing the layer sent, which Paraffin and Kira-kira read here;
    /// they only begin a run, so it is what they are given.
    fn fx(&self, steps: &mut Vec<Step>, source: &WorkingBuffer, (w, h): (usize, usize), still: &wgpu::TextureView, f: &Fx) -> (wgpu::TextureView, (usize, usize)) {
        use crate::effects::Effect as E;
        let passes = self.fx.as_ref().expect("the batch of ten is refused without the passes");
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
        let buffer = |bytes: usize| {
            self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("B-65 work"),
                size: bytes.max(4) as u64,
                usage: wgpu::BufferUsages::STORAGE,
                mapped_at_creation: false,
            })
        };
        // A pass the size of the drawing, on `still`.
        let same = |steps: &mut Vec<Step>, pass: &Pass, p: FxParams, k: &[f64], other: Option<&wgpu::TextureView>| {
            let out = self.scratch("B-65", w, h);
            self.fx_step(steps, pass, p, Some(still), Some(&out), Some(k), other, none, tiles(w, h));
            (out, (w, h))
        };
        // The drawing's covering blurred at `sigma`, as `effects::blur`, and the blur's radius.
        let covering = |steps: &mut Vec<Step>, input: &wgpu::TextureView, size, sigma: f64| {
            let (view, _) = self.gaussian(steps, input, size, Gaussian { sigma, repeat: false });
            (view, crate::effects::kernel_radius(sigma))
        };
        let (ox, oy) = f.origin;
        match &f.instance.effect {
            E::Curves { master, red, green, blue } => {
                // Where each curve starts, then each curve's count, ins, outs and second derivatives.
                let mut k = vec![0.0; 4];
                for (i, c) in [red, green, blue, master].into_iter().enumerate() {
                    k[i] = k.len() as f64;
                    let (xs, ys, m) = crate::grade::knots(c);
                    k.push(xs.len() as f64);
                    k.extend(xs.into_iter().chain(ys).chain(m));
                }
                same(steps, &passes.grade, FxParams { mode: 0, ..Default::default() }, &k, None)
            }
            E::Levels { input_black, input_white, gamma, output_black, output_white } => {
                let k = [*input_black, *input_white, 1.0 / gamma, *output_black, *output_white];
                same(steps, &passes.grade, FxParams { mode: 1, ..Default::default() }, &k, None)
            }
            E::HueSaturation { hue, saturation, lightness } => {
                same(steps, &passes.grade, FxParams { mode: 2, ..Default::default() }, &[*hue, *saturation, *lightness], None)
            }
            E::Gradient { shape, start, end, start_color, end_color, start_opacity, end_opacity, blend: b } => {
                let (sx, sy) = crate::effects::radial_center(*start, (w, h), f.origin);
                let (ex, ey) = crate::effects::radial_center(*end, (w, h), f.origin);
                let (ex, ey) = (ex - sx, ey - sy);
                let ll = ex * ex + ey * ey;
                let mut k = vec![sx, sy, ex, ey, ll, ll.sqrt(), (shape == "radial") as u8 as f64, *start_opacity, *end_opacity];
                k.extend(crate::effects::encoded(start_color));
                k.extend(crate::effects::encoded(end_color));
                same(steps, &passes.grade, FxParams { mode: 3, blend: blend(b), ..Default::default() }, &k, None)
            }
            E::Noise { amount, mode, seed, frame, .. } => {
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
            E::ChromaticAberration { amount, center } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let (w0, h0) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let k = amount / ((w0 * w0 + h0 * h0).sqrt() / 2.0);
                same(steps, &passes.aberr, FxParams::default(), &[cx, cy, 1.0 - k, 1.0 + k], None)
            }
            E::RimLight { color, direction, width, softness, intensity, blend: b } => {
                let (blurred, r) = covering(steps, still, (w, h), softness / 3.0);
                let (ux, uy) = crate::blurs::along(*direction);
                let mut k = vec![width * ux, width * uy, intensity / 100.0];
                k.extend(linear(color));
                let p = FxParams { r: r as i32, blend: blend(b), ..Default::default() };
                same(steps, &passes.rim, p, &k, Some(&blurred))
            }
            E::DropShadow { color, opacity, direction, distance, softness } => {
                let (blurred, r) = covering(steps, still, (w, h), softness / 3.0);
                let g = distance.ceil() as usize + r;
                let (ux, uy) = crate::blurs::along(*direction);
                let mut k = vec![distance * ux, distance * uy, opacity / 100.0];
                k.extend(linear(color));
                let out = self.scratch("B-65 shadow", w + 2 * g, h + 2 * g);
                let p = FxParams { g: g as i32, r: r as i32, ..Default::default() };
                self.fx_step(steps, &passes.shadow, p, Some(still), Some(&out), Some(&k), Some(&blurred), none, tiles(w + 2 * g, h + 2 * g));
                (out, (w + 2 * g, h + 2 * g))
            }
            E::Outline { color, width, softness, opacity } => {
                let n = width.ceil() as usize;
                let (bw, bh) = (w + 2 * n, h + 2 * n);
                let runs = crate::layer_fx::disc_runs(*width);
                let (row, band) = (buffer(h * bw * 4), buffer(bw * bh * 4));
                let top = runs.iter().map(|&(_, hw)| hw).max().unwrap_or(0);
                for k in 0..=top {
                    // Each row's greatest covering within k, widened from within k - 1.
                    let p = FxParams { n: n as u32, count: k as u32, ..Default::default() };
                    self.fx_step(steps, &passes.rows, p, Some(still), None, None, None, [Some(&row), None, None, None], tiles(bw, h));
                    let dys: Vec<f64> = runs.iter().filter(|&&(_, hw)| hw == k).map(|&(dy, _)| dy as f64).collect();
                    if !dys.is_empty() {
                        let p = FxParams { n: n as u32, count: dys.len() as u32, ..Default::default() };
                        self.fx_step(steps, &passes.bands, p, Some(still), None, Some(&dys), None, [Some(&row), Some(&band), None, None], tiles(bw, bh));
                    }
                }
                let lifted = self.scratch("B-65 band", bw, bh);
                self.fx_step(steps, &passes.lift, FxParams::default(), None, Some(&lifted), None, None, [None, Some(&band), None, None], tiles(bw, bh));
                let (blurred, r) = covering(steps, &lifted, (bw, bh), softness / 3.0);
                let g = n + r;
                let mut k = vec![opacity / 100.0];
                k.extend(linear(color));
                let out = self.scratch("B-65 outline", w + 2 * g, h + 2 * g);
                let p = FxParams { g: g as i32, ..Default::default() };
                self.fx_step(steps, &passes.ring, p, Some(still), Some(&out), Some(&k), Some(&blurred), none, tiles(w + 2 * g, h + 2 * g));
                (out, (w + 2 * g, h + 2 * g))
            }
            E::LensBlur { radius, edges, iris, roundness, rotation, aspect, highlight_gain, highlight_threshold } => {
                let repeat = edges == "repeat";
                let g = if repeat { 0 } else { crate::layer_fx::lens_reach(*radius, *aspect).ceil() as usize };
                let blades = crate::layer_fx::blades(iris).unwrap_or(0);
                let runs = crate::layer_fx::iris_runs(*radius, blades, *roundness, *rotation, *aspect);
                let n: isize = runs.iter().map(|&[_, lo, hi]| hi - lo + 1).sum();
                // The threshold and the gain as the CPU rounds them to single precision.
                let (at, m) = ((highlight_threshold / 100.0) as f32 as f64, (1.0 + highlight_gain) as f32 as f64);
                let mut k = vec![n as f64, at, m];
                k.extend(runs.iter().flatten().map(|&v| v as f64));
                let sums = buffer((w + 1) * h * 32);
                let p = FxParams { mode: repeat as u32, flag: (*highlight_gain > 0.0) as u32, count: runs.len() as u32, g: g as i32, ..Default::default() };
                self.fx_step(steps, &passes.prefix, p, Some(still), None, Some(&k), None, [None, None, Some(&sums), None], ((h as u32).div_ceil(64), 1));
                let out = self.scratch("B-65 lens", w + 2 * g, h + 2 * g);
                self.fx_step(steps, &passes.gather, p, Some(still), Some(&out), Some(&k), None, [None, None, Some(&sums), None], tiles(w + 2 * g, h + 2 * g));
                (out, (w + 2 * g, h + 2 * g))
            }
            // B-76: the second batch.
            E::ExposureFlicker { amount, hold, seed, frame } => {
                let gain = 2f64.powf(crate::effects::flicker_stops(*amount, *hold, *seed, *frame)) as f32;
                same(steps, &passes.grade, FxParams { mode: 5, ..Default::default() }, &[gain as f64], None)
            }
            E::ColorBalance { shadows, midtones, highlights } => {
                let k: Vec<f64> = [shadows, midtones, highlights].iter().flat_map(|t| t[..3].to_vec()).collect();
                same(steps, &passes.grade, FxParams { mode: 6, ..Default::default() }, &k, None)
            }
            E::GradientMap { shadow_color, midtone_color, highlight_color, midpoint, amount } => {
                let mut k = vec![midpoint / 100.0, amount / 100.0];
                for c in [shadow_color, midtone_color, highlight_color] {
                    k.extend(crate::effects::encoded(c));
                }
                same(steps, &passes.grade, FxParams { mode: 7, ..Default::default() }, &k, None)
            }
            E::Vignette { amount, color, size, roundness, softness, center } => {
                let v = crate::effects::vignette_settings(*amount, color, [*size, *roundness, *softness], *center, (w, h), f.origin);
                let mut k = vec![v.center.0, v.center.1, v.radii.0, v.radii.1, v.inner, v.outer, v.amount, 2f64.sqrt()];
                k.extend(v.color.map(crate::grade::to_linear));
                same(steps, &passes.grade, FxParams { mode: 8, ..Default::default() }, &k, None)
            }
            E::FractalNoise { size, complexity, contrast, brightness, evolution, speed, seed, dark_color, light_color, opacity, blend: b, frame } => {
                let base = crate::grade::mix(seed.floor() as u64);
                let mut k = vec![*size, crate::effects::depth(*evolution, *speed, *frame), *contrast, *brightness, opacity / 100.0];
                k.extend(crate::effects::encoded(dark_color));
                k.extend(crate::effects::encoded(light_color));
                let p = FxParams {
                    mode: 9,
                    blend: blend(b),
                    count: complexity.floor() as u32,
                    base: [base as u32, (base >> 32) as u32],
                    ox: ox as i32,
                    oy: oy as i32,
                    ..Default::default()
                };
                same(steps, &passes.grade, p, &k, None)
            }
            E::TurbulentDisplace { amount, size, complexity, evolution, speed, seed, edges, frame } => {
                let repeat = edges == "repeat";
                let g = if repeat { 0 } else { amount.ceil() as usize };
                let base = crate::grade::mix(seed.floor() as u64);
                let k = [*amount, *size, crate::effects::depth(*evolution, *speed, *frame), (ox + g) as f64, (oy + g) as f64];
                let p = FxParams {
                    count: complexity.floor() as u32,
                    flag: repeat as u32,
                    g: g as i32,
                    base: [base as u32, (base >> 32) as u32],
                    ..Default::default()
                };
                let out = self.scratch("B-76 turbulent", w + 2 * g, h + 2 * g);
                self.fx_step(steps, &passes.turb, p, Some(still), Some(&out), Some(&k), None, none, tiles(w + 2 * g, h + 2 * g));
                (out, (w + 2 * g, h + 2 * g))
            }
            E::Offset { shift } => {
                let (qx, qy) = (-shift[0], -shift[1]);
                let (i0, j0) = (qx.floor(), qy.floor());
                // Whole turns of the drawing change nothing, and keep the sums small.
                let inside = |v: f64, n: usize| (v as i64).rem_euclid(n as i64) as i32;
                let p = FxParams { ox: inside(i0, w), oy: inside(j0, h), ..Default::default() };
                same(steps, &passes.slide, p, &[qx - i0, qy - j0], None)
            }
            E::LightRays { center, length, threshold, intensity, color } => {
                let bloom = self.bloom.as_ref().expect("Light Rays are refused without the passes");
                let none = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("B-76 none"),
                    contents: bytemuck::cast_slice(&[0.0f32; 3]),
                    usage: wgpu::BufferUsages::STORAGE,
                });
                let lit = self.scratch("B-76 bright", w, h);
                let p = Params { level: crate::bloom::bright_level(*threshold), ..Default::default() };
                self.step(steps, &bloom.bright, p, still, Some(&lit), None, Some(&none), tiles(w, h));
                let center = crate::effects::radial_center(*center, (w, h), f.origin);
                let rays = self.blur(steps, &lit, (w, h), Radial { spin: false, amount: *length, center, repeat: false });
                let mut k = vec![*intensity];
                k.extend(linear(color));
                same(steps, &passes.rays, FxParams::default(), &k, Some(&rays))
            }
            E::DistanceGradation { color, width, opacity, invert, blend: b } => {
                let dist = buffer(dist_bytes(w, h));
                let mut k = vec![*width, *opacity, (invert == "on") as u8 as f64];
                k.extend(linear(color));
                let p = FxParams { blend: blend(b), ..Default::default() };
                let work = [None, None, None, Some(&dist)];
                self.fx_step(steps, &passes.cols, p, Some(still), None, None, None, work, (((w + 2) as u32).div_ceil(64), 1));
                let out = self.scratch("B-76 distance", w, h);
                self.fx_step(steps, &passes.edt, p, Some(still), Some(&out), Some(&k), None, work, ((h as u32).div_ceil(64), 1));
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
                same(steps, &passes.tone, FxParams { mode, count, ..Default::default() }, &[amount / 100.0], None)
            }
            E::BrightnessContrast { brightness, contrast } => {
                let k = if *contrast <= 0.0 { 1.0 + contrast / 100.0 } else { 1.0 / (1.0 - 0.99 * contrast / 100.0) };
                same(steps, &passes.tone, FxParams { mode: 2, ..Default::default() }, &[k, brightness / 255.0], None)
            }
            E::BlackWhite { reds, yellows, greens, cyans, blues, magentas } => {
                let k = [*reds, *yellows, *greens, *cyans, *blues, *magentas];
                same(steps, &passes.tone, FxParams { mode: 3, ..Default::default() }, &k, None)
            }
            E::Posterize { levels } => same(steps, &passes.tone, FxParams { mode: 4, ..Default::default() }, &[levels.floor()], None),
            E::Threshold { level } => same(steps, &passes.tone, FxParams { mode: 5, ..Default::default() }, &[*level], None),
            E::ChannelMixer { red, green, blue, monochrome } => {
                let rows = if monochrome == "on" { [red, red, red] } else { [red, green, blue] };
                let k: Vec<f64> = rows.iter().flat_map(|r| r[..4].to_vec()).collect();
                same(steps, &passes.tone, FxParams { mode: 6, ..Default::default() }, &k, None)
            }
            E::Vibrance { vibrance, saturation } => {
                same(steps, &passes.tone, FxParams { mode: 7, ..Default::default() }, &[*vibrance, *saturation], None)
            }
            E::LeaveColor { color, tolerance, softness, amount } => {
                let hc = crate::grade::hsv_hue(crate::effects::encoded(color)).unwrap_or(-1.0);
                let k = [hc, tolerance / 100.0, softness / 100.0, amount / 100.0];
                same(steps, &passes.tone, FxParams { mode: 8, ..Default::default() }, &k, None)
            }
            E::Solarize { threshold } => same(steps, &passes.tone, FxParams { mode: 9, ..Default::default() }, &[*threshold], None),
            E::Halftone { size, angle, ink, paper, amount } => {
                let (sin, cos) = angle.to_radians().sin_cos();
                let mut k = vec![sin, cos, *size, amount / 100.0];
                k.extend(linear(ink));
                k.extend(linear(paper));
                let p = FxParams { mode: 10, ox: ox as i32, oy: oy as i32, ..Default::default() };
                same(steps, &passes.tone, p, &k, None)
            }
            E::Mosaic { size } => {
                // Where each run of columns, or of rows, in one block starts, the end last, and
                // each column's or row's run, as layer_fx::mosaic cuts them.
                let runs = |n: usize, o: usize| {
                    let block = |p: usize| ((p as f64 - o as f64) / size).floor();
                    let (mut starts, mut of) = (vec![0.0], Vec::with_capacity(n));
                    for p in 0..n {
                        if p > 0 && block(p) != block(starts[starts.len() - 1] as usize) {
                            starts.push(p as f64);
                        }
                        of.push((starts.len() - 1) as f64);
                    }
                    starts.push(n as f64);
                    (starts, of)
                };
                let ((cs, col_of), (rs, row_of)) = (runs(w, ox), runs(h, oy));
                let (nc, nr) = (cs.len() - 1, rs.len() - 1);
                let mut k = vec![nc as f64, nr as f64];
                k.extend(cs.into_iter().chain(rs).chain(col_of).chain(row_of));
                let means = buffer(16 * nc * nr);
                self.fx_step(steps, &passes.blocks, FxParams::default(), Some(still), None, Some(&k), None, [Some(&means), None, None, None], tiles(nc, nr));
                let out = self.scratch("B-107 mosaic", w, h);
                self.fx_step(steps, &passes.tiles, FxParams::default(), Some(still), Some(&out), Some(&k), None, [Some(&means), None, None, None], tiles(w, h));
                (out, (w, h))
            }
            E::Emboss { direction, relief, contrast, mode } => {
                let (ux, uy) = crate::blurs::along(*direction);
                let k = [relief * ux, relief * uy, contrast / 100.0, (mode == "color") as u8 as f64];
                same(steps, &passes.relief, FxParams::default(), &k, None)
            }
            E::FindEdges { invert, amount } => {
                let k = [amount / 100.0, (invert == "on") as u8 as f64];
                same(steps, &passes.relief, FxParams { mode: 1, ..Default::default() }, &k, None)
            }
            E::Sharpen { amount, radius } => {
                let (blurred, r) = covering(steps, still, (w, h), *radius);
                same(steps, &passes.sharp, FxParams { r: r as i32, ..Default::default() }, &[amount / 100.0], Some(&blurred))
            }
            E::Diffusion { radius, amount, blend: b } => {
                let (blurred, r) = covering(steps, still, (w, h), radius / 3.0);
                let blend = match b.as_str() {
                    "screen" => 1,
                    "lighten" => 2,
                    _ => 0,
                };
                same(steps, &passes.sharp, FxParams { mode: 1, blend, r: r as i32, ..Default::default() }, &[amount / 100.0], Some(&blurred))
            }
            E::WaveWarp { shape, height, width, direction, speed, phase, edges, frame } => {
                let repeat = edges == "repeat";
                let g = if repeat { 0 } else { height.ceil() as usize };
                let (tx, ty) = crate::blurs::along(*direction);
                let phi = (phase + speed * *frame as f64).to_radians();
                let k = [(ox + g) as f64, (oy + g) as f64, tx, ty, -ty, tx, *width, phi, *height, (shape == "triangle") as u8 as f64];
                let out = self.scratch("B-107 wave", w + 2 * g, h + 2 * g);
                let p = FxParams { flag: repeat as u32, g: g as i32, ..Default::default() };
                self.fx_step(steps, &passes.warp, p, Some(still), Some(&out), Some(&k), None, none, tiles(w + 2 * g, h + 2 * g));
                (out, (w + 2 * g, h + 2 * g))
            }
            E::Ripple { center, amplitude, wavelength, speed, phase, fade, frame } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let k = [cx, cy, *amplitude, *wavelength, (phase + speed * *frame as f64).to_radians(), *fade];
                same(steps, &passes.warp, FxParams { mode: 1, ..Default::default() }, &k, None)
            }
            E::Twirl { angle, radius, center } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                same(steps, &passes.warp, FxParams { mode: 2, ..Default::default() }, &[cx, cy, angle.to_radians(), *radius], None)
            }
            E::Bulge { center, radius, height } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                same(steps, &passes.warp, FxParams { mode: 3, ..Default::default() }, &[cx, cy, *height, *radius], None)
            }
            E::Mirror { center, angle } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let (nx, ny) = crate::layer_fx::mirror_normal(*angle);
                same(steps, &passes.warp, FxParams { mode: 4, ..Default::default() }, &[cx, cy, nx, ny, 0.0], None)
            }
            E::CameraShake { amount, rotation, hold, seed, frame } => {
                let ((cx, cy), g) = crate::layer_fx::shake_reach(*amount, *rotation, (w, h), f.origin);
                let (dx, dy, sb, cb) = crate::layer_fx::shake_jolt([*amount, *rotation, *hold, *seed], *frame);
                let out = self.scratch("B-107 shake", w + 2 * g, h + 2 * g);
                let p = FxParams { mode: 5, g: g as i32, ..Default::default() };
                self.fx_step(steps, &passes.warp, p, Some(still), Some(&out), Some(&[cx, cy, dx, dy, sb, cb]), None, none, tiles(w + 2 * g, h + 2 * g));
                (out, (w + 2 * g, h + 2 * g))
            }
            E::MotionTile { mirror, .. } => {
                let (gx, gy) = f.grow;
                let (tw, th) = (w + 2 * gx, h + 2 * gy);
                let out = self.scratch("B-115 tile", tw, th);
                let p = FxParams { mode: 6, flag: (mirror == "on") as u32, ox: gx as i32, oy: gy as i32, ..Default::default() };
                self.fx_step(steps, &passes.warp, p, Some(still), Some(&out), Some(&[0.0]), None, none, tiles(tw, th));
                (out, (tw, th))
            }
            E::LinearWipe { completion, angle, feather } => {
                let ((ux, uy), edge) = crate::layer_fx::linear_edge(*completion, *angle, *feather, (w, h), f.origin);
                let p = FxParams { mode: 0, flag: (*completion == 100.0) as u32, ox: ox as i32, oy: oy as i32, ..Default::default() };
                same(steps, &passes.wipe, p, &[ux, uy, edge, *feather, 0.0], None)
            }
            E::RadialWipe { completion, start_angle, center, wipe, feather } => {
                let way = match wipe.as_str() {
                    "counterclockwise" => 1,
                    "both" => 2,
                    _ => 0,
                };
                let (w0, h0) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let edge = completion / 100.0 * (360.0 + feather) - feather / 2.0;
                let k = [center[0] / 100.0 * w0, center[1] / 100.0 * h0, *start_angle, edge, *feather];
                let p = FxParams { mode: 1, count: way, flag: (*completion == 100.0) as u32, ox: ox as i32, oy: oy as i32, ..Default::default() };
                same(steps, &passes.wipe, p, &k, None)
            }
            E::VenetianBlinds { completion, angle, width, feather } => {
                let (ux, uy) = crate::blurs::along(*angle);
                let edge = completion / 100.0 * (width + feather) - feather / 2.0;
                let p = FxParams { mode: 2, flag: (*completion == 100.0) as u32, ox: ox as i32, oy: oy as i32, ..Default::default() };
                same(steps, &passes.wipe, p, &[ux, uy, *width, edge, *feather, 0.0], None)
            }
            E::IrisWipe { completion, center, feather, invert } => {
                let invert = invert == "on";
                let ((cx, cy), r) = crate::layer_fx::iris_circle(*completion, *center, *feather, invert, (w, h), f.origin);
                let p = FxParams { mode: 3, flag: (*completion == 100.0) as u32, ox: ox as i32, oy: oy as i32, ..Default::default() };
                same(steps, &passes.wipe, p, &[cx, cy, r, *feather, invert as u8 as f64], None)
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
                    let p = FxParams { n: g as u32, count: k as u32, flag: !spread as u32, ..Default::default() };
                    self.fx_step(steps, &passes.rows, p, Some(still), None, None, None, [Some(&row), None, None, None], tiles(bw, h));
                    let dys: Vec<f64> = runs.iter().filter(|&&(_, hw)| hw == k).map(|&(dy, _)| dy as f64).collect();
                    if !dys.is_empty() {
                        // A shrink's band starts from infinity, a spread's from the empty buffer.
                        let mode = (!spread && first) as u32;
                        first = false;
                        let p = FxParams { mode, n: g as u32, count: dys.len() as u32, flag: !spread as u32, ..Default::default() };
                        self.fx_step(steps, &passes.bands, p, Some(still), None, Some(&dys), None, [Some(&row), Some(&band), None, None], tiles(bw, bh));
                    }
                }
                let sums = buffer(if spread { (w + 1) * h * 32 } else { 32 });
                if spread {
                    self.fx_step(steps, &passes.prefix, FxParams::default(), Some(still), None, Some(&[0.0; 3]), None, [None, None, Some(&sums), None], ((h as u32).div_ceil(64), 1));
                }
                let k: Vec<f64> = runs.iter().flat_map(|&(dy, hw)| [dy as f64, hw as f64]).collect();
                let out = self.scratch("B-107 choker", bw, bh);
                let p = FxParams { flag: !spread as u32, g: g as i32, count: runs.len() as u32, ..Default::default() };
                self.fx_step(steps, &passes.choke, p, Some(still), Some(&out), Some(&k), None, [None, Some(&band), Some(&sums), None], tiles(bw, bh));
                (out, (bw, bh))
            }
            E::SpeedLines { center, color, count, thickness, inner, inner_jitter, angle_jitter, seed, hold, opacity, frame } => {
                let (cx, cy) = crate::effects::radial_center(*center, (w, h), f.origin);
                let lines = crate::layer_fx::speed_line_list([*count, *thickness, *inner, *inner_jitter, *angle_jitter, *seed, *hold], *frame);
                let widest = lines.iter().map(|l| l.1).fold(0.0, f64::max);
                let mut k = vec![cx, cy, opacity / 100.0];
                k.extend(linear(color));
                k.extend([widest, lines.len() as f64]);
                k.extend(lines.iter().flat_map(|&(t, h, r)| [t, h, r]));
                same(steps, &passes.lines, FxParams::default(), &k, None)
            }
            E::CrossGlare { threshold, length, points, angle, intensity, color } => {
                let bloom = self.bloom.as_ref().expect("a Cross Glare is refused without the passes");
                let empty = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                    label: Some("B-107 none"),
                    contents: bytemuck::cast_slice(&[0.0f32; 3]),
                    usage: wgpu::BufferUsages::STORAGE,
                });
                let lit = self.scratch("B-107 bright", w, h);
                let p = Params { level: crate::bloom::bright_level(*threshold), ..Default::default() };
                self.step(steps, &bloom.bright, p, still, Some(&lit), None, Some(&empty), tiles(w, h));
                let l = length.floor() as usize;
                let n = points.floor();
                let mut k = vec![*intensity];
                k.extend(linear(color));
                k.push(n);
                k.extend((0..n as usize).flat_map(|j| {
                    let (vx, vy) = crate::blurs::along(angle + 360.0 * j as f64 / n);
                    [vx, vy]
                }));
                let fade: Vec<f64> = (1..=l).map(|t| (1.0 - t as f64 / (l + 1) as f64).powi(2)).collect();
                let total: f64 = fade.iter().sum();
                k.extend(fade.iter().enumerate().flat_map(|(t, f)| [(t + 1) as f64, f / total]));
                let out = self.scratch("B-107 glare", w + 2 * l, h + 2 * l);
                let p = FxParams { g: l as i32, ..Default::default() };
                self.fx_step(steps, &passes.glare, p, Some(still), Some(&out), Some(&k), Some(&lit), none, tiles(w + 2 * l, h + 2 * l));
                (out, (w + 2 * l, h + 2 * l))
            }
            E::Rain { color, density, spacing, length, width, direction, speed, seed, opacity, frame } => {
                let (tx, ty) = crate::blurs::along(*direction);
                let base = crate::grade::mix(seed.floor() as u64);
                let mut k = vec![tx, ty, -ty, tx, width / 2.0 + 0.5, length / 2.0, speed * *frame as f64, *spacing, density / 100.0, width / 2.0, *opacity];
                k.extend(linear(color));
                let p = FxParams { base: [base as u32, (base >> 32) as u32], ox: ox as i32, oy: oy as i32, ..Default::default() };
                same(steps, &passes.rain, p, &k, None)
            }
            E::ColorLookup { table, .. } => {
                let table = table.as_ref().expect("compose leaves a Color Lookup only with its table");
                same(steps, &passes.tone, FxParams { mode: 11, ..Default::default() }, &table.0.packed(), None)
            }
            E::HsvKey { hue, saturation, value, hue_range, saturation_range, value_range, invert } => {
                // The single-precision power 1/2.4 the CPU's encoding uses, less the true one.
                let power = f64::from(1.0f32 / 2.4) - 1.0 / 2.4;
                let k = [*hue, *hue_range, *saturation, *saturation_range, *value, *value_range, (invert == "on") as u8 as f64, power, 0.0];
                same(steps, &passes.tone, FxParams { mode: 12, ..Default::default() }, &k, None)
            }
            E::Paraffin { color, direction, spread, opacity, blend: b } => {
                // No pixel covered half or more: nothing is washed.
                let (span, share) = match crate::grade::paraffin_span(source, *direction, *spread) {
                    Some(span) => (span, opacity / 100.0),
                    None => ([0.0, 0.0, 0.0, 1.0], 0.0),
                };
                let mut k = span.to_vec();
                k.push(share);
                k.extend(crate::effects::encoded(color));
                same(steps, &passes.tone, FxParams { mode: 13, blend: blend(b), ..Default::default() }, &k, None)
            }
            E::LineBlur { length, strength, lines_only } => {
                let g = crate::line_blur::GROW;
                let weights = crate::line_blur::weights(*length);
                let mut k = vec![strength / 100.0, (lines_only == "on") as u8 as f64];
                k.extend(&weights);
                let out = self.scratch("B-123 line blur", w + 2 * g, h + 2 * g);
                let p = FxParams { g: g as i32, count: weights.len() as u32, ..Default::default() };
                self.fx_step(steps, &passes.line_blur, p, Some(still), Some(&out), Some(&k), None, none, tiles(w + 2 * g, h + 2 * g));
                (out, (w + 2 * g, h + 2 * g))
            }
            E::KiraKira { threshold, spacing, density, size, shape, angle, twinkle, period, seed, opacity, color, frame } => {
                let g = f.grow.0;
                let stars = crate::layer_fx::kira_stars(source, [*threshold, *spacing, *density, *size, *twinkle, *period, *seed], *frame, f.origin);
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
                let p = FxParams { g: g as i32, count: n, ..Default::default() };
                if n > 0 {
                    self.fx_step(steps, &passes.stars, p, Some(still), None, Some(&k), None, [None, None, None, None, Some(&lit)], (n.min(65535), n.div_ceil(65535)));
                }
                let out = self.scratch("B-123 kira", tw, th);
                self.fx_step(steps, &passes.kira, p, Some(still), Some(&out), Some(&k), None, [None, None, None, None, Some(&lit)], tiles(tw, th));
                (out, (tw, th))
            }
            // B-151: ten of the fourth batch.
            E::Median { radius, operate_on_alpha } => {
                let k: Vec<f64> = crate::layer_fx::disc_runs(*radius).iter().map(|&(_, hw)| hw as f64).collect();
                let p = FxParams { r: radius.floor() as i32, flag: (operate_on_alpha == "on") as u32, ..Default::default() };
                same(steps, &passes.median, p, &k, None)
            }
            E::SmartBlur { radius, threshold } => {
                // The threshold, then level8's power and 0, then the disc's rows.
                let power = f64::from(1.0f32 / 2.4) - 1.0 / 2.4;
                let mut k = vec![*threshold, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, power, 0.0];
                k.extend(crate::layer_fx::disc_runs(*radius).iter().map(|&(_, hw)| hw as f64));
                let p = FxParams { r: radius.floor() as i32, n: 9, ..Default::default() };
                let (levels, _) = same(steps, &passes.smart, p, &k, Some(still));
                same(steps, &passes.smart, FxParams { mode: 1, ..p }, &k, Some(&levels))
            }
            E::RoughenEdges { edge_type, edge_color, border, size, complexity, evolution, speed, seed, frame } => {
                let base = crate::grade::mix(seed.floor() as u64);
                let mut k = vec![*border, *size, crate::effects::depth(*evolution, *speed, *frame), (edge_type == "roughen_color") as u8 as f64];
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
            E::RadialShadow { color, opacity, light, distance, softness, render, color_influence, shadow_only } => {
                // The drawing cast from the light, blurred, then laid behind the drawing.
                let (lx, ly, scale, gx, gy) = crate::layer_fx::radial_cast(*light, *distance, (w, h), f.origin);
                let (cw, ch) = (w + 2 * gx, h + 2 * gy);
                let cast = self.scratch("B-151 cast", cw, ch);
                let k = [lx, ly, scale, gx as f64, gy as f64];
                self.fx_step(steps, &passes.warp, FxParams { mode: 11, ..Default::default() }, Some(still), Some(&cast), Some(&k), None, none, tiles(cw, ch));
                let (blurred, (bw, bh)) = self.gaussian(steps, &cast, (cw, ch), Gaussian { sigma: softness / 3.0, repeat: false });
                let r = (bw - cw) / 2;
                let mut k = vec![opacity / 100.0, color_influence / 100.0, (render == "glass_edge") as u8 as f64, (shadow_only == "on") as u8 as f64];
                k.extend(linear(color));
                let out = self.scratch("B-151 radial shadow", bw, bh);
                let p = FxParams { ox: (gx + r) as i32, oy: (gy + r) as i32, ..Default::default() };
                self.fx_step(steps, &passes.rshadow, p, Some(&blurred), Some(&out), Some(&k), Some(still), none, tiles(bw, bh));
                (out, (bw, bh))
            }
            E::BevelAlpha { edge_thickness, light_angle, light_color, light_intensity } => {
                let (blurred, r) = covering(steps, still, (w, h), edge_thickness / 2.0);
                let (ux, uy) = crate::blurs::along(*light_angle);
                let mut k = vec![*edge_thickness, ux, uy, *light_intensity];
                k.extend(linear(light_color));
                same(steps, &passes.bevel, FxParams { r: r as i32, ..Default::default() }, &k, Some(&blurred))
            }
            E::Snowfall { color, density, spacing, size, depth, speed, wind, wiggle, period, seed, opacity, frame } => {
                let base = crate::grade::mix(seed.floor() as u64);
                let mut k = vec![density / 100.0, *opacity, *period, *frame as f64];
                k.extend(crate::layer_fx::snow_planes([*spacing, *size, *depth, *speed, *wind, *wiggle], *frame).iter().flatten());
                k.extend(linear(color));
                let p = FxParams { base: [base as u32, (base >> 32) as u32], ox: ox as i32, oy: oy as i32, ..Default::default() };
                same(steps, &passes.snow, p, &k, None)
            }
            E::CellPattern { pattern, invert, contrast, disperse, size, evolution, seed, dark_color, light_color, opacity, blend: b } => {
                let (m0, n0, cols, points) = crate::grade::cell_points([*disperse, *size, *evolution, *seed], (w, h), f.origin);
                let code = ["bubbles", "crystals", "plates"].iter().position(|p| *p == pattern.as_str()).unwrap_or(3);
                let mut k = vec![*size, *contrast, opacity / 100.0, code as f64, (invert == "on") as u8 as f64, m0 as f64, n0 as f64, cols as f64];
                k.extend(crate::effects::encoded(dark_color));
                k.extend(crate::effects::encoded(light_color));
                k.push(0.0);
                k.extend(points.iter().flat_map(|&(x, y, grey)| [x, y, grey]));
                same(steps, &passes.cells, FxParams { blend: blend(b), ox: ox as i32, oy: oy as i32, ..Default::default() }, &k, None)
            }
            E::PolarCoordinates { interpolation, conversion } => {
                let k = [interpolation / 100.0, (conversion == "rect_to_polar") as u8 as f64, (w - 2 * ox) as f64, (h - 2 * oy) as f64, ox as f64, oy as f64];
                same(steps, &passes.warp, FxParams { mode: 8, ..Default::default() }, &k, None)
            }
            E::OpticsCompensation { field_of_view, reverse, orientation, center } => {
                let (dw, dh) = ((w - 2 * ox) as f64, (h - 2 * oy) as f64);
                let r = match orientation.as_str() {
                    "vertical" => dh / 2.0,
                    "diagonal" => dw.hypot(dh) / 2.0,
                    _ => dw / 2.0,
                };
                let (cx, cy) = (center[0] / 100.0 * dw + ox as f64, center[1] / 100.0 * dh + oy as f64);
                let k = [cx, cy, r, field_of_view.to_radians() / 2.0, (reverse == "on") as u8 as f64, 0.0];
                same(steps, &passes.warp, FxParams { mode: 9, ..Default::default() }, &k, None)
            }
            E::CornerPin { upper_left, upper_right, lower_left, lower_right } => {
                // Crossed corners leave the determinant 0, which draws nothing.
                let (gx, gy) = f.grow;
                let mut k = [0.0; 17];
                if let Some((adj, det, _)) = crate::layer_fx::corner_map([*upper_left, *upper_right, *lower_left, *lower_right], (w, h), f.origin) {
                    k[..9].copy_from_slice(adj.as_flattened());
                    k[9] = det;
                }
                k[10..16].copy_from_slice(&[(w - 2 * ox) as f64, (h - 2 * oy) as f64, ox as f64, oy as f64, (ox + gx) as f64, (oy + gy) as f64]);
                let (tw, th) = (w + 2 * gx, h + 2 * gy);
                let out = self.scratch("B-151 corner pin", tw, th);
                self.fx_step(steps, &passes.warp, FxParams { mode: 10, ..Default::default() }, Some(still), Some(&out), Some(&k), None, none, tiles(tw, th));
                (out, (tw, th))
            }
            _ => unreachable!("compose leaves only the first two batches of ten, twenty-nine of the third batch's thirty and the fourth batch's fifteen as Fx"),
        }
    }

    /// B-51: `glow::glow` of `still`, `width` by `height`: the light in Bloom's bright pass, blurred
    /// in its gauss pass, and laid on the drawing by its combine pass, into a texture grown by the
    /// blur's radius, which is returned with its size.
    fn glow(&self, steps: &mut Vec<Step>, still: &wgpu::TextureView, (w, h): (usize, usize), g: Glow) -> (wgpu::TextureView, (usize, usize)) {
        let passes = self.bloom.as_ref().expect("a Glow is refused without the passes");
        let sigma = g.radius / 3.0;
        let r = crate::effects::kernel_radius(sigma);
        let (gw, gh) = (w + 2 * r, h + 2 * r);
        let tiles = |w: usize, h: usize| ((w as u32).div_ceil(16), (h as u32).div_ceil(16));
        let init = |label, contents: &[f32]| {
            self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some(label),
                contents: bytemuck::cast_slice(contents),
                usage: wgpu::BufferUsages::STORAGE,
            })
        };
        // The colours, then the tint in linear light, as the bright pass reads them.
        let mut colours: Vec<f32> = g.targets[..g.count].iter().flatten().map(|&v| v as f32).collect();
        colours.extend(g.tint.map_or([0.0; 3], |t| t.map(|v| crate::color::srgb_to_linear(v as f32 / 255.0))));
        let light = self.scratch("B-51 light", w, h);
        let p = Params {
            // 256 lets nothing through: a Glow on chosen colours has no brightness test.
            level: if g.bright { crate::bloom::bright_level(g.threshold) } else { 256 },
            count: if g.bright { 0 } else { g.count as u32 },
            // The levels are whole numbers, so within a tolerance is within its whole part.
            tolerance: g.tolerance as u32,
            tinted: g.tint.is_some() as u32,
            ..Default::default()
        };
        self.step(steps, &passes.bright, p, still, Some(&light), None, Some(&init("B-51 colours", &colours)), tiles(w, h));
        // Made empty: wgpu clears every buffer it makes.
        let halo = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("B-51 halo"),
            size: (gw * gh * 16) as u64,
            usage: wgpu::BufferUsages::STORAGE,
            mapped_at_creation: false,
        });
        let add = Params { width: gw as u32, height: gh as u32, weight: 1.0, ..Default::default() };
        if r == 0 {
            self.step(steps, &passes.add, add, &light, None, Some(&halo), None, tiles(gw, gh));
        } else {
            let tall = self.blurred(steps, passes, "B-51", &light, (w, h), sigma);
            self.step(steps, &passes.add, add, &tall, None, Some(&halo), None, tiles(gw, gh));
        }
        let out = self.scratch("B-51 glow", gw, gh);
        let p = Params { g: r as i32, weight: g.intensity as f32, screen: g.screen as u32, ..Default::default() };
        self.step(steps, &passes.combine, p, still, Some(&out), Some(&halo), None, tiles(gw, gh));
        (out, (gw, gh))
    }

    fn target(&mut self, width: usize, height: usize) -> &Target {
        if self.target.as_ref().is_none_or(|t| (t.width, t.height) != (width, height)) {
            let n = (width * height) as u64;
            let buffer = |label, size, usage| {
                self.device.create_buffer(&wgpu::BufferDescriptor { label: Some(label), size, usage, mapped_at_creation: false })
            };
            use wgpu::BufferUsages as U;
            self.target = Some(Target {
                width,
                height,
                sum: buffer("B-44 sum", n * 16, U::STORAGE | U::COPY_DST),
                bytes: buffer("B-44 bytes", n * 4, U::STORAGE | U::COPY_SRC | U::COPY_DST),
                readback: buffer("B-44 readback", n * 4, U::MAP_READ | U::COPY_DST),
                size: self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
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
        // B-156 (D-225): an adjustment layer whose every effect the card draws on the frame.
        let frame = (plan.width, plan.height);
        if plan.layers.iter().filter_map(|l| l.adjust.as_ref()).any(|s| self.fx.is_none() || crate::compose::adjust_run(s, frame).is_none()) {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: its adjustment layer has an effect the GPU does not draw there.".into(),
                "B-156 draws an adjustment layer (D-66) on the card when the card draws each of its effects on the frame (D-225); Bloom, Glow, Paraffin, Kira-kira and HSV Key stay the CPU's.".into(),
            ));
        }
        // B-152: a motion-blurred, frame-mixed or dissolved layer arrives already built by the
        // CPU, and the card lays it like any drawing. B-153b: but sending that new picture costs
        // the card more than the CPU takes to draw the frame whole, so it does so unless the card
        // has an effect of its own to draw (D-218).
        if plan.layers.iter().any(|l| l.motion_blur || l.mixed) && !plan.layers.iter().any(|l| !l.on_card.is_empty() || !wraps(l).is_empty() || l.adjust.is_some()) {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: it has motion blur or frame blending and no effect for the GPU, which the CPU draws faster.".into(),
                "B-153b hands a blurred or mixed frame (D-218) with no card effect to the CPU whole; measured faster there.".into(),
            ));
        }
        if plan.layers.iter().flat_map(|l| &l.wrap).any(|i| i.enabled && i.is_valid() && i.mix < 100.0) {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: it has a Light Wrap with a Mix below 100, which the GPU does not draw yet.".into(),
                "B-137b draws a mixed Light Wrap (D-202) on the CPU; a card version is a later unit.".into(),
            ));
        }
        // B-76: a Light Wrap blurs the frame beneath, grown by the blur's radius.
        let radii: Vec<usize> = plan.layers.iter().flat_map(wraps).map(|(width, _, _)| crate::effects::kernel_radius(width / 3.0)).collect();
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
                Some(stack) => (std::borrow::Cow::Owned(crate::compose::adjust_run(stack, frame).unwrap_or_default()), frame),
                None => (std::borrow::Cow::Borrowed(&l.on_card[..]), (l.source.width(), l.source.height())),
            };
            for card in run.iter() {
                let halo = |g: usize| ((w + 2 * g) * (h + 2 * g) * 16) as u64;
                let (name, grow, bytes) = match card {
                    OnCard::Bloom(b) => {
                        let g = crate::bloom::reach(b.radius, b.lines, b.length);
                        ("a Bloom".to_string(), (g, g), halo(g))
                    }
                    OnCard::Directional(d) => ("a Directional Blur".into(), (d.grow(), d.grow()), 0),
                    OnCard::Gaussian(g) => ("a Gaussian Blur".into(), (g.grow(), g.grow()), 0),
                    OnCard::Glow(g) => {
                        let g = crate::effects::kernel_radius(g.radius / 3.0);
                        ("a Glow".into(), (g, g), halo(g))
                    }
                    OnCard::Fx(f) => {
                        let bytes = match &f.instance.effect {
                            crate::effects::Effect::LensBlur { .. } => ((w + 1) * h * 32) as u64,
                            crate::effects::Effect::Outline { width, .. } => {
                                let n = width.ceil() as usize;
                                ((w + 2 * n) * (h + 2 * n) * 4) as u64
                            }
                            crate::effects::Effect::DistanceGradation { .. } => dist_bytes(w, h) as u64,
                            // B-107: a Simple Choker's band as Outline's, and a spread's running totals.
                            crate::effects::Effect::SimpleChoker { choke } => {
                                let g = if *choke < 0.0 { (-choke).floor() as usize } else { 0 };
                                let sums = if *choke < 0.0 { (w + 1) * h * 32 } else { 0 };
                                ((w + 2 * g) * (h + 2 * g) * 4).max(sums) as u64
                            }
                            // Each block's mean.
                            crate::effects::Effect::Mosaic { size } => (16.0 * (w as f64 / size + 2.0) * (h as f64 / size + 2.0)) as u64,
                            // B-123: a Color Lookup's table rides with its settings; Kira-kira keeps
                            // its light in a buffer the size of the grown drawing, and its stars, at
                            // most one a cell, with its settings.
                            crate::effects::Effect::ColorLookup { table: Some(t), .. } => ((8 + 3 * t.0.entries()) * 8) as u64,
                            crate::effects::Effect::KiraKira { spacing, .. } => {
                                let cells = (w as f64 / spacing + 2.0) * (h as f64 / spacing + 2.0);
                                (((w + 2 * f.grow.0) * (h + 2 * f.grow.1) * 4) as f64).max((19.0 + 4.0 * cells) * 8.0) as u64
                            }
                            // B-151: Cell Pattern's points ride with its settings.
                            crate::effects::Effect::CellPattern { size, .. } => ((15.0 + 3.0 * (w as f64 / size + 5.0) * (h as f64 / size + 5.0)) * 8.0) as u64,
                            _ => 0,
                        };
                        (format!("the effect {}", f.instance.effect.name()), f.grow, bytes)
                    }
                    OnCard::Radial(_) => continue,
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
                if gw + gh + 1 > self.limits.max_texture_dimension_2d as usize
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
                s.width() > most || s.height() > most || sending as u64 > self.limits.max_buffer_size
            })
            .map(|s| format!("a drawing is {} by {}", s.width(), s.height()))
            .or_else(|| {
                (frame_bytes > self.limits.max_storage_buffer_binding_size as u64
                    || frame_bytes > self.limits.max_buffer_size
                    || plan.width.div_ceil(16) > self.limits.max_compute_workgroups_per_dimension as usize
                    || plan.height.div_ceil(16) > self.limits.max_compute_workgroups_per_dimension as usize)
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
    fn lay(&self, steps: &mut Vec<Step>, numbers: [u32; 20], source: &wgpu::TextureView, matte: Option<&wgpu::TextureView>, onto: &wgpu::Buffer) {
        let uniform = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("B-44 layer"),
            contents: bytemuck::cast_slice(&numbers),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &self.layer_layout,
            entries: &[
                wgpu::BindGroupEntry { binding: 0, resource: uniform.as_entire_binding() },
                wgpu::BindGroupEntry { binding: 1, resource: wgpu::BindingResource::TextureView(source) },
                wgpu::BindGroupEntry { binding: 2, resource: wgpu::BindingResource::TextureView(matte.unwrap_or(&self.no_matte)) },
                wgpu::BindGroupEntry { binding: 3, resource: onto.as_entire_binding() },
            ],
        });
        steps.push((self.layer.clone(), group, (numbers[14].div_ceil(16), numbers[15].div_ceil(16))));
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
                let (l, t, r, b) = bounds(layer);
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
                };
                let mut numbers = [
                    f(s0.0), f(s0.1), f(inverse.a), f(inverse.b), f(inverse.c), f(inverse.d),
                    f(m.0), f(m.1), f(m.2), f(m.3), f(m.4), f(m.5),
                    x0, y0, x1 - x0, y1 - y0,
                    width as u32,
                    blend,
                    matte.is_some() as u32,
                    layer.opacity.to_bits(),
                ];
                let tiles = ((width as u32).div_ceil(16), (height as u32).div_ceil(16));
                // B-156 (D-225), render::adjust_frame: the frame laid so far through the layer's
                // stack, then mixed back by what the layer's shape, opacity and matte cover.
                if let Some(stack) = &layer.adjust {
                    let run = crate::compose::adjust_run(stack, (width, height)).expect("refused when the CPU runs it");
                    let taken = self.scratch("B-156 frame", width, height);
                    let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &self.take.1,
                        entries: &[
                            wgpu::BindGroupEntry { binding: 0, resource: size.as_entire_binding() },
                            wgpu::BindGroupEntry { binding: 1, resource: sum.as_entire_binding() },
                            wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&taken) },
                        ],
                    });
                    steps.push((self.take.0.clone(), group, tiles));
                    // Only Paraffin and Kira-kira read the drawing on the CPU, and they stay there.
                    let mut views = Vec::new();
                    self.run(&mut steps, &layer.source, (taken.clone(), (width, height)), &run, &mut views);
                    let (effected, (ew, eh)) = views.pop().unwrap_or((taken, (width, height)));
                    // Its shape is the composition's size, opaque, unless a mask cut it: that one is
                    // kept on the card, and only the cut ones are sent each frame. Both wide, so the
                    // cover is the CPU's to the bit.
                    let shape = if layer.source.data().par_iter().all(|v| *v == 1.0) {
                        if self.ones.as_ref().is_none_or(|o| (o.width(), o.height()) != (layer.source.width(), layer.source.height())) {
                            self.ones = Some(layer.source.clone());
                        }
                        self.ones.clone().expect("kept above")
                    } else {
                        layer.source.clone()
                    };
                    let shape = self.resident(&mut uploads, &shape, None, true);
                    let matte_view = matte.map(|(m, _)| self.resident(&mut uploads, &m.source, cache.name_of(&m.source), true));
                    let m = matte.map_or([0.0; 6], |(_, i)| [i.a, i.b, i.c, i.d, i.tx, i.ty]);
                    let k = [inverse.a, inverse.b, inverse.c, inverse.d, inverse.tx, inverse.ty, m[0], m[1], m[2], m[3], m[4], m[5], layer.opacity as f64];
                    // Every effect grows the frame alike on both sides.
                    let p = FxParams {
                        flag: matte.is_some() as u32,
                        base: [width as u32, height as u32],
                        ox: ((ew - width) / 2) as i32,
                        oy: ((eh - height) / 2) as i32,
                        ..Default::default()
                    };
                    let init = |label, contents: &[u8], usage| {
                        self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(label), contents, usage })
                    };
                    let numbers = init("B-156 numbers", bytemuck::bytes_of(&p), wgpu::BufferUsages::UNIFORM);
                    let k = init("B-156 cover", bytemuck::cast_slice(&k), wgpu::BufferUsages::STORAGE);
                    let passes = self.fx.as_ref().expect("an adjustment layer is refused without the passes");
                    let texture = |binding, view| wgpu::BindGroupEntry { binding, resource: wgpu::BindingResource::TextureView(view) };
                    let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &passes.adjust.1,
                        entries: &[
                            wgpu::BindGroupEntry { binding: 0, resource: numbers.as_entire_binding() },
                            texture(1, &shape),
                            wgpu::BindGroupEntry { binding: 3, resource: k.as_entire_binding() },
                            texture(4, matte_view.as_ref().unwrap_or(&self.no_matte)),
                            texture(10, &effected),
                            wgpu::BindGroupEntry { binding: 11, resource: sum.as_entire_binding() },
                        ],
                    });
                    steps.push((passes.adjust.0.clone(), group, tiles));
                    continue;
                }
                let wraps = wraps(layer);
                // B-156: under an adjustment layer every drawing is sent whole, not in half
                // precision: Posterize, Threshold, a wipe's cut and the like turn the smallest
                // difference in the frame beneath into a large one (D-225).
                let wide = plan.layers.iter().any(|l| l.adjust.is_some()) || !wraps.is_empty() || matches!(layer.on_card.first(), Some(OnCard::Bloom(_) | OnCard::Glow(_) | OnCard::Fx(_)));
                let mut source = self.resident(&mut uploads, &layer.source, cache.name_of(&layer.source), wide);
                if !layer.on_card.is_empty() {
                    source = self.applied(&mut steps, &layer.source, &source, &layer.on_card);
                }
                let matte = matte.map(|(m, _)| self.resident(&mut uploads, &m.source, cache.name_of(&m.source), false));
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
                        wgpu::BindGroupEntry { binding: 0, resource: size.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 1, resource: sum.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 2, resource: placed.as_entire_binding() },
                        wgpu::BindGroupEntry { binding: 3, resource: wgpu::BindingResource::TextureView(&drawn) },
                        wgpu::BindGroupEntry { binding: 4, resource: wgpu::BindingResource::TextureView(&light) },
                    ],
                });
                steps.push((self.unpack.clone(), group, tiles));
                for (w, intensity, add) in wraps {
                    if intensity == 0.0 {
                        continue;
                    }
                    let sigma = w / 3.0;
                    let r = crate::effects::kernel_radius(sigma);
                    let blurred = if r == 0 { light.clone() } else { self.gaussian(&mut steps, &light, (width, height), Gaussian { sigma, repeat: false }).0 };
                    let out = self.scratch("B-76 wrapped", width, height);
                    let p = FxParams { r: r as i32, flag: add as u32, ..Default::default() };
                    let passes = self.fx.as_ref().expect("a Light Wrap is refused without the passes");
                    self.fx_step(&mut steps, &passes.wrap, p, Some(&drawn), Some(&out), Some(&[intensity / 100.0]), Some(&blurred), [None; 4], tiles);
                    drawn = out;
                }
                let f = |v: f32| v.to_bits();
                let placed = [
                    f(x0 as f32 + 0.5), f(y0 as f32 + 0.5), f(1.0), f(0.0), f(0.0), f(1.0),
                    0, 0, 0, 0, 0, 0,
                    x0, y0, x1 - x0, y1 - y0,
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
                    wgpu::BindGroupEntry { binding: 0, resource: target.size.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: target.sum.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 2, resource: target.bytes.as_entire_binding() },
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
                for (pipeline, group, (x, y)) in &steps {
                    pass.set_pipeline(pipeline);
                    pass.set_bind_group(0, group, &[]);
                    pass.dispatch_workgroups(*x, *y, 1);
                }
                pass.set_pipeline(&self.encode);
                pass.set_bind_group(0, &encode_group, &[]);
                pass.dispatch_workgroups((width as u32).div_ceil(16), (height as u32).div_ceil(16), 1);
            }
            // The drawings arrive before the frame that draws with them: one queue, in order.
            if std::mem::take(&mut self.sending.open) {
                self.sending.buffer.as_ref().expect("open only while there").unmap();
            }
            self.sending.after = Some(self.queue.submit([uploads.finish(), encoder.finish()]));
            // Working textures neither this frame nor the one before used, and no drawing keeps,
            // are let go: an effect redrawn each frame frees last frame's result only as it
            // keeps this one.
            if let Some(working) = &self.working {
                working.borrow_mut().retain(|(t, used)| *used + 1 >= self.frame || self.keeps(t));
            }

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
        encoder.copy_buffer_to_buffer(&target.bytes, 0, &target.readback, 0, (target.width * target.height * 4) as u64);
        self.queue.submit([encoder.finish()]);
        let slice = target.readback.slice(..);
        let (tell, told) = std::sync::mpsc::channel();
        slice.map_async(wgpu::MapMode::Read, move |r| drop(tell.send(r)));
        self.device.poll(wgpu::PollType::Wait).map_err(|e| e.to_string())?;
        told.recv().map_err(|e| e.to_string())?.map_err(|e| e.to_string())?;
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
        let target = unsafe { wgpu::SurfaceTargetUnsafe::from_window(window) }.map_err(|e| e.to_string())?;
        let surface = unsafe { self.instance.create_surface_unsafe(target) }.map_err(|e| e.to_string())?;
        let caps = surface.get_capabilities(&self.adapter);
        // Plain eight-bit, not sRGB: the page's colour numbers go to the screen as they are.
        let format = [wgpu::TextureFormat::Bgra8Unorm, wgpu::TextureFormat::Rgba8Unorm]
            .into_iter()
            .find(|f| caps.formats.contains(f))
            .ok_or_else(|| format!("{} cannot paint this window in eight-bit colour", self.about))?;
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
        let module = self.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("B-45"),
            source: wgpu::ShaderSource::Wgsl(SCREEN_SHADER.into()),
        });
        let fragment = |binding, ty| wgpu::BindGroupLayoutEntry { visibility: wgpu::ShaderStages::FRAGMENT, ..entry(binding, ty) };
        let layout = self.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("B-45 screen"),
            entries: &[fragment(0, uniform()), fragment(1, storage(true)), fragment(2, storage(true))],
        });
        let pipeline = self.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("B-45 screen"),
            layout: Some(&self.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: None,
                bind_group_layouts: &[&layout],
                push_constant_ranges: &[],
            })),
            vertex: wgpu::VertexState { module: &module, entry_point: Some("corner"), compilation_options: Default::default(), buffers: &[] },
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
        if let Some(wgpu::rwh::RawWindowHandle::Win32(h)) = window.window_handle().ok().map(|w| w.as_raw()) {
            use windows::Win32::Foundation::HWND;
            use windows::Win32::UI::WindowsAndMessaging::{GetWindowLongPtrW, SetWindowLongPtrW, GWL_STYLE, WS_CLIPCHILDREN};
            let hwnd = HWND(h.hwnd.get() as *mut _);
            // SAFETY: the handle is the live window the caller vouched for; only its style changes.
            unsafe { SetWindowLongPtrW(hwnd, GWL_STYLE, GetWindowLongPtrW(hwnd, GWL_STYLE) & !(WS_CLIPCHILDREN.0 as isize)) };
        }
        self.screen = Some(Screen { surface, config, pipeline, layout });
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
    pub fn show(&mut self, size: (u32, u32), paints: &[Paint], alpha_only: bool) -> Result<(), String> {
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
                    screen.surface.get_current_texture().map_err(|e| e.to_string())?
                }
                Err(e) => return Err(e.to_string()),
            };
            self.device.push_error_scope(wgpu::ErrorFilter::Validation);
            let numbers = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("B-45 screen"),
                contents: bytemuck::cast_slice(&[paints.len() as u32, target.width as u32, target.height as u32, alpha_only as u32]),
                usage: wgpu::BufferUsages::UNIFORM,
            });
            let list = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("B-45 paints"),
                contents: bytemuck::cast_slice(paints),
                usage: wgpu::BufferUsages::STORAGE,
            });
            let group = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &screen.layout,
                entries: &[
                    wgpu::BindGroupEntry { binding: 0, resource: numbers.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 1, resource: list.as_entire_binding() },
                    wgpu::BindGroupEntry { binding: 2, resource: target.bytes.as_entire_binding() },
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
                        ops: wgpu::Operations { load: wgpu::LoadOp::Clear(wgpu::Color::BLACK), store: wgpu::StoreOp::Store },
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
