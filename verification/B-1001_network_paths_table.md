# B-1001: network paths and Collect Files (D-268)

Written by `tests/b1001_network_paths.rs`. Do not edit by hand.

What changed: a project that names a media file on a network share (`\\server\share\...` and its other spellings) no longer makes the program read it, because reading it would make Windows send your sign-in proof to that server. The project keeps the path as it was, and you are told the file was not opened and how to fix it: copy the file to a drive on this computer, or to a mapped drive letter such as Z:. Collect Files now copies only the kinds of file the program can import, and names every file it left out.

**35 of 35 checks match.**

## Which paths are not read (M2)

| Check | Expected | The build's answer | Matches |
| --- | --- | --- | --- |
| `\\nas.invalid\share\cel.png` | not read | not read | yes |
| `//nas.invalid/share/cel.png` | not read | not read | yes |
| `/\nas.invalid\share\cel.png` | not read | not read | yes |
| `\\?\UNC\nas.invalid\share\cel.png` | not read | not read | yes |
| `//?/UNC/nas.invalid/share/cel.png` | not read | not read | yes |
| `\??\UNC\nas.invalid\share\cel.png` | not read | not read | yes |
| `\\.\UNC\nas.invalid\share\cel.png` | not read | not read | yes |
| `\\.\pipe\anything` | not read | not read | yes |
| `\\?\GLOBALROOT\Device\Mup\nas.invalid\share\cel.png` | not read | not read | yes |
| `\\?\C:\shots\cel.png` | read | read | yes |
| `Z:\shots\cel.png` | read | read | yes |
| `C:/shots/cel.png` | read | read | yes |
| `media/cel_0001.png` | read | read | yes |
| `..\media\cel_0001.png` | read | read | yes |
| `\shots\cel.png` | read | read | yes |
| a network path is refused with document 28's identifier | INVALID_PATH | INVALID_PATH | yes |
| and the reason and the way out are said | network share, sign-in, copy it to a drive on this computer, or a mapped drive letter | network share, sign-in, copy it to a drive on this computer, or a mapped drive letter | yes |
| a path on this computer is joined to the project's folder | I:\AI Coding\Claude\TotallyNotAfterEffects\target/b1001-scratch\local.png | I:\AI Coding\Claude\TotallyNotAfterEffects\target/b1001-scratch\local.png | yes |

## Opening a project that names them (M2)

| Check | Expected | The build's answer | Matches |
| --- | --- | --- | --- |
| one INVALID_PATH for each network path, and none for anything else | 6 | 6 | yes |
| `\\nas.invalid\share\cel.png` is named | named | named | yes |
| `//nas.invalid/share/fwd.png` is named | named | named | yes |
| `\\?\UNC\nas.invalid\share\verbatim.png` is named | named | named | yes |
| `\\.\UNC\nas.invalid\share\device.png` is named | named | named | yes |
| `\\nas.invalid\share\cel_0002.png` is named | named | named | yes |
| `\\nas.invalid\share\voice.wav` is named | named | named | yes |
| MEDIA_MISSING only for the mapped drive letter, which is looked for as usual | asset-mapped | asset-mapped | yes |
| every stored path is kept exactly as the project gave it | true | true | yes |
| and saving it writes them back unchanged | true | true | yes |

## Drawing a frame (M2)

| Check | Expected | The build's answer | Matches |
| --- | --- | --- | --- |
| frame 0 still draws | true | true | yes |
| the layer on the share says INVALID_PATH, and nothing else is said | INVALID_PATH | INVALID_PATH | yes |

## Collect Files (L1, and M2 for the files it reads)

| Check | Expected | The build's answer | Matches |
| --- | --- | --- | --- |
| collecting finishes | true | true | yes |
| the files it left out, by the path the project gave, in order | notes.txt; \\nas.invalid\share\cel.png; //nas.invalid/share/fwd.png; \\?\UNC\nas.invalid\share\verbatim.png; \\.\UNC\nas.invalid\share\device.png; \\nas.invalid\share\cel_0002.png; \\nas.invalid\share\voice.wav | notes.txt; \\nas.invalid\share\cel.png; //nas.invalid/share/fwd.png; \\?\UNC\nas.invalid\share\verbatim.png; \\.\UNC\nas.invalid\share\device.png; \\nas.invalid\share\cel_0002.png; \\nas.invalid\share\voice.wav | yes |
| only the drawing is copied (into both assets that use it) | media/asset-local/local.png; media/asset-seq/local.png; package-manifest.json; shot.json | media/asset-local/local.png; media/asset-seq/local.png; package-manifest.json; shot.json | yes |
| the manifest lists every file with its status; nothing left out was read (no size, no hash) | media/asset-local/local.png copied 75 true; media/asset-notes/notes.txt not media null false; media/asset-unc/cel.png network share null false; media/asset-fwd/fwd.png network share null false; media/asset-verbatim/verbatim.png network share null false; media/asset-device/device.png network share null false; media/asset-seq/local.png copied 75 true; media/asset-seq/cel_0002.png network share null false; media/asset-mapped/cel.png missing null false; media/asset-voice/voice.wav network share null false | media/asset-local/local.png copied 75 true; media/asset-notes/notes.txt not media null false; media/asset-unc/cel.png network share null false; media/asset-fwd/fwd.png network share null false; media/asset-verbatim/verbatim.png network share null false; media/asset-device/device.png network share null false; media/asset-seq/local.png copied 75 true; media/asset-seq/cel_0002.png network share null false; media/asset-mapped/cel.png missing null false; media/asset-voice/voice.wav network share null false | yes |
| checking the package: what was left out is excluded, the absent mapped file missing | media/asset-notes/notes.txt excluded; media/asset-unc/cel.png excluded; media/asset-fwd/fwd.png excluded; media/asset-verbatim/verbatim.png excluded; media/asset-device/device.png excluded; media/asset-seq/cel_0002.png excluded; media/asset-mapped/cel.png missing; media/asset-voice/voice.wav excluded | media/asset-notes/notes.txt excluded; media/asset-unc/cel.png excluded; media/asset-fwd/fwd.png excluded; media/asset-verbatim/verbatim.png excluded; media/asset-device/device.png excluded; media/asset-seq/cel_0002.png excluded; media/asset-mapped/cel.png missing; media/asset-voice/voice.wav excluded | yes |
