# Backlog

- [x] Algorithm version per background (`version`), so improvements reach new backgrounds without changing saved ones; replaces opt-in flags like `smooth_waves`
- [ ] SVG output (Cairo `SvgSurface`), for small vector wallpapers
- [x] Contours style
- [x] More styles: organic blobs, mesh gradients, low-poly triangles, Bauhaus geometry
- [ ] A time parameter for animated backgrounds (ScreenForge WebM, Kulisse transitions)
- [ ] Faster rendering: tiles on several threads, a quicker blur (mist and arcs take up to 0.85 s at 4K)
- [x] GitHub Actions running the golden test on every push (passes on Ubuntu 24.04's older Cairo and Pixman too)
- [ ] crates.io and docs.rs, once the API has settled
- [ ] Mesh renders visibly differently on Ubuntu 26.04 (Cairo 1.18.4; all nine golden lines, block colors up to 176 steps off), while Arch (Cairo 1.18.6) and Ubuntu 24.04 (1.18.0) match. Find the cause before moving CI off `ubuntu-24.04`
