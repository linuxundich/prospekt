//! Prints how long each style takes at a few sizes: `cargo run --release --example timing`.
use cairo::{Context, Format, ImageSurface};
use prospekt::{palette, GeneratedBackground, GeneratorStyle};
fn main() {
    for style in [GeneratorStyle::Waves, GeneratorStyle::Layers, GeneratorStyle::Arcs, GeneratorStyle::Ribbons, GeneratorStyle::Planes, GeneratorStyle::Lines, GeneratorStyle::Mist] {
        let mut bg = GeneratedBackground { style, ..GeneratedBackground::new(42) };
        bg.palette = palette::resolve_palette_for(&[], bg.color_strategy, 0.5, 42, style, bg.mood);
        let mut row = format!("{style:?}:");
        for (w, h) in [(320, 180), (1280, 720), (3840, 2160)] {
            let t = std::time::Instant::now();
            let s = ImageSurface::create(Format::ARgb32, w, h).unwrap();
            prospekt::render(&Context::new(&s).unwrap(), &bg, w as f64, h as f64, &[]).unwrap();
            row += &format!(" {w}px {:>5} ms", t.elapsed().as_millis());
        }
        println!("{row}");
    }
}
