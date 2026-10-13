# B-518: reading sound (the first step of the sound plan)

Built on 2026-10-13 as the first step (S-1) of the sound plan you accepted (D-638), decided as
D-639. Until now the program itself could only read WAV files; every other sound was played by
the window but not understood by the program. Now the program reads:

- **WAV**, as before, exactly the same.
- **FLAC, MP3 and Ogg Vorbis**, through a small library (`symphonia`).
- **AAC**: `.m4a` and `.aac` files, and **the sound of `.mp4` and `.mov` videos**, through the
  decoder Windows already has.
- **Opus is not read.** It still plays in the window, but you get a warning that it will be
  left out of exports and sound effects.

Every sound is also turned into the program's working sound: 48,000 samples a second, two
channels. This is what the next steps (hearing it in step, export, the sound effects) will use.
Nothing you hear changes yet: the window still plays the sound as before.

The check, `verification/D-639_sound_read_table.md` (77 of 77 checks pass), compares every
test file with ffmpeg's reading of the same file. Each test sound has a click exactly on the
first sample of frames 2 and 12, and every file's clicks land on the right sample, so the sound
is in step with the picture. The compressed formats (MP3, Ogg, AAC) came out within a millionth
of ffmpeg's reading.

## What to check

**The pictures** in `verification/D-639 pictures/` each show one test file's sound wave: two
thin spikes (the clicks), then a band of steady tone.

1. **Every picture looks alike.** The spikes sit in the same two places, about a twentieth and
   a third of the way across, and the tone starts at the same place, in every file, whatever
   its format or rate.
2. **The one exception.** `stereo_48k_aac.png` (a raw `.aac` file) is very slightly later:
   that kind of file does not record the encoder's start delay, so every program reads it about
   a fiftieth of a second late. This is expected.
3. **In the app: import an MP3.** Import a sound and pick an `.mp3` (or `.flac`, `.ogg`,
   `.m4a`). It comes in with no warning, and plays as before.
4. **Import a video's sound.** Import a sound and pick an `.mp4` or `.mov` (they are now in the
   list). It comes in as a sound layer, and the message says only its sound is brought in, not
   the picture.
5. **Opus.** Import an `.opus` file: it comes in and plays, with a warning that the program
   does not read it, so exports and sound effects will leave it out.
6. **Audio Spectrum on an MP3.** Put an MP3 on a sound layer, add a solid with **Audio
   Spectrum** pointed at that layer, and play: the bars move with the music. Before this they
   only worked with WAV files. **Audio Waveform** works the same way.
7. **Older projects.** Open a project that already has sound in it: it plays, draws and saves
   exactly as before.

**Timing.** Reading a three-minute song takes a fraction of a second for WAV, FLAC and MP3,
and about two and a half seconds for an M4A (Windows' decoder is slower). It happens once, when
the sound is brought in or the project opens. PROVISIONAL, measured while other builds were
running (`verification/B-518_sound_read_timing_table.md`).

## Not built (known limits)

- **Opus** is not read (the plan's choice, so the program ships no Opus decoder).
- A raw **`.aac`** file reads about a fiftieth of a second late, as in every other program;
  `.m4a` files do not have this problem.
- On systems other than Windows, AAC needs `ffmpeg` installed; this build only runs on Windows.
