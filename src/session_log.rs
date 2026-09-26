//! P-19: a log of where each preview frame's time went, kept only while the window is open.
//!
//! P-01's stopwatch answers "where does a frame go" for a harness that renders one fixture. This
//! answers it for the session the owner is actually having: every frame the viewer shows while
//! the switch is on becomes one row, held in memory and nowhere else. Nothing is written to disk
//! unless somebody asks for a copy, and closing the window empties it, which is ADR-012's rule
//! for trace mode: a diagnostic is off at every launch and on only when asked for.
//!
//! It is a diagnostic and not a benchmark. The rows say what this machine did with this project
//! while other programs were running, which is the question a bug report asks; P-01, P-15 and
//! P-18 remain the measurements a claim about speed rests on.

use std::collections::VecDeque;
use std::time::Instant;

use crate::perf::{self, Stage};
use crate::preview::PreviewQuality;

/// How many rows are kept. At 24 frames a second this is about three and a half minutes of
/// playback; older rows are let go, and counted, so the log never grows without bound.
pub const KEEPS: usize = 5000;

/// One frame the viewer showed.
#[derive(Clone, Debug)]
pub struct Row {
    /// Milliseconds from when the switch was turned on to when this frame was finished.
    /// Filled in by [`SessionLog::push`].
    pub since_on_ms: u64,
    pub frame: i32,
    pub quality: PreviewQuality,
    /// True when the frame came from the playback clock, false when it was stepped or scrubbed.
    pub playing: bool,
    /// The whole frame, as P-15's `x-ms` measures it.
    pub ms: f64,
    /// Each stage's nanoseconds on the thread that made this frame (`perf::end_capture`).
    pub stages: Vec<(Stage, u64)>,
    /// Frames the D-32 clock skipped to reach this one.
    pub dropped: u32,
    /// Whether the next frame's drawings were read ahead after this one (P-18).
    pub read_ahead: bool,
    /// Whether an export was running while this frame was made.
    pub exporting: bool,
    /// Drawings served from the cache, and read from disk, for this frame.
    pub cel_hits: u64,
    pub cel_misses: u64,
    /// Effect results served from the P-11 cache for this frame.
    pub effect_hits: u64,
    /// This frame's warnings, already limited by D-25.
    pub warnings: Vec<String>,
}

impl Row {
    /// The stage this frame spent longest in, if it spent measurable time in any.
    pub fn costliest(&self) -> Option<(Stage, u64)> {
        self.stages
            .iter()
            .copied()
            .filter(|&(_, n)| n > 0)
            .max_by_key(|&(_, n)| n)
    }
}

/// What the panel shows at the top.
#[derive(Clone, Debug, PartialEq)]
pub struct Summary {
    pub frames: usize,
    pub let_go: u64,
    pub median_ms: f64,
    /// The time the slowest 5% of frames took at least (nearest rank).
    pub slowest_5_percent_ms: f64,
    pub dropped: u64,
    /// The stage with the most time across every row kept, and that time in nanoseconds.
    pub costliest: Option<(Stage, u64)>,
}

/// The log itself. Off, and empty, until [`switch`](Self::switch) turns it on.
#[derive(Default)]
pub struct SessionLog {
    on_since: Option<Instant>,
    rows: VecDeque<Row>,
    let_go: u64,
}

impl SessionLog {
    pub fn is_on(&self) -> bool {
        self.on_since.is_some()
    }

    /// Turn the log, and with it P-01's stopwatch, on or off. Turning it off keeps the rows, so
    /// they can still be read and saved; [`clear`](Self::clear) is what empties it.
    pub fn switch(&mut self, on: bool) {
        if on && self.on_since.is_none() {
            self.on_since = Some(Instant::now());
            perf::enable();
        } else if !on && self.on_since.is_some() {
            self.on_since = None;
            perf::disable();
        }
    }

    /// Add a frame. Ignored while the switch is off; past [`KEEPS`] the oldest row is let go.
    pub fn push(&mut self, mut row: Row) {
        let Some(since) = self.on_since else { return };
        row.since_on_ms = since.elapsed().as_millis() as u64;
        if self.rows.len() == KEEPS {
            self.rows.pop_front();
            self.let_go += 1;
        }
        self.rows.push_back(row);
    }

    pub fn clear(&mut self) {
        self.rows.clear();
        self.let_go = 0;
    }

    pub fn rows(&self) -> &VecDeque<Row> {
        &self.rows
    }

    pub fn summary(&self) -> Summary {
        let mut times: Vec<f64> = self.rows.iter().map(|r| r.ms).collect();
        times.sort_by(f64::total_cmp);
        let rank = |p: f64| {
            if times.is_empty() {
                0.0
            } else {
                let at = ((p * times.len() as f64).ceil() as usize).clamp(1, times.len());
                times[at - 1]
            }
        };
        let mut totals = [0u64; Stage::ALL.len()];
        for row in &self.rows {
            for &(stage, nanos) in &row.stages {
                totals[stage as usize] += nanos;
            }
        }
        let costliest = Stage::ALL
            .iter()
            .map(|&s| (s, totals[s as usize]))
            .filter(|&(_, n)| n > 0)
            .max_by_key(|&(_, n)| n);
        Summary {
            frames: self.rows.len(),
            let_go: self.let_go,
            median_ms: rank(0.5),
            slowest_5_percent_ms: rank(0.95),
            dropped: self.rows.iter().map(|r| r.dropped as u64).sum(),
            costliest,
        }
    }

    /// The `n` slowest rows, slowest first.
    pub fn slowest(&self, n: usize) -> Vec<&Row> {
        let mut rows: Vec<&Row> = self.rows.iter().collect();
        rows.sort_by(|a, b| b.ms.total_cmp(&a.ms));
        rows.truncate(n);
        rows
    }

    /// The saved copy: `heading` (the machine and build, written by the caller, one line each),
    /// the summary, then one table row per frame kept, oldest first.
    pub fn to_markdown(&self, heading: &[String]) -> String {
        let s = self.summary();
        let mut out = String::from("# Session log\n\n");
        for line in heading {
            out.push_str(&format!("- {line}\n"));
        }
        out.push_str(&format!(
            "\nFrames logged: {}. Let go because the log was full: {}. Median {:.1} ms, \
             slowest 5% {:.1} ms or more. Frames the clock dropped: {}. Costliest stage: {}.\n\n",
            s.frames,
            s.let_go,
            s.median_ms,
            s.slowest_5_percent_ms,
            s.dropped,
            s.costliest.map_or("none".to_string(), |(st, n)| format!(
                "{} ({:.1} ms in all)",
                st.label(),
                n as f64 / 1e6
            )),
        ));
        out.push_str(
            "| # | Seconds in | Frame | Quality | Playing | ms | Dropped | Drawings hit / read \
             | Effects hit | Read ahead | Exporting | Stages (ms) | Warnings |\n",
        );
        out.push_str("|---|---|---|---|---|---|---|---|---|---|---|---|---|\n");
        for (i, r) in self.rows.iter().enumerate() {
            let stages: Vec<String> = r
                .stages
                .iter()
                .filter(|&&(_, n)| n > 0)
                .map(|&(st, n)| format!("{} {:.2}", st.label(), n as f64 / 1e6))
                .collect();
            let yes = |b: bool| if b { "yes" } else { "no" };
            out.push_str(&format!(
                "| {} | {:.3} | {} | {} | {} | {:.1} | {} | {} / {} | {} | {} | {} | {} | {} |\n",
                i + 1,
                r.since_on_ms as f64 / 1000.0,
                r.frame,
                r.quality.label(),
                yes(r.playing),
                r.ms,
                r.dropped,
                r.cel_hits,
                r.cel_misses,
                r.effect_hits,
                yes(r.read_ahead),
                yes(r.exporting),
                stages.join("; "),
                r.warnings.join(" / ").replace('|', "\\|").replace('\n', " "),
            ));
        }
        out
    }
}
