# B-758: the card motion finder against the processor's (D-878)

Written by `tests/b758_card_motion_finder.rs`. Change nothing here by hand.

A frame planned for the card finds its motion on the card at Draft, and at Full from vector detail 40; at Full below 40 (vector detail 20 as added) the processor finds it (checked above this table on Pixel Motion Blur and Timewarp).

## Found motion, card against processor

The reference shot's frames 0 to 210, every 30th, each with the next, as the layer's picture the finder is given.

| quality | vector detail | vectors | largest difference (px) | over 1e-4 px | over 0.05 px |
|---|---:|---:|---:|---:|---:|
| Full | 20 | 33177600 | 0.000e0 | 0 | 0 |
| Full | 40 | 33177600 | 0.000e0 | 0 | 0 |
| Full | 100 | 33177600 | 0.000e0 | 0 | 0 |
| Draft | 20 | 2073600 | 0.000e0 | 0 | 0 |
| Draft | 40 | 2073600 | 0.000e0 | 0 | 0 |
| Draft | 100 | 2073600 | 0.000e0 | 0 | 0 |

## Pictures, the card's finder against the processor's

Each frame planned for the card as the viewer plans it, once with each finder, both drawn by the processor so only the finder differs. A pixel transparent in both is not counted.

| set | frames | pixels | within 1 level | largest (levels) | float, solid | float, any | motions the card found |
|---|---:|---:|---:|---:|---:|---:|---:|
| Fixtures/pixel_motion_blur, every frame, Full and Draft | 288 | 156672 | 100.0000% | 0 | 1.19e-7 | 1.79e-7 | 234 |
| Fixtures/timewarp, every frame, Full and Draft | 396 | 215424 | 100.0000% | 0 | 1.19e-7 | 4.17e-7 | 182 |
| Fixtures/warp_stabilizer, every frame, Full and Draft | 360 | 195840 | 100.0000% | 0 | 1.19e-7 | 1.79e-7 | 84 |
| Fixtures/rolling_shutter_repair, every frame, Full and Draft | 312 | 169728 | 100.0000% | 0 | 0.00e0 | 5.96e-8 | 147 |
| reference shot, Pixel Motion Blur as added, frames 0 to 210 every 30th, Full and Draft | 16 | 17625600 | 100.0000% | 0 | 1.79e-7 | 1.79e-7 | 16 |
| reference shot, Timewarp speed 50, frames 0 to 210 every 30th, Full and Draft | 16 | 17625600 | 100.0000% | 0 | 8.49e-7 | 8.49e-7 | 14 |
| reference shot, Warp Stabilizer with fx_ws_001's settings, frames 0 to 210 every 30th, Full and Draft | 16 | 17625600 | 100.0000% | 1 | 1.07e-6 | 1.07e-6 | 48 |
| reference shot, Rolling Shutter Repair, pixel motion, fx_rsr_001's settings, frames 0 to 210 every 30th, Full and Draft | 16 | 17625600 | 100.0000% | 0 | 2.68e-7 | 2.68e-7 | 15 |

Bars: motion within 1e-4 px (largest here 0.000e0); pictures at most 1 level on 99.9% of pixels, at most 2 anywhere, float on solid areas at most 1e-4. All hold: true.
