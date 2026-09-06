# B-11, what the running program actually connects to

R-11 says the application works offline and sends nothing anywhere. `B-11_offline_table.md` is
that promise read off the build: which crates could open a socket, which of them the core can
reach, what addresses the page contains, what the content security policy allows. This file is
the other half — the program was started, left alone, and watched.

The result is not clean, and the honest version of it is below rather than in a footnote:
**this application opened no connection to anything. The web view component Windows runs inside
its window opened four, to two addresses off this machine, in every run.**

## What was run

```
cargo build -p anime_compositor_app --release
powershell -ExecutionPolicy Bypass -File tools/offline_check.ps1 -Open "target\shot\my_shot.json" -Seconds 30
```

The script starts the release shell on a project, finds every process underneath it by walking
the parent links Windows keeps — the shell itself, and the web view processes it starts — and
records every TCP connection any of them holds, once a second, for thirty seconds. Nobody touched
the window while it watched. This is idle, not use.

## What it recorded, on 2026-09-05

```
watched for 30 seconds
processes in the tree: anime_compositor_app.exe, msedgewebview2.exe
tcp connections held, in total: 16
of those, to somewhere off this machine: 4
```

Sixteen entries, and they sort like this:

| Kind | Held by | Count | What it is |
| --- | --- | --- | --- |
| Bound, no remote address | `msedgewebview2.exe` | 8 | a socket reserved and not connected |
| To `127.0.0.1:80` / `::1:80`, `SynSent` | `msedgewebview2.exe` | 4 | an attempt to reach this machine that never completed |
| To `2001:578:3f::30:443`, established | `msedgewebview2.exe` | 2 | the reverse name is `cdns1.cox.net` — this machine's own internet provider's DNS service, over HTTPS |
| To `2603:1036:30c:*::2:443`, established | `msedgewebview2.exe` | 2 | no reverse name; the last part of the address differs from run to run |
| Anything at all | `anime_compositor_app.exe` | **0** | — |

The bottom row is the one this project is answerable for. The application's own process held no
connection, to anything, in any run.

The four `SynSent` entries are attempts to reach port 80 on this machine which never connected —
`SynSent` means the first packet went out and nothing came back, and nothing is listening there.
The likeliest explanation is the two addresses the page uses, `frame.localhost` and
`project.localhost`, being tried as ordinary web addresses before the web view answers them
itself — those requests never reach a socket, because they are intercepted inside the process and
answered by Rust. That explanation is not proven here. What is certain from the addresses alone
is that these four are aimed at this machine, that they never connected, and that they carry
nothing.

The other four are real, outbound, and encrypted. Two are a DNS service — the machine asking what
a name resolves to, over HTTPS rather than in the clear. The other two have no name to look up.
None of them belongs to code in this repository.

## What was done about it, and what it did not achieve

The web view is Chromium, and Chromium has switches for this. `app/src/main.rs` sets five of them
before the window is created — no background networking, no component updates, no reporting, no
pings, no sync — and later a sixth, an attempt to turn off DNS-over-HTTPS.

They did not close it. The run before any of them showed five off-machine connections in twenty
seconds; every run since has shown four, including after the DNS switch was added. The switches
are kept, because leaving them unset would also be a choice and they narrow what is left to
explain, but nothing here should be read as saying they solved the problem. They did not.

## What this leaves standing, and what the owner should decide

What can be said, and is backed by this file and the table beside it:

- This application makes no network connection. It contains no HTTP client, and no crate in this
  build that can open a socket is reachable from the code that reads projects and renders frames.
- Nothing it writes goes anywhere except to a file the person chose.
- The page inside the window is confined by its content security policy to two addresses on this
  machine, and contains no other address.

What cannot be said, and is the open question:

- **A project's contents pass through Microsoft's code.** The frames are drawn into the web view
  and the project's name and warnings arrive there as text. That process is the one holding the
  four connections. Nothing in this repository can prove what does or does not travel over them,
  because the code at both ends is not ours.

That is a decision for the owner rather than a defect an agent can fix, and it is registered as
**D-39** in `Markdown/14_Decisions_Risks.md`. The options it lays out are: accept it and say so
in the shipped documentation; block the process at the firewall as part of installing; or replace
the web view, which is a different architecture and a different program.

## What this check cannot do

It cannot disconnect the machine. It says what the program tried to do while a network was
available; it says nothing about whether the program still works when there is none. That is the
other half of R-11 and it needs a person: disable the network adapter, then open a project,
scrub, play and export. Both halves are worth having and neither is the other.

It also watched an idle window for thirty seconds. It is a tripwire, not a proof — a program can
be quiet on the day somebody watches it, which is exactly why `B-11_offline_table.md` reads the
build instead of the moment.
