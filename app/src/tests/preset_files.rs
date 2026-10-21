use super::*;

fn fixture(file: &str) -> String {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../Fixtures/preset_file")
        .join(file);
    std::fs::read_to_string(path).expect("read the fixture")
}

#[test]
fn a_name_the_window_has_comes_in_with_a_number() {
    let have = ["Poster".to_string(), "Night 2".to_string()];
    let text = fixture("fx_pre_002.fxpreset");
    let (list, said) = imported_presets(&text, "fx_pre_002.fxpreset", &have).expect("imported");
    let names: Vec<&str> = list
        .as_array()
        .unwrap()
        .iter()
        .map(|p| p["name"].as_str().unwrap())
        .collect();
    assert_eq!(names, ["Night", "Poster 2", "Lifted"]);
    assert_eq!(
        said,
        "Imported 3 presets from fx_pre_002.fxpreset. Names you already had came in with a number: \"Poster\" as \"Poster 2\"."
    );
    let again = imported_presets(&text, "f", &["Night".into(), "Night 2".into()])
        .unwrap()
        .0;
    assert_eq!(again[0]["name"], "Night 3");
    assert_eq!(
        again[0]["effects"],
        serde_json::from_str::<serde_json::Value>(&text).unwrap()["presets"][0]["effects"]
    );
}

#[test]
fn the_starter_presets_go_to_the_page_whole() {
    let sent: serde_json::Value = serde_json::from_str(&starter_presets()).unwrap();
    let mut read = persist::read_presets(persist::STARTER_PRESETS).unwrap();
    read.extend(persist::read_presets(persist::STARTER_ANIMATIONS).unwrap());
    let sent = sent.as_array().unwrap();
    assert_eq!(sent.len(), read.len());
    for (s, r) in sent.iter().zip(&read) {
        assert_eq!(s["name"], r.name.as_str());
        assert!(!s["about"].as_str().unwrap().is_empty());
        let effects = s["effects"].as_array().map_or(0, Vec::len);
        assert_eq!(effects, r.effects.len());
        assert_eq!(s["kind"] == "animation", r.animation.is_some());
    }
}

#[test]
fn a_refused_file_brings_nothing() {
    let said = imported_presets(&fixture("fx_pre_018.fxpreset"), "f", &[]).unwrap_err();
    assert!(said.contains("core.lens_sparkle"), "{said}");
}

#[test]
fn export_writes_nothing_import_would_refuse() {
    let text = fixture("fx_pre_015.fxpreset");
    let list = serde_json::from_str::<serde_json::Value>(&text).unwrap()["presets"].to_string();
    assert!(persist::write_presets(&list).is_err());
}
