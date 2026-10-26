use super::*;

fn header(r: &Response<Vec<u8>>, name: &str) -> String {
    r.headers()
        .get(name)
        .map_or_else(String::new, |v| v.to_str().unwrap_or("").to_string())
}

/// The page's `decodeURIComponent` of `for_a_header`'s `%XX` escapes.
fn profile_of(r: &Response<Vec<u8>>) -> Option<serde_json::Value> {
    let text = header(r, "x-profile");
    if text.is_empty() {
        return None;
    }
    let mut bytes = Vec::new();
    let raw = text.as_bytes();
    let mut i = 0;
    while i < raw.len() {
        if raw[i] == b'%' {
            bytes.push(u8::from_str_radix(&text[i + 1..i + 3], 16).expect("a %XX escape"));
            i += 3;
        } else {
            bytes.push(raw[i]);
            i += 1;
        }
    }
    Some(serde_json::from_slice(&bytes).expect("x-profile is JSON"))
}

fn layer<'a>(p: &'a serde_json::Value, name: &str) -> &'a serde_json::Value {
    p["layers"]
        .as_array()
        .expect("layers")
        .iter()
        .find(|l| l["name"] == name)
        .unwrap_or_else(|| panic!("no row for {name} in {p}"))
}

fn effect<'a>(l: &'a serde_json::Value, name: &str) -> Option<&'a serde_json::Value> {
    l["effects"]
        .as_array()?
        .iter()
        .find(|e| e["name"].as_str().is_some_and(|n| n.eq_ignore_ascii_case(name)))
}

/// D-901: the viewer's profiler, on the reference shot with a Gaussian Blur on layer2 (and then
/// an Echo before it). Each of the four layers gets one row under its own name; the blurred
/// layer is the slowest and its blur has a time; the processor draws everything when the CPU
/// draws the frame; on the graphics card the blur is the card's and the Echo the processor's;
/// the layers' times add up to no more than the frame's; a frame shown from memory has no rows;
/// nothing is reported with the profiler off; and the picture is byte for byte the same with it
/// on or off.
///
/// Writes `verification/B-781_profiler_table.md`.
#[test]
fn b781_profiler_reports_layers_and_where_they_were_drawn() {
    use PreviewQuality::Full;
    let export = Mutex::new(Export::default());
    // `viewer` is profiled; `plain` is the same project never profiled, for the pixels.
    let viewer = Mutex::new(demo());
    let plain = Mutex::new(demo());
    let add = |id: &str, effect: Effect, index: Option<usize>| Command::AddEffect {
        composition: Id::new("comp-reference-shot"),
        layer_id: Id::new("layer-2"),
        effect: EffectInstance::new(Id::new(id), effect),
        index,
    };
    let blur = || Effect::GaussianBlur {
        sigma_px: 24.0,
        edges: "transparent".into(),
        dimensions: "both".into(),
        units: "sigma".into(),
    };
    edit(&viewer, add("b781-blur", blur(), None));
    edit(&plain, add("b781-blur", blur(), None));
    let ask_of = |viewer: &Mutex<Viewer>, prof: u8, card: Option<&Mutex<Card>>| {
        profiled(prof, || {
            serve_frame(viewer, &export, None, card, Ask::Frame(100), Some(Full), None, false, None)
        })
    };
    let ask = |prof: u8, card: Option<&Mutex<Card>>| ask_of(&viewer, prof, card);
    let mut rows: Vec<(String, String, String, bool)> = Vec::new();
    let mut check = |what: &str, expected: &str, got: String, ok: bool| {
        rows.push((what.to_string(), expected.to_string(), got, ok));
    };

    // The processor.
    let off = ask_of(&plain, 0, None);
    check(
        "Profiler off: nothing reported",
        "no x-profile",
        format!("x-profile {:?}", header(&off, "x-profile")),
        header(&off, "x-profile").is_empty(),
    );
    let on = ask(2, None);
    let p = profile_of(&on).expect("a profile when asked");
    let names: Vec<&str> = p["layers"].as_array().unwrap().iter().map(|l| l["name"].as_str().unwrap()).collect();
    let mut sorted = names.clone();
    sorted.sort_unstable();
    check(
        "CPU: one row per layer, by name",
        "layer1, layer2, layer3, layer4",
        sorted.join(", "),
        sorted == ["layer1", "layer2", "layer3", "layer4"],
    );
    let all_processor = p["layers"].as_array().unwrap().iter().all(|l| l["on"] == "processor");
    check(
        "CPU: every layer drawn on the processor",
        "processor x4",
        p["layers"].as_array().unwrap().iter().map(|l| l["on"].to_string()).collect::<Vec<_>>().join(", "),
        all_processor && p["drawn_on"] == "CPU" && p["from_memory"] == false,
    );
    let l2 = layer(&p, "layer2");
    let blur = effect(l2, "gaussian blur").map(|e| (e["ms"].as_f64().unwrap_or(0.0), e["on"].clone()));
    check(
        "CPU: layer2's blur timed, on the processor",
        "gaussian blur > 0 ms, processor",
        format!("{blur:?}"),
        blur.as_ref().is_some_and(|(ms, on)| *ms > 0.0 && on == "processor"),
    );
    let ms = |l: &serde_json::Value| l["ms"].as_f64().unwrap();
    let slowest = p["layers"].as_array().unwrap().iter().max_by(|a, b| ms(a).total_cmp(&ms(b))).unwrap();
    check(
        "CPU: the blurred layer is the slowest",
        "layer2",
        format!("{} ({} ms; others {})", slowest["name"], slowest["ms"],
            p["layers"].as_array().unwrap().iter().filter(|l| l["name"] != "layer2").map(|l| l["ms"].to_string()).collect::<Vec<_>>().join(", ")),
        slowest["name"] == "layer2",
    );
    let blur_ms = blur.map_or(0.0, |b| b.0);
    let sum: f64 = p["layers"].as_array().unwrap().iter().map(ms).sum();
    check(
        "CPU: blur within its layer, layers within the frame",
        "blur <= layer2 <= sum of layers <= frame",
        format!("{blur_ms} <= {} <= {sum:.2} <= {}", ms(l2), p["ms"]),
        blur_ms <= ms(l2) && sum <= p["ms"].as_f64().unwrap() + 0.01,
    );
    check(
        "CPU: the picture with the profiler on",
        "byte for byte the picture with it off",
        format!("{} bytes, same: {}", on.body().len(), on.body() == off.body()),
        on.body() == off.body(),
    );
    let again = ask(1, None);
    let q = profile_of(&again).expect("a profile when asked");
    check(
        "The same frame asked again, profiler still open",
        "no layer rows if shown from memory, else four",
        format!("from_memory {}, {} rows", q["from_memory"], q["layers"].as_array().unwrap().len()),
        q["layers"].as_array().unwrap().len() == if q["from_memory"] == true { 0 } else { 4 },
    );
    let fresh = ask(2, None);
    let q = profile_of(&fresh).expect("a profile when asked");
    let l2 = layer(&q, "layer2");
    check(
        "Measured again: layer2's blur kept from the first time",
        "layer2 from cache 1 effect, no blur time",
        format!("kept {}, blur {:?}", l2["kept"], effect(l2, "gaussian blur").map(|e| e["ms"].clone())),
        l2["kept"] == 1 && effect(l2, "gaussian blur").is_none(),
    );

    // The graphics card, when this machine has one.
    let card = Mutex::new(Card::default());
    let said = card.lock().unwrap().set(DrawOn::Gpu);
    if card.lock().unwrap().on {
        let off = ask(0, Some(&card));
        let on = ask(2, Some(&card));
        let p = profile_of(&on).expect("a profile when asked");
        let l2 = layer(&p, "layer2");
        let blur = effect(l2, "gaussian blur");
        check(
            "Card: layer2's blur drawn by the card",
            "layer2 on card, blur on card",
            format!("frame {}, layer2 {}, blur {:?}", p["drawn_on"], l2["on"], blur.map(|b| b["on"].clone())),
            p["drawn_on"] == "GPU" && l2["on"] == "card" && blur.is_some_and(|b| b["on"] == "card"),
        );
        check(
            "Card: the picture with the profiler on",
            "byte for byte the picture with it off",
            format!("same: {}", on.body() == off.body()),
            on.body() == off.body(),
        );
        edit(&viewer, add("b781-echo", new_effect(ECHO).expect("an Echo"), Some(0)));
        let on = ask(2, Some(&card));
        let p = profile_of(&on).expect("a profile when asked");
        let l2 = layer(&p, "layer2");
        let echo = effect(l2, "echo");
        let blur = effect(l2, "gaussian blur");
        check(
            "Card: an Echo put before the blur, which only the processor draws",
            "layer2 card and processor; echo processor; blur card",
            format!(
                "layer2 {}, echo {:?}, blur {:?}",
                l2["on"],
                echo.map(|e| e["on"].clone()),
                blur.map(|b| b["on"].clone())
            ),
            l2["on"] == "card and processor"
                && echo.is_some_and(|e| e["on"] == "processor")
                && blur.is_some_and(|b| b["on"] == "card"),
        );
    } else {
        check("Card", "a graphics card", format!("none: {said:?}"), true);
    }

    let mut table = String::from(
        "# B-781 profiler (D-901)\n\nWritten by `cargo test -p anime_compositor_app \
         b781_profiler_reports_layers_and_where_they_were_drawn`. The reference shot, frame 100, \
         Full quality, a Gaussian Blur (sigma 24) on layer2. Times are this run's and vary; the \
         checks are on names, places and order.\n\n| Check | Expected | Got | Result |\n|---|---|---|---|\n",
    );
    for (what, expected, got, ok) in &rows {
        table.push_str(&format!(
            "| {what} | {expected} | {} | {} |\n",
            got.replace('|', "\\|"),
            if *ok { "pass" } else { "FAIL" }
        ));
    }
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the app crate has a parent directory")
        .join("verification/B-781_profiler_table.md");
    std::fs::write(&path, &table).expect("write the table");
    assert!(rows.iter().all(|r| r.3), "{table}");
}
