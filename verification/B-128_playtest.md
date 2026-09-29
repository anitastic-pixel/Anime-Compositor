# B-128: Displacement Map, by hand

Built on 2026-09-29 against D-193, which you accepted with the rest of the After Effects picks
("take everything"). It does what After Effects' Displacement Map does, by a rule of our own: one
layer's pixels are moved by the colours of another layer of the same composition, the map. Where
the chosen channel is white the picture moves left (across) or up (down) by the maximum; where it
is black, as far the other way; mid grey does not move it. Waves, heat haze, glass and water are
all a map away. It is the second effect that reads another layer (D-189), after Compound Blur.

The generated halves are `verification/B-128_displacement_map_table.md`, 142 of 142 checks
passing, which renders every FX-DMAP case against the numbers written before the code and draws
the pictures below, and `verification/B-12b_state_fields_table.md`, which checks the Displacement
Map card sends every setting the command reads. This sheet covers what the tables cannot: how it
looks and feels in the window.

## The pictures

`verification/B-128 pictures/`, B-127's street at a quarter of 1920 by 1080. A quarter-size
picture takes a quarter of the distance, as a quarter-size draft does, so 3 here shows what 12
does at full size:

- `town.png`: the street.
- `map_waves.png`: a map of waves, red in bands down the picture, green in waves across it.
- `map_ball.png`: a map of a glass ball, clear outside it.
- `before.png`: the street with no effect.
- `waves.png`: the waves, red across and green down, at most 3. The houses and the road ripple,
  and clear comes in at the edges where the picture moves away from them.
- `waves_wrapped.png`: the waves at most 10 with Wrap Pixels Around. Much stronger, and the edges
  are filled from the far side: the road's dark shows along the top.
- `glass_ball.png`: the ball, at most 40% of its size. Inside it the street is magnified;
  outside it not one pixel changes.
- `heat_haze.png`: the street naming itself, luminance across at most 4 and nothing down. The
  lit windows shift sideways against the dark ones; the even sky does not show it. The dark road
  reads from its left, so a clear strip two pixels wide comes in at its left edge.

## Before you start

Import `verification/B-128 pictures/town.png` and `verification/B-128 pictures/map_waves.png`,
make a layer from each with `town` on top, make the composition 480 by 270, and press **Full
resolution**. Switch the `map_waves` layer off with its eye: a map is read whether or not it is
shown.

## What to check

1. **Adding it.** On `town`, pick **Displacement Map** in **Add effect…**, under **Distort**,
   after Ripple; typing "displace", "warp" or "haze" in the search finds it too. The card shows
   **Displacement Map Layer** None, **Use For Horizontal Displacement** Red, **Max Horizontal
   Displacement** 5, **Use For Vertical Displacement** Green, **Max Vertical Displacement** 5,
   **Displacement Map Behavior** Stretch Map to Fit and **Wrap Pixels Around** Off. With no
   layer named, nothing changes.
2. **The map.** Set Displacement Map Layer to `map_waves` and both maxima to 3: it looks like
   `waves.png`. The list names every layer of the composition; `town` says "(this layer)".
3. **Wrap.** Set both maxima to 10 and Wrap Pixels Around On: like `waves_wrapped.png`. Off
   again: clear comes in at the edges instead.
4. **Channels.** Set Use For Horizontal Displacement to Off: the ripples across stop, the ones
   up and down stay. Full: the whole picture moves left by the maximum. Try Luminance, Hue,
   Lightness and Saturation: each moves the picture by a different reading of the map.
5. **The other way.** Type -10 in Max Horizontal Displacement: the ripples across turn the other
   way.
6. **The glass ball.** Import `map_ball.png`, switch it off, name it, both maxima 41: like
   `glass_ball.png`. Outside the ball nothing moves, since the map is clear there.
7. **Itself.** Set the layer to `town (this layer)`, Horizontal Luminance at 4 and Vertical Off:
   like `heat_haze.png`.
8. **Out of range.** Type 1001 in Max Horizontal Displacement: it is refused with a sentence
   saying it runs from -1000 to 1000, and the card keeps its old number.
9. **Sizes.** Make a small solid, 120 by 60, white, and name it, Horizontal Red. Stretch Map to
   Fit: the whole picture moves left. Centre Map: only the middle 120 by 60 moves. Tile Map: all
   of it, since white repeated is white.
10. **A circle.** Add a Displacement Map, or a Compound Blur, to `map_waves` and name `town`. It
    is refused with a sentence: the two layers would read each other in a circle. Naming the
    layer itself is never a circle.
11. **A missing layer.** Delete `map_waves` while `town` names it: the ripples go, and the
    warning panel says the effect reads a layer that is not in the composition. Undo: it comes
    back and so do the ripples.
12. **Keyed.** Key Max Horizontal Displacement from 0 at the first frame to 10 a second later:
    the ripples grow.
13. **Moving.** Move, scale or rotate `map_waves`: nothing changes, since a map is read before its
    layer is placed. Move `town`: the ripples move with it, since the map lies on the layer.
14. **Draft.** Press **Draft**: the picture is smaller and the ripples with it, the same shape.
15. **Undo and saved.** Ctrl+Z steps back each change, the layer named included. Save, close and
    open again: every setting and key is still there.

## Known limits, on purpose

- After Effects' Half channels are left out: Full and Off cover what they are used for.
- A map is read the size of its own drawing and fitted to the layer by Displacement Map
  Behavior, not where it sits in the frame. On an adjustment layer the map is fitted to the frame.
- The picture never grows: what moves past the layer's edge is gone, and what comes in from
  beyond it is clear, or with Wrap on the far side.
- It is drawn on the processor; the graphics card learns it in a card unit of its own. The
  reference shot's frame 100 at full size, 1920 by 1080, takes 49 to 52 ms without it and 95 to
  102 ms with a Displacement Map on its first layer reading another, whatever the channels, the
  maxima (5, 40 or 1000) or Wrap: about 45 ms for the map and the move together.
  **Machine:** AMD Ryzen 9 9900X with 24 threads; Windows 11; release build; timed by a
  throwaway test, not kept.

## What to answer

"works", or which step number did something else and what it did.
