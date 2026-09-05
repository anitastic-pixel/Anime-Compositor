//! Render trace mode, per ADR-012: every intermediate layer buffer written as a tagged PNG.
//!
//! ADR-012's justification is the verification model of ADR-013. The owner cannot read code
//! and an agent in a terminal cannot see the screen, so "the composite is wrong" would
//! otherwise be an argument between two parties who each cannot check the other. A directory
//! of intermediate images turns that into a looking problem.
//!
//! Three constraints from the ADR are structural here rather than remembered. Trace is never
//! on by default: it is a separate entry point, [`render_traced`], and [`crate::render::render`]
//! cannot write a file. It is bounded: one call traces exactly one composition frame, named in
//! the request, so nothing can be "left running". And every image is tagged, in PNG `iTXt`
//! chunks and again in a manifest, with the layer, the frame, the pipeline stage, and what was
//! done to the pixels to make them viewable.
//!
//! Trace re-renders the stack once per layer rather than reaching inside the tiled renderer.
//! That is O(n^2) in layers, and it is the point: the stage images are produced by the same
//! [`crate::render::render`] the real frame comes from, so a trace cannot drift from what it
//! claims to be tracing. `verification/B-05b_trace_table.md` checks that the last composite
//! stage is byte-identical to the untraced render. A diagnostic that re-implemented the
//! pipeline in order to observe it would be able to disagree with it, which is the one thing
//! this facility must not do.

use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::model::Id;
use crate::render::{render, FramePlan, LayerDraw};
use crate::WorkingBuffer;

/// A stage of document 21's layer render order that this build actually produces a buffer for.
///
/// Document 21 lists seven stages. Four of them do not exist yet: the polygon mask (step 2),
/// layer effects (step 3) and the alpha matte (step 5).
/// There is deliberately no variant for those. A trace directory that contained an `effects`
/// image identical to its `decode` image would be a lie told in pictures, which is exactly the
/// silent fidelity fallback document 28 forbids. [`missing_stages`] states the absence in
/// words instead, in the manifest the owner reads.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /// Step 1, in layer space: the selected source drawing, decoded into the working space.
    Decode,
    /// Step 4, in composition space: the layer resampled through its transform, opacity not
    /// yet applied.
    Transform,
    /// Step 6, in composition space: after multiplying by animated layer opacity.
    Opacity,
    /// Step 7, in composition space: the accumulated frame with this layer composited over it.
    Composite,
}

impl Stage {
    pub fn tag(self) -> &'static str {
        match self {
            Stage::Decode => "decode",
            Stage::Transform => "transform",
            Stage::Opacity => "opacity",
            Stage::Composite => "composite",
        }
    }

    /// The step number in document 21's "Layer render order" list.
    pub fn document_21_step(self) -> u32 {
        match self {
            Stage::Decode => 1,
            Stage::Transform => 4,
            Stage::Opacity => 6,
            Stage::Composite => 7,
        }
    }

    pub const ALL: [Stage; 4] = [
        Stage::Decode,
        Stage::Transform,
        Stage::Opacity,
        Stage::Composite,
    ];
}

/// The stages of document 21's layer render order this build does not implement, and why.
///
/// The manifest prints these so a trace directory can never be read as a complete pipeline.
pub fn missing_stages() -> [(u32, &'static str, &'static str); 3] {
    [
        (
            2,
            "layer polygon mask",
            "R-04, PARKED under D-12 in document 23",
        ),
        (3, "ordered layer effects", "R-05, B-07, PARKED under D-12"),
        (
            5,
            "referenced alpha matte",
            "R-04, PARKED; the model records mattes, the renderer does not apply them",
        ),
    ]
}

/// One traced frame: where to write, and which composition frame is being traced.
///
/// ADR-012: "It is bounded by an explicit frame range, never left running." One request is one
/// frame. A range is the caller's loop, and the caller has to write the loop down.
#[derive(Clone, Debug)]
pub struct TraceRequest {
    pub dir: PathBuf,
    pub frame: i32,
}

/// One written image, as recorded in the manifest.
#[derive(Clone, Debug)]
pub struct TracedImage {
    pub path: PathBuf,
    pub layer_index: usize,
    pub layer_id: Id,
    pub stage: Stage,
    pub width: usize,
    pub height: usize,
}

/// Render one frame and write every intermediate layer buffer as a tagged PNG.
///
/// Returns the finished frame, which is byte-identical to
/// `render(plan, tile_size)`, and the manifest of what was written.
pub fn render_traced(
    plan: &FramePlan,
    tile_size: usize,
    request: &TraceRequest,
) -> io::Result<(WorkingBuffer, Vec<TracedImage>)> {
    let dir = request.dir.join(format!("frame_{:05}", request.frame));
    fs::create_dir_all(&dir)?;

    let mut written = Vec::new();
    for (index, layer) in plan.layers.iter().enumerate() {
        // Step 1. The source is already in the working space; this is what the decoder handed
        // the renderer, at the layer's own extent, before any geometry.
        write_stage(
            &dir,
            &mut written,
            index,
            layer,
            Stage::Decode,
            &layer.source,
        )?;

        // Step 4. One layer alone, through its transform, at full opacity.
        let transform_only = FramePlan {
            width: plan.width,
            height: plan.height,
            layers: vec![LayerDraw {
                opacity: 1.0,
                ..layer.clone()
            }],
        };
        let transformed = render(&transform_only, tile_size);
        write_stage(
            &dir,
            &mut written,
            index,
            layer,
            Stage::Transform,
            &transformed,
        )?;

        // Step 6. The same layer alone, with its animated opacity applied.
        let with_opacity = FramePlan {
            width: plan.width,
            height: plan.height,
            layers: vec![layer.clone()],
        };
        let faded = render(&with_opacity, tile_size);
        write_stage(&dir, &mut written, index, layer, Stage::Opacity, &faded)?;

        // Step 7. The stack up to and including this layer. At the last layer this is the
        // whole frame, which is why the final composite image and the returned frame are the
        // same render rather than two renders that agree.
        let stack = FramePlan {
            width: plan.width,
            height: plan.height,
            layers: plan.layers[..=index].to_vec(),
        };
        let composited = render(&stack, tile_size);
        write_stage(
            &dir,
            &mut written,
            index,
            layer,
            Stage::Composite,
            &composited,
        )?;
    }

    let frame = render(plan, tile_size);
    write_png(
        &dir.join("frame.png"),
        &frame,
        &[
            ("Frame", request.frame.to_string()),
            ("Stage", "frame".to_string()),
        ],
    )?;
    fs::write(dir.join("manifest.md"), manifest(plan, request, &written))?;
    Ok((frame, written))
}

fn write_stage(
    dir: &Path,
    written: &mut Vec<TracedImage>,
    index: usize,
    layer: &LayerDraw,
    stage: Stage,
    buffer: &WorkingBuffer,
) -> io::Result<()> {
    let path = dir.join(format!(
        "layer{index:02}_{}_{}.png",
        safe_name(layer.id.as_str()),
        stage.tag()
    ));
    write_png(
        &path,
        buffer,
        &[
            ("Layer", layer.id.to_string()),
            ("LayerIndex", index.to_string()),
            ("Stage", stage.tag().to_string()),
            ("Document21Step", stage.document_21_step().to_string()),
        ],
    )?;
    written.push(TracedImage {
        path,
        layer_index: index,
        layer_id: layer.id.clone(),
        stage,
        width: buffer.width(),
        height: buffer.height(),
    });
    Ok(())
}

/// A layer ID reduced to something every filesystem accepts, without pretending to be the ID.
///
/// Document 19 puts no character restriction on an ID, and the reference shot already proves
/// non-ASCII names reach this code: `layer2_桜_013.png` is a required fixture. The file name is
/// a label; the manifest carries the exact ID, and [`TracedImage::layer_id`] carries it in
/// memory. Nothing downstream parses a trace file name back into an ID.
fn safe_name(id: &str) -> String {
    let cleaned: String = id
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() || c == '-' || c == '_' {
                c
            } else {
                '-'
            }
        })
        .collect();
    let trimmed = cleaned.trim_matches('-');
    if trimmed.is_empty() {
        "unnamed".to_string()
    } else {
        trimmed.chars().take(48).collect()
    }
}

/// Write a working buffer as an 8-bit sRGB PNG, tagged with what was done to it.
///
/// The pixels are converted for viewing: unpremultiplied in linear light, through the sRGB
/// transfer function, quantised to 8 bits, written with straight alpha, exactly as
/// [`WorkingBuffer::to_srgb8_straight`] does for export. A linear-light file would be viewable
/// but wrong-looking, and a trace image nobody can read defeats the purpose. The conversion is
/// not silent: `ColorSpace` and `AlphaMode` chunks state the encoding of the file, and
/// `WorkingSpace` states what it was converted from.
pub fn write_png(path: &Path, buffer: &WorkingBuffer, tags: &[(&str, String)]) -> io::Result<()> {
    let file = io::BufWriter::new(fs::File::create(path)?);
    let mut encoder = png::Encoder::new(file, buffer.width() as u32, buffer.height() as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let fixed = [
        ("Software", "anime_compositor render trace (ADR-012)"),
        ("ColorSpace", "sRGB IEC 61966-2-1, 8 bits per channel"),
        ("AlphaMode", "Straight"),
        (
            "WorkingSpace",
            "converted from linear light, premultiplied, float32",
        ),
    ];
    // iTXt, not tEXt: a layer ID is UTF-8 and the reference shot already contains one that
    // Latin-1 cannot hold. `layer2_桜_013.png` is a required fixture, so a tag chunk that
    // could only carry Latin-1 would fail on the owner's own drawings.
    for (key, value) in fixed {
        encoder
            .add_itxt_chunk(key.to_string(), value.to_string())
            .map_err(io::Error::other)?;
    }
    for (key, value) in tags {
        encoder
            .add_itxt_chunk((*key).to_string(), value.clone())
            .map_err(io::Error::other)?;
    }
    encoder
        .write_header()
        .map_err(io::Error::other)?
        .write_image_data(&buffer.to_srgb8_straight())
        .map_err(io::Error::other)?;
    Ok(())
}

fn manifest(plan: &FramePlan, request: &TraceRequest, written: &[TracedImage]) -> String {
    use std::fmt::Write as _;
    let mut s = format!(
        "# Render trace, composition frame {}\n\nADR-012. Diagnostic output. Not part of \
         export, not committed, and produced only by an explicit call to `render_traced`.\n\n\
         Output extent {}x{}, {} layer(s), bottom of the stack first.\n\n",
        request.frame,
        plan.width,
        plan.height,
        plan.layers.len()
    );

    s.push_str(
        "## What these images are\n\nEvery PNG here is 8-bit sRGB with straight alpha, \
         converted from the linear-light premultiplied float32 buffer the renderer actually \
         holds. The conversion is the same one export uses. Each file repeats that in its own \
         PNG text chunks, so a file separated from this manifest still says what it is.\n\n\
         `decode` is at the layer's own size; every later stage is at the composition size.\n\n",
    );

    s.push_str("## Files\n\n| File | Layer | Layer ID | Stage | Document 21 step | Size |\n");
    s.push_str("|---|---|---|---|---|---|\n");
    for image in written {
        let name = image
            .path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        let _ = writeln!(
            s,
            "| `{}` | {} | `{}` | {} | {} | {}x{} |",
            name,
            image.layer_index,
            image.layer_id,
            image.stage.tag(),
            image.stage.document_21_step(),
            image.width,
            image.height
        );
    }
    let _ = writeln!(
        s,
        "| `frame.png` | - | - | finished frame | 7 | {}x{} |",
        plan.width, plan.height
    );

    s.push_str(
        "\n## Stages this build does not have\n\nDocument 21's layer render order has seven \
         steps. This trace shows four. The missing steps are absent from the renderer, not \
         omitted from the trace, and no image here stands in for one.\n\n\
         | Step | Stage | Why it is not here |\n|---|---|---|\n",
    );
    for (step, name, why) in missing_stages() {
        let _ = writeln!(s, "| {step} | {name} | {why} |");
    }

    s.push_str(
        "\n## How to read a wrong frame\n\nWalk up the stack. `decode` wrong means the import \
         or the exposure sheet picked the wrong drawing, which is B-03 and B-04. `decode` \
         right and `transform` wrong means the anchor, position, scale or rotation is wrong, \
         which is the layer's own numbers or B-05a. `transform` right and `opacity` wrong \
         means the opacity property. Every stage right for a layer but the `composite` after \
         it wrong means the stacking order or the compositing itself, which is B-02.\n",
    );
    s
}
