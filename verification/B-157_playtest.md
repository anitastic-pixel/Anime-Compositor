# B-157: five effects quicker on the graphics card

Built on 2026-09-30 under your "sounds great! let's do 1 through 7 to your discretion." This is
item 6 of the GPU plan, the double-precision audit, D-228.

The graphics card can add numbers in two precisions. The finer one runs 64 times slower on this
card, and the card's version of each effect used it in many places to match the processor. Every
effect was timed (`verification/B-157_audit_table.md`), and five that spent most of their time on
the finer precision now add up their many samples in the ordinary one: **Cross Glare, Bloom,
Directional Blur, Line Blur and Turbulent Displace.**

**What you should see:** nothing different, only quicker. Every picture the card draws with these
effects is still within 1 level of 255 of the processor's, the same limit as before; a few more
pixels differ by that 1 level. Exports are drawn by the processor and do not change at all.

## What to check

Open the reference shot, set the viewer to **Full**, and for each effect below: put it on layer 2
from the Effects panel with its starting settings, press play, then delete it.

1. **Cross Glare.** Bright parts get star-shaped arms. Playing should now be close to smooth; it
   used to stutter badly.
2. **Bloom.** Bright parts glow with streaks.
3. **Directional Blur.** Set **Length** to 40. The layer smears along one direction.
4. **Line Blur.** The drawing's lines soften along their own direction, not across it.
5. **Turbulent Displace.** The layer wobbles as if seen through moving water.

For each, the look should be the same as before the build, with nothing flickering from frame to
frame.

## How long a frame takes

Measured on this machine, release build, the reference shot with the effect on three layers, the
median of three runs (`verification/B-157_timing_table.md`):

| Effect | Before ms | After ms |
|---|---:|---:|
| Cross Glare | 207.4 | 36.7 |
| Bloom | 139.5 | 57.1 |
| Directional Blur | 72.4 | 31.6 |
| Line Blur | 66.8 | 25.9 |
| Turbulent Displace | 40.9 | 28.4 |
| no effect (for comparison) | 19.2 | 19.4 |

## What to report

"works", or the number of the step that did something else, what it did, and the frame number.
