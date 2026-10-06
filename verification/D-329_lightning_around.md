# D-329 / B-211: Lightning that goes round shapes

From P-26's tutorial 2 (Advanced Electric). After Effects' **Advanced Lightning** goes round what is solid on its layer, and a negative **Alpha Obstacle** keeps it inside what is solid. D-324's bolt could only stop at an obstacle.

## What changed

- Lightning Bolt has a new choice in Effect controls, **At an Obstacle**:
  - **Stop**: D-324's bolt, as before. This is the default.
  - **Go Round**: the bolt finds a way round what blocks it and goes on to its end point. If there is no way through, it goes as near to the end as it can.
- **Alpha Obstacle** now runs from -100 to 100:
  - Above 0, as before: the bolt keeps out of what is solid.
  - Below 0, new: the bolt keeps **inside** what is solid. Past the layer's edge counts as not solid.
- The forks still stop at an obstacle, as before. Only the main bolt goes round.
- A file from before has no **At an Obstacle** setting. It opens as Stop, draws exactly as before and is saved as it was.
- A word other than Stop or Go Round in a file is kept and reported, and the effect is left out (as D-46).

**What is not built, and why:**

- After Effects grows its bolt through a "resistance" field. This program finds the shortest way round on the layer's pixels and then adds the jaggedness on top. Both go round; the exact shape will not match After Effects'. No After Effects frame was compared.
- After Effects' Termination Threshold is not built.

## Checks (cargo test)

All in `verification/D-329_lightning_around_table.md`, **70 of 70 pass**:

| Check | Expected | Got |
|---|---|---|
| FX-LIGHTA-001 to 012: Stop and Go Round, at 50 and at -40 to -60, on a block, a ground strip, a U shape and a mist | every pixel as `Fixtures/lightning_around/expected_lightning_around.json`, within 2e-5 | pass |
| FX-LIGHTA-001 and 002: Stop, or Go Round with no obstacle | exactly D-324's bolt | pass |
| D-324's FX-LIGHTX-001 to 024 (B-203) and D-190's FX-BOLT-001 to 032 (B-126) | unchanged | pass |
| The 14 files opened and saved | the same file back | pass |
| An old Lightning Bolt file | saved without the new setting | pass |
| FX-LIGHTA-013, "sideways" | kept in the file, the effect left out, a sentence naming Stop and Go Round | pass |
| A draft preview | only width and glow halved, as before | pass |
| "sideways", Alpha Obstacle -101, Alpha Obstacle keyed to -150 | refused with a sentence, nothing changes | pass |
| Go Round, Alpha Obstacle -100, Alpha Obstacle keyed from 100 to -100 | taken; the undos give back the frame byte for byte | pass |
| Frames in tiles of 1 and of 64 | byte for byte the same | pass |

## Pictures

In `verification/D-329 pictures/`, drawn by the test (B-211). A night sky (`sky.png`) with a drawing of rock over it, the rock carrying the bolt: a round rock (`disc.png`) or a ring of rock (`ring.png`).

| Picture | What it shows | Look for | Pass? |
|---|---|---|---|
| `disc_before.png` | The sky and the round rock, no bolt | A brown disc in the middle of the night sky | |
| `1_disc_stop.png` | A bolt from the top edge to the bottom edge, Alpha Obstacle 50, **Stop** | The bolt ends where it touches the top of the rock. Nothing below the rock is lit. | |
| `2_disc_go_round.png` | The same with **Go Round** | The bolt reaches the rock, runs down beside it without crossing it, and goes on to the bottom edge | |
| `ring_before.png` | The sky and a ring of rock, no bolt | A brown ring with the sky showing through its hole | |
| `3_ring_inside_stop.png` | A bolt from the ring's left side to its right side, Alpha Obstacle **-50**, **Stop** | The bolt starts in the ring's left side and ends where it would leave the rock into the hole | |
| `4_ring_inside_go_round.png` | The same with **Go Round** | The bolt stays inside the brown ring, going round its lower half to the right side. It never crosses the hole and never goes outside the ring. | |

## How to try it in the app

1. Put a Lightning Bolt on a layer with a solid shape on it, start above the shape and end below it.
2. Set Alpha Obstacle to 50. The bolt stops on the shape.
3. Set **At an Obstacle** to **Go Round**. The bolt goes round the shape to its end point.
4. Set Alpha Obstacle to -50, with the start and end inside the shape. The bolt stays inside the shape.

**Awaiting the owner's playtest.**
