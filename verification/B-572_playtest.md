# B-572: Reshape's Correspondence Points and Interpolation

Built on 2026-10-13 as D-702. This closes the gaps B-485 (D-606) left on Reshape:

- **Correspondence Points** say which places on the source outline answer which places on the
  destination outline.
- **Interpolation** (Discrete, Linear or Smooth) says how Percent moves between its keys.

Adobe does not publish how these are worked out, so the rule is this program's own.

The check is `verification/D-702_reshape_points_table.md`, and 113 of its 113 lines pass. It holds:

- every pixel to numbers that a separate program worked out before the code was written;
- the graphics card's picture to the processor's, within 1 level.

D-606's own check still passes: with no pairs and Discrete, every frame is the same as before.
The pictures are in `verification/D-702 pictures/`.

## What to check

1. **`1_before.png`**: the town street, no effect.
2. **`2_matched_from_the_start.png`**: a square in the middle of the street pulled 60 pixels
   right, with no correspondence points. One pair (0, 0) draws exactly the same picture, which
   the check confirms.
3. **`3_turned_a_quarter.png`**: the same pull with the pair (0, 25). The source's top-left corner
   now answers the destination's top-right corner, so the middle of the street swirls about a
   quarter turn on its way, not sliding straight. It should look clearly different from 2.
4. **`4_discrete_frame_3.png`**, **`5_linear_frame_3.png`** and **`6_smooth_frame_3.png`**:
   Percent keyed 0, 100 and 40 at frames 0, 2 and 4, all shown at frame 3.
   - Discrete pulls once, 70 per cent of the way.
   - Linear mixes the 100 and 40 per cent pulls half and half, so the pull looks doubled and
     softer.
   - Smooth follows a curve through all three keys.
   - The three should not look the same.
5. **In the app.** Add Reshape to a layer that has two masks. Set Source Mask 1 and Destination
   Mask 2.
   - Under **Correspondence Points**, type `0 25` and watch the picture swirl. Type `none` to go
     back.
   - Key Percent at three frames. Switch **Interpolation** between Discrete, Linear and Smooth,
     and step through frames that fall between the keys. The in-between frames change; the
     frames on the keys do not.

### Not like After Effects

- The pairs are typed as numbers (per cent of the way round each outline from its first point).
  They are not dragged on the viewer.
- The masks are still chosen by number.
- Linear and Smooth blend whole pulls instead of moving the outline point by point. With strong
  keys, the in-between frames can show two pulls laid over each other.

## Speed

See `verification/B-572_reshape_points_timing_table.md`.
