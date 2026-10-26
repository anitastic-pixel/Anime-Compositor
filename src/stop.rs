//! D-893: a viewer frame that is being drawn can be stopped when newer input makes it useless.
//!
//! One process-wide count of stops. A thread that draws a viewer frame [`watching`] remembers the
//! count as it was when its request arrived; [`asked`] is then true once anything has called
//! [`stop`] since. Threads that are not watching (exports, stored checks, tests) never see a stop,
//! so only the viewer's frames can be stopped. It is looked at between the top composition's
//! layers and, while the finished plan is drawn ([`tiles`]), before each tile. A stopped frame is
//! an error, never a picture: the caller throws it away and nothing of it is kept.

use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};

static STOPS: AtomicU64 = AtomicU64::new(0);

thread_local! {
    static SINCE: Cell<Option<u64>> = const { Cell::new(None) };
    static TILES: Cell<bool> = const { Cell::new(false) };
    static AT_CHECK: Cell<u32> = const { Cell::new(0) };
}

/// The count now: what a request takes as it arrives, to hand to [`watching`].
pub fn now() -> u64 {
    STOPS.load(Ordering::SeqCst)
}

/// Stops every frame that arrived before this call and is still being drawn.
pub fn stop() {
    STOPS.fetch_add(1, Ordering::SeqCst);
}

/// Runs `job` on this thread as a frame that arrived when the count was `since`.
pub fn watching<T>(since: u64, job: impl FnOnce() -> T) -> T {
    let before = SINCE.with(|s| s.replace(Some(since)));
    let out = job();
    SINCE.with(|s| s.set(before));
    out
}

/// Runs `job`, the drawing of a finished plan, with its tiles stoppable ([`tile_watch`]). Only the
/// viewer's last draw of a frame is; a composition layer's inner frame, drawn while planning, is
/// never cut short.
pub fn tiles<T>(job: impl FnOnce() -> T) -> T {
    let before = TILES.with(|t| t.replace(true));
    let out = job();
    TILES.with(|t| t.set(before));
    out
}

/// For B-773's test: the `n`th look on this thread from now ([`asked`], [`tile_watch`]) calls
/// [`stop`] first, as a `/stop` arriving at that point would. 0 never does.
#[doc(hidden)]
pub fn stop_at_check(n: u32) {
    AT_CHECK.with(|c| c.set(n));
}

fn count_down() {
    let left = AT_CHECK.with(|c| c.get());
    if left > 0 {
        AT_CHECK.with(|c| c.set(left - 1));
        if left == 1 {
            stop();
        }
    }
}

/// True when the frame this thread is drawing has been stopped since it arrived.
pub fn asked() -> bool {
    count_down();
    passed(SINCE.with(|s| s.get()))
}

/// What a tile drawn on another thread hands to [`passed`]: the frame's count inside [`tiles`],
/// and `None`, never stopped, outside it.
pub fn tile_watch() -> Option<u64> {
    if !TILES.with(|t| t.get()) {
        return None;
    }
    count_down();
    SINCE.with(|s| s.get())
}

/// True when a stop came after `since`, from [`tile_watch`].
pub fn passed(since: Option<u64>) -> bool {
    since.is_some_and(|since| since != now())
}

#[cfg(test)]
mod tests {
    #[test]
    fn only_a_watched_frame_that_arrived_before_the_stop_sees_it() {
        assert!(!super::asked());
        let since = super::now();
        super::watching(since, || {
            assert!(!super::asked());
            assert_eq!(super::tile_watch(), None, "tiles outside `tiles` are never stopped");
            super::stop();
            assert!(super::asked());
            assert!(super::tiles(|| super::passed(super::tile_watch())));
        });
        assert!(!super::asked());
        super::watching(super::now(), || assert!(!super::asked()));
    }
}
