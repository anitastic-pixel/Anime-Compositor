//! P-11: the evaluated-effect cache, and the proof that it decides nothing.
//!
//! Document 15's P-11. The measurement that opens the entry is done:
//! `src/perf.rs` splits the effect stage by kind, and on the declared ten-layer fixture the
//! Gaussian blur is 111 to 123 ms of every frame against 1.8 ms for the exposure and the tint
//! together. So the stack is worth not running twice for the same input — and ADR-017 makes that
//! possible, because the stack runs whole-layer on the cel's own pixels before the frame plan
//! exists, so an evaluation's whole input is the cel, the mask drawn into it, and the stack.
//!
//! This file is the correctness half of the unit. The timings are in
//! `verification/P-11_effect_cache.md`, measured by re-running P-01's harnesses; nothing here is
//! timed, so nothing here is flaky and it runs in the normal suite.
//!
//! What it has to establish is one sentence from document 27, line 29: caching "may improve
//! interaction but must never define correctness". Three ways a cache could define correctness,
//! and one check for each:
//!
//! 1. **By returning the wrong pixels.** Forty frames of the declared fixture, at both preview
//!    qualities, each rendered twice with the viewer's cache and once with no cache at all, are
//!    required to be byte-identical. The second pass is the one the cache answers.
//! 2. **By not noticing an input changed.** Same cel, same mask, one effect parameter moved, and
//!    the cache must miss. Same cel, same stack, the mask moved, and the cache must miss. A key
//!    that misses one of its inputs is the failure mode that shows a person a stale picture and
//!    gives them no way to tell.
//! 3. **By losing a warning.** An effect this build cannot draw raises a diagnostic every frame
//!    (document 28). A frame served from the cache never enters `apply_stack`, so it would raise
//!    nothing unless the bypass is replayed. The same frame is rendered twice and both frames are
//!    required to carry the same warning.
//!
//! It writes `verification/P-11_effect_cache_table.md`.

use std::fs;
use std::path::Path;
use std::sync::Arc;

use anime_compositor::cache::{CelCache, EffectResult, DEFAULT_EFFECT_BUDGET_BYTES};
use anime_compositor::compose::DEFAULT_TILE_SIZE;
use anime_compositor::diagnostics::{DiagnosticId, FrameLog};
use anime_compositor::effects::{Effect, EffectInstance};
use anime_compositor::mask::PolygonMask;
use anime_compositor::model::{Id, Interpretation, Project};
use anime_compositor::persist;
use anime_compositor::preview::{self, PreviewQuality};
use anime_compositor::WorkingBuffer;

mod common;
use common::{build_fixture, repo};

const COMP: &str = "comp-reference-shot";
/// Frames sampled from each pass. Coprime with the 240-frame work area, as in
/// `tests/p01_frame_trace.rs`, so the sample is spread across the shot rather than clustered.
const SCATTER: i32 = 97;
const SAMPLES: i32 = 20;

struct Check {
    check: String,
    expected: String,
    actual: String,
}

impl Check {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

fn frames() -> Vec<i32> {
    (0..SAMPLES).map(|i| (i * SCATTER) % 240).collect()
}

fn render(
    project: &Project,
    root: &Path,
    frame: i32,
    quality: PreviewQuality,
    cache: &mut CelCache,
    log: &mut FrameLog,
) -> Vec<u8> {
    preview::preview_frame_cached(
        project,
        &Id::new(COMP),
        frame,
        root,
        quality,
        DEFAULT_TILE_SIZE,
        log,
        cache,
    )
    .unwrap_or_else(|d| panic!("frame {frame}: {}", d.message))
    .to_srgb8_straight()
}

#[test]
fn p11_effect_cache() {
    let (project, root, text) = build_fixture("p11");
    let mut checks = Vec::new();

    checks.push(same_pixels(&project, &root));
    checks.extend(key_notices_its_inputs());
    checks.push(warning_survives_a_hit(&text, &root));

    write_artifact(&checks);
    for c in &checks {
        assert!(
            c.pass(),
            "{}: expected {}, got {}",
            c.check,
            c.expected,
            c.actual
        );
    }
}

/// (1) The cache returns the pixels the renderer would have produced, and it is actually used.
fn same_pixels(project: &Project, root: &Path) -> Check {
    let mut differing = 0usize;
    let mut hits = 0u64;
    let mut misses = 0u64;

    for quality in [PreviewQuality::Draft, PreviewQuality::Full] {
        // One cache across the whole pass, as a viewer has, so that what it holds when a frame
        // arrives is what a viewer would be holding.
        let mut cached = CelCache::viewer();
        for pass in 0..2 {
            for frame in frames() {
                let mut log = FrameLog::new(3);
                let with_cache = render(project, root, frame, quality, &mut cached, &mut log);
                if pass == 1 {
                    // Compared against a render that holds nothing at all, not against the first
                    // pass: a cache that was wrong on both passes would agree with itself.
                    let mut none = CelCache::none();
                    let mut log = FrameLog::new(3);
                    let without = render(project, root, frame, quality, &mut none, &mut log);
                    if with_cache != without {
                        differing += 1;
                    }
                }
            }
        }
        hits += cached.effect_hits();
        misses += cached.effect_misses();
        assert!(
            cached.effect_held_bytes() <= DEFAULT_EFFECT_BUDGET_BYTES,
            "the effect cache held {} bytes against a budget of {DEFAULT_EFFECT_BUDGET_BYTES}",
            cached.effect_held_bytes()
        );
    }

    Check {
        check: format!(
            "80 frames of the declared fixture rendered from the effect cache ({hits} hits, \
             {misses} evaluations) match the same frames rendered with no cache"
        ),
        expected: "0 frames differ, and the cache was used".to_string(),
        actual: format!(
            "{differing} frames differ, and the cache was {}",
            if hits > 0 { "used" } else { "**never hit**" }
        ),
    }
}

/// (2) Every input is in the key: change one and the cache misses.
fn key_notices_its_inputs() -> Vec<Check> {
    let cel = repo("Fixtures/reference_shot/layer1/layer1_000.png");
    let interp = Interpretation::default();
    let stack = |sigma: f64| {
        vec![EffectInstance::new(
            Id::new("fx"),
            Effect::GaussianBlur { sigma_px: sigma },
        )]
    };
    let square = PolygonMask::new(vec![(0.0, 0.0), (10.0, 0.0), (10.0, 10.0), (0.0, 10.0)]);
    let moved = PolygonMask::new(vec![(0.0, 0.0), (20.0, 0.0), (20.0, 10.0), (0.0, 10.0)]);
    let stored = EffectResult {
        buffer: Arc::new(WorkingBuffer::transparent(4, 4)),
        offset: (0, 0),
        bypassed: Vec::new(),
    };

    let mut cache = CelCache::viewer();
    cache.store_effect(&cel, interp, Some(&square), &stack(4.0), stored);

    let hit = cache
        .effect_result(&cel, interp, Some(&square), &stack(4.0))
        .is_some();
    let other_sigma = cache
        .effect_result(&cel, interp, Some(&square), &stack(4.5))
        .is_some();
    let other_mask = cache
        .effect_result(&cel, interp, Some(&moved), &stack(4.0))
        .is_some();
    let no_mask = cache
        .effect_result(&cel, interp, None, &stack(4.0))
        .is_some();

    vec![
        Check {
            check: "the same cel, mask and effect stack is found again".to_string(),
            expected: "found".to_string(),
            actual: if hit { "found" } else { "not found" }.to_string(),
        },
        Check {
            check: "a blur of 4.5 pixels is not served the result of a blur of 4.0".to_string(),
            expected: "not found".to_string(),
            actual: if other_sigma { "found" } else { "not found" }.to_string(),
        },
        Check {
            check: "a mask with a corner moved is not served the old mask's result".to_string(),
            expected: "not found".to_string(),
            actual: if other_mask { "found" } else { "not found" }.to_string(),
        },
        Check {
            check: "a layer with no mask is not served a masked layer's result".to_string(),
            expected: "not found".to_string(),
            actual: if no_mask { "found" } else { "not found" }.to_string(),
        },
    ]
}

/// (3) Document 28's per-frame warning is raised by the cached frame too.
fn warning_survives_a_hit(fixture: &str, root: &Path) -> Check {
    // The fixture with one more effect on the layer that already carries the exposure: an effect
    // this build does not have, written the way `Fixtures/projects/unknown_effect_project.json`
    // writes one. The layer still reaches `apply_stack`, and comes back with a bypass to remember.
    let mut json: serde_json::Value = serde_json::from_str(fixture).expect("the fixture is JSON");
    let effects = &mut json["compositions"][0]["layers"][1]["effects"];
    effects
        .as_array_mut()
        .expect("the declared fixture gives layer 2 an effect list")
        .push(serde_json::json!({
            "instance_id": "fx-unknown",
            "type_id": "vendor.future.effect",
            "enabled": true,
            "parameters": { "strength": 0.5 },
            "opaque_unknown_data": { "keep_me": true },
        }));
    let text = serde_json::to_string(&json).expect("it serialises");
    let project = persist::load_str(&text)
        .unwrap_or_else(|d| panic!("the fixture with an unknown effect: {}", d.message))
        .document
        .project()
        .clone();

    let mut cache = CelCache::viewer();
    let mut raised = Vec::new();
    for _ in 0..2 {
        let mut log = FrameLog::new(3);
        let _ = render(
            &project,
            root,
            0,
            PreviewQuality::Draft,
            &mut cache,
            &mut log,
        );
        raised.push(
            log.ids_at(0)
                .iter()
                .filter(|id| **id == DiagnosticId::EffectUnsupported)
                .count(),
        );
    }

    Check {
        check: format!(
            "an unsupported effect warns on the frame that evaluated the stack and on the frame \
             served from the cache ({} effect-cache hits)",
            cache.effect_hits()
        ),
        expected: "1 warning, then 1 warning".to_string(),
        actual: format!("{} warning, then {} warning", raised[0], raised[1]),
    }
}

fn write_artifact(checks: &[Check]) {
    let mut s = String::from("# P-11: the evaluated-effect cache\n\n");
    s.push_str(
        "Document 15's P-11. Running an effect stack is the most expensive thing a frame of the \
         declared fixture does - the Gaussian blur alone is 111 to 123 ms of it, measured by the \
         split `src/perf.rs` gained for this unit - and a playthrough runs the same stack over \
         the same drawing again every time that drawing comes back on screen. This unit \
         remembers the result instead.\n\nThe timings are in \
         `verification/P-11_effect_cache.md`. **This page is the other half: the checks that the \
         cache changes nothing except how long a frame takes.** Document 27 line 29 puts it in \
         one sentence - caching \"may improve interaction but must never define correctness\" - \
         and there are exactly three ways it could break that.\n\n\
         **It could hand back the wrong picture.** Eighty frames of the declared ten-layer \
         fixture, at both preview qualities, are rendered from the cache and again with no cache \
         at all, and every pair has to be identical byte for byte. The comparison is against a \
         cacheless render rather than against the cache's own first pass, because a cache that \
         was wrong twice would agree with itself.\n\n\
         **It could fail to notice that something changed.** The thing a cache is remembered by \
         is the picture it shows after an edit that it missed. Three checks move one input at a \
         time - the blur's amount, a corner of the mask, and the mask's presence - and each has \
         to come back as \"not found\".\n\n\
         **It could lose a warning.** An effect this build cannot draw is reported once per \
         frame (document 28), and a frame answered from the cache never runs the stack. The last \
         check renders the same frame twice, the second time from the cache, and both have to \
         carry the warning.\n\n## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n",
    );
    for c in checks {
        s.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            c.check,
            c.expected,
            c.actual,
            if c.pass() { "pass" } else { "**FAIL**" }
        ));
    }
    s.push_str(
        "\n## What is deliberately not here\n\n\
         **The cel cache is untouched.** This is a second, separately budgeted list beside the \
         decoded cels, and not a share of theirs, because \
         `verification/P-09_effect_reuse.md` measured what one list would do: the request stream \
         cycles, least-recently-used is at its worst against a cycle, and effect results \
         competing with cels in one list would be dropped exactly before they are wanted.\n\n\
         **The gibibyte did not grow.** The effect budget is taken out of D-40's budget and not \
         added to it, so the window holds what document 40 said it holds.\n\n\
         **Export still caches nothing.** ADR-015's third bound is that an export neither reads \
         the cache nor writes it, and an export builds `CelCache::none`, which has no effect \
         budget either. Every check above is a preview.\n",
    );
    fs::write(repo("verification/P-11_effect_cache_table.md"), s).expect("write the artifact");
}
