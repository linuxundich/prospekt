//! Contact sheet soft vs. strong: `cargo run --release --example vibrance_sheet -- OUT.png`
use cairo::{Context, Format, ImageSurface};
use prospekt::{palette, GeneratedBackground, GeneratorStyle, Mood};
fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| "vibrance.png".into());
    let styles = [GeneratorStyle::Layers, GeneratorStyle::Arcs, GeneratorStyle::Ribbons, GeneratorStyle::Blobs, GeneratorStyle::Mesh, GeneratorStyle::Bauhaus];
    let moods = [Mood::Vivid, Mood::Light, Mood::Dark];
    let (w, h) = (320, 180);
    let cols = styles.len() as i32;
    let rows = moods.len() as i32 * 3;
    let sheet = ImageSurface::create(Format::ARgb32, w * cols, h * rows).unwrap();
    let cx = Context::new(&sheet).unwrap();
    for (mi, &mood) in moods.iter().enumerate() {
        for (vi, v) in [0.0, 0.5, 1.0].iter().enumerate() {
            for (si, &style) in styles.iter().enumerate() {
                let seed = 2026 + si as u64 * 7 + mi as u64;
                let mut bg = GeneratedBackground { style, mood, vibrance: *v, ..GeneratedBackground::new(seed) };
                bg.palette = palette::vibrant_mood_palette(seed, mood, None, 1.0, *v);
                let s = ImageSurface::create(Format::ARgb32, w, h).unwrap();
                prospekt::render(&Context::new(&s).unwrap(), &bg, w as f64, h as f64, &[]).unwrap();
                cx.set_source_surface(&s, (si as i32 * w) as f64, ((mi * 3 + vi) as i32 * h) as f64).unwrap();
                cx.paint().unwrap();
            }
        }
    }
    sheet.write_to_png(&mut std::fs::File::create(out).unwrap()).unwrap();
}
