use dioxus::prelude::*;

use crate::components::ui::icons::{CalendarIcon, GitBranchIcon, LinkedInIcon, MailIcon, XIcon};

pub struct Channel<'a> {
    pub label: &'a str,
    pub handle: &'a str,
    pub href: &'a str,
    pub is_mail: bool,
}

pub const CHANNELS: &[Channel] = &[
    Channel {
        label: "calendar",
        handle: "schedule a chat",
        href: "https://cal.com/samzanca/15min",
        is_mail: false,
    },
    Channel {
        label: "linkedin",
        handle: "@samuele-zanca",
        href: "https://linkedin.com/in/samuele-zanca",
        is_mail: false,
    },
    Channel {
        label: "x",
        handle: "@metruzanca",
        href: "https://x.com/metruzanca",
        is_mail: false,
    },
    Channel {
        label: "github",
        handle: "@metruzanca",
        href: "https://github.com/metruzanca",
        is_mail: false,
    },
    Channel {
        label: "email",
        handle: "sam(at)zanca.dev",
        href: "",
        is_mail: true,
    },
];

#[component]
pub fn PortfolioContact() -> Element {
    rsx! {
        section { id: "contact", class: "px-4 py-12",
            div { class: "rounded-xl border border-border bg-card p-6 md:p-8",
                h2 { class: "font-display text-2xl font-bold uppercase tracking-tight text-balance text-foreground md:text-3xl",
                    "Let's build something "
                    span { class: "text-accent text-glow-cyan", "electric" }
                    "."
                }
                p { class: "mt-3 max-w-md text-pretty leading-relaxed text-muted-foreground",
                    "Open to freelance, collaborations, and the occasional late-night \
                     side project. Reach out on any channel below."
                }

                div { class: "mt-6 flex flex-wrap gap-6",
                    for (i, ch) in CHANNELS.iter().enumerate() {
                        if ch.is_mail {
                            div { class: "group flex flex-col gap-1.5",
                                div { class: "flex items-center gap-2",
                                    {channel_icon(i)}
                                    span { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
                                        "{ch.label}"
                                    }
                                }
                                span { class: "font-mono text-xs text-foreground",
                                    "{ch.handle}"
                                }
                            }
                        } else {
                            a {
                                class: "group flex flex-col gap-1.5",
                                href: "{ch.href}",
                                target: "_blank",
                                rel: "noreferrer",
                                div { class: "flex items-center gap-2",
                                    {channel_icon(i)}
                                    span { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
                                        "{ch.label}"
                                    }
                                }
                                span { class: "font-mono text-xs text-foreground transition-colors group-hover:text-accent",
                                    "{ch.handle}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

pub fn channel_icon(i: usize) -> Element {
    match i {
        0 => rsx! { GitBranchIcon { class: "size-4 text-muted-foreground" } },
        1 => rsx! { XIcon { class: "size-4 text-muted-foreground" } },
        2 => rsx! { LinkedInIcon { class: "size-4 text-muted-foreground" } },
        3 => rsx! { MailIcon { class: "size-4 text-muted-foreground" } },
        4 => rsx! { CalendarIcon { class: "size-4 text-muted-foreground" } },
        _ => rsx! {},
    }
}
