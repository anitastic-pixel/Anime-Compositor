# B-11 dependency record, checked against the build

The artifact is `docs/DEPENDENCIES.md` and the archived licence texts under `Licenses/`. This is the check that keeps them true: it reads `Cargo.lock`, which is the graph the compiler actually resolved, and requires that it and the record agree in both directions. Produced by `tests/b11_dependency_record.rs`.

The build currently resolves **28 dependencies** beneath the three that `Cargo.toml` names.

This check reads no licence and decides nothing about one. Document 10 reserves that for a reviewer, and there has not been one.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| the record's bill of materials is a table this check can read | the table parses into rows: true | the table parses into rows: true | pass |
| every crate the build resolves has a row in the record | none missing from the record | none missing from the record | pass |
| the record names no crate the build does not use | no rows without a crate | no rows without a crate | pass |
| every row carries the exact version the build resolved, not a range or an older one | no version disagrees | no version disagrees | pass |
| every crate's own licence text is archived in this repository, not merely named | every crate has an archived licence | every crate has an archived licence | pass |
| every row names a licence | every row names a licence | every row names a licence | pass |
| the record marks as direct exactly the three dependencies Cargo.toml asks for | png, rayon, serde_json | png, rayon, serde_json | pass |
| the crate declares the licence D-31 chose | MIT OR Apache-2.0 | MIT OR Apache-2.0 | pass |
| both licence texts the declaration names are in the repository | both present | both present | pass |
| the comparison can fail: a crate that is in neither file is reported as in neither | in the lock file: false, in the record: false | in the lock file: false, in the record: false | pass |

**10 of 10 checks pass.**
