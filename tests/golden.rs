//! Pins the exact pixels of a spread of backgrounds. The reference
//! checksums were taken from ScreenForge 0.34.0, where this generator came
//! from, so a failure here means saved ScreenForge scenes and Kulisse
//! favorites would no longer look the same. Those lines use algorithm
//! version 1; lines starting with `v2` pin what version 2 changed, and
//! styles added later (contours) follow at the end.
//!
//! After an intended visual change, rewrite the file with
//! `PROSPEKT_BLESS=1 cargo test --test golden` and say so in the changelog.

use cairo::{Context, Format, ImageSurface};
use prospekt::palette::resolve_palette_for;
use prospekt::{GeneratedBackground, GeneratorStyle, Mood, Region};

const STYLES: [GeneratorStyle; 7] = [
    GeneratorStyle::Waves,
    GeneratorStyle::Layers,
    GeneratorStyle::Arcs,
    GeneratorStyle::Ribbons,
    GeneratorStyle::Planes,
    GeneratorStyle::Lines,
    GeneratorStyle::Mist,
];

fn checksum(bg: &GeneratedBackground, regions: &[Region]) -> u64 {
    let (w, h) = (320, 200);
    let mut surface = ImageSurface::create(Format::ARgb32, w, h).unwrap();
    prospekt::render(&Context::new(&surface).unwrap(), bg, w as f64, h as f64, regions).unwrap();
    let data = surface.data().unwrap();
    data.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x0100_0000_01b3))
}

fn lines() -> Vec<String> {
    let regions = [Region { x: 150.0, y: 40.0, width: 100.0, height: 130.0 }];
    let mut out = Vec::new();
    let mut v2 = Vec::new();
    for style in STYLES {
        for mood in [Mood::Vivid, Mood::Light, Mood::Dark] {
            for seed in [7u64, 42, 2026] {
                for avoid in [false, true] {
                    let mut bg = GeneratedBackground { style, mood, adapt_to_screenshots: avoid, grain: 0.4, version: 1, ..GeneratedBackground::new(seed) };
                    bg.palette = resolve_palette_for(&[], bg.color_strategy, bg.inverse_contrast, seed, style, mood);
                    out.push(format!("{style:?} {mood:?} {seed} {avoid} {:016x}", checksum(&bg, &regions)));
                    if style == GeneratorStyle::Waves {
                        v2.push(format!("v2 {style:?} {mood:?} {seed} {avoid} {:016x}", checksum(&GeneratedBackground { version: 2, ..bg }, &regions)));
                    }
                }
            }
        }
    }
    // Version 2 only changes the classic waves.
    out.extend(v2);
    // Styles added later go last, so the lines above never move.
    for mood in [Mood::Vivid, Mood::Light, Mood::Dark] {
        for seed in [7u64, 42, 2026] {
            let mut bg = GeneratedBackground { style: GeneratorStyle::Contours, mood, grain: 0.4, ..GeneratedBackground::new(seed) };
            bg.palette = resolve_palette_for(&[], bg.color_strategy, bg.inverse_contrast, seed, bg.style, mood);
            out.push(format!("Contours {mood:?} {seed} {:016x}", checksum(&bg, &regions)));
        }
    }
    out
}

#[test]
fn renders_match_the_reference() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/tests/golden.txt");
    let actual = lines();
    if std::env::var_os("PROSPEKT_BLESS").is_some() {
        std::fs::write(path, actual.join("\n") + "\n").unwrap();
        return;
    }
    let expected = std::fs::read_to_string(path).expect("tests/golden.txt missing; run with PROSPEKT_BLESS=1");
    let changed: Vec<_> = expected.lines().zip(&actual).filter(|(e, a)| e != a).map(|(_, a)| a.clone()).collect();
    assert!(changed.is_empty(), "{} renders changed:\n{}", changed.len(), changed.join("\n"));
    assert_eq!(expected.lines().count(), actual.len());
}
