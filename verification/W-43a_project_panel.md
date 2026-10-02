# W-43a: the Project panel (D-248)

The first half of the fifth screen of the redesign. The Project panel now looks and works like the Sandbox's Project panel:

- **Four ways to list the project**, from buttons in the panel's tab: **List**, **Pictures**, **Tree** and **By cut**. The choice is remembered.
- **An info line at the top** about the chosen item. For a composition it gives the size, frame rate, number of layers, length, and where it is used. For drawings it gives the kind, how many files, how many layers use them, and the first and last file name.
- **List** has columns: Name, Kind, Length, Drawings. A composition's length is in seconds and frames, such as "10s 0f". For drawings, the Drawings column counts the files.
- **Missing drawings are red.** The name reads, for example, "layer3 · 1 missing". A red line under the list names the missing drawing and has its own **Relink…** button.
- **A row's buttons** (add as a layer, duplicate, delete, "can be passed on") sit over the right end of the row. They show while the pointer is over the row, or while the row is chosen.
- **Pictures** shows the compositions and drawings as cards with their counts, length and a red "missing" badge.
- **Tree** folds the project into Compositions, Precomps, Drawings, Sound and Other.
- **By cut** groups compositions under "Episode › Scene" chips. It uses the cut details written in Composition settings (Ctrl+K), under Sheet.
- The Compose workspace gives the Project panel more room: 300 pixels wide, as on the Sandbox board.

Everything sends the same commands as before. No new command was added.

## A fix found on the way

**Applying Composition settings (Ctrl+K) made an empty copy of the composition.** Since B-124b, pressing Apply without changing the shutter also created a new, empty composition with the same name. Writing the episode, scene and cut for By cut showed it.

Now Apply only changes the composition that is open. In the check, the project has 2 compositions after Apply, where before the fix it had 3.

## Pictures

| | Picture |
|---|---|
| The Sandbox board it copies (the Project panel, top left) | `W-38 pictures/sandbox_compose.png` |
| The four views side by side, with layer3 chosen | `W-43 pictures/project_four_views.png` |
| Each view on its own | `W-43 pictures/project_list.png`, `project_pics.png`, `project_tree.png`, `project_cut.png` |
| The whole window | `W-43 pictures/whole_window.png` |
| The real window at 100% / 150% / 200% | `W-43 pictures/scale_1.0.png`, `scale_1.5.png`, `scale_2.0.png` |

The four views were taken from the reference shot with these test edits, none of them saved:

- layer4 was turned into "Precomp 1";
- Episode 03, Scene 12, Cut 012 and animator A. Tanaka were written in Composition settings;
- drawing 7 of layer3 is missing in the reference shot as shipped.

## What it does, checked in the running app (`w43_check.js`)

| Step | What the page said |
|---|---|
| List's header | Name, Kind, Length, Drawings |
| List's rows | reference shot · Composition · 10s 0f · — / layer1 · One drawing · 1 / layer2 · Drawings · 24 / **layer3 · 1 missing** · Drawings · 11 / layer4 · Drawings · 20 |
| The info line, composition on screen | "reference shot · Composition · 1920 × 1080 · 24 fps · 4 layers · 10s 0f · top level" |
| The red line | "layer3: drawing 7 is missing", with Relink… that sends the usual relink for that asset |
| Clicked layer3 | "layer3 · Drawings · 11 files · used by 1 layer · drawing 7 is missing" |
| Clicked layer2 | "layer2 · Drawings · 24 files · used by 1 layer · layer2_000.png to layer2_023.png" |
| Pictures | "Compositions · 2": reference shot (marked as on screen), Precomp 1. "Drawings · 4": four cards, layer3's with "1 missing" |
| Clicked the layer2 card | layer2 chosen; the info line follows |
| Clicked the Precomp 1 card | Precomp 1 opens in the viewer |
| Tree | Compositions (1 item), Precomps (1 item, used once), Drawings (4 items), with "layer3 · 1 missing" in red |
| Folded Drawings, then unfolded | 10 rows → 6 → 10 |
| By cut | "Episode 03 › Scene 12", then "Cut 012 · 10s 0f · 1 missing · reference shot · Ep 03 · Sc 12 · Cut 012 · A. Tanaka", marked as on screen. Then "No episode or scene written" with Precomp 1. Then a hint pointing to Ctrl+K |
| The view choice | saved with the window's own settings (`projView`), which are read again at start |
| Compositions after Apply in Ctrl+K | 2 (the fix above) |
| Errors on the page | none |

## Limits, stated

- **The tiles are symbols, not pictures:** ▦ for a composition, ✎ for drawings, ♪ for sound. The window has no way yet to hand the page a small picture of a drawing or composition. Real thumbnails are an engine proposal, brought with the others.
- **No coloured label dots.** Label colours are an engine proposal.
- **No stage pills in By cut** (such as "Layout", "Key", "Colour"). Cut status is an engine proposal.
- **Drawings have no length.** A drawing asset has no frame rate of its own, so its Length shows —.
- **Dragging an item into the timeline works from List only.** In Pictures, Tree and By cut, a click chooses drawings or opens a composition.
- **Tree's folders are fixed groups by kind.** The project file has no folders of its own to show.
- **Narrow panels drop columns.** Below 330 pixels Length is hidden; below 240 pixels Drawings is hidden too.

## Click-through

| Clicked | What runs | What you see |
|---|---|---|
| List / Pictures / Tree / By cut | nothing is sent | the panel switches view; the pressed button turns blue |
| a composition (any view) | opens it, as before | it fills the viewer; the info line describes it |
| drawings (any view) | nothing is sent | chosen; the info line describes them |
| Relink… on the red line | the same relink the old Relink… sends, for that asset | the file picker |
| a Tree folder | nothing is sent | it folds or opens |
| a By cut card | opens that composition | as clicking it in List |
| the tick on a drawings row | the same "can be passed on" command as before | the tick changes |

## Keyboard reach

- **The four view buttons, the cards, the Tree's rows and folders, the By cut cards and Relink… are buttons that Tab stops at.** Enter or Space presses them.
- **After a press the focus is put back on the same item** when the panel redraws. This is built in but was not checked with a real keyboard.
- **The row buttons in List show while the row has the focus**, so Tab can reach them.

## Sizes

`scale_1.0.png`, `scale_1.5.png` and `scale_2.0.png` show a 1280 by 800 window at 100%, 150% and 200%. The panel's tab, the view buttons and the info line fit at all three.

## Checks

- **All 81 of the app's tests pass.** The wiring table and the key table are unchanged: no new button ID and no new key.
- **The export tests pass:** `t08_export`, `h04_exported_file`, `t07e_roundtrip_export` and `d48_export_threads`. The exported pictures are unchanged.

## Playtest

Open the reference shot. The Project panel is at the top left.

| # | Do this | You should see | ✓ / ✗ |
|---|---|---|---|
| 1 | Look at the Project panel | four buttons in its tab, List pressed; columns Name, Kind, Length, Drawings | |
| 2 | Look at layer3 | red: "layer3 · 1 missing"; a red line under the list: "layer3: drawing 7 is missing" with Relink… | |
| 3 | Click layer2 | the top line reads "Drawings · 24 files · used by 1 layer" and the first and last file names | |
| 4 | Click Pictures | cards for the composition and the four drawings; layer3's has a red "1 missing" | |
| 5 | Click Tree, then the ▾ by Drawings | the four drawings fold away; click again and they come back | |
| 6 | Press Ctrl+K, open Sheet, write episode 03, scene 12, cut 012; Apply | the Project panel still shows one "reference shot", not two | |
| 7 | Click By cut | "Episode 03 › Scene 12", with a "Cut 012" card for the reference shot | |
| 8 | Close and reopen the window | the panel opens on the view you left it on | |
| 9 | Hover a drawings row in List | its tick shows at the right end | |

Anything marked ✗, tell me the row number.
