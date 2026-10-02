# W-42: Effect controls B (D-248)

The fourth screen of the redesign. The Effect controls panel now looks and works like the Sandbox's "Effect controls B":

- **One tree of rows.** Grey bars for Layer, Transform, "Blend, masks and parent", Camera, Effects, and one per effect. Each bar has a twirl (▾) that folds it.
- **A stopwatch on every row that can be keyed.** Blue means the setting has keys. Clicking a lit stopwatch takes all its keys off in one step, which one Undo puts back.
- **◀ ◆ ▶ beside the number.** ◀ and ▶ jump to the previous or next key, and ◆ adds or removes a key at the playhead. They show when the pointer is over the row, when the row is chosen, or when there is a key on this frame.
- **A blue switch on each effect** in place of the old dot. When it is off, the effect's bar reads "switched off".
- **A slider under the chosen row.** Clicking a setting's name chooses the row. If it is an effect's one-number setting with a range, a slider appears under it.
- **The tab** reads "Effect controls", then the layer's name, then the frame number at the right.

All of this sends the same commands the old panel sent. No new command was added.

## Pictures

| | Picture |
|---|---|
| The Sandbox board it copies (Effect controls B, marked 5) | `W-38 pictures/sandbox_compose.png` |
| The panel before | taken in the session, not kept |
| The panel after, a layer with Glow and Blur, opacity keyed | `W-42 pictures/effect_controls.png` |
| The whole window | `W-42 pictures/whole_window.png` |
| The real window at 100% / 150% / 200% | `W-42 pictures/scale_1.0.png`, `scale_1.5.png`, `scale_2.0.png` |

The scale pictures were taken with no layer selected. They show the Layer, Camera and Effects bars, the camera's stopwatches and blue numbers, and "frame 0" at the right of the tab.

## What it does, checked in the running app (`w42_check.js`)

The check used the reference shot's last layer (layer4). It added Glow and Gaussian Blur. None of it was saved.

| Step | What the page said |
|---|---|
| The tab | "Effect controls · layer4 · frame N" |
| The grey bars | Layer, Transform, Blend, masks and parent, Camera, Effects, Glow, Blur |
| The effect switches | "Switch off Glow", "Switch off Blur" |
| Opacity's stopwatch clicked at frame 0, then its ◆ at frame 12 | keys at 0 and 12; the stopwatch is lit, titled "Stopwatch on: opacity has 2 keys. Click to take them all off" |
| At frame 6 | ◀ and ▶ both lit on the opacity row; rows without a key here keep their arrows hidden |
| ◀ from frame 6, then ▶ | to frame 0, then to frame 12 |
| Clicked "Radius" under Glow | the row is chosen (blue bar at its left); one slider appears, 0 to 500; it is the only slider on the panel |
| Moved that slider | Glow's radius 10 → 40, and the number shows 40 |
| Blur's switch | it turns grey, now "Switch on Blur"; the bar reads "switched off"; the effect is off in the project |
| Folded Transform, then unfolded | Anchor hidden, then back |
| Folded Camera, then unfolded | the camera rows hidden and the twirl reads "Show the camera's settings"; then back |
| Rotation keyed twice, then its stopwatch clicked | 2 keys → 0 keys; the stopwatch unlit |
| Errors on the page | none |

## Limits, stated

- **The slider is only for an effect's one-number settings that have a range** (for example Glow's Radius, 0 to 500). Transform rows such as Opacity and Rotation have no slider yet; their blue number is dragged or typed as before.
- **Folding lasts until the window closes.** Fold state is not saved with the workspace.
- **Not built from the Sandbox board:** the "Reset · About…" links on each effect's bar, the "JUST ADDED" badge, and a Reset on the Transform bar.
- **The stopwatch takes off every key at once.** As in After Effects, there is no question first. One Undo puts them all back.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| a twirl (▾) on a grey bar | nothing is sent | the rows under it fold or come back |
| a stopwatch with no keys | adds a key at the playhead (`/keyframe.add_remove`), as the old ◇ did | the stopwatch turns blue |
| a lit stopwatch | removes all that setting's keys in one command (`/keyframe.add_remove` with every key) | the stopwatch goes grey |
| Alt+click a stopwatch | adds an expression, as Alt+click on the old ◇ did | the expression box |
| ◀ / ▶ | nothing is sent | the playhead jumps to the previous or next key |
| ◆ | adds or removes the key at the playhead | the diamond fills or empties |
| a setting's name | nothing is sent | the row is chosen; an effect's ranged number gets a slider |
| the slider | the same command a drag on the blue number sends | the number and the picture follow |
| an effect's blue switch | `/effect.toggle_bypass`, as the old dot did | the switch goes grey and the bar reads "switched off" |

## Keyboard reach

- **The twirls, stopwatches, ◀ ◆ ▶ and switches are buttons Tab stops at.** Enter or Space presses them. The arrows also show while one of them has the focus.
- **The slider takes the arrow keys** once Tab reaches it.
- **Ctrl+Space still adds an effect**, as the hint at the foot of the panel says.

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png` show a 1280 by 800 window at 100%, 150% and 200%. The bars, stopwatches and blue numbers fit the panel at all three.

## Checks

- **All 81 of the app's tests pass.** The wiring table and the key table are unchanged: no new button ID and no new key.
- **The export tests pass:** `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the reference shot and click a layer.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Look at Effect controls | grey bars: Layer, Transform, Blend, masks and parent, Camera, Effects; the layer's name and the frame number in the tab | |
| 2 | Click the twirl on Transform | its rows fold away; click again and they come back | |
| 3 | Click Opacity's stopwatch | it turns blue; a diamond appears in the timeline at the playhead | |
| 4 | Move the playhead on, change opacity's number | a second key appears | |
| 5 | Put the playhead between the two keys; hover the Opacity row | ◀ ◆ ▶ show; ◀ and ▶ jump to the keys | |
| 6 | Click the lit stopwatch | both keys are gone; Ctrl+Z brings them back | |
| 7 | Add Glow (Ctrl+Space), click the word "Radius" | the row turns blue at the left; a slider appears under it | |
| 8 | Drag that slider | the number and the glow follow | |
| 9 | Click Glow's blue switch | it goes grey; the bar reads "switched off"; the glow is gone from the picture | |
| 10 | Click the twirl on the Camera bar | the camera's rows fold away | |

Anything marked ✗, tell me the row number.
