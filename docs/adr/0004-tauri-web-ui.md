# ADR-004: Tauri with an HTML and CSS interface

Status: ACCEPTED
Date: 2026-09-04
Deciders: Andrew (owner)

## Context

The interface needs a technology. The natural pairing with a Rust core is a native immediate-mode toolkit such as egui, which was the initial version 0.3 recommendation.

The owner then stated two requirements that recommendation did not satisfy: the interface should be adaptable through Claude Design, and Claude should be able to visually investigate the running application in order to verify rendering behavior.

## Decision

Tauri, with the interface written in HTML and CSS rendered in WebView2, and the Rust core behind it. The web layer holds no authoritative state; it renders what the core reports and sends user intent back as commands.

## Rationale

Design work becomes the product rather than a picture of it. Claude Design emits HTML and CSS, which is directly usable in a Tauri frontend. With a native toolkit, every design would be hand-translated, the design artifact would remain a mockup, and design effort would be spent twice and then drift away from the build.

The interface becomes machine-inspectable. A running web interface can be opened in a browser context, screenshotted, its console read and its elements inspected. Under ADR-013 the owner cannot investigate a visual defect by reading code, so an agent that can actually look at the screen is not a convenience but part of the verification story.

Dense dockable panel layouts, which a timeline, inspector and media bin require, are ordinary work in CSS and awkward in immediate-mode toolkits.

## Consequences and accepted risks

WebView2 is a Windows system component updated by Microsoft outside the control of this project. Recorded as K-08, mitigated by re-running viewer fixtures against each release candidate.

Frame transport across the Rust to webview boundary constrains full-resolution playback. Roughly 8 MB per 1080p RGBA frame makes naive per-frame serialization too slow. SP-05 measures achievable rates and selects the transport. Frame stepping and draft-scale scrubbing are expected to be comfortable regardless.

Browser color management may alter displayed pixels. SP-06 verifies byte-exactness of the display path against document 25. This matters more here than in most applications, because predictable pixels is a charter promise and a preview that quietly shifts color would undermine it.

Both the performance and the color risks are preview-side only. Export never passes through the display path, so exported output is unaffected by either.

Input latency is marginally higher than native. Accepted.

OS integration such as Explorer drag-and-drop, native file dialogs and taskbar progress is per-feature work rather than free. Accepted as a small recurring cost.

Not a real cost, though it looks like one: the application will not follow native Windows 11 styling. No compositor does. Every tool in this category paints a custom dark theme, and one is wanted here anyway.

## Fallback

If SP-05 or SP-06 fails badly, host a native rendering surface for the viewer inside the web shell, keeping HTML and CSS for the interface around it. This preserves both the design transferability and the inspection benefits while removing the transport and color risks.

Do not pre-engineer this. Measure first.
