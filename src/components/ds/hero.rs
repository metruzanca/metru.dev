use dioxus::prelude::*;

use crate::components::ui::icons::{PaletteIcon, TypeIcon, ComponentIcon, SparklesIcon};

const HERO_IMAGE: Asset = asset!("/assets/synthwave-horizon.png");

#[component]
fn BentoCard(class: Option<String>, children: Element) -> Element {
    let mut classes = String::from(
        "group relative overflow-hidden rounded-xl border border-border bg-card p-6 transition-colors hover:border-primary/50",
    );
    if let Some(c) = class {
        classes.push(' ');
        classes.push_str(&c);
    }

    rsx! {
        div {
            class: classes,
            {children}
        }
    }
}

#[component]
pub fn DsHero() -> Element {
    rsx! {
        section {
            id: "overview",
            class: "px-4 py-10 md:px-8 md:py-14",
            div {
                class: "mb-10 max-w-3xl",
                span {
                    class: "inline-flex items-center gap-2 rounded-full border border-border bg-card px-3 py-1 font-mono text-xs uppercase tracking-widest text-accent",
                    SparklesIcon { class: "size-3.5" }
                    "Design System"
                }
                h1 {
                    class: "mt-5 font-display text-4xl font-extrabold uppercase leading-tight tracking-tight text-balance text-foreground md:text-6xl",
                    "Outrun "
                    span {
                        class: "text-primary text-glow-pink",
                        "Synthwave"
                    }
                }
                p {
                    class: "mt-4 max-w-xl text-pretty leading-relaxed text-muted-foreground md:text-lg",
                    "A neon-drenched, retro-futuristic system for building interfaces that feel like a 1985 dream of the future. Colors, type, and components, all glowing."
                }
            }

            div {
                class: "grid grid-cols-1 gap-4 md:grid-cols-3",
                BentoCard {
                    class: "md:col-span-2 md:row-span-2 p-0",
                    div {
                        class: "relative h-full min-h-72 w-full",
                        img {
                            src: HERO_IMAGE,
                            alt: "Neon synthwave sunset over a glowing perspective grid",
                            class: "absolute inset-0 h-full w-full object-cover",
                        }
                        div { class: "absolute inset-0 synth-scanlines" }
                        div { class: "absolute inset-0 bg-gradient-to-t from-card via-card/20 to-transparent" }
                        div {
                            class: "absolute bottom-0 left-0 p-6",
                            p {
                                class: "font-mono text-xs uppercase tracking-widest text-accent",
                                "Brand"
                            }
                            p {
                                class: "mt-1 font-display text-2xl font-bold uppercase text-foreground",
                                "Drive into the neon"
                            }
                        }
                    }
                }

                BentoCard {
                    class: "",
                    PaletteIcon { class: "size-5 text-primary" }
                    p {
                        class: "mt-4 font-display text-lg font-bold uppercase text-foreground",
                        "Colors"
                    }
                    p {
                        class: "mt-1 text-sm leading-relaxed text-muted-foreground",
                        "Electric magenta, cyan, and violet on deep midnight."
                    }
                    div {
                        class: "mt-4 flex gap-1.5",
                        span { class: "size-5 rounded-full bg-primary shadow-glow-pink" }
                        span { class: "size-5 rounded-full bg-accent shadow-glow-cyan" }
                        span { class: "size-5 rounded-full bg-neon-purple" }
                        span { class: "size-5 rounded-full bg-neon-amber" }
                    }
                }

                BentoCard {
                    class: "",
                    TypeIcon { class: "size-5 text-accent" }
                    p {
                        class: "mt-4 font-display text-lg font-bold uppercase text-foreground",
                        "Typography"
                    }
                    p {
                        class: "mt-1 text-sm leading-relaxed text-muted-foreground",
                        "Orbitron for displays, Geist for clean body copy."
                    }
                }

                BentoCard {
                    class: "md:col-span-3",
                    div {
                        class: "flex flex-wrap items-center justify-between gap-4",
                        div {
                            class: "flex items-center gap-3",
                            ComponentIcon { class: "size-5 text-primary" }
                            div {
                                p {
                                    class: "font-display text-lg font-bold uppercase text-foreground",
                                    "24 Components"
                                }
                                p {
                                    class: "text-sm text-muted-foreground",
                                    "Accessible building blocks, glowing by default."
                                }
                            }
                        }
                        div {
                            class: "flex flex-wrap gap-2 font-mono text-xs text-muted-foreground",
                            for tag in ["Buttons", "Badges", "Inputs", "Cards", "Toggles"].iter() {
                                span {
                                    class: "rounded-md border border-border bg-muted/50 px-2 py-1",
                                    "{tag}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
