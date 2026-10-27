//! B-782 (D-902): a PNG's own colour chunks are named at import, and the picture is unchanged.
//!
//! Expected values are `Fixtures/colour/metadata/expected_colour_metadata.json`, written with
//! the files by `tools/colour_metadata_reference.py`. Writes
//! `verification/B-782_colour_metadata_table.md`.

use anime_compositor::diagnostics::DiagnosticId;
use anime_compositor::media;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

#[test]
fn b782_png_colour_chunks_are_named_and_the_picture_is_unchanged() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("Fixtures/colour/metadata");
    let expected: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(root.join("expected_colour_metadata.json")).unwrap(),
    )
    .unwrap();
    let pixels: Vec<u8> = expected["pixels_rgba"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u8)
        .collect();
    let plain = media::decode(&root.join("plain.png")).unwrap();
    let mut rows: Vec<(String, String, bool)> = Vec::new();

    // FX-COLMETA-001 and 002: each file chosen on its own.
    for (name, want) in expected["files"].as_object().unwrap() {
        let path = root.join(name);
        let chunks: Vec<&str> = want["chunks"]
            .as_array()
            .unwrap()
            .iter()
            .map(|c| c.as_str().unwrap())
            .collect();
        let said = media::colour_metadata(&path);
        let (expect, got, ok) = match (&said, want["warned"].as_bool().unwrap()) {
            (None, false) => ("no warning".into(), "no warning".into(), true),
            (Some(d), true) => {
                let found = format!("Found: {}.", chunks.join("; "));
                (
                    format!("MEDIA_COLOR_METADATA_IGNORED, {found}"),
                    format!("{} {} {}", d.id.as_str(), d.message, d.detail),
                    d.id == DiagnosticId::MediaColorMetadataIgnored
                        && d.message.starts_with(name.as_str())
                        && d.detail.starts_with(&found),
                )
            }
            (d, _) => (
                format!("warned: {}", want["warned"]),
                format!("{d:?}"),
                false,
            ),
        };
        rows.push((format!("{name}: {expect}"), got, ok));

        let bytes = media::decode_8bit(&path).unwrap().unwrap().2;
        let picture = media::decode(&path).unwrap();
        rows.push((
            format!("{name}: decodes to the 16 stored bytes, the working picture plain.png's"),
            if bytes == pixels && picture.data() == plain.data() {
                "byte for byte".into()
            } else {
                format!("{bytes:?}")
            },
            bytes == pixels && picture.data() == plain.data(),
        ));
    }

    // FX-COLMETA-003: a sequence says it once, naming its drawings.
    let seq = &expected["sequence"];
    let files: Vec<PathBuf> = seq["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| root.join(f.as_str().unwrap()))
        .collect();
    let imported = media::import_sequence(&files);
    let warned: Vec<_> = imported
        .diagnostics
        .iter()
        .filter(|d| d.id == DiagnosticId::MediaColorMetadataIgnored)
        .collect();
    let chunk = seq["chunks"][0].as_str().unwrap();
    let drawings = seq["drawings"].as_str().unwrap();
    rows.push((
        format!("seq/: one warning, \"{chunk}\", drawings {drawings}"),
        warned
            .iter()
            .map(|d| format!("{} {}", d.message, d.detail))
            .collect::<Vec<_>>()
            .join(" | "),
        warned.len() == seq["warnings"].as_u64().unwrap() as usize
            && warned[0]
                .message
                .starts_with("3 drawings in frame_%04d.png carry ")
            && warned[0].detail.contains(&format!("Found: {chunk}."))
            && warned[0].detail.contains(&format!("Drawings: {drawings}.")),
    ));
    let fixture_seq = media::import_sequence(&[
        root.join("seq/frame_0001.png"),
        root.join("seq/frame_0002.png"),
    ]);
    rows.push((
        "two drawings of the sequence: still one warning, not two".into(),
        format!("warnings: {}", fixture_seq.diagnostics.len()),
        fixture_seq
            .diagnostics
            .iter()
            .filter(|d| d.id == DiagnosticId::MediaColorMetadataIgnored)
            .count()
            == 1,
    ));

    let passed = rows.iter().filter(|r| r.2).count();
    let mut md = String::from(
        "# B-782: a PNG's colour chunks named at import (D-902)\n\n\
         Written by `tests/b782_colour_metadata.rs` from \
         `Fixtures/colour/metadata/expected_colour_metadata.json`. \
         MEDIA_COLOR_METADATA_IGNORED is said for a chunk that is not sRGB; every picture is \
         unchanged.\n\n| Check | Result | Pass |\n|---|---|---|\n",
    );
    for (what, got, ok) in &rows {
        let _ = writeln!(
            md,
            "| {} | {} | {} |",
            what.replace('|', "/"),
            got.replace('|', "/").replace('\n', " "),
            if *ok { "yes" } else { "NO" }
        );
    }
    let _ = writeln!(md, "\n{passed} of {} pass.", rows.len());
    std::fs::write(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("verification/B-782_colour_metadata_table.md"),
        md,
    )
    .unwrap();
    assert_eq!(
        passed,
        rows.len(),
        "see verification/B-782_colour_metadata_table.md"
    );
}
