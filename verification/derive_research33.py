"""Check report arithmetic and source anchors; never runs the compositor or edits fixtures."""
from pathlib import Path
import hashlib
import re
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    checks = [
        ("1080p RGBA32F bytes", 33177600, 1920 * 1080 * 4 * 4),
        ("4K RGBA32F bytes", 132710400, 3840 * 2160 * 4 * 4),
        ("8K RGBA32F MiB", 506.25, 7680 * 4320 * 16 / 2**20),
        ("240 full-HD RGBA32F frames, GiB rounded", 7.42, round(240 * 1920 * 1080 * 16 / 2**30, 2)),
        ("240 UHD RGBA32F frames, GiB rounded", 29.66, round(240 * 3840 * 2160 * 16 / 2**30, 2)),
        ("240 UHD RGBA8 frames, GiB rounded", 7.42, round(240 * 3840 * 2160 * 4 / 2**30, 2)),
        ("166 full-HD RGBA32F cels, GiB rounded", 5.13, round(166 * 1920 * 1080 * 16 / 2**30, 2)),
        ("166 draft RGBA32F cels, GiB rounded", 0.321, round(166 * 480 * 270 * 16 / 2**30, 3)),
        ("4K60 RGBA8 payload, decimal GB/s rounded", 1.991, round(3840 * 2160 * 4 * 60 / 1e9, 3)),
        ("24 fps deadline ms rounded", 41.667, round(1000 / 24, 3)),
        ("Tenth-loop median/deadline rounded", 6.34, round(264.17 / (1000 / 24), 2)),
        ("First-loop aggregate fps rounded", 4.30, round(240 / (55875.2 / 1000), 2)),
        ("First-loop reciprocal median fps rounded", 4.14, round(1000 / 241.69, 2)),
        ("Half of work accelerated tenfold, speedup rounded", 1.82, round(1 / (0.5 + 0.5 / 10), 2)),
    ]
    anchors = [
        ("src/cache.rs", "pub const DEFAULT_BUDGET_BYTES: usize = 1024 * 1024 * 1024;"),
        ("src/cache.rs", "let buffer = entry.1.clone();"),
        ("src/compose.rs", "crate::mask::apply(&mut source, mask);"),
        ("src/compose.rs", "crate::effects::apply_stack(&mut source"),
        ("src/render.rs", "pub a: f64,"),
        ("src/render.rs", ".par_iter()"),
        ("src/render.rs", "for layer in &plan.layers {"),
        ("app/src/main.rs", "let mut pixels = buffer.to_srgb8_straight();"),
        ("app/ui/index.html", "if (inFlight) return;"),
        ("verification/T-06_declared_fixture.md", "| 10 | 60922.6 | 264.17 | 319.42 |"),
        ("verification/T-06_declared_fixture.md", "| The frame just shown, again | 1 GiB"),
    ]
    report = ROOT / "Markdown/33_Hardware_Optimization_and_Future_Architecture_Research.md"
    body = report.read_text(encoding="utf-8")
    used = set(re.findall(r"\[(\d+)\]", body))
    defined = set(re.findall(r"^(\d+)\. .*https://", body, re.M))
    checks.append(("Numbered external source definitions", 23, len(defined)))
    checks.append(("Undefined numbered citations", 0, len(used - defined)))
    lines = [
        "# Research 33 evidence and verification",
        "",
        "Research-only verification, September 9, 2026. This artifact checks arithmetic, citation numbering, and selected source anchors. It is not a rendering fixture or performance benchmark.",
        "",
        "## Arithmetic and document checks",
        "",
        "Expected numbers below were written as report arithmetic, independently of the compositor. No renderer output generated an expectation. Equality applies to the stated rounded values.",
        "",
        "| Check | Expected | Actual | Result |",
        "|---|---:|---:|---|",
    ]
    failed = 0
    for name, expected, actual in checks:
        ok = expected == actual
        failed += not ok
        lines.append(f"| {name} | {expected} | {actual} | {'PASS' if ok else 'FAIL'} |")
    lines += ["", "## Source anchors", "", "These confirm cited text exists at the inspected checkout; they do not prove runtime behavior. The lock lifetime, data flow, and dependency observations were inspected separately.", "", "| File | Expected anchor | Actual location | Result |", "|---|---|---|---|"]
    for name, anchor in anchors:
        content = (ROOT / name).read_text(encoding="utf-8")
        offset = content.find(anchor)
        ok = offset >= 0
        failed += not ok
        line = content[:offset].count("\n") + 1 if ok else "missing"
        lines.append(f"| `{name}` | `{anchor.replace('|', '&#124;')}` | {line} | {'PASS' if ok else 'FAIL'} |")
    baseline = "d609310a003a13a9af5b947210388dd23230685d"
    revision = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    changed = subprocess.check_output(["git", "diff", "--name-only", baseline, "--", "src", "app", "Cargo.toml", "Cargo.lock", "rust-toolchain.toml", "verification/T-06_declared_fixture.md"], cwd=ROOT, text=True).strip()
    if changed:
        failed += 1
    lines += ["", "## Provenance", "", f"- Inspected source baseline: `{baseline}`.", f"- HEAD at verification: `{revision}`. HEAD changed during this research through other work; this research made no commits or branch changes.", f"- Relevant source/build/T-06 differences from baseline: {'NONE (PASS)' if not changed else changed + ' (REVIEW REQUIRED)'}. This additional check guards against source drift during research.", f"- Research report SHA-256: `{hashlib.sha256(report.read_bytes()).hexdigest()}`.", "- Regeneration: run `verification/derive_research33.py` with Python 3 from this repository; only this research evidence file is written.", "- Vendor manuals were downloaded to ignored `target/performance-research/`; relevant text was extracted with bundled pypdf 6.10.0. No dependency was added to the product."]
    for name in ["resolve.pdf", "fusion.pdf"]:
        path = ROOT / "target/performance-research" / name
        if path.exists():
            lines.append(f"- `{name}` source snapshot SHA-256: `{hashlib.sha256(path.read_bytes()).hexdigest()}`; {path.stat().st_size:,} bytes. Original URLs and versions are recorded in report sources 6 and 7.")
    lines += [
        "", "## What was and was not run", "",
        f"- Ran: {len(checks)} arithmetic/document checks and {len(anchors)} source-anchor checks; {failed} failed.",
        "- Read: governing rendering/cache/architecture contracts, relevant ADRs, source paths, historical verification records, document 32, and cited external sources. Resolve chapter 8 and selected Fusion sections were read, not the entire manuals.",
        "- Not run: Rust unit/integration fixtures, ignored T-06 performance tests, GPU experiments, window responsiveness benchmarks, export comparisons, AE/Resolve application benchmarks, or new screenshot/color validation. This request was research and production code was not changed.",
        "- No new measured speedup, GPU utilization, memory peak, codec throughput, or 64 GB-machine performance result is claimed. Historical timings remain labeled historical.",
        "- Hardware inventory attempt: Windows CIM/storage queries returned Access denied. The supplied CPU/GPU/64 GB specification is the planning input; storage and runtime headroom were not established. The queries were optional and no elevation was sought.",
        "- PDF text extraction initially encountered a console encoding error after successfully saving the Resolve contents; subsequent extraction to UTF-8 files succeeded. PyMuPDF was unavailable; bundled pypdf was used. These are research-tool limitations, not product failures.",
        "- No expected values in Fixtures or document 25 were edited; no production source, dependency, schema, accepted ADR, or document 32 was changed.",
        "- The report includes a Mermaid architecture diagram. No new exported rendering images or screenshots are supplied or implied.",
        "", "The report proposes future verification artifacts for implementation work. This research artifact does not close R-06, T-06, or any other implementation requirement.", "",
    ]
    output = ROOT / "verification/Research_33_evidence.md"
    output.write_text("\n".join(lines), encoding="utf-8")
    print(f"{len(checks)} arithmetic/document checks; {len(anchors)} anchors; {failed} failures")
    print(output)
    return int(failed > 0)


if __name__ == "__main__":
    raise SystemExit(main())
