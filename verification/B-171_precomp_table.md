# B-171: a composition inside another kept

The reference shot, with a Gaussian Blur (24 px) on its first layer and another (16 px) on its third to make it heavy, shown whole in an outer composition. The viewer keeps what it draws of the inner composition in memory and in its disk folder. Each row draws one frame of the outer composition. "Drawings asked for" counts the drawings the frame asked the cache for: none means the inner composition was not drawn at all. A row passes when the frame is, byte for byte, the frame a cache that keeps nothing (the export's) draws, and the inner composition was drawn, or not, as the row says it should be.

**42 of 42 pass.**

| Quality | Frame | What | Drawings asked for | Same bytes | Result |
|---|---:|---|---:|---|---|
| Full | 0 | drawn the first time | 4 | yes | pass |
| Full | 0 | drawn again | 0 | yes | pass |
| Full | 0 | copies of composition frames on disk: 1 | 0 | yes | pass |
| Full | 0 | drawn by a new viewer, the program opened again | 0 | yes | pass |
| Full | 0 | the outer layer's opacity edited | 0 | yes | pass |
| Full | 0 | an inner layer's opacity edited | 4 | yes | pass |
| Full | 0 | drawn after the drawings were dated anew | 4 | yes | pass |
| Full | 1 | drawn the first time | 4 | yes | pass |
| Full | 1 | drawn again | 0 | yes | pass |
| Full | 1 | copies of composition frames on disk: 3 | 0 | yes | pass |
| Full | 1 | drawn by a new viewer, the program opened again | 0 | yes | pass |
| Full | 1 | the outer layer's opacity edited | 0 | yes | pass |
| Full | 1 | an inner layer's opacity edited | 4 | yes | pass |
| Full | 1 | drawn after the drawings were dated anew | 4 | yes | pass |
| Full | 100 | drawn the first time | 4 | yes | pass |
| Full | 100 | drawn again | 0 | yes | pass |
| Full | 100 | copies of composition frames on disk: 5 | 0 | yes | pass |
| Full | 100 | drawn by a new viewer, the program opened again | 0 | yes | pass |
| Full | 100 | the outer layer's opacity edited | 0 | yes | pass |
| Full | 100 | an inner layer's opacity edited | 4 | yes | pass |
| Full | 100 | drawn after the drawings were dated anew | 4 | yes | pass |
| Draft | 0 | drawn the first time | 4 | yes | pass |
| Draft | 0 | drawn again | 0 | yes | pass |
| Draft | 0 | copies of composition frames on disk: 7 | 0 | yes | pass |
| Draft | 0 | drawn by a new viewer, the program opened again | 0 | yes | pass |
| Draft | 0 | the outer layer's opacity edited | 0 | yes | pass |
| Draft | 0 | an inner layer's opacity edited | 4 | yes | pass |
| Draft | 0 | drawn after the drawings were dated anew | 4 | yes | pass |
| Draft | 1 | drawn the first time | 4 | yes | pass |
| Draft | 1 | drawn again | 0 | yes | pass |
| Draft | 1 | copies of composition frames on disk: 9 | 0 | yes | pass |
| Draft | 1 | drawn by a new viewer, the program opened again | 0 | yes | pass |
| Draft | 1 | the outer layer's opacity edited | 0 | yes | pass |
| Draft | 1 | an inner layer's opacity edited | 4 | yes | pass |
| Draft | 1 | drawn after the drawings were dated anew | 4 | yes | pass |
| Draft | 100 | drawn the first time | 4 | yes | pass |
| Draft | 100 | drawn again | 0 | yes | pass |
| Draft | 100 | copies of composition frames on disk: 11 | 0 | yes | pass |
| Draft | 100 | drawn by a new viewer, the program opened again | 0 | yes | pass |
| Draft | 100 | the outer layer's opacity edited | 0 | yes | pass |
| Draft | 100 | an inner layer's opacity edited | 4 | yes | pass |
| Draft | 100 | drawn after the drawings were dated anew | 4 | yes | pass |
