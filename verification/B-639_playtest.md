# B-639 playtest: Drop Shadow gains After Effects' Shadow Only checkbox (D-760)

## What to try

1. Add **Drop Shadow** to a picture layer. The panel now has a **Shadow Only** row, set to **Off**.
2. Set **Shadow Only** to **On**. The layer's own picture disappears and only its shadow is left: soft, in the shadow colour, moved by the distance and direction.
3. Change **Opacity**, **Direction**, **Distance** and **Softness**. The shadow changes exactly as it did with Shadow Only off. Only the picture on top is gone.
4. Set **Opacity** to 0 with Shadow Only on. The layer shows nothing at all.
5. Set **Shadow Only** back to **Off**. The picture is back, with its shadow behind it, the same as before.
6. Save, close and reopen the project. Shadow Only is still on.
7. Open a project saved before this build that has a Drop Shadow. It opens with Shadow Only off and looks exactly as before. Saving it writes the same file.

Before this build there was no such control. A file that said `shadow_only` was opened, the word was ignored without a warning, and the picture was drawn with the layer on top.

## What the check found

Fixture check `tests/b639_drop_shadow_only.rs`: 61 of 61 (see `B-639_drop_shadow_only_table.md`). FX-SHADOW-024 to 030 are pixel-compared with `tools/drop_shadow_reference.py` (tolerance 2e-5; the largest difference is 1.9e-7). FX-SHADOW-001 to 023 did not change. Their files and pictures are the same, and `tests/b58_drop_shadow.rs` still passes on all thirty.

Other checks:
- An old file (`fx_shadow_001.json`, no Shadow Only) opens as off. Its picture is byte-identical to `fx_shadow_026.json`, which says "off". Saved again, it still has no Shadow Only, so it is unchanged.
- Files that say "on", "off", "On" and "yes" all read and write back unchanged.
- "On" (the panel's name, not the stored word) and "yes" are refused with a sentence, and nothing changes. A file with either word keeps the effect and leaves it out of the picture, with the warning `EFFECT_PARAMETER_INVALID`.
- Turning Shadow Only on in `fx_shadow_001.json` draws `fx_shadow_024.json`'s picture exactly, and one undo gives back the picture it had.
- The graphics card against the processor: all 30 fixtures, at Full and Draft, are 0 levels of 255 apart. Shadow Only is drawn on the card, not sent back to the processor.
- `tests/b65_gpu_fx.rs` (the card's check, which walks every Drop Shadow fixture) passes: 2810 of 2810.
- The app's tests: typing Shadow Only on in the panel reaches the effect; "yes" is refused; it saves and reopens as on; a send without it (an older page's) is off. The panel-contract checks pass, including that the panel's `shadow_only` is a setting the command reads.

## Against the tutorial (Jake In Motion, "Drop Shadow | Effects of After Effects", 0:44 and 0:47)

Both stills have the same settings: colour 004728, Opacity 69%, Direction 111, Distance 21, Softness 36. At 0:44 Shadow Only is off, and at 0:47 it is ticked. Ours uses the stand-in face cut from the video (B-58's), at the same settings, drawn by the release app.

| Picture | Yellow drawing pixels | What is seen | Verdict |
| --- | --- | --- | --- |
| After Effects, off (0:44) | 37276 | yellow face over a soft dark-green shadow | |
| After Effects, on (0:47) | 23 (the anchor icon) | the face gone, only the soft dark-green shadow | |
| Ours, off | 45582 | yellow face over a soft dark-green shadow | matches |
| Ours, on | 0 | the face gone, only the soft dark-green shadow | matches |

The card and the processor draw the "on" picture at most 1 level of 255 apart.

## Pictures (`B-639 pictures/`)

- `shadow_only_sheet.png`: After Effects above, ours below; off on the left, on on the right. In both programs ticking Shadow Only removes the yellow face and leaves the same soft green shadow.
- `ours_off.png`, `ours_on.png`: our two frames, full size.

After Effects' view is panned at 0:44 and 0:47, so its face sits higher and further left than ours. Our stand-in face was cut from a frame of the video, so its lower strokes are faint and their shadow is lighter than After Effects'. Neither is a difference in the effect.

## How this differs from After Effects

- Nothing known. Shadow Only leaves the layer out and keeps the shadow as it was, which is what After Effects' stills show.
