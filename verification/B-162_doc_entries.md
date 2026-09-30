# B-162 / B-163 / D-233: text for documents 00, 14 and 15

These are the entries this worktree does not write into the documents itself. The coordinator folds them in when merging the branch.

## Document 14, a new decision after D-232

D-233 / Serve only the newest viewer request (G16) and refine when idle (G9) / **ACCEPTED on 2026-09-29 under the owner's delegation: "let's add all of these as well into queue, but defer if not to our standard/to your discretion."** Both are bit-exact, so they fall inside that delegation. Every final frame is byte for byte the frame the same request made before. Draft pixels are unchanged (D-99). Export is not touched.

**What it is.**

- **G16.** The viewer keeps one frame request out and only the newest one waiting behind it. P-04 built that; it is kept.
  - The frame in flight is always finished. The window answers frame requests one after another on its own thread, so word of a newer request cannot reach a render in progress, and the renderer has no point at which to stop. Cancelling part way is not cheap, so it is not done.
  - What is new: a Full picture that comes back while the hand is moving, with a newer frame already waiting, is not drawn. Every other answer is drawn, a Draft one included, so the picture keeps up during a drag rather than freezing until the hand stops.
- **G9, "Refine when idle".** A viewer choice beside the Full/Draft button, kept with the window's preferences, on by default, and doing nothing at Draft.
  - At Full, a frame asked for by number while the hand is moving goes out at Draft. "Moving" means within 200 ms (`SETTLE_MS`) of:
    - a seek: a playhead drag, a ruler click or an arrow key;
    - a drag of a layer or mask;
    - a live change of a value.
  - 200 ms after the hand stops, the same frame is asked for at Full and replaces the stand-in.
  - While a stand-in is on screen, the viewer shows "sharpening…" and its quality label says Draft.
  - The Full/Draft choice and the button keep their meaning.
  - Playback is not changed.
  - Layer outlines are now asked for at the quality of the picture they are drawn over.

**Evidence.**

- `verification/B-162_latest_request_table.md`, 17 of 17.
- `verification/B-163_refine_on_stop_table.md`, 21 of 21.
- Both are from `latest_request_and_refine` in `app/src/main.rs`. It runs the page's own decision lines under Node on a pretend clock, then asks the real request handler for the requests that came out.

**Not done:** cancelling a render part way, for the reason above. It would need the frame address answered off the window's thread, and stopping points in the renderer.

## Document 15, a new backlog entry after B-161

B-162 / B-163 / Serve only the newest viewer request (G16) and refine when idle (G9), D-233. **BUILT on 2026-09-29.**

- **The page.** `app/ui/index.html` has the following:
  - `askedAs` and `drawnWhenBack` between the `B-162 / B-163` markers;
  - `stir`, `restless` and `sharpen` beside them;
  - `show` using them;
  - the Refine when idle tick and the "sharpening…" note in the viewer's row.
- **The CONTROLS pin** gains `refine` (79).
- **The tables.**
  - `verification/B-162_latest_request_table.md`, 17 of 17;
  - `verification/B-163_refine_on_stop_table.md`, 21 of 21.
- **The playtest sheet** `verification/B-162_B-163_playtest.md`, **awaiting the owner's playtest**.

## Document 00, the running log

2026-09-29: B-162 and B-163 built (D-233, owner-delegated, bit-exact).

- The viewer makes only the newest frame, and does not draw a sharp picture of a place already left.
- At Full, with the new Refine when idle tick, it shows Draft while the hand moves and swaps in the exact Full picture 0.2 s after it stops.
- Tables 17/17 and 21/21. The playtest awaits the owner.
