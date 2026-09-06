# B-11, offline by construction

R-11 asks that the reference shot can be created, edited, saved and exported "without authentication or network connectivity", and that no project content leaves the device. The connections recorded in `B-11_offline_run.md` are that promise being watched, and they are not all clean. This table is the same promise read off the build itself, because a program can be quiet on the day somebody watches it. Produced by `tests/b11_offline_record.rs`.

The first row is the honest one. This build **does** contain code that can open a socket: `tauri` brings `tokio`, `mio` and `socket2`, and `tokio`'s networking is switched on. Naming them is the point — the row exists so that a fifth name appearing is something a person has to look at. The rows under it are what makes that bearable: none of it is reachable from the part of the program that reads, renders and writes projects, so the code that touches the work cannot send it anywhere.

| Check | Expected | Actual | Result |
|---|---|---|---|
| the crates in this build that could open a network socket, named rather than hidden | http, mio, socket2, tokio | http, mio, socket2, tokio | pass |
| none of them is reachable from the part that reads, renders and writes projects | none | none | pass |
| and that part's whole dependency list is small enough to read | adler2, bitflags, cfg-if, crc32fast, crossbeam-deque, crossbeam-epoch, crossbeam-utils, either, fdeflate, flate2, itoa, memchr, miniz_oxide, png, rayon, rayon-core, serde_core, serde_json, simd-adler32, zlib-rs, zmij | adler2, bitflags, cfg-if, crc32fast, crossbeam-deque, crossbeam-epoch, crossbeam-utils, either, fdeflate, flate2, itoa, memchr, miniz_oxide, png, rayon, rayon-core, serde_core, serde_json, simd-adler32, zlib-rs, zmij | pass |
| the page names no address off this machine | ://frame.localhost, ://project.localhost | ://frame.localhost, ://project.localhost | pass |
| and is refused every other address by the window's own content policy | default-src 'self'; connect-src 'self' http://frame.localhost http://project.localhost | default-src 'self'; connect-src 'self' http://frame.localhost http://project.localhost | pass |
| the whole interface is one file, carried inside the program | index.html | index.html | pass |
| the page has no words in it that belong to signing in | none | none | pass |
| the window asks for one framework plugin, and it opens file dialogs | tauri-plugin-dialog | tauri-plugin-dialog | pass |
| the watchlist can tell one of these apart from another: reqwest, then tokio | reqwest in the build: false, tokio in the build: true | reqwest in the build: false, tokio in the build: true | pass |

**9 of 9 checks pass.**

## What this does not cover

Whether the operating system's own web view component contacts anything on its own account. That is Microsoft's code running inside this window, it is not in this dependency graph, and no test in this repository can speak for it. What can be said about it is in `B-11_offline_run.md`, where the connections the running program actually held were recorded.
