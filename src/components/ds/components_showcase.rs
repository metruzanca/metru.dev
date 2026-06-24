use dioxus::prelude::*;

use crate::components::ui::button::{Button, ButtonVariant, ButtonSize};
use crate::components::ui::section_heading::SectionHeading;
use crate::components::ui::panel::Panel;
use crate::components::ui::input::{Input, InputGlow};
use crate::components::ui::toggle::Toggle;
use crate::components::ui::icons::{ZapIcon, ArrowRightIcon, HeartIcon, PlayIcon, PlusIcon};

const BADGES: &[(&str, &str)] = &[
    ("Online", "border-accent/40 bg-accent/10 text-accent"),
    ("New", "border-primary/40 bg-primary/10 text-primary"),
    ("Beta", "border-neon-purple/40 bg-neon-purple/10 text-neon-purple"),
    ("Pro", "border-neon-amber/40 bg-neon-amber/10 text-neon-amber"),
    ("Offline", "border-border bg-muted text-muted-foreground"),
];

#[component]
pub fn DsComponentsShowcase() -> Element {
    let mut callsign = use_signal(|| String::new());
    let mut frequency = use_signal(|| String::new());
    let mut toggled = use_signal(|| true);

    rsx! {
        div {
            class: "flex flex-col gap-16 px-4 py-12 md:px-8",
            // Buttons
            section {
                class: "flex flex-col gap-6",
                SectionHeading {
                    id: "buttons".to_string(),
                    eyebrow: "Components".to_string(),
                    title: "Buttons".to_string(),
                    description: "Action triggers across variants and sizes. The primary button carries the signature neon glow.".to_string(),
                }
                Panel {
                    class: None,
                    div {
                        class: "flex flex-col gap-6",
                        div {
                            class: "flex flex-wrap items-center gap-3",
                            Button {
                                variant: ButtonVariant::Default,
                                size: ButtonSize::Default,
                                class: "shadow-glow-pink",
                                ZapIcon { class: "size-4" }
                                "Primary"
                            }
                            Button {
                                variant: ButtonVariant::Secondary,
                                size: ButtonSize::Default,
                                "Secondary"
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Default,
                                "Outline"
                            }
                            Button {
                                variant: ButtonVariant::Ghost,
                                size: ButtonSize::Default,
                                "Ghost"
                            }
                            Button {
                                variant: ButtonVariant::Destructive,
                                size: ButtonSize::Default,
                                "Destructive"
                            }
                            Button {
                                variant: ButtonVariant::Link,
                                size: ButtonSize::Default,
                                "Link"
                                ArrowRightIcon { class: "size-4" }
                            }
                        }
                        div {
                            class: "flex flex-wrap items-center gap-3",
                            Button {
                                variant: ButtonVariant::Default,
                                size: ButtonSize::Sm,
                                "Small"
                            }
                            Button {
                                variant: ButtonVariant::Default,
                                size: ButtonSize::Default,
                                "Default"
                            }
                            Button {
                                variant: ButtonVariant::Default,
                                size: ButtonSize::Lg,
                                "Large"
                            }
                            Button {
                                variant: ButtonVariant::Default,
                                size: ButtonSize::Icon,
                                aria_label: "Add",
                                PlusIcon { class: "size-4" }
                            }
                            Button {
                                variant: ButtonVariant::Outline,
                                size: ButtonSize::Icon,
                                aria_label: "Play",
                                PlayIcon { class: "size-4" }
                            }
                        }
                    }
                }
            }

            // Badges
            section {
                class: "flex flex-col gap-6",
                SectionHeading {
                    id: "badges".to_string(),
                    eyebrow: "Components".to_string(),
                    title: "Badges".to_string(),
                    description: "Compact status indicators with subtle neon tints for quick scanning.".to_string(),
                }
                Panel {
                    class: None,
                    div {
                        class: "flex flex-wrap gap-3",
                        for (label, variant_classes) in BADGES.iter() {
                            span {
                                class: "inline-flex items-center gap-1.5 rounded-full border px-3 py-1 font-mono text-xs uppercase tracking-widest {variant_classes}",
                                span { class: "size-1.5 rounded-full bg-current" }
                                "{label}"
                            }
                        }
                    }
                }
            }

            // Inputs & Controls
            section {
                class: "flex flex-col gap-6",
                SectionHeading {
                    id: "inputs".to_string(),
                    eyebrow: "Components".to_string(),
                    title: "Inputs & Controls".to_string(),
                    description: "Form fields glow on focus, switches snap with neon feedback.".to_string(),
                }
                Panel {
                    class: None,
                    div {
                        class: "grid grid-cols-1 gap-6 md:grid-cols-2",
                        Input {
                            id: "callsign".to_string(),
                            label: "Callsign".to_string(),
                            value: callsign,
                            glow: InputGlow::Pink,
                            placeholder: "Enter your handle".to_string(),
                        }
                        Input {
                            id: "freq".to_string(),
                            label: "Frequency".to_string(),
                            value: frequency,
                            glow: InputGlow::Cyan,
                            placeholder: "88.5 FM".to_string(),
                        }
                        div {
                            class: "flex items-center justify-between gap-4 md:col-span-2",
                            div {
                                p {
                                    class: "text-sm font-medium text-foreground",
                                    "Neon Mode"
                                }
                                p {
                                    class: "text-xs text-muted-foreground",
                                    "Crank the glow to maximum."
                                }
                            }
                            Toggle {
                                checked: toggled,
                                class: None,
                            }
                        }
                    }
                }
            }

            // Cards
            section {
                class: "flex flex-col gap-6",
                SectionHeading {
                    id: "cards".to_string(),
                    eyebrow: "Components".to_string(),
                    title: "Cards".to_string(),
                    description: "Composable surfaces for grouping content, stats, and calls to action.".to_string(),
                }
                div {
                    class: "grid grid-cols-1 gap-4 md:grid-cols-3",
                    div {
                        class: "rounded-xl border border-border bg-card p-6 transition-colors hover:border-primary/50",
                        HeartIcon { class: "size-5 text-primary" }
                        p {
                            class: "mt-4 font-display text-lg font-bold uppercase text-foreground",
                            "Track 01"
                        }
                        p {
                            class: "mt-1 text-sm leading-relaxed text-muted-foreground",
                            "Midnight cruise through endless neon boulevards."
                        }
                        Button {
                            variant: ButtonVariant::Outline,
                            size: ButtonSize::Sm,
                            class: "mt-4",
                            PlayIcon { class: "size-4" }
                            "Play"
                        }
                    }

                    div {
                        class: "rounded-xl border border-primary/40 bg-card p-6 shadow-glow-pink",
                        span {
                            class: "font-mono text-xs uppercase tracking-widest text-primary",
                            "Featured"
                        }
                        p {
                            class: "mt-3 font-display text-3xl font-extrabold text-foreground",
                            "88"
                            span {
                                class: "ml-1 text-base font-normal text-muted-foreground",
                                "bpm"
                            }
                        }
                        p {
                            class: "mt-1 text-sm leading-relaxed text-muted-foreground",
                            "The exact tempo of the future."
                        }
                        Button {
                            variant: ButtonVariant::Default,
                            size: ButtonSize::Default,
                            class: "mt-4 w-full shadow-glow-pink",
                            "Subscribe"
                        }
                    }

                    div {
                        class: "relative overflow-hidden rounded-xl border border-border bg-card p-6 synth-grid",
                        div {
                            class: "relative",
                            ZapIcon { class: "size-5 text-accent" }
                            p {
                                class: "mt-4 font-display text-lg font-bold uppercase text-foreground",
                                "Boost"
                            }
                            p {
                                class: "mt-1 text-sm leading-relaxed text-muted-foreground",
                                "Overclock your interface with pure voltage."
                            }
                            Button {
                                variant: ButtonVariant::Secondary,
                                size: ButtonSize::Sm,
                                class: "mt-4",
                                "Activate"
                            }
                        }
                    }
                }
            }
        }
    }
}
