//! B-194: D-311, a composition's background colour.
//!
//! P-26, tutorial 1 (Shockwave) sets After Effects' composition background colour; here a
//! composition had none. It is shown behind the picture in the viewer, with the checkerboard off, and an MP4, which has no alpha, is laid over it. A PNG, an EXR and a
//! composition inside another stay see-through, as in After Effects.

use anime_compositor::command::Command;
use anime_compositor::export::lay_over;
use anime_compositor::persist;

fn shot() -> anime_compositor::command::Document {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("verification/B-08a_project.json");
    persist::load(&path).unwrap_or_else(|d| panic!("{}", d.message)).document
}

fn set(doc: &mut anime_compositor::command::Document, background_color: Option<[f64; 3]>) -> Result<(), String> {
    let comp = doc.project().compositions[0].clone();
    doc.apply(Command::SetCompositionSettings {
        composition: comp.id.clone(),
        name: comp.name.clone(),
        width: comp.width,
        height: comp.height,
        frame_rate: comp.frame_rate,
        duration_frames: comp.duration_frames,
        sheet_details: comp.sheet_details.clone(),
        background_color,
    })
    .map(|_| ())
    .map_err(|d| d.message)
}

fn saved(doc: &anime_compositor::command::Document) -> serde_json::Value {
    let text = persist::to_json(doc.project(), &Default::default());
    serde_json::from_str::<serde_json::Value>(&text).unwrap()["compositions"][0].clone()
}

#[test]
fn it_is_set_saved_read_and_undone() {
    let mut doc = shot();
    assert_eq!(doc.project().compositions[0].background_color, None, "an old file has none");
    assert!(saved(&doc).get("background_color").is_none(), "and saves none");

    set(&mut doc, Some([0.5, 0.25, 1.0])).unwrap();
    let comp = saved(&doc);
    assert_eq!(comp["background_color"], serde_json::json!([0.5, 0.25, 1]), "a set one is written");
    let back = persist::load_str(&persist::to_json(doc.project(), &Default::default())).unwrap();
    assert_eq!(back.document.project().compositions[0].background_color, Some([0.5, 0.25, 1.0]), "and read back");

    doc.undo().expect("one step to undo");
    assert_eq!(doc.project().compositions[0].background_color, None, "undo takes it away");

    assert!(set(&mut doc, Some([1.5, 0.0, 0.0])).is_err(), "a channel past 1 is refused");
    assert!(set(&mut doc, Some([0.0, -0.1, 0.0])).is_err(), "and one below 0");
}

#[test]
fn a_bad_one_in_a_file_is_refused() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("verification/B-08a_project.json");
    let mut file: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(path).unwrap()).unwrap();
    for bad in [serde_json::json!([1, 0]), serde_json::json!([2, 0, 0]), serde_json::json!("red")] {
        file["compositions"][0]["background_color"] = bad.clone();
        assert!(persist::load_str(&file.to_string()).is_err(), "{bad} is refused");
    }
}

#[test]
fn an_mp4_is_laid_over_it() {
    // Opaque, half and clear, red over a blue-grey 0.2, 0.2, 0.5 (linear).
    let mut px = vec![255, 0, 0, 255, 255, 0, 0, 128, 255, 0, 0, 0];
    lay_over(&mut px, [0.2, 0.2, 0.5]);
    let bg = [0.2f32, 0.2, 0.5].map(|v| anime_compositor::color::linear_to_srgb(v) * 255.0);
    let a = 128.0 / 255.0;
    let half = [255.0 * a + bg[0] * (1.0 - a), bg[1] * (1.0 - a), bg[2] * (1.0 - a)].map(|v: f32| v.round() as u8);
    assert_eq!(&px[0..4], &[255, 0, 0, 255], "an opaque pixel stays");
    assert_eq!(&px[4..8], &[half[0], half[1], half[2], 255], "a half one is mixed with the colour");
    assert_eq!(&px[8..12], &[bg[0].round() as u8, bg[1].round() as u8, bg[2].round() as u8, 255], "a clear one is the colour");

    // Over black it is what the MP4 always showed: the colour times its alpha.
    let mut px = vec![200, 100, 50, 128];
    lay_over(&mut px, [0.0, 0.0, 0.0]);
    assert_eq!(px, [200, 100, 50].map(|v: u8| (v as f32 * a).round() as u8).into_iter().chain([255]).collect::<Vec<_>>());
}
