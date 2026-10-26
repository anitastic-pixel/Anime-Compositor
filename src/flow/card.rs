//! D-878: the motion finder's levels on the graphics card, for a frame the card draws. The
//! processor's [`super::estimate`] is the reference and the fallback: each pass here is its step,
//! in its order and in single precision, and anything the card cannot do gives `None`, so the
//! processor finds the motion instead. The two pictures' planes come from the processor (the same
//! numbers its own finder starts from) and the field comes back at full size.
//!
//! The card has a device of its own here, apart from the viewer's (`gpu::Gpu`): compose, where
//! these effects run, does not hold the viewer's. It is made the first time it is asked for.

use std::sync::{Mutex, OnceLock};

use rayon::prelude::*;
use wgpu::util::DeviceExt as _;

use super::{Field, Planes};

const SHADER: &str = r#"
struct P { w: u32, h: u32, ow: u32, oh: u32, r: i32, ridge: f32, area: f32, k: f32, one: f32 }
@group(0) @binding(0) var<uniform> p: P;
@group(0) @binding(1) var<storage, read_write> in2: array<vec2<f32>>;
@group(0) @binding(2) var<storage, read_write> out2: array<vec2<f32>>;
@group(0) @binding(3) var<storage, read_write> aux2a: array<vec2<f32>>;
@group(0) @binding(4) var<storage, read_write> aux2b: array<vec2<f32>>;
@group(0) @binding(5) var<storage, read_write> in4: array<vec4<f32>>;
@group(0) @binding(6) var<storage, read_write> out4: array<vec4<f32>>;

fn held(i: i32, n: u32) -> u32 { return u32(clamp(i, 0, i32(n) - 1)); }

// `in2` read at (x, y), held, a picture `w` by `h`: flow::bilinear_held on both planes.
fn bilinear(x0: f32, y0: f32, w: u32, h: u32) -> vec2<f32> {
    let x = clamp(x0, 0.0, f32(w) - 1.0);
    let y = clamp(y0, 0.0, f32(h) - 1.0);
    let fx = floor(x);
    let fy = floor(y);
    let ux = x - fx;
    let uy = y - fy;
    let ix = u32(fx);
    let iy = u32(fy);
    let jx = min(ix + 1u, w - 1u);
    let jy = min(iy + 1u, h - 1u);
    let top = in2[iy * w + ix] * (1.0 - ux) * p.one + in2[iy * w + jx] * ux * p.one;
    let bottom = in2[jy * w + ix] * (1.0 - ux) * p.one + in2[jy * w + jx] * ux * p.one;
    return top * (1.0 - uy) * p.one + bottom * uy * p.one;
}

// n / d nearer the processor's division: the card's own may be a few units off.
fn quotient(n: f32, d: f32) -> f32 {
    let q = n / d;
    return q + fma(-q, d, n) / d;
}

fn index(g: vec3<u32>, n: vec3<u32>) -> u32 { return g.x + g.y * n.x * 64u; }

// flow::halve: in2 (ow by oh) to out2 (w by h).
@compute @workgroup_size(64) fn halve(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let x = i32(i % p.w);
    let y = i32(i / p.w);
    var s = vec2<f32>(0.0);
    for (var dy = 0; dy < 2; dy++) {
        for (var dx = 0; dx < 2; dx++) {
            s += in2[held(2 * y + dy, p.oh) * p.ow + held(2 * x + dx, p.ow)];
        }
    }
    out2[i] = s / 4.0;
}

// The level's gradients across and down, both planes: out4 = (gx0, gx1, gy0, gy1).
@compute @workgroup_size(64) fn grad(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let x = i32(i % p.w);
    let y = i32(i / p.w);
    let row = u32(y) * p.w;
    let gx = (in2[row + held(x + 1, p.w)] - in2[row + held(x - 1, p.w)]) / 2.0;
    let gy = (in2[held(y + 1, p.h) * p.w + u32(x)] - in2[held(y - 1, p.h) * p.w + u32(x)]) / 2.0;
    out4[i] = vec4<f32>(gx.x, gx.y, gy.x, gy.y);
}

// The gradient products: out4 = (gxx, gxy, gyy, 0), before they are boxed.
@compute @workgroup_size(64) fn products(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let q = in4[i];
    let o = p.one;
    out4[i] = vec4<f32>(q.x * q.x * o + q.y * q.y * o, q.x * q.z * o + q.y * q.w * o, q.z * q.z * o + q.w * q.w * o, 0.0);
}

// flow::boxed, across then down, on four planes and on two.
@compute @workgroup_size(64) fn across4(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let x = i32(i % p.w);
    let row = (i / p.w) * p.w;
    var s = vec4<f32>(0.0);
    for (var k = -p.r; k <= p.r; k++) { s += in4[row + held(x + k, p.w)]; }
    out4[i] = s;
}
@compute @workgroup_size(64) fn down4(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let x = i % p.w;
    let y = i32(i / p.w);
    var s = vec4<f32>(0.0);
    for (var k = -p.r; k <= p.r; k++) { s += in4[held(y + k, p.h) * p.w + x]; }
    out4[i] = s;
}
@compute @workgroup_size(64) fn across2(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let x = i32(i % p.w);
    let row = (i / p.w) * p.w;
    var s = vec2<f32>(0.0);
    for (var k = -p.r; k <= p.r; k++) { s += in2[row + held(x + k, p.w)]; }
    out2[i] = s;
}
@compute @workgroup_size(64) fn down2(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let x = i % p.w;
    let y = i32(i / p.w);
    var s = vec2<f32>(0.0);
    for (var k = -p.r; k <= p.r; k++) { s += in2[held(y + k, p.h) * p.w + x]; }
    out2[i] = s;
}

// One iteration's mismatch: in2 is b, aux2a is a, aux2b the field, in4 the gradients.
@compute @workgroup_size(64) fn mismatch(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let u = aux2b[i];
    let e = bilinear(f32(i % p.w) + u.x, f32(i / p.w) + u.y, p.w, p.h) - aux2a[i];
    let q = in4[i];
    let o = p.one;
    out2[i] = vec2<f32>(q.x * e.x * o + q.y * e.y * o, q.z * e.x * o + q.w * e.y * o);
}

// One iteration's step: in4 the boxed products, in2 the boxed mismatch, out2 the field.
@compute @workgroup_size(64) fn solve(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let m = in4[i];
    let b = in2[i];
    let a = m.x + p.ridge;
    let d = m.z + p.ridge;
    let c = m.y;
    let det = (a * d) * p.one - (c * c) * p.one;
    var u = out2[i];
    u.x -= quotient((d * b.x) * p.one - (c * b.y) * p.one, det);
    u.y -= quotient((a * b.y) * p.one - (c * b.x) * p.one, det);
    out2[i] = u;
}

// The level's smoothing: in2 the boxed field, out2 the field, k Local Smoothness.
@compute @workgroup_size(64) fn smoothen(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    var u = out2[i];
    let m = in2[i];
    u += p.k * (vec2<f32>(quotient(m.x, p.area), quotient(m.y, p.area)) - u) * p.one;
    out2[i] = u;
}

// flow::enlarge: in2 (ow by oh) read for each pixel of out2 (w by h), times k.
@compute @workgroup_size(64) fn enlarge(@builtin(global_invocation_id) g: vec3<u32>, @builtin(num_workgroups) n: vec3<u32>) {
    let i = index(g, n);
    if (i >= p.w * p.h) { return; }
    let sx = (f32(i % p.w) + 0.5) / p.k - 0.5;
    let sy = (f32(i / p.w) + 0.5) / p.k - 0.5;
    out2[i] = p.k * bilinear(sx, sy, p.ow, p.oh);
}
"#;

const PASSES: [&str; 11] = [
    "halve", "grad", "products", "across4", "down4", "across2", "down2", "mismatch", "solve",
    "smoothen", "enlarge",
];

struct Card {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipelines: Vec<wgpu::ComputePipeline>,
    /// The last size's buffers and steps, kept: making them takes longer than the finding.
    /// ponytail: one size kept (about 125 bytes a pixel of card memory, held until the app
    /// closes); keep more, or drop it when idle, if two sizes alternate or memory runs short.
    kept: Option<(Key, Plan)>,
}

/// What a [`Plan`] is made for: the size, the levels and the settings in its parameters.
type Key = (usize, usize, usize, usize, isize, u32, u32, u32, u32);

struct Plan {
    /// The two pictures' planes at full size, written each call.
    a0: wgpu::Buffer,
    b0: wgpu::Buffer,
    /// The top level's field, cleared each call: the finder starts from no motion.
    start: wgpu::Buffer,
    /// The field at full size, copied into `readback`.
    last: wgpu::Buffer,
    readback: wgpu::Buffer,
    steps: Vec<(usize, wgpu::BindGroup, usize)>,
}

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Params {
    w: u32,
    h: u32,
    ow: u32,
    oh: u32,
    r: i32,
    ridge: f32,
    area: f32,
    k: f32,
    /// 1, unknown to the card's compiler: a product times it is rounded before it is added,
    /// as the processor's is, rather than fused into the addition.
    one: f32,
}

fn open() -> Option<Card> {
    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor {
        // DX12 only: on this machine's Vulkan the found motion was up to 1.06e-4 px off the
        // processor's (D-878), on DX12 the same to the bit.
        backends: wgpu::Backends::DX12,
        ..Default::default()
    });
    let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: None,
    }))
    .ok()?;
    if adapter.get_info().device_type == wgpu::DeviceType::Cpu {
        return None;
    }
    let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("D-878 motion finder"),
        required_features: wgpu::Features::empty(),
        required_limits: adapter.limits(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
    }))
    .map_err(|e| eprintln!("D-878: the motion finder cannot open the card: {e}"))
    .ok()?;
    // Every call runs inside error scopes; the default would end the program.
    device.on_uncaptured_error(Box::new(|e| eprintln!("D-878: the card reported {e}")));
    device.push_error_scope(wgpu::ErrorFilter::Validation);
    let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("D-878"),
        source: wgpu::ShaderSource::Wgsl(SHADER.into()),
    });
    let pipelines = PASSES
        .iter()
        .map(|entry| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry),
                layout: None,
                module: &module,
                entry_point: Some(entry),
                compilation_options: Default::default(),
                cache: None,
            })
        })
        .collect();
    if let Some(e) = pollster::block_on(device.pop_error_scope()) {
        eprintln!("D-878: the motion finder cannot start on the card: {e}");
        return None;
    }
    Some(Card {
        device,
        queue,
        pipelines,
        kept: None,
    })
}

fn card() -> Option<&'static Mutex<Card>> {
    static CARD: OnceLock<Option<Mutex<Card>>> = OnceLock::new();
    CARD.get_or_init(|| open().map(Mutex::new)).as_ref()
}

/// Whether the motion finder has a card on this machine, starting it if it has not been.
pub(crate) fn available() -> bool {
    card().is_some()
}

/// [`super::estimate`]'s levels `top` down to `lf` and its last enlargement, on the card, from
/// the two pictures' planes at full size; `None` when the card cannot.
#[allow(clippy::too_many_arguments)]
pub(super) fn estimate(
    ca: &Planes,
    cb: &Planes,
    top: usize,
    lf: usize,
    r: isize,
    ridge: f32,
    area: f32,
    iterations: u32,
    smooth: f32,
) -> Option<Field> {
    let card = card()?;
    let pair = |p: &Planes| -> Vec<[f32; 2]> {
        p.c[0]
            .par_iter()
            .zip(&p.c[1])
            .map(|(&a, &b)| [a, b])
            .collect()
    };
    let (a0, b0) = (pair(ca), pair(cb));
    // No rayon inside the lock: a thread waiting here could otherwise be handed the work of the
    // thread that holds it.
    let field = {
        let mut card = card.lock().unwrap_or_else(|e| e.into_inner());
        run(
            &mut card, &a0, &b0, ca.w, ca.h, top, lf, r, ridge, area, iterations, smooth,
        )?
    };
    let (x, y) = field
        .par_iter()
        .map(|v| (f64::from(v[0]), f64::from(v[1])))
        .unzip();
    Some(Field { x, y })
}

#[allow(clippy::too_many_arguments)]
fn run(
    card: &mut Card,
    a0: &[[f32; 2]],
    b0: &[[f32; 2]],
    w: usize,
    h: usize,
    top: usize,
    lf: usize,
    r: isize,
    ridge: f32,
    area: f32,
    iterations: u32,
    smooth: f32,
) -> Option<Vec<[f32; 2]>> {
    let key = (
        w,
        h,
        top,
        lf,
        r,
        ridge.to_bits(),
        area.to_bits(),
        iterations,
        smooth.to_bits(),
    );
    if card.kept.as_ref().is_none_or(|(k, _)| *k != key) {
        card.kept = None;
        let made = plan(card, w, h, top, lf, r, ridge, area, iterations, smooth)?;
        card.kept = Some((key, made));
    }
    let (device, queue) = (&card.device, &card.queue);
    let plan = &card.kept.as_ref()?.1;
    device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
    device.push_error_scope(wgpu::ErrorFilter::Validation);
    queue.write_buffer(&plan.a0, 0, bytemuck::cast_slice(a0));
    queue.write_buffer(&plan.b0, 0, bytemuck::cast_slice(b0));
    let mut encoder = device.create_command_encoder(&Default::default());
    encoder.clear_buffer(&plan.start, 0, None);
    {
        let mut pass = encoder.begin_compute_pass(&Default::default());
        for (k, group, n) in &plan.steps {
            let groups = n.div_ceil(64) as u32;
            let (gx, gy) = (groups.min(65535), groups.div_ceil(65535));
            pass.set_pipeline(&card.pipelines[*k]);
            pass.set_bind_group(0, group, &[]);
            pass.dispatch_workgroups(gx, gy, 1);
        }
    }
    encoder.copy_buffer_to_buffer(&plan.last, 0, &plan.readback, 0, (w * h * 8) as u64);
    queue.submit([encoder.finish()]);
    let slice = plan.readback.slice(..);
    let (tell, told) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| drop(tell.send(r)));
    let waited = device.poll(wgpu::PollType::Wait).is_ok();
    let invalid = pollster::block_on(device.pop_error_scope());
    let memory = pollster::block_on(device.pop_error_scope());
    if let Some(e) = invalid.or(memory) {
        eprintln!("D-878: the card could not find this motion, the processor finds it: {e}");
        card.kept = None;
        return None;
    }
    if !waited {
        eprintln!("D-878: the card did not finish, the processor finds this motion");
        card.kept = None;
        return None;
    }
    if told.recv().ok().and_then(Result::ok).is_none() {
        eprintln!("D-878: the card's motion could not be read back, the processor finds it");
        card.kept = None;
        return None;
    }
    let out = bytemuck::cast_slice::<u8, [f32; 2]>(&slice.get_mapped_range()).to_vec();
    plan.readback.unmap();
    Some(out)
}

/// The buffers and steps for one size and setting; `None` when the card cannot make them.
#[allow(clippy::too_many_arguments)]
fn plan(
    card: &Card,
    w: usize,
    h: usize,
    top: usize,
    lf: usize,
    r: isize,
    ridge: f32,
    area: f32,
    iterations: u32,
    smooth: f32,
) -> Option<Plan> {
    let device = &card.device;
    device.push_error_scope(wgpu::ErrorFilter::OutOfMemory);
    device.push_error_scope(wgpu::ErrorFilter::Validation);
    let storage = wgpu::BufferUsages::STORAGE;
    let empty = |n: usize, item: u64, more: wgpu::BufferUsages| {
        device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (n.max(1) as u64) * item,
            usage: storage | more,
            mapped_at_creation: false,
        })
    };
    let none = wgpu::BufferUsages::empty();
    let uniform = |q: Params| {
        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: None,
            contents: bytemuck::bytes_of(&q),
            usage: wgpu::BufferUsages::UNIFORM,
        })
    };
    let params = |w: usize, h: usize, ow: usize, oh: usize, k: f32| Params {
        w: w as u32,
        h: h as u32,
        ow: ow as u32,
        oh: oh as u32,
        r: r as i32,
        ridge,
        area,
        k,
        one: 1.0,
    };
    // The pyramid: each level's size, and its two pictures' planes in pairs.
    let mut sizes = vec![(w, h)];
    for _ in 0..top {
        let (lw, lh) = *sizes.last().unwrap();
        sizes.push((lw.div_ceil(2), lh.div_ceil(2)));
    }
    let level = |(lw, lh): (usize, usize)| empty(lw * lh, 8, wgpu::BufferUsages::COPY_DST);
    let a: Vec<wgpu::Buffer> = sizes.iter().map(|&s| level(s)).collect();
    let b: Vec<wgpu::Buffer> = sizes.iter().map(|&s| level(s)).collect();
    let most = sizes[lf].0 * sizes[lf].1;
    let [g, m, mb, t4] = std::array::from_fn(|_| empty(most, 16, none));
    let [e, t2, be] = std::array::from_fn(|_| empty(most, 8, none));
    // The fields: the last one is copied out when the finest level is the picture's own size.
    let [u0, u1] = std::array::from_fn(|_| {
        empty(
            most,
            8,
            wgpu::BufferUsages::COPY_SRC | wgpu::BufferUsages::COPY_DST,
        )
    });
    let start = u1.clone();
    let full = (lf > 0).then(|| empty(w * h, 8, wgpu::BufferUsages::COPY_SRC));
    let readback = device.create_buffer(&wgpu::BufferDescriptor {
        label: None,
        size: (w * h * 8) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });
    let mut uniforms = Vec::new();
    let mut steps: Vec<(usize, wgpu::BindGroup, usize)> = Vec::new();
    let mut step = |pass: usize, q: Params, bound: &[(u32, &wgpu::Buffer)]| {
        let u = uniform(q);
        let layout = card.pipelines[pass].get_bind_group_layout(0);
        let entries: Vec<wgpu::BindGroupEntry> = std::iter::once((0, &u))
            .chain(bound.iter().copied())
            .map(|(binding, buffer)| wgpu::BindGroupEntry {
                binding,
                resource: buffer.as_entire_binding(),
            })
            .collect();
        let group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: None,
            layout: &layout,
            entries: &entries,
        });
        steps.push((pass, group, (q.w * q.h) as usize));
        uniforms.push(u);
    };
    let [halve, grad, products, across4, down4, across2, down2, mismatch, solve, smoothing, enlarge] =
        std::array::from_fn(|i| i);
    for l in 1..=top {
        let (lw, lh) = sizes[l];
        let (ow, oh) = sizes[l - 1];
        for planes in [&a, &b] {
            step(
                halve,
                params(lw, lh, ow, oh, 0.0),
                &[(1, &planes[l - 1]), (2, &planes[l])],
            );
        }
    }
    // The field of the level before, and this level's.
    let (mut was, mut is) = (&u0, &u1);
    for level in (lf..=top).rev() {
        let (lw, lh) = sizes[level];
        let q = params(lw, lh, 0, 0, smooth);
        if level < top {
            let (ow, oh) = sizes[level + 1];
            step(enlarge, params(lw, lh, ow, oh, 2.0), &[(1, was), (2, is)]);
        }
        step(grad, q, &[(1, &a[level]), (6, &g)]);
        step(products, q, &[(5, &g), (6, &m)]);
        step(across4, q, &[(5, &m), (6, &t4)]);
        step(down4, q, &[(5, &t4), (6, &mb)]);
        for _ in 0..iterations {
            step(
                mismatch,
                q,
                &[(1, &b[level]), (2, &e), (3, &a[level]), (4, is), (5, &g)],
            );
            step(across2, q, &[(1, &e), (2, &t2)]);
            step(down2, q, &[(1, &t2), (2, &be)]);
            step(solve, q, &[(1, &be), (2, is), (5, &mb)]);
        }
        step(across2, q, &[(1, is), (2, &t2)]);
        step(down2, q, &[(1, &t2), (2, &be)]);
        step(smoothing, q, &[(1, &be), (2, is)]);
        std::mem::swap(&mut was, &mut is);
    }
    // `was` is the finest level's field now.
    let last = match &full {
        Some(full) => {
            let (ow, oh) = sizes[lf];
            step(
                enlarge,
                params(w, h, ow, oh, f32::powi(2.0, lf as i32)),
                &[(1, was), (2, full)],
            );
            full
        }
        None => was,
    };
    let last = last.clone();
    let made = pollster::block_on(device.pop_error_scope());
    let memory = pollster::block_on(device.pop_error_scope());
    if let Some(e) = made.or(memory) {
        eprintln!("D-878: the card cannot hold this motion finder, the processor finds it: {e}");
        return None;
    }
    Some(Plan {
        a0: a[0].clone(),
        b0: b[0].clone(),
        start,
        last,
        readback,
        steps,
    })
}
