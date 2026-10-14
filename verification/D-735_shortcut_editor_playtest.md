# D-735 playtest: the Keyboard shortcuts window (W-37)

What it is: one window that lists every shortcut, lets you change any of them, warns before one key is taken from another command, puts them back, and prints the list. Changes stay on this computer; they are not saved in the project.

Pictures of what to expect are in `verification/D-735 pictures/` (taken in a browser with no project open, so the window behind looks empty).

## Steps

1. Open any project. At the right end of the bottom strip, where the long line of key hints used to be, there is a **Keyboard shortcuts** button. Click it.
   - A window opens with the shortcuts in areas: Project, Edit, New layers, Layers, Time, Keys, Properties, Timeline, Viewer, Tools, Finding, Sheet. The footer says "All at their defaults." (picture 1)
2. Type **work area** in the search box.
   - Only the five work area lines are left. (picture 2)
3. Click **A key**, then press **Ctrl+D**.
   - Only "Duplicate layers" is left. Press **Esc**: back to searching by words, the window stays open. (picture 3)
4. Search **playhead**, and click **Change…** on "Split layers at the playhead". Press **Ctrl+D**.
   - It says: "Ctrl+D already does Duplicate layers (Layers). If Split layers at the playhead takes it, Duplicate layers (Layers) will have no shortcut until you give it one." with **Replace**, **Cancel** and **Reset to its default, Ctrl+Shift+D**. (picture 4)
5. Click **Replace**.
   - Split now shows Ctrl+D and "was Ctrl+Shift+D"; Duplicate layers shows no key. Both say "changed". The footer says "2 changed from the defaults. Kept on this computer, not in the project." (picture 5)
6. Click **Change…** on "Undo" and press **Alt+Left arrow**.
   - It refuses: "Alt+Left already does Move the chosen keys a frame (Keys). That key is not one this window changes; press other keys." Press **Esc**: nothing changed.
7. Click **Close**. Select a layer, put the playhead in the middle of it, and press **Ctrl+D**.
   - The layer is split at the playhead (not duplicated). **Ctrl+Shift+D** now does nothing.
   - The Layer menu shows Ctrl+D beside the split line. Ctrl+Shift+P, type "split": the line shows Ctrl+D too.
8. Close the app and open it again. Ctrl+D still splits.
9. Open the window again (Help > Keyboard shortcuts this time) and click **Reset all**.
   - Every line is back to its default; the footer says "All at their defaults." Close it: Ctrl+D duplicates again.
10. Open the window and click **Print or save as PDF…**.
    - The print dialog shows the whole list in two columns under the twelve area headings. Choose "Save as PDF" to keep it. (`6_printed_list.pdf`)

## Pass if

A changed key does the new thing everywhere (key press, menu, command finder), the clash warning names what loses its key before anything happens, Reset all brings back every default, and the printed list is readable.

## Known limits

- The arrows, A S R T M, the Alt+Shift keys that set a key, the Sheet's keys and the Sketch board's keys cannot be changed here; the window says so when you try one.
- Ctrl+C and Ctrl+V each belong to several lines (copy keys or layers, paste keys, layers or effects, whichever is selected). Giving one of those lines new keys leaves Ctrl+C or Ctrl+V with the others.
- While you type in a text box or the Sheet, a changed key only acts if it uses Ctrl.
