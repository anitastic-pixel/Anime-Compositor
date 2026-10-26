# B-772 playtest: CC Light Rays fades with distance as After Effects' does (D-892)

## What changed for you

Light Rays (CC's controls) is now about as bright as After Effects' and fades with distance the
way After Effects' does. Before, its rays lit the whole frame: on Jake In Motion's photograph the
frame was lifted about five times as much as in After Effects, and the rays still showed at the
picture's edge.

Two things changed:

- The rays are laid on at 0.625 of their old strength, at every depth.
- In an 8 bpc composition and in 32 bpc (After Effects), the rays are worked on display values,
  as Glow already is. So a faint ray far from the centre no longer brightens the dark parts a long
  way.

A Light Rays you add from now on starts with its centre a quarter of the way in (25, 25), where
After Effects starts it. Before, it started in the middle.

**Saved projects that change look:** every project with a Light Rays made since D-423 (the CC
controls). Its rays are darker, and in 8 bpc or 32 bpc (After Effects) much darker far from the
centre. Its centre does not move. Files open with no warning and save exactly as they were.
Light Rays from older projects (with Length and Threshold) are unchanged.

## What to look at

`verification/B-772 pictures/` holds five sheets of three panels each. The left panel is After
Effects (Jake In Motion, "CC Light Rays | Effects of After Effects"). The middle panel is ours
before this change, and the right panel is ours after it. Under each picture is the brightness
added at each distance from the centre, with the reach (where it drops below 10 levels) and the
whole frame's average.

| Sheet | After Effects | Before | After |
| --- | --- | --- | --- |
| `jake_0051_intensity100.png`, Intensity 100, Radius 40 | frame 8.66, reach 365, 600 px 6.1 | 43.22, 1135, 42.2 | 9.51, 405, 6.8 |
| `jake_0124_intensity406.png`, Intensity 406 | frame 38.56, 600 px 30.2 | 97.43, 94.5 | 38.23, 29.9 |
| `jake_0154_radius86.png`, Radius 86 | frame 53.72, 600 px 45.0 | 114.43, 111.8 | 55.41, 45.7 |
| `jake_0213_color_off.png`, Color from Source off | frame 96.33, 600 px 84.8 | 187.25, 188.8 | 95.04, 82.5 |
| `jake_0230_lighten_gap.png`, Transfer Mode Lighten | frame 63.25, 50 px 38.0 | 128.58, 96.4 | 63.90, 96.0 |

On the first four, the right panel should look like the left one: dark corners and a glow that
fades out around the window. The middle one should look washed out.

The last sheet shows a gap that is not fixed. With Transfer Mode Lighten, After Effects is much
darker close to the centre than ours.

`verification/B-772_cc_light_rays_display_table.md` has 178 of 178 checks:

- the fixtures FX-RAYSCC-001 to 026 and the new 040 to 042, all within 2e-5;
- 25 projects saved before D-892 opening and saving unchanged;
- the old Light Rays fixtures FX-RAYS-001 to 024 drawn as before;
- the card within 1 level of the processor, in 8 bpc and 32 bpc (After Effects).

## Gaps still open

- Right at the centre (10 px) After Effects is 25 to 50 levels darker than ours.
- Warp Softness at a large radius: After Effects' Warp 0 and Warp 50 at Radius 290 differ, ours
  do not, and ours are lighter.
- Transfer Mode Lighten and None do not match After Effects.

## Try it

1. Make a comp at 8 bpc with a photograph. Add Light Rays. Its centre should start a quarter of
   the way in from the top left.
2. Move the centre onto a bright spot and set Radius to about 40. The rays should glow around the
   bright spot and fade out well before the edges. The corners should stay dark.
3. Raise Intensity to 400. The glow should spread further but still fade out.
4. Open an older project with a Light Rays. It opens without any message. It now draws darker,
   with its centre where it was.
