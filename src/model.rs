//! The inputs that fully describe one generated background.

use serde::{Deserialize, Serialize};

/// A straight RGBA color, channels in `0.0..=1.0`.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

impl Rgba {
    pub const fn new(r: f64, g: f64, b: f64, a: f64) -> Self {
        Self { r, g, b, a }
    }

    pub const WHITE: Rgba = Rgba::new(1.0, 1.0, 1.0, 1.0);
    pub const BLACK: Rgba = Rgba::new(0.0, 0.0, 0.0, 1.0);
}

/// How a [`GeneratedBackground`]'s palette is derived. `Random` and
/// `Grayscale` ignore the screenshots entirely; `Manual` uses the user's own
/// 4 chosen colors as-is; `FromScreenshots` derives from the currently
/// visible screenshots' dominant color, with `inverse_contrast` sliding
/// between staying close to it and swinging to its complement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ColorStrategy {
    /// The user's own 4 colors, picked directly (`palette` holds exactly
    /// what they chose; nothing is derived).
    Manual,
    /// The screenshots' dominant color, shifted toward its complement by
    /// `inverse_contrast`.
    FromScreenshots,
    /// A pure lightness ramp, no hue.
    Grayscale,
    /// Ignores the screenshots entirely; a fresh aesthetically-valid
    /// palette each time, seeded like everything else here.
    Random,
}

/// Which procedural scene a [`GeneratedBackground`] draws. Every style
/// shares the same seed, palette and knobs; see `crate::styles` for how
/// each one interprets them. `Waves` is the original wave-layer generator,
/// kept so projects saved before styles existed render exactly as before
/// (it is the serde default for a missing `style` field).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum GeneratorStyle {
    #[default]
    Waves,
    Layers,
    Arcs,
    Ribbons,
    Planes,
    Lines,
    Mist,
    /// Paper-cut topography: nested contour layers of a smooth height
    /// field. New in Prospekt 0.3.0.
    Contours,
    /// Organic rounded shapes floating over each other. New in 0.4.0.
    Blobs,
    /// A soft mesh gradient of a few color points. New in 0.4.0.
    Mesh,
    /// Faceted triangles, shaded along a gradient. New in 0.4.0.
    LowPoly,
    /// Flat geometric shapes on a grid: quarter and half circles,
    /// triangles, bars. New in 0.4.0.
    Bauhaus,
}

impl GeneratorStyle {
    /// The styles offered for new backgrounds and mixed variants — every
    /// style except the legacy `Waves`.
    pub const MODERN: [GeneratorStyle; 11] = [
        GeneratorStyle::Layers,
        GeneratorStyle::Arcs,
        GeneratorStyle::Ribbons,
        GeneratorStyle::Planes,
        GeneratorStyle::Lines,
        GeneratorStyle::Mist,
        GeneratorStyle::Contours,
        GeneratorStyle::Blobs,
        GeneratorStyle::Mesh,
        GeneratorStyle::LowPoly,
        GeneratorStyle::Bauhaus,
    ];
}

/// The lightness range a generated palette spans. Only affects how a palette
/// is *resolved* (`crate::palette::resolve_palette_for`); rendering itself
/// reads just the resolved `palette`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Default)]
#[serde(rename_all = "kebab-case")]
pub enum Mood {
    #[default]
    Vivid,
    Light,
    Dark,
}

/// A background rendered procedurally from a seed and a handful of
/// parameters, rather than a fixed image or a simple gradient (spec §4).
/// Deliberately holds only *inputs* to generation — `palette`, `seed`, and
/// every numeric knob below — never the resolved vector scene itself: the
/// same inputs always regenerate the identical scene (see
/// `crate::generator::render`), so there's nothing else worth persisting
/// (spec §20/§24 — "the project must be able to regenerate the exact same
/// background", not store a rendered image of it).
///
/// The scene itself is a single algorithm — a fan of nested, wave-perturbed
/// arc layers around a focus point (see `crate::generator::draw_wave_layers`)
/// — rather than a choice of unrelated styles; `corner_bias` alone controls
/// whether that focus point (and thus the whole look) reads as flat wave
/// bands or as nested arcs anchored in a canvas corner.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GeneratedBackground {
    pub seed: u64,
    pub color_strategy: ColorStrategy,
    /// The resolved palette this background was last generated with —
    /// derived from `color_strategy` (and the screenshots, unless it's
    /// `Random`) at the moment Generate/Regenerate was pressed. Kept
    /// explicit rather than re-derived from the live document on every
    /// render, so the background stays stable while editing unrelated
    /// things, and so `adapt_to_screenshots = false` really means
    /// "frozen" (spec §21 "Lock Background"). For `ColorStrategy::Manual`
    /// this *is* the user's input, not a derived value.
    pub palette: Vec<Rgba>,
    /// When true, changing which screenshots are visible re-resolves
    /// `palette` (and re-renders) automatically; when false, the palette
    /// stays exactly as generated until the user explicitly regenerates
    /// (spec §22). Meaningless for `Manual`/`Random`, which never derive
    /// from the screenshots in the first place.
    pub adapt_to_screenshots: bool,
    /// `0.0..=1.0` — how strongly `ColorStrategy::FromScreenshots` contrasts
    /// against, rather than matches, the screenshots' own colors (spec §2).
    pub inverse_contrast: f64,
    /// `0.0..=1.0` generation knobs — see `GeneratedBackground::new` for
    /// sensible defaults.
    pub density: f64,
    pub flow: f64,
    pub variation: f64,
    pub contrast: f64,
    pub softness: f64,
    /// `0.0..=1.0` — 0 reads as flat, near-parallel wave bands; 1 as nested
    /// arcs anchored in a canvas corner; in between, a continuous morph
    /// between the two. See `crate::generator::draw_wave_layers`.
    pub corner_bias: f64,
    /// `-1.0..=1.0` — shifts the pattern's focus point horizontally
    /// (`-1.0`/`1.0` move it a full half-canvas-width left/right), letting
    /// the user recenter it without touching `corner_bias`'s wave-vs-arc
    /// morph. `#[serde(default)]` so a project saved before this field
    /// existed still loads, with the pattern staying exactly where it was
    /// (`0.0` is "unmoved").
    #[serde(default)]
    pub offset_x: f64,
    /// The vertical counterpart of `offset_x`.
    #[serde(default)]
    pub offset_y: f64,
    /// `>0.0` — zooms the whole pattern in (`> 1.0`) or out (`< 1.0`) around
    /// its focus point; `1.0` is unscaled. `#[serde(default = "..")]` so a
    /// project saved before this field existed still loads at its original
    /// size.
    #[serde(default = "default_generated_scale")]
    pub scale: f64,
    /// Which scene to draw. Missing in projects saved before v0.26.0, which
    /// therefore load as `Waves` and look exactly as they did.
    #[serde(default)]
    pub style: GeneratorStyle,
    #[serde(default)]
    pub mood: Mood,
    /// `0.0..=1.0` film grain over the finished scene, against gradient
    /// banding. `0.0` for old projects, which had none.
    #[serde(default)]
    pub grain: f64,
    /// Which revision of the drawing algorithms this background uses. A
    /// change that would alter how saved backgrounds look only applies
    /// from the version that introduced it, so saved scenes keep their
    /// pixels while new backgrounds ([`GeneratedBackground::new`]) get the
    /// improvements. Data saved before the field existed loads as 1.
    ///
    /// - 1: as in ScreenForge 0.34.0.
    /// - 2: the classic waves are smooth curves instead of a 24-point
    ///   polygon.
    #[serde(default = "first_version")]
    pub version: u32,
}

fn default_generated_scale() -> f64 {
    1.0
}

fn first_version() -> u32 {
    1
}

impl GeneratedBackground {
    /// The newest algorithm version; see [`GeneratedBackground::version`].
    pub const LATEST_VERSION: u32 = 2;

    /// A fresh background with reasonable defaults — restrained enough
    /// that the very first "Generate" click already looks intentional,
    /// not just technically valid. `color_strategy` defaults to `Random`
    /// (spec: "Zufällig soll zum Default-Modus für generierte
    /// Hintergründe werden") — the one strategy that needs nothing from
    /// the document (no screenshots to derive from, no manual colors to
    /// have picked yet) to already produce a good-looking result on the
    /// very first "Generieren" click.
    pub fn new(seed: u64) -> Self {
        Self {
            seed,
            color_strategy: ColorStrategy::Random,
            palette: Vec::new(),
            adapt_to_screenshots: true,
            inverse_contrast: 0.5,
            density: 0.4,
            flow: 0.5,
            variation: 0.4,
            contrast: 0.5,
            softness: 0.6,
            corner_bias: 0.5,
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            style: GeneratorStyle::Layers,
            mood: Mood::Vivid,
            grain: 0.35,
            version: GeneratedBackground::LATEST_VERSION,
        }
    }
}
