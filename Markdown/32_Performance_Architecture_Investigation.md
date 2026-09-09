# Performance architecture investigation

Version 0.1 | 2026-09-08 | Investigation only

## What this document is, and what it is not

This is a research report. It changes no code, opens no decision, and closes none. It exists
because `verification/T-06_declared_fixture.md` ended with a number the project has not yet
answered — **267.17 ms a frame against the 41.7 ms a 24 fps clock allows, a factor of about
six** — and because the register entry that number opens has to be written against something.
This is that something.

Three kinds of statement appear below and they are kept apart on purpose, because document 12
requires it and because mixing them is how a plan becomes a wish:

- **Measured.** A figure from `verification/` or `spikes/`, quoted with the file that states it.
  Every one of these can be checked by re-running the test that produced it.
- **Read from the source.** A structural claim about what the code in `src/` does, which can be
  checked by opening the named file at the named function. These are facts about this build, not
  about its speed.
- **Estimated.** Arithmetic over the two above, or an expectation from outside evidence. Every
  estimate below is marked as one and carries the reasoning that produced it. **No estimate in
  this document is a performance claim, and none of them may be quoted as one.** The project's
  own rule is that `Markdown/` holds targets and `verification/` holds measurements; this file
  is in `Markdown/`.

Where a technique would change output pixels, that is stated in the technique's own paragraph,
because in this project that is the fact that decides whether it is a performance change or a
specification change.

Related documents: 06 (architecture), 08 (rendering, colour and the performance envelope), 21
(rendering math and the tile contract), 27 (cache and invalidation), 30 (competitive analysis).
Related records: ADR-006 (CPU-only, trigger-gated), ADR-011 (tile-based render), ADR-015
(bounded cache), ADR-017 (effects are not tiled). Related register entries: D-32, D-37, D-40,
D-41.


## Part 1 — Where the frame actually goes

### 1.1 What the measurements pin down

Four artifacts carry every performance number this project owns:

| Artifact | What it measured | Headline |
|---|---|---|
| `verification/B-08_preview_latency.md` | A draft preview frame of the four-layer reference shot | ~82 ms, of which ~75 ms is reading and decoding four cels and ~6.5 ms is rendering them |
| `verification/D-37_decode_cost.md` | One cel of each reference layer, file already fetched | 15.22 / 6.89 / 6.69 / 6.19 ms median; **34.98 ms for one cel of each** |
| `verification/T-06_performance_envelope.md` | The four-layer reference shot, whole work area | Sits on the 24 fps deadline and crosses either side of it between runs |
| `verification/T-06_declared_fixture.md` | Document 08 line 41's ten-layer, two-matte, three-effect fixture | **267.17 ms median frame, 4.1 fps cold, 0 of 10 loops inside the deadline** |

Two derived figures from the T-06 loop table are worth stating explicitly because the rest of
this document leans on them. Each loop over the 240-frame work area issues **1,290 decodes and
1,530 answers from memory** — 2,820 cel requests, which is **11.75 requests per frame**: ten
layers plus the two matte layers resolved a second time through their own transform, minus the
requests that find a deliberately missing drawing. Per frame that is **5.375 decodes and 6.375
cache hits**.

And the fixture was measured **at draft resolution**, which is `PreviewQuality::Draft`, a quarter
of the composition on each axis (`src/preview.rs`). The composited output of a frame in that
table is 480 × 270, not 1920 × 1080.

### 1.2 What the measurements do not pin down, and why that matters most

**There is no per-stage breakdown of a frame of the declared fixture.** B-08 split a *four-layer*
draft frame into "decoding" and "rendering" and nothing finer, and D-37 timed decode in isolation
on a different shot. Nothing in `verification/` says how the 267.17 ms divides between decode,
the conversion into the working space, cache bookkeeping, the whole-layer effect stack, mask
rasterisation, buffer allocation, and the tile loop.

That gap is the single most important finding in this document, and it is a process finding
rather than a technical one. Every candidate below is an argument about which part of a frame to
attack, and the project cannot currently rank those arguments by evidence. **The first thing to
build is not an optimisation. It is the instrument.** Section 7 names it as SP-P1.

What follows is the best reconstruction available from what has been measured, and it is
arithmetic, not measurement.

**Estimated frame budget for the declared fixture at draft, reconstructed:**

| Stage | Estimate | Reasoning |
|---|---|---|
| Decode on a miss | ~100 ms | 5.375 misses × ~18.75 ms. The per-cel figure is B-08's 75 ms over four cels, which includes finding and reading the file; D-37's 8.75 ms mean is the same work with the file already fetched |
| Answering a hit | ~20 ms | 6.375 hits × a full copy of a 33.2 MB buffer. See 1.3 |
| Everything else | ~145 ms | The remainder. Whole-layer Gaussian blur at source resolution, mask rasterisation, ten 33 MB working buffers built per frame, plan construction, and the tile loop |

The third row is 54% of the frame and is entirely unaccounted for. That is what SP-P1 is for.

### 1.3 Five things the code does that cost time and buy nothing

These are read from the source. They are not bugs — every one of them is a reasonable thing to
have written while getting the picture right, which was the correct priority — but each is work
the frame does not need to do, and each is bit-exact to remove.

**A cache hit copies 33 megabytes.** `CelCache::decoded` in `src/cache.rs` returns
`entry.1.clone()` on a hit and calls `self.store(key, buffer.clone())` on a miss. A
`WorkingBuffer` is 1920 × 1080 × 4 × 4 bytes = 33,177,600 bytes, so a hit is a 33 MB allocation
plus a 33 MB copy, and a miss pays that copy a second time on top of the decode. At the
reference machine's memory bandwidth a 33 MB copy is on the order of a few milliseconds; at
6.375 hits a frame that is an estimated 20 ms of a 267 ms frame spent duplicating buffers that
nobody mutates. Handing out `Arc<WorkingBuffer>` instead makes a hit a pointer increment and
removes the transient second copy from a miss. The bytes that reach the picture are identical
because they are the same bytes.

**Decoding is serial.** `rayon` appears in exactly one place in the crate: `src/render.rs` line
274, the tile loop. `compose::plan_frame_cached` walks `comp.layers_in_order()` in a `for` loop
calling `resolve_layer`, which calls `cache.decoded`, and the cache is taken as `&mut` — so the
signature itself forbids two layers decoding at once. On a machine with **24 hardware threads,
the ~100 ms of decode in a frame of the declared fixture runs on one of them.**

**The transfer function is evaluated per sample when it has 256 possible inputs.**
`ImageBuffer::from_srgb8_straight` turns each byte into a float, and `into_working` then calls
`color::srgb_to_linear` on every non-alpha sample, which is `((c + 0.055)/1.055).powf(2.4)`.
For one 1920 × 1080 cel that is **6,220,800 calls to `powf`**. The input to the whole chain is a
`u8`. A 256-entry table of `srgb_to_linear(dequantise_u8(v))`, built by calling the existing
function, replaces every one of those calls with a load from a one-kilobyte array, and is
bit-identical by construction rather than within a tolerance — the table *is* the function,
evaluated ahead of time over its entire domain.

**Every layer is sampled at every pixel of every tile, whether or not it is there.**
`render::render_tile` loops the whole layer stack for each pixel of the tile, and for each
layer computes the inverse transform of the pixel centre and calls `sample_bilinear`. There is
no test of whether the layer's transformed extent intersects the tile at all. Ten layers over a
480 × 270 draft frame is 1.3 million bilinear samples; at full resolution it is **20.7 million**,
and cel animation is mostly transparent — a character cel over a 1080p canvas may cover a tenth
of it. This is the work-elimination idea Fusion and Nuke are built around, and section 5.4
argues it is bit-exact here for a reason specific to document 21's math.

**The frame is written twice and the tiles are allocated once each.** `render::render` collects
every tile into a `Vec<(Tile, Vec<f32>)>` — one heap allocation per tile — then builds the output
with `WorkingBuffer::transparent`, which is `vec![0.0; width * height * 4]`, a 33 MB zeroing pass,
and then overwrites every byte of it from the tiles. The zeroing is dead, and the tile buffers
could be windows into the frame rather than copies into it.

### 1.4 The parallelism the draft path cannot use

`compose::DEFAULT_TILE_SIZE` is 128 and `PreviewQuality::Draft` is a quarter on each axis, so a
draft frame is 480 × 270 and `render::tiles(480, 270, 128)` returns **four columns by three rows
— twelve tiles**. Twelve tasks on twenty-four hardware threads, in the only phase of the frame
that is parallel at all.

At full resolution the same call returns 15 × 9 = 135 tiles and the pool is fed properly. So the
tile size is tuned for the export path and starves the preview path, which is the path the
performance target is about. This is a one-constant observation and it is offered as exactly
that: it does not fix the frame, because the tile loop is not the expensive part at draft, but it
means **the draft frame is currently both the slow case and the least parallel case.**


## Part 2 — How After Effects gets its performance, and what it cost them

After Effects is the interaction reference this project already names in document 30, so its
engineering history is worth reading closely: it is a layer-based compositor that spent
twenty-eight years single-threaded and then spent eighteen months buying its way out.

### 2.1 Multi-Frame Rendering: parallelism across frames, not within them

AE's answer to modern core counts is to render several *frames* concurrently rather than to
divide one frame. This shipped in public beta in July 2021 and by default in AE 22.0. It
replaced an older scheme, "Render Multiple Frames Simultaneously", which achieved concurrency by
launching hidden background copies of the entire application, each loading its own copy of the
project — process-level parallelism with a per-process memory cost, and a well-earned reputation
for instability.

The retrofit was expensive. Adobe's Sean Jenkin has described the work publicly as making
roughly 540 individually compiled sub-projects thread-safe across a codebase spanning about
thirty-five years, over about eighteen months, with the acceptance criterion that concurrent
rendering produce **bit-identical output to sequential rendering** — the same property ADR-011
asks of tiling here, arrived at from the opposite direction.

Independent benchmarking by Puget Systems found MFR delivering roughly **1.7× to 2.5× on
average and up to about 4.5× at peak**, with scaling falling off at high core counts, and — the
interesting part — a RAM overhead of only about 6% on average, because the frames share one
loaded project rather than duplicating it.

**What this project should take from it.** Frame-level parallelism is the right dimension when
per-frame work is a mostly serial dependency chain, which is exactly the shape here: decode,
then the effect stack, then the transform. It is also the dimension that would use the twelve
idle threads identified in 1.4. **What it should not take** is Adobe's reason for choosing it:
AE went frame-parallel partly because it could not go pixel-parallel through a plugin API that
assumed a single thread. This project has no such API and can have both.

### 2.2 The plugin API as an ossification constraint — the sharpest lesson available

AE's C/C++ effect API was designed for a single-threaded host, so effect code was free to keep
mutable global state. Making the host concurrent therefore could not be a host-side change.
Adobe's solution was an opt-in flag, `PF_OutFlag2_SUPPORTS_THREADED_RENDERING`, that a plugin
sets to declare its render function safe to call concurrently; plugins that do not set it are
serialised, and the effect controls panel shows a warning. Adobe additionally made
`sequence_data` const at render time, added a compatibility flag
(`PF_OutFlag2_MUTABLE_RENDER_SEQUENCE_DATA_SLOWER`) that gives each thread a private copy at a
performance cost, added a "Compute Cache" so one thread can compute a shared expensive value
while others wait, and shipped static-analysis tooling for third parties to find unsafe statics
in their own binaries.

The same shape appears in AE's region optimisation. "Smart Render" lets an effect render only
the sub-region that changed rather than the whole frame — and, per Jenkin, not all third-party
plugins implement it, so its benefit is inconsistent from project to project. And bit depth is
not a uniform internal pipeline: effects historically carry **three separate code paths** for 8-,
16- and 32-bit processing rather than one depth-agnostic path, which is why Adobe's own guidance
is to put 8- and 16-bit-only effects last in the stack, immediately before output, to limit
repeated conversions.

**The lesson, stated generally: the extension contract sets the ceiling of the optimiser.** AE's
API asks nothing of an effect — not its thread-safety, not its spatial support, not its bit
depths, not whether it is the identity for the current parameters — and therefore the host can
assume nothing and optimise nothing. Everything Adobe has recovered since has been recovered one
opt-in flag at a time, and each flag is a negotiation with an ecosystem of binaries they do not
own.

This project is currently on the right side of that line and does not seem to know it.
`Effect::bounds_expansion` in `src/effects.rs` already requires every effect to declare its
spatial support, and ADR-017 records why. **That declaration is the seed of an effect contract,
and section 6.3 argues it should be grown deliberately rather than left as a blur's private
detail.**

### 2.3 Caching: the right idea, conservatively applied

AE keeps a RAM preview cache and a disk cache, plus a persistent disk cache that survives
relaunch and is scanned for frames matching the current project state on reopen. Green bars in
the timeline mark RAM-cached frames; blue bars mark disk-cached ones.

How invalidation works is documented, unusually, in a patent: **US 7,103,839 B1**, "Tracking the
validity of cache frames in digital movie editing", filed 2000 by Adobe with Michael Natkin and
David Simons as inventors. Rather than a valid/invalid flag per node, each node in the
compositing tree keeps **interval lists** — pairs of (time interval, edit timestamp) — checked
against a global monotonically increasing edit counter, with **separate interval lists per edit
category**, so a transform edit and an effect-parameter edit invalidate different things. The
design is explicitly conservative: it may over-invalidate and must never under-invalidate.

That last sentence is the whole of AE's cache reputation. The engine cannot *prove* that a
cached frame equals what a re-render would produce, so when in doubt it throws the frame away.
Users experience this as a cache that empties for reasons they cannot predict.

A hard constraint worth noting because this project will hit the same wall from the other side:
AE requires each cached frame to occupy **one contiguous memory block**, capped at 2 GB, which
is why heavy comps fail abruptly with an out-of-memory dialog rather than degrading.

### 2.4 Why AE is still CPU-bound

AE's GPU surface is narrow. When the Mercury GPU-accelerated renderer rolled out in CC 2015.3 it
covered three native effects — Gaussian Blur, Lumetri Color and Sharpen — claimed at 2× to 4×
over CPU. Adobe's current documentation lists CUDA, Metal and OpenCL as supported, plus
GPU-accelerated panel compositing and a handful of GPU-only VR effects, and then says the thing
that gives the game away: "individual GPU technologies are less important than overall GPU
performance." That is an admission that the accelerated surface is small enough not to reward
vendor-specific work.

Jenkin's stated reason is the one that matters architecturally: **if one effect in a stack is
CPU-only and the next is GPU-only, pixels round-trip across the bus between them**, and on a
mixed stack that transfer can erase the speedup entirely. AE cannot keep the image resident on
the GPU because it cannot guarantee that the next thing to touch it lives there.


## Part 3 — How DaVinci Resolve gets its performance, and what it cost them

Resolve is the opposite bet. It began as a GPU hardware colour corrector and never acquired a
CPU fallback worth the name: choosing a GPU processing mode is mandatory, and an unsupported one
is a hard error rather than a slow path.

### 3.1 GPU residency as the default, not the optimisation

Because the entire Color page is a chain of GPU kernels, the image can stay resident in VRAM
across the whole node graph within a clip's evaluation instead of round-tripping to system
memory between operations. This is the exact thing AE cannot do, and it is not a tuning
difference — it is a consequence of every operation being on the same device by construction.

The strongest public evidence that this is real rather than marketing is **DCTL**, Blackmagic's
C-like GPU shader language for custom per-pixel colour operations. DCTL plugins are distributed
as *shader source* rather than compiled binaries, and are compiled and executed as GPU kernels
inside the node chain. A host can only accept source-level pixel kernels if the pipeline they
are injected into is itself a chain of GPU kernels.

Multi-GPU is organised by assigning stages or frames to devices — one GPU for decode and
debayer, others for grading — which is why Resolve is one of the few applications where a second
identical GPU produces close to linear real-world gains on RAW debayer and noise-reduction
chains.

### 3.2 Domain of definition and region of interest — the work-elimination idea

Fusion's manual documents two bounding boxes that travel through the node graph in opposite
directions, and this is the single most transferable idea in either product.

**Domain of Definition** is "a rectangular region that defines what part of an image actually
contains data", established when an image is created or loaded and then **recomputed and
propagated forward** through every node. A Text node over transparency has a DoD tight to the
glyphs; a blur downstream of it grows the DoD outward by the blur radius. It is inspectable in
the viewer.

**Region of Interest** is the backward-propagating counterpart: what the viewer or output
actually needs, walked backwards up the graph, each node asking its inputs for only the pixels it
requires — a blur asking for its own output box padded by the kernel radius.

**When a node renders, it intersects the current RoI with the current DoD and computes only
that.** Nuke's published NDK architecture describes the same two-pass structure under different
names: `_validate()` computes the region of definition bottom-up, `_request()` propagates the
region of interest top-down, and the engine only ever computes the intersection.

This is not a micro-optimisation. It is a decision that the compositor's job is to *avoid*
touching pixels, and it is worth more on sparse artwork than any amount of SIMD applied to
touching them anyway. Cel animation is the sparsest content there is.

### 3.3 Decode designed for the pipeline instead of inherited from it

The Blackmagic RAW SDK documents an explicit **read → decode → process** pipeline with each
stage async across CPU threads and GPU contexts, GPU decode backends for Metal, CUDA and OpenCL
with a CPU fallback on SSE4.1/AVX/AVX2, a configurable thread count, and
`PreparePipeline()`/`PreparePipelineForDevice()` calls that precompile GPU kernels ahead of
playback so the first frame does not stutter.

Two design choices in it are directly relevant here.

**Partial-resolution decode.** `BlackmagicRawResolutionScale` supports full, 1/2, 1/4 and 1/8,
meaning a lower-resolution image can be extracted **without fully decoding to full resolution
first**. That is the mechanism behind Resolve scrubbing RAW smoothly at a reduced viewer
resolution with no proxies generated. Compare section 1.1: this build's draft mode reduces the
*composite* to a quarter on each axis and decodes every cel at full resolution anyway. **Draft
quality currently buys nothing at all on the stage that dominates the frame.**

**Metadata late-binding.** Clip-level attributes are applied uniformly, while frame-level
metadata — white balance, exposure, ISO — is applied late, per frame. So changing white balance
is a re-process, not a re-decode. The general principle is: put the cheap, frequently-changed
parameters as late in the pipeline as possible, so that the expensive early stages stay cacheable.

Above the codec, Resolve keeps two more layers that avoid decode entirely: **Optimized Media**,
a per-source transcode to a lighter intraframe codec substituted transparently, and the
**Render Cache** (Smart and User), which bakes effects at the timeline and node level, caches to
formats chosen for alpha and dynamic range (16-bit float, ProRes 4444, DNxHR 444), caches during
playback or automatically once the user stops interacting, and **flushes only the changed
clip's cache** when something is modified.

### 3.4 What it cost them

Resolve's weaknesses are the mirror image of AE's, and the report would be dishonest without
them.

**No graceful degradation.** No supported GPU means no application, not a slow application.
There is no public documentation of how or whether Resolve tiles a frame that exceeds available
VRAM; what is visible to users is a "GPU memory is full" failure and a set of workarounds —
lower the timeline resolution, disable GPUs, reduce cache resolution. A pipeline that assumes
its device always fits the work has no answer when it does not.

**Fusion is a graft.** Fusion was eyeon's Digital Fusion, acquired in 2014 and embedded as a
page in Resolve 15 in 2018. It keeps its own caching model — a per-tool RAM cache with its own
"Limit Caching To" percentage, a "Leave At Least X MBytes" floor, LRU eviction and a low-memory
auto-purge — and Blackmagic's own manual notes that those preferences apply to **Fusion Studio
only**, with the embedded Fusion page inheriting Resolve's memory preferences instead. Two memory
models in one process is a documented fact, not a rumour, and it is the most credible explanation
for the long-standing user reports that the Fusion page is slower than standalone Fusion. Claims
about specific Fusion tools being single-threaded are widely repeated in the community and are
**not** confirmed by any primary Blackmagic source; they are noted here as unverified.

**Conservative cache flushing, again.** Modifying a cached clip flushes that clip's cache
wholesale, so a trivial parameter change on a heavily cached composite forces a full re-cache
before smooth playback returns. Both products arrive at the same compromise from opposite
architectures, for the same reason: neither can prove that a cached result still equals a fresh
one, so both throw away more than they must.


## Part 4 — What each could have taken from the other

Stated as four crossings, because each is a design lever this project gets to choose freely.

**AE should have taken Resolve's extension contract, not its GPU.** The GPU is downstream of the
contract. Resolve can keep pixels resident on the device because OFX and DCTL require a plugin to
declare its GPU support, its bit depths and its region behaviour before the host schedules it.
AE's API requires none of that, so AE's host cannot plan a stack — and every GPU effect it adds
risks being sandwiched between two CPU effects and paying for two bus transfers. Adobe has spent
a decade retrofitting declarations (`SUPPORTS_THREADED_RENDERING`, Smart Render, per-depth
paths) that a contract could have required from the beginning.

**Resolve should have taken AE's degradation.** A pipeline with no CPU path has no floor. AE is
slow without a GPU; Resolve does not run. For a tool whose users are individuals on whatever
machine they own — which is this project's user exactly — the floor matters more than the
ceiling.

**AE should have taken DoD/RoI; Resolve should have taken frame-level parallelism.** These are
the two independent axes of going faster: *do the work on more cores* and *do less work*. AE
picked the first and got it thoroughly (MFR); it has the second only partially and inconsistently
(Smart Render, unevenly implemented). Resolve picked the second and got it thoroughly (DoD/RoI,
per-node caching); it does not advertise a frame-parallel renderer at all. **Neither product has
both, and they are not in tension.**

**Both should have taken determinism, and neither could.** Both caches over-invalidate because
neither engine can prove that a cached result equals a fresh one. AE's own patent states the
conservatism as a design property. This is the crossing this project is already on the right side
of, and section 6.1 is about spending it.


## Part 5 — The techniques, ranked

Ranked by expected value against the T-06 gap, with the two facts that decide whether each is a
performance change or a specification change stated for every one: **does it change output
pixels**, and **what would demonstrate it**.

### Tier 0 — Exact, self-contained, no decision required

Everything in this tier is provably bit-identical, is confined to one or two modules, and needs
no owner decision beyond scheduling. If SP-P1 confirms the budget in 1.2, this tier is where the
first factor of two or three lives.

**T0-1. Share cached buffers instead of copying them.** `Arc<WorkingBuffer>` in `CelCache` and
in `LayerDraw::source`. Removes a 33 MB allocation and copy per cache hit and a second one per
miss. *Pixels:* identical — the same buffer. *Estimated:* ~20 ms of a 267 ms frame, plus a
reduction in peak working set and in allocator pressure. *Demonstrated by:* the existing
`verification/B-08b_cache_table.md` byte comparison, unchanged and still passing, plus a
before-and-after on the frame timer from SP-P1.

**T0-2. Decode the frame's cels in parallel.** The obstacle is `&mut CelCache`, not the decoder.
A cache behind a shared structure — a sharded map, or a mutex around a map of `Arc` entries with
in-flight de-duplication so two layers naming the same file decode it once — lets
`plan_frame_cached` resolve layers with `par_iter`. *Pixels:* identical; decode is a pure
function of the file. *Estimated:* the largest single win available in Tier 0 — ~100 ms of
serial decode across up to 24 threads, bounded below by the slowest single cel (D-37's background
at 15.22 ms) rather than by their sum. *Watch for:* diagnostics currently accumulate into a
`FrameLog` in layer order, and that order must survive, so the log has to be collected per layer
and merged in composition order rather than appended from threads.

**T0-3. Replace `powf` with a 256-entry table.** The sRGB decode chain's input domain is a byte.
*Pixels:* identical by construction — the table is built by calling `srgb_to_linear` on all 256
dequantised values, so it cannot disagree with the function it replaces. *Estimated:* removes
6.2 million transcendental calls per cel; the fraction of decode this represents is unknown and
is precisely what SP-P1 must report. *Note:* the reverse direction (`linear_to_srgb`, in
`WorkingBuffer::encode`) has a continuous domain and **cannot** be tabulated exactly. It is on
the export path, not the preview path, and should be left alone.

**T0-4. Stop zeroing and re-copying the frame.** `render::render` builds a 33 MB zeroed buffer
and overwrites every byte of it, having first allocated one `Vec<f32>` per tile. Tiles writing
into disjoint windows of the output remove one full-frame memset and N allocations per frame.
*Pixels:* identical; each tile already writes to a position fixed before any thread starts, which
is the property ADR-011 asks `tests/b05a_transform.rs` to prove and which this preserves exactly.

**T0-5. Hoist the per-pixel branches out of the inner loop.** `render_tile` evaluates
`match mode` inside `blend_pixel` once per pixel per layer, tests `layer.opacity != 1.0` once per
pixel, and tests matte coverage once per pixel. All three are constant for the whole layer.
Specialising the loop per blend mode, and per has-matte/has-opacity, is a mechanical change.
*Pixels:* identical — same operations, same order.

**T0-6. Size draft tiles for the draft frame.** Twelve tiles on twenty-four threads (section
1.4). Document 21 already says tile size "is a tunable measured on the reference machine, not a
constant chosen in advance", and `DEFAULT_TILE_SIZE` is currently a single constant serving two
very different extents. *Pixels:* identical — `verification/B-07_effects_table.md` already
renders the same frame at six tile sizes including three that do not divide the frame, and
requires six byte-identical results. **The evidence that this is safe already exists.**

### Tier 1 — Structural, high value, needs a decision

**T1-1. Domain of definition and region of interest.** Give every `LayerDraw` a bounding box in
composition space and skip the layer entirely for tiles it does not touch.

The bit-exactness argument is specific and worth spelling out, because it is stronger here than
in Fusion or Nuke. Document 21 says samples outside the source extent are transparent black, and
`sample_bilinear` implements that literally — a neighbour off the edge contributes its weight
times `(0,0,0,0)`. So for any destination pixel whose inverse-transformed sample point lies more
than one pixel outside the source extent, `src` is exactly `[0,0,0,0]`. Substituting that into
`composite::blend_pixel`: `over_pixel` returns `dst` unchanged, and the general blend equation
gives `Co = (1-0)*Cd + (1-Ad)*0 + 0*Ad*B = Cd` and `Ao = 0 + Ad - 0 = Ad` for multiply, screen
and add alike. **A transparent source is exactly the identity in all four of document 21's blend
modes**, so skipping it is not an approximation. The geometric bounding box — the transformed
source rectangle grown by one pixel for the bilinear footprint — is always safe and needs no
scan of the image. A tighter box from the alpha channel is also available and would help more on
character cels, at the cost of a scan.

*Pixels:* identical, by the argument above. *Estimated:* proportional to how much of the canvas
the layers do not cover, which for cel work is most of it; worth the most at Full quality and on
export, where the tile loop dominates. *Demonstrated by:* the strongest possible artifact — render
the fixture with and without culling and require byte equality over all 2,073,600 pixels, which
is the same shape of check as `verification/H-01_whole_picture_table.md`. *Note:* this must be
revisited if effect evaluation ever moves into the tile loop, which is exactly the condition
ADR-017 says voids that record.

**T1-2. Decode at the resolution the preview asks for.** Draft renders a 480 × 270 composite
from 1920 × 1080 cels. Resolve solves this two ways — partial-resolution decode in the codec
(BRAW's 1/2, 1/4, 1/8 scales) and Optimized Media on disk. PNG offers no partial decode, so the
options here are the second: a generated draft-resolution proxy cel beside or alongside the
source, or a downsampled cel held in the cache.

The arithmetic is dramatic. A draft cel is 1/16 the samples: **2,073,600 bytes instead of
33,177,600.** On the declared fixture, holding all 166 distinct drawings costs **about 344 MB at
draft against the roughly 5.5 GB `T-06_declared_fixture.md` quotes at full resolution** — the
difference between "166 distinct drawings do not fit in any budget this project will set" and a
figure comfortably inside the 1 GiB default that is already set. On the reference shot, D-37's
57 distinct drawings come to about 118 MB at draft.

*An inconsistency worth flagging while quoting these.* `verification/D-37_decode_cost.md` prices
57 cels at 473 MB on the stated basis that "a decoded cel is 1920 x 1080 x 4 bytes", which is
four bytes a pixel — the size of the *encoded* RGBA8 samples. `src/cache.rs` and
`verification/T-06_declared_fixture.md` both price a held cel at 33,177,600 bytes, which is
sixteen bytes a pixel, because what the cache actually holds is a `WorkingBuffer` of f32. Those
57 cels therefore cost about **1.89 GB** to hold, not 473 MB. The two artifacts are pricing
different things without saying so, and D-37's figure is the one that understates the decision it
was written to inform. This document has no authority to amend a verification artifact; it is
recorded here so somebody with that authority can.

*Pixels:* **changes them.** Downsampling then transforming is not the same as transforming then
downsampling. This is a specification decision, not an optimisation — but it is one document 05
has already made in principle: D-33 puts the preview at draft by default and requires "a visible
indication when preview quality differs from final export", and `PreviewQuality::differs_from_export`
already exists to say so. The export path is untouched: ADR-015 keeps the cache out of export
entirely, and `plan_frame` and `crate::export` pass `CelCache::none()`. So the change is confined
to a path that is already declared as not authoritative. *This is the largest single lever in
the document and the first thing that needs an owner decision.*

**T1-3. Exposure-aware caching — the thing neither competitor can do.** See section 6.2. This is
the report's main argument and it is separated out because it is a design proposal rather than a
technique off a shelf.

**T1-4. Multi-frame rendering.** Render several frames concurrently, as AE does. Attractive here
for the reason AE found it attractive — the per-frame chain is largely serial — and doubly so
because the draft tile loop cannot fill the machine on its own. *Pixels:* identical if frames are
independent, which they are: `plan_frame` takes a project, a frame and a root and reads nothing
mutable. *Cost:* memory, linearly in the number of in-flight frames, and this build's frames are
expensive to hold. AE's measured 1.7×–2.5× average with about 6% RAM overhead is the outside
reference; **AE's numbers are AE's and are quoted here as context, not as a prediction for this
build.** *Interaction:* T0-2 and T1-4 compete for the same threads and should not both be tuned
blind — which is another argument for SP-P1 first.

**T1-5. Prefetch on the exposure sheet.** During playback the compositor knows exactly which
drawings frames N+1..N+k need, because the exposure sheet says so. Decoding them on idle threads
ahead of the playhead is a pure latency win. AE's analogue is Speculative Preview, which renders
around the playhead during idle time; Resolve's is background caching once the user stops
interacting. **Both of them are guessing. This project would not be** — see 6.2.

### Tier 2 — Worthwhile, but only after Tier 0 and Tier 1

**T2-1. SIMD in the sample and blend loops.** The current state of the ecosystem, as of 2026:
`std::simd` remains nightly-only and is not production-ready; the stable choices are `wide`
(portable, no built-in multiversioning), `pulp` (stable, built-in runtime dispatch,
production-proven in `faer`), and `multiversion` to generate dispatch shims. Linebender's
`fearless_simd` is aimed squarely at 2D graphics kernels and is worth reading for design, but is
a prototype rather than a dependency.

*Pixels:* **can be identical, and the condition is precise.** Lane-parallel SIMD over
*independent pixels* performs the same f32 operations in the same order on each lane, so it is
bit-exact — provided fused multiply-add contraction is not enabled (an FMA has one rounding step
where the source has two) and no reduction reassociates a sum. Both are controllable. This
matters more here than in most projects, because `verification/H-01`..`H-04` compare whole
frames sample for sample, and a tolerance regime would be a real loss.

*Estimated:* four to eight times on the blend arithmetic itself with AVX2, which is the standard
expectation for eight-lane f32 work after load/store overhead; **not measured, and not measured
for this code.** *Caveat:* by the time Tier 0 and Tier 1 have landed, the blend loop may no
longer be the expensive part — which is the whole reason it sits in Tier 2.

**T2-2. Narrower cel storage.** A cel is stored as f32 RGBA. Premultiplied linear f16 halves the
memory traffic and doubles effective cache capacity; the transfer function is already applied so
the precision question is only about the linear values. *Pixels:* **changes them** — this is a
tolerance decision, and document 08 already anticipates the question: "assess half precision only
after image-difference tests." Note that T1-2 (draft-resolution cels) achieves a 16× reduction
against f16's 2× and does not touch the export path at all, so it should be settled first.

**T2-3. A faster decoder, or a different on-disk format.** Outside benchmarking on the QOI corpus
puts the Rust `image-png` family at roughly **340 MP/s on a Ryzen 9 7950X, about 1.9× libpng**;
at that rate a 1080p frame is about 6 ms of pure decode. This build already uses the `png` crate
(0.18.1) and D-37's smallest cel takes 6.19 ms, so **this build may already be close to the
decoder's floor and the remaining cost may be everything around it** — the `to_vec()` copy in
`decode_png`, the byte-to-f32 collect, and the `powf` pass of T0-3. That hypothesis is cheap to
test and SP-P1 should test it before any decoder is replaced.

If the format itself is the problem, the honest framing is that PNG optimises for compression
ratio and this workload wants decode speed. QOI decodes at about 226 MP/s against libpng's 66.6
MP/s on the same corpus at a middling ratio; QOIR adds tiling and back-reference compression.
A purpose-built cel container — planar, tiled, each tile independently compressed with LZ4 or
Zstd, memory-mappable, carrying a precomputed alpha bounding box for T1-1 and a mip chain for
T1-2 — is the design that falls out of everything above. **But it is a format decision with
import, relink and provenance consequences, it is squarely against document 04's anti-bloat
rule, and it should not be considered until T1-2 has been decided, because a draft proxy is the
same idea at a tenth of the cost.**

**T2-4. I/O.** Almost certainly not the problem, and this is worth saying so the project does not
spend time there. Current NVMe drives sustain many gigabytes per second, and a ten-layer 1080p
cel set is tens of megabytes. There is direct outside evidence that `io_uring` can be *slower*
than `mmap` when I/O volume is too low to amortise its bookkeeping. A readahead thread that opens
and reads the next few frames' files is worth having as part of T1-5; nothing more.

### Tier 3 — The GPU, and the fork it puts in front of the project

**ADR-006's reopening condition has been met.** The record says: "Reopen when a real shot is
measurably unpleasant to work with and the measurement is recorded." `verification/T-06_declared_fixture.md`
is that recording, on the reference machine, in a release build, against the fixture document 08
itself declares. The RTX 4070 Ti Super that ADR-006 explicitly left idle is still idle.

ADR-011 was written for this moment and is the reason the port is a port: "the unit of render
work is a tile ... it is the same decomposition a GPU compute dispatch requires."

What the project must decide first is not CUDA versus Vulkan. It is **what the fixtures compare.**

Modern GPUs are IEEE-754 compliant for basic operations, but shader compilers are permitted to
contract multiply-adds and reorder in ways that change results bit for bit across driver
versions, across vendors, and between API paths (Vulkan/SPIR-V versus CUDA on the same hardware).
`verification/H-01_whole_picture_table.md` requires **every one of 2,073,600 pixels exactly
equal** against an independently written compositor. A GPU path cannot promise that.

There are exactly two honest ways forward, and the choice belongs to the owner:

- **GPU as a preview tier, CPU as the sole authority.** Export stays on the CPU and stays
  bit-exact. The preview gains a GPU path and is compared to the CPU reference within a declared
  tolerance. This fits everything the project already believes: ADR-015 already keeps the cache
  out of export; D-33 already declares the preview as not authoritative; document 05 already
  requires the viewer to say when it differs. The GPU becomes one more thing the preview does
  differently and announces.
- **GPU everywhere, and the fixture regime moves to tolerances.** This is a larger change than it
  sounds. It would retire the strongest artifacts in `verification/`, and those artifacts are the
  project's stated primary quality control for an owner who does not read code.

**Document 08 already leans toward the first and pre-authorises the second's machinery:**
"Bit-identical results across every GPU are not promised" and "Document numeric tolerances per
operation before accepting optimized implementations." That sentence was written before there
was a GPU path and it is the sentence a GPU decision would be built on.

Two practical notes if it goes ahead. `wgpu` is the reasonable Rust choice over raw Vulkan or a
DX12-only path — portable, far less unsafe surface, headless rendering for export with no window
required — and compute shaders rather than fragment shaders, since compositing wants direct
read-modify-write into a storage texture and none of the rasteriser. And the transfer cost is
real: a 1080p RGBA f32 frame is 33.2 MB, so a design that uploads cels and downloads frames every
frame can spend its winnings on the bus. **The AE lesson from 2.4 applies directly — the win comes
from keeping the image resident, which means the whole stack must be able to run there, which
means the effect contract of 6.3 comes first.**


## Part 6 — What a ground-up architecture would look like

The brief asked what could be done from the ground up. Three things, in the order they should be
thought about.

### 6.1 Determinism is an asset, not only a constraint

This project has verified, whole-picture, bit-exact determinism: `H-01` through `H-04` check
frames against an independently written compositor and an independently written encoder, and
`B-10` exported the whole 240-frame shot twice and got 240 byte-identical pairs.

Both competitors' caches over-invalidate because neither can prove a cached result still equals a
fresh one — AE's own patent states conservatism as a design property, and Resolve flushes a
cached clip wholesale on any change. **A deterministic renderer does not have that problem.** If
the output of a stage is a pure function of a stated set of inputs, then a hash of those inputs
is a sound cache key, and a hit is not a guess.

That turns caching from a heuristic into a correctness-preserving transformation, and it is the
one thing this project already has that neither competitor can retrofit. Document 08 has already
written the key: "Cache keys include asset content, exposure frame, upstream revisions, effect
versions/parameters, working color configuration, scale, quality and time." What is missing is
not the specification. It is that the key is currently spent on one thing.

### 6.2 The exposure sheet is a schedule, and nothing else in the industry has one

`src/cache.rs` caches decoded cels keyed on path, length, mtime and interpretation. That is a
media cache. It knows nothing about exposure.

But exposure is this project's central concept and its stated reason to exist, and an exposure
sheet is a complete, exact, ahead-of-time description of which drawing every layer shows on every
frame. Three consequences follow, and no general-purpose compositor can have any of them:

**A held drawing is a guaranteed cache hit, and the compositor can know it in advance.** D-37
measured the shape: layer 1 of the reference shot is one drawing held for all 240 frames, it is
the most expensive of the four to decode at 15.22 ms, and 239 of its 240 requests produce a
buffer identical to the one before. That is not a probability to be discovered by an LRU; it is a
fact readable from the sheet before the first frame renders.

**Prefetch is exact rather than speculative.** AE's Speculative Preview renders around the
playhead during idle time and its edits throw the work away; Resolve caches in the background
once you stop interacting. Both are guessing what you will look at. A compositor holding an
exposure sheet and a playback direction knows precisely which files frames N+1..N+k need. T1-5 is
not a heuristic here.

**The stack has a stable prefix, and that prefix can be composited once.** Consider what
`render_tile` does across a run of frames where only the top layer's drawing changes: it
re-samples and re-blends every unchanged layer beneath it, identically, every frame. A background
held for 240 frames, under a character layer on twos, is transformed and blended 240 times to
produce 240 identical results. If the layers below the lowest layer that changed between frame N
and frame N+1 have identical drawings, identical transforms, identical effects and identical
mattes, then their composite is identical — **and a deterministic renderer can prove that rather
than hope it.** Cache the partial composite of the stable prefix, keyed on a hash of exactly
those inputs, and a frame's work becomes the layers that actually changed.

This is AE's cache idea and Resolve's node-cache idea applied at the granularity the domain
actually offers, and the domain offers it because holds are the artistic point. **Held cels are
not an edge case in cel animation. They are what cel animation is.** CONTEXT.md already says so:
"Holds are intentional artistic timing and must survive save, preview and export unchanged."

This is the report's answer to "how could this surpass them." Not by out-engineering Adobe's
thread pool or Blackmagic's GPU pipeline, which is not a fight worth picking with one owner and
no committed weekly capacity. By exploiting a structure in the work that a general-purpose
compositor cannot see, because a general-purpose compositor does not model exposure — and by
having the determinism that makes exploiting it provably safe rather than merely probably safe.

*Pixels:* identical, if and only if the key is complete. That "if" is the entire risk, and it is
the same risk document 27 already names when it lists what a cache key must include. The
verification is the one the project already knows how to write: render the shot with the prefix
cache and without it, and require byte equality — which is exactly the shape of
`verification/B-08b_cache_table.md` and would extend it.

*Scope note:* this is a substantial feature, it is squarely the kind of thing document 04's
anti-bloat rule exists to challenge, and it should be a G2 proposal with its own record rather
than something that arrives inside a milestone. Nothing in this section is a recommendation to
build it now.

### 6.3 Grow the effect contract deliberately

Section 2.2's lesson: the extension contract sets the ceiling of the optimiser. AE cannot plan a
stack because its API asks effects to declare nothing; Resolve can, because OFX and DCTL require
declarations before the host schedules anything.

`Effect::bounds_expansion` already exists in `src/effects.rs` and already requires every effect
to declare its spatial support — ADR-017 records that `verification/B-07_effects_table.md` checks
all four cases. That is one declaration. The ones that would matter next, in the order the
techniques above would need them:

- **Is this effect the identity for its current parameters?** A blur with sigma zero, a tint with
  amount zero. Cheap to answer and it removes the whole operation. Nuke and OFX both have this.
- **Is it separable, per-pixel, or neighbourhood?** A per-pixel effect can be fused into the
  sampling loop instead of materialising a whole intermediate layer buffer, which is what T2-2's
  memory pressure and ADR-017's whole-layer evaluation both push against.
- **Can it run on the device?** The declaration that makes GPU residency plannable, and the one
  AE has spent a decade retrofitting.
- **What is its version, and what does it hash to?** Already required by document 08's cache-key
  sentence, and load-bearing for 6.2.

None of these needs building now. The point is narrower: **every one of them is nearly free to
add while there is one effect module and three effects, and expensive to add once effects are an
extension point with anything outside this repository implementing them.** ADR-017 already notes
the condition that voids it. This is the same observation, one level up.


## Part 7 — Proposed spikes

Named so they can be scheduled or refused individually. Each states what it would measure and
what artifact it would leave. **None of these is authorised by this document.**

**SP-P1 — Where the frame goes.** A per-stage timer through one frame of
`verification/T-06_declared_fixture.json` at draft and at full: file open, decode, the
byte-to-f32 conversion, the transfer function, cache lookup and copy, mask, effect stack,
buffer allocation, tile loop, assembly. *Artifact:* a table that sums to the measured frame
time. **This is the prerequisite for every other item in this document**, because section 1.2 is
currently 54% "everything else". It is also the cheapest thing here.

**SP-P2 — The Tier 0 bundle.** T0-1 through T0-6 together, measured against SP-P1's baseline.
*Artifact:* the same table again, plus a byte-equality check over the whole 240-frame shot
against the pre-change build — which is `verification/B-10_full_shot_table.md`'s method applied
to a refactor. The claim to test is not a speedup figure; it is that the picture did not move.

**SP-P3 — Bounding boxes.** T1-1, geometric box first, alpha-derived box second, measured
separately so the extra cost of the alpha scan is visible against what it buys. *Artifact:* a
whole-picture byte-equality table in the shape of `H-01`, plus per-frame timings at Full quality
where the effect should be largest.

**SP-P4 — Draft-resolution cels.** T1-2, as a measurement of what it would buy and what it would
change: frame time, cache bytes held, and an image-difference table against the current draft
output showing how far the picture moves. *Artifact:* the difference table is the deliverable —
this spike exists to let the owner see the cost, not to argue for it. Export output must be shown
unchanged, which ADR-015 makes structurally true and which the spike should demonstrate anyway.

**SP-P5 — One GPU tile.** Before any GPU decision: port `render_tile` alone to a `wgpu` compute
shader, run the `H-01` fixture through it, and report **how far the GPU result actually is from
the CPU result** on the reference machine's own hardware. *Artifact:* a distance, in code values,
over 2,073,600 pixels. That number is what the owner needs in order to answer Tier 3's fork, and
it does not currently exist. The port is throwaway and belongs under `spikes/` with the rest of
the quarantined code.

**SP-P6 — Prefix reuse, on paper.** 6.2's stable-prefix idea, counted rather than built, the way
`verification/derive_d37_reuse.py` counted decode reuse without running the compositor: read the
exposure sheets of both fixtures and report how many frames of each shot have a stable prefix,
how deep it goes, and what fraction of layer-samples it would eliminate. *Artifact:* a table
derived from the fixtures alone. If the answer is small on real artwork, 6.2 is over and this
document was wrong about it.


## Part 8 — What this hands back to the owner

Four things, none of them decided here.

**The register entry T-06 asked for is still unopened.** `verification/T-06_declared_fixture.md`
ends by saying so: whether a 24 fps target for a ten-layer shot is one this project keeps,
lowers, or reaches by doing the decoding differently, "is a register entry somebody has to open,
and opening it is the owner's." This document argues the third option is available and does not
argue it is chosen.

**ADR-006's reopening condition is met.** The recorded measurement the record asks for exists.
That does not mean a GPU should be built; it means the record is now open rather than closed, and
Tier 3's fork — preview-tier GPU with a CPU authority, versus a tolerance-based fixture regime —
is the question inside it. SP-P5 exists to put a number in front of that question.

**T1-2 is the largest lever and it is a specification question, not an engineering one.**
Decoding at draft resolution changes preview pixels. D-33 and document 05 have already accepted
that a draft preview differs from export and must say so. Whether they accepted *this* difference
is the owner's to say.

**And the smallest recommendation is the one to act on first.** SP-P1. Half of a frame of the
fixture that this project's performance target is written against is currently unaccounted for,
and every argument above about what to fix is, until that table exists, an argument from
arithmetic rather than from measurement. Document 12's whole premise is that those are not the
same thing.


## Sources

External sources are listed with what they support. Where a claim rests on community consensus
rather than a vendor's own documentation, it is marked in the text and here.

**After Effects**
- After Effects SDK, multi-frame rendering and thread-safety contracts — `PF_OutFlag2_SUPPORTS_THREADED_RENDERING`, const `sequence_data`, the Compute Cache, the static-analysis tooling: https://ae-plugins.docsforadobe.dev/effect-details/multi-frame-rendering-in-ae/
- Adobe patent US 7,103,839 B1, "Tracking the validity of cache frames in digital movie editing" (Natkin, Simons; filed 2000, issued 2006) — interval lists, per-edit-category invalidation, conservative over-invalidation: https://patents.google.com/patent/US7103839B1
- ProVideo Coalition, "After Effects & Performance" parts 17 and 18, Chris Zwar's interviews with Adobe's Sean Jenkin — 540 sub-projects, the bit-identical requirement, Smart Render's uneven third-party adoption, three per-bit-depth code paths, the CPU/GPU round-trip argument, the project format parsed twice: https://www.provideocoalition.com/after-effects-performance-part-17-adobe-sean-jenkins/ and https://www.provideocoalition.com/ae-2022-multi-frame-rendering-has-arrived/
- Adobe Help, memory and storage — RAM/disk/persistent cache tiers, the contiguous-block requirement and 2 GB per-frame cap: https://helpx.adobe.com/after-effects/using/memory-storage1.html
- Adobe Help, GPU basics — supported APIs and the "individual GPU technologies are less important than overall GPU performance" statement: https://helpx.adobe.com/after-effects/using/basics-gpu-after-effects.html
- Puget Systems, MFR processor performance analysis — the independent 1.7×–2.5× average / 4.5× peak and ~6% RAM overhead figures: https://www.pugetsystems.com/labs/articles/after-effects-multi-frame-rendering-processor-performance-analysis-2217/

**DaVinci Resolve and Fusion**
- Fusion / Resolve manual, Domain of Definition and Region of Interest — the forward and backward bounding boxes and their intersection at render time: https://www.steakunderwater.com/VFXPedia/__man/Resolve18-6/DaVinciResolve18_Manual_files/part1700.htm and .../part1702.htm
- Resolve manual, render cache — Smart and User cache, node and timeline level, cache formats, per-clip flush on modification: https://www.steakunderwater.com/VFXPedia/__man/Resolve18-6/DaVinciResolve18_Manual_files/part246.htm
- Fusion memory preferences — per-tool cache, "Limit Caching To", LRU eviction, and the note that these apply to Fusion Studio only: https://www.steakunderwater.com/VFXPedia/__man/Resolve18-6/DaVinciResolve18_Manual_files/part1872.htm
- Blackmagic RAW SDK manual — read/decode/process stages, GPU backends with CPU fallback, `SetCPUThreads`, `PreparePipeline`, `BlackmagicRawResolutionScale` partial decode, clip- versus frame-level metadata: https://documents.blackmagicdesign.com/DeveloperManuals/BlackmagicRAW-SDK.pdf
- Puget Systems, hardware decode support in Resolve Studio — NVDEC and Quick Sync: https://www.pugetsystems.com/labs/articles/What-H-264-and-H-265-Hardware-Decoding-is-Supported-in-DaVinci-Resolve-Studio-2122/
- Fusion's history as eyeon Digital Fusion, acquired 2014, embedded in Resolve 15: https://en.wikipedia.org/wiki/Blackmagic_Fusion
- *Community consensus, not vendor-confirmed:* reports that the embedded Fusion page is slower than Fusion Studio, and that specific Fusion tools are single-threaded. Noted in 3.4 as unverified.

**Technique and implementation**
- Foundry Nuke NDK, 2D architecture — `_validate()` / `_request()`, region of definition and region of interest, rows and tiles: https://learn.foundry.com/nuke/developers/15.1/ndkdevguide/2d/architecture.html
- image-rs PNG adoption and benchmarks — ~340 MP/s on a Ryzen 9 7950X, ~1.9× libpng, on the QOI corpus: https://blog.image-rs.org/2026/06/18/png-adoption.html
- PNG decode profile — inflate ~56% of runtime, unfiltering ~18% and SIMD-reducible: https://github.com/image-rs/image-png/issues/415
- Blend2D PNG codec — a from-scratch decoder at ~2× spng/libpng/stb, with the tables: https://blend2d.com/blog/png-image-codec.html
- QOI benchmark — 226 MP/s decode against libpng's 66.6 MP/s: https://qoiformat.org/benchmark/ ; QOIR, tiled and back-reference compressed: https://nigeltao.github.io/blog/2022/qoir.html
- The state of SIMD in Rust — `std::simd` still nightly and not production-ready, `wide` versus `pulp`, `multiversion`: https://shnatsel.medium.com/the-state-of-simd-in-rust-2025-32c263e5f53d ; Linebender's `fearless_simd` design notes: https://linebender.org/blog/towards-fearless-simd/
- NVIDIA floating point and IEEE 754 — compliance for basic operations, and the latitude compilers have to contract and reorder: https://docs.nvidia.com/cuda/floating-point/index.html ; cross-API divergence reports: https://forums.developer.nvidia.com/t/floating-point-computations-on-vulkan-spir-v-differ-from-opencl-and-other-hardware/363502
- WebGPU buffer uploads — staging rings versus `queue.write_buffer`: https://toji.dev/webgpu-best-practices/buffer-uploads.html
- io_uring slower than mmap at low I/O volume, with the follow-up on what it takes to make it pay: https://www.conviva.ai/resource/we-replaced-mmap-with-io_uring-in-our-rust-query-engine-it-got-slower/
- Blender's compositor rewrite from tiled to full-frame with unified CPU/GPU numerical behaviour, and how they tested parity: https://projects.blender.org/blender/blender/issues/116694 and https://conference.blender.org/2024/presentations/3932/
- OpenColorIO GPU shader path, for how colour transforms move to a device without per-pixel CPU cost: https://opencolorio.readthedocs.io/en/latest/api/shaders.html
