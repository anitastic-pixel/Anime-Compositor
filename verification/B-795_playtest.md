# B-795 playtest: colour management from File > Project Settings (D-916)

What changed: the File menu has a new line, **Project Settings...**. It opens a small box with one list, **Colour management**: Off, On working in ACEScg, or On working in linear Rec.709. Before this, colour management could only be turned on by editing the project file by hand.

About 5 minutes. You need two projects: any of your own projects, and one composition you can set to 8 bpc (AE).

## 1. The box shows what the project has

1. Open any project of yours that you have never turned colour management on for.
2. Choose **File > Project Settings...**.
3. **Expect:** the box opens with **Colour management: Off**. Close it.
4. Open the **View** menu. **Expect:** there is no **Colour view** line.

Picture: `B-795 pictures/1 File menu with Project Settings.jpg` and `2 Project Settings, colour management off.jpg`.

## 2. Turning it on

1. Open **File > Project Settings...** and choose **On, working in ACEScg**.
2. **Expect:** the line at the bottom of the window says "Turn colour management on, working in ACEScg". The viewer's picture may change: it is now shown through the ACES 2.0 SDR view, which rolls off bright colours instead of clipping them.
3. Close the box and open the **View** menu. **Expect:** a **Colour view** line, set to **ACES 2.0 SDR**.

Pictures: `3 Project Settings, on working in ACEScg.jpg` and `4 View menu now has Colour view.jpg`.

## 3. Undo puts it back

1. Press **Ctrl+Z**.
2. **Expect:** the bottom line says "Undone: Turn colour management on, working in ACEScg". The picture is as it was in step 1, the **View** menu has no **Colour view**, and **File > Project Settings...** shows **Off** again.

Picture: `5 after Undo, colour management off again.jpg`.

## 4. Refused while a composition is in an After Effects depth

1. Set a composition's depth to **8 bpc (AE)** in **Composition Settings**, with colour management Off.
2. Choose **File > Project Settings...** and pick **On, working in ACEScg**.
3. **Expect:** the bottom line says "Colour management stays off: composition "(its name)" works in 8 bpc (AE). ... Choose Display or Float in that composition's settings, then turn colour management on." The list goes back to **Off**. The composition is still at 8 bpc (AE): nothing was changed for you.
4. Press **Ctrl+Z**. **Expect:** it undoes your depth change from step 1, not anything from step 2 (the refusal added nothing to the history).

Picture: `6 refused, a composition in 8 bpc (AE).jpg` (the message is in the line at the very bottom).

## 5. Saving

1. Turn colour management on, save, close and reopen the project. **Expect:** **File > Project Settings...** still shows it on, with the same working space.
2. Turn it off again and save. **Expect:** the project is the same as before you ever turned it on.

## Known, not a fault

- **Exports are not colour managed yet** (step C6, under way separately). With ACEScg working, an exported PNG or MP4 looks more saturated than the viewer. The box says this.
- The Project panel's thumbnails, the colour pickers and Info still show the working numbers (step C7).
- No keyboard shortcut for Project Settings yet.

If anything above does not happen as written, tell me which step and what you saw.
