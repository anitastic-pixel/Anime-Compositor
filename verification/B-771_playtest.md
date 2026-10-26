# B-771 playtest: Echo's Blend and Minimum as After Effects draws them (D-891)

## What changed for you

Echo's **Blend** operator now draws as light as its **Add**: the copies are summed and held at
white. Before, Blend averaged the copies, so a lone copy came out about half as bright.

Echo's **Minimum** operator now draws the whole trail. Where copies overlap, it keeps the darker
colour. Before, a pixel had to be covered by every copy, so only the overlaps were left.

The other operators (Add, Maximum, Screen, Composite In Back, Composite In Front) are unchanged.
CC Wide Time and CC Force Motion Blur also still average, as before.

**Saved projects that change look:** any project with an Echo whose operator is Blend (about
twice as light where the copies do not overlap) or Minimum (the whole trail is drawn, darker where
copies overlap). Files open with no warning and save exactly as they were.

## What to look at

`verification/B-771 pictures/jake_minimum.png` and `jake_blend.png` each show three panels. The
left panel is After Effects (Jake In Motion's Echo tutorial). The middle panel is ours before
this change, and the right panel is ours after it. The table under each picture gives the middle
colour of each region.

- **Minimum:** After Effects shows both copies of the logo, yellow and dark green, and the green
  wins where green and yellow overlap. Before, ours showed only scraps where the copies overlap.
  After, ours shows both copies: yellow 245,254,151 against AE's 253,253,146, and green 8,144,79
  against AE's 13,143,82. The yellow and green are the stand-in logo's own colours.
- **Blend:** After Effects' lone copies are full yellow, 253,253,146. Before, ours was a faded
  123,217,127. After, ours is 245,254,151. Overlaps go to white (yellow on yellow: AE
  255,254,255, ours 255,255,255) and bright green on green (AE 26,253,166, ours 16,254,158).

`verification/B-771_echo_blend_minimum_table.md` has the fixtures FX-ECHO-008 and 012, all
within 2e-5. It also shows two projects saved before D-891 opening with no warnings and saving
unchanged.

The "before" panels were drawn by the wave 7 build. Echo's Blend and Minimum did not change
between that build and this change.

## Try it

1. Put a moving drawing in a comp and add Echo with Echo Time a frame or two back, Number of
   Echoes 3.
2. Set Echo Operator to Add, then to Blend. The two should look the same.
3. Set it to Minimum. The whole trail should show. Where two copies overlap, you should see the
   darker of the two colours.
4. Open an older project that used Blend or Minimum. It opens without any message and now draws
   as above.
