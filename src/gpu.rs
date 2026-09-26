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

use std::sync::{Arc, Weak};

use half::slice::HalfFloatSliceExt as _;
use rayon::prelude::*;
use wgpu::util::DeviceExt as _;

use crate::cache::{CelCache, Name};
use crate::diagnostics::{Diagnostic, DiagnosticId, Severity};
use crate::model::BlendMode;
use crate::perf::{self, Stage};
use crate::render::{bounds, FramePlan, Radial};
use crate::WorkingBuffer;

/// What the card may hold in drawings when Windows cannot say how much memory it has: D-40's
/// gibibyte, the CPU's own budget.
pub const DEFAULT_BUDGET_BYTES: usize = 1024 * 1024 * 1024;

/// A drawing on the card costs eight bytes a pixel: four sixteen-bit floats.
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
        total += bilinear(still, p);
    }
    textureStore(moved, id.xy, total / f32(n));
}
"#;

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
    /// B-46: its last Radial Blur, kept while the settings stay the same.
    blurred: Option<(Radial, wgpu::TextureView)>,
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
    /// Bound as the matte of a layer that has none. Never read.
    no_matte: wgpu::TextureView,
    store: Vec<Stored>,
    frame: u64,
    target: Option<Target>,
    /// Bytes of drawings the card may hold; public so the B-44 test can squeeze it.
    pub budget: usize,
    /// Drawings sent to the card since it was opened.
    sent: u64,
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
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("B-44 preview"),
            required_features: wgpu::Features::empty(),
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
        let radial_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("B-46 radial"),
            entries: &[
                entry(0, uniform()),
                entry(1, texture()),
                entry(
                    2,
                    wgpu::BindingType::StorageTexture {
                        access: wgpu::StorageTextureAccess::WriteOnly,
                        format: wgpu::TextureFormat::Rgba32Float,
                        view_dimension: wgpu::TextureViewDimension::D2,
                    },
                ),
                entry(3, storage(true)),
            ],
        });
        let pipeline = |layout: &wgpu::BindGroupLayout, entry_point: &str| {
            device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
                label: Some(entry_point),
                layout: Some(&device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[layout],
                    push_constant_ranges: &[],
                })),
                module: &module,
                entry_point: Some(entry_point),
                compilation_options: Default::default(),
                cache: None,
            })
        };
        let (layer, encode) = (pipeline(&layer_layout, "layer"), pipeline(&encode_layout, "encode"));
        let radial = pipeline(&radial_layout, "radial");
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
            no_matte,
            store: Vec::new(),
            frame: 0,
            target: None,
            // Half, so the frame's own buffers, the window and every other program keep room.
            budget: memory.map_or(DEFAULT_BUDGET_BYTES, |m| (m / 2) as usize),
            sent: 0,
        })
    }

    /// How many drawings have been sent to the card since it was opened.
    pub fn sent(&self) -> u64 {
        self.sent
    }

    /// The card, its driver and the backend, for tables and the switch.
    pub fn about(&self) -> &str {
        &self.about
    }

    /// Let go of every drawing on the card.
    pub fn forget(&mut self) {
        self.store.clear();
    }

    /// `source` on the card, sending it first if it is not there. The view itself, not a place
    /// in the store, since sending one drawing can evict another this frame has not yet drawn.
    fn resident(&mut self, uploads: &mut wgpu::CommandEncoder, source: &Arc<WorkingBuffer>, name: Option<Name>) -> wgpu::TextureView {
        // A drawing the CPU still holds is found by its address; one it has let go of, by the
        // CPU cache's name for it.
        let found = self.store.iter_mut().find(|s| {
            (s.held.strong_count() > 0 && s.held.as_ptr() == Arc::as_ptr(source))
                || (name.is_some() && s.name == name)
        });
        if let Some(s) = found {
            s.held = Arc::downgrade(source);
            s.used = self.frame;
            return s.view.clone();
        }
        // A drawing nothing holds and nothing names can never be asked for again.
        self.store.retain(|s| s.name.is_some() || s.held.strong_count() > 0);
        let (width, height) = (source.width(), source.height());
        let bytes = width * height * BYTES_PER_PIXEL;
        // Least recently used first, never one this frame draws with. A frame that needs more
        // than the budget still gets every drawing it needs.
        while self.store.iter().map(|s| s.bytes).sum::<usize>() + bytes > self.budget {
            let Some(oldest) = (0..self.store.len())
                .filter(|&i| self.store[i].used < self.frame)
                .min_by_key(|&i| self.store[i].used)
            else {
                break;
            };
            self.store.swap_remove(oldest);
        }
        let size = wgpu::Extent3d { width: width as u32, height: height as u32, depth_or_array_layers: 1 };
        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some("B-44 drawing"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba16Float,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        // Every thread converts rows straight into memory the card copies from.
        let row = (width * BYTES_PER_PIXEL).next_multiple_of(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT as usize);
        let staging = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("B-44 sending"),
            size: (row * height) as u64,
            usage: wgpu::BufferUsages::MAP_WRITE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: true,
        });
        {
            let mut mapped = staging.slice(..).get_mapped_range_mut();
            let halves: &mut [half::f16] = bytemuck::cast_slice_mut(&mut mapped);
            halves
                .par_chunks_mut(row / 2)
                .zip(source.data().par_chunks(width * 4))
                .for_each(|(to, from)| to[..width * 4].convert_from_f32_slice(from));
        }
        staging.unmap();
        uploads.copy_buffer_to_texture(
            wgpu::TexelCopyBufferInfo {
                buffer: &staging,
                layout: wgpu::TexelCopyBufferLayout { offset: 0, bytes_per_row: Some(row as u32), rows_per_image: None },
            },
            texture.as_image_copy(),
            size,
        );
        self.sent += 1;
        let view = texture.create_view(&Default::default());
        self.store.push(Stored { held: Arc::downgrade(source), name, view: view.clone(), bytes, used: self.frame, blurred: None });
        view
    }

    /// B-46: `blurs::radial_blur` of `source`, which [`Gpu::resident`] has just put on the card
    /// as `still`. A blur already made of it with the same settings is reused; otherwise what to
    /// dispatch is added to `blurs`, to run before the layers.
    fn blurred(
        &mut self,
        blurs: &mut Vec<(wgpu::BindGroup, (u32, u32))>,
        source: &Arc<WorkingBuffer>,
        still: &wgpu::TextureView,
        r: Radial,
    ) -> wgpu::TextureView {
        let stored = self.store.iter().position(|s| s.used == self.frame && &s.view == still);
        if let Some((kept, view)) = stored.and_then(|i| self.store[i].blurred.as_ref()) {
            if *kept == r {
                return view.clone();
            }
        }
        let (width, height) = (source.width() as u32, source.height() as u32);
        let moved = self.blur(blurs, still, (width, height), r);
        if let Some(i) = stored {
            let s = &mut self.store[i];
            if s.blurred.is_none() {
                s.bytes += width as usize * height as usize * 16;
            }
            s.blurred = Some((r, moved.clone()));
        }
        moved
    }

    /// `blurs::radial_blur` of `still`, `width` by `height`, into a texture of its own, which is
    /// returned. What to dispatch is added to `blurs`, to run before the layers.
    fn blur(&self, blurs: &mut Vec<(wgpu::BindGroup, (u32, u32))>, still: &wgpu::TextureView, (width, height): (u32, u32), r: Radial) -> wgpu::TextureView {
        let moved = self
            .device
            .create_texture(&wgpu::TextureDescriptor {
                label: Some("B-46 radial"),
                size: wgpu::Extent3d { width, height, depth_or_array_layers: 1 },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Rgba32Float,
                usage: wgpu::TextureUsages::STORAGE_BINDING | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            })
            .create_view(&Default::default());
        let turns: Vec<[f32; 2]> = crate::blurs::radial_turns(r.spin, r.amount)
            .into_iter()
            .flatten()
            .map(|(a, b)| [a as f32, b as f32])
            .collect();
        let init = |label, contents: &[u8], usage| {
            self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor { label: Some(label), contents, usage })
        };
        let f = |v: f64| (v as f32).to_bits();
        let settings = [f(r.center.0), f(r.center.1), f(r.amount), r.spin as u32];
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
        blurs.push((group, (width, height)));
        moved
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
        if plan.layers.iter().any(|l| l.adjust.is_some()) {
            return Some(on_cpu(
                Severity::Info,
                "The CPU drew this frame: it has an adjustment layer, which the GPU does not draw yet.".into(),
                "B-44 draws a frame with an adjustment layer (D-66) wholly on the CPU.".into(),
            ));
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

        // Each layer that can show anything: its drawing, its matte's, and its numbers, laid
        // out as the shader's `Layer`. The same skips as `render_tile`.
        let mut layers: Vec<(wgpu::TextureView, Option<wgpu::TextureView>, [u32; 20], (u32, u32))> = Vec::new();
        let mut uploads = self.device.create_command_encoder(&Default::default());
        let mut blurs = Vec::new();
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
                let numbers = [
                    f(s0.0), f(s0.1), f(inverse.a), f(inverse.b), f(inverse.c), f(inverse.d),
                    f(m.0), f(m.1), f(m.2), f(m.3), f(m.4), f(m.5),
                    x0, y0, x1 - x0, y1 - y0,
                    width as u32,
                    match layer.blend {
                        BlendMode::Normal => 0,
                        BlendMode::Multiply => 1,
                        BlendMode::Screen => 2,
                        BlendMode::Add => 3,
                    },
                    matte.is_some() as u32,
                    layer.opacity.to_bits(),
                ];
                let mut source = self.resident(&mut uploads, &layer.source, cache.name_of(&layer.source));
                if let Some(r) = layer.radial {
                    source = self.blurred(&mut blurs, &layer.source, &source, r);
                }
                let matte = matte.map(|(m, _)| self.resident(&mut uploads, &m.source, cache.name_of(&m.source)));
                layers.push((source, matte, numbers, (x1 - x0, y1 - y0)));
            }
        });

        let drawn = perf::time(Stage::GpuDraw, || {
            let stride = 80usize.next_multiple_of(self.limits.min_uniform_buffer_offset_alignment as usize);
            let mut numbers = vec![0u32; layers.len().max(1) * stride / 4];
            for (i, layer) in layers.iter().enumerate() {
                numbers[i * stride / 4..i * stride / 4 + 20].copy_from_slice(&layer.2);
            }
            let uniforms = self.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("B-44 layers"),
                contents: bytemuck::cast_slice(&numbers),
                usage: wgpu::BufferUsages::UNIFORM,
            });
            self.target(width, height);
            let target = self.target.as_ref().expect("made above");
            let groups: Vec<wgpu::BindGroup> = layers
                .iter()
                .enumerate()
                .map(|(i, (source, matte, _, _))| {
                    self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                        label: None,
                        layout: &self.layer_layout,
                        entries: &[
                            wgpu::BindGroupEntry {
                                binding: 0,
                                resource: wgpu::BindingResource::Buffer(wgpu::BufferBinding {
                                    buffer: &uniforms,
                                    offset: (i * stride) as u64,
                                    size: wgpu::BufferSize::new(80),
                                }),
                            },
                            wgpu::BindGroupEntry {
                                binding: 1,
                                resource: wgpu::BindingResource::TextureView(source),
                            },
                            wgpu::BindGroupEntry {
                                binding: 2,
                                resource: wgpu::BindingResource::TextureView(
                                    matte.as_ref().unwrap_or(&self.no_matte),
                                ),
                            },
                            wgpu::BindGroupEntry { binding: 3, resource: target.sum.as_entire_binding() },
                        ],
                    })
                })
                .collect();
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
            if !blurs.is_empty() {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&self.radial);
                for (group, (w, h)) in &blurs {
                    pass.set_bind_group(0, group, &[]);
                    pass.dispatch_workgroups(w.div_ceil(16), h.div_ceil(16), 1);
                }
            }
            {
                let mut pass = encoder.begin_compute_pass(&Default::default());
                pass.set_pipeline(&self.layer);
                for (group, (_, _, _, (w, h))) in groups.iter().zip(&layers) {
                    pass.set_bind_group(0, group, &[]);
                    pass.dispatch_workgroups(w.div_ceil(16), h.div_ceil(16), 1);
                }
                pass.set_pipeline(&self.encode);
                pass.set_bind_group(0, &encode_group, &[]);
                pass.dispatch_workgroups((width as u32).div_ceil(16), (height as u32).div_ceil(16), 1);
            }
            // The drawings arrive before the frame that draws with them: one queue, in order.
            self.queue.submit([uploads.finish(), encoder.finish()]);

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
        // Whatever the card was holding may be what failed; start again from nothing.
        self.store.clear();
        self.target = None;
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
