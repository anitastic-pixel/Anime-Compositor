# W7: every effect's time at 1920 by 1080

Wave 7, lane w7d, 2026-10-24. Every effect the core lists, at the setting it is added with and at
one heavy but usual setting, timed by `perf_audit_timing` (`app/src/tests/perf_audit.rs`): one
1920 by 1080 layer, the effect on it, frames 0 to 210 every 30th (8 frames), the median of them.

- **Machine:** NVIDIA GeForce RTX 4070 Ti SUPER; AMD64 Family 26 Model 68 processor, 24 threads;
  Windows 11 Education 10.0.26300.
- **Build:** `cargo test --release -p anime_compositor_app` (`CARGO_INCREMENTAL=0`), 440cecf0 for
  "before", 9d25bbef (B-660 to B-662) for "after".
- **Columns:** CPU = the processor's frame (`preview_frame_cached`; a Final Render makes the same
  frame, so CPU Full is the export's time); Viewer = Draw on: GPU (`preview_frame_srgb8`), which
  falls back to the processor for an effect with no card pass; 8 bpc = the viewer with 8 bpc
  (After Effects) on, which the card refuses (D-763), so every effect there is the processor's.
  "On the card" says which the viewer took at Draft and Full.
- **Load:** the machine was shared with other lanes building and testing. The full runs
  (03:55 and 04:16) had `rustc` running in 36 of 54 samples of the second; single values swing
  by tens of per cent (Pixel Motion Blur's viewer Full read 1663.9 ms in them, 190 ms in the
  quiet A/B rounds). The rows B-660 to B-662 touch were therefore timed again in 5 alternating
  rounds of the before and after binaries (05:19 to 05:43, `rustc` at the start of 4 of 10 runs);
  those rows say "A/B" and give those medians. The other rows are the median of the two full
  runs and are good for ranking, not for small differences.
- 369 rows; 221 of them are drawn on the card in the viewer.

## Top 20, before

| Rank | Effect | Setting | On the card (Draft/Full) | CPU Draft | CPU Full (Final Render) | Viewer Draft | Viewer Full | 8 bpc Draft | 8 bpc Full | From |
|---:|---|---|---|---:|---:|---:|---:|---:|---:|---|
| 1 | core.refine_soft_matte | edge_radius 40, feather 40, decontamination_radius 40 | CPU/CPU | 43.1 | 463.6 | 56.3 | 587.1 | 83.3 | 642.7 | A/B, 5 rounds |
| 2 | core.timewarp | vector_detail 40, smoothing_iterations 8, shutter_samples 10 | CPU/CPU | 51.7 | 442.3 | 70.3 | 436.5 | 79.0 | 450.3 | A/B, 5 rounds |
| 3 | core.cartoon | detail_radius 40, detail_threshold 20, shading_steps 16 | card/card | 13.0 | 422.9 | 19.7 | 17.5 | 41.0 | 447.1 | full runs |
| 4 | core.caustics | bottom_blur 40 | CPU/CPU | 12.1 | 371.5 | 26.6 | 394.8 | 33.3 | 415.1 | A/B, 5 rounds |
| 5 | core.bilateral_blur | radius 40 | card/card | 8.4 | 385.8 | 20.4 | 14.0 | 32.9 | 431.6 | full runs |
| 6 | core.refine_soft_matte | as added | CPU/CPU | 26.1 | 342.9 | 41.8 | 351.9 | 53.4 | 343.8 | A/B, 5 rounds |
| 7 | core.key_cleaner | additional_edge_radius 40 | CPU/CPU | 32.7 | 328.2 | 46.2 | 271.0 | 51.0 | 320.3 | A/B, 5 rounds |
| 8 | core.pixel_motion_blur | shutter_samples 32, vector_detail 40 | CPU/CPU | 41.3 | 273.4 | 48.2 | 313.4 | 56.8 | 352.2 | A/B, 5 rounds |
| 9 | core.matte_choker | geometric_softness_1 40, gray_level_softness_1 40, geometric_softness_2 40, iterations 2 | card/card | 13.8 | 303.1 | 38.2 | 55.1 | 37.0 | 288.3 | A/B, 5 rounds |
| 10 | core.key_cleaner | as added | CPU/CPU | 19.8 | 242.5 | 36.3 | 282.8 | 58.0 | 360.3 | A/B, 5 rounds |
| 11 | core.refine_hard_matte | as added | CPU/CPU | 6.1 | 243.1 | 22.9 | 276.9 | 37.2 | 264.5 | A/B, 5 rounds |
| 12 | core.contour_paint | smear_iterations 6 | card/card | 33.9 | 264.9 | 23.1 | 27.4 | 71.3 | 320.6 | full runs |
| 13 | core.refine_hard_matte | feather 40, decontamination_radius 40 | CPU/CPU | 19.7 | 225.0 | 36.7 | 246.1 | 40.5 | 242.7 | A/B, 5 rounds |
| 14 | core.timewarp | as added | CPU/CPU | 40.6 | 189.1 | 40.5 | 225.3 | 63.2 | 254.6 | A/B, 5 rounds |
| 15 | core.warp_stabilizer | as added | CPU/CPU | 33.2 | 148.6 | 34.9 | 203.8 | 123.5 | 252.9 | full runs |
| 16 | core.deband | edge_softness 40, blur_amount 40 | card/card | 11.7 | 195.3 | 21.8 | 18.2 | 48.5 | 203.6 | full runs |
| 17 | core.pixel_motion_blur | as added | CPU/CPU | 36.6 | 188.0 | 31.0 | 190.0 | 46.1 | 223.7 | A/B, 5 rounds |
| 18 | core.scatterize | as added | card/card | 7.8 | 84.8 | 37.3 | 189.8 | 31.6 | 166.9 | full runs |
| 19 | core.halation | radius 40 | card/card | 13.6 | 174.6 | 19.1 | 25.4 | 49.4 | 215.6 | full runs |
| 20 | core.cc_wide_time | forward_steps 6, backward_steps 6 | CPU/CPU | 42.6 | 150.6 | 53.9 | 169.7 | 69.3 | 199.7 | full runs |

## Top 20, after B-660 to B-662

| Rank | Effect | Setting | On the card (Draft/Full) | CPU Draft | CPU Full (Final Render) | Viewer Draft | Viewer Full | 8 bpc Draft | 8 bpc Full | From |
|---:|---|---|---|---:|---:|---:|---:|---:|---:|---|
| 1 | core.refine_soft_matte | edge_radius 40, feather 40, decontamination_radius 40 | CPU/CPU | 43.3 | 459.2 | 59.1 | 489.4 | 69.2 | 519.2 | A/B, 5 rounds |
| 2 | core.cartoon | detail_radius 40, detail_threshold 20, shading_steps 16 | card/card | 13.0 | 422.9 | 19.7 | 17.5 | 41.0 | 447.1 | full runs |
| 3 | core.timewarp | vector_detail 40, smoothing_iterations 8, shutter_samples 10 | CPU/CPU | 59.5 | 393.1 | 58.0 | 410.9 | 76.4 | 388.0 | A/B, 5 rounds |
| 4 | core.bilateral_blur | radius 40 | card/card | 8.4 | 385.8 | 20.4 | 14.0 | 32.9 | 431.6 | full runs |
| 5 | core.refine_soft_matte | as added | CPU/CPU | 40.0 | 350.9 | 42.9 | 348.1 | 52.5 | 340.4 | A/B, 5 rounds |
| 6 | core.caustics | bottom_blur 40 | CPU/CPU | 11.9 | 322.6 | 28.4 | 332.1 | 34.7 | 373.8 | A/B, 5 rounds |
| 7 | core.pixel_motion_blur | shutter_samples 32, vector_detail 40 | CPU/CPU | 36.5 | 259.1 | 59.8 | 288.9 | 57.9 | 294.6 | A/B, 5 rounds |
| 8 | core.refine_hard_matte | feather 40, decontamination_radius 40 | CPU/CPU | 24.2 | 243.4 | 45.2 | 274.0 | 55.5 | 297.7 | A/B, 5 rounds |
| 9 | core.contour_paint | smear_iterations 6 | card/card | 33.9 | 264.9 | 23.1 | 27.4 | 71.3 | 320.6 | full runs |
| 10 | core.refine_hard_matte | as added | CPU/CPU | 8.9 | 236.8 | 20.1 | 262.7 | 35.2 | 264.6 | A/B, 5 rounds |
| 11 | core.key_cleaner | additional_edge_radius 40 | CPU/CPU | 26.2 | 256.1 | 31.4 | 253.1 | 48.3 | 287.5 | A/B, 5 rounds |
| 12 | core.key_cleaner | as added | CPU/CPU | 23.3 | 232.9 | 31.2 | 240.8 | 46.1 | 284.0 | A/B, 5 rounds |
| 13 | core.matte_choker | geometric_softness_1 40, gray_level_softness_1 40, geometric_softness_2 40, iterations 2 | card/card | 14.0 | 232.9 | 40.2 | 57.2 | 38.0 | 274.7 | A/B, 5 rounds |
| 14 | core.warp_stabilizer | as added | CPU/CPU | 33.2 | 148.6 | 34.9 | 203.8 | 123.5 | 252.9 | full runs |
| 15 | core.deband | edge_softness 40, blur_amount 40 | card/card | 11.7 | 195.3 | 21.8 | 18.2 | 48.5 | 203.6 | full runs |
| 16 | core.scatterize | as added | card/card | 7.8 | 84.8 | 37.3 | 189.8 | 31.6 | 166.9 | full runs |
| 17 | core.halation | radius 40 | card/card | 13.6 | 174.6 | 19.1 | 25.4 | 49.4 | 215.6 | full runs |
| 18 | core.timewarp | as added | CPU/CPU | 38.8 | 159.7 | 40.0 | 170.2 | 46.4 | 197.1 | A/B, 5 rounds |
| 19 | core.cc_wide_time | forward_steps 6, backward_steps 6 | CPU/CPU | 42.6 | 150.6 | 53.9 | 169.7 | 69.3 | 199.7 | full runs |
| 20 | core.pixel_motion_blur | as added | CPU/CPU | 36.5 | 166.5 | 33.4 | 159.1 | 50.6 | 192.3 | A/B, 5 rounds |

## Why the heavy ones are slow

| Effect | Why | What was done |
|---|---|---|
| Pixel Motion Blur, Timewarp | Processor only; the motion finder (`flow::estimate`) builds a pyramid and solves per level, several per-pixel steps on one thread | B-662: those steps on all threads, 5 to 16% faster; a card motion finder proposed (D-783) |
| Caustics | Processor only; its Gaussian read down columns pixel by pixel, one cache line per tap | B-661: rows at a time, 10 to 13% faster; a running-box or FFT blur proposed (D-783) |
| Refine Soft / Hard Matte | Processor only; guided filter of 13 channels in f64, box sums memory-bound | B-660 box sums threaded; not measurably faster for these two |
| Key Cleaner | Processor only; same box sums | B-660: 4 to 22% faster |
| Matte Choker | Card in the viewer (20 to 57 ms); processor in export and 8 bpc (93 to 303 ms) | B-660: 23% faster on the processor |
| Cartoon, Bilateral Blur, Contour Paint, Deband, Halation, Find Edges, Fast Zoom Blur, Blobbylize | On the card in the viewer (14 to 60 ms); slow only where the processor draws them: export and 8 bpc | 8 bpc on the card proposed (D-783); an export on the card would change export bytes, which stay PROPOSED (earlier decisions) |
| Scatterize | The card is slower than the processor (190 against 85 ms at Full): its card pass works in f64 and uploads a 56 MB f64 list each frame | f32 on the card proposed (D-783) |
| Warp Stabilizer, CC Wide Time | Processor only; tracking and many-frame reads | Not changed |
| Curl Noise | On the card (64 to 65 ms viewer Full); processor 91 to 115 ms | Single precision proposed (D-783) |

A speed change was kept only when it was faster beyond the 6% the untouched `(none)` row moved
and every frame stayed the same bit for bit (`perf_audit_unchanged` against
`verification/W7_pixel_hashes.txt`, 369 of 369 rows). One more change was built and dropped: an
sRGB "way back" table (linear to display values by lookup for the 256 exact byte values) was bit
for bit the same but not measurably faster (8 bpc `(none)` 48.4 against 51.2 ms), so it is not in.

## Every effect, before

| Rank | Effect | Setting | On the card (Draft/Full) | CPU Draft | CPU Full (Final Render) | Viewer Draft | Viewer Full | 8 bpc Draft | 8 bpc Full | From |
|---:|---|---|---|---:|---:|---:|---:|---:|---:|---|
| 1 | core.refine_soft_matte | edge_radius 40, feather 40, decontamination_radius 40 | CPU/CPU | 43.1 | 463.6 | 56.3 | 587.1 | 83.3 | 642.7 | A/B, 5 rounds |
| 2 | core.timewarp | vector_detail 40, smoothing_iterations 8, shutter_samples 10 | CPU/CPU | 51.7 | 442.3 | 70.3 | 436.5 | 79.0 | 450.3 | A/B, 5 rounds |
| 3 | core.cartoon | detail_radius 40, detail_threshold 20, shading_steps 16 | card/card | 13.0 | 422.9 | 19.7 | 17.5 | 41.0 | 447.1 | full runs |
| 4 | core.caustics | bottom_blur 40 | CPU/CPU | 12.1 | 371.5 | 26.6 | 394.8 | 33.3 | 415.1 | A/B, 5 rounds |
| 5 | core.bilateral_blur | radius 40 | card/card | 8.4 | 385.8 | 20.4 | 14.0 | 32.9 | 431.6 | full runs |
| 6 | core.refine_soft_matte | as added | CPU/CPU | 26.1 | 342.9 | 41.8 | 351.9 | 53.4 | 343.8 | A/B, 5 rounds |
| 7 | core.key_cleaner | additional_edge_radius 40 | CPU/CPU | 32.7 | 328.2 | 46.2 | 271.0 | 51.0 | 320.3 | A/B, 5 rounds |
| 8 | core.pixel_motion_blur | shutter_samples 32, vector_detail 40 | CPU/CPU | 41.3 | 273.4 | 48.2 | 313.4 | 56.8 | 352.2 | A/B, 5 rounds |
| 9 | core.matte_choker | geometric_softness_1 40, gray_level_softness_1 40, geometric_softness_2 40, iterations 2 | card/card | 13.8 | 303.1 | 38.2 | 55.1 | 37.0 | 288.3 | A/B, 5 rounds |
| 10 | core.key_cleaner | as added | CPU/CPU | 19.8 | 242.5 | 36.3 | 282.8 | 58.0 | 360.3 | A/B, 5 rounds |
| 11 | core.refine_hard_matte | as added | CPU/CPU | 6.1 | 243.1 | 22.9 | 276.9 | 37.2 | 264.5 | A/B, 5 rounds |
| 12 | core.contour_paint | smear_iterations 6 | card/card | 33.9 | 264.9 | 23.1 | 27.4 | 71.3 | 320.6 | full runs |
| 13 | core.refine_hard_matte | feather 40, decontamination_radius 40 | CPU/CPU | 19.7 | 225.0 | 36.7 | 246.1 | 40.5 | 242.7 | A/B, 5 rounds |
| 14 | core.timewarp | as added | CPU/CPU | 40.6 | 189.1 | 40.5 | 225.3 | 63.2 | 254.6 | A/B, 5 rounds |
| 15 | core.warp_stabilizer | as added | CPU/CPU | 33.2 | 148.6 | 34.9 | 203.8 | 123.5 | 252.9 | full runs |
| 16 | core.deband | edge_softness 40, blur_amount 40 | card/card | 11.7 | 195.3 | 21.8 | 18.2 | 48.5 | 203.6 | full runs |
| 17 | core.pixel_motion_blur | as added | CPU/CPU | 36.6 | 188.0 | 31.0 | 190.0 | 46.1 | 223.7 | A/B, 5 rounds |
| 18 | core.scatterize | as added | card/card | 7.8 | 84.8 | 37.3 | 189.8 | 31.6 | 166.9 | full runs |
| 19 | core.halation | radius 40 | card/card | 13.6 | 174.6 | 19.1 | 25.4 | 49.4 | 215.6 | full runs |
| 20 | core.cc_wide_time | forward_steps 6, backward_steps 6 | CPU/CPU | 42.6 | 150.6 | 53.9 | 169.7 | 69.3 | 199.7 | full runs |
| 21 | core.find_edges | as added | card/card | 7.9 | 161.3 | 59.8 | 40.8 | 37.9 | 516.6 | full runs |
| 22 | core.fast_zoom_blur | as added | card/card | 8.4 | 158.2 | 13.4 | 23.8 | 118.9 | 215.2 | full runs |
| 23 | core.blobbylize | as added | card/card | 40.8 | 148.2 | 27.1 | 19.1 | 94.5 | 167.7 | full runs |
| 24 | core.halation | as added | card/card | 12.0 | 146.2 | 13.7 | 24.2 | 40.4 | 183.1 | full runs |
| 25 | core.shadow_highlight | as added | card/card | 9.0 | 142.2 | 12.9 | 31.1 | 49.6 | 132.4 | full runs |
| 26 | core.contour_paint | as added | card/card | 21.2 | 141.2 | 11.0 | 18.7 | 46.1 | 196.3 | full runs |
| 27 | core.light_burst | as added | card/card | 14.6 | 131.7 | 24.9 | 24.0 | 39.5 | 175.2 | full runs |
| 28 | core.caustics | as added | CPU/CPU | 8.4 | 108.8 | 23.0 | 126.8 | 35.1 | 169.9 | A/B, 5 rounds |
| 29 | core.brush_strokes | as added | card/card | 12.9 | 123.6 | 36.5 | 27.7 | 41.4 | 150.2 | full runs |
| 30 | core.remove_grain | passes 2, unsharp_radius 40 | card/card | 36.6 | 122.8 | 19.4 | 27.8 | 122.6 | 139.8 | full runs |
| 31 | core.color_stabilizer | as added | CPU/CPU | 67.1 | 121.0 | 28.4 | 18.6 | 42.5 | 92.6 | full runs |
| 32 | core.bloom | as added | card/card | 14.8 | 117.4 | 10.2 | 22.7 | 43.6 | 146.8 | full runs |
| 33 | core.brush_strokes | stroke_length 40 | card/card | 13.5 | 117.3 | 28.1 | 29.1 | 39.5 | 169.8 | full runs |
| 34 | core.curl_noise | as added | card/card | 13.7 | 115.2 | 11.5 | 65.1 | 52.2 | 147.7 | full runs |
| 35 | core.rolling_shutter_repair | as added | CPU/CPU | 19.6 | 112.8 | 23.1 | 81.6 | 44.5 | 107.6 | full runs |
| 36 | core.radial_blur | as added | card/card | 7.0 | 112.3 | 8.0 | 13.8 | 33.6 | 148.6 | full runs |
| 37 | core.shatter | as added | CPU/CPU | 13.2 | 110.2 | 18.1 | 104.7 | 35.5 | 132.9 | full runs |
| 38 | core.bloom | radius 40 | card/card | 13.2 | 107.0 | 17.6 | 29.1 | 37.0 | 140.1 | full runs |
| 39 | core.deband | as added | card/card | 10.6 | 105.0 | 33.0 | 14.0 | 38.7 | 159.1 | full runs |
| 40 | core.shadow_highlight | shadow_radius 40, highlight_radius 40 | card/card | 6.5 | 99.8 | 14.2 | 18.8 | 32.1 | 126.4 | full runs |
| 41 | core.remove_grain | as added | card/card | 76.6 | 97.2 | 31.2 | 44.0 | 153.8 | 402.7 | full runs |
| 42 | core.tiles | as added | card/card | 21.3 | 95.6 | 14.2 | 26.1 | 84.7 | 148.1 | full runs |
| 43 | core.cc_wide_time | as added | CPU/CPU | 35.9 | 93.8 | 27.2 | 95.2 | 48.1 | 317.4 | full runs |
| 44 | core.turbulent_displace | complexity 4 | card/card | 8.8 | 94.4 | 11.5 | 19.6 | 33.8 | 133.6 | full runs |
| 45 | core.matte_choker | as added | card/card | 12.7 | 93.4 | 38.0 | 20.3 | 33.1 | 110.5 | A/B, 5 rounds |
| 46 | core.curl_noise | softness 40, flow_softness 40 | card/card | 8.2 | 90.8 | 19.1 | 64.1 | 33.9 | 125.0 | full runs |
| 47 | core.change_color | as added | CPU/CPU | 9.7 | 90.8 | 9.8 | 17.6 | 74.4 | 100.8 | full runs |
| 48 | core.cartoon | as added | card/card | 12.0 | 90.5 | 16.3 | 25.4 | 46.4 | 151.3 | full runs |
| 49 | core.glass | as added | card/card | 24.2 | 89.6 | 17.1 | 24.9 | 66.3 | 132.7 | full runs |
| 50 | core.spin_zoom_blur | quality 100 | card/card | 8.7 | 86.6 | 13.4 | 20.7 | 87.2 | 139.3 | full runs |
| 51 | core.screen_key | softness 40 | CPU/CPU | 7.8 | 73.2 | 12.7 | 85.6 | 36.5 | 106.8 | full runs |
| 52 | core.rolling_shutter_repair | pixel_motion_detail 40 | CPU/CPU | 10.7 | 83.2 | 26.7 | 84.5 | 34.7 | 104.5 | full runs |
| 53 | core.blobbylize | softness 40 | card/card | 14.0 | 81.2 | 11.8 | 23.2 | 44.9 | 112.0 | full runs |
| 54 | core.lens_blur | as added | card/card | 8.7 | 78.4 | 17.1 | 46.1 | 85.0 | 212.4 | full runs |
| 55 | core.paint_bucket | as added | card/card | 8.0 | 77.7 | 8.7 | 34.7 | 31.0 | 116.3 | full runs |
| 56 | core.lens_blur | radius 40 | card/card | 11.3 | 76.5 | 41.4 | 31.8 | 49.1 | 117.8 | full runs |
| 57 | core.paint_bucket | spread_radius 40, feather_softness 40 | card/card | 9.9 | 76.1 | 16.8 | 31.4 | 35.9 | 114.8 | full runs |
| 58 | core.light_rays | as added | card/card | 19.2 | 75.3 | 20.2 | 22.9 | 59.1 | 132.9 | full runs |
| 59 | core.glass | softness 40 | card/card | 9.8 | 74.8 | 16.6 | 13.2 | 36.0 | 98.0 | full runs |
| 60 | core.turbulent_displace | as added | card/card | 7.7 | 71.2 | 7.6 | 14.7 | 33.9 | 100.5 | full runs |
| 61 | core.split | as added | card/card | 9.7 | 70.5 | 9.9 | 15.9 | 48.5 | 137.8 | full runs |
| 62 | core.bender | as added | card/card | 9.1 | 70.0 | 22.9 | 20.8 | 53.0 | 146.4 | full runs |
| 63 | core.wave_world | as added | CPU/CPU | 24.2 | 67.7 | 32.6 | 57.1 | 72.8 | 154.5 | full runs |
| 64 | core.detail_upscale | as added | card/card | 9.2 | 66.9 | 7.8 | 13.3 | 35.2 | 98.6 | full runs |
| 65 | core.moment_map | as added | CPU/CPU | 15.2 | 55.6 | 23.2 | 65.8 | 45.5 | 101.2 | full runs |
| 66 | core.reduce_interlace_flicker | softness 40 | CPU/CPU | 4.9 | 60.1 | 11.5 | 65.7 | 30.5 | 95.8 | full runs |
| 67 | core.vector_blur | map_softness 40 | card/card | 8.8 | 65.3 | 15.5 | 14.8 | 47.9 | 119.9 | full runs |
| 68 | core.light_wrap | as added | CPU/CPU | 8.1 | 64.5 | 11.0 | 32.0 | 32.5 | 100.0 | full runs |
| 69 | core.burn_film | as added | CPU/CPU | 6.1 | 63.4 | 7.5 | 9.5 | 28.1 | 88.7 | full runs |
| 70 | core.card_wipe | as added | card/card | 5.4 | 63.0 | 15.2 | 26.2 | 45.0 | 112.2 | full runs |
| 71 | core.cross_glare | as added | card/card | 7.0 | 62.5 | 7.0 | 15.5 | 31.7 | 86.4 | full runs |
| 72 | core.card_dance | as added | CPU/CPU | 11.2 | 61.2 | 15.1 | 58.0 | 38.0 | 98.5 | full runs |
| 73 | core.magnify | feather 40 | card/card | 28.7 | 60.7 | 23.6 | 41.1 | 110.7 | 106.9 | full runs |
| 74 | core.radial_shadow | softness 40 | card/card | 11.1 | 60.1 | 7.6 | 16.8 | 41.0 | 110.1 | full runs |
| 75 | core.diffusion | radius 40 | card/card | 7.5 | 46.5 | 21.2 | 58.6 | 69.3 | 85.2 | full runs |
| 76 | core.split_2 | as added | card/card | 11.6 | 57.5 | 9.0 | 20.2 | 169.5 | 190.2 | full runs |
| 77 | core.drop_shadow | softness 40 | card/card | 12.0 | 57.3 | 18.6 | 16.6 | 44.5 | 121.5 | full runs |
| 78 | core.beam | as added | card/card | 8.9 | 57.1 | 8.4 | 16.9 | 41.2 | 93.2 | full runs |
| 79 | core.sharpen | radius 40 | card/card | 12.0 | 56.9 | 11.1 | 13.6 | 49.1 | 113.7 | full runs |
| 80 | core.cross_blur | as added | card/card | 9.2 | 56.9 | 16.1 | 11.3 | 33.8 | 97.3 | full runs |
| 81 | core.vector_blur | as added | card/card | 7.6 | 56.8 | 8.0 | 17.4 | 38.8 | 116.6 | full runs |
| 82 | core.detail_upscale | detail 40 | card/card | 8.9 | 56.7 | 6.9 | 11.9 | 57.5 | 128.0 | full runs |
| 83 | core.outline | softness 40 | card/card | 8.6 | 56.3 | 9.9 | 12.1 | 37.5 | 85.2 | full runs |
| 84 | core.cross_blur | radius_x 40, radius_y 40 | card/card | 8.1 | 56.3 | 7.1 | 17.2 | 33.4 | 97.5 | full runs |
| 85 | core.lens_flare | as added | card/card | 6.3 | 56.1 | 10.5 | 24.1 | 32.8 | 119.1 | full runs |
| 86 | core.sharpen | as added | card/card | 19.4 | 55.4 | 8.9 | 14.1 | 43.0 | 104.4 | full runs |
| 87 | core.timecode | as added | card/card | 7.9 | 53.8 | 12.7 | 54.4 | 32.5 | 84.8 | full runs |
| 88 | core.magnify | as added | card/card | 11.8 | 54.2 | 8.4 | 13.2 | 219.0 | 599.5 | full runs |
| 89 | core.ripple_pulse | as added | CPU/CPU | 44.3 | 51.4 | 8.7 | 34.2 | 49.9 | 89.0 | full runs |
| 90 | core.spin_zoom_blur | as added | card/card | 6.7 | 51.2 | 10.7 | 21.8 | 33.2 | 85.0 | full runs |
| 91 | core.line_blur | as added | card/card | 8.2 | 50.9 | 8.9 | 13.7 | 33.9 | 79.9 | full runs |
| 92 | core.bilateral_blur | as added | card/card | 6.0 | 50.9 | 7.6 | 11.8 | 32.6 | 86.5 | full runs |
| 93 | core.chromatic_aberration | fringe_blur 40 | card/card | 6.4 | 50.9 | 11.3 | 28.8 | 36.0 | 145.9 | full runs |
| 94 | core.bend_it | as added | card/card | 7.4 | 50.5 | 7.0 | 17.2 | 29.9 | 78.4 | full runs |
| 95 | core.gaussian_blur_legacy | blurriness 40 | card/card | 7.1 | 50.3 | 20.4 | 13.7 | 42.0 | 107.3 | full runs |
| 96 | core.outline | as added | card/card | 6.8 | 50.1 | 12.8 | 24.4 | 40.4 | 82.1 | full runs |
| 97 | core.spherize | as added | card/card | 10.1 | 50.0 | 18.4 | 13.8 | 53.4 | 104.3 | full runs |
| 98 | core.fast_box_blur | radius 40, iterations 6 | card/card | 6.2 | 49.9 | 10.1 | 19.9 | 31.0 | 87.0 | full runs |
| 99 | core.drop_shadow | as added | card/card | 6.3 | 49.6 | 7.0 | 12.6 | 36.3 | 86.7 | full runs |
| 100 | core.diffusion | as added | card/card | 11.3 | 49.2 | 13.8 | 13.8 | 43.7 | 101.5 | full runs |
| 101 | core.rim_light | softness 40 | card/card | 8.3 | 48.5 | 13.1 | 24.1 | 41.6 | 95.9 | full runs |
| 102 | core.bevel_alpha | as added | card/card | 6.5 | 48.5 | 10.3 | 12.8 | 31.5 | 86.3 | full runs |
| 103 | core.gaussian_blur_legacy | as added | CPU/CPU | 8.0 | 48.3 | 14.6 | 23.4 | 115.4 | 131.1 | full runs |
| 104 | core.flow_motion | as added | card/card | 6.8 | 48.2 | 10.9 | 12.5 | 31.4 | 73.2 | full runs |
| 105 | core.star_burst | as added | card/card | 11.9 | 48.1 | 12.7 | 22.4 | 30.7 | 65.8 | full runs |
| 106 | core.cc_rainfall | as added | CPU/CPU | 8.6 | 39.6 | 25.8 | 47.9 | 49.9 | 84.9 | full runs |
| 107 | core.rim_light | as added | card/card | 8.3 | 47.8 | 16.0 | 30.6 | 38.8 | 93.8 | full runs |
| 108 | core.glow | radius 40 | card/card | 7.2 | 47.6 | 7.0 | 15.3 | 32.6 | 88.9 | full runs |
| 109 | core.echo | as added | CPU/CPU | 12.5 | 47.1 | 9.4 | 38.5 | 33.2 | 86.1 | full runs |
| 110 | core.bulge | as added | card/card | 4.9 | 46.7 | 13.6 | 15.3 | 49.6 | 123.8 | full runs |
| 111 | core.radial_shadow | as added | card/card | 9.1 | 46.6 | 7.3 | 13.6 | 36.0 | 89.6 | full runs |
| 112 | core.hair | length 40 | card/card | 10.4 | 46.2 | 10.5 | 30.4 | 32.0 | 78.4 | full runs |
| 113 | core.checkerboard | feather_width 40, feather_height 40 | card/card | 7.8 | 46.0 | 13.6 | 29.4 | 47.9 | 120.0 | full runs |
| 114 | core.color_link | as added | CPU/CPU | 7.3 | 45.9 | 7.2 | 30.4 | 37.0 | 114.9 | full runs |
| 115 | core.fast_blur_legacy | blurriness 40 | card/card | 5.9 | 44.7 | 12.2 | 16.6 | 49.2 | 137.2 | full runs |
| 116 | core.channel_blur | red_blurriness 40, green_blurriness 40, blue_blurriness 40, alpha_blurriness 40 | card/card | 5.7 | 44.5 | 7.4 | 15.3 | 30.4 | 85.0 | full runs |
| 117 | core.line_blur | length 40 | card/card | 6.8 | 44.3 | 10.4 | 23.2 | 30.2 | 82.2 | full runs |
| 118 | core.roughen_edges | as added | card/card | 7.2 | 44.2 | 8.7 | 14.4 | 37.3 | 84.7 | full runs |
| 119 | core.median | as added | CPU/card | 5.4 | 44.1 | 5.3 | 31.2 | 30.0 | 80.0 | full runs |
| 120 | core.kaleidoscope | as added | CPU/CPU | 5.8 | 44.0 | 7.1 | 32.5 | 47.0 | 98.0 | full runs |
| 121 | core.threads | as added | card/card | 6.2 | 43.7 | 12.5 | 23.9 | 46.2 | 76.4 | full runs |
| 122 | core.cc_rainfall | spread 40 | CPU/CPU | 5.5 | 34.6 | 16.4 | 43.2 | 39.2 | 86.3 | full runs |
| 123 | core.color_grade | key_hue_softness 40, key_softness 40, key_blur 40 | CPU/CPU | 15.4 | 43.1 | 9.9 | 19.1 | 35.5 | 144.8 | full runs |
| 124 | core.glow | as added | card/card | 6.2 | 43.0 | 10.4 | 27.9 | 31.2 | 84.2 | full runs |
| 125 | core.polygon_shards | as added | card/card | 9.9 | 43.0 | 9.4 | 23.5 | 33.0 | 74.7 | full runs |
| 126 | core.hair | as added | card/card | 9.3 | 42.9 | 12.9 | 19.1 | 34.8 | 74.3 | full runs |
| 127 | core.gaussian_blur | sigma_px 40 | card/card | 5.5 | 42.2 | 7.9 | 14.7 | 34.5 | 81.3 | full runs |
| 128 | core.slant | as added | CPU/CPU | 6.3 | 41.5 | 36.6 | 10.3 | 38.8 | 88.9 | full runs |
| 129 | core.roughen_edges | complexity 6 | card/card | 6.6 | 41.2 | 8.0 | 13.1 | 41.1 | 127.1 | full runs |
| 130 | core.ripple | wavelength 40 | card/card | 5.7 | 41.1 | 11.4 | 20.8 | 34.3 | 97.8 | full runs |
| 131 | core.distance_gradation | as added | card/card | 9.7 | 41.0 | 21.6 | 23.4 | 80.4 | 131.2 | full runs |
| 132 | core.glue_gun | as added | card/card | 6.1 | 40.9 | 15.0 | 15.2 | 37.1 | 75.2 | full runs |
| 133 | core.mr_smoothie | as added | card/card | 7.6 | 40.6 | 7.3 | 13.1 | 41.3 | 96.2 | full runs |
| 134 | core.plastic | softness 40 | card/card | 6.5 | 40.5 | 6.8 | 13.3 | 30.6 | 80.5 | full runs |
| 135 | core.fractal | as added | card/card | 7.8 | 40.3 | 8.4 | 19.8 | 32.8 | 76.7 | full runs |
| 136 | core.noise_hls | as added | card/card | 7.0 | 40.1 | 14.1 | 17.3 | 134.1 | 85.9 | full runs |
| 137 | core.audio_spectrum | as added | card/card | 8.2 | 40.0 | 8.8 | 15.9 | 37.6 | 72.0 | full runs |
| 138 | core.plastic | as added | card/card | 5.7 | 40.0 | 15.0 | 20.0 | 32.2 | 73.3 | full runs |
| 139 | core.cc_cylinder | as added | card/card | 5.0 | 40.0 | 10.9 | 14.6 | 27.4 | 71.2 | full runs |
| 140 | core.gamma_pedestal_gain | as added | CPU/CPU | 5.5 | 39.5 | 8.3 | 13.1 | 38.1 | 82.0 | full runs |
| 141 | core.bulge | vertical_radius 40, taper_radius 40 | CPU/CPU | 8.7 | 39.2 | 16.2 | 32.0 | 34.6 | 82.2 | full runs |
| 142 | core.directional_blur | as added | card/card | 5.5 | 39.1 | 12.0 | 21.7 | 29.7 | 72.7 | full runs |
| 143 | core.fisheye | as added | card/card | 4.8 | 38.8 | 10.2 | 12.1 | 28.5 | 72.2 | full runs |
| 144 | core.wave_warp | as added | card/card | 6.0 | 38.7 | 19.3 | 13.8 | 31.3 | 74.7 | full runs |
| 145 | core.cc_sphere | as added | card/card | 4.8 | 38.6 | 7.4 | 13.4 | 27.1 | 67.7 | full runs |
| 146 | core.fractal | cycle_steps 20 | card/card | 5.7 | 38.4 | 9.2 | 20.9 | 36.0 | 80.2 | full runs |
| 147 | core.broadcast_safe | as added | card/card | 5.3 | 38.4 | 15.2 | 16.9 | 78.8 | 93.2 | full runs |
| 148 | core.ripple | as added | card/card | 6.2 | 38.2 | 7.2 | 12.0 | 33.5 | 91.5 | full runs |
| 149 | core.smear | as added | card/card | 7.5 | 38.2 | 7.8 | 13.4 | 53.9 | 122.5 | full runs |
| 150 | core.twirl | as added | card/card | 4.9 | 38.0 | 10.6 | 14.2 | 31.9 | 80.5 | full runs |
| 151 | core.warp | as added | card/card | 6.7 | 37.9 | 8.1 | 14.2 | 31.8 | 74.7 | full runs |
| 152 | core.color_balance_hls | as added | CPU/CPU | 4.5 | 37.7 | 7.2 | 13.7 | 36.2 | 99.6 | full runs |
| 153 | core.dust_scratches | as added | CPU/card | 3.7 | 37.6 | 4.7 | 31.9 | 29.1 | 72.6 | full runs |
| 154 | core.page_turn | as added | card/card | 5.4 | 37.4 | 7.2 | 11.9 | 29.4 | 70.2 | full runs |
| 155 | core.page_turn | fold_radius 40 | card/card | 5.0 | 37.1 | 6.5 | 11.8 | 27.6 | 69.4 | full runs |
| 156 | core.chromatic_aberration | as added | card/card | 4.7 | 37.0 | 11.6 | 12.2 | 35.2 | 85.4 | full runs |
| 157 | core.spread_tones | as added | CPU/CPU | 4.1 | 34.4 | 12.8 | 37.0 | 42.2 | 79.2 | full runs |
| 158 | core.drizzle | as added | card/card | 4.6 | 36.5 | 7.2 | 15.1 | 26.8 | 67.9 | full runs |
| 159 | core.griddler | as added | card/card | 5.0 | 36.5 | 7.2 | 13.3 | 32.6 | 70.3 | full runs |
| 160 | core.directional_blur | length 40 | card/card | 5.2 | 36.3 | 7.8 | 11.2 | 39.0 | 78.2 | full runs |
| 161 | core.smart_blur | as added | CPU/card | 4.8 | 36.2 | 5.3 | 11.6 | 28.4 | 77.0 | full runs |
| 162 | core.exposure_flicker | as added | card/card | 6.5 | 36.0 | 14.1 | 23.1 | 37.0 | 70.3 | full runs |
| 163 | core.mirror | as added | card/card | 5.6 | 36.0 | 15.4 | 11.7 | 32.8 | 73.9 | full runs |
| 164 | core.change_color | softness 40 | CPU/CPU | 5.2 | 35.8 | 9.0 | 9.8 | 33.7 | 76.8 | full runs |
| 165 | core.line_smooth | as added | card/card | 5.4 | 35.8 | 8.7 | 24.1 | 27.5 | 71.2 | full runs |
| 166 | core.halftone | as added | card/card | 7.6 | 35.8 | 9.6 | 15.7 | 38.9 | 67.7 | full runs |
| 167 | core.light_sweep | as added | card/card | 4.9 | 35.8 | 7.9 | 10.8 | 30.1 | 69.1 | full runs |
| 168 | core.polar_coordinates | as added | card/card | 4.7 | 35.5 | 6.9 | 12.9 | 28.9 | 76.2 | full runs |
| 169 | core.speed_lines | as added | card/card | 8.4 | 35.0 | 11.7 | 17.5 | 32.0 | 68.8 | full runs |
| 170 | core.foam | as added | card/card | 9.6 | 34.9 | 11.7 | 16.6 | 34.3 | 69.7 | full runs |
| 171 | core.beam | length 40 | card/card | 6.6 | 34.9 | 7.3 | 13.1 | 37.1 | 76.4 | full runs |
| 172 | core.three_d_glasses | as added | card/card | 4.8 | 34.8 | 8.4 | 14.8 | 29.1 | 70.6 | full runs |
| 173 | core.colorama | as added | card/card | 6.1 | 34.8 | 8.6 | 14.6 | 40.5 | 72.2 | full runs |
| 174 | core.noise_alpha | as added | card/card | 5.8 | 34.6 | 8.2 | 13.1 | 43.4 | 75.2 | full runs |
| 175 | core.scatter | as added | card/card | 5.0 | 34.6 | 10.4 | 21.4 | 28.6 | 66.2 | full runs |
| 176 | core.grid | as added | card/card | 6.2 | 34.5 | 7.6 | 12.6 | 33.8 | 68.5 | full runs |
| 177 | core.color_neutralizer | as added | CPU/CPU | 4.7 | 34.5 | 6.3 | 10.5 | 35.0 | 80.4 | full runs |
| 178 | core.cc_environment | as added | CPU/CPU | 5.7 | 34.5 | 10.1 | 28.4 | 28.3 | 58.0 | full runs |
| 179 | core.depth_key | feather 40 | CPU/CPU | 5.9 | 34.2 | 9.4 | 19.9 | 40.2 | 85.9 | full runs |
| 180 | core.lightning_bolt | as added | card/card | 4.9 | 34.2 | 7.9 | 19.3 | 42.0 | 89.2 | full runs |
| 181 | core.time_difference | as added | CPU/CPU | 6.0 | 34.2 | 13.8 | 30.1 | 29.4 | 70.5 | full runs |
| 182 | core.line_width | as added | card/card | 4.6 | 34.1 | 11.3 | 22.1 | 36.8 | 68.5 | full runs |
| 183 | core.photo_filter | as added | card/card | 5.6 | 34.1 | 7.8 | 13.8 | 34.8 | 107.8 | full runs |
| 184 | core.toner | as added | card/card | 4.6 | 34.0 | 10.2 | 17.1 | 47.5 | 372.8 | full runs |
| 185 | core.reduce_interlace_flicker | as added | CPU/CPU | 5.2 | 33.8 | 9.7 | 29.8 | 30.6 | 74.2 | full runs |
| 186 | core.simple_wire_removal | as added | card/card | 5.7 | 33.3 | 13.0 | 22.1 | 27.8 | 62.1 | full runs |
| 187 | core.twirl | radius 40, twirl_radius 40 | card/card | 5.0 | 33.1 | 10.5 | 13.9 | 33.0 | 69.5 | full runs |
| 188 | core.rain | as added | card/card | 4.3 | 33.1 | 7.3 | 12.8 | 47.6 | 70.2 | full runs |
| 189 | core.basic_text | as added | card/card | 4.2 | 32.9 | 10.5 | 21.3 | 27.2 | 64.8 | full runs |
| 190 | core.snowfall | as added | card/card | 6.5 | 32.9 | 8.9 | 12.4 | 36.6 | 80.2 | full runs |
| 191 | core.emboss | as added | card/card | 5.8 | 32.8 | 7.3 | 11.4 | 35.5 | 274.9 | full runs |
| 192 | core.numbers | as added | card/card | 4.1 | 32.8 | 10.7 | 30.6 | 27.6 | 66.3 | full runs |
| 193 | core.circle | as added | card/card | 8.8 | 32.6 | 17.6 | 23.2 | 50.5 | 81.2 | full runs |
| 194 | core.cc_vignette | as added | card/card | 4.3 | 27.0 | 16.1 | 32.6 | 43.4 | 65.7 | full runs |
| 195 | core.grid | feather_width 40, feather_height 40 | card/card | 4.3 | 32.5 | 9.8 | 12.1 | 31.9 | 66.8 | full runs |
| 196 | core.cc_spotlight | edge_softness 40 | card/card | 5.7 | 32.5 | 9.7 | 14.3 | 27.4 | 57.8 | full runs |
| 197 | core.camera_shake | as added | card/card | 4.8 | 32.4 | 7.0 | 12.6 | 31.6 | 82.4 | full runs |
| 198 | core.depth_key | as added | CPU/CPU | 5.4 | 32.4 | 7.4 | 9.8 | 38.4 | 80.0 | full runs |
| 199 | core.id_key | feather 40 | CPU/CPU | 5.2 | 32.4 | 10.8 | 17.2 | 31.4 | 66.8 | full runs |
| 200 | core.extract | black_softness 40, white_softness 40 | card/card | 4.6 | 32.3 | 8.2 | 15.1 | 36.9 | 75.4 | full runs |
| 201 | core.circle | feather_outer 40, feather_inner 40 | card/card | 5.1 | 32.3 | 15.5 | 11.8 | 35.6 | 70.2 | full runs |
| 202 | core.channel_blur | as added | CPU/CPU | 4.2 | 32.2 | 4.8 | 8.6 | 29.2 | 68.1 | full runs |
| 203 | core.ellipse | as added | card/card | 5.2 | 32.2 | 7.7 | 12.2 | 38.0 | 74.2 | full runs |
| 204 | core.bubbles | as added | card/card | 4.6 | 32.2 | 7.8 | 11.9 | 27.9 | 65.4 | full runs |
| 205 | core.extract | as added | card/card | 4.8 | 32.1 | 14.1 | 13.9 | 37.5 | 72.9 | full runs |
| 206 | core.id_key | as added | CPU/CPU | 5.1 | 32.1 | 17.9 | 24.6 | 35.8 | 80.1 | full runs |
| 207 | core.stretch_wipe | as added | CPU/CPU | 5.0 | 32.1 | 8.2 | 16.4 | 27.8 | 60.5 | full runs |
| 208 | core.optics_compensation | as added | CPU/CPU | 4.4 | 32.0 | 6.2 | 10.3 | 29.0 | 67.9 | full runs |
| 209 | core.cell_pattern | as added | card/card | 5.3 | 31.9 | 7.8 | 11.5 | 41.0 | 76.0 | full runs |
| 210 | core.checkerboard | as added | card/card | 5.9 | 31.6 | 8.0 | 15.4 | 45.2 | 117.2 | full runs |
| 211 | core.radio_waves | as added | card/card | 4.2 | 31.6 | 10.3 | 22.3 | 28.7 | 65.0 | full runs |
| 212 | core.reshape | as added | CPU/CPU | 4.8 | 31.6 | 12.4 | 10.1 | 30.6 | 65.4 | full runs |
| 213 | core.kira_kira | as added | card/card | 5.0 | 31.6 | 10.6 | 21.7 | 33.2 | 77.0 | full runs |
| 214 | core.spill_suppressor | as added | card/card | 4.2 | 31.6 | 22.1 | 13.6 | 28.6 | 64.5 | full runs |
| 215 | core.colorama | matching_softness 40 | card/card | 4.7 | 31.5 | 11.6 | 13.3 | 32.2 | 71.3 | full runs |
| 216 | core.shift_channels | as added | CPU/CPU | 4.6 | 31.4 | 5.7 | 9.2 | 27.9 | 65.2 | full runs |
| 217 | core.force_motion_blur | as added | CPU/CPU | 5.7 | 31.1 | 15.6 | 20.0 | 38.6 | 75.5 | full runs |
| 218 | core.turbulent_noise | complexity 12 | card/card | 4.2 | 31.0 | 10.8 | 21.0 | 28.6 | 62.1 | full runs |
| 219 | core.block_dissolve | as added | CPU/CPU | 4.2 | 31.0 | 12.1 | 17.2 | 33.1 | 66.6 | full runs |
| 220 | core.linear_color_key | as added | card/card | 4.2 | 31.0 | 18.1 | 14.5 | 32.0 | 70.0 | full runs |
| 221 | core.color_grade | as added | CPU/CPU | 5.1 | 30.9 | 5.8 | 8.8 | 35.1 | 80.2 | full runs |
| 222 | core.hex_tile | as added | card/card | 4.2 | 30.6 | 10.6 | 22.0 | 26.6 | 68.2 | full runs |
| 223 | core.paraffin | as added | card/card | 4.4 | 30.5 | 7.1 | 16.2 | 28.8 | 60.9 | full runs |
| 224 | core.kernel | as added | CPU/CPU | 5.5 | 30.5 | 5.4 | 9.8 | 35.4 | 72.5 | full runs |
| 225 | core.pass_extract | as added | CPU/CPU | 5.0 | 30.3 | 15.5 | 18.4 | 33.8 | 74.2 | full runs |
| 226 | core.color_offset | as added | CPU/CPU | 5.3 | 30.1 | 6.7 | 12.8 | 35.5 | 95.8 | full runs |
| 227 | core.four_color_gradient | as added | card/card | 4.5 | 30.1 | 6.8 | 11.9 | 45.2 | 75.3 | full runs |
| 228 | core.vegas | as added | CPU/CPU | 4.5 | 30.1 | 6.2 | 10.1 | 38.5 | 69.3 | full runs |
| 229 | core.cc_spotlight | as added | card/card | 4.2 | 30.1 | 7.0 | 11.8 | 28.2 | 84.6 | full runs |
| 230 | core.selective_color | as added | CPU/CPU | 5.2 | 30.1 | 7.8 | 9.1 | 34.8 | 79.5 | full runs |
| 231 | core.inner_outer_key | single_mask_highlight_radius 40, edge_feather 40 | CPU/CPU | 5.9 | 30.1 | 6.4 | 18.0 | 36.5 | 75.0 | full runs |
| 232 | core.stretch_levels | as added | CPU/CPU | 4.7 | 28.5 | 12.4 | 30.0 | 27.9 | 60.1 | full runs |
| 233 | core.angle_control | as added | CPU/CPU | 4.8 | 30.0 | 5.0 | 14.9 | 26.9 | 60.5 | full runs |
| 234 | core.fill | horizontal_feather 40, vertical_feather 40 | card/card | 4.8 | 29.9 | 9.8 | 10.2 | 27.9 | 61.3 | full runs |
| 235 | core.linear_color_key | matching_softness 40 | card/card | 5.0 | 29.9 | 9.2 | 16.8 | 32.1 | 65.2 | full runs |
| 236 | core.solid_composite | as added | card/card | 4.4 | 29.7 | 11.4 | 16.1 | 30.3 | 79.8 | full runs |
| 237 | core.radial_wipe | feather 40 | CPU/CPU | 5.5 | 29.6 | 6.2 | 8.9 | 30.4 | 68.0 | full runs |
| 238 | core.mesh_warp | as added | CPU/CPU | 4.3 | 29.6 | 8.0 | 10.5 | 32.8 | 59.8 | full runs |
| 239 | core.arbitrary_map | as added | CPU/CPU | 6.1 | 29.6 | 16.6 | 15.9 | 28.6 | 64.7 | full runs |
| 240 | core.mask_smear | as added | CPU/CPU | 4.6 | 29.6 | 8.1 | 9.8 | 31.1 | 89.9 | full runs |
| 241 | core.particle_playground | as added | card/card | 4.2 | 29.6 | 10.3 | 19.8 | 26.3 | 59.7 | full runs |
| 242 | core.noise_hls_auto | as added | card/card | 4.0 | 29.3 | 9.9 | 21.8 | 27.4 | 65.3 | full runs |
| 243 | core.leave_color | softness 40 | card/card | 5.0 | 29.3 | 6.7 | 13.1 | 30.6 | 59.7 | full runs |
| 244 | core.change_to_color | as added | card/card | 4.5 | 29.2 | 10.7 | 12.8 | 27.1 | 62.6 | full runs |
| 245 | core.bevel_edges | as added | card/card | 6.2 | 29.1 | 7.5 | 13.8 | 29.5 | 67.1 | full runs |
| 246 | core.audio_waveform | displayed_samples 64 | card/card | 4.4 | 29.1 | 6.7 | 11.0 | 30.1 | 70.5 | full runs |
| 247 | core.warp_blend | as added | CPU/CPU | 3.7 | 29.1 | 5.0 | 11.0 | 29.9 | 61.5 | full runs |
| 248 | core.rain | length 40 | card/card | 5.2 | 29.1 | 7.4 | 11.3 | 31.8 | 68.8 | full runs |
| 249 | core.screen_key | as added | card/card | 4.1 | 29.1 | 10.8 | 14.2 | 27.3 | 63.5 | full runs |
| 250 | core.fractal_noise | as added | card/card | 4.3 | 28.9 | 14.6 | 23.9 | 29.2 | 63.7 | full runs |
| 251 | core.mercury | radius_x 40, radius_y 40 | card/card | 3.9 | 28.9 | 10.3 | 21.9 | 25.0 | 57.8 | full runs |
| 252 | core.color_range | as added | card/card | 4.8 | 28.8 | 6.7 | 12.9 | 31.7 | 64.3 | full runs |
| 253 | core.color_difference_key | as added | card/card | 3.9 | 28.7 | 7.4 | 13.3 | 29.0 | 66.0 | full runs |
| 254 | core.iris_wipe | feather 40 | CPU/CPU | 5.3 | 28.6 | 5.2 | 9.4 | 30.1 | 59.8 | full runs |
| 255 | core.compound_blur | max_blur 40 | CPU/CPU | 4.5 | 28.6 | 5.9 | 9.3 | 34.2 | 64.5 | full runs |
| 256 | core.particle_plane | as added | card/card | 4.2 | 28.5 | 10.5 | 21.9 | 33.0 | 57.0 | full runs |
| 257 | core.mercury | as added | card/card | 4.0 | 28.5 | 6.9 | 11.9 | 25.4 | 57.2 | full runs |
| 258 | core.line_recolor | as added | CPU/CPU | 5.2 | 28.5 | 12.2 | 26.5 | 29.2 | 61.3 | full runs |
| 259 | core.black_white | as added | card/card | 4.2 | 28.4 | 11.3 | 15.1 | 28.5 | 59.5 | full runs |
| 260 | core.audio_waveform | as added | card/card | 5.0 | 28.4 | 7.6 | 11.0 | 38.0 | 70.7 | full runs |
| 261 | core.leave_color | as added | card/card | 4.7 | 28.4 | 14.9 | 14.8 | 30.8 | 72.6 | full runs |
| 262 | core.cryptomatte | as added | CPU/CPU | 4.4 | 28.4 | 8.0 | 10.9 | 29.8 | 69.3 | full runs |
| 263 | core.fractal_noise | complexity 8 | card/card | 4.5 | 28.3 | 7.1 | 11.1 | 29.1 | 62.6 | full runs |
| 264 | core.add_grain | as added | card/card | 4.2 | 28.3 | 8.3 | 14.6 | 36.2 | 75.4 | full runs |
| 265 | core.mosaic | as added | card/card | 5.2 | 28.1 | 11.7 | 25.0 | 30.5 | 63.0 | full runs |
| 266 | core.turbulent_noise | as added | card/card | 4.5 | 28.0 | 8.3 | 12.2 | 28.3 | 68.0 | full runs |
| 267 | core.cc_ball_action | as added | card/card | 4.0 | 28.0 | 7.1 | 10.8 | 28.3 | 68.2 | full runs |
| 268 | core.compound_blur | as added | CPU/CPU | 6.9 | 27.9 | 9.9 | 16.4 | 33.1 | 63.4 | full runs |
| 269 | core.fast_blur_legacy | as added | CPU/CPU | 3.9 | 27.9 | 5.3 | 8.8 | 30.2 | 72.3 | full runs |
| 270 | core.vignette | as added | card/card | 5.0 | 27.9 | 11.7 | 22.4 | 38.0 | 70.8 | full runs |
| 271 | core.fast_box_blur | as added | CPU/CPU | 4.3 | 27.9 | 5.6 | 10.8 | 33.1 | 71.8 | full runs |
| 272 | core.color_control | as added | CPU/CPU | 4.3 | 27.8 | 6.3 | 16.2 | 29.3 | 61.8 | full runs |
| 273 | core.particle_space | as added | card/card | 4.3 | 27.8 | 10.5 | 13.4 | 26.5 | 56.0 | full runs |
| 274 | core.point_control | as added | CPU/CPU | 3.9 | 27.7 | 11.8 | 23.4 | 27.4 | 60.9 | full runs |
| 275 | core.advanced_spill_suppressor | as added | card/card | 4.3 | 27.7 | 11.9 | 15.3 | 31.4 | 66.2 | full runs |
| 276 | core.unmult | as added | card/card | 3.9 | 27.7 | 7.2 | 12.8 | 26.8 | 61.0 | full runs |
| 277 | core.depth_of_field | as added | CPU/CPU | 4.2 | 27.7 | 8.3 | 16.4 | 30.4 | 62.7 | full runs |
| 278 | core.force_motion_blur | motion_blur_samples 40 | CPU/CPU | 6.8 | 27.6 | 6.7 | 16.7 | 37.2 | 74.2 | full runs |
| 279 | core.linear_wipe | as added | CPU/CPU | 4.1 | 27.6 | 8.4 | 20.0 | 29.6 | 66.0 | full runs |
| 280 | core.bezier_warp | as added | CPU/CPU | 5.0 | 27.4 | 5.7 | 8.9 | 28.2 | 63.0 | full runs |
| 281 | core.scribble | as added | CPU/CPU | 5.2 | 27.2 | 5.9 | 8.4 | 28.7 | 61.4 | full runs |
| 282 | core.particle_playground | barrel_radius 40, direction_random_spread 40, velocity_random_spread 40, cannon_particle_radius 40, grid_particle_radius 40, explode_radius 40, particle_exploder_radius 40, gravity_random_spread 40, particle_exploder_age_feather 40, layer_map_age_feather 40, gravity_age_feather 40, wall_age_feather 40, persistent_age_feather 40, ephemeral_age_feather 40 | card/card | 4.2 | 27.2 | 7.3 | 12.1 | 25.4 | 57.0 | full runs |
| 283 | core.particle_emitter | as added | card/card | 6.3 | 27.2 | 10.9 | 14.7 | 32.6 | 65.7 | full runs |
| 284 | core.checkbox_control | as added | CPU/CPU | 4.2 | 27.2 | 9.4 | 22.9 | 28.8 | 59.5 | full runs |
| 285 | core.match_grain | as added | CPU/CPU | 3.9 | 27.2 | 5.9 | 8.2 | 27.1 | 68.5 | full runs |
| 286 | core.difference_matte | matching_softness 40, blur_before_difference 40 | CPU/CPU | 4.8 | 27.2 | 7.8 | 16.4 | 47.7 | 65.8 | full runs |
| 287 | core.slider_control | as added | CPU/CPU | 3.6 | 27.2 | 5.7 | 15.8 | 27.0 | 61.8 | full runs |
| 288 | core.solarize | as added | card/card | 4.2 | 27.1 | 11.4 | 13.9 | 30.9 | 69.1 | full runs |
| 289 | core.venetian_blinds | as added | CPU/CPU | 5.6 | 27.1 | 5.2 | 8.8 | 28.4 | 61.8 | full runs |
| 290 | core.gradient_wipe | as added | CPU/CPU | 4.0 | 27.1 | 8.1 | 15.8 | 29.1 | 62.0 | full runs |
| 291 | core.fill | as added | card/card | 4.1 | 27.1 | 6.6 | 10.4 | 29.7 | 71.5 | full runs |
| 292 | core.difference_matte | as added | CPU/CPU | 4.0 | 27.1 | 5.3 | 11.3 | 29.2 | 75.2 | full runs |
| 293 | core.threshold_rgb | as added | card/card | 4.3 | 27.1 | 11.2 | 12.9 | 29.3 | 64.4 | full runs |
| 294 | core.texturize | as added | CPU/CPU | 6.2 | 27.0 | 6.2 | 8.8 | 28.9 | 60.8 | full runs |
| 295 | core.unmult | softness 40 | card/card | 4.0 | 27.0 | 7.2 | 13.6 | 27.4 | 58.4 | full runs |
| 296 | core.twister | as added | CPU/CPU | 3.7 | 27.0 | 8.2 | 9.9 | 28.0 | 63.5 | full runs |
| 297 | core.vibrance | as added | CPU/CPU | 4.1 | 26.9 | 8.6 | 16.6 | 29.2 | 64.8 | full runs |
| 298 | core.noise | as added | card/card | 4.2 | 26.9 | 7.1 | 11.6 | 28.9 | 60.9 | full runs |
| 299 | core.stretch_color | as added | CPU/CPU | 4.0 | 26.9 | 8.1 | 23.3 | 27.6 | 58.7 | full runs |
| 300 | core.hsv_key | as added | card/card | 4.7 | 26.9 | 6.9 | 12.7 | 28.6 | 64.0 | full runs |
| 301 | core.particle_grid | as added | card/card | 5.2 | 26.9 | 17.1 | 13.5 | 27.6 | 55.8 | full runs |
| 302 | core.depth_of_field | maximum_radius 40 | CPU/CPU | 4.2 | 26.9 | 8.4 | 14.9 | 28.5 | 61.7 | full runs |
| 303 | core.fog_3d | as added | CPU/CPU | 4.0 | 26.9 | 9.2 | 16.2 | 28.6 | 62.6 | full runs |
| 304 | core.simple_choker | as added | CPU/CPU | 4.6 | 26.8 | 5.1 | 9.8 | 28.4 | 61.4 | full runs |
| 305 | core.hue_saturation | as added | CPU/CPU | 4.0 | 24.8 | 12.4 | 26.8 | 30.5 | 60.9 | full runs |
| 306 | core.stretch_contrast | as added | CPU/CPU | 3.9 | 26.8 | 9.2 | 16.7 | 27.3 | 58.8 | full runs |
| 307 | core.color_lookup | as added | CPU/CPU | 4.6 | 26.7 | 13.3 | 8.9 | 37.6 | 77.7 | full runs |
| 308 | core.channel_wipe | as added | CPU/CPU | 4.1 | 26.7 | 10.8 | 25.5 | 27.6 | 60.8 | full runs |
| 309 | core.exposure | as added | CPU/CPU | 4.1 | 26.6 | 5.3 | 16.2 | 32.6 | 68.8 | full runs |
| 310 | core.displacement_map | as added | CPU/CPU | 6.0 | 26.6 | 6.0 | 9.1 | 32.2 | 62.8 | full runs |
| 311 | core.channel_threshold | as added | card/card | 3.9 | 26.6 | 6.8 | 14.9 | 29.0 | 62.6 | full runs |
| 312 | core.teeth_wipe | as added | CPU/CPU | 3.5 | 26.5 | 5.7 | 8.7 | 26.5 | 56.5 | full runs |
| 313 | core.select_color | as added | CPU/CPU | 4.3 | 26.5 | 5.3 | 9.3 | 29.8 | 61.2 | full runs |
| 314 | core.levels | as added | CPU/CPU | 4.3 | 26.4 | 5.3 | 8.4 | 28.1 | 62.9 | full runs |
| 315 | core.linear_wipe | feather 40 | CPU/CPU | 4.0 | 26.4 | 7.1 | 10.1 | 33.2 | 61.7 | full runs |
| 316 | core.particle_emitter | direction_spread 40 | card/card | 4.3 | 26.4 | 8.0 | 11.8 | 26.0 | 57.0 | full runs |
| 317 | core.gaussian_blur | as added | CPU/CPU | 4.0 | 26.4 | 7.7 | 9.4 | 36.0 | 67.3 | full runs |
| 318 | core.color_balance | as added | CPU/CPU | 5.2 | 26.2 | 5.0 | 9.3 | 26.5 | 57.4 | full runs |
| 319 | core.inner_outer_key | as added | CPU/CPU | 4.0 | 26.2 | 7.9 | 21.1 | 41.5 | 82.3 | full runs |
| 320 | core.aerial_haze | as added | card/card | 3.8 | 26.2 | 7.8 | 13.7 | 27.4 | 65.2 | full runs |
| 321 | core.gradient_map | as added | card/card | 4.2 | 26.1 | 10.1 | 20.0 | 28.4 | 63.0 | full runs |
| 322 | core.invert | as added | card/card | 3.9 | 26.1 | 7.4 | 12.6 | 28.4 | 62.2 | full runs |
| 323 | core.identifier | as added | CPU/CPU | 4.0 | 26.1 | 7.9 | 17.2 | 28.5 | 59.2 | full runs |
| 324 | core.strobe_light | as added | CPU/CPU | 4.0 | 26.1 | 6.7 | 9.8 | 29.3 | 81.2 | full runs |
| 325 | core.particle_space | bounce_spread 40 | card/card | 4.2 | 26.0 | 7.0 | 11.6 | 27.1 | 60.6 | full runs |
| 326 | core.stroke | as added | CPU/CPU | 4.0 | 26.0 | 18.5 | 9.2 | 28.6 | 61.5 | full runs |
| 327 | core.color_key | softness 40 | CPU/CPU | 3.8 | 25.9 | 5.1 | 9.6 | 28.6 | 63.9 | full runs |
| 328 | core.threshold | as added | card/card | 3.9 | 25.9 | 6.9 | 13.7 | 29.3 | 60.5 | full runs |
| 329 | core.motion_tile | as added | CPU/CPU | 4.0 | 25.9 | 5.5 | 8.7 | 28.9 | 64.7 | full runs |
| 330 | core.text_animator | as added | CPU/CPU | 3.9 | 25.9 | 5.2 | 19.3 | 27.2 | 58.7 | full runs |
| 331 | core.iris_wipe | as added | CPU/CPU | 4.3 | 25.9 | 6.5 | 8.7 | 30.6 | 63.1 | full runs |
| 332 | core.write_on | as added | card/card | 4.7 | 25.9 | 8.6 | 14.2 | 29.5 | 63.5 | full runs |
| 333 | core.radial_wipe | as added | CPU/CPU | 3.7 | 25.8 | 8.0 | 11.6 | 50.3 | 99.5 | full runs |
| 334 | core.venetian_blinds | feather 40 | CPU/CPU | 4.2 | 25.8 | 5.4 | 17.1 | 29.7 | 62.9 | full runs |
| 335 | core.block_load | as added | card/card | 4.5 | 25.7 | 6.3 | 11.1 | 27.3 | 56.3 | full runs |
| 336 | core.corner_pin | as added | CPU/CPU | 3.8 | 25.6 | 5.1 | 8.8 | 30.6 | 61.5 | full runs |
| 337 | core.power_pin | as added | CPU/CPU | 3.8 | 25.6 | 8.0 | 15.9 | 30.5 | 62.9 | full runs |
| 338 | core.channel_wipe | softness 40, blur 40 | CPU/CPU | 3.7 | 25.6 | 7.8 | 16.4 | 26.1 | 60.6 | full runs |
| 339 | core.brightness_contrast | as added | CPU/CPU | 4.8 | 25.6 | 5.2 | 9.6 | 28.7 | 71.0 | full runs |
| 340 | core.repeat_tile | as added | CPU/CPU | 4.1 | 25.6 | 5.0 | 9.9 | 31.6 | 58.9 | full runs |
| 341 | core.glass_melt | as added | CPU/CPU | 3.6 | 25.5 | 7.8 | 17.1 | 35.1 | 68.4 | full runs |
| 342 | core.gradient_wipe | softness 40 | CPU/CPU | 6.2 | 25.5 | 5.2 | 8.8 | 28.6 | 60.7 | full runs |
| 343 | core.gradient | as added | card/card | 3.9 | 25.4 | 10.4 | 21.9 | 30.5 | 68.0 | full runs |
| 344 | core.transform | as added | CPU/CPU | 6.4 | 25.3 | 4.8 | 9.3 | 27.2 | 60.0 | full runs |
| 345 | core.write_on | stroke_length 40 | card/card | 4.4 | 25.3 | 10.9 | 16.9 | 28.9 | 58.6 | full runs |
| 346 | core.tritone | as added | card/card | 4.0 | 25.3 | 6.3 | 10.5 | 25.2 | 55.3 | full runs |
| 347 | core.channel_mixer | as added | CPU/CPU | 3.7 | 25.2 | 5.5 | 9.2 | 28.8 | 62.8 | full runs |
| 348 | core.block_dissolve | feather 40 | CPU/CPU | 4.0 | 25.2 | 4.8 | 8.7 | 27.2 | 64.2 | full runs |
| 349 | core.levels_individual | as added | CPU/CPU | 4.0 | 25.1 | 5.0 | 9.4 | 29.4 | 63.5 | full runs |
| 350 | core.liquify | as added | CPU/CPU | 3.8 | 25.1 | 6.6 | 8.8 | 28.9 | 73.4 | full runs |
| 351 | core.curves | as added | CPU/CPU | 3.9 | 25.0 | 5.1 | 16.4 | 30.4 | 65.2 | full runs |
| 352 | core.tint | as added | card/card | 3.8 | 25.0 | 24.4 | 16.6 | 27.5 | 57.7 | full runs |
| 353 | core.glass_melt | softness 40 | CPU/CPU | 3.8 | 25.0 | 9.8 | 17.1 | 26.4 | 55.4 | full runs |
| 354 | core.energy_stroke | core_softness 40 | card/card | 4.5 | 24.9 | 8.1 | 10.7 | 26.4 | 56.0 | full runs |
| 355 | core.posterize | as added | card/card | 4.1 | 24.9 | 11.2 | 12.9 | 29.4 | 61.8 | full runs |
| 356 | core.eyedropper_fill | as added | card/card | 4.5 | 24.7 | 7.0 | 11.9 | 27.0 | 57.9 | full runs |
| 357 | core.tile_wipe | as added | CPU/CPU | 3.8 | 24.7 | 7.5 | 16.4 | 27.5 | 54.9 | full runs |
| 358 | core.color_key | as added | CPU/CPU | 4.2 | 24.6 | 7.9 | 13.2 | 27.0 | 57.5 | full runs |
| 359 | core.layer_control | as added | CPU/CPU | 4.0 | 24.6 | 7.8 | 22.6 | 27.9 | 57.7 | full runs |
| 360 | core.energy_stroke | as added | card/card | 3.7 | 24.6 | 9.8 | 11.4 | 26.3 | 62.5 | full runs |
| 361 | core.glow_wipe | as added | CPU/CPU | 3.9 | 24.5 | 8.1 | 9.0 | 26.2 | 58.4 | full runs |
| 362 | core.selective_color_blur | blur 40 | CPU/CPU | 4.0 | 24.5 | 8.2 | 15.6 | 28.8 | 58.8 | full runs |
| 363 | core.posterize_time | as added | CPU/CPU | 3.5 | 24.5 | 7.8 | 21.6 | 26.4 | 58.4 | full runs |
| 364 | core.push_iris | as added | CPU/CPU | 4.2 | 24.5 | 8.6 | 16.1 | 27.1 | 73.3 | full runs |
| 365 | core.offset | as added | CPU/CPU | 4.2 | 24.4 | 8.0 | 10.2 | 27.2 | 57.3 | full runs |
| 366 | core.stripe_sweep | as added | CPU/CPU | 3.5 | 24.2 | 7.8 | 9.2 | 26.5 | 55.6 | full runs |
| 367 | core.selective_color_blur | as added | CPU/CPU | 3.5 | 24.1 | 4.8 | 8.3 | 29.1 | 78.0 | full runs |
| 368 | core.eyedropper_fill | sample_radius 40 | card/card | 3.8 | 23.2 | 9.9 | 11.8 | 26.8 | 60.7 | full runs |
| 369 | (none) | as added | CPU/CPU | 3.0 | 19.3 | 6.9 | 8.4 | 36.8 | 54.3 | full runs |
