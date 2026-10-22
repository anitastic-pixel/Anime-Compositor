# B-619 playtest: the window's name lists are checked against the core's

## What changed for you

Nothing on screen. The window keeps its own copy of several lists: the numbers a shape, a mask, a shape group and a repeater can be keyed on, each parametric shape's settings, and each path operation's numbers, ranges and choices. Until now nothing checked that the copy matched the list the app's core works from. A name added to one list and forgotten in the other would show up only as a row that never draws or a key the app refuses.

## What the check found: 24 of 24 lists match

`app/src/tests/name_lists.rs` reads the page the window shows and compares each list with the core's. See `B-619_name_lists_table.md`: one line per list, with both copies side by side.

To show that the check catches a mismatch, I added a made-up name, `choke`, to the page's mask list and ran it again. It failed with 23 of 24, with the mask line marked **NO**. The page was then put back.

## Nothing to try

There is nothing to playtest; this only guards future changes.
