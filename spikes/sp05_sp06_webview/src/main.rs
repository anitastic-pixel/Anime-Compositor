#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

//! G0 spike: SP-05 frame transport into WebView2, SP-06 viewer colour exactness.
//!
//! Quarantined per document 06. This answers two questions and is not production code:
//!   SP-05  How fast can RGBA frames cross the Rust -> WebView2 boundary, by transport?
//!   SP-06  Does the webview alter the bytes it is given?
//!
//! SP-03, added here rather than in a fourth binary: the scrub latency question is about
//! the whole path from input to presented pixels, and that path only exists inside the
//! webview. Every scrub step composites its 1080p frame on demand; see src/render.rs.
//!
//! SP-06 expected values are the input bytes themselves. Document 25 specifies
//! linear-light float compositing results, not display-side 8-bit sRGB values, so
//! there is no display fixture to compare against. The risk in K-06 is that the
//! webview CHANGES colour, and identity is the correct test for that. No expected
//! value is invented here.

mod render;

use std::sync::Arc;

const FULL: (u32, u32) = (1920, 1080);
const DRAFT: (u32, u32) = (480, 270);
const FRAME_COUNT: usize = 6;

/// SP-06 probe colours, straight RGBA8. Chosen for the ways a display path goes wrong:
/// endpoints, both mid-greys, primaries (colour-management shifts show here first),
/// the sRGB encoding of linear 0.5, and partial alpha.
const PROBE: &[[u8; 4]] = &[
    [0, 0, 0, 255],
    [255, 255, 255, 255],
    [128, 128, 128, 255],
    [127, 127, 127, 255],
    [188, 188, 188, 255],
    [255, 0, 0, 255],
    [0, 255, 0, 255],
    [0, 0, 255, 255],
    [255, 255, 0, 255],
    [0, 255, 255, 255],
    [255, 0, 255, 255],
    [1, 2, 3, 255],
    [254, 253, 252, 255],
    [255, 0, 0, 128],
    [0, 0, 0, 128],
    [64, 96, 160, 255],
];
const PROBE_TILE: u32 = 8;

struct Frames {
    full: Vec<Arc<Vec<u8>>>,
    draft: Vec<Arc<Vec<u8>>>,
}

impl Frames {
    fn generate() -> Self {
        Frames {
            full: (0..FRAME_COUNT).map(|i| Arc::new(synth(FULL, i))).collect(),
            draft: (0..FRAME_COUNT).map(|i| Arc::new(synth(DRAFT, i))).collect(),
        }
    }

    fn get(&self, scale: &str, idx: usize) -> Arc<Vec<u8>> {
        let set = if scale == "draft" { &self.draft } else { &self.full };
        set[idx % set.len()].clone()
    }
}

/// Deterministic synthetic frame. Content is irrelevant to the measurement; it only
/// has to differ per frame so a stalled transport is visible rather than silent.
fn synth((w, h): (u32, u32), i: usize) -> Vec<u8> {
    let mut buf = vec![0u8; (w * h * 4) as usize];
    let phase = (i * 40) as u32;
    for y in 0..h {
        for x in 0..w {
            let o = ((y * w + x) * 4) as usize;
            buf[o] = ((x * 255 / w.max(1)) as u8).wrapping_add(phase as u8);
            buf[o + 1] = (y * 255 / h.max(1)) as u8;
            buf[o + 2] = 128;
            buf[o + 3] = 255;
        }
    }
    buf
}

/// SP-06 probe image: one flat tile per probe colour, laid out in a single row.
fn probe_rgba() -> Vec<u8> {
    let w = PROBE.len() as u32 * PROBE_TILE;
    let h = PROBE_TILE;
    let mut buf = vec![0u8; (w * h * 4) as usize];
    for y in 0..h {
        for x in 0..w {
            let c = PROBE[(x / PROBE_TILE) as usize];
            let o = ((y * w + x) * 4) as usize;
            buf[o..o + 4].copy_from_slice(&c);
        }
    }
    buf
}

// --- Transport A: JSON IPC. The naive baseline document 06 expects to be too slow. ---
#[tauri::command]
fn frame_json(state: tauri::State<Arc<Frames>>, scale: String, idx: usize) -> Vec<u8> {
    state.get(&scale, idx).as_ref().clone()
}

// --- Transport B: raw bytes over IPC, no JSON encoding. ---
#[tauri::command]
fn frame_raw(state: tauri::State<Arc<Frames>>, scale: String, idx: usize) -> tauri::ipc::Response {
    tauri::ipc::Response::new(state.get(&scale, idx).as_ref().clone())
}

// --- SP-06 probe, delivered as raw bytes and as PNG. ---
#[tauri::command]
fn probe_raw() -> tauri::ipc::Response {
    tauri::ipc::Response::new(probe_rgba())
}

#[tauri::command]
fn probe_spec() -> serde_json::Value {
    serde_json::json!({
        "colors": PROBE,
        "tile": PROBE_TILE,
        "width": PROBE.len() as u32 * PROBE_TILE,
        "height": PROBE_TILE,
    })
}

#[tauri::command]
fn probe_png() -> tauri::ipc::Response {
    let w = PROBE.len() as u32 * PROBE_TILE;
    let mut out: Vec<u8> = Vec::new();
    {
        let mut enc = png::Encoder::new(&mut out, w, PROBE_TILE);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        // No gAMA, no cHRM, no iCCP, no sRGB chunk: an untagged PNG, which document 21
        // says defaults to sRGB. Tagging is a separate question from transport fidelity.
        let mut writer = enc.write_header().unwrap();
        writer.write_image_data(&probe_rgba()).unwrap();
    }
    tauri::ipc::Response::new(out)
}

// --- SP-03: composite one 1080p frame on demand for a scrub step. ---
//
// The response is 8 bytes of little-endian f64 core-composite milliseconds, followed by
// W*H*4 straight RGBA8. The split is carried in-band because the caller has to attribute
// the latency it measures: render, transport and draw are three different problems with
// three different fixes, and a single total number identifies none of them.
#[tauri::command]
fn scrub_frame(idx: usize) -> tauri::ipc::Response {
    let (rgba, core_ms) = render::composite_rgba8(idx);
    let mut out = Vec::with_capacity(8 + rgba.len());
    out.extend_from_slice(&core_ms.to_le_bytes());
    out.extend_from_slice(&rgba);
    tauri::ipc::Response::new(out)
}

/// Writes the measured results where the session can read them and the owner can keep
/// them. `spike-output/` is gitignored diagnostic output, never committed.
#[tauri::command]
fn save_report(name: String, body: String) -> Result<String, String> {
    let safe: String = name.chars().filter(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '.' || *c == '-').collect();
    if safe.is_empty() {
        return Err("empty report name".into());
    }
    let dir = std::path::Path::new("spike-output");
    std::fs::create_dir_all(dir).map_err(|e| e.to_string())?;
    let path = dir.join(safe);
    std::fs::write(&path, body).map_err(|e| e.to_string())?;
    Ok(std::fs::canonicalize(&path).map_err(|e| e.to_string())?.display().to_string())
}

#[tauri::command]
fn env_report() -> serde_json::Value {
    serde_json::json!({
        "rustc": option_env!("SPIKE_RUSTC").unwrap_or("see spike report"),
        "profile": if cfg!(debug_assertions) { "debug" } else { "release" },
        "full": [FULL.0, FULL.1],
        "draft": [DRAFT.0, DRAFT.1],
        "frame_count": FRAME_COUNT,
        "scrub_size": [render::W, render::H],
    })
}

fn main() {
    // One frame set, shared by the IPC commands and the custom protocol handler.
    let frames = Arc::new(Frames::generate());
    // Transport C: custom URI scheme. The webview fetches frames as ordinary resources.
    let proto = frames.clone();

    tauri::Builder::default()
        .manage(frames)
        .register_uri_scheme_protocol("frame", move |_ctx, request| {
            // Path is /<scale>/<index>, e.g. /full/3
            let path = request.uri().path().to_string();
            let mut parts = path.trim_start_matches('/').split('/');
            let scale = parts.next().unwrap_or("full").to_string();
            let idx: usize = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
            let body = proto.get(&scale, idx).as_ref().clone();
            tauri::http::Response::builder()
                .header("Content-Type", "application/octet-stream")
                .header("Cache-Control", "no-store")
                .header("Access-Control-Allow-Origin", "*")
                .body(body)
                .unwrap()
        })
        .invoke_handler(tauri::generate_handler![
            frame_json,
            frame_raw,
            scrub_frame,
            probe_raw,
            probe_png,
            probe_spec,
            save_report,
            env_report
        ])
        .run(tauri::generate_context!())
        .expect("spike failed to start");
}
