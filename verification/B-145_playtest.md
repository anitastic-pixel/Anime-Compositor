# B-145: Optics Compensation, by hand

Built on 2026-09-29 against D-210, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Optics Compensation does, by a rule of our own:
it bends the layer as a wide lens would, the middle keeping its size and the picture squeezed in
more and more toward the edges, so straight lines bow outward round the middle, a fisheye for a
peephole, a security camera, a dream or a big impact. With Reverse Lens Distortion on it does the
opposite and stretches the edges out, which takes a fisheye out again. The layer does not grow.
It sits in the **Distort** group after Polar Coordinates.

The generated halves are `verification/B-145_optics_compensation_table.md`, 100 of 100 checks
passing, which renders every FX-OPTICS case against the numbers written before the code and
draws the pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card
sends every setting the command reads. This sheet covers what the tables cannot: how it looks in
the window.

## The pictures

`verification/B-145 pictures/`, three times enlarged:

- `drawing.png`: the drawing at its own size, a cream card ruled every ten pixels, a blue frame
  round it and a red ball in the middle; `before.png` is the same enlarged.
- `fov_90.png`: Field of View 90: a fisheye. The ball keeps its size, the ruled lines bow
  outward, and the left and right edges are empty from top to bottom, as they read from past the
  card.
- `fov_60.png`: Field of View 60: the same, gentler.
- `fov_180.png`: Field of View 180, the most: only a round patch shows, everything farther than
  80 pixels from the middle empty.
- `reverse.png`: Field of View 90 with Reverse Lens Distortion on: the edges stretched out, the
  lines bowed inward, the blue frame pushed off the sides, nothing empty.
- `vertical.png`: FOV Orientation Vertical: the 90 degrees across the height, so it bends harder
  and more is empty than in `fov_90.png`.
- `diagonal.png`: FOV Orientation Diagonal: across the diagonal, gentler, less empty.
- `centre_25.png`: View Center 25, 50: the lens's middle a quarter of the way across.
- `in_then_out.png`: Field of View 60 put in, then a second Optics Compensation at 60 with
  Reverse on: back to the drawing, a little softer.

## Before you start

Make the composition 160 by 100 and import `drawing.png` from `verification/B-145 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the drawing, pick **Optics Compensation** in **Add effect…**, under
   **Distort**, after Polar Coordinates; typing "optics", "lens", "fisheye" or "barrel" in the
   search finds it too. The card shows Field of View 0, Reverse Lens Distortion Off, FOV
   Orientation Horizontal and View Center 50, 50, and the picture does not change.
2. **Fisheye.** Field of View 90: as `fov_90.png`. 60: as `fov_60.png`. 180: as `fov_180.png`.
   Back to 90.
3. **Reverse.** Reverse Lens Distortion On: as `reverse.png`. Back to Off.
4. **Orientation.** FOV Orientation Vertical: as `vertical.png`. Diagonal: as `diagonal.png`.
   Back to Horizontal.
5. **Centre.** View Center 25, 50: as `centre_25.png`. Drag the centre about: the bend follows.
   Back to 50, 50.
6. **In and out.** Field of View 60, then add a second Optics Compensation below it at 60 with
   Reverse On: as `in_then_out.png`, the card nearly straight again. Delete the second one.
7. **Keys.** Key Field of View from 0 at the first frame to 120 at frame 24 and play: the lens
   swells in smoothly.
8. **Moved.** Move the layer: the bend moves with the drawing.
9. **Out of range.** Type 181 or -1 in Field of View, or 1001 in View Center: it is refused with
   a sentence saying what it runs to, and the card keeps its old number.
10. **Draft.** Press **Draft**: the picture is smaller and bends the same.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- After Effects' **Optimal Pixels** and **Resize** are left out: the layer keeps its size, so a
  squeezed picture leaves empty space round it and a stretched one loses its edges.
- It runs on the processor; a graphics card version is its own later unit.
- It is modelled on After Effects' Optics Compensation and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 49 to 54 ms without it; Field of View 90, 75 to 80 ms; Reverse On, 79 to 83 ms; Field
  of View 180, 77 to 84 ms. **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release
  build; timed by a throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
