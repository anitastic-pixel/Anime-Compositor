//! P-20 (P-27's audit): what one edit costs outside the picture, as the project grows.
//!
//! Ignored, because it is a timer: `cargo test --release --test p20_edit_cost -- --ignored
//! --nocapture --test-threads=1` prints one table. Nothing is asserted about time (document 12).
//!
//! The p-series before this measured the picture. None of them sees the work every edit does
//! around it, which grows with the size of the project rather than the size of the frame:
//! - `Document::apply` copies the whole project twice and keeps one copy as the undo step
//!   (src/command.rs, `apply_all`); `update_drag` copies it once per move of the hand;
//! - the window's `/state` (app/src/main.rs, `state`) writes the whole project as JSON, reads
//!   that back into a JSON tree and writes it out again, after every edit;
//! - every frame answer compares the whole project with the last save (`is_dirty`), and the
//!   frame memory finds a frame by comparing whole projects (src/cache.rs, `FrameCache`), after
//!   copying the project the frame is for (app/src/main.rs, `shown`);
//! - opening reads the whole file (`persist::load_str`).
//!
//! The projects are the reference shot (4 layers) and the same shot with layer 2 copied to 50
//! and 200 layers, each copy with 25 position keys, so the largest has 5,000 keys: the size the
//! audit brief names. A counting allocator measures the memory one undo step keeps.
//!
//! The last row is a fact, not a time: frames the frame memory holds for the shot, and how many
//! it still holds after a layer is renamed, which changes no pixel.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;
use std::time::Instant;

use anime_compositor::cache::{Finished, FrameCache, Sight};
use anime_compositor::command::{Command, Document, Target};
use anime_compositor::model::{Id, Prop, Value};
use anime_compositor::persist;
use anime_compositor::preview::PreviewQuality;
use serde_json::{json, Value as J};

struct Counting;
static HELD: AtomicIsize = AtomicIsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        HELD.fetch_add(l.size() as isize, Ordering::Relaxed);
        System.alloc(l)
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        HELD.fetch_sub(l.size() as isize, Ordering::Relaxed);
        System.dealloc(p, l)
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        HELD.fetch_add(n as isize - l.size() as isize, Ordering::Relaxed);
        System.realloc(p, l, n)
    }
}

#[global_allocator]
static A: Counting = Counting;

const COMP: &str = "comp-reference-shot";

/// The reference shot with `layers` layers: the original four when `layers` is 4, otherwise
/// copies of layer 2, each with 25 position keys.
fn project_text(layers: usize) -> String {
    let repo = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let text = std::fs::read_to_string(repo.join("verification/B-08a_project.json")).unwrap();
    if layers == 4 {
        return text;
    }
    let mut p: J = serde_json::from_str(&text).unwrap();
    let comp = &mut p["compositions"][0];
    let model = comp["layers"][1].clone();
    let mut list = Vec::new();
    let mut order = Vec::new();
    for n in 0..layers {
        let mut l = model.clone();
        let id = format!("layer-p20-{n}");
        l["id"] = json!(id);
        l["name"] = json!(format!("copy {n}"));
        l["transform"]["position"]["keyframes"] = J::Array(
            (0..25)
                .map(|k| json!({"frame": k * 9, "value": [k as f64 * 3.0, n as f64], "interp": "linear"}))
                .collect(),
        );
        order.push(json!(id));
        list.push(l);
    }
    comp["layers"] = J::Array(list);
    comp["layer_order"] = J::Array(order);
    serde_json::to_string_pretty(&p).unwrap()
}

/// Median and fastest of `f`, at least 5 runs and at most about 3 seconds.
fn time(mut f: impl FnMut()) -> (f64, f64, usize) {
    let mut t = Vec::new();
    let start = Instant::now();
    while t.len() < 5 || (t.len() < 51 && start.elapsed().as_secs_f64() < 3.0) {
        let s = Instant::now();
        f();
        t.push(s.elapsed().as_secs_f64() * 1000.0);
    }
    t.sort_by(|a, b| a.partial_cmp(b).unwrap());
    (t[t.len() / 2], t[0], t.len())
}

fn row(what: &str, size: usize, (median, fastest, runs): (f64, f64, usize)) {
    println!("| {what} | {size} | {median:.3} | {fastest:.3} | {runs} |");
}

fn rotate(first: &Id, degrees: f64) -> Command {
    Command::SetPropertyBase {
        composition: Id::new(COMP),
        target: Target::Layer(first.clone()),
        prop: Prop::Rotation,
        value: Value::Scalar(degrees),
    }
}

#[test]
#[ignore]
fn p20_edit_cost() {
    println!("| Step | Layers | Median ms | Fastest ms | Runs |");
    println!("|---|---|---|---|---|");
    let mut facts = Vec::new();
    for layers in [4usize, 50, 200] {
        let text = project_text(layers);
        let loaded = persist::load_str(&text).expect("the project opens");
        let project = loaded.document.project().clone();
        let preserved = loaded.preserved;
        assert_eq!(project.compositions[0].layers_in_order().count(), layers);
        let first = project.compositions[0].layers_in_order().next().unwrap().id.clone();
        let saved_bytes = persist::to_json(&project, &preserved).len();
        facts.push(format!("{layers} layers: project file {saved_bytes} bytes"));

        row("open: persist::load_str", layers, time(|| {
            std::hint::black_box(persist::load_str(&text).unwrap());
        }));
        row("save text: persist::to_json", layers, time(|| {
            std::hint::black_box(persist::to_json(&project, &preserved));
        }));
        // app/src/main.rs `state`, step for step.
        row("/state body: to_json, read back, written again", layers, time(|| {
            let tree: J = serde_json::from_str(&persist::to_json(&project, &preserved)).unwrap();
            std::hint::black_box(json!({"project": tree}).to_string());
        }));
        row("copy of the project (Project::clone)", layers, time(|| {
            std::hint::black_box(project.clone());
        }));
        let copy = project.clone();
        row("whole-project compare, equal (is_dirty, FrameCache)", layers, time(|| {
            std::hint::black_box(copy == project);
        }));

        let mut doc = Document::new(project.clone());
        let mut degrees = 0.0;
        row("one edit: Document::apply (rotation)", layers, time(|| {
            degrees += 1.0;
            let c = rotate(&first, degrees);
            doc.apply(c).unwrap();
        }));
        doc.begin_drag().unwrap();
        row("one drag move: Document::update_drag", layers, time(|| {
            degrees += 1.0;
            let c = rotate(&first, degrees);
            doc.update_drag(c).unwrap();
        }));
        doc.end_drag();

        // Memory one undo step keeps: 20 edits on a fresh document, counted by the allocator.
        let mut doc = Document::new(project.clone());
        let before = HELD.load(Ordering::Relaxed);
        for n in 0..20 {
            let c = rotate(&first, n as f64 + 0.5);
            doc.apply(c).unwrap();
        }
        let per_step = (HELD.load(Ordering::Relaxed) - before) / 20;
        facts.push(format!(
            "{layers} layers: memory kept per undo step {per_step} bytes ({:.2} MB); undo depth {} after 20 edits, no limit in Document",
            per_step as f64 / 1e6,
            doc.undo_depth()
        ));

        // The frame memory: 240 frames held for the shot as it is, then a layer renamed.
        let sight = |p: &anime_compositor::model::Project| Sight {
            project: p.clone(),
            composition: Id::new(COMP),
            root: std::path::PathBuf::from("."),
            quality: PreviewQuality::Draft,
            card: None,
        };
        let mut frames = FrameCache::with_budget(usize::MAX);
        let shot = sight(&project);
        for f in 0..240 {
            let finished = Finished {
                pixels: Arc::new(vec![0u8; 16]),
                width: 2,
                height: 2,
                fell_back: false,
                warnings: Vec::new(),
                on_processor: String::new(),
            };
            frames.store(shot.clone(), f, Vec::new(), finished);
        }
        let mut renamed = Document::new(project.clone());
        renamed
            .apply(Command::RenameLayer {
                composition: Id::new(COMP),
                layer_id: first.clone(),
                name: "renamed".into(),
            })
            .unwrap();
        let after = sight(renamed.project());
        row("frame memory lookup, one sight held (FrameCache::get)", layers, time(|| {
            std::hint::black_box(frames.get(&shot, 100));
        }));
        facts.push(format!(
            "{layers} layers: frame memory holds {} frames for the shot; after renaming a layer (no pixel changes) it offers {}",
            frames.kept(&shot).len(),
            frames.kept(&after).len()
        ));
    }
    println!();
    for f in facts {
        println!("- {f}");
    }
}

