# B-116a: effect presets in a file (D-180)

Written on 2026-09-28, before any code. Accepted with items 1 to 9 of the list of effects and presets left to build.

## What you will be able to do

Today your presets live inside this window only (D-166). If the computer is replaced, or you want the same presets on a second machine, they cannot go with you.

After B-116:

- **Effects panel → right-click → Export presets…** asks where to save, and writes every preset you made into one file ending `.fxpreset`.
- **Right-click one preset → Export this preset…** writes just that one.
- **Effects panel → right-click → Import presets…** asks for a `.fxpreset` file and adds its presets to your Presets folder.

A preset whose name you already have is not replaced. It comes in as the same name with " 2" after it (or " 3", and so on), and the status line tells you. Nothing you already had is changed by an import.

## What is in the file

Plain text you could open in Notepad. For one preset holding a Glow:

```
{
  "preset_file_version": 0,
  "presets": [
    {
      "name": "Soft glow",
      "effects": [ ...the Glow, written exactly as a project file writes it... ]
    }
  ]
}
```

## All or nothing

If anything in a file is wrong, **nothing** is imported and the status line says what was wrong. The fixtures test each of these:

| You choose | What happens |
| --- | --- |
| a good file with three presets | all three are added, in the file's order |
| a preset with keyed settings | the keys come too, at the same frames |
| a preset with a setting out of range | it is added; applied, that effect is not drawn and the layer says why, as in a project |
| a text file, not a preset file | refused: "not JSON" |
| a project file by mistake | refused, and told it is a project |
| a file from a newer version of the program | refused, never guessed at |
| a file with something extra in it | refused, naming the extra thing, since it would be lost |
| a preset with no name, no effects, or a name used twice | refused, naming it |
| an effect this build does not have, even in the last preset | refused whole; not even the first presets are added |
| a setting this build does not have | refused, naming the setting |

## How you will check it

The build's test reads all nineteen fixture files and writes a table, `verification/B-116_preset_file_table.md`, with one line per file: what it should do and what it did. A playtest sheet walks you through exporting, importing on a fresh window, and trying a broken file.
