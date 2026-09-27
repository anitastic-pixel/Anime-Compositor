//! B-82: the channel mixer in the core, against D-139.
//!
//! Writes `verification/B-82_channel_mixer_table.md`.
//!
//! Every expected pixel is `Fixtures/channel_mixer/expected_channel_mixer.json`, written by
//! `tools/channel_mixer_reference.py` before this code existed and printed in document 25 as
//! FX-MIXER-001 to 022. Tolerance 2e-5. Nothing here is a snapshot of a run.

mod effect_table;

use effect_table::{keys, set, Table};

use anime_compositor::effects::Effect;

fn mixer(red: &[f64], green: &[f64], blue: &[f64], monochrome: &str) -> Effect {
    Effect::ChannelMixer {
        red: red.to_vec(),
        green: green.to_vec(),
        blue: blue.to_vec(),
        monochrome: monochrome.to_string(),
    }
}

const R: [f64; 4] = [100.0, 0.0, 0.0, 0.0];
const G: [f64; 4] = [0.0, 100.0, 0.0, 0.0];
const B: [f64; 4] = [0.0, 0.0, 100.0, 0.0];

#[test]
fn b82_channel_mixer() {
    let mut t = Table::new(
        "channel_mixer",
        "# B-82: channel mixer\n\nD-139, accepted on 2026-09-26 by the owner's message asking \
         for thirty more effects, the sixth of the third batch. Every expected pixel is \
         `Fixtures/channel_mixer/expected_channel_mixer.json`, written by \
         `tools/channel_mixer_reference.py` before this code existed and printed in document 25 \
         as FX-MIXER-001 to 022. The build's frame is compared sample by sample; the answer is \
         the largest difference over all of them, against the catalogue's tolerance of 2e-5.\n",
    );

    t.heading("FX-MIXER-001 to 022 (document 25)");
    t.fixtures("expected_channel_mixer.json");

    t.heading("How far it reaches");
    let got = mixer(&B, &G, &R, "on").bounds_expansion();
    t.row("it grows the drawing's bounds by nothing", &got.to_string(), got == 0);
    let mut draft = mixer(&B, &G, &R, "off");
    draft.scale_distances(|d| d * 0.5);
    t.row(
        "a half-size draft preview changes nothing: it has no distances",
        &format!("{draft:?}"),
        draft == mixer(&B, &G, &R, "off"),
    );

    t.heading("The file");
    t.round_trips(&[
        "fx_mixer_001.json",
        "fx_mixer_006.json",
        "fx_mixer_009.json",
        "fx_mixer_012.json",
        "fx_mixer_013.json",
        "fx_mixer_016.json",
        "fx_mixer_019.json",
        "fx_mixer_020.json",
        "fx_mixer_021.json",
        "fx_mixer_022.json",
    ]);
    t.shape_refused(
        "fx_mixer_001.json",
        "no `blue` at all",
        r#"{"red": [100, 0, 0, 0], "green": [0, 100, 0, 0], "monochrome": "off"}"#,
    );
    t.shape_refused(
        "fx_mixer_001.json",
        "a red row that is a word",
        r#"{"red": "all", "green": [0, 100, 0, 0], "blue": [0, 0, 100, 0], "monochrome": "off"}"#,
    );

    t.heading("Commands");
    let mut document = t.load("fx_mixer_001.json").document;
    t.refused(
        &mut document,
        vec![
            ("red 201, 0, 0, 0", set(mixer(&[201.0, 0.0, 0.0, 0.0], &G, &B, "off"))),
            ("green's constant -201", set(mixer(&R, &[0.0, 100.0, 0.0, -201.0], &B, "off"))),
            ("blue as three numbers", set(mixer(&R, &G, &[0.0, 0.0, 100.0], "off"))),
            ("monochrome \"yes\"", set(mixer(&R, &G, &B, "yes"))),
            (
                "red keyed to 0, 0, 300, 0",
                keys("red", &[(0, &R), (4, &[0.0, 0.0, 300.0, 0.0])]),
            ),
        ],
    );
    t.taken(
        &mut document,
        "fx_mixer_001.json",
        vec![
            (
                "every number at -200, the bottom,",
                set(mixer(&[-200.0; 4], &[-200.0; 4], &[-200.0; 4], "off")),
            ),
            (
                "every number at 200 with monochrome on, the top,",
                set(mixer(&[200.0; 4], &[200.0; 4], &[200.0; 4], "on")),
            ),
            ("red keyed from red to blue", keys("red", &[(0, &R), (4, &B)])),
        ],
    );

    t.heading("The frame does not depend on how it is cut up");
    t.tiles(&[("fx_mixer_002.json", 0), ("fx_mixer_008.json", 0), ("fx_mixer_012.json", 2)]);

    t.finish("B-82_channel_mixer_table.md");
}
