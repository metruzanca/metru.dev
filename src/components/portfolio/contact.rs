use dioxus::prelude::*;

use crate::components::ui::icons::{GitBranchIcon, AtSignIcon, MailIcon, Gamepad2Icon};
use super::section_heading::PortfolioSectionHeading;

struct Channel {
    label: &'static str,
    handle: &'static str,
    href: &'static str,
}

const CHANNELS: &[Channel] = &[
    Channel { label: "github", handle: "@sam", href: "https://github.com" },
    Channel { label: "x", handle: "@sam", href: "https://x.com" },
    Channel { label: "twitch", handle: "@sam_dev", href: "https://twitch.tv" },
    Channel { label: "email", handle: "sam@dev.io", href: "mailto:sam@dev.io" },
];

#[component]
pub fn PortfolioContact() -> Element {
    rsx! {
        section { id: "contact", class: "px-4 py-12",
            PortfolioSectionHeading {
                index: "04".to_string(),
                label: "contact".to_string(),
                action: None,
            }

            div { class: "rounded-xl border border-border bg-card p-6 md:p-8",
                h3 { class: "font-display text-2xl font-bold uppercase tracking-tight text-balance text-foreground md:text-3xl",
                    "Let's build something "
                    span { class: "text-accent text-glow-cyan", "electric" }
                    "."
                }
                p { class: "mt-3 max-w-md text-pretty leading-relaxed text-muted-foreground",
                    "Open to freelance, collaborations, and the occasional late-night \
                     side project. Reach out on any channel below."
                }

                div { class: "mt-6 grid grid-cols-2 gap-px overflow-hidden rounded-lg border border-border bg-border sm:grid-cols-4",
                    {render_channel(0, CHANNELS)}
                    {render_channel(1, CHANNELS)}
                    {render_channel(2, CHANNELS)}
                    {render_channel(3, CHANNELS)}
                }
            }
        }
    }
}

fn render_channel(i: usize, channels: &[Channel]) -> Element {
    let ch = &channels[i];
    rsx! {
        a {
            class: "group flex flex-col gap-2 bg-card p-4 transition-colors hover:bg-muted/40",
            href: "{ch.href}",
            target: "_blank",
            rel: "noreferrer",
            {channel_icon(i)}
            span { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
                "{ch.label}"
            }
            span { class: "truncate font-mono text-xs text-foreground",
                "{ch.handle}"
            }
        }
    }
}

fn channel_icon(i: usize) -> Element {
    match i {
        0 => rsx! { GitBranchIcon { class: "size-4 text-muted-foreground transition-colors group-hover:text-primary" } },
        1 => rsx! { AtSignIcon { class: "size-4 text-muted-foreground transition-colors group-hover:text-primary" } },
        2 => rsx! { Gamepad2Icon { class: "size-4 text-muted-foreground transition-colors group-hover:text-primary" } },
        _ => rsx! { MailIcon { class: "size-4 text-muted-foreground transition-colors group-hover:text-primary" } },
    }
}
