# B-11 dependency record, checked against the build

The artifact is `docs/DEPENDENCIES.md` and the archived licence texts under `Licenses/`. This is the check that keeps them true: it asks cargo what the build resolves for the one platform this project supports, and requires that the answer and the record agree in both directions. Produced by `tests/b11_dependency_record.rs`.

The build currently resolves **264 dependencies** beneath the four the workspace manifests name. `Cargo.lock` lists more, because it covers every platform cargo could resolve for; the count here is what compiles on `x86_64-pc-windows-msvc`.

This check reads no licence and decides nothing about one. Document 10 reserves that for a reviewer, and there has not been one.

## Checks

| Check | Expected | Actual | Result |
|---|---|---|---|
| the record's bill of materials is a table this check can read | the table parses into rows: true | the table parses into rows: true | pass |
| every crate the build resolves has a row in the record | none missing from the record | none missing from the record | pass |
| the record names no crate the build does not use | no rows without a crate | no rows without a crate | pass |
| every row carries the exact version the build resolved, not a range or an older one | no version disagrees | no version disagrees | pass |
| every crate's own licence text is archived in this repository, not merely named | every crate has an archived licence | every crate has an archived licence | pass |
| every crate the build resolves is in the committed lock file at that version | the lock file covers the build | the lock file covers the build | pass |
| every row names a licence | every row names a licence | every row names a licence | pass |
| the record marks as direct exactly the dependencies the manifests ask for | png, rayon, serde_json, tauri, tauri-build | png, rayon, serde_json, tauri, tauri-build | pass |
| the crate declares the licence D-31 chose | MIT OR Apache-2.0 | MIT OR Apache-2.0 | pass |
| both licence texts the declaration names are in the repository | both present | both present | pass |
| the comparison can fail: a crate that is in neither file is reported as in neither | in the build: false, in the record: false | in the build: false, in the record: false | pass |

**11 of 11 checks pass.**
