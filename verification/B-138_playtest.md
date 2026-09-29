# B-138: Median and Smart Blur, by hand

Built on 2026-09-29 against D-203, which you accepted with the rest of the After Effects picks
("take everything"). They do what After Effects' Median and Smart Blur do, by rules of our own.
**Median** gives each pixel the middle colour of the pixels round it, so specks, dust and
pinholes go while edges stay; with **Operate on Alpha** on it fills holes too. **Smart Blur**
mixes each pixel only with the pixels round it whose colour is close to its own, so grain and
flat paint are smoothed while lines stay sharp; **Threshold** says how close. Neither grows the
layer. Both sit in the **Blur & Sharpen** group after Diffusion, with After Effects' own names
for their settings.

The generated halves are `verification/B-138_median_smart_blur_table.md`, 111 of 111 checks
passing, which renders every FX-MEDIAN and FX-SMART case against the numbers written before the
code and draws the pictures below, and `verification/B-12b_state_fields_table.md`, which checks
each card sends every setting the command reads. This sheet covers what the tables cannot: how
they look and feel in the window.

## The pictures

`verification/B-138 pictures/`, three times enlarged, over a grey check where nothing is drawn:

- `before.png`: a face as a scan gives it: grainy skin, a dark line round it, two eyes, a thin
  strand of hair, and sixty specks of white, of dark and of nothing (pinholes).
- `median_2.png`: Median at its start, radius 2: the white and dark specks gone and the skin
  calmer; the line and eyes kept; the thin hair gone too; the pinholes still there.
- `median_2_alpha_on.png`: the same with Operate on Alpha on: the pinholes filled as well.
- `smart_blur_64.png`: Smart Blur at its start, radius 3 and threshold 64: the grain smoothed
  away, the line, eyes and hair exactly as they were. The specks stay: they are too far from
  the skin's colour to be mixed.
- `smart_blur_255.png`: threshold 255: everything that shows is mixed, a plain blur inside the
  face, the line gone soft; nothing spills outside the face.

## Before you start

Make the composition 120 by 80 and import `face.png` from `verification/B-138 pictures/`.
Press **Full resolution**.

## What to check

1. **Adding Median.** On the face, pick **Median** in **Add effect…**, under **Blur & Sharpen**,
   after Diffusion; typing "median", "specks", "dust" or "despeckle" in the search finds it
   too. The card shows **Radius** 2 and **Operate on Alpha** Off, and the picture is as
   `median_2.png` at once.
2. **Radius.** Drag Radius slowly up: the eyes shrink, then vanish, and by 10 the face is plain
   skin, its line gone too. Drag it down below 1: the face untouched.
3. **Operate on Alpha.** Set it to On at radius 2: as `median_2_alpha_on.png`, the pinholes
   filled.
4. **Adding Smart Blur.** Remove Median and add **Smart Blur**, also under **Blur & Sharpen**;
   "smart", "skin", "smooth" or "surface blur" finds it. The card shows **Radius** 3 and
   **Threshold** 64, and the picture is as `smart_blur_64.png`.
5. **Threshold.** Drag Threshold down to 0: the face untouched. Up to 255: as
   `smart_blur_255.png`, the line blurred into the skin.
6. **Keyed.** Key Median's Radius from 0 at the first frame to 6 at a later one and play: the
   specks melt away. The same with Smart Blur's Threshold from 0 to 64: the grain melts away.
7. **Moved.** Move the layer to the right: the cleaned face moves with it; nothing is left
   behind.
8. **Out of range.** Type 11 in Radius, or 256 in Threshold: it is refused with a sentence
   saying what it runs to, and the card keeps its old number.
9. **Draft.** Press **Draft**: the picture is smaller and cleaned the same way.
10. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: the settings
    and their keys are still there.

## Known limits, on purpose

- Radius stops at 10 pixels, for speed.
- Median takes away lines thinner than about its radius, as the hair in `median_2.png`, and
  shapes smaller than about its radius, as the eyes at a large radius, and rounds corners. That
  is what a median does; keep the radius small on line art.
- After Effects' Smart Blur has an Edge Only and an Overlay Edge mode; ours has not. Find
  Edges draws edges.
- Smart Blur's mix is flat, every close pixel counting the same, not weighted by distance.
- It is a picture effect: there is no card version yet; that comes later as its own unit.
- They are modelled on After Effects' Median and Smart Blur and are not claimed to match them.
  After Effects' Smart Blur threshold runs 0 to 100; ours runs 0 to 255, in the 8-bit steps of
  the colour, and starts at 64, about After Effects' 25.
- The cost on the reference shot's frames 100 and 101 at full size, the effect on all four
  layers: 49 to 55 ms without it; Median at radius 2, 115 to 122 ms; Median at radius 10 with
  Operate on Alpha on, 674 to 695 ms, the dearest setting; Smart Blur at radius 3 and threshold
  64, 91 to 97 ms; Smart Blur at radius 10, 252 to 275 ms. Median skips flat paint, which is
  its own median, so a flat-coloured drawing costs less than this shot.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
