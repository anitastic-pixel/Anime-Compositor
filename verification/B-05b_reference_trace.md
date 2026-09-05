# B-05b render trace of the reference shot

One traced frame of the reference shot, 4 layers at 1920x1080, written to `trace/` by `tests/b05b_trace.rs`.

`trace/` is in `.gitignore` and is not committed. ADR-012: trace output is diagnostic, never part of export, never on by default. Re-create it with:

```text
cargo test --test b05b_trace b05b_traces_a_frame
```

## What was written

| File | Layer | Stage | Document 21 step |
|---|---|---|---|
| `trace/frame_00000/layer00_layer1_decode.png` | `layer1` | decode | 1 |
| `trace/frame_00000/layer00_layer1_transform.png` | `layer1` | transform | 4 |
| `trace/frame_00000/layer00_layer1_opacity.png` | `layer1` | opacity | 6 |
| `trace/frame_00000/layer00_layer1_composite.png` | `layer1` | composite | 7 |
| `trace/frame_00000/layer01_layer2_decode.png` | `layer2` | decode | 1 |
| `trace/frame_00000/layer01_layer2_transform.png` | `layer2` | transform | 4 |
| `trace/frame_00000/layer01_layer2_opacity.png` | `layer2` | opacity | 6 |
| `trace/frame_00000/layer01_layer2_composite.png` | `layer2` | composite | 7 |
| `trace/frame_00000/layer02_layer3_decode.png` | `layer3` | decode | 1 |
| `trace/frame_00000/layer02_layer3_transform.png` | `layer3` | transform | 4 |
| `trace/frame_00000/layer02_layer3_opacity.png` | `layer3` | opacity | 6 |
| `trace/frame_00000/layer02_layer3_composite.png` | `layer3` | composite | 7 |
| `trace/frame_00000/layer03_layer4_decode.png` | `layer4` | decode | 1 |
| `trace/frame_00000/layer03_layer4_transform.png` | `layer4` | transform | 4 |
| `trace/frame_00000/layer03_layer4_opacity.png` | `layer4` | opacity | 6 |
| `trace/frame_00000/layer03_layer4_composite.png` | `layer4` | composite | 7 |

## How to use it

Open the directory and walk up the stack. Each layer has four images. `decode` is the drawing as imported, at its own size. `transform` is that drawing moved, scaled and turned into the composition. `opacity` is the same again after the layer's opacity. `composite` is everything up to and including that layer.

When a frame looks wrong, the first image in that walk that looks wrong names the stage that broke it, and naming the stage is enough to say what to fix without reading any code.
