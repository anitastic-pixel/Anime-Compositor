# D-282 to D-287: sketch strokes and layers

Written by `tests/d282_sketch_edits.rs`.

**30 of 30 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Taking strokes 1 and 2 off leaves 0, 300 and 400 | 0,300,400 | 0,300,400 | PASS |
| One undo puts both back where they were | 0,100,200,300,400 | 0,100,200,300,400 | PASS |
| A place past the last stroke is refused | refused | refused | PASS |
| The same place twice is refused | refused | refused | PASS |
| No places at all is refused | refused | refused | PASS |
| Moving strokes 0 and 3 by 5, -2.5 moves every point of both | [5,97.5] [25,117.5] [305,97.5] [325,117.5] | [5,97.5] [25,117.5] [305,97.5] [325,117.5] | PASS |
| and leaves the others where they were | 100,200,400 | 100,200,400 | PASS |
| One undo puts them back | 0,100,200,300,400 | 0,100,200,300,400 | PASS |
| A move by an amount that is not a number is refused | refused | refused | PASS |
| An opacity of 0.4 is taken | taken | taken | PASS |
| An opacity of 1.5 is refused | refused | refused | PASS |
| An opacity below 0 is refused | refused | refused | PASS |
| On a locked layer, a stroke is refused | refused | refused | PASS |
| taking strokes off is refused | refused | refused | PASS |
| moving strokes is refused | refused | refused | PASS |
| Clear is refused | refused | refused | PASS |
| deleting the layer is refused | refused | refused | PASS |
| and the refusal says how to go on | says Unlock it | says Unlock it | PASS |
| Unlocked, a stroke is taken again | taken | taken | PASS |
| Moving Rough to the top puts Clean under it | Clean,Rough | Clean,Rough | PASS |
| Moving a layer past the top is refused | refused | refused | PASS |
| Rough is saved with its opacity and lock | 0.4 true | 0.4 true | PASS |
| Clean, at full opacity and unlocked, saves neither | neither | neither | PASS |
| The filled pen stroke saves filled and pressure_opacity | true true | true true | PASS |
| A plain stroke saves neither | neither | neither | PASS |
| Opened again, both layers are the same | the same | the same | PASS |
| Saved again, the text is the same, byte for byte | the same | the same | PASS |
| A file with opacity 2 is refused, saying where | refused at /sketches/1/opacity | refused at /sketches/1/opacity | PASS |
| A file with locked "yes" is refused, saying where | refused at /sketches/1/locked | refused at /sketches/1/locked | PASS |
| A file with filled 1 is refused, saying where | refused at /strokes/0/filled | refused at /strokes/0/filled | PASS |
