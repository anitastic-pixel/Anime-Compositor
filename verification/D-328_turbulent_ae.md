# D-328 / B-212: Turbulent Displace in After Effects' units

From P-26's tutorial 2. You approved D-328 on 2026-10-05 ("proceed with proposals").

## What was wrong

Tutorial 2 puts Turbulent Displace, **Amount 80, Size 2**, on its glow. After Effects shows the glow still smooth, with a fine shimmer. This program pushed every pixel up to 80 pixels whatever the size (D-127), and at Size 2 each pixel goes a different way from its neighbour, so the glow broke into dust.

## What the sources say

None of them gives After Effects' own formula:

- Adobe's manual, [Distort effects](https://helpx.adobe.com/after-effects/using/distort-effects.html): Amount sets how much distortion there is, and Size sets the size of the distorted areas.
- An Adobe ideas thread, [Change Turbulent Displace's "Amount" and "Size" to "Amplitude" and "Wavelength"](https://community.adobe.com/t5/after-effects-ideas/change-turbulent-displace-s-quot-amount-quot-and-quot-size-quot-to-quot-amplitude-quot-and-quot/idi-p/14075919): users read Amount as how far a pixel moves and Size as how far apart the waves are.
- Tutorials ([Adobe Video World](https://adobevideoworld.com/turbulent-displace-after-effects/), [PremiumBeat heat waves](https://www.premiumbeat.com/blog/how-to-create-heat-waves-in-after-effects/), [Toon Squid's handbook](https://toonsquid.com/handbook/effects/turbulent_displace)) use a small Size with a large Amount for a fine heat-haze shimmer, never for dust.

## The rule chosen (this program's reading, not a measurement)

The push shrinks with the wave: **push = Amount × Size ÷ 100 pixels when Size is under 100**, and is the Amount itself at Size 100 and above. So Amount 80 at Size 2 pushes 1.6 pixels at most: the shimmer the tutorial shows. It is the simplest rule that fits every example above. If an After Effects frame ever shows otherwise, it can be corrected the way D-331 corrected Glow.

## What changed

- Turbulent Displace gains a **Units** choice in Effect controls: **After Effects** or **Classic (older projects)**.
- A Turbulent Displace you add from now on takes **After Effects**.
- **Any project saved before this is Classic.** It draws and saves exactly as before; the file is not even given a units word. FX-TURB-001 to 026 are unchanged.
- At Size 100 and above the two units give the same picture.
- The preview card draws it the same as the processor (largest difference 0 of 255).

## Pictures (`verification/D-328 pictures/`)

A soft gold glow over black, as made by `cargo test --test b212_turbulent_ae`:

| Picture | What to look for |
|---|---|
| `1_glow.png` | The glow, no warp. |
| `2_classic_amount80_size2.png` | Amount 80, Size 2 in Classic (every older project): the glow has turned to dust. |
| `3_ae_amount80_size2.png` | The same settings in After Effects units: the glow stays whole and smooth, with a fine shimmer at its edge, as tutorial 2 shows. |
| `4_ae_amount80_size100.png` | Amount 80 at Size 100 in After Effects units: the same as Classic would draw, the whole glow carried and bent by large waves. |

Measured on those pictures: the After Effects version differs from the plain glow by 1.3 levels of 255 on average, Classic by 50.5. Its grain (the step from each pixel to the next) is 2.1 against the plain glow's 1.8; Classic's is 34.8.

## Checks (cargo test)

All in `verification/D-328_turbulent_ae_table.md`, **67 of 67 pass**. Among them:

| Check | Result |
|---|---|
| FX-TURB-AE-001 to 012: every pixel matches `tools/turbulent_ae_reference.py`, written before the code | pass, tolerance 2e-5 |
| An old file (no units) draws exactly as before and is saved without units | pass |
| Units written wrong ("ae", "After_Effects") are refused with a sentence, and a file holding one warns and leaves the effect out | pass |
| A half-size draft pushes half as far | pass |
| The preview card matches the processor within 1 level of 255 | pass, largest difference 0 |

## For you to try

1. Open an older project with a Turbulent Displace: it looks exactly as before, and Effect controls show Units: Classic (older projects).
2. Add a Turbulent Displace to a soft glow. Set Amount 80 and Size 2: the glow shimmers but stays whole.
3. Switch Units to Classic: the same numbers break it into dust. Switch back.
4. Set Size 100 or more: switching Units changes nothing.

**Awaiting the owner's playtest.**
