# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.6.0] - 2026-11-19

### Added

- Strong colors: `GeneratedBackground::vibrance` (`0.0..=1.0`, default
  `0.0`) and `palette::vibrant_mood_palette`/`resolve_palette_vibrant`.
  `0.0` gives exactly the soft, slightly pastel palettes as before; `1.0`
  narrows each mood's lightness range toward the middle and raises every
  step's chroma to what sRGB holds at its lightness and hue
  (`palette::max_chroma`), so colors are deep and saturated without
  clipping hues. Same seed, same hues, only stronger. Above zero the
  classic waves get a mood ramp too.
- Algorithm version 3 (`LATEST_VERSION`): the accent of the modern styles
  may be as saturated as sRGB allows instead of stopping at chroma 0.16,
  and with such a strong accent the arcs' second set of rings takes its
  hue instead of an RGB mix that turned gray. Soft palettes look exactly
  as in version 2. `palette::ramp_and_accent_versioned` exposes it;
  `ramp_and_accent` keeps version 2.
- Example `vibrance_sheet`: a contact sheet of soft, half and full
  strength.
- 30 golden lines for strong colors at the end; all earlier lines are
  unchanged.

### Changed

- CI runs on Ubuntu 24.04 instead of `ubuntu-latest`. On Ubuntu 26.04
  (Cairo 1.18.4) the mesh style renders visibly differently from the
  references, while Arch and Ubuntu 24.04 match; the cause is noted in the
  backlog.

## [0.5.0] - 2026-11-18

### Added

- `Quiet` style: plain and unobtrusive, for brand colors and calm
  backdrops behind screenshots. A ground color with a barely visible
  gradient, one large area a small step lighter or darker (a circle
  reaching in from a corner, or a gentle sweep across the canvas) and one
  to three fine lines along its edge. Unlike the other styles it takes the
  palette's colors as they are: the most saturated color draws the lines,
  `mood` picks the ground (lightest for `Light`, darkest for `Dark`, the
  first color for `Vivid`), so the same two or three colors give a light
  and a dark version. Twelve golden lines added at the end; all earlier
  lines are unchanged.

## [0.4.1] - 2026-10-11

### Fixed

- Low-poly: no more single facets in the accent color; they looked out of
  place. The facets use only the palette's ramp now. The style came out
  the same day, so this changes it directly instead of adding an
  algorithm version; its nine golden lines are renewed.

## [0.4.0] - 2026-10-11

### Added

- Four styles:
  - `Blobs`: organic rounded shapes over each other with soft shadows,
    one in the accent color.
  - `Mesh`: a soft mesh gradient of a few color points that flow along
    the palette, always with an accent glow.
  - `LowPoly`: faceted triangles shaded along a gradient.
  - `Bauhaus`: flat geometric shapes on a grid (quarter and half circles,
    circles, triangles, bars).
- Golden lines for the four styles, and a coarse fingerprint on every
  line: `PROSPEKT_GOLDEN_TOLERANCE` lets the test pass on other Cairo and
  Pixman versions when only edge anti-aliasing differs (CI uses 2).

### Changed

- `GeneratorStyle::MODERN` has eleven styles, so mixed variants cycle
  through all of them; existing backgrounds render exactly as before.

## [0.3.0] - 2026-10-11

### Added

- Contours style (`GeneratorStyle::Contours`): a smooth height field of
  random hills cut into paper layers, like a relief map. `density` sets
  the number of levels, `variation` the number of hills and how much the
  outlines meander.
- GitHub Actions run all tests, the golden pixel test included, on every
  push.
- Golden lines for the contours style.

### Changed

- `GeneratorStyle::MODERN` includes contours, so mixed variants
  (`variant(…, true)`) now cycle through seven styles; existing
  backgrounds render exactly as before.

## [0.2.0] - 2026-10-11

### Added

- `GeneratedBackground::version`, the algorithm version a background was
  made with, and `GeneratedBackground::LATEST_VERSION` (2). Version 1 draws
  as ScreenForge 0.34.0; version 2 draws the classic waves as smooth
  curves. Data without the field loads as version 1; `new` uses the
  latest version.
- `palette::chance_hue`, the hue `mood_palette` picks for a seed.
- Three example pictures in the README, rendered by the new `showcase`
  example.
- Golden lines for version 2.

### Removed

- `GeneratedBackground::smooth_waves`; use `version: 2`. A saved
  `smooth_waves` field is ignored.

### Fixed

- The render-time test no longer fails when the other tests share the CPU.

## [0.1.0] - 2026-10-11

### Added

- The background generator, taken out of ScreenForge 0.34.0 by way of
  `kulisse-core` (Kulisse 0.2.2): seven styles, Oklab mood palettes, film
  grain, variants, keep-clear regions, a preview cache.
- `GeneratedBackground::smooth_waves` draws the classic waves as smooth
  curves; off by default, so the reference pixels stay the same.
- `BackgroundCache::get_or_render` is generic over the error type.
- Golden test with 126 reference renders from ScreenForge 0.34.0.
