// `render_tile` as a compute shader: one thread per destination pixel, one dispatch per layer,
// bottom of the stack first. The frame buffer is both read and written by the same thread and no
// thread touches another thread's pixel, so the layer order is the dispatch order and nothing
// else needs a barrier.
//
// This is a translation of `src/render.rs::render_tile` and `src/composite.rs::blend_pixel`
// written out longhand rather than shared, because the whole question P-07 asks is how far the
// two land apart. Every arithmetic step below is f32; the CPU does the geometry in f64 and only
// the sample weights in f32. That difference is the measurement, not a defect in this file.

struct Layer {
    // The inverse transform, destination pixel -> source pixel: x' = a*x + c*y + tx.
    a: f32, b: f32, c: f32, d: f32, tx: f32, ty: f32,
    src_w: u32, src_h: u32,
    frame_w: u32, frame_h: u32,
    opacity: f32,
    blend: u32,       // 0 normal, 1 multiply, 2 screen, 3 add
}

@group(0) @binding(0) var<uniform> layer: Layer;
@group(0) @binding(1) var<storage, read> source: array<vec4<f32>>;
@group(0) @binding(2) var<storage, read_write> frame: array<vec4<f32>>;

// Document 21: bilinear weights from source pixel centres, and samples outside the source extent
// are transparent black - contributed as zero rather than dropped, so the weights are never
// renormalised and the edge does not brighten.
fn sample_bilinear(x: f32, y: f32) -> vec4<f32> {
    let fx = x - 0.5;
    let fy = y - 0.5;
    let x0 = floor(fx);
    let y0 = floor(fy);
    let ux = fx - x0;
    let uy = fy - y0;

    var out = vec4<f32>(0.0);
    for (var dy = 0; dy < 2; dy = dy + 1) {
        let wy = select(uy, 1.0 - uy, dy == 0);
        let sy = i32(y0) + dy;
        if (wy == 0.0 || sy < 0 || sy >= i32(layer.src_h)) { continue; }
        for (var dx = 0; dx < 2; dx = dx + 1) {
            let wx = select(ux, 1.0 - ux, dx == 0);
            let sx = i32(x0) + dx;
            if (wx == 0.0 || sx < 0 || sx >= i32(layer.src_w)) { continue; }
            out = out + source[u32(sy) * layer.src_w + u32(sx)] * (wx * wy);
        }
    }
    return out;
}

fn unpremultiply(p: vec4<f32>) -> vec3<f32> {
    if (p.a == 0.0) { return vec3<f32>(0.0); }
    return p.rgb / p.a;
}

fn blend_pixel(src: vec4<f32>, dst: vec4<f32>) -> vec4<f32> {
    if (layer.blend == 0u) {
        // Document 21 line 57: Co = Cs + Cd*(1-As), Ao = As + Ad*(1-As).
        return src + dst * (1.0 - src.a);
    }
    let cs = unpremultiply(src);
    let cd = unpremultiply(dst);
    var b = vec3<f32>(0.0);
    if (layer.blend == 1u) { b = cs * cd; }
    else if (layer.blend == 2u) { b = cs + cd - cs * cd; }
    else { b = min(cs + cd, vec3<f32>(1.0)); }
    let rgb = (1.0 - src.a) * dst.rgb + (1.0 - dst.a) * src.rgb + src.a * dst.a * b;
    return vec4<f32>(rgb, src.a + dst.a - src.a * dst.a);
}

@compute @workgroup_size(8, 8, 1)
fn main(@builtin(global_invocation_id) gid: vec3<u32>) {
    if (gid.x >= layer.frame_w || gid.y >= layer.frame_h) { return; }
    // Pixel (i,j) is centred at (i+0.5, j+0.5), so the sample point is the centre.
    let dx = f32(gid.x) + 0.5;
    let dy = f32(gid.y) + 0.5;
    let sx = layer.a * dx + layer.c * dy + layer.tx;
    let sy = layer.b * dx + layer.d * dy + layer.ty;
    var src = sample_bilinear(sx, sy);
    if (layer.opacity != 1.0) { src = src * layer.opacity; }
    let i = gid.y * layer.frame_w + gid.x;
    frame[i] = blend_pixel(src, frame[i]);
}
