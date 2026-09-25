# Effects and expressions specification

> **PARKED in version 0.3 under D-12.** The G1 effect stack (R-05) is not part of G1-core, and expressions (R-13) were a G2 concern with an undecided runtime per D-10. Effects were unparked on 2026-09-06 by the D-12 amendment and built by B-07. Expressions are specified in the section below, accepted on 2026-09-16 by the owner as D-59, the answer to D-10; that section replaces the earlier G2 proposal and keeps every rule it set.
>
> Revisit trigger for effects: repeated manual effort in finishing real shots that an effect would remove.
>
> Note for implementation when promoted: Gaussian blur has unbounded spatial support and therefore requires explicit tile-margin handling under the tile contract in ADR-011 and document 21. That interaction is part of why effects are not in the first milestone.


Version 0.2 | 2026-09-04 | Proposed baseline

## Effect contract

Every effect has an original native identifier, contract version, input/output alpha and color tags, typed parameters with units/defaults/ranges, spatial bounds, temporal dependencies and deterministic behavior. The registry must reject unsupported versions explicitly.

G1 effect set: exposure multiplies unpremultiplied linear RGB by 2 to the power of the exposure in stops and preserves alpha; Gaussian blur filters premultiplied RGB and alpha together with a specified kernel/truncation; tint replaces unpremultiplied RGB with the chosen color while preserving alpha. Avoid ambiguities around fully transparent pixels.

Suggested tests: zero exposure is identity; zero-radius blur is identity; blur of an impulse is symmetric and bounded by the declared support; tint does not change coverage. Specify radius units in source pixels and distinguish preview scaling from final rendering.

## Expansion priorities

After G1, investigate levels/curves, hue/saturation, thresholded glow, directional blur, color selection/keying, line recolor, morphology, distance-based gradients and edge smoothing. Edge smoothing was promoted on 2026-09-25 as D-86, line smoothing. Colour selection was promoted the same day as D-87, selective colour blur. Thresholded glow was proposed the same day as D-89, glow. Promote only those that solve observed W-01/W-03 tasks.

OLM's published catalog includes smoothing, cel-oriented blur, color keying, directional blur and highlight effects [S-01]. This supports researching those categories; it does not establish a required clone list or universal studio preference.

## Reuse boundaries

Classify each candidate as independent implementation, permitted source reuse, licensed integration or deferred. Retain upstream licenses and notices for permitted reuse and review the exact revision. A downloadable plugin binary is not automatically usable in a different host.

Keep a candidate record with visual goal, reference source, algorithm provenance, license status, quality fixture, temporal artifacts and maintenance owner. Legal review procedure: document 10.

---

## Native expressions

ACCEPTED on 2026-09-16 by the owner as D-59. This section is the language. `tools/expression_reference.py` is a whole second implementation of it, written from this section and sharing nothing with the build, and document 25's FX-EXPR cases are its output. Where this section and that file disagree, this section is wrong or the file is, and the fix is a specification change, never a build change.

The rules the earlier G2 proposal set are all kept: evaluation against an immutable project snapshot; no filesystem, network, process or native access; limits that actually stop runaway work; cycles refused; errors shown on the property that has them; references by stable ID, so a rename breaks nothing; randomness from a saved seed and a defined time; stated units, dimensions and conversions.

### What it is, and why not a scripting runtime

A small language this project reads and evaluates itself. Its words are After Effects' where After Effects has one, so an expression an animator already knows by heart - `wiggle(2, 30)`, `time * 90`, `loopOut("pingpong")`, `thisComp.layer(...).rotation` - is typed the same way here. It is not JavaScript, and nothing that is not listed below exists: there is no word for a file, a network, a process, a loop or a function definition, so no sandbox has to keep them out. The reasons for this choice rather than an embedded runtime are D-59's.

### Where an expression lives

Any of these properties may carry one: a layer's `anchor`, `position`, `scale`, `rotation`, `opacity` and `depth`, and the camera's `position`, `depth` and `zoom`. In the file it is an optional field of the property record, `"expression": {"text": "...", "enabled": true}`, written only when present. A switched-off expression is kept exactly and ignored entirely: the property is its keys.

An expression's result replaces the property's value at that frame, before anything else in documents 20 and 21 reads it. A parent, a child, the camera and the renderer all see the value after the expression.

### Units

An expression reads and gives every property in the file's own units, which are the window's units, with one exception: **opacity is 0 to 100 inside an expression** and 0 to 1 in the file, because that is what the window shows and what an After Effects user types. This holds whichever way the value is reached - `value`, another layer's `opacity`, or an opacity whose own expression is switched off (FX-EXPR-016). Scale is already a percentage in the file. Time is seconds.

Position, anchor and scale are lists of two numbers; rotation, opacity, depth and zoom are one number.

### Reading the text

- At most 4096 bytes of UTF-8. Longer is `EXPRESSION_SYNTAX`.
- Statements are separated by line breaks or `;`. `//` starts a comment that runs to the end of the line.
- A statement is either `name = expression`, which gives a local name a value, or a bare expression. The last bare expression is the result. `var`, `let` and `const` may be written before an assignment and mean nothing.
- A local name must be given its value on an earlier line than the one that uses it. A word of the language cannot be given a value.
- Numbers are decimal, with an optional fraction and exponent. Text is in single or double quotes, on one line, and exists only to name a layer or a kind of loop.
- Names are ASCII letters, digits and `_`. The operators are `+ - * / %`, with `*`, `/` and `%` binding tighter than `+` and `-`, unary `-` and `+` tighter still, and `.`, calls and `[ ]` tightest. Parentheses group.
- Every other character, and every name that is neither a word below nor an assigned local, is `EXPRESSION_SYNTAX` when the text is read, before anything is evaluated.

The words an expression may start from: `time`, `value`, `thisComp`, `thisLayer`, `thisProperty`, `Math`, `wiggle`, `loopOut`, `linear`, `ease`, `clamp`, `random`, `seedRandom`, `posterizeTime`.

### Values and arithmetic

A value is a number (a 64-bit float), a list of one to four numbers, or a text. A reference to a property, used anywhere a value is expected, is that property's value at the current frame.

- Number with number: the ordinary operation. `%` keeps the sign of the left side. Division by zero is allowed and gives an infinity or not-a-number, which the result check below refuses.
- List `+` or `-` list: item by item, and the lists must be the same length.
- List `*` number, number `*` list, list `/` number: every item.
- Anything else - a list times a list, a number plus a list, any operation on a text - is `EXPRESSION_TYPE`. **After Effects accepts `value + 5` on a position and does something surprising with it; this language refuses it.**
- `list[i]` needs a whole `i` inside the list, and only a list can be indexed.

### The result

The last value must be one number for a one-number property and a list of exactly the property's length for the others; every number in it must be finite; a camera's zoom must be greater than zero. Otherwise `EXPRESSION_TYPE`. An expression whose last statement gives nothing - `posterizeTime(8)` alone - is `EXPRESSION_TYPE`.

Opacity is then limited to 0..100 without a diagnostic, as the window limits a typed opacity. No other property is limited or repaired.

### Time

`time` is the composition frame in seconds: `frame * denominator / numerator`.

`posterizeTime(rate)` changes what `time` means for everything after it in the same expression: `floor(frame * denominator * rate / numerator + 1e-9) / rate`. At 24 frames a second, `posterizeTime(8)` holds each value for three frames (FX-EXPR-003). The rate must be finite and greater than zero.

`prop.valueAtTime(t)` reads a property at the frame nearest `t` seconds, rounding a half away from zero. **Only whole frames are ever evaluated**; there is no sub-frame value, which is D-58's no-shutter rule applied to expressions. A time before the first key or after the last reads the held key, as document 20 does.

### What can be named

- `thisComp.layer("id")` is a layer, **named by its ID and never by its name or index**, so renaming a layer or moving it in the stack cannot break a reference (FX-EXPR-009). An ID that names no layer is `EXPRESSION_REFERENCE_MISSING`.
- `thisComp.activeCamera` is the composition's camera, the default one of D-58 if the file gives none. `thisComp.width`, `thisComp.height` and `thisComp.frameDuration` (seconds) are numbers.
- `thisLayer` is the layer the expression is on. On the camera it is `EXPRESSION_SYNTAX`: the camera is not a layer.
- A layer answers `.anchor` (also `.anchorPoint`), `.position`, `.scale`, `.rotation`, `.opacity` and `.depth`, and `.transform` is the layer again so that `layer.transform.rotation` also reads. The camera answers `.position`, `.depth` and `.zoom`.
- A property answers `.value` and `.valueAtTime(t)`.
- `value` is this property's own keyed value at the current frame, before any expression.
- `thisProperty` is this property as its keys give it: `thisProperty.valueAtTime(t)` reads the keys at another time and is never a cycle.
- `Math.PI`, and `Math.sin`, `cos`, `tan` (radians), `abs`, `floor`, `ceil`, `round`, `sqrt`, `pow`, `min` and `max`. `Math.round` is `floor(x + 0.5)`. The square root of a negative number and an impossible power give not-a-number.
- Anything else asked of any of these - `.sourceText`, `.name`, `.effect(...)` - is `EXPRESSION_SYNTAX`.

A reference to another property is that property's **finished** value, after its own expression, at the frame asked for. It is never a world-space value: there is no `toWorld`, and a parented layer's position reads as the number in its own record.

### Functions

`linear(t, v0, v1)` and `ease(t, v0, v1)` map `t` from 0..1; `linear(t, t0, t1, v0, v1)` and `ease(...)` map it from `t0..t1`. Outside the range the result is held at the nearer end. If `t0 > t1` both pairs are swapped. `v0` and `v1` are both numbers or both lists of one length. `ease` shapes the fraction `f` as `f * f * (3 - 2 * f)`. **After Effects does not publish its `ease` curve, and this one is not claimed to match it**; it is the one FX-EXPR-007 pins.

`clamp(v, lo, hi)` limits a number, or every item of a list.

`loopOut(kind, n)` repeats this property's own keys after the last one. `kind` is `"cycle"` (the default), `"pingpong"`, `"offset"` or `"continue"`. `n` is how many keyframe spans before the last one to repeat, 0 meaning all of them. At or before the last key, or with fewer than two keys, it is `value`. With `first` the key where the loop starts, `last` the last key and `period = last - first`:
- cycle: the keyed value at `first + (frame - first) mod period`;
- pingpong: with `q = frame - last`, `laps = floor(q / period)` and `r = q mod period`, the keyed value at `last - r` on even laps and `first + r` on odd ones;
- offset: with `laps = floor((frame - first) / period)` and `r` the remainder, the keyed value at `first + r` plus `laps` times (value at `last` minus value at `first`);
- continue: the value at `last` plus `(frame - last)` times (value at `last` minus value at `last - 1`), the slope of the final frame.
FX-EXPR-006 has all four.

`wiggle(freq, amount, octaves = 1, amount_mult = 0.5, t = time)` adds noise to `value`, never to anything the expression computed. For each item `d` of the property:

    total = sum over k from 0 to octaves-1 of  amount_mult^k * noise(seed, call*256 + d*16 + k, t * freq * 2^k)
    result[d] = value[d] + amount * total

`call` counts the `wiggle` calls made earlier in this expression, from 0. `octaves` is rounded down and must be 1 to 16; `freq` must be zero or more; every argument must be finite.

`random()` is a number in [0, 1). `random(max)` is in [0, max), `random(min, max)` in [min, max), and either may be lists of one length, item by item. For item `d` of the `call`-th `random` in the expression it is `unit(mix(mix(seed, 65536 + call*16 + d), key))`, where `key` is the bit pattern of `time` as a 64-bit float - so a new number every frame - or 0 once `seedRandom` has made it timeless.

`seedRandom(n, timeless)` affects every `wiggle` and `random` after it. `n` is rounded down. A second argument other than 0 makes `random` give the same numbers on every frame (FX-EXPR-013); it does not stop `wiggle` moving.

### The noise, to the operation

Both implementations must agree on this bit for bit, which is why it is written out. All arithmetic is unsigned 64-bit and wraps.

    splitmix64(x): z = x + 0x9E3779B97F4A7C15
                   z = (z xor (z >> 30)) * 0xBF58476D1CE4E5B9
                   z = (z xor (z >> 27)) * 0x94D049BB133111EB
                   return z xor (z >> 31)
    mix(a, b)    = splitmix64(a xor splitmix64(b))
    unit(x)      = (x >> 11) * 2^-53                        a float in [0, 1)
    fnv1a64(s)   = FNV-1a over the UTF-8 bytes of s, offset 0xCBF29CE484222325, prime 0x100000001B3
    seed         = mix(fnv1a64(owner + "/" + property), n as 64-bit two's complement)
    lattice(i)   = 2 * unit(mix(mix(seed, stream), i as 64-bit two's complement)) - 1
    noise(x)     = with i = floor(x), s = x - i, w = s*s*s*(s*(s*6 - 15) + 10):
                   lattice(i) + (lattice(i+1) - lattice(i)) * w

`owner` is the layer's ID, or the composition's ID followed by `/camera`. `property` is the file's name for it (`position`, `rotation`, ...). `n` is the `seedRandom` seed, 0 if none was given. The seed comes from IDs and never from names or positions, so renaming or reordering a layer does not change its wiggle, and two properties with the same expression do not move together (FX-EXPR-004).

### Limits, cycles and failure

- **Cycles.** Evaluating a property at a frame while that same property at that same frame is already being evaluated is `EXPRESSION_CYCLE`. That includes an expression reading its own property through `thisLayer` - **After Effects quietly gives the keyed value there; this language says so instead** - and it excludes `value` and `thisProperty`, which read keys and never evaluate an expression (FX-EXPR-010).
- **Depth.** At most 16 property evaluations nested inside one another. A 17th is `EXPRESSION_TIMEOUT`. Two properties that each read the other one frame earlier never meet themselves at the same frame, so this is the limit that stops them - and going below frame 0 does not, because held keys go on for ever (FX-EXPR-011).
- **Steps.** At most 10,000 evaluation steps in one top-level evaluation, counting every value, operation, name and call, and counting the steps of every property it reads. The 10,001st is `EXPRESSION_TIMEOUT`. With no loops in the language this is a second wall rather than the first, and it is counted, never timed, so the same project stops at the same place on every machine.
- **Failure.** A property whose expression fails is drawn at its keyed value for that frame, and the diagnostic is shown on that property. A property that reads a failed one fails with the same identifier. Nothing is guessed. **An export whose range contains a frame on which an enabled expression fails is refused**, naming the property, the frame and the identifier, until the expression is corrected or switched off; the preview keeps playing on the keyed values with the error visible.

The five identifiers are in document 28: `EXPRESSION_SYNTAX`, `EXPRESSION_TYPE`, `EXPRESSION_REFERENCE_MISSING`, `EXPRESSION_CYCLE` and `EXPRESSION_TIMEOUT`.

## Compatibility ladder

Level A: similar creative capability implemented natively. Level B: documented equivalents for a small list of expressions or effects. Level C: an explicitly tested translator with unsupported-feature diagnostics. Level D: broad project/plugin compatibility, deferred as a separate product effort.

Adobe documents an expression language based on JavaScript with additional built-in objects [S-02]. Supporting JavaScript alone therefore does not establish AE compatibility. Expressions, application automation scripts, project import and compiled plugins are different interfaces and require separate decisions.

For any translator, record the tested input, expected native result, tolerance and known differences. Never silently approximate an unknown property or effect. A compatibility report must identify every translated, unsupported and manually resolved element.

## G2 acceptance

Test arithmetic, keyframe/time access, invalid dimensions, renamed references, cyclic references, runaway evaluation and seeded output reproducibility. A failed expression must be visible in preview and block final export until corrected or explicitly disabled. No expression engine is required for G1.

Related documents: 03, 04, 08, 10, 11 and 16.
