# Diagnosis: silent_settings (bad settings that seem to vanish, Advanced Lightning's obstacle)

Read-only diagnosis on main at 90a33e9a. Nothing was built or run; every finding below comes from reading the code and documents.

## Plain summary

**Bug 1, "an effect vanishes with no message".** The app does give a message. The checking tool looked in the wrong place. When a project opens, the engine checks every effect's settings. If one is bad (a number out of range, or a word the effect does not know), it writes a warning such as *"The layer "layer1" has a core.lens_blur whose settings this build cannot use. Camera Lens Blur's highlight threshold runs from 0 to 100, and this is 200. The effect is kept and bypassed."* (The effect's name in that sentence comes from `Effect::name`, so the exact wording may differ slightly.) The app shows that sentence in the warnings list in the bottom bar (`#notes`) and as a "⚠ 1" chip in the top bar. `tools/app_snapshot.py` copies two other boxes, `#report` and `#status`. `#report` is the playback report ("Played 24 frames…"), empty when a single frame is asked for. `#status` is the answer to the last command. Neither box ever holds a warning, so the tool recorded "nothing said". Fix: make the tool also copy the warnings list. It is a small change to the tool only, with no change to the engine, the app or any fixture.

Two of the reported cases are not this bug:
- **Caustics floor set to its own layer.** This is not an error. The project rule (D-189) says an effect that names its own layer reads that layer's own drawing, before effects, as its map. A plain purple solid as its own floor gives a flat purple floor, so there is nothing to warn about. (Likely; see below.)
- **Lane 3's Tint, Exposure, Levels and CC Toner** are the same tool blind spot as the rest of bug 1. Their warnings are in the warnings list, not in `#report` or `#status`.

**Bug 2, "Advanced Lightning's default Obstacle of 50 hides the bolt".** The real default is 0, not 50. A new Advanced Lightning has Alpha Obstacle 0 and draws normally. The 50 came from copying a test fixture (FX-LIGHTX-002/004, FX-LIGHTA-001…), which sets 50 on purpose to test the obstacle. With 50 on a fully solid layer the bolt cannot start, because the start point is already inside "what blocks". That is exactly the rule this project wrote down (D-324, document 21), so the code is right. **The rule itself is probably not what After Effects does.** Adobe describes the obstacle as resistance "at alpha edges" that the bolt "attempts to wrap … around", and a fully solid layer has no edges. D-324 already records that the rule is this program's own and that no After Effects frame was compared. Whether to change it is a specification decision for the owner, not a bug fix. Options are below.

---

## Bug 1: settings refused with no message seen

### 1. Symptom

Camera Lens Blur with `highlight_threshold` 200 (lane 2), CC Light Wipe `shape` "round" (lane 2), Fractal `palette` "rainbow", Path Stroke `source` "layer", Glow `falloff` "gaussian" and Cross Glare's glint `ray_colors` "two" (lane 1), and refused fixture settings on Tint, Exposure, Levels and CC Toner (lane 3) all draw the picture as if the effect were off. Each snapshot JSON's `report` and `status_line` say nothing about it. Example: `verification/app_snapshots/camera_lens_blur/effect/frame_0012_cpu.json` has `"report": ""` and `"status_line": "The preview is drawn on the CPU."`.

### 2. Root cause (end to end)

The engine produces the message in three places, as document 28 asks. The app shows the one made when the project opens. The tool never reads it.

1. **Where the message is made (engine):**
   - The settings check is `Effect::is_valid` / `why_invalid` / `fault` (`src/effects.rs:9893`, `:9931`, `:10154`), and per instance `EffectInstance::is_valid` / `fault` / `invalid` (`src/effects.rs:115`, `:121`, `:229`). Keys are checked too. Every reported case has a sentence there: Lens Blur's range table (`src/effects.rs:6372`), Glow Wipe's shape (`:12152`), Fractal's palette (`:11759-11775`), Path Stroke's source (`:11133`), Glow's falloff (`:11092`), Cross Glare's ray colours (`:504`, called from `:12172`), Tint and Exposure (`:9896-9948`).
   - **On open:** `persist.rs:9430-9459` pushes a `EFFECT_PARAMETER_INVALID` WARNING with that sentence into the load warnings and keeps the record exactly as written.
   - **Per frame:** compose's bypass `report` closure records `EFFECT_PARAMETER_INVALID` in the `FrameLog` for each frame (`src/compose.rs:3466-3517`, raised at `:3625`; also `src/effects.rs:12849-12850`).
   - **Export:** `src/export.rs:767-812` marks fidelity incomplete when that ID is in the frame log.

2. **Where the app puts it:**
   - `open` turns the load warnings into `viewer.notes` (`app/src/main.rs:1843`). Every frame answer carries them as `x-notes` (`app/src/main.rs:1218`).
   - The page writes them into the `<ul id="notes">` list in the bottom bar, the codes into `#notecodes`, and the count into the ⚠ chip (`app/ui/index.html:2863-2879`, chip `:19055-19058`). **So the person is told when the project opens, and the warning stays on screen.**
   - The per-frame copy is deliberately not sent again: `x-frame-notes` passes only `TEXT_FONT_MISSING` lines (`app/src/main.rs:1667-1676`). The per-frame copy still reaches the session log (`:1642`) and the export report. Sending it again would only print the same sentence twice beside the load note.
   - `#report` is the playback report (`x-report`, `app/src/main.rs:1253-1259`, page `index.html:2810-2812`). It is empty for a frame asked for by number. `#status` is the last command's answer (`x-status`, `index.html:2822`). Neither box is meant for warnings.

3. **Where it is lost (the tool):** `tools/app_snapshot.py:190-206` (`cpu_js`) and `:209-239` (`card_js`) copy `#report`, `#status` and `x-frame-notes`, but never `#notes`, `#notecodes` or `x-notes`. That is the whole cause of "the report was empty".

4. **The project record:** preserved and not repaired, as document 28 requires (`persist.rs:9433-9436`). Nothing is lost there.

**What document 28 requires** (`Markdown/28_Error_Diagnostics_Catalog.md:29`): `EFFECT_PARAMETER_INVALID` is an ERROR on a command and a WARNING on load and per frame. The required behaviour is to reject the command; on load, preserve the record unrepaired; bypass only that effect while the rest of the stack runs; warn once per frame; and report fidelity incomplete. The engine does all of this: command `src/command.rs:3572`, load `persist.rs:9442`, per frame `compose.rs:3480`, export `export.rs:775`. Line 106 asks for background errors to "remain in a diagnostics/status panel". The `#notes` list is that panel.

**Not part of this bug: Caustics with its floor set to its own layer.** `setting_map` (`src/compose.rs:5538-5546`) gives a layer that names itself its own drawing and masks, with its effects not run (D-189 step 1). Caustics therefore uses its own layer, before Caustics, as the floor. If that layer is a flat purple solid, the floor is flat purple. That is the rule working, and there is nothing to warn about.

### 3. Confidence

- The tool not reading the warnings list: **confirmed by reading the code** (the tool's JS against `index.html` and `main.rs`).
- That each reported setting actually raised a load warning in those runs: **likely**. Every case has a `fault` sentence and passes through `persist.rs:9442`. The single check that confirms it: after the tool fix, re-run `lens_blur_iso_highlight` and see the sentence in the JSON's `notes`.
- Caustics: **likely** by design. Check: the floor in `verification/app_snapshots/caustics/…` should match the holder layer's own colour.

### 4. Fix plan

Tool only (`tools/app_snapshot.py`). In both `cpu_js` and `card_js`, add to `meta`:

```js
notes: [...document.querySelectorAll('#notes li')].map((li) => li.textContent),
note_codes: [...document.querySelectorAll('#notecodes li')].map((li) => li.textContent),
```

The CPU side fetches the frame directly and does not redraw the page, but `#notes` was filled by the page's own first frame when the project opened, so reading the list works for both sides. Optionally, print a `NOTE:` line per shot when `notes` is not empty, so a lane cannot miss one.

This fixes every case on the list at once (all lanes, every effect), because they all reach the person by the same `x-notes` → `#notes` route. No engine or app change is needed.

Not needed and not proposed: sending the per-frame `EFFECT_PARAMETER_INVALID` in `x-frame-notes` (it would repeat the load note), or a badge over the picture (document 28 says "may").

Known side gap, separate from this bug: `viewer.notes` is filled only on open (`main.rs:1843`). After someone corrects the setting in Effect Controls, the old note stays until the project is reopened. A stale warning is the safe direction, but it is worth a later note.

### 5. Proof plan

- Fixtures: none change. The engine behaviour is already pinned by the existing `EFFECT_PARAMETER_INVALID` fixtures (for example FX-LIGHTX-024, document 25 line 175063).
- Before (already on disk): `verification/app_snapshots/lens_blur_iso_highlight/effect/frame_0012_cpu.json` has no `notes`, and `report` is empty.
- After, same project:
  `python tools/app_snapshot.py --project verification/app_snapshots/lens_blur_iso_highlight/lens_blur_iso_highlight.json --frames 12 --out <scratchpad>/lens_iso_after`
  Expected: `notes` holds one sentence naming `core.lens_blur` and "highlight threshold runs from 0 to 100, and this is 200", and `note_codes` starts `EFFECT_PARAMETER_INVALID`. The picture is unchanged (still no blur).
- Control: `camera_lens_blur_fixed` gives `notes: []`.
- For the owner, in pictures: open `lens_blur_iso_highlight.json` in the app. The bottom bar shows the sentence and the top bar shows "⚠ 1". A screenshot of that window is the visual proof.

### 6. Size

Small (two lines in each of two JS strings, plus an optional print).

---

## Bug 2: Advanced Lightning with Alpha Obstacle 50 on a solid layer

### 1. Symptom

With `obstacle` 50 on an ordinary fully opaque layer, no bolt is drawn and no message appears. Lane 1 set `obstacle` 0 to get a picture (`LANE1_RESULTS.md:27`).

### 2. Root cause

- **It is not the default.** The file's absent value is 0 (`src/persist.rs:7060`), and the panel's starting value is 0 (`app/ui/index.html`, the `core.lightning_bolt` controls at about line 10845: `'obstacle', 'Alpha Obstacle', …, 0`). Document 21 says "`obstacle` … absent 0" (`Markdown/21_Rendering_Math_Specification.md:427`). The 50 appears only in obstacle test fixtures (`Fixtures/lightning_extras/fx_lightx_002/004/006`, `lightning_around/fx_lighta_001,003-007`, `lightning_forks/fx_lfork_004`). In each of those, the start point lies in a clear area and the obstacle is a sky or ground region (document 25, FX-LIGHTX-002/004).
- **Why nothing is drawn:** each pixel blocks when its covering is above `1 - obstacle/100` (`src/effects.rs:15324-15328`). At 50 every pixel of an opaque layer blocks, including the start point. `bolt_stop` (`src/layer_fx.rs:6956-6958`) cuts every segment at the first blocking point, so nothing is left. This is exactly D-324's rule (`Markdown/14_Decisions_Risks.md:994`: "where the layer … covers more than 1 - obstacle / 100, the bolt and its forks stop") and document 21's step (1) for D-329 (`21_…:429`).
- **Against After Effects:** D-329 quotes Adobe: the lightning "attempts to wrap itself around opaque areas", and Alpha Obstacle "causes resistance to increase at **alpha edges**" (`14_Decisions_Risks.md:1017`). That is a resistance field driven by edges. On a uniformly opaque layer there are no edges, so After Effects most likely still draws the bolt. Our rule blocks on solid pixels, not at edges, so it blocks the whole layer. D-324 and document 21 both state that the rule and the types are "this program's reading … no After Effects frame was compared".

**Verdict:** this is **not a code bug**. The code follows the accepted specification, and the default is 0. It is **probably a difference from After Effects** in an unusual setup (a positive obstacle on a layer with no clear parts). The difference is already recorded as "our own rule". Changing it is a specification decision for the owner.

### 3. Confidence

- The code follows the spec and the default is 0: **confirmed by reading**.
- After Effects drawing the bolt on a fully opaque layer at obstacle 50: **likely, not confirmed**. The only check is one After Effects frame: Advanced Lightning on an opaque solid, Alpha Obstacle 50, everything else default.

### 4. Fix plan (owner's choice; nothing to fix under the current spec)

- **(a) Keep as is.** Optionally, add to the Alpha Obstacle tooltip in `index.html`: "a bolt that starts inside what is solid is not drawn". Tiny.
- **(b) A bolt starting inside what blocks ignores blocking until it first reaches a clear pixel.** A change in `bolt_stop` (`src/layer_fx.rs`) and a sentence in document 21. Small to medium. It changes pixels only where the start point blocks. In FX-LIGHTX/LIGHTA/LFORK with a positive obstacle the start is clear (document 25), so those expected values should not change, but each negative-obstacle and `around` case would need checking.
- **(c) An edge-based resistance like Adobe's.** A new bolt engine, large. It needs an After Effects reference first.

Recommendation: (a) now, and (b) or (c) only if the owner wants it after seeing an After Effects frame.

### 5. Proof plan

- (a) No fixture change. Proof: lane 1's `advanced_lightning` snapshot at obstacle 0 already shows the bolt.
- (b) Write a new fixture first (owner-approved): an opaque layer, obstacle 50, start inside, with the bolt drawn to the end point. Before and after with `python tools/app_snapshot.py --project verification/app_snapshots/advanced_lightning/advanced_lightning.json --frames 12` after setting `obstacle` to 50 in a scratch copy: before, no bolt; after, a bolt. Rerun every FX-LIGHTX/LIGHTA/LFORK/LFULL case. **Any change to an existing expected value is a specification decision for the owner.**

### 6. Size

(a) small · (b) small to medium · (c) large.
