# P-18: reading the next frame's drawings ahead

Asked for by the owner on 2026-09-25 ("proceed with option 1 and 2"), the second of the two. **No pixel changes.**

## What changed

While playback is running, the viewer asks for one frame at a time. Until now, the first time a frame came up it had to read its drawings from disk before it could draw anything. Now, as soon as one frame is drawn, the drawings for the next frame are read on a separate thread. That happens while the first frame is still being encoded, sent to the page and put on screen. When the next request arrives, its drawings are usually already in memory.

A read-ahead that isn't finished in time just means the next request waits for it rather than reading the same files twice. The read-ahead fills the same list the frame would have filled itself, so what the frame looks like cannot depend on whether it got there first.

A scrub or a step to one frame does not read ahead. Only the playback clock does.

## How to judge it

The table is in `P-18_read_ahead_table.md`, written by `tests/p18_read_ahead.rs`. It plays each workload once through its work area in real time at 24 frames a second, starting each pass with an empty cache, which is the first playthrough after opening a project. The column to read is **Frames dropped**: frames the clock skipped because the one before was not ready in time.

- Machine: AMD Ryzen 9 9900X, 12 cores and 24 threads, Windows 11 Education 10.0.26200
- Build: rustc 1.89.0, release profile (`opt-level=3`), all threads, Draft quality
- **Gap** is how long the page takes between receiving one frame and asking for the next. Nobody has measured that, so it is an assumption, and there are two of it: 0 ms, the hardest case for reading ahead, and 8 ms, the average wait for a 60 Hz screen.

| Workload | Gap | Dropped, off | Dropped, on | Wait p95, off | Wait p95, on |
|---|---|---|---|---|---|
| Reference shot, 4 layers | 0 ms | 0 and 0 | 0 and 0 | 15.6 and 17.9 ms | 13.7 and 14.6 ms |
| Reference shot, 4 layers | 8 ms | 0 and 0 | 0 and 0 | 20.8 and 19.1 ms | 15.8 and 15.5 ms |
| Ten-layer fixture | 0 ms | 0 and 1 | 0 and 0 | 52.8 and 54.7 ms | 39.4 and 39.3 ms |
| Ten-layer fixture | 8 ms | **31 and 25** | **0 and 0** | 54.5 and 52.7 ms | 35.1 and 39.9 ms |

Each cell gives both rounds. "Wait p95" means 19 of every 20 frames were ready within that time. A frame is due every 41.7 ms.

The reference shot was already keeping up. On the ten-layer fixture, the first playthrough stops dropping frames.

**Checked, on any machine:** every frame shown with reading ahead on, 1727 on the reference shot and 376 on the ten-layer fixture, is byte-identical to the same frame rendered with no cache at all.

## A correction

Earlier, the agent told the owner the ten-layer fixture's first playthrough ran "6x over" its frame budget. That was wrong. P-03(b) measured about 1.2x the budget (51 ms against 41.7 ms), and about 47 ms after the same day's parallel colour conversion.

## Limits

- The drawings read ahead are held outside D-40's gibibyte budget until the frame arrives. That is at most one frame's drawings. They are dropped as soon as a different frame is asked for.
- The read-ahead holds the cache while it reads, so a request for the same frame arriving meanwhile waits. The Wait columns include that waiting.

## How to try it in the app

Open the ten-layer fixture, or your heaviest project, and press play straight away from the start. Watch the dropped-frames report on the first pass.
