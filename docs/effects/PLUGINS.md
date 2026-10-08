# Plugins and compositor favourites: what to add to EFFECTS.md

Research note, 2026-10-08. For the owner to review before anything changes in `EFFECTS.md`.

## 1. Summary

**What was researched.**

- Third-party After Effects plugins that compositors buy or install instead of the built-in effects:
  - Trapcode Particular and Trapcode Form.
  - Deep Glow.
  - Video Copilot Saber and Color Vibrance.
  - Red Giant Universe Blurs & Glows and Universe Distortions.
  - Optical Flares, ReelSmart Motion Blur, Lenscare and PSOFT CelFX, which came up in Japanese compositors' own lists.
- What compositors recommend for:
  - Western VFX.
  - Japanese anime compositing, 撮影 (satsuei): diffusion, light wrap, chromatic aberration, line boil, gradients and para, 透過光 (transmitted light), flares, bokeh, grain, halation, aerial perspective, shadow and highlight tinting, camera shake and sparkle.
- Sources in both English and Japanese, listed in section 6.

**How to read it.** Every effect we would build has our own name. A product name appears only as the thing that was researched. Behaviour is described in my own words; no text, code or presets were taken from any product.

**Headline findings.**

1. **Glow is the most common reason people buy a plugin.**
   - Every source, English and Japanese, complains about After Effects' own Glow. It clips to white, its threshold has a hard step, and it falls off too quickly.
   - The fix people pay for:
     - a glow that fades slowly, the way real light does (described as "inverse square", meaning brightness drops with the square of distance; Deep Glow 2's panel calls its default falloff "Exponential");
     - worked out in linear light;
     - with a soft threshold.
   - Deep Glow 2's full control panel is now confirmed from the owner-supplied screenshots (section 2.3).
   - Japanese compositors build the same thing by hand: 3 to 6 blurred copies, each blur twice as wide as the last.
   - We have the parts (Glow, Bloom's four scales, float depth P0-9) but no shared engine for this. **Biggest single gap.**
2. **Lens flares come second.**
   - Both Japanese plugin rankings found put a flare plugin at or near the top, and Western reviews say the same.
   - We have no Lens Flare at all (row "Generate / Lens Flare", missing).
3. **Particles come third, but they are parked.**
   - Trapcode Particular is on every list, including anime studios' lists.
   - Our charter leaves particles out (D-309; roadmap item L-01, waits for the owner to ask by name). This note does not change that. It records that particles are the most-requested category.
4. **Most anime satsuei looks are already covered by our own effects:** Diffusion, Paraffin, Light Wrap, Kira-kira, Camera Shake, Chromatic Aberration, Cross Glare, Rim Light, Line Smooth and Selective Color Blur. What is left are small upgrades to these, not new effects:
   - a Soft Light pass on Diffusion;
   - a lens-style falloff on Chromatic Aberration;
   - a "re-seed every N frames" switch for line boil;
   - inner and outer colours on a glow.
5. **Two pieces of shared infrastructure would unlock most of the top 20** (section 5):
   - a shared soft-glow engine;
   - letting effects draw along a path. Our shape layers can already do this (D-78, D-169 trim), but no effect can use it yet.

## 2. Plugin candidates

The scores in every table are each 1 to 5:

- **Impact:** how much better finished work looks.
- **Effort:** how much work it is for us; higher means more.
- **Frequency:** how often a compositor would reach for it.

"Overlap" names the `EFFECTS.md` row and our type id.

### 2.1 Trapcode Particular, our name "Particle Emitter"

**What it does.**

- Throws out thousands of small sprites from a point, a box, a layer, a light, text or a 3D model.
- Moves them with physics: gravity, wind, turbulence, bouncing off a floor, flocking and fluid-like swirling.
- Shades them with After Effects' lights.
- Adds soft shadows inside the cloud and depth-of-field.
- Has a second particle system that can spawn from the first (trails, sparks off sparks).
- Runs on the GPU.

**Why people prefer it.**

- It is treated as the industry standard. The built-in CC Particle World is slower and has fewer controls.
- Japanese studios use it in anime: CGWORLD's Prisma Illya feature, and taka2composite's list of five plugins they keep using.

**Overlap.**

- The particle rows (Particle Playground, CC Particle World, CC Particle Systems II): all missing.
- P0-6: missing.

**Recommendation: park.**

- Keep it under L-01 until the owner asks for it by name.
- It needs a written rule for particle state carried from frame to frame (document 20), plus its own card path.

**Infrastructure.** P0-6, P0-12 (lights) and P0-4. **No change to the P0 list:** leave P0-6 parked, and note in the row that it is the most-requested plugin category.

**Score.** Impact 5, effort 5, frequency 5.

- Basis: on every list, and the biggest build in the whole catalogue.

### 2.2 Trapcode Form, our name "Particle Grid"

**What it does.**

- Particles that do not fly away. They sit in a fixed grid, a sphere, or the shape of a 3D model or layer.
- They ripple with noise, react to audio, and take colour or size from another layer.

**Why people prefer it.** No built-in effect makes a persistent 3D cloud of points.

**Overlap.** None directly. It would sit on the same engine as Particle Emitter.

**Recommendation: park with L-01.** Build it only after Particle Emitter; it would share that engine.

**Infrastructure.** P0-6, P0-12, P0-14 (audio, done).

**Score.** Impact 3, effort 5, frequency 2.

- Basis: impressive for music videos and titles, rare in narrative or anime shots.

### 2.3 Deep Glow, our name "Soft Physical Glow"

**Sources, updated 2026-10-08.** The owner supplied primary sources:

- three screenshots of the Deep Glow 2 effect panel from the seller's product page ("compact, Exponential", "compact, Iris", and "expanded", which opens every group);
- seven YouTube videos: the seller's launch video, the developer's quickstart, in-depth Iris Mode and Tone Mapping tutorials, and two showcase clips; plus Video Copilot's Saber launch tutorial (2.4).

What was read: the screenshots in full, and each video's title, description and chapter list. **No transcript was reachable** (YouTube returned empty captions), so nothing below rests on what is said inside a video. The product page itself still returned 403.

**What it does.** Earlier search snippets described a glow that falls off with the inverse square of distance, so it has a bright centre and a long, gentle tail. The Deep Glow 2 panel names its default falloff **"Exponential"**, not inverse square (screenshots). The Iris tutorial's chapters ("what is exponential?", "what is iris?") suggest Exponential and Lens Iris are the two modes in version 2. Whether an inverse-square mode is still offered is not shown.

The launch video's description lists what version 2 added: a glow driven by an image, cinematic tone mapping, red/green/blue radius multipliers, performance gains, a lens-dirt texture, a multi-colour tint and custom gamma correction. Every one of these shows up as a control group in the screenshots.

**Controls confirmed from the screenshots.** "C" means both compact screenshots show it, "E" the expanded one, "I" the Iris screenshot only.

| Group | Control (exact name) | Default shown | What it means, in my words | Seen in |
|---|---|---|---|---|
| Main | Glow Mode | Exponential / Lens Iris | Shape of the falloff: a smooth exponential tail, or a kernel taken from an image | C, E |
| Main | Blend Mode | Screen | How the glow goes back over the original | C, E |
| Main | Radius | 500.00 | How far the glow reaches | C, E |
| Main | Exposure | 1.00 | Glow brightness | C, E |
| Input | Threshold | 0.00% | Brightness below which nothing glows | E |
| Input | Threshold Smooth | 0.00% | Softens the threshold step, so highlights near it do not pop on and off | E |
| Input | Saturation Bias | 0.00% | Weights which pixels glow by how saturated they are, not only how bright | E (quickstart chapter "saturation bias") |
| Input / Input Masking | Mask Layer (+ Source/effects dropdown), Mask Mode (Alpha), Mask Invert | None | Limits what glows to another layer's alpha (or another channel), optionally inverted | E |
| Iris (Lens Iris mode only) | Iris Layer (+ Source dropdown) | a layer | The image whose shape the glow takes | I |
| Iris | Iris Sampling Quality | Standard | Speed against noise when sampling the iris | I |
| Iris | Iris Iterations | 10 | Probably how many sizes of the iris are layered (my reading of the name) | I |
| Iris | Radii Easing | 90.00% | Probably how those sizes are spaced (my reading of the name) | I |
| Style / Gamma Correction | Auto Detect Gamma, Gamma Info button, read-out "Scene Gamma: 2.20" | on | Works out the project's gamma so the glow is computed as if in linear light | E |
| Style / Aspect Ratio | Aspect Ratio, Enable Angle | 1.00, off | Stretches the glow into an oval, optionally at an angle | E |
| Style / Chromatic Aberration | Enable Pixel Aberration, Aberration Channels (Red & Blue), Pixel Offset | off, 1.00 | Shifts colour channels apart by some pixels | E |
| Style / Chromatic Aberration | Enable Glow Aberration, Multiply Red / Green / Blue, Shuffle Channels | off, 80% / 100% / 120% | A different glow radius for each channel, so the glow edge turns rainbow | E |
| Style / Tint | Tint Mode, Tint Blend Mode (Multiply), Color Inner, Color Outer, Swap Colors, Tint Strength | Off, 100% | Colours the glow, with one colour near the core and another at the outer edge | E |
| Style / Tone Mapping | Operation, Mix | None, 100% | Squeezes very bright values back into range instead of clipping, blended by Mix | E |
| Style / Lens Dirt Texture | Texture (+ Source), Opacity, Inherit Glow Color, Matte Exposure, Matte Gamma, Repeat Edges | None, 100%, 0%, 0.00, 1.00, on | A dirt image that lights up only where glow falls on it, optionally taking the glow's colour | E |
| Quality | Quality Preset, Half Float, Buffer Expansion Mode, Aux Buffer Expansion | Medium, on, Auto, 0 | Speed against accuracy; lower-precision buffers; how far the image grows past the layer's edges so the glow is not cut off | E |
| (bottom) | View | Final Result | Shows the result or an intermediate stage | C, E |
| (bottom) | Source Opacity | 100.00% | Fades the original under the glow | C, E |
| (bottom) | Unmult | on, labelled "(Required for Alpha)" | Turns black into transparency so the glow keeps a correct alpha | C, E |

**From the video chapter lists (titles only, not transcripts):**

- Iris mode can take shape layers, effects or photographs as the iris; the iris can be animated; its aspect ratio can change; there is a radius normalisation step; too few samples give visible noise patterns. The video points to Optical Flares' iris images as a source.
- Tone mapping offers several algorithms. The chapters name a "clamp chroma" method and an "ACES filmic" look, say several tone maps can be combined, and say the tone map can be used as a utility on any HDR image, not only on glows.
- The two showcase clips (Peter Clark; Ravie & Co) are demonstrations only; they confirm no controls.

**Not shown in any of the new sources:** controls named "Highlight Rolloff" or "Adaptation"; dithering; a downsample control (Quality Preset and Half Float may have replaced it); spread; GPU use. The GPU and dithering points rest on the earlier snippets only.

**What the confirmed controls mean for Soft Physical Glow and P0-21:**

- **Exponential falloff.** This fits the plan in section 5: several blurs, each double the last, added together, is an exponential-style tail. P0-21 needs no true inverse-square kernel to match the product's default mode. One Radius control can drive the whole stack.
- **Lens Iris.** The glow takes its shape from an image. My reading of the control names is that the image is used as a blur shape at several growing sizes; the exact method is not shown. This is far more costly and close to what Camera Lens Blur's shaped kernel already does. **Leave it out of the first version.** Record it as a later P0-21 extension that would reuse the shaped-kernel work behind Shaped Bokeh (item 12).
- **Threshold Smooth and Saturation Bias.** Both belong in P0-21's threshold stage, so every glow user gets them.
- **Input mask from a layer.** P0-3 (layer references, done) covers it.
- **Gamma correction.** We already work in linear light (P0-9, done), so there is nothing to auto-detect. Soft Physical Glow follows the project's working space. If a project is set up so that the glow cannot be computed linearly, that is diagnosed, not silently changed (document 28).
- **Aspect Ratio and angle.** An oval, angled glow. Put it in P0-21: Lens Flare's anamorphic stretch and the anime oval flare (3.2) want the same thing.
- **Glow Aberration (per-channel radius).** Confirms item 10 (Spectral Glow) as part of the same engine. **Pixel Aberration** is a plain channel shift before the glow; Shift Channels or Chromatic Aberration can do this already.
- **Tint with Color Inner and Color Outer.** This is exactly item 6, "Glow core-to-edge colour" (透過光). It is now confirmed as something a leading glow plugin ships, not only an anime habit.
- **Tone Mapping (Operation and Mix).** Make this the last stage of P0-21, off by default, so existing looks are unchanged. Which algorithms to offer is a decision for the owner; the product's exact list was not visible.
- **Lens Dirt Texture.** Confirms item 14, and gives its control set: a texture layer (P0-3), opacity, how much it takes the glow's colour, and exposure and gamma for the dirt matte.
- **Quality / Half Float.** We should **not** copy a half-precision switch that changes pixels; GPU and CPU must stay within 1 level in 255 (ADR-006). A quality preset is fine only if every setting has its own fixture.
- **Buffer Expansion.** The glow must be able to grow past the layer's edges. P0-21 needs a written rule for how far the output grows (tied to Radius), so a glow is never cut off at the layer bounds.
- **View.** A "show glow only" view is useful for checking; it costs little.
- **Source Opacity and Unmult.** Small. Unmult is the same maths as the Unmult row (item 7), so build that first and reuse it.

**Why people prefer it.**

- A review (edit-films) says After Effects' Glow shows a step at the threshold and burns to white, while the plugin works in linear light with a soft threshold.
- Japanese compositors list it as a must-have: note by tomoex; afuta-ya's top five puts it 4th.
- A Japanese tutorial (terriblejunkshow) explains how to tame the built-in Glow's white clipping: switch it to Screen and turn off compositing the original.

**Overlap.** These rows already do part of the job:

| Row | Type id | Status | What it already has |
|---|---|---|---|
| Stylize / Glow | `core.glow` | done | threshold, radius, intensity, add/screen, tint; "classic" and "after_effects" units |
| Ours / Bloom | `core.bloom` | done | four blur scales at sizes 1, 1/2, 1/4, 1/8; optional streaks |
| Ours / Diffusion | `core.diffusion` | done | none of the glow engine, but it is a related soft light |

**Recommendation: merge into `core.glow` as a new falloff mode called "physical",** so existing projects keep their look. Do not make a new effect.

- Build the falloff as the shared soft-glow engine in section 5, so that Bloom, Diffusion, Lightning's and Beam's glows, Halation, Lens Flare and Energy Stroke can use it too.
- First version: Exponential falloff, Radius, Exposure, Threshold, Threshold Smooth, Saturation Bias, input mask, aspect and angle, per-channel radius, inner/outer tint, Source Opacity, Unmult. Later: tone mapping, lens dirt, Lens Iris.
- **Dropped from the spec:** "Highlight Rolloff" and "Adaptation". They came from one review, and neither appears anywhere in the fully expanded Deep Glow 2 panel. If they were real, they were version 1 controls; tone mapping covers the same need.

**Infrastructure.** P0-9 (done), and the new shared soft-glow engine (section 5).

**Score.** Impact 5, effort 3, frequency 5.

- Basis: the most-complained-about built-in effect, used in almost every shot with light in it.

### 2.4 Video Copilot Saber, our name "Energy Stroke"

**What it does.**

- Draws a glowing energy line: a bright core with a soft glow around it, with built-in flicker and distortion.
- The core can be a straight line between two points, or it can follow a mask path, text outlines or a layer's alpha edge.
- Start and end offsets animate it on, like a write-on.
- It can render over black or onto transparency.

**Confirmed controls.** Only some were confirmed by the pages I could read:

- core size, start and end offset;
- glow colour, glow intensity and glow size;
- a black or transparent composite;
- presets.

**Confirmed features, 2026-10-08.** The owner pasted the feature list from Video Copilot's product page, and the Saber launch video's description (https://youtu.be/reSXGxkyr0k, a 37-minute tutorial) carries nearly the same list. Neither names individual controls. Only the description was read; no transcript was reachable.

- **Uses** (product page): energy beams, lightsabers, lasers, portals, neon lights, electricity and haze.
- **Features** (both sources): high-quality energy and light beams; realistic glow falloff; advanced core settings; built-in distortion; text and mask outlines; effects that stack.
- **Stacking in Add mode** (product page): several copies layer by adding light. For Energy Stroke this means it must composite additively over black or transparency, so two instances on one layer brighten where they overlap instead of covering each other.
- **Presets:** the product page says 50; the 2016 video description says 25. The count has probably grown since launch. We would ship our own presets, not theirs.

So **built-in distortion** and **advanced core settings** now count as confirmed features. The individual controls behind them (softness, flicker, glow spread, glow bias) are still only named in snippets, so they stay in the unconfirmed list.

**What this means for Energy Stroke.** "Realistic glow falloff" is the same need as Soft Physical Glow, which supports building Energy Stroke's glow on P0-21. "Haze" and "portals" show it is also used as a soft, wide glow, not only a thin line, so the glow radius needs a wide range.

**Why people prefer it.**

- It is free and looks far better than stroking a path and adding Glow.
- The glow falloff is described as realistic.
- It is on School of Motion's 2026 list of essential free plugins.

**Overlap.**

- Generate / Stroke: missing.
- Lightning (`core.lightning_bolt`), which has a soft core and glow (D-334).
- Beam (`core.beam`).
- Shape layers already stroke paths with trim (`src/shape.rs`, D-78, D-169), but no *effect* can draw along a mask or text outline.

**Recommendation: build separately.** It is its own effect, the way Lightning is. Build Stroke (missing row) first on the same path-drawing piece, then Energy Stroke on top of it.

**Infrastructure.**

- New P0, "effects draw along paths" (section 5).
- The shared soft-glow engine.
- P0-7 for text outlines (partial; outlines exist).

**Score.** Impact 4, effort 3, frequency 3.

- Basis: very popular for titles, magic and sci-fi. Needs the path piece first.

### 2.5 Video Copilot Color Vibrance, our name "Hot Colourize" plus "Unmult"

**What it does.**

- Pushes bright areas towards a chosen colour, so a white flash becomes a hot orange.
- A grey choice leaves the original colours.
- It also includes tools that make transparency out of footage shot on black (fire, sparks, stock flares), and that restore colours faded at feathered mask edges.

**Why people prefer it.** It is free and does in one step what otherwise takes Levels plus Tint plus a matte.

**Overlap.**

- Keying / Unmult (missing; native in After Effects 26.0).
- Ours / Tint (`core.tint`) and Gradient Map (`core.gradient_map`) already colourise by brightness.

**Recommendation: split it in two.**

- **Build the Unmult row separately.** It is small, sits on P0-9, and helps every flare and fire element.
- **Skip the colourise half.** Gradient Map covers it.

**Infrastructure.** None new.

**Score (for Unmult).** Impact 3, effort 1, frequency 4.

- Basis: stock elements on black are everywhere, and the maths is small.

### 2.6 Red Giant Universe Blurs & Glows

The Maxon product page did not list the effects. This list comes from Maxon's per-effect help pages (help.maxon.net), one per effect.

| Effect in the pack | What it does, in my words | Overlap (row, type id) | Recommendation | Impact / effort / frequency |
|---|---|---|---|---|
| Blur | Blur with a choice to blur in linear light, plus an angle | Gaussian Blur `core.gaussian_blur` (done) | **Skip.** Fold "blur in linear light" into the shared soft-glow engine rather than a new blur | 2 / 2 / 3 |
| Bokeh | Lens-shaped blur, 8 iris shapes, highlight boost, focus point, blur strength taken from another layer | Camera Lens Blur `core.lens_blur` (partial: no blur map) | **Merge** into `core.lens_blur` as a "blur map from a layer" setting. Called "Shaped Bokeh with blur map" in section 4 | 4 / 3 / 3 |
| Chromatic Glow | Rainbow-edged glow: each of red, green and blue is shifted and blurred separately, with a threshold and a highlight cut-off | Glow `core.glow` | **Merge** into Soft Physical Glow as per-channel radius. Called "Spectral Glow" in section 4 | 3 / 1 / 3 |
| Compound Blur | Blur whose strength comes from another layer's brightness | Compound Blur `core.compound_blur` (done) | **Skip**, already built | 1 / 1 / 2 |
| Edge Glow | Finds edges and makes them glow | Find Edges `core.find_edges`, Outline `core.outline`, Glow `core.glow` | **Skip** as an effect. Outline or Find Edges followed by Glow does it; offer it as a preset later | 2 / 1 / 2 |
| Glimmer | Animated multi-ray glints on highlights, up to six rays with their own colours, shimmer and looping | Cross Glare `core.cross_glare`, Kira-kira `core.kira_kira` | **Merge** into `core.cross_glare`: colour per ray and a shimmer setting. Called "Glint Streaks" in section 4 | 3 / 2 / 3 |
| Glo Fi | Glow that animates itself with fractal noise, aimed at text | Glow plus Fractal Noise `core.fractal_noise` as a layer map | **Skip.** Low demand, and the combination is already possible | 1 / 2 / 1 |
| Glo Fi II | Second version of the same idea with more glow layers | as above | **Skip** | 1 / 2 / 1 |
| Glow | Glow with an inverse-square-style falloff, isolation by channel or colour, a chromatic aberration option with RGB scale, noise and gamma | Glow `core.glow` | **Merge**: same as Soft Physical Glow (2.3) | (see 2.3) |
| Point Zoom | Zoom blur from a point, plus glow, for a "hologram" or "warp" look | Radial Blur `core.radial_blur` (zoom) plus Glow | **Skip** as an effect; offer as a preset. Called "Zoom Glow" in section 4 | 2 / 1 / 2 |
| Quantum | An animated light trail made from a layer, with three glows and a post glow | Echo `core.echo`, Glow | **Skip** for now. Echo plus Glow comes close; revisit if the owner asks | 2 / 3 / 1 |
| Spot Blur | Blur inside or outside a shape, with feather | Gaussian Blur on an adjustment layer with a mask (P0-8, P0-11, both done) | **Skip**, already possible | 1 / 1 / 2 |

Two more glow effects showed up under the same help categories but **are not in this pack**:

- **Optical Glow** belongs to Red Giant VFX Suite.
- **Shine and Starglow** are, as far as I know, Trapcode products. That is from memory, not confirmed by a page in this research.

Optical Glow's idea is the same as Soft Physical Glow and needs no separate row. Starglow's idea, streaks from highlights, is what `core.cross_glare` already does.

### 2.7 Red Giant Universe Distortions

**Not fully confirmed:** the Maxon product page did not list the pack's contents, so I cannot say this is the whole pack. I found four effects through help.maxon.net:

| Effect | What it does, in my words | Overlap (row, type id) | Recommendation | Impact / effort / frequency |
|---|---|---|---|---|
| Chromatic Aberration | Colour fringing that grows from a centre point outward. Each channel can be distorted and scaled separately; the fringes can be blurred (straight or radial) and roughened with a lens texture; optional linear light | Ours / Chromatic Aberration `core.chromatic_aberration` (done: only amount and centre) | **Merge.** Add per-channel scale, a falloff (none at the centre, strongest at the edges), and blur on the fringes. Called "Lens Chromatic Aberration" in section 4 | 4 / 2 / 5 |
| Heatwave | Rippling heat shimmer that drifts in a chosen direction and speed, with blur and a mask | Turbulent Displace `core.turbulent_displace` (done) | **Merge** as a drift (direction and speed) setting on Turbulent Displace, if anything. Called "Heat Shimmer" in section 4 | 2 / 2 / 2 |
| Prism Displacement | Bends the image using another layer's brightness, with a different bend for each colour channel (like light through glass), plus some lighting | Displacement Map `core.displacement_map`, Glass `core.glass` (partial) | **Merge** as a "spread by channel" setting on Displacement Map. Called "Map Chromatic Displacement" in section 4 | 2 / 3 / 2 |
| RGB Separation | Red and blue offset in a straight line by a distance and angle, green stays put, with optional distortion | Shift Channels `core.shift_channels`, Offset `core.offset`, Chromatic Aberration | **Merge** into Chromatic Aberration as an "offset" mode beside the current "radial" mode | 3 / 1 / 4 |

A related effect from Red Giant VFX Suite, not this pack: **Chromatic Displacement** treats a map as a height map, so colours separate smoothly along slopes. A Japanese anime compositor (Qiita, median_ky) recommends it for natural-looking colour fringing on water. It is the same idea as Prism Displacement and goes into the same merge.

### 2.8 Other plugins that Japanese compositors put on their lists

Two Japanese lists name the plugins working compositors keep installed:

- **taka2composite's five:**
  1. Optical Flares.
  2. Trapcode.
  3. ReelSmart Motion Blur.
  4. PSOFT CelFX.
  5. Lenscare.
- **afuta-ya's top five:**
  1. Element 3D.
  2. Optical Flares ("always first", in his words).
  3. ReelSmart Motion Blur.
  4. Deep Glow 2.
  5. Sound Keys.

Most of these are covered elsewhere. The new ones are:

#### Optical Flares (Video Copilot), our name "Lens Flare"

**What it does.**

- Builds a lens flare from parts: glows, streaks, rings, iris ghosts along the flare's axis, and texture-based elements.
- Can sit on a 3D light, and can carry several lights in one instance.

**Why people prefer it.**

- After Effects' own Lens Flare looks dated and fake.
- taka2 says the plugin makes it easy to add the dirt and colour fringing of a real lens.
- ProVideo Coalition's review says it looks like real lights in the shot.

**Overlap.** Generate / Lens Flare: missing.

**Recommendation: build it separately**, filling the missing row. Base the glows and streaks on the shared soft-glow engine.

**Infrastructure.** Shared soft-glow engine, P0-3 (done) for a lens-dirt texture, and Unmult.

**Score.** Impact 5, effort 4, frequency 4.

#### ReelSmart Motion Blur, no name proposed

**What it does.** Adds motion blur after the fact by estimating how pixels move between frames.

**Overlap.**

- Vector Blur `core.vector_blur`: partial.
- CC Force Motion Blur: partial.
- P0-13 (motion estimation): missing, and the charter leaves it out (D-309).

**Recommendation: park.** It is third on both Japanese lists, which is worth recording, but it needs P0-13.

**Score.** Impact 4, effort 5, frequency 4.

#### Lenscare, depth-of-field and bokeh

Same need as Universe Bokeh above: merge into `core.lens_blur` with a blur map.

#### PSOFT CelFX, eleven anime tools

The eleven tools are BlurCel, BlurRGB, BoundaryLine, ColorSelection, Deband, DeltaFX, Extend, Fill, Gradient, ReplaceColor and Texture. OLM's free tools cover similar ground (Smoother, Toon Dilate, OLM Blur). Most of this is already ours:

| Tool's job | Our effect |
|---|---|
| Line anti-aliasing (OLM Smoother) | Line Smooth `core.line_smooth` |
| Blur inside a colour region, for cel gradients (OLM Blur, BlurCel) | Selective Color Blur `core.selective_color_blur` |
| Line colour | Line Recolor `core.line_recolor` |
| Line width (Toon Dilate) | Line Width `core.line_width` |
| Choosing colours | Select Color `core.select_color` |

The one clear gap is **Deband**: smoothing the visible steps in a soft gradient. It is a small separate effect.

**Score (for Deband).** Impact 2, effort 2, frequency 2.

- **Not checked:** I did not confirm CelFX Deband's own controls.

## 3. Compositor recommendations (VFX and anime satsuei)

### 3.1 How the anime satsuei workflow differs from Western VFX

**Western VFX compositing** makes made-up elements look photographed:

- Work in linear light.
- Match grain, blur and lens faults to a real camera.
- Light-wrap the background onto the foreground.
- Add motion blur from real motion.

The effects it wants are accurate copies of optics, and particles and flares are often bought in.

**Anime satsuei (撮影)** starts from flat cel paint and line art. It adds light and air that the drawing does not have:

- It works largely with **stacked copies of the frame and blend modes**, not single effects:
  - Lighten (比較(明)) for diffusion;
  - Multiply for para (パラ) shadows;
  - Screen or Add for light;
  - Soft Light or Overlay for atmosphere.
- It **cares about line art.** Lines must stay crisp, get anti-aliased, and sometimes boil or change colour. Western tools have nothing for this.
- Its "camera" is drawn: shake, focus pulls and flares are added on top for feel, not to match a real lens.
  - Even a high-end studio feature (Kimi no Iro, Adobe Japan interview) says the photography team mostly used built-in effects: fractal noise texture, curves, lens distortion, diffusion and a wiggle-driven camera shake.
- Typical satsuei looks, named in the sources:
  - 透過光 (transmitted light, a white-hot core with a coloured halo);
  - ディフュージョン (diffusion);
  - パラ (para gradient shadows);
  - 空気遠近 (aerial perspective);
  - 十字光 (cross light);
  - ghost flares, sparkle;
  - and a stretched oval or anamorphic flare (CGWORLD on Prisma Illya and Attack on Titan Final Season).

For us this means many anime looks are **better served as small upgrades to effects we already have** than as new plugins.

### 3.2 Each recommendation

| Look | What compositors do, in my words | Overlap (row, type id) | Recommendation | Infrastructure | Impact / effort / frequency | Sources |
|---|---|---|---|---|---|---|
| Diffusion (ディフュージョン) | Blur a copy of the frame and lay it back in Lighten, often around 50%. Many then add a second copy in Soft Light or Overlay at around 50% (tomoex). A RETAS lesson uses a Lighten layer blurred about 50 px plus a 30% overlay layer. taka2 calls Lighten "essential" for diffusion | Ours / Diffusion `core.diffusion` (done: blur, amount, blend screen/lighten/normal) | **Merge.** Add a second "soft light" or "overlay" pass with its own amount, so the common two-layer stack is one effect | none | 4 / 1 / 5. Done in nearly every anime shot; the second pass is the missing half | tomoex, taka2 blend modes, RETAS lesson |
| 透過光 (transmitted light) | A core layer under 3 to 6 glow copies, each blurred twice as much as the last, in Screen or Add. Inner layers brighter, outer layers more muted or coloured. RETAS describes it as a white centre with a slightly blue edge | Glow `core.glow`, Bloom `core.bloom` (4 scales) | **Merge** into Glow: the "physical" falloff (2.3) plus an **inner colour and outer colour** pair. Called "Glow core-to-edge colour" in section 4 | shared soft-glow engine | 4 / 2 / 4. The signature anime light; today needs several stacked effects | ishimaru note, RETAS lesson, animestyle lecture |
| Light wrap | Blur the background and let it bleed over the foreground's edges. Called a "cornerstone of compositing" that After Effects lacks natively | Ours / Light Wrap `core.light_wrap` (done) | **Skip.** Already built; re-check its blur against the shared soft-glow engine when that lands | none | 2 / 1 / 4 | lesterbanks |
| Chromatic aberration | Split the channels, weak in the middle and strong at the edges, often together with lens distortion. Japanese tutorials do it with a channel-shift setup; for water, the map-based plugin above | Chromatic Aberration `core.chromatic_aberration`, Optics Compensation `core.optics_compensation` | **Merge** (same as 2.7) | none | 4 / 2 / 5 | School of Motion, Qiita median_ky |
| Line boil | Lines that wobble a little and jump to a new random shape every few frames, like hand-drawn lines redrawn each frame. The usual recipe: Turbulent Displace at a small amount, with its seed changed every 2 to 4 frames by an expression (`posterizeTime`) | Turbulent Displace `core.turbulent_displace` (done; no held seed). Camera Shake already has a "hold" setting | **Merge.** Add "new seed every N frames" to Turbulent Displace, like Camera Shake's hold. Called "Line Boil" in section 4 | maybe a shared "held random seed" rule (section 5) | 4 / 1 / 4. Common in hand-drawn-look work; today needs an expression | PremiumBeat, Adobe community thread |
| Para and gradients (パラ, グラデーション) | A gradient laid over the frame in Multiply to darken (para), or a colour gradient on hair and cheeks inside the cel | Paraffin `core.paraffin` (done, includes multiply), Gradient Ramp `core.gradient`, 4-Color Gradient, Selective Color Blur | **Skip.** Already covered | none | 1 / 1 / 5 | taka2 blend modes, hotakasugi |
| Flares (ghost flare, 十字光, oval or anamorphic flare) | Stretched flares, ghosts along a line through the centre, cross light | Lens Flare (missing), Cross Glare `core.cross_glare` | **Build separately:** Lens Flare, with an anamorphic-stretch setting (2.8) | shared soft-glow engine | 5 / 4 / 4 | animestyle, usapen, CGWORLD (AoT FS, Prisma Illya), taka2 plugins, afuta-ya |
| Bokeh and focus | Lens-shaped blur with bright discs, focus set by a map or a distance | Camera Lens Blur `core.lens_blur` (partial) | **Merge** (same as 2.6 Bokeh) | P0-3 (done) is enough for a map from a layer; P0-5 only for real depth passes | 4 / 3 / 3 | taka2 plugins (Lenscare), usapen |
| Film grain | Grain mostly in brightness, less in colour, strongest in the midtones, laid on in Overlay; or a scanned grain plate | Noise & Grain / Add Grain (missing), Noise `core.noise` (done, flat) | **Build separately**, filling the Add Grain row | P0-19 (done) | 4 / 2 / 4. Every "filmic" grade; flat noise looks digital | Adobe Camera Raw grain docs, film-grain explainers (section 6) |
| Halation | A red-orange halo hugging high-contrast bright edges, the way film's backing layer scatters light | none | **Build separately**, small, on the shared soft-glow engine | shared soft-glow engine | 3 / 2 / 3. Popular "film look" ingredient | vsco halation explainer, darkroom docs, miracamp |
| Aerial perspective (空気遠近) | Distant things get *less contrast*, not just brighter: raise the black point, then shift a little towards blue (taka2) | Levels `core.levels`, Tint `core.tint`, Color Balance `core.color_balance`. Ours / Distance Gradation is about distance from line art, so it is not this | **Build separately** as a small "Aerial Haze" effect (lift blacks, blue shift, amount), or leave it as a Levels-plus-Tint recipe | none | 3 / 1 / 4. Every background with depth | taka2 aerial perspective |
| Shadow and highlight tinting, backlight | Darken the character, add a blurred Screen edge light, and use Lighten or Overlay layers for colour | Rim Light `core.rim_light`, Color Balance, Gradient Map, Tint (all done); Photo Filter and Shadow/Highlight rows missing | **Skip** new effects; already covered | none | 2 / 1 / 4 | Qiita median_ky, hotakasugi |
| Camera shake | Wiggle-style shake, sometimes held on frames | Camera Shake `core.camera_shake` (done) | **Skip**, already built | none | 1 / 1 / 4 | Kimi no Iro interview, usapen |
| Sparkle (キラ) | Star glints that twinkle on highlights | Kira-kira `core.kira_kira` (done), Cross Glare | **Skip** as an effect; Glimmer's extras merge into Cross Glare (2.6) | none | 2 / 1 / 3 | animestyle, usapen |
| Lens dirt | A dirt texture lit up only where bright light hits the lens | none; it would be part of Glow and Lens Flare | **Merge**: a "dirt texture from a layer" setting on Soft Physical Glow and Lens Flare | P0-3 (done) | 3 / 2 / 3 | Deep Glow 2 panel screenshots (Lens Dirt Texture group) and launch video description, taka2 plugins (Optical Flares) |
| Rays (crepuscular, 薄明光線) | Light shafts from a bright source | Light Rays `core.light_rays` (partial) | **Skip**; finish the existing row | none | 2 / 1 / 3 | usapen, animestyle |
| Particles: dust, snow, sparks, steam | Particles, or Fractal Noise for steam, mist and dust | Snowfall, Rain, Fractal Noise (done); P0-6 missing | **Park** (L-01) | P0-6 | 5 / 5 / 5 | animestyle lecture, CGWORLD Prisma Illya, taka2 plugins |

## 4. Ranked list (top 20)

**How it is ordered.** Roughly by (impact + frequency − effort). Items that build the shared soft-glow engine come earlier, because later items reuse it. This is a judgement, not a formula. Parked items are shown last whatever their score.

| Rank | Our name | Merge / separate | Impact / effort / frequency | Reason |
|---|---|---|---|---|
| 1 | Soft Physical Glow | Merge into `core.glow` as a "physical" falloff mode | 5 / 3 / 5 | Top complaint about the built-in Glow in every source. It builds the shared engine items 6, 9, 10, 12, 13 and 14 use |
| 2 | Diffusion soft-light pass | Merge into `core.diffusion` | 4 / 1 / 5 | The standard anime two-layer diffusion in one effect; tiny change |
| 3 | Lens Chromatic Aberration (with offset mode) | Merge into `core.chromatic_aberration` | 4 / 2 / 5 | Ours lacks the falloff, per-channel scale and fringe blur that both plugins and tutorials use; RGB Separation folds in as a mode |
| 4 | Line Boil | Merge: "new seed every N frames" on `core.turbulent_displace` | 4 / 1 / 4 | Removes the expression everyone uses; same idea as Camera Shake's hold |
| 5 | Film Grain | Separate, fills the Add Grain row | 4 / 2 / 4 | Missing row; brightness-weighted midtone grain is what "filmic" means; uses P0-19 |
| 6 | Glow core-to-edge colour (transmitted light, 透過光) | Merge into `core.glow` | 4 / 2 / 4 | Signature anime light: white core, coloured halo, in one effect instead of a stack. Deep Glow 2 ships the same thing (Color Inner / Color Outer, confirmed from screenshots) |
| 7 | Unmult | Separate, fills the existing Unmult row | 3 / 1 / 4 | Small; every stock flare or fire element shot on black needs it |
| 8 | Aerial Haze | Separate, small (or leave as a recipe) | 3 / 1 / 4 | Contrast-down plus blue-shift is in every deep background; easy |
| 9 | Lens Flare | Separate, fills the Lens Flare row | 5 / 4 / 4 | Second most-bought plugin type; ours is missing entirely; big but high value |
| 10 | Spectral Glow | Merge into Soft Physical Glow (per-channel radius) | 3 / 1 / 3 | Almost free once item 1 exists; Deep Glow 2's Multiply Red / Green / Blue is the same control (screenshots) |
| 11 | Energy Stroke | Separate | 4 / 3 / 3 | Very popular free plugin; needs the path-drawing piece (section 5), which Stroke also needs |
| 12 | Shaped Bokeh with blur map | Merge into `core.lens_blur` | 4 / 3 / 3 | Fills the known gap in that row; P0-3 is enough for a map taken from a layer |
| 13 | Halation | Separate | 3 / 2 / 3 | Popular film-look ingredient; small on the shared engine |
| 14 | Lens Dirt | Merge into Soft Physical Glow and Lens Flare | 3 / 2 / 3 | Named by both Deep Glow 2 and the flare plugin; a texture read via P0-3. Deep Glow 2's control set is now confirmed (2.3) |
| 15 | Glint Streaks | Merge into `core.cross_glare` | 3 / 2 / 3 | Colour per ray and shimmer; Cross Glare and Kira-kira already do the rest |
| 16 | Zoom Glow and Edge Glow | Skip as effects; presets only | 2 / 1 / 2 | Radial Blur + Glow and Outline + Glow already make them |
| 17 | Heat Shimmer | Merge: drift direction and speed on `core.turbulent_displace` | 2 / 2 / 2 | Useful, rare; turbulence already does most of it |
| 18 | Deband | Separate | 2 / 2 / 2 | Anime-specific gap (CelFX); helps soft gradients; controls not confirmed |
| 19 | Map Chromatic Displacement | Merge into `core.displacement_map` | 2 / 3 / 2 | Recommended for water in anime, but niche |
| 20 | Particle Emitter | Park under L-01 | 5 / 5 / 5 | On every list including anime studios; the charter leaves it out (D-309), so only if the owner asks by name |

Also parked and not ranked: after-the-fact motion blur (ReelSmart-style, needs P0-13) and Particle Grid (Form).

**No change to the order after the 2026-10-08 sources.** The screenshots confirm items 1, 6, 10 and 14 as real product features but change no score. Item 1's effort stays 3 only because Lens Iris and tone mapping are left to later versions (2.3).

## 5. Infrastructure to settle before the /loop starts

These were proposals for the P0 table in `EFFECTS.md`. The owner accepted them on 2026-10-08 (particles and after-the-fact motion blur last); they are now P0-21, P0-22 and P0-23 there, with the Camera Lens Blur dependency split and P0-6 / P0-13 parked as last.

1. **Add a new P0-21, "Shared soft-glow engine".**
   - **What it is.** One piece that takes the bright parts of an image and spreads them out the way real light does:
     - several blur sizes, each double the last, added together so the glow fades gently (a stand-in for the inverse-square falloff);
     - a soft threshold;
     - worked in linear light (needs P0-9, done);
     - optional per-channel radius and a roll-off for very bright values.
   - **Stages confirmed by Deep Glow 2's panel (2.3)**, in order:
     - input: threshold, threshold smoothing, saturation bias, mask from a layer;
     - spread: radius, aspect ratio and angle, per-channel radius;
     - colour: inner and outer tint;
     - output: exposure, optional tone mapping, blend mode, source opacity, unmult.
     - A written rule for how far the output grows past the layer's edges, tied to the radius, so a glow is never cut off.
     - Later, not in the first version: an image-shaped kernel (Deep Glow's Lens Iris), reusing Camera Lens Blur's shaped-kernel work.
     - No half-precision switch that changes pixels.
   - **Why now.** Today Glow and Bloom each have their own blur code, and Bloom has its own four scales. Without a shared piece, items 1, 6, 9, 10, 13 and 14 would each re-invent this and drift apart. Lightning's and Beam's glows, Light Wrap and Diffusion could move onto it later.
   - **How to check it.** It needs its own independent `tools/*_reference.py`, like every effect, and GPU and CPU must agree within 1 level in 255 (ADR-006).
   - **Main risk.** Existing Glow and Bloom fixtures must stay unchanged. The new falloff must be a new mode, never a change to the current ones.
2. **Add a new P0-22, "Effects draw along paths".**
   - **What it is.** Let an effect take a mask, a shape path or text outlines and draw along it, with start and end trimming.
   - **What already exists:**
     - the drawing (`src/shape.rs` strokes with joins, caps and trim, D-78, D-169);
     - the distance to a path (`src/mask.rs` `distance_to_path`);
     - masks (P0-8, done).
   - **What is missing:** the hook that lets an *effect* use them.
   - **What it unlocks:** Stroke, Vegas, Scribble, Write-on, Fill and Audio Waveform (all missing rows), and Energy Stroke.
3. **Re-word P0-5's dependency note for Camera Lens Blur.**
   - The row says its blur map needs P0-5 (depth channels). A blur map *taken from another layer* only needs P0-3, which is done.
   - Split the two: the layer-map version can be built now; only true depth passes from EXR need P0-5.
   - This moves item 12 off the blocked list.
4. **Optional: one written rule for "held random seeds".**
   - Camera Shake already holds its random value for N frames. Line Boil (item 4) and Heat Shimmer would do the same.
   - Writing the rule down once, so effects agree on which frame a held value changes, avoids three slightly different answers.
   - Not a big piece of code; a note in document 20 would do.
5. **Leave P0-6 (particles) and P0-13 (motion estimation) parked.**
   - Add one line to each, saying they are respectively the first and third most-requested plugin categories in this research, so the owner can see the demand when deciding.
   - No change of priority without the owner asking by name (D-179, D-309).

## 6. Sources

Pages marked **(403)** could not be fetched; only search-result snippets were used for them. Reddit was not fetched directly, so nothing here rests on Reddit.

**Red Giant / Maxon help pages (per-effect behaviour).**

- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-blur.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-bokeh.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-chromatic-glow.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-compound-blur.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-edge-glow.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-glimmer.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-glo-fi.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-glo-fi-ii.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-glow.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-point-zoom.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-quantum.html
- https://help.maxon.net/rg/en-us/Content/html/Blurs-and-Glows-spot-blur.html
- https://help.maxon.net/rg/en-us/Content/html/Distortions_chromatic-aberration.html
- https://help.maxon.net/rg/en-us/Content/html/Distortions_heatwave.html
- https://help.maxon.net/rg/en-us/Content/html/Distortions_prism-displacement.html
- https://help.maxon.net/rg/en-us/Content/html/Distortions_rgb-separation.html
- https://help.maxon.net/rg/en-us/Content/html/Cat_ChromaticDispl.html
- https://help.maxon.net/rg/en-us/Content/html/Cat_OpticalGlow.html
- https://help.maxon.net/rg/en-us/Content/html/plugin-overview.html
- https://help.maxon.net/rg/en-us/Content/html/Particles-and-3D/Trapcode%20Particular/overview-of-particular-interface.html
- https://help.maxon.net/rg/en-us/Content/html/Particles-and-3D/Trapcode%20Particular/emitter.html

**Maxon product pages.** Fetched, but they did not list the pack contents:

- https://www.maxon.net/en/product-detail/red-giant/universe/blurs-and-glows
- https://www.maxon.net/en/product-detail/red-giant/universe/distortions
- https://www.maxon.net/en/product-detail/red-giant/universe/glow
- https://www.maxon.net/en/red-giant/universe
- https://maxon.net/en/product-detail/red-giant/trapcode/form

**Deep Glow.**

- https://aescripts.com/deep-glow/ (403 again on 2026-10-08; owner-supplied primary source, its screenshots below were read)
- https://aescripts.com/media/catalog/product/u/i/ui_compact_exponential_1.jpg (panel screenshot, Exponential mode; owner-supplied)
- https://aescripts.com/media/catalog/product/u/i/ui_compact_iris_1.jpg (panel screenshot, Lens Iris mode with the Iris group; owner-supplied)
- https://aescripts.com/media/catalog/product/u/i/ui_expanded_4_1.jpg (panel screenshot, every group expanded; owner-supplied)
- https://www.youtube.com/watch?v=3XdA_cZAZao ("Deep Glow 2 for After Effects Available Now!", aescripts; title and description with the what's-new list)
- https://www.youtube.com/watch?v=_vd8g7PaS_U ("Deep Glow 2: Quickstart Guide & What's New?", Plugin Everything; title, description, chapters)
- https://www.youtube.com/watch?v=bTG5r89UakA ("Deep Glow 2: In Depth Tutorial - Iris Mode", Plugin Everything; title, description, chapters)
- https://www.youtube.com/watch?v=l0MKEtsdZpg ("Deep Glow 2: In Depth Guide to Tone Mapping", Plugin Everything; title, description, chapters)
- https://www.youtube.com/watch?v=mvl5lHPYtL8 ("Made with Deep Glow 2 - Peter Clark"; showcase, title and description only)
- https://www.youtube.com/watch?v=tCHgTuewZR4 ("Made with Deep Glow 2 - Ravie & Co"; showcase, title and description only)
- No YouTube transcript was reachable for any video (empty captions); only titles, descriptions and chapter lists were read.
- https://aescripts.com/learn/deep-glow-review-physically-accurate-glows-inside-after-effects (403)
- https://www.toolfarm.com/tutorial/plugin-everything-deep-glow-create-bettter-glow-effects/ (403)
- https://edit-films.com/deep-glow-review/

**Video Copilot.**

- https://www.videocopilot.net/tutorials/saber_plug-in/
- Video Copilot's Saber product page feature list, as pasted by the owner on 2026-10-08 (uses and feature list; no control names)
- https://youtu.be/reSXGxkyr0k ("New Plug-in: SABER + Tutorial! 100% Free", Video Copilot; title and description only, no transcript reachable)
- https://motionarray.com/learn/post-production/video-copilots-free-saber-plug-in-review/ (403; snippet only)
- https://www.webdew.com/blog/how-to-use-saber-in-after-effects
- https://www.provideocoalition.com/saber-new-free-effects-plug-video-copilot/ (snippet only)
- https://www.videocopilot.net/blog/2014/05/new-plug-in-color-vibrance/
- https://www.provideocoalition.com/honest-review-video-copilots-optical-flares/

**Western roundups and technique.**

- https://www.schoolofmotion.com/blog/best-after-effects-plugins-and-effect-packs-you-need-in-2026
- https://schoolofmotion.com/blog/chromatic-aberration-nuke-after-effects
- https://lesterbanks.com/2021/01/a-super-easy-way-to-create-light-wrap-in-after-effects/
- https://www.premiumbeat.com/blog/wiggle-text-line-boil/ (snippet only)
- https://community.adobe.com/t5/after-effects-discussions/posterize-time-on-a-turbulent-displace-effect/td-p/8714041 (snippet only)
- https://helpx.adobe.com/my_en/camera-raw/using/vignette-grain-effects-camera-raw.html (snippet only)
- https://www.vsco.co/learn/halation-vs-bloom-vs-lens-flare (snippet only)
- https://darkroom.co/help/edit/bloom-halation.md (snippet only)
- https://miracamp.com/learn/premiere-pro/film-halation-effect (snippet only)

**Japanese anime compositing (撮影).**

- https://note.com/tomoex/n/n325babc1bcba (diffusion: Lighten plus Soft Light; Deep Glow)
- https://note.com/taka2composite/n/n040c7d42053d (blend modes per satsuei process)
- https://note.com/taka2composite/n/n6f6dcdab3f52 (five plugins in use)
- https://note.com/taka2composite/n/n8beb322132f8 (aerial perspective)
- https://note.com/ishimaru_home/n/nfa0302a85bfa (transmitted-light glow stack)
- https://flashbackj.com/afuta_ya-plugins-top-5 (plugin top five)
- https://qiita.com/median_ky/items/5c214e5ca3bd54c118e4 (backlight blending)
- https://qiita.com/median_ky/items/e060aa46d67dba07fe6d (chromatic aberration, Chromatic Displacement for water)
- https://howto.clip-studio.com/library/page/view/retasstudio_cr_sfx_00_001 (RETAS diffusion and transmitted light)
- https://animestyle.jp/2016/03/07/9837/ (satsuei lecture: T光, flares, fractal noise, diffusion)
- https://usapen3.hatenablog.com/entry/2016/12/18/234110 (satsuei terms list)
- https://hotakasugi-jp.com/2020/12/21/column-anime-composition/ (satsuei processes overview)
- https://terriblejunkshow.com/tutorial/defaultglow (taming the built-in Glow)
- https://blog.adobe.com/jp/publish/2024/08/23/cc-video-aftereffects-interview-kiminoiro (Kimi no Iro photography)
- https://cgworld.jp/feature/202104-shingekifs-cmp.html (Attack on Titan Final Season compositing)
- https://cgworld.jp/feature/201512-prisma-illya-cgw208t2.html (Prisma Illya compositing, Particular)
- https://cgworld.jp/flashnews/202412-OLMOpenTools.html (OLM tools)
- https://sites.google.com/site/annamillersclub/welcome/%E3%82%A2%E3%83%8B%E3%83%A1%E6%92%AE%E5%BD%B1/%E3%82%A2%E3%83%8B%E3%83%A1%E6%92%AE%E5%BD%B1%E3%81%AE%E7%82%BA%E3%81%AE%E3%82%BD%E3%83%95%E3%83%88%E3%82%A6%E3%82%A7%E3%82%A2 (satsuei software and plugins)
- https://www.psoft.co.jp/en/product/celfx/ (CelFX tool list)
- https://community.adobe.com/t5/after-effects-discussions/alternative-of-quot-olm-blur-quot-to-create-smooth-gradients-for-cel-animation/m-p/13294650 (OLM Blur for cel gradients)

**Unconfirmed or weakly sourced points, listed in one place.**

- Deep Glow's "Highlight Rolloff" and "Adaptation": one review only, and absent from the fully expanded Deep Glow 2 panel. Dropped from our spec; at most version 1 controls.
- Deep Glow: whether an inverse-square falloff mode still exists beside Exponential and Lens Iris; the options inside Tint Mode, Tone Mapping Operation, Quality Preset, Buffer Expansion Mode and View; whether it still has dithering or a downsample control; GPU use. None of these is visible in the screenshots.
- Deep Glow Lens Iris: what Iris Iterations and Radii Easing do exactly (read from their names only).
- Saber's individual controls for softness, flicker, glow spread and glow bias: still snippets only. Built-in distortion and advanced core settings are now confirmed as features (owner-pasted product list, launch video description), but not their control names.
- The full list of the Universe Distortions pack: only four effects were found.
- Shine and Starglow being Trapcode products is from memory.
- The CelFX Deband controls were not checked.
- No timing or performance claims are made anywhere in this note.
