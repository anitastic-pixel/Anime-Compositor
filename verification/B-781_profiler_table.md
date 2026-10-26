# B-781 profiler (D-901)

Written by `cargo test -p anime_compositor_app b781_profiler_reports_layers_and_where_they_were_drawn`. The reference shot, frame 100, Full quality, a Gaussian Blur (sigma 24) on layer2. Times are this run's and vary; the checks are on names, places and order.

| Check | Expected | Got | Result |
|---|---|---|---|
| Profiler off: nothing reported | no x-profile | x-profile "" | pass |
| CPU: one row per layer, by name | layer1, layer2, layer3, layer4 | layer1, layer2, layer3, layer4 | pass |
| CPU: every layer drawn on the processor | processor x4 | "processor", "processor", "processor", "processor" | pass |
| CPU: layer2's blur timed, on the processor | gaussian blur > 0 ms, processor | Some((95.7, String("processor"))) | pass |
| CPU: the blurred layer is the slowest | layer2 | "layer2" (109.41 ms; others 0.41, 0.16, 0.08) | pass |
| CPU: blur within its layer, layers within the frame | blur <= layer2 <= sum of layers <= frame | 95.7 <= 109.41 <= 110.06 <= 207.01 | pass |
| CPU: the picture with the profiler on | byte for byte the picture with it off | 8294400 bytes, same: true | pass |
| The same frame asked again, profiler still open | no layer rows if shown from memory, else four | from_memory true, 0 rows | pass |
| Measured again: layer2's blur kept from the first time | layer2 from cache 1 effect, no blur time | kept 1, blur None | pass |
| Card: layer2's blur drawn by the card | layer2 on card, blur on card | frame "GPU", layer2 "card", blur Some(String("card")) | pass |
| Card: the picture with the profiler on | byte for byte the picture with it off | same: true | pass |
| Card: an Echo put before the blur, which only the processor draws | layer2 card and processor; echo processor; blur card | layer2 "card and processor", echo Some(String("processor")), blur Some(String("card")) | pass |
