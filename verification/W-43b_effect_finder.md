# W-43b: the effect finder, Ctrl+Space (D-248)

The second half of the fifth screen of the redesign. The finder that Ctrl+Space opens is now laid out like the Sandbox's finder:

- **Along the top:** the search, then "Adds to" and the chosen layer's name. If no layer that takes effects is chosen, it reads "no layer" in red.
- **The left half has two parts.**
  - **Yours:** three buttons, Favourites, Recent and Presets. Each lists just those on the right.
  - **The families:** Blur & Sharpen, Color Correction, Light & Glow and so on, each with a coloured dot and a count. Clicking one opens it, and every effect in it is shown with a line on what it does. A gold star marks a favourite. Families left open stay open the next time.
- **The right half** has the matches, grouped as Recently used, Effects and Your presets. Each match has its family's dot and family name, and the typed letters are lit in blue. Favourites carry a "★ favourite" tag. The chosen line shows "Enter".
- **Along the bottom:** the chosen effect's name, family and description, then ↑ ↓ choose, the **Add to [layer] · Enter** button and **Esc**.
- **If nothing matches,** it says so, and offers to look for the same words among the commands (Ctrl+Shift+P).

It adds effects and presets with the same commands as before. No new command was added.

**One change in how it behaves:** on the right, **a click now chooses a line, and a double-click adds it.** Before, one click added it. This follows the Sandbox. Enter and the Add button still add at once.

## Pictures

| | Picture |
|---|---|
| The Sandbox finder beside the app's, both with "gl" typed and Light & Glow open | `W-43 pictures/finder_sandbox_vs_app.png` |
| The app's finder on its own | `W-43 pictures/finder.png` |
| The whole window with the finder open | `W-43 pictures/finder_whole_window.png` |

The pictures were taken with Glow and Drop Shadow set as favourites and Glow as recently used, for the picture only. The viewer shows white in pictures taken this way, as in earlier screens.

## What it does, checked in the running app (`w43b_check.js`)

The check used the reference shot's layer4. Favourites were set to Glow and Drop Shadow, and Recent was emptied. Nothing was saved, and the window's own favourites and recent list were put back afterwards.

| Step | What the page said |
|---|---|
| Opened with layer4 chosen | "Adds to layer4"; the search has the keyboard; left: Yours (Favourites, Recent, Presets), then 10 families; right: "Favourites": Glow (chosen, "★ favourite", "Enter"), Drop Shadow |
| The foot | "Glow · Light & Glow · Makes the bright parts shine softly into their surroundings." and "Add to layer4 · Enter" |
| Typed "gl" | "Effects": Glow first, then Bloom, Rim Light and the others whose names or family contain "gl"; "Your presets": Soft bloom, Impact; "Gl" lit in Glow and in Cross Glare |
| ↓ | Bloom chosen; the foot describes Bloom |
| Clicked the Light & Glow family | it opens: 10 effects, each with its line ("Glow / Makes the bright parts shine softly…") |
| Clicked Bloom there | the foot describes Bloom; nothing added |
| Double-clicked Bloom | the finder closes; layer4 now has Bloom |
| Opened the finder again | Light & Glow is still open |
| Favourites | Glow, Drop Shadow |
| Presets | the presets: Soft bloom, Night, Sunset, Cel shadow, Rim light, Old film, Impact, Dream haze… |
| Nothing typed and no button pressed | "Recently used": Bloom; "Favourites": Glow, Drop Shadow |
| Typed "zzqq" | "No effect or preset is called "zzqq"." and "Look for "zzqq" in the commands" |
| Clicked that | the finder closes; the command search opens with "zzqq" typed |
| Typed "gaussian", Enter | Blur (the app's name for Gaussian Blur) is added to layer4 |
| Escape / the Esc button / a click outside | each closes the finder |
| No layer chosen | "Adds to no layer"; the Add button is greyed out |
| Errors on the page | none |

## Limits, stated

- **The search also looks at each effect's family and its other search words**, as the Effects panel's search does. So "gl" finds Bloom (in Light & Glow) as well as Glow. Effects whose names start with the typed letters come first. The Sandbox matched names only.
- **No label colour on the "Adds to" chip.** Label colours are an engine proposal.
- **Presets are under the Presets button and in typed matches.** They are not a family in the tree.
- **The descriptions in the tree are cut to one line.** The whole line is in the foot when the effect is chosen.
- **The finder was pictured at 100% only.** At 200% on a small window it is narrower and shorter, and its lists scroll.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| Ctrl+Space, the Effects button in the top bar, or Effect › Find an effect or preset | nothing is sent | the finder, with the search ready for typing |
| Favourites / Recent / Presets | nothing is sent | only those on the right; the button turns blue; again to go back |
| a family | nothing is sent | it opens or closes |
| an effect in a family | nothing is sent | it is chosen; the foot describes it |
| double-click an effect (left or right) | adds it to every chosen layer that takes effects, as before | the finder closes; the effect is in Effect controls |
| a line on the right | nothing is sent | it is chosen |
| Add to … · Enter | the same add | the same |
| Esc, Escape, or outside the finder | nothing is sent | it closes |
| Look for "…" in the commands | nothing is sent | the command search, with the words typed |

## Keyboard reach

- **The search has the keyboard when the finder opens.** ↑ and ↓ choose on the right. Enter adds. Escape closes.
- **Tab reaches the Yours buttons, the families, the effects in an open family, Add and Esc.** Enter or Space presses them.
- **Enter on an effect in a family adds it.** When a family is opened from the keyboard, the focus stays on that family.
- These were checked by sending the keys to the page, not with a real keyboard.

## Checks

- **All 81 of the app's tests pass.** The wiring table and the key table are unchanged: no new button ID and no new key.
- **The export tests pass:** `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the reference shot and click a layer.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Press Ctrl+Space | the finder in the middle; "Adds to" and the layer's name at the top right | |
| 2 | Click "Light & Glow" on the left | it opens; each effect has a line on what it does | |
| 3 | Click Bloom there | the bottom line describes Bloom; nothing is added yet | |
| 4 | Double-click Bloom | the finder closes; Bloom is in Effect controls | |
| 5 | Press Ctrl+Space, type "gl" | Glow near the top with "Gl" in blue; each line has its family at the right | |
| 6 | Press ↓ twice, then Enter | the effect on the chosen line is added | |
| 7 | Press Ctrl+Space, click Recent | the effects you just added | |
| 8 | Click Presets | the presets list | |
| 9 | Type "zzqq" | "No effect or preset is called "zzqq"", and the offer to look in the commands | |
| 10 | Click nowhere in particular outside the finder | it closes | |

Anything marked ✗, tell me the row number.
