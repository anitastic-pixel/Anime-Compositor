# B-11, the thing that would be handed to somebody

Until now this project has produced fixtures, artifacts and an executable under `target/`. This
is the first time it has produced something shaped like a copy of the program, with the papers
that have to travel with it. T-16 asks that the final dependency manifest be compared with the
notices actually included; there was no distribution to inspect, and now there is one.

## What was made

```
cargo build -p anime_compositor_app --release
powershell -ExecutionPolicy Bypass -File tools/package.ps1
```

`target/package/AnimeCompositor-0.1.0/`, and a zip of the same, 3.7 MB:

| In the folder | Size | What it is |
| --- | --- | --- |
| `Anime Compositor.exe` | 9.3 MB | the whole program — the interface is compiled into it, there is nothing beside it to load |
| `READ ME FIRST.md` | 4.9 KB | `docs/SUPPORTED_ENVELOPE.md`: what it needs, what it does, what it does not do yet, and the network finding |
| `DEPENDENCIES.md` | 44.6 KB | every crate in this build with its version and licence, generated from the build |
| `LICENSE-MIT`, `LICENSE-APACHE` | 12 KB | this project's own terms, D-31 |
| `Licenses/` | 486 files in 271 directories | the licence and notice text of every crate, one directory each |

271 directories and 271 crates in the manifest, which is the comparison T-16 asks for. It is not
a claim made by this file: `tools/archive_licenses.py --check` runs on every CI build and fails
if a single crate in the resolved graph has no directory or a single directory has no crate, and
the package copies that archive whole rather than rebuilding it. The staging script keeps the
path below `Licenses/`, because many crates ship a file called `LICENSE-MIT` and flattening them
into one folder would keep one and silently lose 270.

## What this is not

**It is not an installer, and `bundle.active` in `app/tauri.conf.json` is still false.** Building
an MSI or an NSIS installer needs a signing certificate this project does not have, and an
unsigned installer is a worse thing to hand somebody than a folder: Windows warns harder about it
and it writes to places the person cannot easily inspect. The folder is the whole program, and
deleting it removes the program. `READ ME FIRST.md` says all of that to whoever receives it.

**It is not signed.** Windows will warn about an unrecognised program the first time it runs.

**It has not been run from anywhere but this machine.** Copying it to a computer that has never
had a compiler on it is the check that matters and it needs a second machine. The one thing it
depends on and does not carry is the Microsoft Edge WebView2 Runtime, which Windows 11 includes.

**Nothing here is committed.** The package is written to `target/`, which is gitignored, because
a 3.7 MB zip rebuilt from the same inputs does not belong in the history. This file, the script
and the envelope document are what is committed.

## What T-16 still owes

The mechanical half is discharged and checked on every build: the record describes the resolved
graph in both directions, the archive matches the record, and the distribution carries the
archive. The half that remains is the one document 10 reserves for a person — a legal review of
what those licences require. There has not been one, the reviewer and date fields in
`docs/DEPENDENCIES.md` are blank rather than invented, and the entries that would need a decision
are flagged there: the MPL-2.0 crates under `tauri`, the `Unicode-3.0` terms with no alternative,
the conjunctive licences, and the crates that declare a licence and ship no text for it.
