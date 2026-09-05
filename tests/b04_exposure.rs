//! T-02 / R-02 / B-04: rational time and explicit exposure spans.
//!
//! Writes `verification/B-04_exposure_table.md`. Document 15 asks B-04 for "the complete 240-row
//! frame-to-drawing table, which the owner checks against the shot as drawn", so the artifact
//! leads with that table and the pass/fail checks follow it.
//!
//! The expected drawing numbers come from `Fixtures/reference_shot/exposure_sheet.json`, which
//! the fixture README calls "the authority on timing". The test builds exposure spans and expands
//! them; the sheet says what the expansion must equal. Nothing here derives an expected value by
//! running the evaluator.

use std::fs;
use std::path::{Path, PathBuf};

use anime_compositor::diagnostics::DiagnosticId;
use anime_compositor::media::{import_sequence, SequenceAsset};
use anime_compositor::time::{
    resolve, Composition, ExposureMap, ExposureSpan, FrameRate, LayerTiming, SourceAt, TimeError,
};

const FRAMES: usize = 240;

struct Row {
    check: String,
    expected: String,
    actual: String,
}

impl Row {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Default)]
struct Report {
    rows: Vec<Row>,
    notes: Vec<String>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push(Row {
            check: check.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }
}

fn repo(rel: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn layer_asset(name: &str) -> SequenceAsset {
    let dir = repo("Fixtures/reference_shot").join(name);
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap_or_else(|e| panic!("read {}: {e}", dir.display()))
        .map(|e| e.unwrap().path())
        .collect();
    files.sort();
    import_sequence(&files)
        .asset
        .unwrap_or_else(|| panic!("no asset for {name}"))
}

/// The exposure sheet, split into the four per-frame drawing arrays it calls authoritative.
struct Sheet {
    per_frame: Vec<(String, Vec<i64>)>,
    layer4_lengths: Vec<i64>,
    layer4_drawings: Vec<i64>,
}

fn load_sheet() -> Sheet {
    let text = fs::read_to_string(repo("Fixtures/reference_shot/exposure_sheet.json"))
        .expect("read exposure sheet");
    let sheet: serde_json::Value = serde_json::from_str(&text).expect("the exposure sheet is JSON");
    let array = |parent: &serde_json::Value, key: &str| -> Vec<i64> {
        parent
            .get(key)
            .unwrap_or_else(|| panic!("exposure sheet has no key {key}"))
            .as_array()
            .unwrap_or_else(|| panic!("{key} is not an array"))
            .iter()
            .map(|n| {
                n.as_i64()
                    .unwrap_or_else(|| panic!("{key} holds a non-integer"))
            })
            .collect()
    };
    let per_frame = sheet
        .get("frame_to_drawing")
        .expect("exposure sheet has frame_to_drawing");
    Sheet {
        per_frame: ["layer1", "layer2", "layer3", "layer4"]
            .into_iter()
            .map(|l| (l.to_string(), array(per_frame, l)))
            .collect(),
        layer4_lengths: array(&sheet, "layer4_exposure_lengths"),
        layer4_drawings: array(&sheet, "layer4_exposure_drawing_ids"),
    }
}

/// The reference shot's cadences, written from the fixture README rather than read from the
/// per-frame arrays the test then checks against.
///
/// README: layer 1 static across all 240 frames; layer 2 on 1s with 24 drawings; layer 3 on 2s
/// with 12; layer 4 on 3s and irregular, which is the only one whose structure has to come from
/// the sheet because no rule generates it.
fn build_maps(sheet: &Sheet) -> Vec<(String, ExposureMap)> {
    let cycle = |count: u32, hold: u32| -> ExposureMap {
        let per_cycle = count * hold;
        let repeats = FRAMES as u32 / per_cycle;
        let drawings: Vec<u32> = (0..repeats).flat_map(|_| 0..count).collect();
        ExposureMap::on_twos_style(&drawings, hold).expect("valid cyclic map")
    };

    let layer4: Vec<(u32, u32)> = sheet
        .layer4_drawings
        .iter()
        .zip(&sheet.layer4_lengths)
        .map(|(&d, &l)| (d as u32, l as u32))
        .collect();

    vec![
        (
            "layer1".to_string(),
            ExposureMap::new(vec![ExposureSpan {
                start_frame: 0,
                end_frame_exclusive: FRAMES as i32,
                drawing_number: 0,
            }])
            .expect("static map"),
        ),
        ("layer2".to_string(), cycle(24, 1)),
        ("layer3".to_string(), cycle(12, 2)),
        (
            "layer4".to_string(),
            ExposureMap::from_lengths(&layer4).expect("layer 4 map"),
        ),
    ]
}

#[test]
fn b04_exposure_table() {
    let mut report = Report::default();
    let sheet = load_sheet();
    let maps = build_maps(&sheet);
    let rate = FrameRate::new(24, 1).unwrap();
    let comp = Composition::from_inclusive_ui_range(0, 239, rate).expect("240 frames");
    let timing = LayerTiming {
        in_frame: 0,
        out_frame: FRAMES as i32,
        source_offset_frames: 0,
    };

    report.check("composition: frame count", FRAMES, comp.frames().count());
    report.check("composition: first frame", 0, comp.start_frame);
    report.check("composition: last frame", 239, comp.last_frame());
    report.check(
        "composition: frame 240 is outside",
        false,
        comp.contains(240),
    );

    // -- The 240-row table --------------------------------------------------------------------
    //
    // Every layer, every frame, against the sheet. This is the artifact.
    let asset3 = layer_asset("layer3");
    let mut table: Vec<Vec<String>> = Vec::with_capacity(FRAMES);
    let mut missing_frames: Vec<i32> = Vec::new();
    for frame in comp.frames() {
        let mut row = vec![frame.to_string(), {
            let (n, d) = rate.seconds_at(frame);
            format!("{n}/{d}")
        }];
        for (_, map) in &maps {
            row.push(match map.drawing_at(timing.local_frame(frame).unwrap()) {
                Some(d) => d.to_string(),
                None => "-".to_string(),
            });
        }
        // Layer 3 is the one that exposes a drawing that is not on disk.
        if let Err(d) = resolve(&timing, &maps[2].1, &asset3, frame) {
            assert_eq!(d.id, DiagnosticId::MediaSequenceGap);
            missing_frames.push(frame);
            row.push("layer3 drawing 7 missing".to_string());
        } else {
            row.push(String::new());
        }
        table.push(row);
    }

    for (layer, expected) in &sheet.per_frame {
        let map = &maps.iter().find(|(l, _)| l == layer).unwrap().1;
        // Without this the comparison below would pass vacuously on a mis-parsed sheet.
        report.check(
            &format!("{layer}: the exposure sheet supplies 240 expected drawing numbers"),
            FRAMES,
            expected.len(),
        );
        let actual: Vec<i64> = comp
            .frames()
            .map(|f| map.drawing_at(f).map(|d| d as i64).unwrap_or(-1))
            .collect();
        let first_bad = expected
            .iter()
            .zip(&actual)
            .position(|(e, a)| e != a)
            .map(|i| format!("frame {i}: sheet {} evaluator {}", expected[i], actual[i]));
        report.check(
            &format!("{layer}: all 240 drawing numbers match the exposure sheet"),
            "match",
            first_bad.unwrap_or_else(|| "match".to_string()),
        );
    }

    // -- The three structures that break naive implementations ---------------------------------
    let layer4 = &maps[3].1;
    report.check("layer4: exposure count", 80, layer4.spans().len());
    report.check(
        "layer4: five-frame hold covers frames 60-64",
        "60-64 -> drawing 0",
        {
            let s = layer4.spans().iter().find(|s| s.len() == 5).unwrap();
            format!(
                "{}-{} -> drawing {}",
                s.start_frame,
                s.end_frame_exclusive - 1,
                s.drawing_number
            )
        },
    );
    report.check(
        "layer4: one-frame accent at frame 152",
        "152-152 -> drawing 10",
        {
            let s = layer4.spans().iter().find(|s| s.len() == 1).unwrap();
            format!(
                "{}-{} -> drawing {}",
                s.start_frame,
                s.end_frame_exclusive - 1,
                s.drawing_number
            )
        },
    );
    report.check(
        "layer4: drawing numbers decrease at the re-exposure, and are accepted",
        "12,13,14,11,16,17",
        (52..58)
            .map(|i| layer4.spans()[i].drawing_number.to_string())
            .collect::<Vec<_>>()
            .join(","),
    );
    report.check(
        "layer4: frames 165-167 return the re-exposed drawing 11, not 15",
        "11,11,11",
        (165..168)
            .map(|f| layer4.drawing_at(f).unwrap().to_string())
            .collect::<Vec<_>>()
            .join(","),
    );

    // -- The missing drawing, reached through time rather than through import -------------------
    report.check(
        "layer3: composition frames whose exposed drawing is missing",
        "14,15,38,39,62,63,86,87,110,111,134,135,158,159,182,183,206,207,230,231",
        missing_frames
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(","),
    );
    report.check(
        "layer3: a present drawing resolves to its file",
        "layer3_006.png",
        match resolve(&timing, &maps[2].1, &asset3, 12) {
            Ok(SourceAt::Drawing { path, .. }) => {
                path.file_name().unwrap().to_string_lossy().into_owned()
            }
            other => format!("{other:?}"),
        },
    );

    // -- Fixture catalog, document 25 -----------------------------------------------------------
    let fx1 = ExposureMap::new(vec![
        ExposureSpan {
            start_frame: 0,
            end_frame_exclusive: 2,
            drawing_number: 1,
        },
        ExposureSpan {
            start_frame: 2,
            end_frame_exclusive: 5,
            drawing_number: 2,
        },
    ])
    .unwrap();
    report.check(
        "FX-TIME-001: frames 0..4 map to drawings 1,1,2,2,2",
        "1,1,2,2,2",
        (0..5)
            .map(|f| fx1.drawing_at(f).unwrap().to_string())
            .collect::<Vec<_>>()
            .join(","),
    );

    let ntsc = FrameRate::new(24000, 1001).unwrap();
    report.check(
        "FX-TIME-003: 24000/1001 is stored unreduced",
        "24000/1001",
        ntsc.to_string(),
    );
    report.check(
        "FX-TIME-003: an equivalent rate reduces to the same pair, not to a decimal",
        "24000/1001",
        FrameRate::new(48000, 2002).unwrap().to_string(),
    );
    report.check(
        "FX-TIME-003: frame 1 is exactly 1001/24000 seconds",
        "1001/24000",
        {
            let (n, d) = ntsc.seconds_at(1);
            format!("{n}/{d}")
        },
    );
    report.check(
        "FX-TIME-003: the decimal label is display only",
        "23.976",
        ntsc.label(),
    );
    report.check(
        "FX-TIME-003: 1001/24000 seconds converts back to frame 1",
        1,
        ntsc.frame_at_seconds(1001, 24000).unwrap(),
    );

    let negative = Composition {
        start_frame: -12,
        duration_frames: 24,
        frame_rate: rate,
    };
    report.check("FX-TIME-004: first frame", -12, negative.start_frame);
    report.check("FX-TIME-004: last frame", 11, negative.last_frame());
    report.check(
        "FX-TIME-004: export frame count",
        24,
        negative.frames().count(),
    );
    report.check(
        "FX-TIME-004: seconds at frame -12 are negative and exact",
        "-1/2",
        {
            let (n, d) = rate.seconds_at(-12);
            format!("{n}/{d}")
        },
    );

    // FX-TIME-002 in its own numbering: 1001, 1002, 1004 present, 1003 requested.
    let dir = std::env::temp_dir().join("anime_compositor_b04");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    for n in [1001, 1002, 1004] {
        let file = fs::File::create(dir.join(format!("seq_{n}.png"))).unwrap();
        let mut enc = png::Encoder::new(std::io::BufWriter::new(file), 4, 4);
        enc.set_color(png::ColorType::Rgba);
        enc.set_depth(png::BitDepth::Eight);
        enc.write_header()
            .unwrap()
            .write_image_data(&[0u8; 64])
            .unwrap();
    }
    let mut files: Vec<PathBuf> = fs::read_dir(&dir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    files.sort();
    let sparse = import_sequence(&files).asset.unwrap();
    let sparse_map =
        ExposureMap::from_lengths(&[(1001, 1), (1002, 1), (1003, 1), (1004, 1)]).unwrap();
    let sparse_timing = LayerTiming {
        in_frame: 0,
        out_frame: 4,
        source_offset_frames: 0,
    };
    report.check(
        "FX-TIME-002: requesting drawing 1003 diagnoses rather than substituting",
        "MEDIA_SEQUENCE_GAP",
        match resolve(&sparse_timing, &sparse_map, &sparse, 2) {
            Err(d) => d.id.to_string(),
            Ok(other) => format!("substituted {other:?}"),
        },
    );
    report.check(
        "FX-TIME-002: 1002 and 1004 either side still resolve",
        "seq_1002.png,seq_1004.png",
        [1, 3]
            .iter()
            .map(
                |&f| match resolve(&sparse_timing, &sparse_map, &sparse, f) {
                    Ok(SourceAt::Drawing { path, .. }) => {
                        path.file_name().unwrap().to_string_lossy().into_owned()
                    }
                    other => format!("{other:?}"),
                },
            )
            .collect::<Vec<_>>()
            .join(","),
    );
    let gap_text = resolve(&sparse_timing, &sparse_map, &sparse, 2).unwrap_err();

    // -- Layer-local time -----------------------------------------------------------------------
    let offset = LayerTiming {
        in_frame: 100,
        out_frame: 110,
        source_offset_frames: 5,
    };
    report.check(
        "layer-local: frame 99 is before the layer",
        "None",
        format!("{:?}", offset.local_frame(99)),
    );
    report.check(
        "layer-local: frame 100 maps to local 5",
        "Some(5)",
        format!("{:?}", offset.local_frame(100)),
    );
    report.check(
        "layer-local: frame 109 maps to local 14",
        "Some(14)",
        format!("{:?}", offset.local_frame(109)),
    );
    report.check(
        "layer-local: frame 110 is outside the half-open interval",
        "None",
        format!("{:?}", offset.local_frame(110)),
    );
    report.check(
        "layer-local: an inactive frame reads no file and renders transparent",
        "Transparent",
        format!(
            "{:?}",
            resolve(&offset, &maps[1].1, &layer_asset("layer2"), 99).unwrap()
        ),
    );

    // -- Rounding, document 20: half away from zero ----------------------------------------------
    report.check(
        "rounding: 0.5 frames rounds away from zero to 1",
        1,
        rate.frame_at_seconds(1, 48).unwrap(),
    );
    report.check(
        "rounding: -0.5 frames rounds away from zero to -1",
        -1,
        rate.frame_at_seconds(-1, 48).unwrap(),
    );
    report.check(
        "rounding: 1.5 frames rounds to 2",
        2,
        rate.frame_at_seconds(1, 16).unwrap(),
    );
    report.check(
        "rounding: 2.5 frames rounds to 3, not to even",
        3,
        rate.frame_at_seconds(5, 48).unwrap(),
    );

    // -- Rejected at construction ----------------------------------------------------------------
    report.check(
        "overlapping spans are rejected",
        "SpansNotDisjoint { previous_end: 5, next_start: 3 }",
        format!(
            "{:?}",
            ExposureMap::new(vec![
                ExposureSpan {
                    start_frame: 0,
                    end_frame_exclusive: 5,
                    drawing_number: 1
                },
                ExposureSpan {
                    start_frame: 3,
                    end_frame_exclusive: 8,
                    drawing_number: 2
                },
            ])
            .unwrap_err()
        ),
    );
    report.check(
        "a span covering no frame is rejected",
        "EmptySpan { start_frame: 4, end_frame_exclusive: 4 }",
        format!(
            "{:?}",
            ExposureMap::new(vec![ExposureSpan {
                start_frame: 4,
                end_frame_exclusive: 4,
                drawing_number: 1
            }])
            .unwrap_err()
        ),
    );
    report.check(
        "a zero frame rate is rejected",
        "DegenerateFrameRate { numerator: 24, denominator: 0 }",
        format!("{:?}", FrameRate::new(24, 0).unwrap_err()),
    );
    report.check(
        "an inverted UI range is rejected",
        "None",
        format!(
            "{:?}",
            Composition::from_inclusive_ui_range(10, 5, rate).map(|c| c.duration_frames)
        ),
    );
    report.check(
        "a hole between spans is transparent, not an error",
        "None",
        format!(
            "{:?}",
            ExposureMap::new(vec![
                ExposureSpan {
                    start_frame: 0,
                    end_frame_exclusive: 2,
                    drawing_number: 1
                },
                ExposureSpan {
                    start_frame: 4,
                    end_frame_exclusive: 6,
                    drawing_number: 2
                },
            ])
            .unwrap()
            .drawing_at(3)
        ),
    );

    report.notes.push(gap_text.to_string());
    report.notes.push(
        resolve(&timing, &maps[2].1, &asset3, 14)
            .unwrap_err()
            .to_string(),
    );

    write_artifact(&report, &table, &maps, &comp);

    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} expected {:?} got {:?}",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn write_artifact(
    report: &Report,
    table: &[Vec<String>],
    maps: &[(String, ExposureMap)],
    comp: &Composition,
) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str(&format!(
        "# B-04 exposure and time table\n\n\
         Test T-02, requirement R-02. Produced by `tests/b04_exposure.rs`. \
         **{passed} of {} checks pass.**\n\n\
         The composition is {} frames at {} fps ({} seconds exactly), frames {} to {} inclusive.\n\n\
         Expected drawing numbers come from `Fixtures/reference_shot/exposure_sheet.json`, which \
         the fixture README calls the authority on timing. The test builds exposure spans from \
         the cadences the README states — layer 1 static, layer 2 on 1s, layer 3 on 2s — and \
         expands them; the sheet says what the expansion must equal. Layer 4 is the exception: \
         its timing is irregular and no rule generates it, so its 80 exposures are read from the \
         sheet and the check is that expanding them reproduces the sheet's per-frame numbers.\n\n",
        report.rows.len(),
        comp.duration_frames,
        comp.frame_rate,
        {
            let (n, d) = comp.frame_rate.seconds_at(comp.duration_frames as i32);
            if d == 1 { n.to_string() } else { format!("{n}/{d}") }
        },
        comp.start_frame,
        comp.last_frame(),
    ));

    out.push_str("## The 240-row frame-to-drawing table\n\n");
    out.push_str(
        "Seconds are exact rationals, not decimals. A dash means no exposure span covers that \
         frame, which renders transparent.\n\n",
    );
    out.push_str("| Frame | Seconds | Layer 1 | Layer 2 | Layer 3 | Layer 4 | Note |\n");
    out.push_str("|---|---|---|---|---|---|---|\n");
    for row in table {
        out.push_str(&format!("| {} |\n", row.join(" | ")));
    }

    out.push_str("\n## Exposure structure\n\n| Layer | Spans | Frames covered | Cadence |\n|---|---|---|---|\n");
    for (layer, map) in maps {
        let lengths: Vec<u32> = map.spans().iter().map(|s| s.len()).collect();
        let mut distinct: Vec<u32> = lengths.clone();
        distinct.sort_unstable();
        distinct.dedup();
        out.push_str(&format!(
            "| {layer} | {} | {} | span lengths {} |\n",
            map.spans().len(),
            map.exposed_frame_count(),
            distinct
                .iter()
                .map(u32::to_string)
                .collect::<Vec<_>>()
                .join(", ")
        ));
    }

    out.push_str("\n## Checks\n\n| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for r in &report.rows {
        out.push_str(&format!(
            "| {} | `{}` | `{}` | {} |\n",
            r.check,
            r.expected,
            r.actual,
            if r.pass() { "PASS" } else { "**FAIL**" }
        ));
    }

    out.push_str(
        "\n## The missing drawing, as the user would read it\n\n\
         Document 20: \"Sequence gaps are not collapsed. If drawing 1002 is referenced but \
         absent, evaluation returns a missing-source diagnostic for 1002 rather than \
         substituting 1001 or 1003.\" Layer 3 exposes drawing 7 on twenty of the 240 frames, and \
         drawings 6 and 8 are both on disk, so substitution was available every time and did not \
         happen.\n\n",
    );
    for note in &report.notes {
        out.push_str(&format!("```\n{note}\n```\n\n"));
    }

    out.push_str(
        "## Not run by this test\n\n\
         - Property keyframes: hold and linear interpolation, the before-first and after-last \
           rules (document 20, \"Property keyframes\"). They belong to B-05 with the model \
           commands that create them; nothing yet has a property to animate.\n\
         - Save and reopen of the exposure spans and the 24000/1001 rate (T-07, B-09). This test \
           proves the rate is stored exactly in memory, not that it survives a round trip through \
           the project file.\n\
         - Rendering any of these frames (B-05a, B-08). The table says which drawing each frame \
           exposes, not what it looks like.\n\
         - Matte layers evaluating at the same composition frame (document 20); mattes are \
           parked with R-04 under D-12.\n\
         - Work area, which the schema allows and no requirement yet consumes.\n\
         - Rate-limiting of the twenty layer-3 warnings into one summary carrying counts and \
           ranges, which document 28 requires of frame-level diagnostics. `resolve` returns one \
           diagnostic per frame by design, because it answers about one frame; the aggregation \
           belongs to whatever drives the frame loop, which is B-08.\n",
    );

    fs::write(repo("verification/B-04_exposure_table.md"), out).expect("write artifact");
}

/// The evaluator must not care that drawing numbers go down. Guards the reference shot's
/// out-of-order re-exposure independently of the big table.
#[test]
fn drawing_numbers_need_not_increase() {
    let map = ExposureMap::from_lengths(&[(14, 3), (11, 3), (16, 3)]).expect("valid");
    assert_eq!(map.drawing_at(0), Some(14));
    assert_eq!(map.drawing_at(3), Some(11));
    assert_eq!(map.drawing_at(6), Some(16));
}

/// `TimeError` is user-facing through B-05's commands, so it has to read as English.
#[test]
fn time_errors_describe_themselves() {
    let err = ExposureMap::new(vec![
        ExposureSpan {
            start_frame: 0,
            end_frame_exclusive: 5,
            drawing_number: 1,
        },
        ExposureSpan {
            start_frame: 3,
            end_frame_exclusive: 8,
            drawing_number: 2,
        },
    ])
    .unwrap_err();
    assert!(matches!(err, TimeError::SpansNotDisjoint { .. }));
    assert!(err.to_string().contains("overlap"), "{err}");
}
