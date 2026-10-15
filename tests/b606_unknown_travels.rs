//! ADR-008: "Unknown additive data is preserved" -- also when the layer, effect or mask holding
//! it moves. Saving used to find a record's unknown keys again by its place in the file (a layer's
//! ID, an effect's instance ID, a mask's place in the list), so a pre-composed or copied layer
//! lost them, and an effect this build does not have was saved with no parameters at all; and
//! deleting the first of two masks gave the second one the first one's keys.

use anime_compositor::command::{precompose, Command};
use anime_compositor::model::Id;
use anime_compositor::persist;
use serde_json::{json, Value as J};

const FIXTURE: &str = include_str!("../Fixtures/projects/unknown_effect_project.json");

/// Every layer in every composition of a saved project, with its composition's ID.
fn layers(saved: &J) -> Vec<(String, J)> {
    let mut out = Vec::new();
    for comp in saved["compositions"].as_array().expect("compositions") {
        for layer in comp["layers"].as_array().expect("layers") {
            out.push((comp["id"].as_str().unwrap_or("").to_string(), layer.clone()));
        }
    }
    out
}

#[test]
fn an_unknown_effect_keeps_its_settings_on_every_copy_of_its_layer() {
    let loaded = persist::load_str(FIXTURE).expect("the fixture opens");
    let mut document = loaded.document;
    let main = Id::new("comp-main");
    let cel = document
        .project()
        .composition(&main)
        .unwrap()
        .layer(&Id::new("layer-cel"))
        .unwrap()
        .clone();

    // A copy under a new ID, as Duplicate, Split and Paste in the window all make one.
    let mut copy = cel.clone();
    copy.id = Id::new("layer-copy");
    document
        .apply(Command::AddLayer {
            composition: main.clone(),
            layer: Box::new(copy),
            index: 1,
        })
        .expect("the copy goes in");
    // And the original moved into a composition of its own.
    let commands = precompose(
        document.project(),
        &main,
        &[Id::new("layer-cel")],
        Id::new("comp-inner"),
        Id::new("layer-precomp"),
        "Inner",
    )
    .expect("pre-compose is allowed");
    document.apply_all(commands).expect("pre-compose applies");

    let saved: J =
        serde_json::from_str(&persist::to_json(document.project(), &loaded.preserved)).unwrap();
    let with_effects: Vec<(String, J)> = layers(&saved)
        .into_iter()
        .filter(|(_, l)| l["effects"].as_array().is_some_and(|e| !e.is_empty()))
        .collect();
    let where_: Vec<String> = with_effects
        .iter()
        .map(|(c, l)| format!("{c}/{}", l["id"].as_str().unwrap()))
        .collect();
    assert_eq!(where_, ["comp-main/layer-copy", "comp-inner/layer-cel"]);
    for (comp, layer) in &with_effects {
        let effect = &layer["effects"][0];
        let at = format!("{comp}/{}", layer["id"]);
        assert_eq!(effect["type_id"], "vendor.future.effect", "{at}");
        assert_eq!(
            effect["parameters"],
            json!({"strength": 0.5}),
            "{at}: the effect's settings"
        );
        assert_eq!(
            effect["opaque_unknown_data"],
            json!({"keep_me": true}),
            "{at}: the effect's unknown record"
        );
    }
}

#[test]
fn deleting_a_mask_leaves_the_others_their_own_unknown_keys() {
    let mut file: J = serde_json::from_str(FIXTURE).unwrap();
    let layer = &mut file["compositions"][0]["layers"][0];
    layer.as_object_mut().unwrap().remove("mask");
    let mask = |name: &str, extra: &str| {
        json!({
            "name": name, "enabled": true, "inverted": false, "mode": "add",
            "opacity": 1, "feather_px": 0, "expansion_px": 0,
            "path": {"base": {"points": [
                {"point": [0, 0], "in": [0, 0], "out": [0, 0]},
                {"point": [9, 0], "in": [0, 0], "out": [0, 0]},
                {"point": [0, 9], "in": [0, 0], "out": [0, 0]}
            ]}, "keyframes": []},
            extra: {"owner": name}
        })
    };
    layer["masks"] = json!([mask("Mask 1", "first_only"), mask("Mask 2", "second_only")]);

    let loaded = persist::load_str(&file.to_string()).expect("the masked file opens");
    let mut document = loaded.document;
    let main = Id::new("comp-main");
    let id = Id::new("layer-cel");
    let mut masks = document
        .project()
        .composition(&main)
        .unwrap()
        .layer(&id)
        .unwrap()
        .masks
        .clone();
    masks.remove(0);
    document
        .apply(Command::SetMasks {
            composition: main,
            layer_id: id,
            masks,
        })
        .expect("the first mask is deleted");

    let saved: J =
        serde_json::from_str(&persist::to_json(document.project(), &loaded.preserved)).unwrap();
    let left = &saved["compositions"][0]["layers"][0]["masks"];
    assert_eq!(left.as_array().map(Vec::len), Some(1));
    assert_eq!(left[0]["name"], "Mask 2");
    assert_eq!(
        left[0]["second_only"],
        json!({"owner": "Mask 2"}),
        "its own key is kept"
    );
    assert!(
        left[0].get("first_only").is_none(),
        "the deleted mask's key is not handed on: {}",
        left[0]
    );
}
