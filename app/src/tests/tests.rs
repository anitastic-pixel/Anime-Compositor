use super::*;

#[test]
fn reads_both_forms_and_the_quality_beside_them() {
    assert_eq!(parse("/at/0", None), Some((Ask::At(0), None)));
    assert_eq!(parse("/at/16683", None), Some((Ask::At(16683), None)));
    assert_eq!(parse("/frame/100", None), Some((Ask::Frame(100), None)));
    assert_eq!(parse("/play/30", None), Some((Ask::Play(30), None)));
    assert_eq!(
        parse("/frame/-3", Some("q=full")),
        Some((Ask::Frame(-3), Some(PreviewQuality::Full)))
    );
    assert_eq!(
        parse("/at/1000", Some("t=9&q=draft")),
        Some((Ask::At(1000), Some(PreviewQuality::Draft)))
    );
}

#[test]
fn refuses_what_it_does_not_understand() {
    for path in ["/", "/at", "/at/", "/at/soon", "/frame/1/2", "/frames/1"] {
        assert_eq!(parse(path, None), None, "{path} should not be understood");
    }
}

/// A quality that cannot be read leaves the preview alone rather than changing it. Silently
/// switching resolution on a typo would put a frame on screen at a resolution nobody asked
/// for, with an indicator that agreed with it.
#[test]
fn an_unreadable_quality_changes_nothing() {
    assert_eq!(parse("/at/0", Some("q=fastest")), Some((Ask::At(0), None)));
    assert_eq!(
        parse("/at/0", Some("quality=full")),
        Some((Ask::At(0), None))
    );
}

/// Expected bytes worked out from UTF-8 by hand, not read off this function: 夜 is
/// E5 A4 9C and 空 is E7 A9 BA. A wrong encoder that agreed with itself would still fail
/// here.
#[test]
fn japanese_survives_the_journey_into_a_header() {
    assert_eq!(for_a_header("夜空"), "%E5%A4%9C%E7%A9%BA");
    assert_eq!(
        for_a_header("media/背景/夜空.png"),
        "media/%E8%83%8C%E6%99%AF/%E5%A4%9C%E7%A9%BA.png"
    );
}

/// The other direction, against the same hand-worked UTF-8: E5 A4 9C is 夜, E7 A9 BA is 空.
/// A path that arrives half-decoded is a path to a file that is not there.
#[test]
fn japanese_survives_the_journey_back() {
    assert_eq!(from_a_query("%E5%A4%9C%E7%A9%BA"), "夜空");
    assert_eq!(
        from_a_query("C:%5CShots%5C%E5%A4%9C%E7%A9%BA.json"),
        r"C:\Shots\夜空.json"
    );
    assert_eq!(from_a_query("100%25 sure"), "100% sure");
    // A stray percent is kept rather than eating what follows it. Shortening a path by two
    // characters would open the wrong file or none.
    assert_eq!(from_a_query("50% of it"), "50% of it");
}

/// Both directions, on the strings this window actually carries.
#[test]
fn the_two_encodings_are_each_other() {
    for text in [
        "夜空",
        r"C:\Shots\背景\shot.json",
        "2 of the files for \"layer3\" are missing.",
        "100% sure",
    ] {
        assert_eq!(from_a_query(&for_a_header(text)), text);
    }
}

/// Newest first, and a project opened twice appears once. The second rule is what makes the
/// list a list of projects rather than a log of openings.
#[test]
fn the_recent_list_is_newest_first_and_holds_each_project_once() {
    let list = remembered(&[], r"C:\a.json");
    assert_eq!(list, vec![r"C:\a.json"]);

    let list = remembered(&list, r"C:\b.json");
    assert_eq!(list, vec![r"C:\b.json", r"C:\a.json"]);

    let list = remembered(&list, r"C:\a.json");
    assert_eq!(list, vec![r"C:\a.json", r"C:\b.json"]);
}

/// ADR-001 is Windows only, where these are one file. Two entries for one project would be
/// two ways to open the same thing, one of them looking like something else.
#[test]
fn one_file_spelled_two_ways_is_one_entry() {
    let list = remembered(&[r"C:\Shots\A.JSON".to_string()], r"c:\shots\a.json");
    assert_eq!(list, vec![r"c:\shots\a.json"]);
}

/// The cap holds, and it is the oldest that goes.
#[test]
fn the_recent_list_forgets_the_oldest_first() {
    let mut list: Vec<String> = Vec::new();
    for n in 0..12 {
        list = remembered(&list, &format!(r"C:\shot{n}.json"));
    }
    assert_eq!(list.len(), RECENT_LIMIT);
    assert_eq!(list[0], r"C:\shot11.json");
    assert_eq!(list[RECENT_LIMIT - 1], r"C:\shot4.json");
}

/// Newlines separate the notes and `%` would otherwise make decoding ambiguous, so both are
/// encoded. Ordinary punctuation is left alone, because a diagnostic is meant to be read in
/// the header as well as after it.
#[test]
fn encodes_only_what_a_header_cannot_carry() {
    assert_eq!(for_a_header("one\ntwo"), "one%0Atwo");
    assert_eq!(for_a_header("100% sure"), "100%25 sure");
    assert_eq!(
        for_a_header("2 of the files for \"layer3\" are missing."),
        "2 of the files for \"layer3\" are missing."
    );
}
