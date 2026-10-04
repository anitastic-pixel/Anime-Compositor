//! B-175 / D-290: a composition's frame rate typed as a person types it.
//!
//! Expected values are document 20's: broadcast rates are the exact n×1000/1001, never the
//! rounded decimal. The After Effects tutorials in P-26 all ask for 23.976.

use anime_compositor::time::FrameRate;

fn rate(text: &str) -> Option<(u32, u32)> {
    FrameRate::parse(text).map(|r| (r.numerator(), r.denominator()))
}

#[test]
fn broadcast_decimals_are_the_exact_1001_rates() {
    assert_eq!(rate("23.976"), Some((24000, 1001)));
    // The two-place spelling people also type.
    assert_eq!(rate("23.98"), Some((24000, 1001)));
    assert_eq!(rate("29.97"), Some((30000, 1001)));
    assert_eq!(rate("59.94"), Some((60000, 1001)));
    assert_eq!(rate("47.952"), Some((48000, 1001)));
    // What the settings window shows for 24000/1001 and sends back unchanged.
    assert_eq!(rate("23.976024"), Some((24000, 1001)));
}

#[test]
fn whole_fractions_and_plain_decimals() {
    assert_eq!(rate("24"), Some((24, 1)));
    assert_eq!(rate(" 30 "), Some((30, 1)));
    assert_eq!(rate("24000/1001"), Some((24000, 1001)));
    assert_eq!(rate("48/2"), Some((24, 1)));
    assert_eq!(rate("12.5"), Some((25, 2)));
}

#[test]
fn nonsense_and_zero_are_refused() {
    for text in ["", "0", "0.0", "-24", "abc", "24/0", "nan", "inf", "1e9"] {
        assert_eq!(rate(text), None, "{text:?}");
    }
}
