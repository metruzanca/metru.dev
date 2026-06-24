use dioxus::prelude::*;

use crate::components::ui::section_heading::SectionHeading;

const SWATCHES: &[(&str, &str, &str, &str)] = &[
    ("Primary", "--primary", "bg-primary", "Neon Magenta"),
    ("Accent", "--accent", "bg-accent", "Electric Cyan"),
    ("Violet", "--neon-purple", "bg-neon-purple", "Laser Violet"),
    ("Amber", "--neon-amber", "bg-neon-amber", "Sunset Amber"),
    ("Background", "--background", "bg-background", "Midnight"),
    ("Card", "--card", "bg-card", "Deep Indigo"),
    ("Muted", "--muted", "bg-muted", "Dusk"),
    ("Foreground", "--foreground", "bg-foreground", "Starlight"),
];

const TYPE_SCALE: &[(&str, &str, &str)] = &[
    ("Display", "font-display text-5xl font-extrabold uppercase tracking-tight", "Neon Drive"),
    ("Heading 1", "font-display text-3xl font-bold uppercase", "Retro Future"),
    ("Heading 2", "font-display text-xl font-semibold uppercase", "Grid Runner"),
    ("Body", "text-base leading-relaxed", "Cruising the highway at the edge of the simulation."),
    ("Mono", "font-mono text-sm", "const speed = 88 // mph"),
];

#[component]
pub fn DsFoundations() -> Element {
    rsx! {
        div {
            class: "flex flex-col gap-16 px-4 py-12 md:px-8",
            // Colors
            section {
                class: "flex flex-col gap-6",
                SectionHeading {
                    id: "colors".to_string(),
                    eyebrow: "Foundations".to_string(),
                    title: "Colors".to_string(),
                    description: "A tight, high-contrast palette: vivid neon accents that glow against deep midnight surfaces. Every color is a CSS variable token.".to_string(),
                }
                div {
                    class: "grid grid-cols-2 gap-3 sm:grid-cols-3 lg:grid-cols-4",
                    for swatch in SWATCHES.iter() {
                        div {
                            class: "overflow-hidden rounded-lg border border-border bg-card",
                            div {
                                class: "h-20 w-full {swatch.2}",
                            }
                            div {
                                class: "p-3",
                                p { class: "text-sm font-medium text-foreground", "{swatch.0}" }
                                p { class: "text-xs text-muted-foreground", "{swatch.3}" }
                                p { class: "mt-1 font-mono text-[0.7rem] text-accent", "{swatch.1}" }
                            }
                        }
                    }
                }
            }

            // Typography
            section {
                class: "flex flex-col gap-6",
                SectionHeading {
                    id: "typography".to_string(),
                    eyebrow: "Foundations".to_string(),
                    title: "Typography".to_string(),
                    description: "Orbitron drives the geometric, retro-arcade headings while Geist keeps body copy crisp and legible.".to_string(),
                }
                div {
                    class: "divide-y divide-border overflow-hidden rounded-xl border border-border bg-card",
                    for t in TYPE_SCALE.iter() {
                        div {
                            class: "flex flex-col gap-2 p-5 md:flex-row md:items-baseline md:gap-8",
                            span {
                                class: "w-24 shrink-0 font-mono text-xs uppercase tracking-widest text-muted-foreground",
                                "{t.0}"
                            }
                            span {
                                class: "text-foreground {t.1}",
                                "{t.2}"
                            }
                        }
                    }
                }
            }

            // Effects
            section {
                class: "flex flex-col gap-6",
                SectionHeading {
                    id: "effects".to_string(),
                    eyebrow: "Foundations".to_string(),
                    title: "Effects".to_string(),
                    description: "Glow utilities and retro textures bring the scene to life — neon text shadows, box glows, perspective grids, and CRT scanlines.".to_string(),
                }
                div {
                    class: "grid grid-cols-1 gap-4 sm:grid-cols-2 lg:grid-cols-3",
                    div {
                        class: "flex h-32 items-center justify-center rounded-xl border border-border bg-card",
                        span {
                            class: "font-display text-2xl font-bold uppercase text-primary text-glow-pink",
                            "Pink Glow"
                        }
                    }
                    div {
                        class: "flex h-32 items-center justify-center rounded-xl border border-border bg-card",
                        span {
                            class: "font-display text-2xl font-bold uppercase text-accent text-glow-cyan",
                            "Cyan Glow"
                        }
                    }
                    div {
                        class: "relative flex h-32 items-center justify-center overflow-hidden rounded-xl border border-border bg-card synth-grid",
                        div { class: "absolute inset-0 synth-scanlines" }
                        span {
                            class: "relative font-mono text-sm uppercase tracking-widest text-foreground",
                            "Grid + Scanlines"
                        }
                    }
                }
            }
        }
    }
}
