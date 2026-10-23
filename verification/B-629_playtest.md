# B-629 playtest: Curl Noise's Uniform Scaling, Scale Width and Scale Height (D-750)

Audit row 15. After Effects' Curl Noise lets you turn off **Uniform Scaling** and stretch the noise with **Scale Width** and **Scale Height**. Curl Noise now has those three settings.

## What to try in the app

1. Add Curl Noise to a layer. **Expect:** Uniform Scaling is On, and Scale Width and Scale Height are 100.
2. Set Scale to 200. **Expect:** the noise doubles in size both ways, as before.
3. Turn Uniform Scaling **Off**. **Expect:** Scale is no longer read, and Scale Width and Scale Height now set the size.
4. Set Scale Width to 300 and Scale Height to 100. **Expect:** the blobs are three times as wide and the same height.
5. Set Scale Width to 7500 and Scale Height to 5 (fimje's tutorial at 4:40). **Expect:** long, level streaks.
6. Key Scale Width, play it, then save and reopen the project. **Expect:** the keys are still there.
7. Open a project saved before this build that has Curl Noise. **Expect:** Uniform Scaling is On and the picture is unchanged.

## Pictures

In `verification/B-629 pictures/`, each picture shows the street with Curl Noise, Input Noise view, Size 10:

- **`1_uniform_on.png`:** as added; round blobs.
- **`2_width_300.png`:** Uniform Scaling Off, Width 300, Height 100; blobs stretched sideways.
- **`3_streaks_7500_by_5.png`:** Width 7500, Height 5; level streaks.

## Checks

`verification/B-629_curl_scale_table.md` has 46 of 46 checks passing:
- FX-CURL-079 to 085 to the reference.
- The files round-trip.
- An older file opens as Uniform Scaling On.
- Bad values are refused with a sentence.
- The card is within 1 level of the processor.
- The street pictures are measured:
  - **As added:** steps across and down about equal (2.29 / 2.36).
  - **Width 300:** steps across under half those down (0.75 / 2.58).
  - **The streaks:** steps down over ten times those across (38.78 / 0.04).

The panel's settings are covered by the app's contract test (Uniform Scaling Off, Width 150, Height 60 typed in).

## Speed

Unchanged. The stretch is two numbers in the same step the card already did for Scale.

## Verdict for the owner

- **Pass** if steps 1 to 7 behave as described.
- **Fail** if the stretch goes the wrong way, if Scale still acts with Uniform Scaling Off, or if an older project changes.
