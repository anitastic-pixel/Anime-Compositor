# B-340: Burn Film

Built on 2026-10-11 as D-460, under your /loop request and your go-ahead: **Burn Film**, in
**Stylize**, after After Effects' CC Burn Film. The layer burns away like film stuck in a
projector: a hole opens round a centre point, more open elsewhere a moment later, each ringed
by a scorched edge darkening to black and a thin black char line, spreading until nothing is left.
Whatever is beneath the layer shows through the holes.

- **Burn** (0%, 0 to 100): how much has burned. Key it from 0 to 100 to play the burn.
- **Center** (50%, 50%): where the first hole opens.
- **Random Seed** (0): where the other holes open and how ragged the edges are.

The controls are After Effects' own; how it draws is our own rule beyond what the sources show,
not matched to After Effects pixel for pixel. The edge is black, as in After Effects, with no
colour control; for a glowing edge, put a Glow above it, as After Effects users do.

The check, `verification/D-460_burn_film_table.md` (136 of 136), holds every pixel to numbers
worked out by a separate program before the code existed, and holds the graphics card's picture
to within 1 level of the processor's. The pictures are in `verification/D-460 pictures/`, over the
street (`town.png`; `1_before.png` without the effect).

## What to check

1. **A hole.** `2_burn_30.png` (Burn 30): a hole through the middle with a black ring and a
   darkened edge round it; the corners untouched.
2. **Further.** `3_burn_60.png` (Burn 60): much more of the street gone, other holes joined in.
3. **From the corner.** `4_from_the_corner.png` (Center 0, 0): the burn starts at the top left.
4. **Another seed.** `5_seed_7.png` (Random Seed 7): the same amount burned, ragged differently.
5. **In the app.** Put a picture under another layer, add Burn Film (Stylize) to the top one,
   key Burn from 0 at the start to 100 two seconds later and play it: the top layer burns
   away to show the one beneath. Move Center and change Random Seed.
6. **Out of range.** Type 101 in Burn: it is refused with a sentence giving the range.
7. **Saved and opened again.** Save, close and open the project: the same settings and picture.

## Speed

In `verification/B-340_burn_film_timing_table.md` (PROVISIONAL, the machine was busy): the reference shot with a moving
Noise and a Burn Film on three layers, played again, 166.9 ms a frame on the card at
burn 50 against 76.7 for the Noise alone; on the processor 213.2 against
59.5: a busy machine cannot say whether the card is quicker than the processor for this effect.
