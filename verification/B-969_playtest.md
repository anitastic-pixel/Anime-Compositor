# B-969: Convert Audio to Keyframes (D-1089)

Built on 2026-11-03 as D-1089 under your request of 2026-10-27 to follow After Effects tutorials
("Now let's do some tutorial follow allongs to achieve the same effects..."). The tutorial is
k6xZUlhlZxs at 1:17 to 2:10: the music layer's Convert Audio to Keyframes makes a null "Audio
Amplitude" with three sliders, and the logo's Scale is tied to Both Channels so it bounces with
the music.

Right-click an audio layer and pick **Convert audio to keyframes** (it is also in the Animation
menu and the command finder). A null "Audio Amplitude" appears at the front, as long as the
composition, with three Slider Controls named Left Channel, Right Channel and Both Channels.
Each slider has a key on every frame of the work area: how loud that frame is, 0 for silence.
It is one step to undo. With no audio layer switched on, nothing is added and a sentence says
why. A sound that cannot be read is named in the result, as an export names it.

A null may now carry Expression Controls (Slider, Checkbox, Angle, Point, Color, Layer), as in
After Effects. Every other effect on a null is still refused.

The check, `verification/B-969_audio_keyframes_table.md` (all pass), compares every frame's
three numbers for six test projects against numbers worked out by a separate program
(`tools/audio_keyframes_reference.py`) before the code existed.

## What to check

1. **The sheet.** `verification/B-969 pictures/1_audio_to_keyframes.png`: at the top, the
   tutorial's frame. In the middle, a graph of the three sliders the command made on a test
   sound. The red line (left) should jump up on each beep and drop to 0 between beeps. The blue
   line (right) should stay low and flat. The black line should sit halfway between them. At the
   bottom are six frames of an orange square whose Scale follows Both Channels. It should be big
   on frames 0, 3 and 12 (beeps) and small on 6, 9 and 18.
2. **In the app.** Import a song (File > Import) and put it in a composition. Right-click the
   song's layer and pick **Convert audio to keyframes**. Check that "Audio Amplitude" appears at
   the top of the timeline. Select it: Effect controls should list Left Channel, Right Channel
   and Both Channels, and pressing U should show a key on every frame.
3. **Make something bounce.** Make a shape or solid. Alt+click the stopwatch of its Scale and
   type
   `s = linear(thisComp.layer("Audio Amplitude").effect("Both Channels")("Slider"), 0, 40, 60, 160); [s, s]`.
   Play: it should grow on the loud parts and shrink on the quiet ones.
4. **Undo.** Press Ctrl+Z once: "Audio Amplitude" should go away.
5. **Nothing to hear.** In a composition with no audio layer (or with the song switched off),
   pick the command from the Animation menu. A sentence should say there is nothing to hear,
   and nothing should be added.

Known differences: After Effects does not publish how it measures loudness, so our numbers rise
and fall in the same places but are not the same numbers. You cannot rename an effect by hand
yet; only this command names one. A saved project with these names opens in an older build, but
there `effect("Both Channels")` is not found. A null holding sliders is refused by an older
build.
