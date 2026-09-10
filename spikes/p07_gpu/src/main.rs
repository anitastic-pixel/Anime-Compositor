//! P-07: one GPU tile, and the number the decision needs. Quarantined per document 06.
//!
//! Nothing here ships and nothing here is reused. It exists to produce one page,
//! `verification/P-07_gpu_distance.md`, holding three numbers the owner does not have:
//!
//! 1. how far a GPU render of the H-01 fixture lands from the CPU render, over all 2,073,600
//!    pixels, in linear code values and in the eight-bit code values a person would see;
//! 2. what one 1080p f32 frame costs to put on the card and take back off it, because a design
//!    that moves 33 MB each way per frame can spend its winnings on the bus;
//! 3. what `Affine`'s f64 becomes in a shading language whose runtime floats are f32, on a large
//!    translation and on a near-singular scale.
//!
//! The GPU side is `src/tile.wgsl`, a longhand translation of `src/render.rs::render_tile` and
//! `src/composite.rs::blend_pixel`. It shares no code with them on purpose: the distance between
//! two implementations is the measurement.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use anime_compositor::command::{Command, Document};
use anime_compositor::compose::{plan_frame, DEFAULT_TILE_SIZE};
use anime_compositor::diagnostics::FrameLog;
use anime_compositor::media::import_sequence;
use anime_compositor::model::{Asset, AssetKind, BlendMode, Composition, Id, Layer, Project};
use anime_compositor::render::{render, Affine, FramePlan};
use anime_compositor::time::{ExposureMap, ExposureSpan, FrameRate};
use anime_compositor::WorkingBuffer;

const COMP: &str = "comp-reference-shot";
const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const FRAMES: [i32; 4] = [0, 14, 100, 239];

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .join(rel)
}

fn root() -> PathBuf {
    repo("Fixtures/reference_shot")
}

fn id(text: &str) -> Id {
    Id::new(text)
}

// ---------------------------------------------------------------------------------------
// The GPU side
// ---------------------------------------------------------------------------------------

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct LayerUniform {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    tx: f32,
    ty: f32,
    src_w: u32,
    src_h: u32,
    frame_w: u32,
    frame_h: u32,
    opacity: f32,
    blend: u32,
}

struct Gpu {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pipeline: wgpu::ComputePipeline,
    adapter: String,
    backend: String,
}

impl Gpu {
    fn new() -> Option<Gpu> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter =
            pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions::default()))
                .ok()?;
        let info = adapter.get_info();
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("p07"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .ok()?;
        let module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("tile"),
            source: wgpu::ShaderSource::Wgsl(include_str!("tile.wgsl").into()),
        });
        let pipeline = device.create_compute_pipeline(&wgpu::ComputePipelineDescriptor {
            label: Some("tile"),
            layout: None,
            module: &module,
            entry_point: Some("main"),
            compilation_options: Default::default(),
            cache: None,
        });
        Some(Gpu {
            device,
            queue,
            pipeline,
            adapter: format!("{} ({:?})", info.name, info.device_type),
            backend: format!("{:?}", info.backend),
        })
    }

    /// The whole layer stack, bottom first, one dispatch a layer over one frame-sized buffer.
    fn render(&self, plan: &FramePlan) -> WorkingBuffer {
        let bytes = (plan.width * plan.height * 16) as u64;
        let frame = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("frame"),
            size: bytes,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        let layout = self.pipeline.get_bind_group_layout(0);

        for layer in &plan.layers {
            // A singular transform has no source pixel behind any destination pixel, which the
            // CPU renderer treats as an empty draw. Same here: the layer is not dispatched.
            let Some(inverse) = layer.transform.invert() else {
                continue;
            };
            let source = self.storage(layer.source.data());
            let uniform = LayerUniform {
                a: inverse.a as f32,
                b: inverse.b as f32,
                c: inverse.c as f32,
                d: inverse.d as f32,
                tx: inverse.tx as f32,
                ty: inverse.ty as f32,
                src_w: layer.source.width() as u32,
                src_h: layer.source.height() as u32,
                frame_w: plan.width as u32,
                frame_h: plan.height as u32,
                opacity: layer.opacity,
                blend: match layer.blend {
                    BlendMode::Normal => 0,
                    BlendMode::Multiply => 1,
                    BlendMode::Screen => 2,
                    BlendMode::Add => 3,
                },
            };
            let uniform = self.uniform(&uniform);
            let bind = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: None,
                layout: &layout,
                entries: &[
                    wgpu::BindGroupEntry {
                        binding: 0,
                        resource: uniform.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 1,
                        resource: source.as_entire_binding(),
                    },
                    wgpu::BindGroupEntry {
                        binding: 2,
                        resource: frame.as_entire_binding(),
                    },
                ],
            });
            let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
            {
                let mut pass = encoder.begin_compute_pass(&wgpu::ComputePassDescriptor {
                    label: None,
                    timestamp_writes: None,
                });
                pass.set_pipeline(&self.pipeline);
                pass.set_bind_group(0, &bind, &[]);
                pass.dispatch_workgroups(
                    plan.width.div_ceil(8) as u32,
                    plan.height.div_ceil(8) as u32,
                    1,
                );
            }
            self.queue.submit([encoder.finish()]);
            self.device.poll(wgpu::PollType::Wait).expect("the queue");
        }

        let read = self.download(&frame, bytes);
        let mut out = WorkingBuffer::transparent(plan.width, plan.height);
        out.data_mut().copy_from_slice(&read);
        out
    }

    fn storage(&self, data: &[f32]) -> wgpu::Buffer {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: (data.len() * 4) as u64,
            usage: wgpu::BufferUsages::STORAGE | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue
            .write_buffer(&buffer, 0, bytemuck::cast_slice(data));
        buffer
    }

    fn uniform(&self, value: &LayerUniform) -> wgpu::Buffer {
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: std::mem::size_of::<LayerUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        self.queue
            .write_buffer(&buffer, 0, bytemuck::bytes_of(value));
        buffer
    }

    fn download(&self, from: &wgpu::Buffer, bytes: u64) -> Vec<f32> {
        let staging = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: bytes,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        encoder.copy_buffer_to_buffer(from, 0, &staging, 0, bytes);
        self.queue.submit([encoder.finish()]);
        staging.slice(..).map_async(wgpu::MapMode::Read, |r| {
            r.expect("map the frame back");
        });
        self.device.poll(wgpu::PollType::Wait).expect("the queue");
        let view = staging.slice(..).get_mapped_range();
        let out = bytemuck::cast_slice::<u8, f32>(&view).to_vec();
        drop(view);
        staging.unmap();
        out
    }

    /// One 1080p f32 frame up and back, with the queue drained on both sides so the numbers are
    /// the transfer and not the driver deferring it.
    fn transfer_cost(&self) -> (f64, f64) {
        let data = vec![0.5f32; WIDTH * HEIGHT * 4];
        let bytes = (data.len() * 4) as u64;
        let up = Instant::now();
        let buffer = self.device.create_buffer(&wgpu::BufferDescriptor {
            label: None,
            size: bytes,
            usage: wgpu::BufferUsages::STORAGE
                | wgpu::BufferUsages::COPY_DST
                | wgpu::BufferUsages::COPY_SRC,
            mapped_at_creation: false,
        });
        self.queue
            .write_buffer(&buffer, 0, bytemuck::cast_slice(&data));
        self.queue.submit([]);
        self.device.poll(wgpu::PollType::Wait).expect("the queue");
        let upload = up.elapsed().as_secs_f64() * 1000.0;

        let down = Instant::now();
        let back = self.download(&buffer, bytes);
        let download = down.elapsed().as_secs_f64() * 1000.0;
        assert_eq!(back.len(), data.len(), "the frame comes back whole");
        (upload, download)
    }
}

// ---------------------------------------------------------------------------------------
// Comparing two whole pictures
// ---------------------------------------------------------------------------------------

struct Distance {
    pixels: usize,
    largest: f32,
    mean: f64,
    largest_code: u8,
    pixels_code: usize,
}

fn compare(cpu: &WorkingBuffer, gpu: &WorkingBuffer) -> Distance {
    let mut pixels = 0usize;
    let mut largest = 0.0f32;
    let mut total = 0.0f64;
    for (a, b) in cpu.data().iter().zip(gpu.data()) {
        let d = (a - b).abs();
        total += d as f64;
        if d > 0.0 {
            largest = largest.max(d);
        }
    }
    for y in 0..cpu.height() {
        for x in 0..cpu.width() {
            if cpu.pixel(x, y) != gpu.pixel(x, y) {
                pixels += 1;
            }
        }
    }
    // What a person would see: the same eight-bit sRGB encoding the export writes.
    let (a, b) = (cpu.to_srgb8_straight(), gpu.to_srgb8_straight());
    let mut largest_code = 0u8;
    let mut pixels_code = 0usize;
    for (chunk_a, chunk_b) in a.chunks(4).zip(b.chunks(4)) {
        let worst = chunk_a
            .iter()
            .zip(chunk_b)
            .map(|(x, y)| x.abs_diff(*y))
            .max()
            .unwrap_or(0);
        if worst > 0 {
            pixels_code += 1;
            largest_code = largest_code.max(worst);
        }
    }
    Distance {
        pixels,
        largest,
        mean: total / cpu.data().len() as f64,
        largest_code,
        pixels_code,
    }
}

// ---------------------------------------------------------------------------------------
// The H-01 fixture, rebuilt here rather than borrowed from `tests/h01_whole_picture.rs`
// ---------------------------------------------------------------------------------------

fn layer4_sheet() -> (Vec<u32>, Vec<u32>) {
    let text =
        fs::read_to_string(root().join("exposure_sheet.json")).expect("read the exposure sheet");
    let sheet: serde_json::Value = serde_json::from_str(&text).expect("the sheet is JSON");
    let array = |key: &str| -> Vec<u32> {
        sheet[key]
            .as_array()
            .unwrap_or_else(|| panic!("{key} is not an array"))
            .iter()
            .map(|n| n.as_u64().expect("a whole number") as u32)
            .collect()
    };
    (
        array("layer4_exposure_drawing_ids"),
        array("layer4_exposure_lengths"),
    )
}

fn spans(layer: u32) -> Vec<ExposureSpan> {
    let (drawings, lengths) = layer4_sheet();
    let exposures: Vec<(u32, u32)> = match layer {
        1 => vec![(0, 240)],
        2 => (0..240).map(|f| (f % 24, 1)).collect(),
        3 => (0..120).map(|k| (k % 12, 2)).collect(),
        4 => drawings.into_iter().zip(lengths).collect(),
        _ => unreachable!(),
    };
    ExposureMap::from_lengths(&exposures)
        .expect("the cadences are disjoint and in order")
        .spans()
        .to_vec()
}

fn asset_for(layer: u32) -> Asset {
    let dir = root().join(format!("layer{layer}"));
    let files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.expect("directory entry").path())
        .filter(|p| p.extension().is_some_and(|e| e == "png"))
        .collect();
    let imported = import_sequence(&files)
        .asset
        .unwrap_or_else(|| panic!("layer{layer} imports as a sequence"));
    let frames: BTreeMap<u32, String> = imported
        .frames()
        .iter()
        .map(|(number, path)| {
            let name = path
                .file_name()
                .and_then(|n| n.to_str())
                .expect("a UTF-8 file name");
            (*number, format!("layer{layer}/{name}"))
        })
        .collect();
    Asset {
        id: id(&format!("asset-layer{layer}")),
        kind: AssetKind::ImageSequence,
        name: format!("layer{layer}"),
        path: None,
        pattern: Some(imported.pattern().to_string()),
        frames,
        interpretation: Default::default(),
    }
}

fn build_project() -> Project {
    let mut project = Project::new(id("proj-p07-gpu"));
    project.compositions.push(Composition::new(
        id(COMP),
        "reference shot",
        WIDTH as u32,
        HEIGHT as u32,
        FrameRate::new(24, 1).expect("24 fps"),
        0,
        240,
    ));
    let mut doc = Document::new(project);
    for n in 1..=4u32 {
        let asset = asset_for(n);
        let mut layer = Layer::new(
            id(&format!("layer-{n}")),
            format!("layer{n}"),
            asset.id.clone(),
            0,
            240,
        );
        layer.exposure_spans = spans(n);
        doc.apply_all(vec![
            Command::AddAsset { asset },
            Command::AddLayer {
                composition: id(COMP),
                layer: Box::new(layer),
                index: (n - 1) as usize,
            },
        ])
        .expect("the opening project is valid");
    }
    doc.project().clone()
}

fn plan(project: &Project, frame: i32) -> FramePlan {
    let mut log = FrameLog::new(8);
    plan_frame(project, &id(COMP), frame, &root(), &mut log)
        .unwrap_or_else(|d| panic!("frame {frame} plans: {}", d.message))
}

// ---------------------------------------------------------------------------------------
// The page
// ---------------------------------------------------------------------------------------

/// How far apart the f64 inverse and an all-f32 inverse put the source pixel behind a destination
/// pixel, in source pixels, over the four corners of a 1920x1080 frame.
///
/// This is the question `Affine`'s f64 actually poses for a shading language, and it is asked
/// here rather than through a rendered frame because the interesting transforms - a near-singular
/// scale above all - collapse the layer to nothing, and a comparison of two empty frames says
/// nothing about arithmetic.
fn worst_shift(t: Affine) -> f64 {
    let Some(exact) = t.invert() else {
        return f64::NAN;
    };
    // `Affine::invert`, transcribed with every operand an f32, which is what the card has.
    let (a, b, c, d, tx, ty) = (
        t.a as f32,
        t.b as f32,
        t.c as f32,
        t.d as f32,
        t.tx as f32,
        t.ty as f32,
    );
    let inv = 1.0f32 / (a * d - b * c);
    let (ia, ib, ic, id) = (d * inv, -b * inv, -c * inv, a * inv);
    let (itx, ity) = ((c * ty - d * tx) * inv, (b * tx - a * ty) * inv);

    let mut worst = 0.0f64;
    for (x, y) in [
        (0.5f64, 0.5f64),
        (WIDTH as f64 - 0.5, 0.5),
        (0.5, HEIGHT as f64 - 0.5),
        (WIDTH as f64 - 0.5, HEIGHT as f64 - 0.5),
    ] {
        let (ex, ey) = exact.apply(x, y);
        let (gx, gy) = (
            ia * x as f32 + ic * y as f32 + itx,
            ib * x as f32 + id * y as f32 + ity,
        );
        let d = ((ex - gx as f64).powi(2) + (ey - gy as f64).powi(2)).sqrt();
        worst = worst.max(d);
    }
    worst
}

fn row(what: &str, d: &Distance) -> String {
    format!(
        "| {what} | {} | {:.3e} | {:.3e} | {} | {} |\n",
        d.pixels, d.largest, d.mean, d.pixels_code, d.largest_code
    )
}

fn main() {
    let Some(gpu) = Gpu::new() else {
        eprintln!("no GPU adapter on this machine; nothing measured");
        std::process::exit(1);
    };
    let project = build_project();

    let mut table = String::new();
    for frame in FRAMES {
        let plan = plan(&project, frame);
        let cpu = render(&plan, DEFAULT_TILE_SIZE);
        let gpu_frame = gpu.render(&plan);
        table.push_str(&row(
            &format!("H-01 frame {frame}"),
            &compare(&cpu, &gpu_frame),
        ));
    }

    // What `Affine`'s f64 becomes in a language whose runtime floats are f32. The rendered case
    // is built on the H-01 stack at frame 0, so only the geometry changes.
    let mut geometry = String::new();

    // A rotation about an anchor 8 388 608 px off canvas - the first integer f32 cannot follow
    // by halves. The layer stays on screen, so this is a picture and not a thought experiment.
    let far = Affine::from_transform((8_388_608.0, 0.0), (8_388_608.0, 0.0), (1.0, 1.0), 0.001);
    let mut moved = plan(&project, 0);
    for layer in &mut moved.layers {
        layer.transform = layer.transform.then(far);
    }
    let cpu = render(&moved, DEFAULT_TILE_SIZE);
    let on_card = gpu.render(&moved);
    geometry.push_str(&row(
        "rotated 0.001 deg about an anchor 8 388 608 px off canvas",
        &compare(&cpu, &on_card),
    ));

    // The same transforms inverted twice - once as the renderer does it, in f64, and once as a
    // shading language would, in f32 throughout - and asked the only question that matters: how
    // far apart do the two put the source pixel behind a destination pixel.
    let mut shifts = String::new();
    for (what, t) in [
        ("the identity, for scale", Affine::IDENTITY),
        (
            "rotated 0.001 deg about an anchor 8 388 608 px off canvas",
            far,
        ),
        (
            "a near-singular scale, 1e-6 on x and 1.0 on y",
            Affine::scaling(1e-6, 1.0),
        ),
        (
            "the same scale, then scaled back up by 1e6 on x",
            Affine::scaling(1e-6, 1.0).then(Affine::scaling(1e6, 1.0)),
        ),
    ] {
        shifts.push_str(&format!("| {what} | {:.4e} |\n", worst_shift(t)));
    }

    let (upload, download) = gpu.transfer_cost();

    let page = format!(
        include_str!("page.md"),
        adapter = gpu.adapter,
        backend = gpu.backend,
        table = table,
        geometry = geometry,
        shifts = shifts,
        upload = upload,
        roundtrip = upload + download,
        download = download,
        megabytes = (WIDTH * HEIGHT * 16) as f64 / (1024.0 * 1024.0),
    );
    let out = repo("verification/P-07_gpu_distance.md");
    fs::write(&out, page).expect("write the artifact");
    println!("wrote {}", out.display());
}
