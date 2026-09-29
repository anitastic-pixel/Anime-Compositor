# B-135a: Radio Waves, rings sent out from a point (D-200)

Written on 2026-09-29, before any code. The tenth of the After Effects picks, A10, accepted with the rest by your "take everything". It does what After Effects' Radio Waves does, by a rule of our own.

## What you will see

A new effect, **Radio Waves**, in the **Generate** group. It sends rings out from a point one after another, each growing as it ages, and paints them over the layer: the shockwave of an anime hit, a sonar ping, a radio mast's signal, a ripple, a target lock. A ring can be round or have 3 to 64 sides, turn as it grows, drift away from the point, fade in and out, and thin or thicken as it goes. The rings are painted over the drawing and the empty parts round it alike, inside the layer's rectangle. Its card, in After Effects' words:

- **Producer Point**, a point: where the rings start, across and down in per cent of the drawing's own width and height; (50, 50) when added. Keyable; a ring already sent follows the point.
- **Sides**, 3 to 64: 64, which looks round, when added; 3 is a triangle, 4 a square. 4.7 counts as 4.
- **Interval**, in frames, 1 to 1000: a new ring every so many frames, the first at frame 0; 24 when added, one a second at 24 frames a second. After Effects calls it Frequency and counts rings a second; ours counts frames between rings, as Rain and Kira-kira count in frames.
- **Expansion**, pixels a frame, 0 to 1000: how fast each ring grows; 5 when added.
- **Orientation**, degrees: where the first corner points; 0 when added, straight up.
- **Direction**, degrees clockwise from up, and **Velocity**, pixels a frame, 0 to 1000: which way and how fast each ring's middle drifts away from the point; 90 and 0 when added, so the rings stay put.
- **Spin**, degrees a frame, -360 to 360: how fast each ring turns as it ages; 0 when added.
- **Lifespan**, in frames, 1 to 1000: how long a ring lives; 96 when added.
- **Profile**: how a ring's line is drawn across its width: **Square**, a hard line, the default; **Triangle**, brightest in the middle, fading evenly to its sides; or **Sine**, fading softly.
- **Color**: white when added.
- **Opacity**, 0 to 100: 100 when added.
- **Fade-in Time** and **Fade-out Time**, in frames, 0 to 1000: how long a ring takes to come up after it is sent and to go at the end of its life; 0 and 48 when added.
- **Start Width** and **End Width**, in pixels, 0 to 1000: the line's width when a ring is sent and when it dies, changing evenly between; 5 and 5 when added.

Every number is keyable. The card's picture is a box standing for the drawing, with the producer point as a mark to drag and a few rings drawn round it by the card's sides and orientation.

`radio_waves.png` in this folder is worked by the rule itself: a badge before; as it starts; a shockwave, wide and thinning as it fades; turning squares; a triangle drifting right; and soft orange rings.

## How it differs from what we already have

**Speed Lines** draws lines rushing to a point, **Kira-kira** scatters sparkles and **Rain** drops streaks. Nothing yet draws rings, and nothing sends shapes out one after another on its own as time passes.

## Known limits

- Only polygon rings: After Effects can also send out a mask's outline or an image's contours, and bend the sides into stars and curves; ours cannot.
- Three profiles: Square and Sine as After Effects has them, and Triangle; After Effects' ramps and saws are left out.
- A ring follows the settings as they are now, not as they were when it was sent: key the colour and every ring on screen changes together. After Effects can hold each ring's settings from its birth.
- Rings pass off the layer's edge; they do not bounce back from it.
- Rings are counted from frame 0 of the composition; the first is sent there.
- There is no handle for the producer point in the viewer yet: move it by the card's mark or its numbers.
- It runs on the processor; the graphics card version is its own later unit.
- It is modelled on After Effects' Radio Waves and is not claimed to match it.

## How you will check it

The build's test draws the twenty fixture cases and the seven wrong settings and compares every pixel with the numbers `tools/radio_waves_reference.py` worked out, and writes `verification/B-135_radio_waves_table.md`, with pictures. A playtest sheet walks you through sending rings out over a badge and playing them.
