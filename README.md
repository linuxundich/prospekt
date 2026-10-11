<h1 align="center">Prospekt</h1>

<p align="center"><strong>Procedural backgrounds that look designed.</strong></p>

<p align="center">
  A small Rust library that draws calm, abstract backgrounds with Cairo: layered paper,
  concentric arcs, silk ribbons, folded planes, line bundles, soft mist and classic waves.
  It is the generator behind Kulisse (a wallpaper app, not public yet)
  and <a href="https://github.com/linuxundich/ScreenForge">ScreenForge</a> (screenshot compositions).
</p>

A *Prospekt* is the painted backdrop at the back of a theatre stage.

## How it works

A `GeneratedBackground` holds only inputs: a seed, a style, a palette and a handful of `0.0..=1.0` knobs (density, flow, variation, contrast, softness, grain, zoom, offset). `render` turns them into vector drawing on any Cairo context. The same inputs always give the same picture, at any size from a thumbnail to 8K, so storing those few hundred bytes is enough to get a background back exactly.

- Palettes are built in Oklab: mood ramps (vivid, light, dark) around a random or chosen hue, grayscale, or colors taken from images.
- Soft shadows from one light direction and gentle gradients give depth.
- `variant` derives any number of alternatives from one background.
- Regions can be kept clear, so a composition's content doesn't sit on the busiest part.
- No GTK dependency.

## Use

```toml
[dependencies]
prospekt = { git = "https://github.com/linuxundich/prospekt", tag = "v0.1.0" }
```

```rust
use cairo::{Context, Format, ImageSurface};
use prospekt::{palette, GeneratedBackground, GeneratorStyle, Mood};

let mut bg = GeneratedBackground { style: GeneratorStyle::Arcs, mood: Mood::Dark, ..GeneratedBackground::new(42) };
bg.palette = palette::mood_palette(bg.seed, bg.mood, None, 1.0);

let surface = ImageSurface::create(Format::ARgb32, 3840, 2160)?;
prospekt::render(&Context::new(&surface)?, &bg, 3840.0, 2160.0, &[])?;
```

## Stability

A golden test pins the exact pixels of 126 backgrounds. A change that alters any of them is a visible change for saved scenes and favorites in both apps, so it is made on purpose and noted in the changelog. New drawing options come as fields that are off by default; `smooth_waves` is one.

```sh
cargo test                                   # includes the golden test
PROSPEKT_BLESS=1 cargo test --test golden     # rewrite after an intended change
cargo run --release --example timing         # render times per style
```

## License

GPL-3.0-or-later
