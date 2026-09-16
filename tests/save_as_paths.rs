//! Save As into another folder must reopen on the same drawings.
//!
//! The owner's B-13e playtest on 2026-09-16 saved a project into another folder and reopened it
//! on a blank canvas: media paths are stored relative to the project file and were written
//! unchanged beside a different one. `persist::rebased` is the fix, and this is what fails if
//! it stops being one.

use std::collections::BTreeMap;
use std::path::PathBuf;

use anime_compositor::model::{Asset, Id, Project};
use anime_compositor::persist;

#[test]
fn a_project_saved_elsewhere_still_finds_its_drawings() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let from = repo.join("Fixtures/reference_shot");
    let outside = std::env::temp_dir().join("save_as_paths");
    let inside = from.join("layer1");

    let mut project = Project::new(Id::new("p"));
    project.assets.push(Asset::still(Id::new("still"), "still", "layer1/layer1_000.png"));
    project.assets.push(
        Asset::sequence(Id::new("seq"), "seq", "layer1_###.png")
            .with_frames(BTreeMap::from([(0, "layer1/layer1_000.png".to_string())])),
    );

    // Into a folder the drawings are not under: whole paths, and every one still on disk.
    let moved = persist::rebased(&project, &from, &outside);
    for asset in &moved.assets {
        for file in asset.files() {
            assert!(outside.join(file).is_file(), "{file} is lost from {}", outside.display());
        }
    }

    // Into a folder they are under: relative again, so a project and its media still move together.
    let near = persist::rebased(&project, &from, &inside);
    assert_eq!(near.assets[0].path.as_deref(), Some("layer1_000.png"));
    assert_eq!(near.assets[1].frames[&0], "layer1_000.png");

    // Into the folder it came from: nothing changes.
    assert_eq!(persist::rebased(&project, &from, &from), project);
}
