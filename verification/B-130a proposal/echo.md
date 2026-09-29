# B-130a: Echo, copies of a layer from other frames laid over it as trails (D-195)

Written on 2026-09-29, before any code. The fifth of the After Effects picks, A5, accepted with the rest by your "take everything". It does what After Effects' Echo does, by a rule of our own, and it is the first effect that reads its layer at other frames.

## What you will see

A new effect, **Echo**, in a new folder, **Time**. It lays copies of the layer's drawing from earlier or later frames over the present one, each fainter than the last: a ball leaves a trail of balls behind it, a fast movement leaves a smear. Its card has these rows, in After Effects' words and order:

- **Echo Time**, in frames, -120 to 120, -1 when added: how far apart the copies are. Below 0 they come from earlier frames, a trail; above 0 from later ones, ghosts of where the layer is going. A part frame counts as the whole frame below it. It can be keyed.
- **Number Of Echoes**, 0 to 30, 1 when added: how many copies, whole numbers. It can be keyed.
- **Starting Intensity**, 0 to 1, 1 when added: how strong the present frame is.
- **Decay**, 0 to 1, 1 when added: each copy is this much of the one before it, so 0.5 halves each time and 1 keeps them all as strong as the present. Both can be keyed.
- **Echo Operator**, how the copies are put together, **Add** when added: **Add** adds them, brighter where they overlap, up to white; **Maximum** and **Minimum** keep the brightest or darkest; **Screen** adds them more gently; **Composite In Back** lays each copy behind the one after it, so the present frame is on top; **Composite In Front** the other way, the oldest copy on top; **Blend** averages them.

The copies are the layer's own drawing at those frames, through its masks, but not through its effects: Echo reads the drawing, as After Effects' time effects do, so an effect placed before Echo is not seen and one placed after it changes the whole trail. A frame before the layer starts or after it ends gives no copy. The copies lie on the layer, not where it was in the frame: a layer moved by its position leaves no trail, and a drawing that moves within itself does. With no echoes and full intensity the layer is exactly as it was. The layer does not grow. On an adjustment layer it changes nothing.

`echo.png` in this folder is worked by the rule itself, at a quarter of 1920 by 1080, over a night blue that stands for the shot beneath: an inked ball drawn bouncing across twenty-four frames; at frame 16 with six echoes two frames back, decay 0.7, Add, a bounce's worth of fading balls, brighter where two overlap; the same with Composite In Back, the present ball whole on top; eleven echoes one frame back at decay 0.8, a smear; Echo Time +2, ghosts of where it is going; and Blend, three echoes averaged, a soft blur of the last four frames.

## How it differs from what we already have

**Motion blur** (D-176) smears a layer along how it is moved, between one frame and the next. Echo repeats the drawing itself from frames further away, so it shows a drawing's own animation as a trail, and the copies stay separate unless the echo time is small. **Speed Lines** draws lines; Echo draws the layer.

## Known limits

- A layer moved by its position, scale or rotation leaves no trail, since the copies lie on the layer; to echo a movement, put the layer in a composition and echo that, as in After Effects.
- Add and Screen hold each number at 1, so bright overlaps turn white rather than brighter than white.
- Each copy is drawn again at its own frame, so thirty echoes cost about thirty drawings.
- It runs on the processor. The graphics card learns it in a card unit of its own.

## How you will check it

The build's test draws the twenty-three fixture cases and the seven wrong settings and compares every pixel with the numbers `tools/echo_reference.py` worked out, and writes `verification/B-130_echo_table.md`, with pictures. A playtest sheet walks you through adding it to a moving drawing and trying each operator.
