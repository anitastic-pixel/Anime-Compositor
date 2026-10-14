# D-732 playtest: Increment and Save

What it is: After Effects' File > Increment and Save. It saves a copy of your project with the next version number in its name, and the window then works on that copy. The older file is left alone.

## Steps

1. Open a project that has been saved before, for example one called `shot.json`.
2. Press **Ctrl+Alt+Shift+S**.
   - You should see "Saved to ...\shot_v002.json" at the top of the window.
   - The project name in the top bar now reads `shot_v002.json`.
3. Change something small (move a layer) and press **Ctrl+Alt+Shift+S** again.
   - You should see "Saved to ...\shot_v003.json".
4. Open the folder in Explorer.
   - `shot.json`, `shot_v002.json` and `shot_v003.json` are all there. `shot_v002.json` does not have the move from step 3; `shot_v003.json` does.
5. Open `shot_v002.json` again (File > Open recent). Press **Ctrl+Alt+Shift+S**.
   - `shot_v003.json` is already there, so it is not written over: you should see "Saved to ...\shot_v004.json".
6. File menu: the line **Increment and save** is under Save As, showing Ctrl+Alt+Shift+S. Clicking it does the same as the keys.
7. Ctrl+Shift+P, type "increment": the same line is in the command list.
8. File > New project, then press **Ctrl+Alt+Shift+S**.
   - A new project has no name to count up from, so the usual "Save the project as" box opens.

## Pass if

Every "should see" above happens, and no existing file in the folder is changed.
