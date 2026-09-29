# B-146: Radial Shadow, by hand

Built on 2026-09-29 against D-211, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Radial Shadow does, by a rule of our own: it
lays a shadow of the drawing behind it, cast from a lamp at a point you place, so the shadow
spreads away from the light and grows the farther the wall is, as under a street lamp or in
front of a torch. Glass Edge colours the shadow with the drawing's own colours, as light through
stained glass. The layer grows to hold the shadow. It sits in the **Stylize** group after Drop
Shadow.

The generated halves are `verification/B-146_radial_shadow_table.md`, 126 of 126 checks passing,
which renders every FX-RSHADOW case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card sends
every setting the command reads. This sheet covers what the tables cannot: how it looks in the
window.

## The pictures

`verification/B-146 pictures/`, three times enlarged:

- `drawing.png`: the drawing at its own size, 160 by 100: a round head of skin with a dark line
  round it, purple hair and a blush, on nothing; `before.png` is the same enlarged.
- `starts.png`: as it starts, the light at the top of the middle and a tenth bigger: a thin
  half-black shadow peeps out below the chin, and nothing above the hair.
- `corner.png`: the light at the top-left, Projection Distance 40: the shadow thrown down and to
  the right.
- `middle.png`: the light in the middle, distance 50: a dark ring all round the head.
- `soft.png`: `corner.png` with Softening 12: the same shadow with a blurred edge.
- `blue.png`: `corner.png` in blue #2040a0 at Opacity 100: a solid blue shadow.
- `glass.png`: `corner.png` with Glass Edge at Opacity 100: the shadow in the head's own
  colours, skin, hair and line, bigger and behind. `glass_half.png`: Color Influence 50, the
  same colours half mixed with black.
- `shadow_only.png`: `corner.png` with Shadow Only On: the head is gone and only its shadow is
  left.
- `low_right.png`: the light low and off to the right, distance 60: the shadow thrown up and to
  the left.

## Before you start

Make the composition 160 by 100 and import `drawing.png` from `verification/B-146 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the drawing, pick **Radial Shadow** in **Add effect…**, under **Stylize**,
   after Drop Shadow; typing "radial", "lamp" or "torch" in the search finds it
   too. The card shows Shadow Color black, Opacity 50, Light Source 50, 0, Projection Distance
   10, Softening 0, Render Regular, Color Influence 100 and Shadow Only Off, and the picture is
   as `starts.png`.
2. **Light.** Light Source 0, 0 and Projection Distance 40: as `corner.png`. Drag the light
   about: the shadow always falls away from it. 50, 50 with distance 50: as `middle.png`. 110,
   80 with distance 60: as `low_right.png`. Back to 0, 0 and 40.
3. **Distance.** Projection Distance 0: the shadow hides under the drawing. Raise it slowly: the
   shadow grows and slides away from the light. Back to 40.
4. **Softening.** Softening 12: as `soft.png`. Back to 0.
5. **Colour and opacity.** Shadow Color #2040a0 and Opacity 100: as `blue.png`. Opacity 0: no
   shadow. Back to black and 50.
6. **Glass Edge.** Opacity 100, Render Glass Edge: as `glass.png`. Color Influence 50: as
   `glass_half.png`. Back to Regular, 100 and Opacity 50.
7. **Shadow Only.** Shadow Only On: as `shadow_only.png`. Back to Off.
8. **Keys.** Key Projection Distance from 0 at the first frame to 60 at frame 24 and play: the
   shadow slides out from behind the head smoothly. Key Light Source too and play: it swings.
9. **Moved.** Move the layer: the shadow moves with the drawing, the light with it.
10. **Out of range.** Type 101 in Opacity, 1001 in Projection Distance or 1001 in Light Source:
    it is refused with a sentence saying what it runs to, and the card keeps its old number.
11. **Draft.** With Softening 12, press **Draft**: the picture is smaller and looks the same.
12. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- After Effects' **Resize Layer** switch is left out: the layer always grows, but by at most its
  own width across and its own height down on each side, so a shadow thrown farther is cut
  there.
- It runs on the processor; a graphics card version is its own later unit.
- It is modelled on After Effects' Radial Shadow and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 50 to 53 ms without it; as it starts, 102 to 107 ms; the light at the top-left and
  distance 40, 150 to 165 ms; with Softening 12, 250 to 278 ms; with Glass Edge, 158 to 170 ms.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
