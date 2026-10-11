# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-11

### Added

- The background generator, taken out of ScreenForge 0.34.0 by way of
  `kulisse-core` (Kulisse 0.2.2): seven styles, Oklab mood palettes, film
  grain, variants, keep-clear regions, a preview cache.
- `GeneratedBackground::smooth_waves` draws the classic waves as smooth
  curves; off by default, so the reference pixels stay the same.
- `BackgroundCache::get_or_render` is generic over the error type.
- Golden test with 126 reference renders from ScreenForge 0.34.0.
