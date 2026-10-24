# B-644: Curl Noise with Source This Layer is faster on the card, with no pixel changed

Times in ms a frame, "Again" column (the shot played again), from the `#[ignore]` timing tests
`b461_curl_sources_timing`, `b326_curl_noise_timing` and `b525_curl_transform_timing`, run with
`--release --ignored` and their raw output written to the lane's scratch folder (`B461_OUT`,
`B326_OUT`, `B525_OUT`). The before build is main d42c3432; the after build is B-644. Both builds
were run one after the other in the same window. The committed B-326, B-461 and B-525 tables are
written by hand from such raw output, so they are not rewritten here.

- Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.88, Vulkan, 16.8 GB
- Processor: AMD64 Family 26 Model 68 (Ryzen 9 9900X), 24 threads; Windows
- Build: release (`cargo test --release`), drawn by the card, 8 loops, reference shot at Full
- Load: the card at 0 to 4 per cent before and after each run; 0 to 4 other lanes' cargo/rustc
  processes were seen at the runs' ends (no other test on the card). Mostly quiet, not fully quiet.

| Shot | Before (d42c3432) | After (B-644) | Change |
|---|---:|---:|---:|
| Noise alone (B-461) | 12.5 | 12.1 | same |
| Noise, then Curl Noise This Layer (Swirl 45, 12 samples, radius 30) | 214.8 | 135.8 | 37% less |
| the same with Softness 50 | 226.1 | 147.5 | 35% less |
| Curl Noise Input Noise as added (B-326) | 74.1 | 85.8 | the noise path is unchanged; run-to-run spread |
| Curl Noise Input Noise (no lines) (B-326) | 75.3 | 73.9 | same |
| radius 100, 24 samples (B-326) | 128.0 | 130.3 | same |
| Curl Noise as added (B-525) | 75.3 | 73.3 | same |
| Rotation 45, Scale 150, Perspective Offset on (B-525) | 73.0 | 72.4 | same |

Earlier fully quiet runs on the same machine (no other cargo/rustc, card 0 to 2 per cent): before
209.2 and 209.8 ms; with the first two of B-644's three changes 143.7 and 139.2 ms.

## What changed in the card's pass (`curlfield` in `src/gpu.rs`)

1. The flow's two length checks compare squares, so a double square root is taken only where the
   flow is cut to 1/9 (the card does double square roots slowly).
2. The five plane reads round a point (right, left, below, above and the point itself) share one
   corner, so 12 pixels are read instead of 20.
3. The flow and the value at each point the trace reaches are read together, and the flow at the
   end of one part is the next part's first, so each part reads the plane twice instead of three
   times.

## No pixel changed

The check tests ran after the change and left every committed table byte for byte as it was
(`git status` shows no change under `verification/`): b461_curl_sources (the D-583 table with its
"pixels differ" counts on the reference shot), b628_curl_look, b629_curl_scale, b326_curl_noise,
b525_curl_transform, p9_curl_field, b107_gpu_fx and b65_gpu_fx all pass.

## Not reached

84 ms, the This Layer time before D-749, is not reached: D-749 traces each pixel along the picture's
flow for every sample, and the remaining time is the double-precision plane reads. Reading the plane
in single precision would be faster but moves pixels (the card's flow lines would part from the
processor's on more pixels than the D-583 table now counts), so it is not built; it would need its own D-entry. A Curl Noise
added from now on starts at 3 samples (D-764), not the 12 this shot uses, so a new one is quicker
still.
