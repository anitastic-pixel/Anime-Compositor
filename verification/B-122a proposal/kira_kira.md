# B-122a: Kira-kira, twinkling stars on a drawing's highlights (D-186)

Written on 2026-09-28, before any code. Accepted with items 1 to 9 of the list of effects and presets left to build. It is our own effect, built from scratch; nothing is ported.

## What it is

In anime finishing, *kira-kira* (キラキラ) is the sparkle a compositor sets on a highlight: a small four- or eight-pointed star on a blade's edge, a jewel, a wet eye or the sun on the sea, each growing and fading on its own beat so the light seems to glint.

## What you will see

A new effect, **Kira-kira**, under **Light & Glow** beside Cross Glare. Its card has these rows:

- **Threshold**, 0 to 100 %, 95 when added: how near white a pixel must be to count as a highlight.
- **Spacing**, 2 to 1000 pixels, 64: the frame is cut into squares this wide, and each square holds at most one star, at the middle of its highlights.
- **Density**, 0 to 100 %, 60: the share of the squares with a highlight that get a star.
- **Size**, 0 to 1000 pixels, 40: how far a star's long arms reach. Each star is between 60 and 100 % of it, its own size.
- **Shape**: Cross, four arms, or Star, four arms and four shorter ones between them, as added.
- **Angle**, a dial, 0 when added: the turn of every star.
- **Twinkle**, 0 to 100 %, 100: how far each star shrinks and fades between its brightest moments. 0 holds them steady; 100 lets each go out once a period.
- **Period**, 1 to 1000 frames, 24: how long one twinkle takes. Each star starts at its own point in it.
- **Seed**, 0 to 100000, 0: another seed picks other squares, other sizes and other beats.
- **Opacity**, 0 to 100 %, 100, and **Colour**, white when added.

Every number can be keyed. A draft scales Spacing and Size with the picture, so a draft looks like the full picture, smaller.

What it does: it finds the highlights, the pixels whose red, green and blue are all at least Threshold per cent of full, that is, near white, as anime highlights are painted. A pale skin, a cream moon or a gold, bright but coloured, is not a highlight, so a face does not fill with stars. Each square with highlights may get a star at their middle. The star is added as light: it brightens what is under it, and where the layer is clear it paints the star, so the layer grows by Size on every side to hold stars at its edges.

## How it differs from what we already have

- **Cross Glare** streaks every bright pixel, the same on every frame. Kira-kira sets one star per patch of highlight, leaves some patches bare, and makes each star twinkle on its own beat.
- **Glow** and **Cross Glare** count a pixel as bright by its brightest channel, so a pale skin or a yellow counts. Kira-kira counts only near-white pixels.

`kira_kira.png` in this folder is worked by the reference rule itself, at half of 1920 by 1080, with Spacing and Size halved as a half-size draft does: a night scene with a face whose eyes have white glints, a sword with a white edge, a jewel with a white glint, and the moon's path on the sea; then as added at frame 0 and at frame 12, half a period on, where other stars are at their brightest; then Density 100; then Spacing 32 and Size 24, smaller stars closer together; then a warm Cross at angle 45. The face, the pale collar and the moon have no star in any of them.

## Known limits

- A white area as large as a shirt, if painted pure white, is all highlight, and gets a star in each square. Raise Threshold, or put the highlights on their own layer, as anime finishing often does.
- Two glints in one square share one star between them. A smaller Spacing gives each its own.
- A star moves with its square's highlights; a glint crossing a square's edge hands its star to the next square.
- It runs on the processor. The graphics card learns it in item 9, with the other new effects.

## How you will check it

The build's test draws the twenty-nine fixture cases and compares every pixel with the numbers `tools/kira_kira_reference.py` worked out, and writes `verification/B-122_kira_kira_table.md`, with pictures. A playtest sheet walks you through adding it to a picture, playing it, and trying each setting.
