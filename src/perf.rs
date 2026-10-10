//! P-01: per-stage timers, so that "where does the frame go" is a table and not an opinion.
//!
//! `verification/T-06_declared_fixture.md` measures 264.17 ms a frame against the 41.667 ms a
//! 24 fps clock allows, and `Markdown/32_Performance_Architecture_Investigation.md` section 1.2
//! could not account for about half of that. Neither research document knows where the time
//! goes because nobody had measured it. This module is the measurement, and it is deliberately
//! the smallest thing that can produce one: an atomic nanosecond counter and a call count per
//! named stage, a switch that is off, and nothing else.
//!
//! Three properties are structural rather than remembered.
//!
//! **Off by default.** [`enable`] is called by the harness that writes P-01's artifact and by
//! nothing else. While it is off, [`time`] is one relaxed atomic load and a call, so no shipped
//! render pays for the timers it is not using. That is ADR-012's rule for trace mode applied to
//! its stopwatch: a diagnostic is never on in a build the owner is looking at unless asked for.
//!
//! **The stages are disjoint.** No stage below is inside another, so a table of them can be
//! summed and subtracted from a frame time to leave a residual. A nested pair would double-count
//! and the residual would go negative, which is a bug that reads as a result. `Decode` is
//! therefore split at its real seams — the read, the byte-to-float pass, and the transfer
//! function with the premultiply — rather than wrapped whole.
//!
//! **It measures, it does not judge.** There is no budget here, no threshold, no assertion. The
//! numbers go into `verification/P-01_frame_trace.md` and the owner reads them.
//!
//! `TileLoop` is wall-clock across the rayon fan-out rather than the sum of the threads' own
//! time, because the frame's cost is when the last tile finishes and not how many core-seconds
//! were spent. Every other stage runs on the calling thread.

use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::time::Instant;

/// A stage of one preview frame that this build spends measurable time in.
///
/// The order is the order they run in, which is the order the artifact prints them, so that a
/// reader walks the frame rather than a sorted list.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Stage {
    /// The wait for the viewer mutex in the window's `serve`, before any work begins. Zero in a
    /// headless harness, which has no second thread to wait for; P-04 is where it is measured.
    LockWait,
    /// The parallel decode of every cel this frame is about to ask for, wall-clock from
    /// fan-out to join (P-03(b)). The three stages below are what it does; while a cel is being
    /// decoded inside this fan-out they record nothing, so that this stage and they stay
    /// disjoint and the table can still be summed. A cel decoded outside it — by an export, by
    /// a caller with no cache, or by a request this could not see coming — reports in them as
    /// before and nothing in this one.
    Prewarm,
    /// Opening a cel's file and reading the compressed bytes back out as 8-bit RGBA.
    FileRead,
    /// Those bytes divided by 255 into f32, which is `ImageBuffer::from_srgb8_straight`.
    Dequantise,
    /// The sRGB transfer function and the premultiply, which is `ImageBuffer::into_working`.
    ToLinear,
    /// A cache hit: the scan for the key, and the copy of the buffer handed back.
    CacheHit,
    /// A cache miss admitting the decoded cel: the second copy, and any eviction it forces.
    CacheStore,
    /// The polygon mask rasterised into the layer's own pixels.
    Mask,
    /// The copy `Arc::make_mut` takes so the effect stack has a buffer of its own to write in.
    /// Separate from the effects themselves because it is paid once a layer however long the
    /// stack is, and P-11 is ranking the effects against each other.
    EffectCopy,
    /// The exposure effect, whole-layer, per ADR-017.
    EffectExposure,
    /// The tint effect, whole-layer, per ADR-017.
    EffectTint,
    /// The gaussian blur, whole-layer, per ADR-017. Split out from the other two by P-11, whose
    /// entry says the first thing it does is find out whether the blur really is the expensive
    /// one rather than assuming it from what the three effects do.
    EffectBlur,
    /// D-86's line smoothing, whole-layer, per ADR-017.
    EffectSmooth,
    /// D-87's selective colour blur, whole-layer, per ADR-017.
    EffectSelBlur,
    /// D-89's glow, whole-layer, per ADR-017.
    EffectGlow,
    /// D-91's line recolour, whole-layer, per ADR-017.
    EffectRecolor,
    /// D-92's directional blur, whole-layer, per ADR-017.
    EffectDirBlur,
    /// D-93's select colour, whole-layer, per ADR-017.
    EffectSelect,
    /// D-94's line width, whole-layer, per ADR-017.
    EffectWidth,
    /// D-95's radial blur, whole-layer, per ADR-017.
    EffectRadial,
    /// D-96's bloom, whole-layer, per ADR-017.
    EffectBloom,
    /// D-97's colour key, whole-layer, per ADR-017.
    EffectColorKey,
    /// D-111's curves, whole-layer, per ADR-017.
    EffectCurves,
    /// D-112's levels, whole-layer, per ADR-017.
    EffectLevels,
    /// D-113's hue and saturation, whole-layer, per ADR-017.
    EffectHueSaturation,
    /// D-114's gradient, whole-layer, per ADR-017.
    EffectGradient,
    /// D-115's drop shadow, whole-layer, per ADR-017.
    EffectDropShadow,
    /// D-116's lens blur, whole-layer, per ADR-017.
    EffectLensBlur,
    /// D-117's rim light, whole-layer, per ADR-017.
    EffectRimLight,
    /// D-118's outline, whole-layer, per ADR-017.
    EffectOutline,
    /// D-119's noise, whole-layer, per ADR-017.
    EffectNoise,
    /// D-120's chromatic aberration, whole-layer, per ADR-017.
    EffectChromaticAberration,
    /// D-123's distance gradation, whole-layer, per ADR-017.
    EffectDistanceGradation,
    /// D-124's light rays, whole-layer, per ADR-017.
    EffectLightRays,
    /// D-125's exposure flicker, per pixel.
    EffectExposureFlicker,
    /// D-126's vignette, per pixel.
    EffectVignette,
    /// D-127's turbulent displace, per pixel.
    EffectTurbulentDisplace,
    /// D-128's fractal noise, per pixel.
    EffectFractalNoise,
    /// D-129's gradient map, per pixel.
    EffectGradientMap,
    /// D-130's colour balance, per pixel.
    EffectColorBalance,
    /// D-131's offset, the buffer slid round.
    EffectOffset,
    /// D-132's light wrap, as the layer is drawn onto the frame.
    EffectLightWrap,
    /// D-134's invert, per pixel.
    EffectInvert,
    /// D-135's brightness and contrast, per pixel.
    EffectBrightnessContrast,
    /// D-136's black and white, per pixel.
    EffectBlackWhite,
    /// D-137's posterize, per pixel.
    EffectPosterize,
    /// D-138's threshold, per pixel.
    EffectThreshold,
    /// D-139's channel mixer, per pixel.
    EffectChannelMixer,
    /// D-140's vibrance, per pixel.
    EffectVibrance,
    /// D-141's leave colour, per pixel.
    EffectLeaveColor,
    /// D-142's solarize, per pixel.
    EffectSolarize,
    /// D-143's halftone, per pixel.
    EffectHalftone,
    /// D-144's mosaic, per block.
    EffectMosaic,
    /// D-145's emboss, per pixel.
    EffectEmboss,
    /// D-146's find edges, per pixel.
    EffectFindEdges,
    /// D-147's sharpen, a blur and a pass per pixel.
    EffectSharpen,
    /// D-148's diffusion, a blur and a pass per pixel.
    EffectDiffusion,
    /// D-149's wave warp, per pixel.
    EffectWaveWarp,
    /// D-150's ripple, per pixel.
    EffectRipple,
    /// D-151's twirl, per pixel.
    EffectTwirl,
    /// D-152's bulge, per pixel.
    EffectBulge,
    /// D-153's mirror, per pixel.
    EffectMirror,
    /// D-154's motion tile, per pixel.
    EffectMotionTile,
    /// D-155's linear wipe, per pixel.
    EffectLinearWipe,
    /// D-156's radial wipe, per pixel.
    EffectRadialWipe,
    /// D-157's venetian blinds, per pixel.
    EffectVenetianBlinds,
    /// D-158's iris wipe, per pixel.
    EffectIrisWipe,
    /// D-159's simple choker, a least or greatest covering over a disc.
    EffectSimpleChoker,
    /// D-160's speed lines, the nearby lines per pixel.
    EffectSpeedLines,
    /// D-161's cross glare, every arm's steps per pixel.
    EffectCrossGlare,
    /// D-162's camera shake, one turned sample per pixel.
    EffectCameraShake,
    /// D-163's rain, the nearby cells per pixel.
    EffectRain,
    /// D-182's colour lookup, eight table lines mixed per pixel.
    EffectColorLookup,
    /// D-183's line blur, a smoothed tensor and a few taps along the line per pixel.
    EffectLineBlur,
    /// D-184's HSV key, one colour conversion per pixel.
    EffectHsvKey,
    /// D-185's paraffin, one pass for the figure's extent and one to wash.
    EffectParaffin,
    /// D-186's kira-kira, one pass for the stars and one row by row to light them.
    EffectKiraKira,
    /// D-190's lightning bolt, its path and one pass row by row to light it.
    EffectLightningBolt,
    /// D-191's compound blur, its map read and up to five Gaussians of the picture mixed.
    EffectCompoundBlur,
    /// D-193's displacement map, its map read and one bilinear read a pixel.
    EffectDisplacementMap,
    /// D-194's gradient wipe, its map read and one luma a pixel.
    EffectGradientWipe,
    /// D-403's aerial haze, a mix toward a colour a pixel, through a matte when one is read.
    EffectAerialHaze,
    /// D-408's transform, one or more samples a pixel through an affine map.
    EffectTransform,
    /// D-195's echo, its copies put together and laid on the layer; drawing them is timed as
    /// any drawing is.
    EffectEcho,
    /// D-197's change to color, per pixel.
    EffectChangeToColor,
    /// D-198's corner pin, per pixel of the grown layer.
    EffectCornerPin,
    EffectLightSweep,
    EffectRadioWaves,
    EffectPolarCoordinates,
    EffectMedian,
    EffectSmartBlur,
    EffectBilateralBlur,
    EffectSnowfall,
    EffectKaleidoscope,
    EffectRoughenEdges,
    EffectBeam,
    EffectFourColorGradient,
    EffectCellPattern,
    EffectOpticsCompensation,
    EffectRadialShadow,
    EffectExtract,
    EffectBevelAlpha,
    EffectBevelEdges,
    EffectBlockDissolve,
    EffectShiftChannels,
    EffectSolidComposite,
    EffectChannelBlur,
    EffectFastBoxBlur,
    EffectColorama,
    EffectGlass,
    EffectVectorBlur,
    EffectBendIt,
    EffectBender,
    EffectBlobbylize,
    EffectFlowMotion,
    EffectGriddler,
    EffectFisheye,
    /// D-388..D-390: Page Turn, Power Pin and Ripple Pulse.
    EffectPageTurn,
    EffectPowerPin,
    EffectRipplePulse,
    /// D-391..D-393: Slant, Smear and Split.
    EffectSlant,
    EffectSmear,
    EffectSplit,
    /// D-395: Arbitrary Map.
    EffectArbitraryMap,
    /// D-396: Selective Color.
    EffectSelectiveColor,
    /// D-400: Shadow/Highlight.
    EffectShadowHighlight,
    EffectMomentMap,
    /// D-348's pass extract and depth key, per pixel; reading the pass from its file is timed
    /// as any file read is.
    EffectPassExtract,
    EffectDepthKey,
    /// D-349's id key: the matte, its feather and the multiply.
    EffectIdKey,
    /// D-351's stretches and spread: the picture's statistics and the pass over it. A temporal
    /// smoothing's other frames are timed as any layer resolved is.
    EffectAutoTone,
    EffectSpreadTones,
    /// D-352's Matte Choker, and Refine Hard Matte and Refine Soft Matte: the guided filters, the
    /// box sums and the passes over the layer.
    EffectMatteChoker,
    EffectRefineMatte,
    /// D-353's Soft Physical Glow: the light, the levels' cells and passes, and the finish.
    EffectSoftGlow,
    /// D-356's Path Stroke: the runs along the paths and the brush laid on them.
    EffectStroke,
    /// D-360..D-362's Cross Blur, Spin & Zoom Blur and Fast Zoom Blur, per pixel.
    EffectCrossBlur,
    EffectSpinZoomBlur,
    EffectFastZoomBlur,
    /// D-365..D-367's Broadcast Safe, Color Neutralizer and Color Offset, per pixel.
    EffectBroadcastSafe,
    EffectColorNeutralizer,
    EffectColorOffset,
    /// D-368..D-370's Kernel, Toner and Change Color.
    EffectKernel,
    EffectToner,
    EffectChangeColor,
    /// D-374..D-376's Color Balance (HLS), Color Link and Color Stabilizer.
    EffectColorBalanceHls,
    EffectColorLink,
    EffectColorStabilizer,
    /// D-382's Gamma/Pedestal/Gain.
    EffectGammaPedestalGain,
    /// D-383's Levels with a set for each channel, and Levels (Individual Controls).
    EffectChannelLevels,
    /// D-384's Photo Filter.
    EffectPhotoFilter,
    /// D-397's Color Grade.
    EffectColorGrade,
    /// D-404's Tiles.
    EffectTiles,
    /// D-405's Magnify.
    EffectMagnify,
    /// D-406's Spherize.
    EffectSpherize,
    /// D-413's Checkerboard.
    EffectCheckerboard,
    /// D-414's Circle.
    EffectCircle,
    /// D-415's Ellipse.
    EffectEllipse,
    /// The lookup and admission of an evaluated effect result (P-11).
    EffectCache,
    /// The tiled sample-and-blend fan-out, wall-clock from fan-out to join.
    TileLoop,
    /// Copying the finished tiles into the frame buffer.
    FrameAssembly,
    /// `WorkingBuffer::to_srgb8_straight`: unpremultiply, encode, quantise, for the page.
    Encode,
    /// B-44: sending drawings the card does not hold yet.
    GpuUpload,
    /// B-44: the card drawing and encoding the frame, and the eight-bit picture coming back.
    GpuDraw,
    /// B-45: the card painting the picture into the window, and handing it to Windows.
    GpuShow,
}

impl Stage {
    pub const ALL: [Stage; 165] = [
        Stage::LockWait,
        Stage::Prewarm,
        Stage::FileRead,
        Stage::Dequantise,
        Stage::ToLinear,
        Stage::CacheHit,
        Stage::CacheStore,
        Stage::Mask,
        Stage::EffectCopy,
        Stage::EffectExposure,
        Stage::EffectTint,
        Stage::EffectBlur,
        Stage::EffectSmooth,
        Stage::EffectSelBlur,
        Stage::EffectGlow,
        Stage::EffectRecolor,
        Stage::EffectDirBlur,
        Stage::EffectSelect,
        Stage::EffectWidth,
        Stage::EffectRadial,
        Stage::EffectBloom,
        Stage::EffectColorKey,
        Stage::EffectCurves,
        Stage::EffectLevels,
        Stage::EffectHueSaturation,
        Stage::EffectGradient,
        Stage::EffectDropShadow,
        Stage::EffectLensBlur,
        Stage::EffectRimLight,
        Stage::EffectOutline,
        Stage::EffectNoise,
        Stage::EffectChromaticAberration,
        Stage::EffectDistanceGradation,
        Stage::EffectLightRays,
        Stage::EffectExposureFlicker,
        Stage::EffectVignette,
        Stage::EffectTurbulentDisplace,
        Stage::EffectFractalNoise,
        Stage::EffectGradientMap,
        Stage::EffectColorBalance,
        Stage::EffectOffset,
        Stage::EffectLightWrap,
        Stage::EffectInvert,
        Stage::EffectBrightnessContrast,
        Stage::EffectBlackWhite,
        Stage::EffectPosterize,
        Stage::EffectThreshold,
        Stage::EffectChannelMixer,
        Stage::EffectVibrance,
        Stage::EffectLeaveColor,
        Stage::EffectSolarize,
        Stage::EffectHalftone,
        Stage::EffectMosaic,
        Stage::EffectEmboss,
        Stage::EffectFindEdges,
        Stage::EffectSharpen,
        Stage::EffectDiffusion,
        Stage::EffectWaveWarp,
        Stage::EffectRipple,
        Stage::EffectTwirl,
        Stage::EffectBulge,
        Stage::EffectMirror,
        Stage::EffectMotionTile,
        Stage::EffectLinearWipe,
        Stage::EffectRadialWipe,
        Stage::EffectVenetianBlinds,
        Stage::EffectIrisWipe,
        Stage::EffectSimpleChoker,
        Stage::EffectSpeedLines,
        Stage::EffectCrossGlare,
        Stage::EffectCameraShake,
        Stage::EffectRain,
        Stage::EffectColorLookup,
        Stage::EffectLineBlur,
        Stage::EffectHsvKey,
        Stage::EffectParaffin,
        Stage::EffectKiraKira,
        Stage::EffectLightningBolt,
        Stage::EffectCompoundBlur,
        Stage::EffectDisplacementMap,
        Stage::EffectGradientWipe,
        Stage::EffectAerialHaze,
        Stage::EffectTransform,
        Stage::EffectEcho,
        Stage::EffectChangeToColor,
        Stage::EffectCornerPin,
        Stage::EffectLightSweep,
        Stage::EffectRadioWaves,
        Stage::EffectPolarCoordinates,
        Stage::EffectMedian,
        Stage::EffectSmartBlur,
        Stage::EffectBilateralBlur,
        Stage::EffectSnowfall,
        Stage::EffectKaleidoscope,
        Stage::EffectRoughenEdges,
        Stage::EffectBeam,
        Stage::EffectFourColorGradient,
        Stage::EffectCellPattern,
        Stage::EffectOpticsCompensation,
        Stage::EffectRadialShadow,
        Stage::EffectExtract,
        Stage::EffectBevelAlpha,
        Stage::EffectBevelEdges,
        Stage::EffectBlockDissolve,
        Stage::EffectShiftChannels,
        Stage::EffectSolidComposite,
        Stage::EffectChannelBlur,
        Stage::EffectFastBoxBlur,
        Stage::EffectColorama,
        Stage::EffectGlass,
        Stage::EffectVectorBlur,
        Stage::EffectBendIt,
        Stage::EffectBender,
        Stage::EffectBlobbylize,
        Stage::EffectFlowMotion,
        Stage::EffectGriddler,
        Stage::EffectFisheye,
        Stage::EffectPageTurn,
        Stage::EffectPowerPin,
        Stage::EffectRipplePulse,
        Stage::EffectSlant,
        Stage::EffectSmear,
        Stage::EffectSplit,
        Stage::EffectArbitraryMap,
        Stage::EffectSelectiveColor,
        Stage::EffectShadowHighlight,
        Stage::EffectMomentMap,
        Stage::EffectPassExtract,
        Stage::EffectDepthKey,
        Stage::EffectIdKey,
        Stage::EffectAutoTone,
        Stage::EffectSpreadTones,
        Stage::EffectMatteChoker,
        Stage::EffectRefineMatte,
        Stage::EffectSoftGlow,
        Stage::EffectStroke,
        Stage::EffectCrossBlur,
        Stage::EffectSpinZoomBlur,
        Stage::EffectFastZoomBlur,
        Stage::EffectBroadcastSafe,
        Stage::EffectColorNeutralizer,
        Stage::EffectColorOffset,
        Stage::EffectKernel,
        Stage::EffectToner,
        Stage::EffectChangeColor,
        Stage::EffectColorBalanceHls,
        Stage::EffectColorLink,
        Stage::EffectColorStabilizer,
        Stage::EffectGammaPedestalGain,
        Stage::EffectChannelLevels,
        Stage::EffectPhotoFilter,
        Stage::EffectColorGrade,
        Stage::EffectTiles,
        Stage::EffectMagnify,
        Stage::EffectSpherize,
        Stage::EffectCheckerboard,
        Stage::EffectCircle,
        Stage::EffectEllipse,
        Stage::EffectCache,
        Stage::TileLoop,
        Stage::FrameAssembly,
        Stage::Encode,
        Stage::GpuUpload,
        Stage::GpuDraw,
        Stage::GpuShow,
    ];

    /// The name the artifact prints. Written for the owner, not for a log parser.
    pub fn label(self) -> &'static str {
        match self {
            Stage::LockWait => "wait for the viewer lock",
            Stage::Prewarm => "decode this frame's cels in parallel",
            Stage::FileRead => "open and read the cel file",
            Stage::Dequantise => "bytes to float",
            Stage::ToLinear => "transfer function and premultiply",
            Stage::CacheHit => "cache lookup and its copy",
            Stage::CacheStore => "cache admit and its copy",
            Stage::Mask => "layer mask",
            Stage::EffectCopy => "effect stack: the copy it writes into",
            Stage::EffectExposure => "effect: exposure",
            Stage::EffectTint => "effect: tint",
            Stage::EffectBlur => "effect: gaussian blur",
            Stage::EffectSmooth => "effect: line smoothing",
            Stage::EffectSelBlur => "effect: selective colour blur",
            Stage::EffectGlow => "effect: glow",
            Stage::EffectRecolor => "effect: line recolour",
            Stage::EffectDirBlur => "effect: directional blur",
            Stage::EffectSelect => "effect: select colour",
            Stage::EffectWidth => "effect: line width",
            Stage::EffectRadial => "effect: radial blur",
            Stage::EffectBloom => "effect: bloom",
            Stage::EffectColorKey => "effect: colour key",
            Stage::EffectCurves => "effect: curves",
            Stage::EffectLevels => "effect: levels",
            Stage::EffectHueSaturation => "effect: hue/saturation",
            Stage::EffectGradient => "effect: gradient",
            Stage::EffectDropShadow => "effect: drop shadow",
            Stage::EffectLensBlur => "effect: lens blur",
            Stage::EffectRimLight => "effect: rim light",
            Stage::EffectOutline => "effect: outline",
            Stage::EffectNoise => "effect: noise",
            Stage::EffectChromaticAberration => "effect: chromatic aberration",
            Stage::EffectDistanceGradation => "effect: distance gradation",
            Stage::EffectLightRays => "effect: light rays",
            Stage::EffectExposureFlicker => "effect: exposure flicker",
            Stage::EffectVignette => "effect: vignette",
            Stage::EffectTurbulentDisplace => "effect: turbulent displace",
            Stage::EffectFractalNoise => "effect: fractal noise",
            Stage::EffectGradientMap => "effect: gradient map",
            Stage::EffectColorBalance => "effect: color balance",
            Stage::EffectOffset => "effect: offset",
            Stage::EffectLightWrap => "effect: light wrap",
            Stage::EffectInvert => "effect: invert",
            Stage::EffectBrightnessContrast => "effect: brightness & contrast",
            Stage::EffectBlackWhite => "effect: black & white",
            Stage::EffectPosterize => "effect: posterize",
            Stage::EffectThreshold => "effect: threshold",
            Stage::EffectChannelMixer => "effect: channel mixer",
            Stage::EffectVibrance => "effect: vibrance",
            Stage::EffectLeaveColor => "effect: leave color",
            Stage::EffectSolarize => "effect: solarize",
            Stage::EffectHalftone => "effect: halftone",
            Stage::EffectMosaic => "effect: mosaic",
            Stage::EffectEmboss => "effect: emboss",
            Stage::EffectFindEdges => "effect: find edges",
            Stage::EffectSharpen => "effect: sharpen",
            Stage::EffectDiffusion => "effect: diffusion",
            Stage::EffectWaveWarp => "effect: wave warp",
            Stage::EffectRipple => "effect: ripple",
            Stage::EffectTwirl => "effect: twirl",
            Stage::EffectBulge => "effect: bulge",
            Stage::EffectMirror => "effect: mirror",
            Stage::EffectMotionTile => "effect: motion tile",
            Stage::EffectLinearWipe => "effect: linear wipe",
            Stage::EffectRadialWipe => "effect: radial wipe",
            Stage::EffectVenetianBlinds => "effect: venetian blinds",
            Stage::EffectIrisWipe => "effect: iris wipe",
            Stage::EffectSimpleChoker => "effect: simple choker",
            Stage::EffectSpeedLines => "effect: speed lines",
            Stage::EffectCrossGlare => "effect: cross glare",
            Stage::EffectCameraShake => "effect: camera shake",
            Stage::EffectRain => "effect: rain",
            Stage::EffectColorLookup => "effect: color lookup",
            Stage::EffectLineBlur => "effect: line blur",
            Stage::EffectHsvKey => "effect: hsv key",
            Stage::EffectParaffin => "effect: paraffin",
            Stage::EffectKiraKira => "effect: kira-kira",
            Stage::EffectLightningBolt => "effect: lightning bolt",
            Stage::EffectCompoundBlur => "effect: compound blur",
            Stage::EffectDisplacementMap => "effect: displacement map",
            Stage::EffectGradientWipe => "effect: gradient wipe",
            Stage::EffectAerialHaze => "effect: aerial haze",
            Stage::EffectTransform => "effect: transform",
            Stage::EffectEcho => "effect: echo",
            Stage::EffectChangeToColor => "effect: change to color",
            Stage::EffectCornerPin => "effect: corner pin",
            Stage::EffectLightSweep => "effect: light sweep",
            Stage::EffectRadioWaves => "effect: radio waves",
            Stage::EffectPolarCoordinates => "effect: polar coordinates",
            Stage::EffectMedian => "effect: median",
            Stage::EffectSmartBlur => "effect: smart blur",
            Stage::EffectBilateralBlur => "effect: bilateral blur",
            Stage::EffectSnowfall => "effect: snowfall",
            Stage::EffectKaleidoscope => "effect: kaleidoscope",
            Stage::EffectRoughenEdges => "effect: roughen edges",
            Stage::EffectBeam => "effect: beam",
            Stage::EffectFourColorGradient => "effect: 4-color gradient",
            Stage::EffectCellPattern => "effect: cell pattern",
            Stage::EffectOpticsCompensation => "effect: optics compensation",
            Stage::EffectRadialShadow => "effect: radial shadow",
            Stage::EffectExtract => "effect: extract",
            Stage::EffectBevelAlpha => "effect: bevel alpha",
            Stage::EffectBevelEdges => "effect: bevel edges",
            Stage::EffectBlockDissolve => "effect: block dissolve",
            Stage::EffectShiftChannels => "effect: shift channels",
            Stage::EffectSolidComposite => "effect: solid composite",
            Stage::EffectChannelBlur => "effect: channel blur",
            Stage::EffectFastBoxBlur => "effect: fast box blur",
            Stage::EffectColorama => "effect: colorama",
            Stage::EffectGlass => "effect: cc glass",
            Stage::EffectVectorBlur => "effect: cc vector blur",
            Stage::EffectBendIt => "effect: bend it",
            Stage::EffectBender => "effect: bender",
            Stage::EffectBlobbylize => "effect: blobbylize",
            Stage::EffectFlowMotion => "effect: flow motion",
            Stage::EffectGriddler => "effect: griddler",
            Stage::EffectFisheye => "effect: fisheye",
            Stage::EffectPageTurn => "effect: page turn",
            Stage::EffectPowerPin => "effect: power pin",
            Stage::EffectRipplePulse => "effect: ripple pulse",
            Stage::EffectSlant => "effect: slant",
            Stage::EffectSmear => "effect: smear",
            Stage::EffectSplit => "effect: split",
            Stage::EffectArbitraryMap => "effect: arbitrary map",
            Stage::EffectSelectiveColor => "effect: selective color",
            Stage::EffectShadowHighlight => "effect: shadow/highlight",
            Stage::EffectMomentMap => "effect: moment map",
            Stage::EffectPassExtract => "effect: pass extract",
            Stage::EffectDepthKey => "effect: depth key",
            Stage::EffectIdKey => "effect: id key",
            Stage::EffectAutoTone => "effect: stretch levels, contrast or color",
            Stage::EffectSpreadTones => "effect: spread tones",
            Stage::EffectMatteChoker => "effect: matte choker",
            Stage::EffectRefineMatte => "effect: refine hard or soft matte",
            Stage::EffectSoftGlow => "effect: soft physical glow",
            Stage::EffectStroke => "effect: path stroke",
            Stage::EffectCrossBlur => "effect: cross blur",
            Stage::EffectSpinZoomBlur => "effect: spin & zoom blur",
            Stage::EffectFastZoomBlur => "effect: fast zoom blur",
            Stage::EffectBroadcastSafe => "effect: broadcast safe",
            Stage::EffectColorNeutralizer => "effect: color neutralizer",
            Stage::EffectColorOffset => "effect: color offset",
            Stage::EffectKernel => "effect: kernel",
            Stage::EffectToner => "effect: toner",
            Stage::EffectChangeColor => "effect: change color",
            Stage::EffectColorBalanceHls => "effect: color balance (hls)",
            Stage::EffectColorLink => "effect: color link",
            Stage::EffectColorStabilizer => "effect: color stabilizer",
            Stage::EffectGammaPedestalGain => "effect: gamma/pedestal/gain",
            Stage::EffectChannelLevels => "effect: levels, a set for each channel",
            Stage::EffectPhotoFilter => "effect: photo filter",
            Stage::EffectColorGrade => "effect: color grade",
            Stage::EffectTiles => "effect: tiles",
            Stage::EffectMagnify => "effect: magnify",
            Stage::EffectSpherize => "effect: spherize",
            Stage::EffectCheckerboard => "effect: checkerboard",
            Stage::EffectCircle => "effect: circle",
            Stage::EffectEllipse => "effect: ellipse",
            Stage::EffectCache => "effect result cache: lookup and admit",
            Stage::TileLoop => "tile loop: sample and blend",
            Stage::FrameAssembly => "assemble the frame from the tiles",
            Stage::Encode => "encode for the page",
            Stage::GpuUpload => "GPU: send drawings to the card",
            Stage::GpuDraw => "GPU: draw, encode and bring the picture back",
            Stage::GpuShow => "GPU: paint the picture into the window",
        }
    }

    /// Its place in [`Stage::ALL`], which lists the stages in the order they are declared.
    fn index(self) -> usize {
        self as usize
    }
}

const N: usize = Stage::ALL.len();

static ON: AtomicBool = AtomicBool::new(false);
#[allow(clippy::declare_interior_mutable_const)]
const ZERO: AtomicU64 = AtomicU64::new(0);
static NANOS: [AtomicU64; N] = [ZERO; N];
static CALLS: [AtomicU64; N] = [ZERO; N];

thread_local! {
    /// Set only by [`untimed`]. `Cell<bool>` rather than an atomic because it is per thread by
    /// definition: one worker suppressing its own timers must not suppress another's.
    static SUPPRESSED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
    /// P-19: this thread's own share of the counters since [`begin_capture`], if one is running.
    static CAPTURE: std::cell::RefCell<Option<[u64; N]>> = const { std::cell::RefCell::new(None) };
}

/// Start recording. The P-01 harness calls this, and so does P-19's session log in the window
/// while its switch is on, and nothing else.
pub fn enable() {
    ON.store(true, Ordering::Relaxed);
}

/// Stop recording. Counters keep whatever they hold; [`reset`] is what clears them.
pub fn disable() {
    ON.store(false, Ordering::Relaxed);
}

pub fn is_enabled() -> bool {
    ON.load(Ordering::Relaxed)
}

/// Zero every counter. Called between measured frames, so a row is one frame and not a run.
pub fn reset() {
    for i in 0..N {
        NANOS[i].store(0, Ordering::Relaxed);
        CALLS[i].store(0, Ordering::Relaxed);
    }
}

/// Run `f`, and if recording is on, add how long it took to `stage`.
///
/// The `Instant::now()` pair costs tens of nanoseconds and every stage it wraps is measured in
/// microseconds or milliseconds, so the timer is inside the noise of the thing it times. When
/// recording is off it is not taken at all.
pub fn time<T>(stage: Stage, f: impl FnOnce() -> T) -> T {
    if !is_enabled() || SUPPRESSED.with(|s| s.get()) {
        return f();
    }
    let at = Instant::now();
    let out = f();
    record(stage, at.elapsed().as_nanos() as u64);
    out
}

/// Run `f` with every [`time`] call on this thread recording nothing (P-03(b)).
///
/// There is exactly one caller: the parallel decode in [`crate::cache::CelCache::prewarm`], which
/// wraps each cel it decodes. Without this, the read, the byte-to-float pass and the transfer
/// function would each add their *core* time to a stage while the frame paid only the wall-clock
/// of the widest thread, the stages would stop being disjoint, and the residual this module
/// promises would go negative — "a bug that reads as a result", in the words at the top of this
/// file. [`Stage::Prewarm`] is what those cels report as instead, once, around the whole fan-out.
///
/// It is a thread-local and set inside the fan-out rather than around it, because a rayon closure
/// runs on whichever worker steals it and the calling thread steals work too.
pub fn untimed<T>(f: impl FnOnce() -> T) -> T {
    if !is_enabled() {
        return f();
    }
    SUPPRESSED.with(|s| s.set(true));
    let out = f();
    SUPPRESSED.with(|s| s.set(false));
    out
}

/// Add an already-measured duration to a stage, for a caller that cannot wrap a closure around
/// it — a lock acquisition whose guard has to outlive the timing.
pub fn record(stage: Stage, nanos: u64) {
    if !is_enabled() || SUPPRESSED.with(|s| s.get()) {
        return;
    }
    NANOS[stage.index()].fetch_add(nanos, Ordering::Relaxed);
    CALLS[stage.index()].fetch_add(1, Ordering::Relaxed);
    CAPTURE.with(|c| {
        if let Some(own) = c.borrow_mut().as_mut() {
            own[stage.index()] += nanos;
        }
    });
}

/// P-19: start collecting, on this thread only, what [`record`] adds from here on.
///
/// The counters above are shared by every thread, so while the window renders a frame they also
/// take in whatever a read-ahead or an export is doing at the same moment, and one frame's row
/// would be charged for another thread's work. A frame's stages all run on the thread that asked
/// for it (the fan-outs are timed around their join, on that thread), so collecting per thread
/// is exactly one frame's own work. Recording must also be on, as for everything else here.
pub fn begin_capture() {
    CAPTURE.with(|c| *c.borrow_mut() = Some([0; N]));
}

/// P-19: stop collecting and hand back each stage's nanoseconds on this thread since
/// [`begin_capture`], in run order, or `None` if nothing was being collected.
pub fn end_capture() -> Option<Vec<(Stage, u64)>> {
    CAPTURE
        .with(|c| c.borrow_mut().take())
        .map(|own| Stage::ALL.iter().map(|&s| (s, own[s.index()])).collect())
}

/// `(nanoseconds, calls)` for one stage since the last [`reset`].
pub fn read(stage: Stage) -> (u64, u64) {
    (
        NANOS[stage.index()].load(Ordering::Relaxed),
        CALLS[stage.index()].load(Ordering::Relaxed),
    )
}

/// Every stage in run order, with its nanoseconds and call count.
pub fn snapshot() -> Vec<(Stage, u64, u64)> {
    Stage::ALL
        .iter()
        .map(|&s| {
            let (nanos, calls) = read(s);
            (s, nanos, calls)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Off means nothing is recorded, on means it is, and reset means it is not any more. The
    /// three properties the artifact depends on, in the order a reader would doubt them.
    #[test]
    fn records_only_while_enabled() {
        // Serialised by holding the whole check in one test: the counters are process-global.
        reset();
        disable();
        time(Stage::Mask, || std::hint::black_box(1));
        assert_eq!(read(Stage::Mask), (0, 0), "recorded while switched off");

        enable();
        // Long enough to be longer than the clock's own step. `black_box(1)` was not: Windows
        // ticks its performance counter in the hundreds of nanoseconds and an integer that is
        // handed straight back takes a few, so the assertion below failed on about one run in a
        // dozen -- a false alarm about the timer rather than a fault in it.
        time(Stage::Mask, || {
            let at = std::time::Instant::now();
            while at.elapsed().as_nanos() == 0 {
                std::hint::spin_loop();
            }
        });
        disable();
        let (nanos, calls) = read(Stage::Mask);
        assert_eq!(calls, 1, "one timed call counted {calls} times");
        assert!(nanos > 0, "a timed call took no measurable time");

        reset();
        assert_eq!(read(Stage::Mask), (0, 0), "reset left {nanos} ns behind");
    }
}
