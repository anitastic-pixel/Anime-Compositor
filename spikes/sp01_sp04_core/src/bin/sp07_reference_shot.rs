//! SP-07 - render the reference shot.
//!
//! ADR-006's exit condition speaks of "a measured result on a real shot on the reference
//! machine". Every number in the B-01 report so far came from synthetic layers. This
//! renders the actual 240-frame reference shot from `Fixtures/reference_shot/` and measures
//! where the time goes.
//!
//! QUARANTINED per document 06. This is a measurement spike, not production code, and it is
//! discarded at integration.
//!
//! IMPORTANT - what this does NOT establish. The colour arithmetic here is provisional and
//! is B-02's job, not this spike's. Nothing in this file is a correctness claim: its
//! expected values live in document 25 and are not consulted here. What this spike measures
//! is cost and determinism on real media. Do not cite it as evidence that the compositing
//! math is right.

use std::collections::HashMap;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rayon::prelude::*;

const W: usize = 1920;
const H: usize = 1080;
const TILE: usize = 64;
const FRAMES: usize = 240;

/// sRGB electro-optical transfer. Built once as a 256-entry table; the per-pixel cost of
/// calling powf 8 million times a frame would swamp what this spike is trying to measure.
fn srgb_lut() -> [f32; 256] {
    let mut t = [0f32; 256];
    for (i, v) in t.iter_mut().enumerate() {
        let c = i as f32 / 255.0;
        *v = if c <= 0.04045 { c / 12.92 } else { ((c + 0.055) / 1.055).powf(2.4) };
    }
    t
}

fn linear_to_srgb8(c: f32) -> u8 {
    let c = c.clamp(0.0, 1.0);
    let s = if c <= 0.0031308 { c * 12.92 } else { 1.055 * c.powf(1.0 / 2.4) - 0.055 };
    (s * 255.0 + 0.5) as u8
}

/// One decoded cel: straight-alpha RGBA8, exactly as the PNG stores it.
struct Cel {
    px: Vec<u8>,
}

fn decode(path: &Path) -> Result<Cel, String> {
    let f = std::fs::File::open(path).map_err(|e| format!("{}: {e}", path.display()))?;
    let mut r = png::Decoder::new(f)
        .read_info()
        .map_err(|e| format!("{}: {e}", path.display()))?;
    let mut buf = vec![0u8; r.output_buffer_size()];
    let info = r.next_frame(&mut buf).map_err(|e| format!("{}: {e}", path.display()))?;
    if info.width as usize != W || info.height as usize != H {
        return Err(format!("{}: {}x{}, want {W}x{H}", path.display(), info.width, info.height));
    }
    buf.truncate(info.buffer_size());
    let px = match info.color_type {
        png::ColorType::Rgba => buf,
        png::ColorType::Rgb => buf.chunks_exact(3).flat_map(|c| [c[0], c[1], c[2], 255]).collect(),
        other => return Err(format!("{}: unsupported colour type {other:?}", path.display())),
    };
    Ok(Cel { px })
}

/// Exposure sheet, read straight out of the JSON the fixture ships. Only the four
/// frame-to-drawing lists are needed, so this pulls them out by hand rather than adding a
/// JSON dependency to a quarantined crate for one read.
fn exposure_lists(json: &str) -> HashMap<String, Vec<usize>> {
    let mut out = HashMap::new();
    let anchor = "\"frame_to_drawing\"";
    let start = json.find(anchor).expect("frame_to_drawing missing from exposure sheet");
    let tail = &json[start..];
    for layer in ["layer1", "layer2", "layer3", "layer4"] {
        let key = format!("\"{layer}\"");
        let k = tail.find(&key).unwrap_or_else(|| panic!("{layer} missing from exposure sheet"));
        let open = tail[k..].find('[').unwrap() + k;
        let close = tail[open..].find(']').unwrap() + open;
        let ids: Vec<usize> = tail[open + 1..close]
            .split(',')
            .map(|s| s.trim().parse().expect("non-numeric drawing id"))
            .collect();
        assert_eq!(ids.len(), FRAMES, "{layer} exposure list is not {FRAMES} long");
        out.insert(layer.to_string(), ids);
    }
    out
}

fn cel_path(root: &Path, layer: usize, id: usize) -> PathBuf {
    // Doc 22's deliberate Japanese filename. It must round-trip unchanged; if this lookup
    // ever needs to be relaxed to find the file, that is a defect worth reporting, not
    // working around.
    if layer == 2 && id == 13 {
        return root.join("layer2").join("layer2_桜_013.png");
    }
    root.join(format!("layer{layer}")).join(format!("layer{layer}_{id:03}.png"))
}

/// Composite one tile, back to front, in linear-light premultiplied f32. Pure function of
/// the frame's cel references and the tile bounds, so tile order can never affect output.
fn render_tile(cels: &[Option<&Cel>; 4], lut: &[f32; 256], y0: usize, y1: usize, out: &mut [u8]) {
    for y in y0..y1 {
        for x in 0..W {
            let o = (y * W + x) * 4;
            let (mut r, mut g, mut b, mut a) = (0f32, 0f32, 0f32, 0f32);
            for cel in cels.iter() {
                let Some(cel) = cel else { continue };   // absent drawing: transparent
                let s = &cel.px[o..o + 4];
                let sa = s[3] as f32 / 255.0;
                if sa == 0.0 {
                    continue;
                }
                let inv = 1.0 - sa;
                r = lut[s[0] as usize] * sa + r * inv;
                g = lut[s[1] as usize] * sa + g * inv;
                b = lut[s[2] as usize] * sa + b * inv;
                a = sa + a * inv;
            }
            let po = (y - y0) * W * 4 + x * 4;
            out[po] = linear_to_srgb8(r);
            out[po + 1] = linear_to_srgb8(g);
            out[po + 2] = linear_to_srgb8(b);
            out[po + 3] = (a * 255.0 + 0.5) as u8;
        }
    }
}

fn render_frame(cels: &[Option<&Cel>; 4], lut: &[f32; 256]) -> Vec<u8> {
    let rows: Vec<(usize, usize)> =
        (0..H).step_by(TILE).map(|y| (y, (y + TILE).min(H))).collect();
    let mut bands: Vec<(usize, Vec<u8>)> = rows
        .par_iter()
        .map(|&(y0, y1)| {
            let mut buf = vec![0u8; (y1 - y0) * W * 4];
            render_tile(cels, lut, y0, y1, &mut buf);
            (y0, buf)
        })
        .collect();
    // Merge by tile origin, never by completion order. This is the whole point of ADR-011's
    // tile contract and the reason SP-04 is byte-identical across thread counts.
    bands.sort_by_key(|&(y0, _)| y0);
    bands.into_iter().flat_map(|(_, b)| b).collect()
}

/// The same frame, rendered the way SP-04 structured it: parallel tiles produce linear
/// f32, then a single serial pass converts to sRGB8. SP-04 measured that serial pass at
/// ~43 ms per frame and the B-01 report called it the largest single cost. This exists so
/// the two structures can be compared on identical work instead of across two spikes with
/// different workloads.
fn render_frame_serial_encode(cels: &[Option<&Cel>; 4], lut: &[f32; 256]) -> Vec<u8> {
    let rows: Vec<(usize, usize)> =
        (0..H).step_by(TILE).map(|y| (y, (y + TILE).min(H))).collect();
    let mut bands: Vec<(usize, Vec<f32>)> = rows
        .par_iter()
        .map(|&(y0, y1)| {
            let mut buf = vec![0f32; (y1 - y0) * W * 4];
            for y in y0..y1 {
                for x in 0..W {
                    let o = (y * W + x) * 4;
                    let (mut r, mut g, mut b, mut a) = (0f32, 0f32, 0f32, 0f32);
                    for cel in cels.iter() {
                        let Some(cel) = cel else { continue };
                        let s = &cel.px[o..o + 4];
                        let sa = s[3] as f32 / 255.0;
                        if sa == 0.0 {
                            continue;
                        }
                        let inv = 1.0 - sa;
                        r = lut[s[0] as usize] * sa + r * inv;
                        g = lut[s[1] as usize] * sa + g * inv;
                        b = lut[s[2] as usize] * sa + b * inv;
                        a = sa + a * inv;
                    }
                    let po = (y - y0) * W * 4 + x * 4;
                    buf[po] = r;
                    buf[po + 1] = g;
                    buf[po + 2] = b;
                    buf[po + 3] = a;
                }
            }
            (y0, buf)
        })
        .collect();
    bands.sort_by_key(|&(y0, _)| y0);
    let linear: Vec<f32> = bands.into_iter().flat_map(|(_, b)| b).collect();

    let mut out = vec![0u8; W * H * 4];
    for i in 0..W * H {
        out[i * 4] = linear_to_srgb8(linear[i * 4]);
        out[i * 4 + 1] = linear_to_srgb8(linear[i * 4 + 1]);
        out[i * 4 + 2] = linear_to_srgb8(linear[i * 4 + 2]);
        out[i * 4 + 3] = (linear[i * 4 + 3] * 255.0 + 0.5) as u8;
    }
    out
}

/// FNV-1a. Only needs to detect difference between two runs of this same binary.
fn hash(b: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &x in b {
        h ^= x as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn main() {
    let root = std::env::args().nth(1).map(PathBuf::from).unwrap_or_else(|| {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Fixtures/reference_shot")
    });
    let out_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../spike-output/sp07");
    std::fs::create_dir_all(&out_dir).expect("cannot create spike-output/sp07");

    let sheet = std::fs::read_to_string(root.join("exposure_sheet.json"))
        .expect("exposure_sheet.json not found; is the reference shot present?");
    let lists = exposure_lists(&sheet);
    let lut = srgb_lut();

    let mut log = String::new();
    macro_rules! say {
        ($($a:tt)*) => {{ let s = format!($($a)*); println!("{s}"); log.push_str(&s); log.push('\n'); }};
    }

    say!("SP-07 - reference shot render");
    say!("shot: {}", root.display());
    say!("{W}x{H}, {FRAMES} frames, {TILE}x{TILE} tiles, 4 layers\n");

    // --- decode every referenced drawing once ---
    let t = Instant::now();
    let mut cels: HashMap<(usize, usize), Cel> = HashMap::new();
    let mut missing: Vec<(usize, usize)> = Vec::new();
    let mut decoded_bytes = 0usize;
    for layer in 1..=4usize {
        let mut ids: Vec<usize> = lists[&format!("layer{layer}")].clone();
        ids.sort_unstable();
        ids.dedup();
        for id in ids {
            let p = cel_path(&root, layer, id);
            match decode(&p) {
                Ok(c) => {
                    decoded_bytes += c.px.len();
                    cels.insert((layer, id), c);
                }
                Err(e) => {
                    // Doc 20: sequence gaps are not collapsed. Report the absent drawing,
                    // do not substitute a neighbour.
                    missing.push((layer, id));
                    say!("MISSING SOURCE  layer {layer} drawing {id:03}  ({e})");
                }
            }
        }
    }
    let decode_ms = t.elapsed().as_secs_f64() * 1000.0;
    say!(
        "decoded {} drawings, {:.1} MiB resident, {:.1} ms total, {:.2} ms per drawing",
        cels.len(),
        decoded_bytes as f64 / 1048576.0,
        decode_ms,
        decode_ms / cels.len().max(1) as f64
    );

    // How many composition frames reference an absent drawing? This is the number B-03's
    // gap diagnostic has to surface, so it is worth stating rather than counting by hand.
    let mut affected = 0usize;
    for f in 0..FRAMES {
        if (1..=4).any(|l| missing.contains(&(l, lists[&format!("layer{l}")][f]))) {
            affected += 1;
        }
    }
    say!(
        "{} absent drawing(s); {affected} of {FRAMES} composition frames reference one\n",
        missing.len()
    );

    // --- render 240 frames, twice, at two thread counts ---
    let render_all = |threads: usize| -> (Vec<u64>, f64) {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
        pool.install(|| {
            let t = Instant::now();
            let hashes: Vec<u64> = (0..FRAMES)
                .map(|f| {
                    let refs: [Option<&Cel>; 4] = std::array::from_fn(|i| {
                        let l = i + 1;
                        cels.get(&(l, lists[&format!("layer{l}")][f]))
                    });
                    hash(&render_frame(&refs, &lut))
                })
                .collect();
            (hashes, t.elapsed().as_secs_f64() * 1000.0)
        })
    };

    let max_threads = rayon::current_num_threads();
    say!("| Threads | Wall ms, 240 frames | ms per frame | fps | Determinism |");
    say!("|---|---|---|---|---|");
    let mut baseline: Option<Vec<u64>> = None;
    let mut single_ms = 0f64;
    for threads in [1usize, max_threads] {
        let (h1, ms1) = render_all(threads);
        let (h2, _) = render_all(threads);
        let same_run = h1 == h2;
        let same_all = baseline.as_ref().map_or(true, |b| *b == h1);
        if threads == 1 {
            single_ms = ms1;
        }
        say!(
            "| {threads} | {ms1:.1} | {:.2} | {:.2} | {} |",
            ms1 / FRAMES as f64,
            1000.0 / (ms1 / FRAMES as f64),
            if same_run && same_all { "PASS" } else { "FAIL" }
        );
        if baseline.is_none() {
            baseline = Some(h1);
        }
    }
    let (_, par_ms) = render_all(max_threads);
    say!("\nspeedup on {max_threads} threads: {:.2}x", single_ms / par_ms);

    // --- fused vs serial sRGB encode, identical work, same shot, same machine ---
    let bench = |serial: bool| -> f64 {
        let t = Instant::now();
        for f in 0..FRAMES {
            let refs: [Option<&Cel>; 4] = std::array::from_fn(|i| {
                let l = i + 1;
                cels.get(&(l, lists[&format!("layer{l}")][f]))
            });
            let px = if serial {
                render_frame_serial_encode(&refs, &lut)
            } else {
                render_frame(&refs, &lut)
            };
            std::hint::black_box(&px);
        }
        t.elapsed().as_secs_f64() * 1000.0
    };
    // Check they agree before comparing their speed; a faster wrong answer is not a result.
    let (a, b) = {
        let refs: [Option<&Cel>; 4] = std::array::from_fn(|i| {
            let l = i + 1;
            cels.get(&(l, lists[&format!("layer{l}")][0]))
        });
        (hash(&render_frame(&refs, &lut)), hash(&render_frame_serial_encode(&refs, &lut)))
    };
    let fused_ms = bench(false);
    let serial_ms = bench(true);
    say!("\n### sRGB encode: fused into the tile vs a serial pass after it\n");
    say!("byte-identical output: {}", if a == b { "yes" } else { "NO - comparison invalid" });
    say!("| Encode structure | Wall ms, 240 frames | ms per frame | fps |");
    say!("|---|---|---|---|");
    for (name, ms) in [("fused into tile (parallel)", fused_ms), ("serial pass after render", serial_ms)] {
        say!("| {name} | {ms:.1} | {:.2} | {:.2} |", ms / FRAMES as f64, FRAMES as f64 * 1000.0 / ms);
    }
    say!(
        "\nserial encode costs {:.2} ms per frame more, {:.2}x the fused wall time",
        (serial_ms - fused_ms) / FRAMES as f64,
        serial_ms / fused_ms
    );

    // --- write evidence frames the owner can look at ---
    let keep = [0usize, 60, 62, 64, 152, 165, 166, 239];
    for f in keep {
        let refs: [Option<&Cel>; 4] = std::array::from_fn(|i| {
            let l = i + 1;
            cels.get(&(l, lists[&format!("layer{l}")][f]))
        });
        let px = render_frame(&refs, &lut);
        let p = out_dir.join(format!("frame_{f:03}.png"));
        let file = std::fs::File::create(&p).unwrap();
        let mut enc = png::Encoder::new(std::io::BufWriter::new(file), W as u32, H as u32);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header().unwrap().write_image_data(&px).unwrap();
    }
    say!("\nwrote {} evidence frames to {}", keep.len(), out_dir.display());

    std::fs::File::create(out_dir.join("sp07_report.txt"))
        .unwrap()
        .write_all(log.as_bytes())
        .unwrap();
}
