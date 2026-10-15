use super::*;

/// Audit B7: a panic is said as a sentence and the program keeps going.
#[test]
fn a_panic_is_said_and_the_window_keeps_going() {
    assert_eq!(caught("drawing a frame", || 7), Ok(7));
    let said = caught("drawing a frame", || -> u8 { panic!("the frame was {} pixels wide", 0) }).unwrap_err();
    assert_eq!(
        said,
  "Something went wrong inside the program while drawing a frame: the frame was 0 pixels wide. That was stopped and the window kept going. If anything looks wrong, save under a new name."
    );
    let lock = std::sync::Mutex::new(1);
    assert!(caught("exporting", || { let _held = lock.lock().unwrap(); panic!("x") }).is_err());
    assert!(lock.is_poisoned());
    lock.clear_poison();
    assert_eq!(*lock.lock().unwrap(), 1);
}
