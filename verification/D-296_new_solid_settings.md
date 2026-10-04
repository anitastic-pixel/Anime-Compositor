# D-296 / B-181: New solid asks for its name, size and colour first

Found by P-26, tutorials 4 and 5 (both lightsabers): After Effects' Layer > New > Solid (Ctrl+Y) opens Solid Settings, where the tutorial types the name, makes the solid 600 x 60 and picks white before it is made. Here the solid arrived at once, comp-sized and grey, and had to be fixed afterwards.

## What changed

- **New solid** (the button, the Layer menu, the command search and Ctrl+Y) opens a **Solid Settings** window: Name, Width, Height, **Make Comp Size**, Colour, Cancel (Esc) and OK (Enter).
- It opens with the next free name ("Solid 1", "Solid 2", ...), the composition's size and the colour last chosen in this session (grey the first time).
- OK makes the solid above the selected layer, as before; Cancel makes nothing.
- No file or engine change: it sends the same command as before with the chosen name, size and colour.

## Pictures from the window (test copy, 2026-10-04)

- `D-296 pictures/1_solid_settings.png`: Ctrl+Y in a new 1280x720 composition opened Solid Settings filled in with "Solid 1", 1280 x 720 and grey; the picture shows it after typing "Saber core", 600 x 60, white. Pass.
- `D-296 pictures/2_white_solid_made.png`: after Enter, the window closed and the timeline has one layer, "Saber core", a white 600 x 60 bar in the middle of the frame. Pass.

The same run also checked that opening it again offered "Solid 1" with white remembered, and that Cancel made nothing (still 1 layer).

## Checks (cargo test)

The app suite's wiring pins now hold the new command line and the three new controls (cancelsolid, makesolid, solidcompsize): 89 passed, 5 ignored as always, none failed.

## For the owner to try

1. Press Ctrl+Y. Solid Settings opens with the composition's size.
2. Type a name, make it 600 x 60, pick white, press Enter. A white bar appears with that name.
3. Press Ctrl+Y again and press Esc. Nothing is added.

Fixtures are unchanged.
