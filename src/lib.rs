//! prospekt: deterministic, procedural backgrounds drawn with Cairo.
//!
//! A [`model::GeneratedBackground`] holds only inputs — a seed, a style, a
//! palette and a handful of `0.0..=1.0` knobs. [`generator::render`] turns
//! those inputs into vector drawing on any Cairo context, so the same
//! inputs give the same picture at every size. No GTK dependency: the
//! crate is shared by Kulisse (wallpapers) and ScreenForge (screenshot
//! compositions). A *Prospekt* is the painted backdrop at the back of a
//! stage.

pub mod background_cache;
pub mod blur;
pub mod error;
pub mod generator;
pub mod model;
pub mod palette;
pub mod rng;
mod styles;

pub use error::RenderError;
pub use generator::{render, variant, Region};
pub use model::{ColorStrategy, GeneratedBackground, GeneratorStyle, Mood, Rgba};
