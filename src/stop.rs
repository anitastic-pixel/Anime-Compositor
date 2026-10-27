//! D-893: a viewer frame that is being drawn can be stopped when newer input makes it useless.
//!
//! One process-wide count of stops. A thread that draws a viewer frame [`watching`] remembers the
//! count as it was when its request arrived; [`asked`] is then true once anything has called
//! [`stop`] since. Threads that are not watching (exports, stored checks, tests) never see a stop,
//! so only the viewer's frames can be stopped. It is looked at between the top composition's
//! layers and, while the finished plan is drawn ([`tiles`]), before each tile. A stopped frame is
//! an error, never a picture: the caller throws it away and nothing of it is kept.
//!
//! B-780 (D-900): a frame made ahead in the background ([`behind`]) is stopped as well by any
//! frame the hand asks for ([`by_hand`]), so the RAM preview's workers give way at once.

use std::cell::Cell;
use std::sync::atomic::{AtomicU64, Ordering};

static STOPS: AtomicU64 = AtomicU64::new(0);
/// B-780: stops and frames asked for by hand, together.
static HAND: AtomicU64 = AtomicU64::new(0);

/// What a frame being drawn is watching: the count it began at, and which count.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Watch {
    since: u64,
    behind: bool,
}

thread_local! {
    static SINCE: Cell<Option<Watch>> = const { Cell::new(None) };
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
    HAND.fetch_add(1, Ordering::SeqCst);
}

/// B-780: a frame was asked for by hand; every frame being made [`behind`] stops.
pub fn by_hand() {
    HAND.fetch_add(1, Ordering::SeqCst);
}

/// Runs `job` on this thread as a frame that arrived when the count was `since`.
pub fn watching<T>(since: u64, job: impl FnOnce() -> T) -> T {
    watch(Watch { since, behind: false }, job)
}

/// B-787: the count of stops and frames asked for by hand, for [`behind`].
pub fn hand() -> u64 {
    HAND.load(Ordering::SeqCst)
}

/// B-780: runs `job` as a frame made ahead in the background, chosen when the count was `since`
/// ([`hand`]): any [`stop`] or [`by_hand`] after that stops it. B-787: `since` is taken when the
/// frame is chosen, not when it starts, so a frame chosen just before the hand's ask and started
/// just after it still gives way.
pub fn behind<T>(since: u64, job: impl FnOnce() -> T) -> T {
    watch(Watch { since, behind: true }, job)
}

/// B-780: the threads frames made [`behind`] are drawn on, as many as the machine's and below
/// normal priority, so a frame asked for by hand, drawn on the usual threads, is never queued
/// behind them while it waits for them to stop.
pub fn pool() -> &'static rayon::ThreadPool {
    static POOL: std::sync::OnceLock<rayon::ThreadPool> = std::sync::OnceLock::new();
    POOL.get_or_init(|| {
        rayon::ThreadPoolBuilder::new()
            .thread_name(|i| format!("frames made ahead {i}"))
            .start_handler(|_| {
                #[cfg(windows)]
                // SAFETY: the handle is this thread's own pseudo-handle; a failure leaves the
                // priority as it was.
                unsafe {
                    use windows::Win32::System::Threading::{
                        GetCurrentThread, SetThreadPriority, THREAD_PRIORITY_BELOW_NORMAL,
                    };
                    let _ = SetThreadPriority(GetCurrentThread(), THREAD_PRIORITY_BELOW_NORMAL);
                }
            })
            .build()
            .expect("start the threads that make frames ahead")
    })
}

fn watch<T>(w: Watch, job: impl FnOnce() -> T) -> T {
    let before = SINCE.with(|s| s.replace(Some(w)));
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
pub fn tile_watch() -> Option<Watch> {
    if !TILES.with(|t| t.get()) {
        return None;
    }
    count_down();
    SINCE.with(|s| s.get())
}

/// True when a stop came after `since`, from [`tile_watch`].
pub fn passed(since: Option<Watch>) -> bool {
    since.is_some_and(|w| {
        w.since
            != if w.behind {
                HAND.load(Ordering::SeqCst)
            } else {
                now()
            }
    })
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
        // B-780, in the same test since the counts are the process's.
        super::behind(super::hand(), || {
            assert!(!super::asked());
            super::watching(super::now(), || {
                super::by_hand();
                assert!(!super::asked(), "a frame asked for by hand is not stopped by another");
            });
            assert!(super::asked(), "a frame made behind gives way to the hand");
        });
        super::behind(super::hand(), || {
            super::stop();
            assert!(super::asked(), "and to a stop");
        });
        // B-787: a frame chosen before the hand's ask and started after it gives way as well.
        let chosen = super::hand();
        super::by_hand();
        super::behind(chosen, || assert!(super::asked(), "chosen before the hand, started after"));
        assert!(!super::asked());
    }
}
