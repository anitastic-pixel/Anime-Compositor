# B-137: Mix on every effect, by hand

Built on 2026-09-29 against D-202, which you accepted with the rest of the After Effects picks
("take everything"). Every effect's card has one more row at its foot, **Mix**, 0 to 100 per
cent, 100 when the effect is added: how much of that one effect's result is laid over what it was
given. At 100 the effect is as it always was; at 0 the picture is as if the effect were switched
off; between, the two are laid over each other. It keys like any other setting. It is After
Effects' Effect Opacity in purpose, by a rule of our own.

The generated halves are `verification/B-137_effect_mix_table.md`, 93 of 93 checks passing,
which renders every FX-MIX case against the numbers written before the code, opens 1950 older
project files to show none of them gains a Mix, and draws the pictures below, and
`verification/B-12b_state_fields_table.md`, which checks the Mix the card reads is in what the
window gives it, and every card still sends every setting the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window.

## The pictures

`verification/B-137 pictures/`:

- `town.png`: a street, sky, houses and a road; the drawing to use below.
- `invert_mix.png`: Invert at Mix 0, 25, 50, 75 and 100, left to right: the street, then fading
  to its negative.
- `blur_mix.png`: Gaussian Blur, Radius 6, Edges Repeat Edge Pixels, at Mix 0, 50 and 100: sharp, the
  sharp street laid over the soft one, and soft.
- `invert_fade_frames.png`: frames 0 to 4 of an Invert whose Mix is keyed from 0 to 100: the
  same five steps as `invert_mix.png`, one per frame.

## Before you start

Make the composition 480 by 270 and import `town.png` from `verification/B-137 pictures/`. Press
**Full resolution**.

## What to check

1. **The row.** Add **Invert** to the street. The card has a **Mix** row at its foot, 100%,
   with a key diamond, and the picture is the negative. Add **Posterize Time** too: its card has
   no Mix row. Delete the Posterize Time again.
2. **Half way.** Set Invert's Mix to 50: as the middle of `invert_mix.png`, a washed, lighter
   street, not a flat grey. Drag it slowly down to 0: the negative fades until, at 0, it is the
   street untouched. One Ctrl+Z puts the whole drag back.
3. **Keyed.** Key Mix at 0 on frame 0 and at 100 on frame 4 and play: the street turns into its
   negative over four frames, as `invert_fade_frames.png`. The timeline shows the keys on a row
   named **Invert Mix**.
4. **Only the one.** Set Invert's Mix to 0, with no keys, and add **Gaussian Blur** below it,
   **Radius σ (px)** 6 and **Edges** Repeat Edge Pixels: the street blurred and not inverted. Set the blur's Mix
   to 50: as the middle of `blur_mix.png`, the sharp street over the soft one, a soft glow round
   the houses.
5. **Out of range.** Type 101 or -1 in a Mix: it is refused with a sentence saying Mix is 0 to
   100 per cent, and the card keeps its old number.
6. **Carried.** Copy the Invert, at Mix 50, from its card's right-click menu and paste it on
   another layer: the pasted one is at Mix 50 too. **Save as preset…** and add the preset to a
   layer: Mix 50 again.
7. **Graphics card.** Switch **Draw on** between **CPU** and **GPU**: the street with the blur
   at Mix 50 does not change in any way you can see. Add a **Light Wrap** to a layer above the street and set its Mix to 50:
   on **GPU** the warning panel says the CPU drew the frame, as a Light Wrap with a Mix below 100
   is not drawn on the graphics card yet.
8. **Draft.** Press **Draft**: the picture is smaller and mixed the same way.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every Mix and
   its keys are still there. A project saved before this build opens with every effect at 100,
   looking as it did.

## Known limits, on purpose

- The mix is in light, as every other mix in the program is, so a Mix of 50 looks lighter than
  half way on dark colours.
- Posterize Time has no Mix: it holds the layer in time and has no picture to mix.
- An effect with a Mix below 100 is drawn on the processor, even where the graphics card would
  draw it; it looks the same either way. The card version comes later as its own unit.
- A project saved with a Mix below 100 and opened in an older build shows that effect at full
  strength; the older build keeps the number and writes it back.
- It is modelled on After Effects' Effect Opacity and is not claimed to match it.
- A Mix below 100 costs a few milliseconds a layer, as the picture the effect was given is kept
  to lay back: the reference shot's frames 100 and 101 at full size take 49 to 55 ms with no
  effect; with an Invert on all four layers, 58 to 67 ms at Mix 100 and 79 to 83 ms at Mix 50;
  with a Gaussian Blur of 6, 89 to 95 ms at Mix 100 and 113 to 118 ms at Mix 50.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
