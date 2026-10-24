# B-658 playtest: Shape Repeater opacity as After Effects steps it (D-778)

## What changed for you

A shape group's Repeater fades its copies from Start Opacity to End Opacity. Ours used to reach
End Opacity exactly on the last copy; After Effects stops one step short. With 6 copies, Start 100
and End 0, After Effects' copies are 100, 83, 67, 50, 33 and 17, and so are ours now; before, ours
were 100, 80, 60, 40, 20 and 0, so the last copy vanished.

Seven After Effects tutorials from six creators were measured and every one steps this way.

Nothing else of the Repeater moved: where the copies go, their order, Composite, Offset. A
repeater whose Start and End Opacity are the same looks exactly as before. Saved projects open as
they did; only the faded copies look a little stronger.

## What to look at

Each picture in `verification/B-658 pictures/` is After Effects on the left, ours before in the
middle, ours after on the right. Look at the faintest copy at the end of each row or circle.

| Picture | Tutorial | What to judge | Measured (last copy) | Verdict |
| --- | --- | --- | --- | --- |
| `rp_A_ssbotaru.jpg` | SSB Otaru, ebvf79jTQWA, about 0:31 | the last of 6 still shows in After Effects; before ours it was gone | AE 0.21, before 0, after 0.17 | close |
| `rp_B_shakemyart.jpg` | Shake my art, DtHNjDQHi2g, about 1:18 | the last of 7 green dots is not full strength | AE 0.85, before 1.00, after 0.87 | matches |
| `rp_D_visualmotions.jpg` | Visual Motions, dJAUUqDuCUM, about 5:43 | a ring of 15 fading in | copy 11: AE 0.66, before 0.70, after 0.65 | matches |
| `rp_E_aitee.jpg` | AITEE Media, 9m2bQ3OfWG8, about 24:12 | the 9th square faint but there | AE 0.11, before 0, after 0.11 | close |
| `rp_G_luisawinters.jpg` | Luisa Winters, sjayul5h-So, about 40:33 | the smallest flower | AE 0.27, before 0.13, after 0.27 of copy 1 | matches |

The numbers per copy are in `verification/B-658_repeater_opacity_table.md`.

## Try it

1. Make a shape layer with a rectangle, add a Repeater to its group: 6 copies, Position 120, 0.
2. Set Start Opacity 100 and End Opacity 0. The sixth copy is faint, not gone.
3. Set both to 100: all six full, as before.

## Known differences from After Effects (not fixed here)

- Every tutorial measured used Composite Below. Above is taken to step the same way.
- With a copies number that is not whole (say 3.5), no tutorial shows the steps; ours divides by
  3.5 itself, so the fade moves smoothly as copies is keyed.
