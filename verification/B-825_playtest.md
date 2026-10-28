# B-825 playtest: three shortcut fixes (D-945)

What changed: the key menu names Ctrl+Shift+F9 for Easy ease out, Ctrl+M adds the composition to the Render queue (as in After Effects) and Ctrl+Alt+M opens the export, and Ctrl+click on a key on the timeline switches it between linear and auto bezier.

## Steps

1. Open any project with a layer that has three or more Position keys. Show them with **P**.
2. Right-click a key. The menu says **Easy ease out  Ctrl+Shift+F9**.
3. Click a key, press **Ctrl+Shift+F9**. The key's right half turns into an hourglass waist (eased out); its left half stays as it was.
4. **Ctrl+click** a plain linear key (a diamond). It becomes a round auto bezier key. **Ctrl+click** it again: it is a plain diamond again. Each Ctrl+click is one **Ctrl+Z**.
5. **Ctrl+click** an eased key (hourglass): it turns into a plain diamond.
6. **Shift+click** two keys: both light up (Shift still adds keys). Ctrl+click no longer adds keys.
7. Press **Ctrl+M**: the Render panel's queue gains a row for the open composition; no folder is asked for.
8. Press **Ctrl+Alt+M**: the export asks for a folder, as Ctrl+M used to.
9. Open the menu with the Export line: it shows **Ctrl+Alt+M**; Add to Render queue shows **Ctrl+M**.

## What you should not see

- Ctrl+Shift+F9 easing the side the key is reached from (that is Shift+F9).
- Ctrl+M opening the folder picker.

The checks the computer made are in `verification/B-825_shortcut_fixes_table.md`; the check in a private copy of the app is `verification/B-825_private_app_table.md`.
