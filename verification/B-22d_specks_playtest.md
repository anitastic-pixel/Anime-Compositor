# B-22d: the specks in a dithered GIF, by hand

Built on 2026-09-19 against D-73a, which the owner accepted the same day ("accept D-73a, proceed
with B-22d"). The change is two limits inside the dithering and nothing in the window.

The generated half: `verification/B-22b_choices_table.md`, 20 of 20, now checks the dithering
against FX-FMT-053 to 055 to the pixel. FX-FMT-055 is the specks in small: a field of dull red
with only greys and one bright red to spend comes out grey, with no red dots.

## To look at

1. Open `verification/B-22d playtest/plain_before_after.png`. Three pieces of the same frame,
   enlarged three times: no dithering on the left; dithering as you played it in B-22c in the
   middle; dithering now on the right. On the right there should be no red dots in the pink
   tree, no yellow in the trunk and no orange in the cloud, and the sky should still be smooth
   where the left one shows bands.
2. Open `verification/B-22 films/dithered.gif` in a browser (made by
   `cargo test --test b22b_choices`, about five minutes): the whole frame, moving, with no
   specks anywhere.
3. From the window: a GIF of a shot of your own with **Dither** ticked, as in step 8 of the
   B-22c sheet. No specks; a fine grain where there were bands.

## Ceilings, stated

- A large area of a colour the 256 have nothing near now comes out as its nearest colour, flat,
  not as that colour with bright dots. On a shot with very many colours that could show as a
  slightly flatter patch.
- The three frames checked are of one shot. A shot of your own is the other half of the trial.
- The dithered file here went from 2,418,879 to 2,320,640 bytes; no document sets a size.

## Result

**Passed on 2026-09-23.** The owner looked at the before/after picture and `dithered.gif`, and
exported the reference shot as a dithered GIF from the window (step 3): "I think it worked well
for a gif", then "yes" to steps 1 and 2.

The agent checked the owner's export (1920x1080, 14 frames, 10,649,332 bytes). The sky is an
even grain with no bands. It then looked for a pixel standing out sharply from a smooth patch
around it: 49 in all 14 frames, none in the first frame, at most 24 in one. Every one is in the
pink tree, where `Fixtures/reference_shot/layer1/layer1_000.png` has darker flecks of its own. The
GIF draws them in the nearest darker colour its palette holds, a browner rose than the painting.
They are not B-22c's red dots. The canopy's main pink comes out flat, as in the first known limit.
