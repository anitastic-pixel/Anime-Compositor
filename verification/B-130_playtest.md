# B-130: Echo, by hand

Built on 2026-09-29 against D-195, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Echo does, by a rule of our own: it lays copies
of the layer from earlier or later frames over it, each fainter than the last, for a trail, a
smear or an onion skin. Echo Time says how many frames apart the copies are, Number Of Echoes how
many there are, Starting Intensity how strong the present frame is, Decay how much of each copy
the next one keeps, and Echo Operator how they are laid together. It is the first effect in a new
**Time** group.

The generated halves are `verification/B-130_echo_table.md`, 134 of 134 checks passing, which
renders every FX-ECHO case against the numbers written before the code and draws the pictures
below, and `verification/B-12b_state_fields_table.md`, which checks the Echo card sends every
setting the command reads. This sheet covers what the tables cannot: how it looks and feels in
the window.

## The pictures

`verification/B-130 pictures/`, a ball bouncing left to right on ones, 24 drawings at a quarter
of 1920 by 1080, over a night-blue solid that stands for the shot beneath. Every picture is
frame 16:

- `ball_01.png` to `ball_24.png`: the drawings.
- `before.png`: the ball with no effect.
- `trail_add.png`: 6 echoes 2 frames back, Decay 0.7, Add. A trail back along the bounce, each
  ball fainter, brighter where two balls overlap.
- `trail_in_back.png`: the same with Composite In Back. The present ball whole on top, not
  brightened.
- `smear.png`: 11 echoes 1 frame back, Decay 0.8, Add. A close smear.
- `ahead_screen.png`: Echo Time +2, 3 echoes, Decay 0.5, Screen. Where the ball is going.
- `blend.png`: 3 echoes 1 frame back, Decay 1, Blend. Four drawings averaged, each a quarter
  strong.

## Before you start

Make the composition 480 by 270, 24 frames. Import all 24 `ball_##.png` files in
`verification/B-130 pictures/` together, so they come in as one layer, a drawing a frame, and
make a night-blue solid the size of the composition beneath it. Press **Full resolution** and go
to frame 16.

## What to check

1. **Adding it.** On the ball layer, pick **Echo** in **Add effect…**, under **Time**, a new
   group after Camera & Lens; typing "echo", "trail", "ghost", "onion skin" or "smear" in the
   search finds it too. The card shows **Echo Time** -1, **Number Of Echoes** 1, **Starting
   Intensity** 1, **Decay** 1 and **Echo Operator** Add. One echo shows: the ball of frame 15
   added to the ball of frame 16, brighter where they overlap.
2. **A trail.** Set Echo Time -2, Number Of Echoes 6 and Decay 0.7: it looks like
   `trail_add.png`. Play: the trail follows the ball.
3. **On top.** Echo Operator Composite In Back: like `trail_in_back.png`, the present ball clean
   on top. Composite In Front puts the furthest echo on top instead.
4. **The other operators.** Maximum keeps the brightest of the balls where they overlap, Minimum
   only what every ball covers (with 6 echoes, nothing), Screen brightens more gently than Add,
   and Blend averages them: at 3 echoes 1 frame back and Decay 1 it looks like `blend.png`.
5. **A smear.** 11 echoes 1 frame back, Decay 0.8, Add: like `smear.png`.
6. **Ahead.** Echo Time +2, 3 echoes, Decay 0.5, Screen: like `ahead_screen.png`, the ball where
   it is going. At the last frames there is nothing after the layer's end, so the echoes run out.
7. **The start.** At frame 0 with Echo Time -2 there is nothing before the layer's first frame:
   only the ball shows.
8. **None.** Number Of Echoes 0: the ball as it is. Starting Intensity 0.5 with it: the ball at
   half strength.
9. **Whole frames.** Echo Time -1.5 draws what -2 draws, and Number Of Echoes 2.5 what 2 draws.
10. **Earlier effects.** Put an Exposure before the Echo: the echoes do not see it, since Echo
    reads the layer's drawings. Put it after the Echo: the whole trail changes.
11. **Masks and moving.** Draw a mask on the ball layer: every echo is cut by it. Move the layer:
    the trail moves with it.
12. **Adjustment layer.** Put the Echo on an adjustment layer above the ball: nothing changes,
    since an adjustment layer has no drawing of its own.
13. **Out of range.** Type 31 in Number Of Echoes: it is refused with a sentence saying it runs
    from 0 to 30, and the card keeps its old number. The same for Echo Time 121.
14. **Keyed.** Key Number Of Echoes from 0 at frame 0 to 10 at frame 23 and play: the trail grows
    as the ball goes. Key Starting Intensity from 1 to 0: the ball and its trail fade out.
15. **Draft.** Press **Draft**: the picture is smaller and the trail with it, the same shape.
16. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- Echo reads the layer's own drawings at the other frames, with its masks, not what effects
  before it made of them, and never the layers beneath.
- Echoes are laid on the layer: the picture never grows past the layer's size, so a trail that
  would run off the layer's edge is cut there.
- Echo Time and Number Of Echoes are taken as whole frames and counted down: -1.5 is -2 and 2.5
  is 2.
- Each echo is a frame of the layer to draw again, so the cost grows with Number Of Echoes.
- It is drawn on the processor; the graphics card learns it in a card unit of its own.
  The reference shot's frame 100 at full size takes about 51 ms without it. With an Echo on its
  first layer, 2 frames back, Decay 0.7: 1 echo about 105 ms, 4 echoes about 166 ms (Add or
  Composite In Back alike), 10 echoes about 303 ms.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
