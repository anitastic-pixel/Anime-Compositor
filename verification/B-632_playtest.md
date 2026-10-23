# B-632 playtest: Directional Blur's streak tapers at its ends (D-753)

## What to try

1. Put **Directional Blur** on a small bright shape on black, Length 80. The streak is brightest in the middle and fades away towards both ends, as After Effects' does. Before this build it was an even bar with hard ends.
2. Compare one After Effects tutorial at its own Length (Khnar film 0:36, 262 degrees, Length 81.4): the faded ends now match.
3. Open any project saved before this build. Its Directional Blur keeps the same Length and direction; only the shape of the streak changes, as D-753 records.
4. Put **Bloom** on a picture. Bloom's pictures are exactly as before (its own blur is not touched).

## What the check found

Fixture check `tests/b632_directional_taper.rs`: 141 of 141 (see `B-632_directional_taper_table.md`). FX-DIRBLUR-001 to 015 are pixel-compared with `tools/directional_blur_reference.py`, which now uses the tent: full span Length, the weight falling straight to nothing at each end. The card's pictures match the CPU's within 1 level.

Bloom's own checks (b40_bloom, b47_gpu_bloom) pass unchanged. Other tests that also pass with the new streak: b11, b155, b156, b172, b173, b221, b240, b329, b340, b39, b410, b412, b442, b495, b49, b50, b52_edges, b52_gpu_edges, b581, b623, b69, b86, b94, b99. Tables that print Directional Blur pictures were run again (B-36, B-49, B-52, B-155, B-156, B-173, B-221, B-623, B-69, B-86, B-94, D-361, D-449, D-460, D-562).

## Against the tutorials (eight stills)

All panels were drawn by the real app on the CPU. The measure is the mean difference from the After Effects still over the streaked area; lower is closer. "Floor" is the lowest any length reaches.

| Still | At the tutorial's Length: before → after | Best length: old bar → new streak | Verdict |
| --- | --- | --- | --- |
| Khnar film, 262°, L81.4 | 0.0224 → 0.0049 | L48 0.0073 → L76.8 0.0041 (floor 0.0041) | matches |
| Esli prosto, 45°, L66.25 | 0.0241 → 0.0135 | L48.6 0.0140 → L77.76 0.0074 (floor 0.0073) | matches at best length; closer at the tutorial's |
| aace, 0°, L53 | 0.0445 → 0.0418 | L43.5 0.0407 → L65.25 0.0375 (floor 0.0375) | matches |
| emaeditzs, 0°, L50 | 0.0185 → 0.0122 | L36 0.0131 → L54 0.0118 (floor 0.0119) | matches |
| twibytez, 0°, L25 | 0.0078 → 0.0132 | L29.7 0.0070 → L47.52 0.0061 (floor 0.0062) | matches at best length |
| WTFx, 0°, L12.3 | 0.0133 → 0.0196 | L20.7 0.0083 → L33.12 0.0068 (floor 0.0068) | matches at best length |
| WTFx, 118°, L13.4 | 0.0121 → 0.0176 | L22 0.0082 → L35.2 0.0070 (floor 0.0073) | matches at best length |
| Justin, 0°, L50 | 0.0489 → 0.0642 | L87.3 0.0357 → L139.68 0.0326 (floor 0.0325) | matches at best length |

What this means:
- The new streak beats the old bar at its best length on all 8 stills, and reaches the floor on all 8. The old bar never could.
- Only Khnar and Esli have a known screen scale (the tutorial shows the composition at a known zoom). On those two the new streak is far closer at the tutorial's own Length (-78 % and -44 %), and Khnar's best length is 0.94 times the tutorial's.
- On the other six the screen scale is unknown, so "at the tutorial's Length" mostly measures the scale, not the streak. That is why four of them read worse there; their best lengths are 1.9 to 2.8 times the typed Length, which is the zoom, not the blur.

## Pictures (`B-632 pictures/`)

Each sheet shows After Effects, before and after.
- `dirblur_khnar_at_ae_length.png`: Khnar at its own Length. The old bar ends sharply; After Effects and this build fade out.
- `dirblur_khnar.png`, `dirblur_esli.png`, `dirblur_aace.png`, `dirblur_emae.png`, `dirblur_twib.png`: each at its best length.
- `dirblur_esli_at_ae_length.png`: Esli at its own Length.

## How this differs from After Effects

- The streak shape is read off tutorial stills, not After Effects' code. The tent fits every still measured; a smoother curve could not be told apart at these sizes.
- Projects saved before this build draw a softer, slightly shorter-looking streak at the same Length. That is the 1:1 fix D-753 asks for, not kept as an option.
