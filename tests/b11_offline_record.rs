//! B-11's second piece, and T-10's first half: the build is examined for anything that could
//! reach a network, and for anything that could ask for an account.
//!
//! Writes `verification/B-11_offline_table.md`.
//!
//! # What this is for
//!
//! R-11 is a Must: "create, edit, save and export the reference shot without authentication or
//! network connectivity. No project content leaves the device." The charter says the same thing
//! in plainer words — the owner wants a tool they can run with the cable out and no subscription.
//!
//! A promise like that is usually shown by watching the program run, and
//! `verification/B-11_offline_run.md` does that — `tools/offline_check.ps1` records every
//! connection the program and the web view underneath it hold while it sits idle. But a program
//! can be quiet on the day it is watched and talkative on another one, so the watching is only
//! half of it. This is the other half: what is *linked into* the program, and what the page it
//! loads is *allowed* to ask for.
//!
//! # What it does not claim
//!
//! It does not claim the build contains no code that can open a socket. It does, and this table
//! names it rather than hiding it: `tauri` brings `tokio`, `mio` and `socket2` with it, and the
//! `net` feature of `tokio` is on. What is checked is narrower and true: none of that is reachable
//! from the part of the build that touches project content, the set of them has not changed, and
//! the page is refused any address but this machine by its own content policy.
//!
//! # Where the expected values come from
//!
//! The watchlist below is a list of crate names, written by hand, of the well-known ways a Rust
//! program talks over a network. The expected answer — which of them are in this build — was
//! read off the graph once, by hand, and is written down here so that a fifth one appearing is a
//! failure somebody has to look at rather than a silence. That is the whole point of the row: it
//! is not asserting that the answer is good, it is asserting that the answer has not changed.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

const PLATFORM: &str = "x86_64-pc-windows-msvc";

/// Crates that exist to talk over a network, or that are how something else does. Written by hand
/// from the common Rust ecosystem: HTTP clients and servers, async runtimes with socket support,
/// TLS stacks, websocket and DNS libraries. Being on this list is not an accusation — `http` is a
/// header type library — it is a reason to look.
const WATCHLIST: &[&str] = &[
    "actix-web",
    "async-std",
    "attohttpc",
    "axum",
    "curl",
    "curl-sys",
    "h2",
    "hickory-resolver",
    "http",
    "hyper",
    "isahc",
    "mio",
    "native-tls",
    "openssl",
    "openssl-sys",
    "quinn",
    "reqwest",
    "rocket",
    "rustls",
    "schannel",
    "smol",
    "socket2",
    "surf",
    "tiny_http",
    "tokio",
    "tokio-tungstenite",
    "tonic",
    "trust-dns-resolver",
    "tungstenite",
    "ureq",
    "warp",
];

/// Words a program uses when it wants an account. If any of these turn up in the one page this
/// application loads, something has changed that R-11 cares about.
const ACCOUNT_WORDS: &[&str] = &[
    "sign in",
    "sign-in",
    "signin",
    "log in",
    "login",
    "password",
    "account",
    "subscription",
    "subscribe",
    "activate",
    "licence key",
    "license key",
    "api key",
    "api_key",
    "token",
];

// ---------------------------------------------------------------------------------------
// Reporting
// ---------------------------------------------------------------------------------------

struct Row {
    check: String,
    expected: String,
    actual: String,
}

impl Row {
    fn pass(&self) -> bool {
        self.expected == self.actual
    }
}

#[derive(Default)]
struct Report {
    rows: Vec<Row>,
}

impl Report {
    fn check(&mut self, check: &str, expected: impl ToString, actual: impl ToString) {
        self.rows.push(Row {
            check: check.to_string(),
            expected: expected.to_string(),
            actual: actual.to_string(),
        });
    }
}

fn repo(rel: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(rel)
}

fn read(rel: &str) -> String {
    let path = repo(rel);
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("read {}: {e}", path.display()))
}

fn joined(items: &BTreeSet<String>) -> String {
    if items.is_empty() {
        "none".to_string()
    } else {
        items.iter().cloned().collect::<Vec<_>>().join(", ")
    }
}

// ---------------------------------------------------------------------------------------
// The graph
// ---------------------------------------------------------------------------------------

/// The resolved dependency graph for the one supported platform, as cargo reports it: every node's
/// package name, and the names it depends on for a normal (not build-time, not test-only) build.
struct Graph {
    /// Node id to package name.
    name: BTreeMap<String, String>,
    /// Node id to the node ids it links at run time.
    links: BTreeMap<String, Vec<String>>,
    /// Package name to node id, for the workspace's own crates.
    ours: BTreeMap<String, String>,
}

impl Graph {
    /// Every package name reachable from `root` by following run-time links only.
    fn beneath(&self, root: &str) -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        let mut names = BTreeSet::new();
        let mut queue = vec![root.to_string()];
        while let Some(id) = queue.pop() {
            if !seen.insert(id.clone()) {
                continue;
            }
            if let Some(name) = self.name.get(&id) {
                names.insert(name.clone());
            }
            if let Some(next) = self.links.get(&id) {
                queue.extend(next.iter().cloned());
            }
        }
        names.remove(self.name.get(root).map(String::as_str).unwrap_or(""));
        names
    }
}

fn graph() -> Graph {
    let out = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--format-version",
            "1",
            "--locked",
            "--filter-platform",
            PLATFORM,
        ])
        .current_dir(repo(""))
        .output()
        .expect("run cargo metadata");
    assert!(
        out.status.success(),
        "cargo metadata failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let meta: serde_json::Value =
        serde_json::from_slice(&out.stdout).expect("cargo metadata is not JSON");

    let mut name = BTreeMap::new();
    let mut ours = BTreeMap::new();
    for p in meta["packages"].as_array().expect("packages") {
        let id = p["id"].as_str().expect("id").to_string();
        let n = p["name"].as_str().expect("name").to_string();
        if p["source"].is_null() {
            ours.insert(n.clone(), id.clone());
        }
        name.insert(id, n);
    }

    let mut links: BTreeMap<String, Vec<String>> = BTreeMap::new();
    for node in meta["resolve"]["nodes"].as_array().expect("nodes") {
        let id = node["id"].as_str().expect("id").to_string();
        let mut deps = Vec::new();
        for dep in node["deps"].as_array().expect("deps") {
            // A build-time or test-only dependency is not linked into the program that ships, and
            // this question is about the program that ships.
            let normal = dep["dep_kinds"]
                .as_array()
                .expect("dep_kinds")
                .iter()
                .any(|k| k["kind"].is_null());
            if normal {
                deps.push(dep["pkg"].as_str().expect("pkg").to_string());
            }
        }
        links.insert(id, deps);
    }
    Graph { name, links, ours }
}

// ---------------------------------------------------------------------------------------
// The test
// ---------------------------------------------------------------------------------------

#[test]
fn b11_nothing_in_this_build_needs_a_network_or_an_account() {
    let mut report = Report::default();
    let g = graph();
    let core = g
        .ours
        .get("anime_compositor")
        .expect("the core crate")
        .clone();
    let shell = g
        .ours
        .get("anime_compositor_app")
        .expect("the shell crate")
        .clone();

    let watch: BTreeSet<String> = WATCHLIST.iter().map(|s| s.to_string()).collect();
    let in_shell = g.beneath(&shell);
    let in_core = g.beneath(&core);

    // ---- what is linked ----------------------------------------------------------------------
    let networked: BTreeSet<String> = in_shell.intersection(&watch).cloned().collect();
    report.check(
        "the crates in this build that could open a network socket, named rather than hidden",
        "http, mio, socket2, tokio",
        joined(&networked),
    );

    let in_the_core: BTreeSet<String> = in_core.intersection(&watch).cloned().collect();
    report.check(
        "none of them is reachable from the part that reads, renders and writes projects",
        "none",
        joined(&in_the_core),
    );

    // Everything the core links, listed, because it is short enough to read and because a project
    // that never touches a socket is easier to believe when you can see the whole list.
    report.check(
        "and that part's whole dependency list is small enough to read",
        "adler2, bitflags, cfg-if, crc32fast, crossbeam-deque, crossbeam-epoch, crossbeam-utils, \
         either, fdeflate, flate2, itoa, memchr, miniz_oxide, png, rayon, rayon-core, serde_core, \
         serde_json, simd-adler32, zlib-rs, zmij",
        joined(&in_core),
    );

    // ---- what the page may do ------------------------------------------------------------------
    let page = read("app/ui/index.html");
    // Every address written anywhere in the page, comments included, because a comment is where a
    // forgotten address hides before somebody uncomments it.
    let addresses: BTreeSet<String> = page
        .match_indices("://")
        .map(|(at, _)| {
            page[at..]
                .split(|c: char| c.is_whitespace() || c == '\'' || c == '"' || c == ';')
                .next()
                .unwrap_or("")
                .to_string()
        })
        .filter(|a| a.len() > 2)
        .collect();
    report.check(
        "the page names no address off this machine",
        "://frame.localhost, ://project.localhost",
        joined(&addresses),
    );

    let config = read("app/tauri.conf.json");
    let csp = config
        .lines()
        .find_map(|l| l.trim().strip_prefix("\"csp\": \""))
        .and_then(|r| r.strip_suffix('"'))
        .unwrap_or("no csp");
    report.check(
        "and is refused every other address by the window's own content policy",
        "default-src 'self'; connect-src 'self' http://frame.localhost http://project.localhost",
        csp,
    );

    // The interface is one file, compiled into the executable. There is nothing to serve and
    // nothing to fetch, which is why the window has no address to lose.
    let files: BTreeSet<String> = fs::read_dir(repo("app/ui"))
        .expect("read app/ui")
        .map(|e| e.expect("entry").file_name().to_string_lossy().to_string())
        .collect();
    report.check(
        "the whole interface is one file, carried inside the program",
        "index.html",
        joined(&files),
    );

    // ---- what it asks of the person --------------------------------------------------------------
    let lowered = page.to_lowercase();
    let found: BTreeSet<String> = ACCOUNT_WORDS
        .iter()
        .filter(|w| lowered.contains(**w))
        .map(|w| w.to_string())
        .collect();
    report.check(
        "the page has no words in it that belong to signing in",
        "none",
        joined(&found),
    );

    let manifest = read("app/Cargo.toml");
    let plugins: BTreeSet<String> = manifest
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .filter(|w| w.starts_with("tauri-plugin-"))
        .map(str::to_string)
        .collect();
    report.check(
        "the window asks for one framework plugin, and it opens file dialogs",
        "tauri-plugin-dialog",
        joined(&plugins),
    );

    // ---- the checks can fail --------------------------------------------------------------------
    // If the watchlist were being compared against itself, everything on it would look absent.
    report.check(
        "the watchlist can tell one of these apart from another: reqwest, then tokio",
        "reqwest in the build: false, tokio in the build: true",
        format!(
            "reqwest in the build: {}, tokio in the build: {}",
            in_shell.contains("reqwest"),
            in_shell.contains("tokio")
        ),
    );

    write_report(&report);
    let failed: Vec<&Row> = report.rows.iter().filter(|r| !r.pass()).collect();
    assert!(
        failed.is_empty(),
        "{} of {} checks failed, first: {} (expected {}, got {}); see verification/B-11_offline_table.md",
        failed.len(),
        report.rows.len(),
        failed[0].check,
        failed[0].expected,
        failed[0].actual
    );
}

fn write_report(report: &Report) {
    let passed = report.rows.iter().filter(|r| r.pass()).count();
    let mut out = String::new();
    out.push_str("# B-11, offline by construction\n\n");
    out.push_str(
        "R-11 asks that the reference shot can be created, edited, saved and exported \"without \
         authentication or network connectivity\", and that no project content leaves the device. \
         The connections recorded in `B-11_offline_run.md` are that promise being watched, and \
         they are not all clean. This table is \
         the same promise read off the build itself, because a program can be quiet on the day \
         somebody watches it. Produced by `tests/b11_offline_record.rs`.\n\n",
    );
    out.push_str(
        "The first row is the honest one. This build **does** contain code that can open a \
         socket: `tauri` brings `tokio`, `mio` and `socket2`, and `tokio`'s networking is switched \
         on. Naming them is the point — the row exists so that a fifth name appearing is something \
         a person has to look at. The rows under it are what makes that bearable: none of it is \
         reachable from the part of the program that reads, renders and writes projects, so the \
         code that touches the work cannot send it anywhere.\n\n",
    );
    out.push_str("| Check | Expected | Actual | Result |\n|---|---|---|---|\n");
    for row in &report.rows {
        out.push_str(&format!(
            "| {} | {} | {} | {} |\n",
            row.check,
            row.expected,
            row.actual,
            if row.pass() { "pass" } else { "**FAIL**" }
        ));
    }
    out.push_str(&format!(
        "\n**{passed} of {} checks pass.**\n",
        report.rows.len()
    ));
    out.push_str(
        "\n## What this does not cover\n\nWhether the operating system's own web view component \
         contacts anything on its own account. That is Microsoft's code running inside this \
         window, it is not in this dependency graph, and no test in this repository can speak for \
         it. What can be said about it is in `B-11_offline_run.md`, where the connections the \
         running program actually held were recorded.\n",
    );
    let path = repo("verification/B-11_offline_table.md");
    fs::write(&path, out).unwrap_or_else(|e| panic!("write {}: {e}", path.display()));
}
