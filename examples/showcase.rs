//! Renders the example pictures in the README:
//! `cargo run --release --example showcase -- OUTPUT_DIR [WIDTH HEIGHT]`
//! (PNG; the README uses them converted to JPEG).

use cairo::{Context, Format, ImageSurface};
use prospekt::{palette, GeneratedBackground, GeneratorStyle, Mood};

/// (file name, style, mood, seed)
const PICTURES: [(&str, GeneratorStyle, Mood, u64); 3] = [
    ("arcs", GeneratorStyle::Arcs, Mood::Vivid, 2026),
    ("ribbons", GeneratorStyle::Ribbons, Mood::Vivid, 7),
    ("layers-dark", GeneratorStyle::Layers, Mood::Dark, 1337),
];

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let dir = std::path::PathBuf::from(args.first().map(String::as_str).unwrap_or("."));
    let width: i32 = args.get(1).and_then(|v| v.parse().ok()).unwrap_or(1600);
    let height: i32 = args.get(2).and_then(|v| v.parse().ok()).unwrap_or(900);
    for (name, style, mood, seed) in PICTURES {
        let mut bg = GeneratedBackground { style, mood, ..GeneratedBackground::new(seed) };
        bg.palette = palette::mood_palette(seed, mood, None, 1.0);
        let surface = ImageSurface::create(Format::ARgb32, width, height).expect("surface");
        prospekt::render(&Context::new(&surface).expect("context"), &bg, width as f64, height as f64, &[]).expect("render");
        let path = dir.join(format!("{name}.png"));
        surface.write_to_png(&mut std::fs::File::create(&path).expect("create file")).expect("write png");
        println!("{}", path.display());
    }
}
