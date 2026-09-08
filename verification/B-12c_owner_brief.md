# A brief for the owner: three things waiting on a decision

Three questions are open that no agent should answer. Two of them were written when mattes and
effects were parked, and both changed meaning when B-06 and B-07 landed on 2026-09-06 without
anybody re-reading them. The third has never been a register entry at all and probably should be.

They are collected here because they are three readings of one measurement:
`verification/T-06_declared_fixture.md`, which is the first run of T-06 against the fixture
document 08 line 41 actually declares — ten raster layers, two alpha mattes, three effects, 1080p,
24 fps, 240 frames.

Nothing here is a defect report. Everything in it is measured, and every measurement it cites is a
file you can open. Decisions belong in `Markdown/14_Decisions_Risks.md`, which is yours to write.

---

## 1. D-41 can be closed

**What was decided.** Document 08 line 41 sets the performance targets against a declared fixture
this build could not assemble, because mattes were B-06 and effects were B-07 and both were parked.
On 2026-09-06 you kept line 41 as written and accepted that the envelope stays a **floor** until
the parks lifted. The entry says what closes it: *"the re-measurement against the real fixture
after B-07, not an edit to line 41."*

**What happened.** B-06 and B-07 both landed. The fixture was built, saved as a project this build
opens (`verification/T-06_declared_fixture.json`), read back through the loader, and measured.
That is `verification/T-06_declared_fixture.md`. It is the re-measurement the entry names, so
**D-41 closes on evidence** and needs nothing from you but the closing note.

**The one sentence worth putting in that note.** The ten sequences are copies of the reference
shot's four, laid down on disk as ten separate sequences. So the fixture is *ten layers of drawings
and four layers of pictures* — anybody looking at a frame of it would see the same art twice. That
is a limitation of what it looks like, not of what it costs: nothing in the render path shares work
between two layers reading different files, so ten sequences are ten decodes. Every figure in that
file is a real ten-layer cost and no figure in it is a picture of a ten-layer shot.

**Evidence:** `verification/T-06_declared_fixture.md`, D-41 in document 14.

---

## 2. D-40 has reopened itself

**What was decided.** On 2026-09-06 you left the preview cache budget at 128 MB and accepted that
scrubbing pays for itself. The reasoning was sound and is not in question: the target document 08
asks about — p95 seek-to-display at or below 100 ms — was met at that budget on the reference shot,
measured at 67.75 ms, and document 08 line 43 declines to assume a machine's spare memory.

**What changed.** The entry ends with its own reopening condition, in your words: *"if B-06 and
B-07 make the reference shot heavy enough that the measured p95 crosses 100 ms, that is a new
measurement and reopens this entry rather than contradicting it."* That measurement now exists, on
the declared fixture, at the default budget, scrubbing:

| | |
| --- | --- |
| p95 of a scattered seek, default budget | **390.40 ms** |
| The target in document 08 | 100 ms |
| Crossed | **yes** |

**So the condition has fired and D-40 is reopened by its own terms** — by a measurement, not by a
disagreement. Your reason for the decision was that the target was met at that budget; on the
declared fixture it is not.

**The arithmetic behind it, which is the useful part.** One cel of this composition is 33,177,600
bytes and a frame of this fixture needs ten of them: **316.4 MiB for one frame**, against a 128 MB
budget. At ten layers the default cannot hold a single frame, so showing a frame evicts the cels
that drew it and the next request re-decodes all ten. That is not the cache misbehaving; it is a
budget smaller than the working set, and no eviction policy fixes it.

**The choices are the three the entry already lists**, unchanged:

| | What it means | What it costs |
| --- | --- | --- |
| Leave the default | scrubbing pays for itself, and on a ten-layer shot it pays a lot | ~390 ms p95 scrubbing at ten layers |
| Raise it to hold a working neighbourhood | the third row of the seek table is what that buys: 210.77 ms p95 at a 332 MB budget, still not 100 ms | memory the machine may not have; document 08 line 43 is the objection |
| Make it a setting with a stated cost | the person decides on their own machine | a setting, its bounds, and words that say what it trades |

**Worth knowing before choosing:** even a budget with room for one whole frame does not reach the
100 ms target — it halves the cost, it does not meet it. Holding every distinct drawing of this
fixture would be 166 cels, about 5.5 GB. The target and the fixture may simply not meet at any
budget, which is question 3.

**Evidence:** `verification/T-06_declared_fixture.md` (the seek table),
`verification/B-08b_cache_budget.md`, D-40 in document 14.

---

## 3. The playback finding: the declared shot is about nine times too slow, and no entry covers it

**This is not a D-number and there is no register entry it belongs to.** D-40 is about the cache
budget; D-41 was about the fixture not existing. Neither is the biggest number on the page.

**What was measured.** A 24 fps clock allows the 240-frame work area 10,000 ms. On the declared
fixture, warm, at draft resolution, **0 of ten loops came in under it**; the tenth loop's median
frame cost **371.08 ms against the 41.7 ms** a frame is allowed, and **240 of its 240 frames** were
over. That is a factor of about nine. A factor is not a margin.

**Read next to the reference shot**, which sits *on* the deadline and flips either side of it
between runs, this says something specific: **nothing here is a regression.** The shot document 08
declares is simply more work than this build does in real time. Two and a half times the layers
costs about nine times the frame, which is more than the layer count alone accounts for and is not
explained by anything measured so far. The obvious suspect is on file —
`verification/D-37_decode_cost.md` puts decoding at 75.15 ms of an 81.69 ms four-cel draft frame —
and the loop table is consistent with it: every loop decodes 2,340 cels and only 480 come from
memory, because ten cels of frame do not fit a 128 MB cache and never will.

**What it costs the person using it is already decided and needs nothing new.** D-32 says the
viewer holds real time and drops the frames it cannot make, so the shot plays at the right speed
and shows fewer frames rather than playing slowly, and it says so on screen.
`verification/B-08_window_shell.md` photographs a real window doing exactly that.

**What is not decided** is whether a 24 fps target for a ten-layer shot is one this project keeps,
lowers, or reaches by doing the decoding differently. That is a register entry somebody has to
open, and opening it is yours.

**The choices.**

| | What it means | What it costs |
| --- | --- | --- |
| Keep the target and schedule work against it | real-time preview becomes a backlog item with a stopwatch on it; ADR-006 gates a GPU path on exactly this kind of reading — *"a measured result on a real shot on the reference machine"* showing the CPU path too slow | a unit of work, and it is decode-bound before it is composite-bound |
| State a preview frame rate below 24 fps | the tool stops missing a target it was never going to meet at ten layers | an amendment to document 08 and an honest sentence about what preview is for |
| Accept it as it stands | dropped frames during preview are normal; the exported file is unaffected either way | nothing, except that it stops being an open finding only once it is written down |

**The recommendation, if you want one.** Run B-12 first — `verification/B-12_acceptance_run.md` is
the sheet — and record how playback felt while you did it. A stopwatch says nine times; a person
says whether that is in the way, and only one of those is the thing the project is for. But note
that B-12 runs on the *reference* shot, which is on the deadline, not nine times past it, so what
the run tells you is the four-layer answer.

**Evidence:** `verification/T-06_declared_fixture.md` (the ten loops),
`verification/T-06_performance_envelope.md` (the reference shot, for contrast),
`verification/D-37_decode_cost.md`, `verification/B-08_window_shell.md`, D-32 in document 14.

---

## What an agent will do next unless told otherwise

Nothing on this page. All three stay as they are — measured, recorded, and cited by the files that
depend on them — until you write into document 14. Closing D-41 and reopening D-40 are both
register edits, and the register is yours.
