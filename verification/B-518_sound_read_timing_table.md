# B-518: how long reading sound takes (D-639)

Measured on 2026-10-13, built with `cargo test --release --test b518_sound_read` from 3d1fe2a9, and run with `--ignored b518_sound_read_timing`. **These numbers are PROVISIONAL**: the machine was not quiet. Four other cargo or rustc processes (other lanes' builds) were running. Treat them as an upper bound, to be measured again on a quiet machine.

- Processor: AMD Ryzen 9 9900X 12-Core Processor, 24 threads, 62 GB of memory
- System: Windows 11
- Build: release

**What is timed.** A three-minute 44,100 stereo sine made by ffmpeg, as a 16-bit WAV, a FLAC, an MP3 (libmp3lame, its default rate) and an M4A (ffmpeg's AAC encoder). **Read** is `audio::read_file`, the whole file into memory. **Working sound** is `Sound::working` afterwards: 44,100 to 48,000, two channels. One run each, milliseconds.

| File | Read (ms) | Working sound (ms) | Working samples |
|---|---:|---:|---:|
| wav | 13 | 228 | 8640000 |
| flac | 145 | 325 | 8640000 |
| mp3 | 207 | 418 | 8640000 |
| m4a | 2622 | 295 | 8640000 |

The M4A is read by Windows' own decoder (Media Foundation's source reader, one piece at a time), which is where its 2.6 s goes; the code around it only copies the pieces. A sound is read once, when it is brought in or the project opens, and kept (D-420), so this is a wait at import, not a cost per frame. The working sound's time is the same rule for every format; the spread between rows is the busy machine.
