# B-91: Diffusion, by hand

Built on 2026-09-26 against D-148, accepted by your message of the same day asking for thirty
more effects ("afterwards, work on 30 more recommended effects for either AE or/and
anime/animation related."). It is the fifteenth of the thirty.

The generated halves are `verification/B-91_diffusion_table.md`, which renders every
FX-DIFFUSE case against the numbers written before the code, and
`verification/B-12b_state_fields_table.md`, which checks the Diffusion card sends every setting
the command reads. This sheet covers what the tables cannot: how it looks and feels in the
window. The picture in `verification/B-91a proposal/` shows what to expect.

## Before you start

Open a project with a drawn, coloured character layer with light and shadow tones. Select the
layer and press **Full resolution**.

## What to check

1. **Adding it.** Pick **Diffusion** in **Add effect…**. It goes to the end of the stack, and
   the card shows **Radius** 10, **Amount** 50 and **Blend** Screen. The drawing takes on the
   soft, dreamy glow of anime compositing's diffusion: every part a little lighter, the shadows
   lifted most, the lights blooming softly into their neighbours.
2. **More.** Amount 100: a stronger glow. Amount 0: the drawing untouched.
3. **Wider.** Radius 40: the glow spreads further and gets more even. Radius 0: untouched.
4. **Blends.** **Lighten**: only the parts darker than the glow change. **Normal**: the drawing
   is replaced toward a soft blur of itself, so the lights can darken too.
5. **Inside the outline.** Nothing glows out past the drawing's outline onto the empty space.
6. **Out of range.** Type 501 in Radius: it is refused with a sentence saying it runs from 0 to
   500, and the card keeps its old number.
7. **Keyed.** Key Amount at 0 and at 100 at a later frame. Play: the picture softens into its
   glow, a dream-sequence fade.
8. **Draft preview.** Switch to a half-size draft: the glow looks about as wide on screen as at
   full size.
9. **Undo and saved.** Ctrl+Z steps back each change. Save, close and open again: every setting,
   the blend and every key is still there.

## Known limits, on purpose

- **The glow is of the whole picture, not only its lights**, so at a large amount a dark line is
  lifted as much as a shadow, and the drawing loses contrast.
- The glow stays inside the drawing's own pixels; for a halo out past its outline, use Glow or
  Light Wrap.
- A wide radius on a small drawing gives mostly the drawing's own average colour.
- After Effects has no effect by this name; it is this program's own, after the compositing
  habit of laying a blurred copy over the picture in screen mode.

## What to answer

"works", or which step number did something else and what it did.
