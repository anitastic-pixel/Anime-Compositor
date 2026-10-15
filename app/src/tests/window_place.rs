use super::*;

/// The owner's display less its taskbar, and a second display to its right.
const MAIN: Area = (0, 0, 3840, 2088);
const SECOND: Area = (3840, 0, 1920, 1040);

fn placed(x: i32, y: i32, width: u32, height: u32, maximized: bool) -> Placement {
    Placement {
        x,
        y,
        width,
        height,
        maximized,
    }
}

fn words(p: Option<Placement>) -> String {
    match p {
        None => "nothing (damaged)".into(),
        Some(p) => format!(
            "{} by {} at {},{}{}",
            p.width,
            p.height,
            p.x,
            p.y,
            if p.maximized { ", maximized" } else { "" }
        ),
    }
}

/// Each case: what the file says (or that there is none), the displays attached, and where the
/// window must open.
#[test]
fn b174_the_window_opens_where_it_was_left_and_never_off_screen() {
    let first = placed(480, 261, 2880, 1566, true);
    let file = crate::test_temp()
        .join("anime_compositor_b174")
        .join("window.txt");
    std::fs::create_dir_all(file.parent().unwrap()).unwrap();
    let _ = std::fs::remove_file(&file);
    let read = || {
        std::fs::read_to_string(&file)
            .ok()
            .and_then(|t| placement_from_text(&t))
    };
    let mut rows: Vec<(String, String, String)> = Vec::new();
    let mut check = |case: &str, expected: Placement, saved: Option<Placement>, areas: &[Area]| {
        rows.push((
            case.to_string(),
            words(Some(expected)),
            words(Some(placement_on_screen(saved, areas, MAIN))),
        ));
    };

    // Reading and writing the file itself.
    check("No file (the very first launch): maximized, Restore goes to three quarters of the main display", first, read(), &[MAIN]);
    for p in [
        placed(300, 250, 2400, 1500, false),
        placed(-11, 0, 1931, 2077, false),
        placed(4000, 100, 1600, 900, true),
    ] {
        std::fs::write(&file, placement_text(p)).unwrap();
        check(
            &format!("Written as `{}`, read back", placement_text(p).trim()),
            p,
            read(),
            &[MAIN, SECOND],
        );
    }
    std::fs::remove_file(&file).unwrap();

    // A damaged file opens as a first launch does.
    for text in [
        "",
        "garbage",
        "300 250 2400",
        "300 250 wide 1500 normal",
        "300 250 2400 1500 sideways",
        "300 250 2400 1500 normal extra",
        "300 250 50 1500 normal",
        "300 250 2400 1500000 normal",
        "300.5 250 2400 1500 normal",
    ] {
        std::fs::write(&file, text).unwrap();
        check(&format!("Damaged: `{text}`"), first, read(), &[MAIN]);
    }
    std::fs::remove_file(&file).unwrap();

    // Displays that are, or are no longer, there.
    check(
        "Left at 20000,20000, where no display is",
        first,
        Some(placed(20000, 20000, 1600, 1000, false)),
        &[MAIN],
    );
    check(
        "Left on the second display, still attached",
        placed(4000, 100, 1600, 900, false),
        Some(placed(4000, 100, 1600, 900, false)),
        &[MAIN, SECOND],
    );
    check(
        "Left on the second display, since unplugged",
        first,
        Some(placed(4000, 100, 1600, 900, false)),
        &[MAIN],
    );
    check(
        "Left maximized on the second display, since unplugged",
        first,
        Some(placed(4000, 100, 1600, 900, true)),
        &[MAIN],
    );
    check(
        "Title bar above the top of the screen",
        first,
        Some(placed(300, -40, 2400, 1500, false)),
        &[MAIN],
    );
    check(
        "Title bar just inside the top of the screen",
        placed(300, -10, 2400, 1500, false),
        Some(placed(300, -10, 2400, 1500, false)),
        &[MAIN],
    );
    check(
        "Bigger than the display it is on: made to fit it",
        placed(3900, 0, 1920, 1040, false),
        Some(placed(3900, 0, 2400, 1500, false)),
        &[MAIN, SECOND],
    );
    check(
        "At the edge of what a number holds",
        first,
        Some(placed(i32::MAX, i32::MIN, 100_000, 100_000, false)),
        &[MAIN],
    );
    check(
        "No display reported but the main one's work area",
        first,
        Some(placed(300, 250, 2400, 1500, false)),
        &[],
    );

    let passed = rows.iter().filter(|(_, e, a)| e == a).count();
    let mut text = format!(
        "# B-174: where the window opens\n\nWritten by `cargo test -p anime_compositor_app window_place`. The window remembers its \
         place in `window.txt` beside the recent list (`%APPDATA%\\dev.anitastic.anime-compositor\\`): one line, the outer \
         corner and inner size in pixels and `normal` or `maximized`. **The rule: it opens where it was left, unless the middle \
         of its title bar would be on no display, or the file is missing or damaged; then it opens maximized on the main \
         display, as the very first launch does.** Displays here: the main one {} by {} (less its taskbar) and, where named, a \
         second one {} by {} to its right. Sizes are width by height, at the window's corner.\n\n**{passed} of {} checks pass.**\n\n\
         | Case | Expected | Opens | Result |\n|---|---|---|---|\n",
        MAIN.2, MAIN.3, SECOND.2, SECOND.3, rows.len(),
    );
    for (case, expected, actual) in &rows {
        text.push_str(&format!(
            "| {case} | {expected} | {actual} | {} |\n",
            if expected == actual { "PASS" } else { "FAIL" }
        ));
    }
    let table = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("verification/B-174_window_table.md");
    std::fs::write(table, text).expect("write the table");
    assert_eq!(passed, rows.len(), "see verification/B-174_window_table.md");
}
