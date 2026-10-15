use super::*;

fn run(viewer: &Mutex<Viewer>, what: &str) -> String {
    let (id, query) = what
        .split_once('?')
        .map_or((what, None), |(i, q)| (i, Some(q)));
    edit_command(viewer, id, query).expect("a command document 24 lists")
}

fn effects(viewer: &Mutex<Viewer>) -> Vec<EffectInstance> {
    let held = viewer.lock().expect("the viewer lock was poisoned");
    let comp = held
        .document
        .project()
        .composition(&held.composition)
        .expect("the composition on screen");
    comp.layer(&Id::new("layer-3"))
        .expect("layer-3")
        .effects
        .clone()
}

#[test]
fn every_keyable_setting_on_every_effect_survives_a_save() {
    let viewer = Mutex::new(demo());
    // Every effect the core names, read from its own list, so one added later is here too.
    let ids: Vec<&str> = include_str!("../../../src/effects.rs")
        .lines()
        .filter_map(|l| {
            l.strip_prefix("pub const ")?
                .split_once(": &str = \"")?
                .1
                .strip_suffix("\";")
        })
        // D-642: a sound effect goes on a sound layer only; b521 saves a keyed gate.
        .filter(|id| id.starts_with("core.") && new_effect(id).is_some_and(|e| !e.is_sound()))
        .collect();
    assert!(ids.len() > 90, "the effect list was not found: {ids:?}");
    let mut keyed = Vec::new();
    for id in &ids {
        let had: Vec<Id> = effects(&viewer)
            .into_iter()
            .map(|e| e.instance_id)
            .collect();
        run(&viewer, &format!("effect.add?layer=layer-3&type={id}"));
        // Some effects go in ahead of others in the stack, so the new one is found by its id.
        let e = effects(&viewer)
            .into_iter()
            .find(|e| !had.contains(&e.instance_id))
            .expect("the effect just added");
        let fx = e.instance_id.as_str().to_string();
        let names: Vec<&str> = e
            .effect
            .names()
            .into_iter()
            .chain(e.has_mix().then_some("mix"))
            .collect();
        for name in names {
            let numbers = e.get(name).expect("a named setting");
            // A list setting with nothing in it yet (Mesh Warp's mesh, Liquify's stroke) has no
            // key to set: asking for one changes nothing and must not stop the app.
            if numbers.is_empty() {
                run(
                    &viewer,
                    &format!("keyframe.add_remove?layer=layer-3&prop=fx:{fx}:{name}&frame=0"),
                );
                let after = effects(&viewer)
                    .into_iter()
                    .find(|x| x.instance_id == e.instance_id)
                    .expect("the effect");
                assert!(
                    !after.tracks.contains_key(name),
                    "{fx} {name}: an empty list setting was keyed"
                );
                continue;
            }
            let value = numbers
                .iter()
                .map(f64::to_string)
                .collect::<Vec<_>>()
                .join(",");
            let prop = format!("fx:{fx}:{name}");
            run(
                &viewer,
                &format!("keyframe.add_remove?layer=layer-3&prop={prop}&frame=0"),
            );
            run(
                &viewer,
                &format!("property.set_base?layer=layer-3&prop={prop}&frame=10&value={value}"),
            );
            keyed.push((fx.clone(), name));
        }
    }
    let before = effects(&viewer);
    let missing: Vec<_> = keyed
        .iter()
        .filter(|(fx, name)| {
            before
                .iter()
                .find(|e| e.instance_id.as_str() == fx)
                .and_then(|e| e.tracks.get(*name))
                .is_none_or(|t| t[0].keyframes().len() != 2)
        })
        .collect();
    assert!(
        missing.is_empty(),
        "these were not keyed by the commands: {missing:?}"
    );

    let written = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        persist::to_json(held.document.project(), &held.preserved)
    };
    let reopened =
        persist::load_str(&written).unwrap_or_else(|d| panic!("{}: {}", d.message, d.detail));
    let again = persist::to_json(reopened.document.project(), &reopened.preserved);
    let comp = reopened
        .document
        .project()
        .compositions
        .iter()
        .find_map(|c| c.layer(&Id::new("layer-3")))
        .expect("layer-3");
    let lost: Vec<_> = keyed
        .iter()
        .filter(|(fx, name)| {
            comp.effects
                .iter()
                .find(|e| e.instance_id.as_str() == fx)
                .and_then(|e| e.tracks.get(*name))
                .is_none_or(|t| t[0].keyframes().len() != 2)
        })
        .collect();
    assert!(lost.is_empty(), "these keys did not come back: {lost:?}");
    assert!(
        again == written,
        "saving the reopened project gave other text"
    );
    println!(
        "{} effects, {} settings keyed, saved and opened again",
        ids.len(),
        keyed.len()
    );
}

/// Every effect, as it is added, that compose hands the card (`compose::card_takes`) is one the
/// card draws: by its own blur, Bloom and Glow passes, or by `gpu::fx`, which stops the app on
/// any other. Read from the core's own list, so one added later is here too.
#[test]
fn every_effect_the_card_is_handed_is_one_it_draws() {
    use anime_compositor::compose;
    use anime_compositor::effects::Effect as E;
    let ids: Vec<&str> = include_str!("../../../src/effects.rs")
        .lines()
        .filter_map(|l| {
            l.strip_prefix("pub const ")?
                .split_once(": &str = \"")?
                .1
                .strip_suffix("\";")
        })
        .filter(|id| id.starts_with("core.") && new_effect(id).is_some())
        .collect();
    assert!(ids.len() > 90, "the effect list was not found: {ids:?}");
    let mut taken = 0;
    let mut not_drawn = Vec::new();
    for id in &ids {
        let instance = EffectInstance::new(Id::new("fx-1"), new_effect(id).expect("an effect"));
        // Compose reads a Depth of Field's depth from the layer's pass before the card is
        // handed it, as its Lens Blur; one without changes nothing and is left out
        // (`compose::card_effect`). b403 draws one with its depth on the card.
        if matches!(instance.effect, E::DepthOfField { channels: None, .. }) {
            continue;
        }
        // A small drawing, so a Fractal's work does not keep it off the card (D-416).
        if !compose::card_takes(&instance, 64 * 64) {
            continue;
        }
        taken += 1;
        // `compose::card_effect` hands these to the card's own passes, not to `fx`.
        let own = matches!(
            instance.effect,
            E::RadialBlur { .. }
                | E::SpinZoomBlur { .. }
                | E::FastZoomBlur { .. }
                | E::DirectionalBlur { .. }
                | E::GaussianBlur { .. }
                | E::GaussianBlurLegacy { .. }
                | E::Bloom { .. }
                | E::Glow { .. }
        );
        if !own && !anime_compositor::gpu::fx_draws(&instance.effect) {
            not_drawn.push(*id);
        }
    }
    assert!(
        not_drawn.is_empty(),
        "the card is handed these but does not draw them: {not_drawn:?}"
    );
    assert!(
        taken > 100,
        "only {taken} of {} effects went to the card",
        ids.len()
    );
    println!(
        "{} effects, {taken} handed to the card, each one it draws",
        ids.len()
    );
}

/// D-407: Fit to Comp Width and Height work out the Upscale's scale once, as the owner chose
/// (one-time buttons): afterwards the layer is as wide, or as tall, as the composition, the
/// scale is an ordinary number Ctrl+Z takes back, and with keys on Scale the fit is a key.
#[test]
fn fit_to_comp_sets_the_upscale_scale_once() {
    let viewer = Mutex::new(demo());
    // The 1920 by 1080 drawings in a composition 3000 by 2000, so there is something to fit.
    run(&viewer, "composition.set_settings?width=3000&height=2000");
    // The effects after the Upscale are not part of what it is handed; this one is added first.
    run(&viewer, "effect.add?layer=layer-3&type=core.detail_upscale");
    let fx = effects(&viewer)
        .into_iter()
        .find(|e| matches!(e.effect, Effect::DetailUpscale { .. }))
        .expect("an Upscale");
    let id = fx.instance_id.as_str().to_string();
    let scale = |viewer: &Mutex<Viewer>| match effects(viewer)
        .into_iter()
        .find(|e| e.instance_id.as_str() == id)
        .map(|e| e.effect)
    {
        Some(Effect::DetailUpscale { scale, .. }) => scale,
        other => panic!("not an Upscale: {other:?}"),
    };
    // The layer drawn on frame 0 with the Upscale, and the composition's size.
    let drawn = |viewer: &Mutex<Viewer>| {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        let comp = held
            .document
            .project()
            .composition(&held.composition)
            .expect("the composition on screen");
        let plan = anime_compositor::compose::plan_frame_at(
            held.document.project(),
            &held.composition,
            0,
            &held.root,
            PreviewQuality::Full,
            &mut FrameLog::new(0),
            &mut anime_compositor::cache::CelCache::viewer(),
        )
        .expect("plan frame 0");
        let layer = plan
            .layers
            .iter()
            .find(|d| d.id.as_str() == "layer-3")
            .expect("layer-3 drawn");
        (
            (layer.source.width(), layer.source.height()),
            (comp.width as usize, comp.height as usize),
        )
    };
    let (before, (cw, ch)) = drawn(&viewer);
    assert_eq!(scale(&viewer), 100.0);

    let said = run(
        &viewer,
        &format!("effect.fit_scale?layer=layer-3&effect={id}&to=width&frame=0"),
    );
    let fitted = scale(&viewer);
    let ((w, _), _) = drawn(&viewer);
    println!("layer {before:?} in {cw} by {ch}: fit to width said {said:?}, scale {fitted}, layer {w} wide");
    assert!((100.0..=1000.0).contains(&fitted), "{fitted}");
    // The growth is whole pixels on each side, so the width lands on the composition's or up to 2 past it.
    assert!(
        fitted == 100.0 && w >= cw || fitted == 1000.0 && w <= cw + 2 || (cw..=cw + 2).contains(&w),
        "{w} against {cw}"
    );

    run(&viewer, "edit.undo");
    assert_eq!(scale(&viewer), 100.0, "Ctrl+Z takes the fit back");

    run(
        &viewer,
        &format!("effect.fit_scale?layer=layer-3&effect={id}&to=height&frame=0"),
    );
    let fitted = scale(&viewer);
    let ((_, h), _) = drawn(&viewer);
    println!("fit to height: scale {fitted}, layer {h} tall");
    assert!(
        fitted == 100.0 && h >= ch || fitted == 1000.0 && h <= ch + 2 || (ch..=ch + 2).contains(&h),
        "{h} against {ch}"
    );

    // With keys on Scale, the fit is a key on the frame asked for, the others kept.
    run(
        &viewer,
        &format!("keyframe.add_remove?layer=layer-3&prop=fx:{id}:scale&frame=0"),
    );
    run(
        &viewer,
        &format!("effect.fit_scale?layer=layer-3&effect={id}&to=width&frame=12"),
    );
    let keys = effects(&viewer)
        .into_iter()
        .find(|e| e.instance_id.as_str() == id)
        .and_then(|e| e.tracks.get("scale").map(|t| t[0].keyframes().len()));
    assert_eq!(keys, Some(2), "the fit on frame 12 is a second key");

    // Refused in words: another effect, and a direction that is neither.
    run(&viewer, "effect.add?layer=layer-3&type=core.spherize");
    let other = effects(&viewer)
        .into_iter()
        .find(|e| matches!(e.effect, Effect::Spherize { .. }))
        .expect("a Spherize");
    let said = run(
        &viewer,
        &format!(
            "effect.fit_scale?layer=layer-3&effect={}&to=width&frame=0",
            other.instance_id.as_str()
        ),
    );
    assert!(
        said.contains("is not a Detail-preserving Upscale"),
        "{said}"
    );
    let said = run(
        &viewer,
        &format!("effect.fit_scale?layer=layer-3&effect={id}&to=sideways&frame=0"),
    );
    assert!(said.contains("to=width or to=height"), "{said}");
}

/// D-353: Soft Physical Glow is added as Glow's "physical" falloff, takes the settings the
/// panel sends for it, turns back into a classic Glow when the falloff is, and a key on its
/// radius is saved and opened again.
#[test]
fn soft_physical_glow_is_glow_with_the_physical_falloff() {
    let viewer = Mutex::new(demo());
    let said = run(
        &viewer,
        "effect.add?layer=layer-3&type=core.glow&falloff=physical",
    );
    assert!(said.starts_with("Add "), "{said}");
    let fx = effects(&viewer)
        .into_iter()
        .find(|e| matches!(e.effect, Effect::SoftGlow { .. }))
        .expect("a Soft Physical Glow");
    let id = fx.instance_id.as_str().to_string();
    let sent = "&falloff=physical&threshold_mode=luminance&threshold=40&threshold_smooth=50&saturation_bias=-20&radius=60\
                &exposure=1.5&aspect_ratio=1.2&aspect_angle=30&operation=add&source_opacity=80&unmult=off";
    let said = run(
        &viewer,
        &format!("effect.set_parameters?layer=layer-3&effect={id}{sent}"),
    );
    assert!(
        said.starts_with("Set ") || said.starts_with("Change "),
        "{said}"
    );
    let now = effects(&viewer)
        .into_iter()
        .find(|e| e.instance_id.as_str() == id)
        .expect("the glow");
    assert!(
        matches!(&now.effect, Effect::SoftGlow { radius, unmult, .. } if *radius == 60.0 && unmult == "off"),
        "{:?}",
        now.effect
    );
    run(
        &viewer,
        &format!("keyframe.add_remove?layer=layer-3&prop=fx:{id}:radius&frame=0"),
    );
    run(
        &viewer,
        &format!("property.set_base?layer=layer-3&prop=fx:{id}:radius&frame=10&value=200"),
    );
    let written = {
        let held = viewer.lock().expect("the viewer lock was poisoned");
        persist::to_json(held.document.project(), &held.preserved)
    };
    let reopened =
        persist::load_str(&written).unwrap_or_else(|d| panic!("{}: {}", d.message, d.detail));
    let comp = reopened
        .document
        .project()
        .compositions
        .iter()
        .find_map(|c| c.layer(&Id::new("layer-3")))
        .expect("layer-3");
    let back = comp
        .effects
        .iter()
        .find(|e| e.instance_id.as_str() == id)
        .expect("the glow came back");
    assert!(
        matches!(back.effect, Effect::SoftGlow { .. }),
        "{:?}",
        back.effect
    );
    assert_eq!(
        back.tracks.get("radius").map(|t| t[0].keyframes().len()),
        Some(2)
    );
    let classic = "&falloff=classic&based_on=bright&threshold=60&colors=&tolerance=0&radius=10&intensity=1&operation=add&tint=";
    run(
        &viewer,
        &format!("effect.set_parameters?layer=layer-3&effect={id}{classic}"),
    );
    let now = effects(&viewer)
        .into_iter()
        .find(|e| e.instance_id.as_str() == id)
        .expect("the glow");
    assert!(
        matches!(now.effect, Effect::Glow { .. }),
        "{:?}",
        now.effect
    );
}
