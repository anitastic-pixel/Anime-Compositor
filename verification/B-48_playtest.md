# B-48: Auto and Memory, by hand

Built on 2026-09-26 against the B-48 entry in document 15. **Passed on 2026-09-26 by the owner's word: "works".**

`verification/B-48_memory.md` explains what was built, with the checks and the timings. This sheet covers what those cannot show: how the window behaves.

## Before you start

- Use the release build. It opens on the reference shot.

## What to check

1. **It starts on Auto.** The choice in the viewer's bar, where "Draw on GPU" used to be, says **Draw on: Auto**. The label beside the frame number ends "drawn on GPU".
2. **It remembers.** Choose **Draw on: CPU**, close the app and open it again. It says CPU, and the label says "drawn on CPU". Choose **Auto** again.
3. **CPU and GPU still work.** Choose each. The label follows, and the picture does not change in any way you can see.
4. **The Memory settings.** Open **Preferences**. Under Memory:
   - **Automatic** is chosen.
   - RAM shows about 16.5 GB and the graphics card about 8.4 GB, both greyed out.
   - The line below says how much of each is in use now, and how much this computer and the card have.
5. **Play at Full.** Close Preferences. Add a Bloom to the background layer, switch to **Full resolution** and press space. Let it loop twice. The second loop should be smooth. Open Preferences again: RAM in use has grown, and stays under 16.5 GB.
6. **Custom.** Choose **Custom**. The two numbers can now be typed in, and are the Automatic ones.
   - Set RAM to 2 and click elsewhere. Play at Full again: it should be slower than in step 5, as the app used to be.
   - Set RAM to 1000. It goes to the most allowed, about 49.6 GB.
7. **Back to Automatic.** Choose **Automatic**. RAM shows about 16.5 GB again, greyed out. Close the app and open it: Preferences still says Automatic.
8. **Export is untouched.** An export is the same file whatever Draw on and Memory say.

## What to report

- Anything the choice or the label says that does not match what you chose.
- Any sentence saying the graphics card failed, with its exact words.
- Any number in Preferences that looks wrong for this computer.
- Windows warning that memory is low, or other programs slowing down while the app plays.
