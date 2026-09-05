# B-04 exposure and time table

Test T-02, requirement R-02. Produced by `tests/b04_exposure.rs`. **46 of 46 checks pass.**

The composition is 240 frames at 24/1 fps (10 seconds exactly), frames 0 to 239 inclusive.

Expected drawing numbers come from `Fixtures/reference_shot/exposure_sheet.json`, which the fixture README calls the authority on timing. The test builds exposure spans from the cadences the README states — layer 1 static, layer 2 on 1s, layer 3 on 2s — and expands them; the sheet says what the expansion must equal. Layer 4 is the exception: its timing is irregular and no rule generates it, so its 80 exposures are read from the sheet and the check is that expanding them reproduces the sheet's per-frame numbers.

## The 240-row frame-to-drawing table

Seconds are exact rationals, not decimals. A dash means no exposure span covers that frame, which renders transparent.

| Frame | Seconds | Layer 1 | Layer 2 | Layer 3 | Layer 4 | Note |
|---|---|---|---|---|---|---|
| 0 | 0/1 | 0 | 0 | 0 | 0 |  |
| 1 | 1/24 | 0 | 1 | 0 | 0 |  |
| 2 | 1/12 | 0 | 2 | 1 | 0 |  |
| 3 | 1/8 | 0 | 3 | 1 | 1 |  |
| 4 | 1/6 | 0 | 4 | 2 | 1 |  |
| 5 | 5/24 | 0 | 5 | 2 | 1 |  |
| 6 | 1/4 | 0 | 6 | 3 | 2 |  |
| 7 | 7/24 | 0 | 7 | 3 | 2 |  |
| 8 | 1/3 | 0 | 8 | 4 | 2 |  |
| 9 | 3/8 | 0 | 9 | 4 | 3 |  |
| 10 | 5/12 | 0 | 10 | 5 | 3 |  |
| 11 | 11/24 | 0 | 11 | 5 | 3 |  |
| 12 | 1/2 | 0 | 12 | 6 | 4 |  |
| 13 | 13/24 | 0 | 13 | 6 | 4 |  |
| 14 | 7/12 | 0 | 14 | 7 | 4 | layer3 drawing 7 missing |
| 15 | 5/8 | 0 | 15 | 7 | 5 | layer3 drawing 7 missing |
| 16 | 2/3 | 0 | 16 | 8 | 5 |  |
| 17 | 17/24 | 0 | 17 | 8 | 5 |  |
| 18 | 3/4 | 0 | 18 | 9 | 6 |  |
| 19 | 19/24 | 0 | 19 | 9 | 6 |  |
| 20 | 5/6 | 0 | 20 | 10 | 6 |  |
| 21 | 7/8 | 0 | 21 | 10 | 7 |  |
| 22 | 11/12 | 0 | 22 | 11 | 7 |  |
| 23 | 23/24 | 0 | 23 | 11 | 7 |  |
| 24 | 1/1 | 0 | 0 | 0 | 8 |  |
| 25 | 25/24 | 0 | 1 | 0 | 8 |  |
| 26 | 13/12 | 0 | 2 | 1 | 8 |  |
| 27 | 9/8 | 0 | 3 | 1 | 9 |  |
| 28 | 7/6 | 0 | 4 | 2 | 9 |  |
| 29 | 29/24 | 0 | 5 | 2 | 9 |  |
| 30 | 5/4 | 0 | 6 | 3 | 10 |  |
| 31 | 31/24 | 0 | 7 | 3 | 10 |  |
| 32 | 4/3 | 0 | 8 | 4 | 10 |  |
| 33 | 11/8 | 0 | 9 | 4 | 11 |  |
| 34 | 17/12 | 0 | 10 | 5 | 11 |  |
| 35 | 35/24 | 0 | 11 | 5 | 11 |  |
| 36 | 3/2 | 0 | 12 | 6 | 12 |  |
| 37 | 37/24 | 0 | 13 | 6 | 12 |  |
| 38 | 19/12 | 0 | 14 | 7 | 12 | layer3 drawing 7 missing |
| 39 | 13/8 | 0 | 15 | 7 | 13 | layer3 drawing 7 missing |
| 40 | 5/3 | 0 | 16 | 8 | 13 |  |
| 41 | 41/24 | 0 | 17 | 8 | 13 |  |
| 42 | 7/4 | 0 | 18 | 9 | 14 |  |
| 43 | 43/24 | 0 | 19 | 9 | 14 |  |
| 44 | 11/6 | 0 | 20 | 10 | 14 |  |
| 45 | 15/8 | 0 | 21 | 10 | 15 |  |
| 46 | 23/12 | 0 | 22 | 11 | 15 |  |
| 47 | 47/24 | 0 | 23 | 11 | 15 |  |
| 48 | 2/1 | 0 | 0 | 0 | 16 |  |
| 49 | 49/24 | 0 | 1 | 0 | 16 |  |
| 50 | 25/12 | 0 | 2 | 1 | 16 |  |
| 51 | 17/8 | 0 | 3 | 1 | 17 |  |
| 52 | 13/6 | 0 | 4 | 2 | 17 |  |
| 53 | 53/24 | 0 | 5 | 2 | 17 |  |
| 54 | 9/4 | 0 | 6 | 3 | 18 |  |
| 55 | 55/24 | 0 | 7 | 3 | 18 |  |
| 56 | 7/3 | 0 | 8 | 4 | 18 |  |
| 57 | 19/8 | 0 | 9 | 4 | 19 |  |
| 58 | 29/12 | 0 | 10 | 5 | 19 |  |
| 59 | 59/24 | 0 | 11 | 5 | 19 |  |
| 60 | 5/2 | 0 | 12 | 6 | 0 |  |
| 61 | 61/24 | 0 | 13 | 6 | 0 |  |
| 62 | 31/12 | 0 | 14 | 7 | 0 | layer3 drawing 7 missing |
| 63 | 21/8 | 0 | 15 | 7 | 0 | layer3 drawing 7 missing |
| 64 | 8/3 | 0 | 16 | 8 | 0 |  |
| 65 | 65/24 | 0 | 17 | 8 | 1 |  |
| 66 | 11/4 | 0 | 18 | 9 | 1 |  |
| 67 | 67/24 | 0 | 19 | 9 | 1 |  |
| 68 | 17/6 | 0 | 20 | 10 | 2 |  |
| 69 | 23/8 | 0 | 21 | 10 | 2 |  |
| 70 | 35/12 | 0 | 22 | 11 | 2 |  |
| 71 | 71/24 | 0 | 23 | 11 | 3 |  |
| 72 | 3/1 | 0 | 0 | 0 | 3 |  |
| 73 | 73/24 | 0 | 1 | 0 | 3 |  |
| 74 | 37/12 | 0 | 2 | 1 | 4 |  |
| 75 | 25/8 | 0 | 3 | 1 | 4 |  |
| 76 | 19/6 | 0 | 4 | 2 | 4 |  |
| 77 | 77/24 | 0 | 5 | 2 | 5 |  |
| 78 | 13/4 | 0 | 6 | 3 | 5 |  |
| 79 | 79/24 | 0 | 7 | 3 | 5 |  |
| 80 | 10/3 | 0 | 8 | 4 | 6 |  |
| 81 | 27/8 | 0 | 9 | 4 | 6 |  |
| 82 | 41/12 | 0 | 10 | 5 | 6 |  |
| 83 | 83/24 | 0 | 11 | 5 | 7 |  |
| 84 | 7/2 | 0 | 12 | 6 | 7 |  |
| 85 | 85/24 | 0 | 13 | 6 | 7 |  |
| 86 | 43/12 | 0 | 14 | 7 | 8 | layer3 drawing 7 missing |
| 87 | 29/8 | 0 | 15 | 7 | 8 | layer3 drawing 7 missing |
| 88 | 11/3 | 0 | 16 | 8 | 8 |  |
| 89 | 89/24 | 0 | 17 | 8 | 9 |  |
| 90 | 15/4 | 0 | 18 | 9 | 9 |  |
| 91 | 91/24 | 0 | 19 | 9 | 9 |  |
| 92 | 23/6 | 0 | 20 | 10 | 10 |  |
| 93 | 31/8 | 0 | 21 | 10 | 10 |  |
| 94 | 47/12 | 0 | 22 | 11 | 10 |  |
| 95 | 95/24 | 0 | 23 | 11 | 11 |  |
| 96 | 4/1 | 0 | 0 | 0 | 11 |  |
| 97 | 97/24 | 0 | 1 | 0 | 11 |  |
| 98 | 49/12 | 0 | 2 | 1 | 12 |  |
| 99 | 33/8 | 0 | 3 | 1 | 12 |  |
| 100 | 25/6 | 0 | 4 | 2 | 12 |  |
| 101 | 101/24 | 0 | 5 | 2 | 13 |  |
| 102 | 17/4 | 0 | 6 | 3 | 13 |  |
| 103 | 103/24 | 0 | 7 | 3 | 13 |  |
| 104 | 13/3 | 0 | 8 | 4 | 14 |  |
| 105 | 35/8 | 0 | 9 | 4 | 14 |  |
| 106 | 53/12 | 0 | 10 | 5 | 14 |  |
| 107 | 107/24 | 0 | 11 | 5 | 15 |  |
| 108 | 9/2 | 0 | 12 | 6 | 15 |  |
| 109 | 109/24 | 0 | 13 | 6 | 15 |  |
| 110 | 55/12 | 0 | 14 | 7 | 16 | layer3 drawing 7 missing |
| 111 | 37/8 | 0 | 15 | 7 | 16 | layer3 drawing 7 missing |
| 112 | 14/3 | 0 | 16 | 8 | 16 |  |
| 113 | 113/24 | 0 | 17 | 8 | 17 |  |
| 114 | 19/4 | 0 | 18 | 9 | 17 |  |
| 115 | 115/24 | 0 | 19 | 9 | 17 |  |
| 116 | 29/6 | 0 | 20 | 10 | 18 |  |
| 117 | 39/8 | 0 | 21 | 10 | 18 |  |
| 118 | 59/12 | 0 | 22 | 11 | 18 |  |
| 119 | 119/24 | 0 | 23 | 11 | 19 |  |
| 120 | 5/1 | 0 | 0 | 0 | 19 |  |
| 121 | 121/24 | 0 | 1 | 0 | 19 |  |
| 122 | 61/12 | 0 | 2 | 1 | 0 |  |
| 123 | 41/8 | 0 | 3 | 1 | 0 |  |
| 124 | 31/6 | 0 | 4 | 2 | 0 |  |
| 125 | 125/24 | 0 | 5 | 2 | 1 |  |
| 126 | 21/4 | 0 | 6 | 3 | 1 |  |
| 127 | 127/24 | 0 | 7 | 3 | 1 |  |
| 128 | 16/3 | 0 | 8 | 4 | 2 |  |
| 129 | 43/8 | 0 | 9 | 4 | 2 |  |
| 130 | 65/12 | 0 | 10 | 5 | 2 |  |
| 131 | 131/24 | 0 | 11 | 5 | 3 |  |
| 132 | 11/2 | 0 | 12 | 6 | 3 |  |
| 133 | 133/24 | 0 | 13 | 6 | 3 |  |
| 134 | 67/12 | 0 | 14 | 7 | 4 | layer3 drawing 7 missing |
| 135 | 45/8 | 0 | 15 | 7 | 4 | layer3 drawing 7 missing |
| 136 | 17/3 | 0 | 16 | 8 | 4 |  |
| 137 | 137/24 | 0 | 17 | 8 | 5 |  |
| 138 | 23/4 | 0 | 18 | 9 | 5 |  |
| 139 | 139/24 | 0 | 19 | 9 | 5 |  |
| 140 | 35/6 | 0 | 20 | 10 | 6 |  |
| 141 | 47/8 | 0 | 21 | 10 | 6 |  |
| 142 | 71/12 | 0 | 22 | 11 | 6 |  |
| 143 | 143/24 | 0 | 23 | 11 | 7 |  |
| 144 | 6/1 | 0 | 0 | 0 | 7 |  |
| 145 | 145/24 | 0 | 1 | 0 | 7 |  |
| 146 | 73/12 | 0 | 2 | 1 | 8 |  |
| 147 | 49/8 | 0 | 3 | 1 | 8 |  |
| 148 | 37/6 | 0 | 4 | 2 | 8 |  |
| 149 | 149/24 | 0 | 5 | 2 | 9 |  |
| 150 | 25/4 | 0 | 6 | 3 | 9 |  |
| 151 | 151/24 | 0 | 7 | 3 | 9 |  |
| 152 | 19/3 | 0 | 8 | 4 | 10 |  |
| 153 | 51/8 | 0 | 9 | 4 | 11 |  |
| 154 | 77/12 | 0 | 10 | 5 | 11 |  |
| 155 | 155/24 | 0 | 11 | 5 | 11 |  |
| 156 | 13/2 | 0 | 12 | 6 | 12 |  |
| 157 | 157/24 | 0 | 13 | 6 | 12 |  |
| 158 | 79/12 | 0 | 14 | 7 | 12 | layer3 drawing 7 missing |
| 159 | 53/8 | 0 | 15 | 7 | 13 | layer3 drawing 7 missing |
| 160 | 20/3 | 0 | 16 | 8 | 13 |  |
| 161 | 161/24 | 0 | 17 | 8 | 13 |  |
| 162 | 27/4 | 0 | 18 | 9 | 14 |  |
| 163 | 163/24 | 0 | 19 | 9 | 14 |  |
| 164 | 41/6 | 0 | 20 | 10 | 14 |  |
| 165 | 55/8 | 0 | 21 | 10 | 11 |  |
| 166 | 83/12 | 0 | 22 | 11 | 11 |  |
| 167 | 167/24 | 0 | 23 | 11 | 11 |  |
| 168 | 7/1 | 0 | 0 | 0 | 16 |  |
| 169 | 169/24 | 0 | 1 | 0 | 16 |  |
| 170 | 85/12 | 0 | 2 | 1 | 16 |  |
| 171 | 57/8 | 0 | 3 | 1 | 17 |  |
| 172 | 43/6 | 0 | 4 | 2 | 17 |  |
| 173 | 173/24 | 0 | 5 | 2 | 17 |  |
| 174 | 29/4 | 0 | 6 | 3 | 18 |  |
| 175 | 175/24 | 0 | 7 | 3 | 18 |  |
| 176 | 22/3 | 0 | 8 | 4 | 18 |  |
| 177 | 59/8 | 0 | 9 | 4 | 19 |  |
| 178 | 89/12 | 0 | 10 | 5 | 19 |  |
| 179 | 179/24 | 0 | 11 | 5 | 19 |  |
| 180 | 15/2 | 0 | 12 | 6 | 0 |  |
| 181 | 181/24 | 0 | 13 | 6 | 0 |  |
| 182 | 91/12 | 0 | 14 | 7 | 0 | layer3 drawing 7 missing |
| 183 | 61/8 | 0 | 15 | 7 | 1 | layer3 drawing 7 missing |
| 184 | 23/3 | 0 | 16 | 8 | 1 |  |
| 185 | 185/24 | 0 | 17 | 8 | 1 |  |
| 186 | 31/4 | 0 | 18 | 9 | 2 |  |
| 187 | 187/24 | 0 | 19 | 9 | 2 |  |
| 188 | 47/6 | 0 | 20 | 10 | 2 |  |
| 189 | 63/8 | 0 | 21 | 10 | 3 |  |
| 190 | 95/12 | 0 | 22 | 11 | 3 |  |
| 191 | 191/24 | 0 | 23 | 11 | 3 |  |
| 192 | 8/1 | 0 | 0 | 0 | 4 |  |
| 193 | 193/24 | 0 | 1 | 0 | 4 |  |
| 194 | 97/12 | 0 | 2 | 1 | 4 |  |
| 195 | 65/8 | 0 | 3 | 1 | 5 |  |
| 196 | 49/6 | 0 | 4 | 2 | 5 |  |
| 197 | 197/24 | 0 | 5 | 2 | 5 |  |
| 198 | 33/4 | 0 | 6 | 3 | 6 |  |
| 199 | 199/24 | 0 | 7 | 3 | 6 |  |
| 200 | 25/3 | 0 | 8 | 4 | 6 |  |
| 201 | 67/8 | 0 | 9 | 4 | 7 |  |
| 202 | 101/12 | 0 | 10 | 5 | 7 |  |
| 203 | 203/24 | 0 | 11 | 5 | 7 |  |
| 204 | 17/2 | 0 | 12 | 6 | 8 |  |
| 205 | 205/24 | 0 | 13 | 6 | 8 |  |
| 206 | 103/12 | 0 | 14 | 7 | 8 | layer3 drawing 7 missing |
| 207 | 69/8 | 0 | 15 | 7 | 9 | layer3 drawing 7 missing |
| 208 | 26/3 | 0 | 16 | 8 | 9 |  |
| 209 | 209/24 | 0 | 17 | 8 | 9 |  |
| 210 | 35/4 | 0 | 18 | 9 | 10 |  |
| 211 | 211/24 | 0 | 19 | 9 | 10 |  |
| 212 | 53/6 | 0 | 20 | 10 | 10 |  |
| 213 | 71/8 | 0 | 21 | 10 | 11 |  |
| 214 | 107/12 | 0 | 22 | 11 | 11 |  |
| 215 | 215/24 | 0 | 23 | 11 | 11 |  |
| 216 | 9/1 | 0 | 0 | 0 | 12 |  |
| 217 | 217/24 | 0 | 1 | 0 | 12 |  |
| 218 | 109/12 | 0 | 2 | 1 | 12 |  |
| 219 | 73/8 | 0 | 3 | 1 | 13 |  |
| 220 | 55/6 | 0 | 4 | 2 | 13 |  |
| 221 | 221/24 | 0 | 5 | 2 | 13 |  |
| 222 | 37/4 | 0 | 6 | 3 | 14 |  |
| 223 | 223/24 | 0 | 7 | 3 | 14 |  |
| 224 | 28/3 | 0 | 8 | 4 | 14 |  |
| 225 | 75/8 | 0 | 9 | 4 | 15 |  |
| 226 | 113/12 | 0 | 10 | 5 | 15 |  |
| 227 | 227/24 | 0 | 11 | 5 | 15 |  |
| 228 | 19/2 | 0 | 12 | 6 | 16 |  |
| 229 | 229/24 | 0 | 13 | 6 | 16 |  |
| 230 | 115/12 | 0 | 14 | 7 | 16 | layer3 drawing 7 missing |
| 231 | 77/8 | 0 | 15 | 7 | 17 | layer3 drawing 7 missing |
| 232 | 29/3 | 0 | 16 | 8 | 17 |  |
| 233 | 233/24 | 0 | 17 | 8 | 17 |  |
| 234 | 39/4 | 0 | 18 | 9 | 18 |  |
| 235 | 235/24 | 0 | 19 | 9 | 18 |  |
| 236 | 59/6 | 0 | 20 | 10 | 18 |  |
| 237 | 79/8 | 0 | 21 | 10 | 19 |  |
| 238 | 119/12 | 0 | 22 | 11 | 19 |  |
| 239 | 239/24 | 0 | 23 | 11 | 19 |  |

## Exposure structure

| Layer | Spans | Frames covered | Cadence |
|---|---|---|---|
| layer1 | 1 | 240 | span lengths 240 |
| layer2 | 240 | 240 | span lengths 1 |
| layer3 | 120 | 240 | span lengths 2 |
| layer4 | 80 | 240 | span lengths 1, 3, 5 |

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| composition: frame count | `240` | `240` | PASS |
| composition: first frame | `0` | `0` | PASS |
| composition: last frame | `239` | `239` | PASS |
| composition: frame 240 is outside | `false` | `false` | PASS |
| layer1: the exposure sheet supplies 240 expected drawing numbers | `240` | `240` | PASS |
| layer1: all 240 drawing numbers match the exposure sheet | `match` | `match` | PASS |
| layer2: the exposure sheet supplies 240 expected drawing numbers | `240` | `240` | PASS |
| layer2: all 240 drawing numbers match the exposure sheet | `match` | `match` | PASS |
| layer3: the exposure sheet supplies 240 expected drawing numbers | `240` | `240` | PASS |
| layer3: all 240 drawing numbers match the exposure sheet | `match` | `match` | PASS |
| layer4: the exposure sheet supplies 240 expected drawing numbers | `240` | `240` | PASS |
| layer4: all 240 drawing numbers match the exposure sheet | `match` | `match` | PASS |
| layer4: exposure count | `80` | `80` | PASS |
| layer4: five-frame hold covers frames 60-64 | `60-64 -> drawing 0` | `60-64 -> drawing 0` | PASS |
| layer4: one-frame accent at frame 152 | `152-152 -> drawing 10` | `152-152 -> drawing 10` | PASS |
| layer4: drawing numbers decrease at the re-exposure, and are accepted | `12,13,14,11,16,17` | `12,13,14,11,16,17` | PASS |
| layer4: frames 165-167 return the re-exposed drawing 11, not 15 | `11,11,11` | `11,11,11` | PASS |
| layer3: composition frames whose exposed drawing is missing | `14,15,38,39,62,63,86,87,110,111,134,135,158,159,182,183,206,207,230,231` | `14,15,38,39,62,63,86,87,110,111,134,135,158,159,182,183,206,207,230,231` | PASS |
| layer3: a present drawing resolves to its file | `layer3_006.png` | `layer3_006.png` | PASS |
| FX-TIME-001: frames 0..4 map to drawings 1,1,2,2,2 | `1,1,2,2,2` | `1,1,2,2,2` | PASS |
| FX-TIME-003: 24000/1001 is stored unreduced | `24000/1001` | `24000/1001` | PASS |
| FX-TIME-003: an equivalent rate reduces to the same pair, not to a decimal | `24000/1001` | `24000/1001` | PASS |
| FX-TIME-003: frame 1 is exactly 1001/24000 seconds | `1001/24000` | `1001/24000` | PASS |
| FX-TIME-003: the decimal label is display only | `23.976` | `23.976` | PASS |
| FX-TIME-003: 1001/24000 seconds converts back to frame 1 | `1` | `1` | PASS |
| FX-TIME-004: first frame | `-12` | `-12` | PASS |
| FX-TIME-004: last frame | `11` | `11` | PASS |
| FX-TIME-004: export frame count | `24` | `24` | PASS |
| FX-TIME-004: seconds at frame -12 are negative and exact | `-1/2` | `-1/2` | PASS |
| FX-TIME-002: requesting drawing 1003 diagnoses rather than substituting | `MEDIA_SEQUENCE_GAP` | `MEDIA_SEQUENCE_GAP` | PASS |
| FX-TIME-002: 1002 and 1004 either side still resolve | `seq_1002.png,seq_1004.png` | `seq_1002.png,seq_1004.png` | PASS |
| a frame exposing an absent drawing is a warning, not a substitution and not silence | `MEDIA_SEQUENCE_GAP` | `MEDIA_SEQUENCE_GAP` | PASS |
| layer-local: frame 99 is before the layer | `None` | `None` | PASS |
| layer-local: frame 100 maps to local 5 | `Some(5)` | `Some(5)` | PASS |
| layer-local: frame 109 maps to local 14 | `Some(14)` | `Some(14)` | PASS |
| layer-local: frame 110 is outside the half-open interval | `None` | `None` | PASS |
| layer-local: an inactive frame reads no file and renders transparent | `Transparent` | `Transparent` | PASS |
| rounding: 0.5 frames rounds away from zero to 1 | `1` | `1` | PASS |
| rounding: -0.5 frames rounds away from zero to -1 | `-1` | `-1` | PASS |
| rounding: 1.5 frames rounds to 2 | `2` | `2` | PASS |
| rounding: 2.5 frames rounds to 3, not to even | `3` | `3` | PASS |
| overlapping spans are rejected | `SpansNotDisjoint { previous_end: 5, next_start: 3 }` | `SpansNotDisjoint { previous_end: 5, next_start: 3 }` | PASS |
| a span covering no frame is rejected | `EmptySpan { start_frame: 4, end_frame_exclusive: 4 }` | `EmptySpan { start_frame: 4, end_frame_exclusive: 4 }` | PASS |
| a zero frame rate is rejected | `DegenerateFrameRate { numerator: 24, denominator: 0 }` | `DegenerateFrameRate { numerator: 24, denominator: 0 }` | PASS |
| an inverted UI range is rejected | `None` | `None` | PASS |
| a hole between spans is transparent, not an error | `None` | `None` | PASS |

## The missing drawing, as the user would read it

Document 20: "Sequence gaps are not collapsed. If drawing 1002 is referenced but absent, evaluation returns a missing-source diagnostic for 1002 rather than substituting 1001 or 1003." Layer 3 exposes drawing 7 on twenty of the 240 frames, and drawings 6 and 8 are both on disk, so substitution was available every time and did not happen.

```
WARNING [MEDIA_SEQUENCE_GAP]
Frame 2 exposes drawing 1003 of seq_%04d.png, which is missing.
Layer-local frame 2 maps to drawing 1003. No file in the sequence carries that number, so the frame renders transparent. No neighbouring drawing is substituted.
Add the missing file and relink the sequence, or change the exposure to a drawing that exists.
```

```
WARNING [MEDIA_SEQUENCE_GAP]
Frame 14 exposes drawing 7 of layer3_%03d.png, which is missing.
Layer-local frame 14 maps to drawing 7. No file in the sequence carries that number, so the frame renders transparent. No neighbouring drawing is substituted.
Add the missing file and relink the sequence, or change the exposure to a drawing that exists.
```

## Not run by this test

- Property keyframes: hold and linear interpolation, the before-first and after-last rules (document 20, "Property keyframes"). They belong to B-05 with the model commands that create them; nothing yet has a property to animate.
- Save and reopen of the exposure spans and the 24000/1001 rate (T-07, B-09). This test proves the rate is stored exactly in memory, not that it survives a round trip through the project file.
- Rendering any of these frames (B-05a, B-08). The table says which drawing each frame exposes, not what it looks like.
- Matte layers evaluating at the same composition frame (document 20); mattes are parked with R-04 under D-12.
- Work area, which the schema allows and no requirement yet consumes.
- Rate-limiting of the twenty layer-3 warnings into one summary carrying counts and ranges, which document 28 requires of frame-level diagnostics. `resolve` returns one diagnostic per frame by design, because it answers about one frame; the aggregation belongs to whatever drives the frame loop, which is B-08.
