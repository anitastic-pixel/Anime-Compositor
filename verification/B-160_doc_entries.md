# B-160: entries for documents 00, 14, 15, 21 and 28

Written by G7 for the coordinator to fold in. Documents 00, 14 and 15 were not edited in the worktree. Every number comes from `verification/B-160_faster_export_table.md`.

## Document 14 (decisions)

D-231 / An export draws several frames at once / **ACCEPTED on 2026-09-29, within the discretion the owner gave** ("sounds great! let's do 1 through 7 to your discretion."), because it is bit-exact: every file and every report is unchanged. **What it does.** An export used to draw one frame, write it and move on to the next. Now it draws a batch side by side across the machine's threads, then writes the batch strictly in frame order. The number of frames drawn at once is capped by memory: whatever fits in 2 GiB of 16-byte pixels, four buffers per frame, and never more than the drawing threads. That is 15 at 1920x1080 and 3 at 3840x2160 on the D-01 machine. PNG and EXR files are compressed inside the batch too. Each frame keeps its own notes, and they are replayed into the export's report in frame order, so the report says exactly what it said before. A stop is noticed before each frame is written, so a stopped export still holds only whole frames, numbered from the start with no gap. The pre-export block check runs in batches the same way. **Evidence.** `tests/b160_faster_export.rs` hashed every file and every report written by the reference shot (PNG 8-bit, PNG 16-bit, EXR half and float, GIF, animated PNG, MP4) and by all 2296 fixture projects (PNG and EXR half, every composition). It did this with the build before B-160 and with this one: 0 of 28684 changed. The MP4 is compared with its creation and modification dates set aside, because Windows stamps those into every MP4 it writes, so the old build's own MP4 differs from run to run. The same table shows 5 at once and the automatic number giving the same bytes as one at a time, for every format. **Speed (PROVISIONAL, median of 7, D-01 machine, release build, other builds running at the same time).** For the whole reference shot, 240 frames at 1920x1080: PNG 8-bit went from 79980 ms to 12291 ms, and MP4 from 20946 ms to 13467 ms. The MP4 is held back by its encoder, which takes frames one at a time. The new build told to draw one frame at a time took 97411 ms, which is slower than the old build's 79980 ms. That gap is put down to the other builds running during that measurement, and should be re-measured on a quiet machine.

D-230 / Hardware video encoding: the graphics card encodes an MP4 / **PROPOSED on 2026-09-29, for the owner to decide.** It changes an MP4's bytes, so it is off unless chosen. **What it does.** Preferences gains a box under a new Export heading, **Hardware video encoding**, not ticked. When it is ticked, an MP4 export asks Windows Media Foundation for a hardware encoder. The profile is Baseline, so the film has no reordered frames, the same as the software film's layout. Every MP4 written with the box ticked carries an `EXPORT_VIDEO_ENCODER` note in its report and in the status line: INFO naming the card's encoder, or WARNING when the software encoder wrote the film instead. That happens when Windows picked a software encoder anyway, when the card's encoder could not be started, or on a system that is not Windows. The change is never silent. With the box off, nothing changes and no note is written. **Measured on the D-01 machine (RTX 4070 Ti Super), whole reference shot, Standard quality.** The encoder used was "NVIDIA H.264 Encoder MFT". The file sizes were 8.1 MB for software and 12.3 MB for the card. Decoded by ffmpeg, card film against software film: PSNR 42.91 dB, and the worst single-channel difference was 53 levels of 255. Against the frames they were made from (over black), software scored 40.58 dB with a worst of 117, and the card 42.27 dB with a worst of 116. The card's film is slightly closer to the source, and larger. **Speed (PROVISIONAL, median of 7):** 15931 ms with the card against 13467 ms for software, so it was not faster here. Handing each frame to the encoder, one at a time, costs the same either way, and other builds were running. A 64x64 film with the box ticked was written by the software encoder, with the WARNING, which is the fallback seen for real. **For the owner:** accept (the box stays, off unless ticked), reject (the box and the note go), or ask for something different. Playtest sheet: `verification/B-160_playtest.md`, steps 4 to 9.

## Document 15 (backlog)

B-160 / Faster export: several frames at once (D-231) and hardware video encoding (D-230, proposed). **BUILT on 2026-09-29 (G7).**
- The batched draw, the in-order write, the memory cap and the per-frame notes are in `src/export.rs` and `src/diagnostics.rs`. The in-memory PNG and EXR encoders are in `src/png_out.rs` and `src/exr_io.rs`. The card encoder is in `src/mp4_out.rs`.
- In the window there is the Preferences box **Hardware video encoding**, in `app/ui/index.html`, and it is passed to the export as `hardware=on` for an MP4 only (`app/src/main.rs`).
- `tests/b160_faster_export.rs` writes `verification/B-160_faster_export_table.md`: 22 of 22, 0 of 28684 files and reports changed, and the D-230 measurements and PROVISIONAL timings.
- The playtest sheet is `verification/B-160_playtest.md`. **D-230 awaits the owner's decision, and the playtest awaits the owner.**

## Document 00 (status line)

B-160, faster export, is built on 2026-09-29. Exports draw several frames at once with every file byte-identical (D-231, accepted under delegation; the reference shot's PNG sequence went from 80.0 s to 12.3 s, PROVISIONAL). Hardware video encoding is a Preferences box, off by default, and awaits the owner's decision (D-230, proposed). The playtest `verification/B-160_playtest.md` awaits the owner.

## Document 21 (rendering math), under export

An export may draw several frames at the same time. Each frame is drawn from the project alone, as it always was, so drawing frames together changes no pixel. Files are written, and notes added to the report, in frame order. D-231, B-160.

## Document 28 (diagnostics catalogue), a proposed row

Proposed with D-230. It is not yet in the built catalogue (`in_catalog` is false until D-230 is accepted):

| EXPORT_VIDEO_ENCODER | INFO or WARNING | Hardware video encoding was chosen for an MP4 | INFO names the graphics card's encoder; WARNING says the software encoder wrote the film instead, and why: Windows chose software, the card's encoder failed to start, or the system is not Windows. Turn the choice off in Preferences to stop the note. Never silent. |

If D-230 is rejected, the row, the identifier and its line in `tests/b12b_diagnostic_catalogue.rs` go.

## PLAYTEST_LIST

- B-160 faster export and the Hardware video encoding box: `verification/B-160_playtest.md` (D-230's decision rides on steps 4 to 9).
