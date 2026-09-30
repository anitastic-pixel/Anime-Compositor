# B-172: runs of colour effects drawn in one pass on the card

Written by `tests/b172_fused.rs`. Each frame is drawn from nothing twice on the card: once with
every effect in a pass of its own, as before, and once with each run of colour effects next to
each other drawn in one pass. "Same" means every byte the viewer shows is the same.

| Shot | Quality | Frame | Passes, one each | Passes, runs in one | Same |
|---|---|---:|---:|---:|---|
| every colour effect, in runs of eight, eight and five | Full | 0 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Full | 50 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Full | 101 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Full | 173 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Full | 239 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Draft | 0 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Draft | 50 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Draft | 101 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Draft | 173 | 25 | 7 | yes |
| every colour effect, in runs of eight, eight and five | Draft | 239 | 25 | 7 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Full | 0 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Full | 50 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Full | 101 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Full | 173 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Full | 239 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Draft | 0 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Draft | 50 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Draft | 101 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Draft | 173 | 25 | 18 | yes |
| runs broken by a blur, an Offset and a Drop Shadow | Draft | 239 | 25 | 18 | yes |
| runs beginning with Paraffin and HSV Key | Full | 0 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Full | 50 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Full | 101 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Full | 173 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Full | 239 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Draft | 0 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Draft | 50 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Draft | 101 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Draft | 173 | 14 | 7 | yes |
| runs beginning with Paraffin and HSV Key | Draft | 239 | 14 | 7 | yes |
| eight colour effects, the fifth's setting changed, drawn a first time | Full | 100 | 28 | 7 | yes |
| eight colour effects, the fifth's setting changed, drawn a second time | Full | 100 | 28 | 10 | yes |
| eight colour effects, the fifth's setting changed, drawn a third time | Full | 100 | 28 | 4 | yes |
| eight colour effects, the fifth's setting changed, drawn a fourth time | Full | 100 | 28 | 4 | yes |
| eight colour effects, the fifth's setting changed, drawn a first time | Draft | 100 | 28 | 7 | yes |
| eight colour effects, the fifth's setting changed, drawn a second time | Draft | 100 | 28 | 10 | yes |
| eight colour effects, the fifth's setting changed, drawn a third time | Draft | 100 | 28 | 4 | yes |
| eight colour effects, the fifth's setting changed, drawn a fourth time | Draft | 100 | 28 | 4 | yes |

**38 of 38 frames the same; 38 of 38 drawn in fewer passes.**
