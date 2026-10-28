# B-827 playtest: sums in number fields (D-947)

What changed: any number field takes a small sum. Starting with +, * or / works on the value already there.

## Steps

1. Open any project and choose a layer. Show its Position with **P**.
2. Click the first Position number (say it reads 960) so you can type in it. Type **+400** and press Enter. It now reads 1360 and the layer moved right.
3. Click it again, type **/2**, Enter: it reads 680.
4. Type **\*2**, Enter: 1360 again.
5. Type **960-400**, Enter: 560.
6. Type **-400**, Enter: -400 (a minus at the start is a negative number, as in After Effects).
7. Type **2+** (an unfinished sum), Enter: the field keeps what it had.
8. Press **Ctrl+Z** a few times: each sum is one step back.
9. Try the same in another panel's number field (Opacity, an effect's slider number): it works the same.

## What you should not see

- "+400" setting the value to 400.
- A half-typed sum changing the value to 0.

The computer's checks are in `verification/B-827_typed_sums_table.md`; the check in a private copy of the app is `verification/B-827_private_app_table.md`.
