# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
