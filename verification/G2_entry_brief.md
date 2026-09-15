# A brief for the owner: what stands between B-12 and B-13

Written on 2026-09-15, the day B-12 closed. The owner asked the agent to proceed with the next
thing it recommended. The agent had recommended performance work, but every performance unit in
document 15 is already done or cut (P-14, the last, on 2026-09-12). The next unit is B-13, the
flat-plane camera and parenting, and it cannot start on the agent's say-so, for three reasons
this page sets out. Nothing here is a defect report. Decisions belong in
`Markdown/14_Decisions_Risks.md`, which is the owner's to write.

---

## 1. The G1 gate is the owner's to call

Document 04 line 13: G2 "requires G1 passing and measured memory headroom". Document 00 line 55:
"An agent must not open a gate on its own reading of the evidence, and that rule still stands for
G1 and G2." Document 11 line 46 says what passing G1 means: every applicable test from T-01 to
T-10, plus T-15 and T-16, W-01 completed, and no known reproducible project-corruption defect.

| What G1 asks | Where the evidence is | Standing |
|---|---|---|
| T-01 import | `verification/B-03_import_table.md` | Done |
| T-02 exposure holds | `verification/B-04_exposure_table.md`, `verification/B-04b_frame_log_table.md` | Done |
| T-03 transforms, order, undo | `verification/B-05_model_table.md`, `verification/B-05a_transform_table.md` | Done |
| T-04 alpha and mattes | `verification/B-05c_blend_table.md`, `verification/B-06_mask_table.md` | Done |
| T-05 effects | `verification/B-07_effects_table.md` | Done |
| T-06 playback, seek, memory | `verification/T-06_performance_envelope.md`, `verification/T-06_declared_fixture.md` | Done, **with D-49 open** (below) |
| T-07 saving and recovery | `verification/B-09_*`, `verification/T-07e_roundtrip_table.md` | Done |
| T-08 export | `verification/B-10_export_table.md`, `verification/T-08_export_table.md` | Done |
| T-09 colour and alpha | `verification/B-02_fixture_table.md` | Done |
| T-10 offline | `verification/B-11_offline_table.md`, `verification/B-11_offline_run.md` | **Half done**, with D-39 open (below) |
| T-15 keyboard, focus, scaling | `verification/B-11_display_and_keyboard.md`, `verification/B-12c_keyboard_table.md` | Done |
| T-16 licence record | `verification/B-11_record_table.md` | **Half done** (below) |
| W-01 completed | `verification/B-12_run_notes.md` | Done, by the owner, 2026-09-15 |
| No known corruption defect | Document 14 has no open entry about project corruption | None known |

No table under `verification/` has a failing row.

The three that are not clean:

- **T-10.** The build is checked by reading it, and the running program was watched. Nobody has
  run the shot with the network adapter actually turned off. D-39 is open as well: Microsoft's web
  view, which draws the window, opens connections of its own that this project cannot stop.
- **T-16.** The dependency and licence record is generated and checked on every build. The half
  that needs a person, a legal review of that record, has not happened.
- **T-06 and D-49.** The published envelope was measured with a cache set up differently from the
  one the viewer now builds. D-49 recommends re-recording it. The build is faster than the
  published numbers, not slower.

**The choice.** Pass G1 as it stands, writing down the three gaps as accepted. Or pass it once
the gaps are closed. The adapter-off run is about ten minutes of the owner's time. The D-49
re-record is agent work the owner asks for. The legal review needs someone qualified.

## 2. Memory headroom has a number, and whether it is enough is a judgement

`verification/T-06_performance_envelope.md` measured the process on the reference shot. It held
1019.5 MiB after the second loop and 1021.8 MiB after the tenth, growing 2.3 MiB over 1,920
renders. The peak working set was 1180.1 MiB. The reference machine in D-01 has 64 GB, so that
peak is under 2 percent of it. The default caches are 1 GiB together (576 MiB of drawings, 448 MiB
of effect results). A camera shot adds planes rather than new kinds of memory: each flat plane is
a drawing held like any other layer's.

**The choice.** Accept this as the headroom document 04 asks for, or ask for a measurement on a
heavier shot first.

## 3. B-13 has no specification yet, and some of it is the owner's taste

Document 21 line 101: "G2 camera projection, intersecting transparent planes ... are separate
contracts. They must not be implied by this G1 specification." Document 08 line 47 names what must
be written before any 2.5D code: "camera coordinates, transform order, depth ordering and shutter
sampling". Document 11's T-11 needs fixtures whose expected values are worked out independently.
The `Fixtures/` rule makes those a specification step, not a coding step. None of this exists yet.

What the agent proposes, each with its reason. The owner may take, change or cut any of them.

1. **Parenting first, as its own unit.** Parenting is useful in today's 2D shots, with no camera:
   a character's mouth cel riding on its head cel. It is the smaller half, and it reuses document
   21's transform unchanged. A child's final transform is its parent's transform applied after its
   own: `M_child_in_comp = M_parent * M_child`, chained up to a layer with no parent. It inherits
   position, rotation, scale and anchor, as After Effects does. Opacity, effects, mattes and
   exposures are not inherited. A parent loop is refused, the way a matte loop is. When a parent
   is set, the child keeps where it is on screen, as After Effects does by default.
2. **Planes stay facing the camera in the first camera unit.** Each layer gains a depth (z). The
   camera has a position and a zoom, and does not tilt or turn. A plane parallel to the screen,
   seen by a camera that does not turn, only ever gets larger or smaller and moves. So no two
   planes can intersect (W-04 already rules intersection out), the existing sampler draws every
   plane, and every expected value in a fixture can be worked out by hand. Tilting planes and
   turning the camera become a later unit, if a real shot needs them.
3. **After Effects' camera convention.** A plane at depth 0 is drawn at its own size when the
   camera sits at the default distance, called zoom. Something twice as far away looks half as
   big. Farther planes draw first, and planes at equal depth keep their order in the layer stack.
4. **No shutter.** The camera is read at the frame's time, with no motion blur. It moves smoothly
   between keys while each cel still holds its drawing, which is W-04's "cel exposure timing
   independently of camera sampling".
5. **The reference parallax shot is generated, like the reference shot was.** A far background,
   a middle plane and a foreground cel on twos, with the camera tracking sideways over 48 frames.
   Expected positions and sizes are worked out from the formula, not from the renderer.

The order this gives: **B-13a** writes the contracts into documents 19, 20, 21, 07 and 25, and the
owner accepts them. **B-13b** is parenting. **B-13c** is the camera and depth, then W-04 walked by
the owner.

## What the agent can do without any of these decisions

Nothing on B-13 itself, because every step depends on section 3. The D-49 re-record and a written
sheet for the adapter-off offline run are both ready to do the moment the owner asks.
