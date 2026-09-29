# B-140: Kaleidoscope, by hand

Built on 2026-09-29 against D-205, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' CC Kaleida does, by a rule of our own: one wedge
of the layer, from a centre, is repeated all the way round, every other copy flipped (Mirror) or
every copy turned the same way (Repeat). Where the pattern reaches past the layer it reads the
layer mirrored back at its edges, so it never goes clear. It does not grow the layer. It sits in
the **Stylize** group after Solarize.

The generated halves are `verification/B-140_kaleidoscope_table.md`, 95 of 95 checks passing,
which renders every FX-KALEIDO case against the numbers written before the code and draws the
pictures below, and `verification/B-12b_state_fields_table.md`, which checks the card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks in the window.

## The pictures

`verification/B-140 pictures/`, three times enlarged:

- `drawing.png`: the plate at its own size, a sky, a sun, a hill and a kite, nothing symmetric;
  `before.png` is the same enlarged.
- `six_mirror.png`: Kaleidoscope as it starts, 6 segments, Mirror: the wedge from straight up
  through 60 degrees clockwise is the drawing's own, and the six copies meet without a seam.
- `six_repeat.png`: the same wedge, every copy turned the same way, so neighbours meet at a seam.
- `twelve.png`: 12 segments, thinner wedges.
- `rotation_30.png`: rotation 30: the pattern turned, a different part of the drawing in it.
- `size_50.png`: size 50: the pattern half as large, reaching past the drawing and still covered.
- `centre_25.png`: the centre at 25 across, 50 down: the pattern turns round the left of the
  picture.

## Before you start

Make the composition 160 by 100 and import `drawing.png` from `verification/B-140 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding it.** On the drawing, pick **Kaleidoscope** in **Add effect…**, under **Stylize**,
   after Solarize; typing "kaleida", "mirror", "mandala" or "symmetry" in the search finds it
   too. The card shows Center 50, 50, Size 100, Segments 6, a Rotation dial at 0 and Mirroring
   set to Mirror, and the picture is as `six_mirror.png` at once.
2. **Mirroring.** Pick Repeat: the picture is as `six_repeat.png`, with seams where copies meet.
3. **Segments.** Drag Segments from 2 to 32: the wedges get thinner. At 2 under Mirror the left
   half is the right half flipped. 6.9 looks the same as 6.
4. **Rotation.** Turn the dial: the pattern turns and shows other parts of the drawing, as
   `rotation_30.png` at 30.
5. **Size.** Size 50 as `size_50.png`: smaller and busier, never clear. Size 400: one small part
   of the drawing blown up.
6. **Center.** Drag the centre's numbers, or set 25, 50, as `centre_25.png`: the pattern turns
   round the new point.
7. **Keyed.** Key Rotation from 0 at the first frame to 360 at frame 48 and play: the pattern
   turns a full circle smoothly.
8. **Moved.** Move the layer: the pattern moves with it.
9. **Out of range.** Type 1 or 33 in Segments, or 9 in Size: it is refused with a sentence saying
   what it runs to, and the card keeps its old number.
10. **Draft.** Press **Draft**: the picture is smaller and the same pattern.
11. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- The pattern is read by the plain sampling every warp here uses: at a large size, enlarged
  pixels look soft, not sharp.
- Under Repeat the seams between copies are hard, one pixel to the next, as they are in a real
  kaleidoscope of that kind.
- After Effects' CC Kaleida has a Floating Center switch and a Mapping choice of three; ours has
  the centre always where you put it and Mirror or Repeat.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- It is modelled on After Effects' CC Kaleida and is not claimed to match it.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 53 to 62 ms without it; Kaleidoscope as it starts, 91 to 103 ms; Repeat, 92 to 108 ms;
  32 segments at size 10, rotation 45, 111 to 116 ms. **Machine:** AMD Ryzen 9 9900X with 24
  threads; Windows 11; release build; timed by a throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
