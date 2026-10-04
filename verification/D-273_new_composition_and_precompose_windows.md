# D-273: the New composition and Pre-compose windows from the redesign

Built on 2026-10-03 because the owner reported that Ctrl+N still opened the old form inside the Project panel and that Ctrl+Shift+C pre-composed straight away with no window ("yes, please copy/paste the redesign versions"). The two windows copy the design boards "Composition settings A · Tabs with the frame beside" and "Pre-compose A · Two choices".

## What changed

**New composition (Ctrl+N, or New composition… in the Project panel) and Composition settings (Ctrl+K)** now open one window in the middle of the screen. The old form inside the Project panel is gone.
- At the top is the composition name, with the Basic and Advanced tabs below it.
- **Basic** has the following:
  - a Preset list (HD, Flat 1.85, Scope 2.39, Vertical 9:16, TV 4:3, 4K UHD, plus your own), a button to save your own preset and one to delete it;
  - Width and Height, with "Lock shape to 16:9";
  - Frame rate;
  - Length, with its seconds-and-frames read back in blue.
- **Advanced** holds the motion blur shutter (angle, phase and samples) and the Sheet's episode, scene, cut and animator.
- **THE FRAME**, on the right, redraws at the shape of the size you type, with the title safe, action safe, field guide and centre cross drawn on it. Each of those four has its own tick.
- **Preview** at the foot makes the ticked guides show in the viewer too.
- Below the frame, the shape buttons 16:9, 1.85, 2.39, 9:16 and 4:3 do the same as picking a preset.
- **Enter** makes the composition (Create it) or applies the change (Apply). **Esc**, Leave it and × close the window without changing anything.

**Pre-compose (Ctrl+Shift+C, the Layer menu, the palette or the right-click menu)** now opens a window before anything is made.
- At the top it shows the layer's name, or "N layers", and asks for the new composition's name. It suggests "layer1 comp", or "Precomp 2" for several layers.
- Two cards show before and after: Leave settings in the outside composition, or **Move every setting into the new one**. The second is how pre-compose works today.
- Three ticks follow:
  - Trim it to the layers' length;
  - **Open it after**;
  - **Show the flowchart after**, which opens the Map.
- **Enter** pre-composes. **Esc** or Cancel closes without making anything.
- The two ticks are remembered.
- In the engine, `layer.precompose` now takes the name typed. A blank name still gets the next "Precomp N".

**Greyed and marked PROPOSED.** These rows from the boards need an engine change, so each is drawn but cannot be used:

| Row | Proposal |
|---|---|
| Match a file… (size and rate from a drawing or video) | D-274 |
| Pixel shape, Resolution, Start frame, Background | D-275 |
| Resize from (anchor) | D-276 |
| Keep frame rate when nested | D-277 |
| Pre-compose: Leave settings in the outside composition | D-278 |
| Pre-compose: Trim it to the layers' length | D-279 |

The boards' Composition tree side panel was not copied, because the Map already does that job.

## Window evidence

The test copy was built from this change (`$S/tgt`, release) and driven through real key presses by `cdp3.ps1`, on the reference shot.

| Step | Seen |
|---|---|
| Ctrl+N | The window opened titled "New composition Ctrl+N" with the button "Create it": 1920 x 1080 at 24 fps, 240 frames, preset "HD · 1920 × 1080", read back "Frame 1920 × 1080 · 16:9 (1.78) · 10s 0f at 24 fps". The old form was no longer in the Project panel. |
| Typed the name "Wide shot", picked Scope 2.39 | 2048 x 858. The 2.39 shape button lit, and the frame was drawn 433 x 181 (ratio 2.39). |
| Typed width 4096 with Lock shape on | The height became 1716 and the preset became "Custom · 4096 × 1716". |
| Typed length 96 | The read back showed "= 4s 0f at 24". |
| Advanced tab, typed episode 03 | The Advanced pane showed, Basic was hidden, the shutter showed 180 / -90 / 16 and the frame stayed visible. |
| Enter | The window closed and the new "Wide shot" composition opened at 4096 x 1716, 96 frames, episode 03. |
| Ctrl+K | The window showed "Composition settings Ctrl+K" with "Apply", filled with Wide shot's values. Start frame showed 0, greyed. |
| Typed length 120, Enter | Wide shot became 120 frames, and there were still 2 compositions. |
| Ctrl+K, typed width 640, Esc | The window closed and the width stayed 4096. |
| Save preset, typed "House size", Enter | The name box opened inside the window. The preset list then showed "House size · 4096 × 1716 · 24 fps", Delete was enabled and the window stayed open. |
| Unticked Field guide, ticked Preview | The field guide left the frame. The viewer's guides showed, with the 10F field box hidden and the title safe showing. |
| Delete preset, Leave it | The saved list was empty and the window closed. |
| Chose layer1, Ctrl+Shift+C | The window "Pre-compose layer1 · 1 layer chosen" opened with the name "layer1 comp". Leave settings was greyed, Move every setting was chosen and the range said "(frames 0-239)". Nothing had been made yet. |
| Typed "Hair comp" | The after diagram followed the name. |
| Ticked Open it after, Enter | "Hair comp" was made and opened, holding layer1. Undo read "New composition Hair comp and 2 more". |
| Chose layer2, Ctrl+Shift+C | "Open it after" was remembered as ticked, and the name was "layer2 comp". |
| Ticked Show the flowchart after, Enter | The Map came to the front, and the view stayed on the reference shot. |
| Ctrl+Shift+C, Esc | The window closed and no composition was made (still 4). |
| Page errors | None. |

The app's own checks were rerun: 88 passed, 0 failed, 5 ignored. They include two new checks:
- the name " Hair comp " is saved trimmed, as "Hair comp";
- a blank name gets "Precomp 1".

## Pictures

In `verification/D-273 pictures/`:
- `d1_new_composition.png`: Ctrl+N's window;
- `d2_scope_typed.png`: Scope with the width typed;
- `d3_advanced.png`: the Advanced tab;
- `d4_preset_saved_guides.png`: a saved preset and the guide ticks;
- `d5_precompose.png`: the Pre-compose window;
- `d6_opened_after.png`: Hair comp opened;
- `d7_flowchart_after.png`: the Map after pre-compose.

## Known limit

When the Sheet's four fields are filled in a new composition, Create it is two undo steps: making the composition, then its Sheet details. One Ctrl+Z takes back the Sheet details, and a second takes back the composition.

## Playtest

1. Press Ctrl+N. A window opens in the middle with the frame drawn on the right. Pick Scope 2.39: the frame turns wide. Type a name and press Enter. The new composition opens.
2. Press Ctrl+K. The same window opens, titled Composition settings, with Apply. Change the length and press Enter. The timeline gets longer. Press Ctrl+K again, change something and press Esc. Nothing changes.
3. Select a layer and press Ctrl+Shift+C. A window asks for a name. Type one, tick Open it after and press Enter. The new composition opens with your layer inside.
4. Say whether either window looks or behaves differently from the boards in a way you dislike.
