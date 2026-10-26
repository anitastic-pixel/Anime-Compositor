# B-758: time of the card motion finder (D-878)

Written by `tests/b758_card_motion_finder.rs` (`b758_timing`, ignored; run alone). 7 rounds, the processor's finder and the card's alternating, medians of the rounds. Frames 0 to 210 every 30th of the reference shot.

## The finder alone (ms per call)

| quality | vector detail | processor | card | faster |
|---|---:|---:|---:|---:|
| Full | 20 | 40.1 | 25.2 | 1.6x |
| Full | 30 | 36.3 | 30.5 | 1.2x |
| Full | 40 | 64.0 | 37.9 | 1.7x |
| Full | 50 | 65.2 | 37.9 | 1.7x |
| Full | 100 | 150.8 | 30.4 | 5.0x |
| Draft | 20 | 6.2 | 3.1 | 2.0x |
| Draft | 30 | 6.2 | 3.1 | 2.0x |
| Draft | 40 | 9.4 | 3.2 | 2.9x |
| Draft | 50 | 9.7 | 3.6 | 2.7x |
| Draft | 100 | 18.4 | 5.4 | 3.4x |

## The whole viewer frame on the card (ms per frame)

| effect | quality | processor's finder | card's finder | faster |
|---|---|---:|---:|---:|
| (none) | Full | 35.1 | 35.0 | 1.00x |
| (none) | Draft | 30.2 | 30.3 | 1.00x |
| Pixel Motion Blur as added (vector detail 20) | Full | 174.8 | 174.3 | 1.00x |
| Pixel Motion Blur as added (vector detail 20) | Draft | 64.1 | 65.5 | 0.98x |
| Pixel Motion Blur, vector detail 40 | Full | 250.7 | 213.0 | 1.18x |
| Pixel Motion Blur, vector detail 40 | Draft | 72.2 | 70.1 | 1.03x |
| Pixel Motion Blur, vector detail 100 | Full | 405.0 | 230.0 | 1.76x |
| Pixel Motion Blur, vector detail 100 | Draft | 88.4 | 71.9 | 1.23x |
| Timewarp speed 50 (vector detail 20) | Full | 182.5 | 182.6 | 1.00x |
| Timewarp speed 50 (vector detail 20) | Draft | 77.3 | 77.2 | 1.00x |
| Timewarp speed 50, vector detail 40 | Full | 269.1 | 238.8 | 1.13x |
| Timewarp speed 50, vector detail 40 | Draft | 70.9 | 68.7 | 1.03x |

The finder alone is asked of the card at every setting; in the viewer the card finds motion only at Draft and at Full from vector detail 40 (D-878), so the whole-frame rows at Full below 40 are the processor's finder both times.
