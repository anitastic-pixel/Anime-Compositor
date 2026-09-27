# B-75: Light Wrap, by hand

Built on 2026-09-26 against D-132, which you accepted the same day as the last of the second
batch of ten ("go ahead with the ten"), with the light read from everything beneath the layer,
your answer the same day.

The generated halves are `verification/B-75_light_wrap_table.md`, 86 of 86, which renders every
FX-WRAP case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Light Wrap card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-75a proposal/` shows what to expect.

Light Wrap is the first effect that reads the layers beneath its own, so the renderer draws a
layer with one differently from any other layer. Frames with a Light Wrap are drawn on the CPU
for now, even with the graphics card on; the log says so on such a frame.

## Before you start

Open a project with a bright background, a sky or a lit room, and a character drawn on a layer
above it. Select the character and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Light Wrap** in **Add effect…** on the character. It goes to the end of
   the stack, and the card shows **Width** 10, **Intensity** 100 and **Blend** Screen. The
   character's outline picks up a soft glow of the background's colours, strongest right at the
   edge, fading inwards. The middle of the character hardly changes.
2. **Colours from behind.** Where the sky behind is blue the edge turns bluish; where a lamp
   behind is orange, orange. Move the character across the background: the edge light changes
   with what is behind it.
3. **Width.** Width 40: the light reaches further in and is softer. Width 3: a thin rim. Width
   0: only the soft antialiased edge pixels take a little light.
4. **Intensity.** Intensity 300: much brighter. Intensity 0: the character as it was.
5. **Add.** Blend Add: brighter than Screen, and it can go past white on a very bright
   background.
6. **Nothing behind.** Hide the background: the wrap has nothing to take, and the character
   looks as it did without it.
7. **Order in the stack.** Put an Exposure of -1 on the character above the Light Wrap, then
   below it: both look the same. The wrap always runs last, as the character is laid on the
   frame.
8. **Opacity and blend.** Set the character's opacity to 50 per cent: the character, with its
   wrap, fades as a whole.
9. **On an adjustment layer.** Add Light Wrap to an adjustment layer: nothing changes, as an
   adjustment layer has no drawing of its own to wrap.
10. **Out of range.** Type 501 in Width: it is refused with a sentence saying it runs from 0 to
    500, and the card keeps its old number.
11. **Keyed.** Key Intensity at 0 and at 200 at a later frame. Play: the edge light fades in.
12. **Draft.** Press **Draft**: the picture is smaller, and the light reaches the same share of
    the way into the character.
13. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting
    and key is still there.

## Known limits, on purpose

- The light comes from everything beneath the layer, all of it; there is no choosing one layer
  to take the light from.
- Past the frame's edge there is nothing beneath, so a character at the very edge of the frame
  takes less light on that side.
- A frame with a Light Wrap is drawn on the CPU, not the graphics card, until the move to the
  card that follows this batch.
- It is modelled in spirit on the light wrap compositors use and is not claimed to match any
  one of them.

## What to answer

"works", or which step number did something else and what it did.
