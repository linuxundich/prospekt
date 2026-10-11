//! Pins the exact pixels of a spread of backgrounds. The reference
//! checksums were taken from ScreenForge 0.34.0, where this generator came
//! from, so a failure here means saved ScreenForge scenes and Kulisse
//! favorites would no longer look the same. Those lines use algorithm
//! version 1; lines starting with `v2` pin what version 2 changed, and
//! styles added later (contours, then blobs, mesh, low-poly and Bauhaus)
//! follow at the end.
//!
//! Each line ends with an exact checksum and a coarse fingerprint (the mean
//! color of 8×5 blocks). Locally the checksum must match. Other Cairo and
//! Pixman versions anti-alias a few edge pixels differently, so CI sets
//! `PROSPEKT_GOLDEN_TOLERANCE=2`: a differing checksum then passes when no
//! block color moved by more than that many steps (of 255).
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

/// `"<checksum> <fingerprint>"` of `bg` rendered at 320×200.
fn checksum(bg: &GeneratedBackground, regions: &[Region]) -> String {
    let (w, h) = (320usize, 200usize);
    let mut surface = ImageSurface::create(Format::ARgb32, w as i32, h as i32).unwrap();
    prospekt::render(&Context::new(&surface).unwrap(), bg, w as f64, h as f64, regions).unwrap();
    let stride = surface.stride() as usize;
    let data = surface.data().unwrap();
    let hash = data.iter().fold(0xcbf2_9ce4_8422_2325u64, |h, &b| (h ^ b as u64).wrapping_mul(0x0100_0000_01b3));
    let (bw, bh) = (w / 8, h / 5);
    let mut fingerprint = String::new();
    for by in 0..5 {
        for bx in 0..8 {
            let mut sum = [0u64; 3];
            for y in by * bh..(by + 1) * bh {
                for x in bx * bw..(bx + 1) * bw {
                    let px = &data[y * stride + x * 4..y * stride + x * 4 + 3];
                    for c in 0..3 {
                        sum[c] += px[c] as u64;
                    }
                }
            }
            for c in sum {
                fingerprint += &format!("{:02x}", c / (bw * bh) as u64);
            }
        }
    }
    format!("{hash:016x} {fingerprint}")
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
                    out.push(format!("{style:?} {mood:?} {seed} {avoid} {}", checksum(&bg, &regions)));
                    if style == GeneratorStyle::Waves {
                        v2.push(format!("v2 {style:?} {mood:?} {seed} {avoid} {}", checksum(&GeneratedBackground { version: 2, ..bg }, &regions)));
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
            out.push(format!("Contours {mood:?} {seed} {}", checksum(&bg, &regions)));
        }
    }
    // Added in 0.4.0.
    for style in [GeneratorStyle::Blobs, GeneratorStyle::Mesh, GeneratorStyle::LowPoly, GeneratorStyle::Bauhaus] {
        for mood in [Mood::Vivid, Mood::Light, Mood::Dark] {
            for seed in [7u64, 42, 2026] {
                let mut bg = GeneratedBackground { style, mood, grain: 0.4, ..GeneratedBackground::new(seed) };
                bg.palette = resolve_palette_for(&[], bg.color_strategy, bg.inverse_contrast, seed, style, mood);
                out.push(format!("{style:?} {mood:?} {seed} {}", checksum(&bg, &regions)));
            }
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
    let tolerance: Option<i32> = std::env::var("PROSPEKT_GOLDEN_TOLERANCE").ok().and_then(|v| v.parse().ok());
    let changed: Vec<_> = expected.lines().zip(&actual).filter(|(e, a)| !matches(e, a, tolerance)).map(|(_, a)| a.clone()).collect();
    assert!(changed.is_empty(), "{} renders changed:\n{}", changed.len(), changed.join("\n"));
    assert_eq!(expected.lines().count(), actual.len());
}

/// `[name, checksum, fingerprint]` of a golden line.
fn split_line(line: &str) -> Vec<&str> {
    let mut parts: Vec<&str> = line.rsplitn(3, ' ').collect();
    parts.reverse();
    parts
}

/// Whether a line matches its reference: exactly, or with a tolerance on
/// the fingerprint when the checksum differs.
fn matches(expected: &str, actual: &str, tolerance: Option<i32>) -> bool {
    if expected == actual {
        return true;
    }
    let Some(tolerance) = tolerance else { return false };
    let (e, a) = (split_line(expected), split_line(actual));
    if e.len() != 3 || a.len() != 3 || e[0] != a[0] || e[2].len() != a[2].len() {
        return false;
    }
    let bytes = |hex: &str| (0..hex.len()).step_by(2).map(|i| i32::from_str_radix(&hex[i..i + 2], 16).unwrap_or(-999)).collect::<Vec<_>>();
    bytes(e[2]).iter().zip(bytes(a[2])).all(|(x, y)| (x - y).abs() <= tolerance)
}
