use dioxus::prelude::*;

use crate::github;
use crate::utils;

#[component]
pub fn About() -> Element {
    rsx! {
        main { class: "mx-auto max-w-3xl px-4 pt-14 pb-24 md:pt-20",
            header { class: "mb-12",
                h1 { class: "font-display text-4xl font-extrabold uppercase text-foreground md:text-6xl",
                    span { class: "text-primary text-glow-pink", "> " }
                    "About"
                }
            }

            section { class: "mb-16",
                p { class: "text-lg text-muted-foreground leading-relaxed",
                    "I grew up in Italy, where I fell in love with software through video games. I wanted to make them and ended up discovering I just liked writing all software. I went to a technical school and wrote my first line of code at 14, building a foundation that led me into a career as a full-stack engineer."
                }
                p { class: "mt-4 text-lg text-muted-foreground leading-relaxed",
                    "I spent my first two years in the industry working at an aerospace company in Milan, building flight simulators and diagnostic tools in C# and React. During COVID I decided to take a chance on my career and move to the United States for better opportunities. All of the startups and innovations I wanted to be part of were happening here. While moving, I found a role and haven't looked back."
                }
                p { class: "mt-4 text-lg text-muted-foreground leading-relaxed",
                    "Since then I've worked across ad tech, restaurant platforms, and crypto compliance, wearing hats from frontend to backend to design systems to DevOps. I'm currently based in Jersey City, building things with Rust, Go, TypeScript, and React."
                }
            }

            section { class: "mb-16",
                h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-8",
                    span { class: "text-accent text-glow-cyan", ">\u{00A0}" }
                    "Timeline"
                }

                div { class: "flex flex-col gap-10",

                    CryptoExitNote {}

                    TimelineEntry {
                        position: "Senior Engineer (Contract)",
                        company: "Predicate.io",
                        company_url: "https://predicate.io",
                        location: "",
                        start_date: "2025-06",
                        end_date: "2025-12",
                        narrative: "Built performant Go APIs and async background job processing for a crypto policy verification startup. Led full-stack development across React and Go, and forked the core policy engine to build a transaction simulation system, letting clients test policy changes against real data before deploying them.",
                    }

                    TimelineEntry {
                        position: "Senior Engineer",
                        company: "Chainalysis",
                        company_url: "https://chainalysis.com",
                        location: "Remote",
                        start_date: "2024-03",
                        end_date: "2025-06",
                        narrative: "Was brought on to a product with no dedicated frontend engineers and became the de facto lead, cutting over 50K lines of legacy code while shipping new features. Contributed heavily to the company-wide design system, an internal open source project where I owned features, reviewed PRs, and coordinated a near-zero-downtime rollout across three product teams. Customers included Coinbase and Crypto.com.",
                    }

                    TimelineEntry {
                        position: "Software Engineer (Contract)",
                        company: "Bentobox",
                        company_url: "https://getbento.com",
                        location: "New York City",
                        start_date: "2023-04",
                        end_date: "2023-12",
                        narrative: "Shipped 50+ pixel-perfect components from Figma designs and resolved critical race conditions affecting thousands of users. Reduced tech debt by 30% while aligning specs across five teams.",
                    }

                    TimelineEntry {
                        position: "Full Stack Engineer",
                        company: "Brkfst.io",
                        company_url: "https://brkfst.io",
                        location: "New York",
                        start_date: "2021-04",
                        end_date: "2023-03",
                        narrative: "Refactored legacy Node.js services into modular REST APIs, built a Playwright testing suite, and led the migration from Webpack to modern tooling. Designed AWS infrastructure with Terraform and established CI/CD pipelines.",
                    }

                    TimelineEntry {
                        position: "Full Stack Engineer",
                        company: "TXT Group",
                        company_url: "http://txtgroup.com",
                        location: "Milan, Italy",
                        start_date: "2019-10",
                        end_date: "2021-04",
                        narrative: "Led development of a React/TypeScript web control panel for remotely operating flight simulators for Leonardo SpA. Built C# diagnostic tools and a turbulence data processing app in collaboration with IATA. Mentored developers learning React and TypeScript.",
                    }
                }
            }

            section { class: "mb-16",
                h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                    span { class: "text-accent text-glow-cyan", ">\u{00A0}" }
                    "Projects I've Built"
                }

                p { class: "mb-6 font-mono text-sm text-muted-foreground",
                    "A few things I'm proud of. Most run in the terminal, because that's where I live."
                }

                div { class: "flex flex-col gap-6",
                    ProjectEntry {
                        slug: "squeal",
                        description: "A terminal-native database viewer built in Rust. I made this because I moved to Zed and missed having a database panel in my editor. It runs in the terminal, stays fast and light, and scratches my own itch perfectly.",
                    }
                    ProjectEntry {
                        slug: "squawk",
                        description: "A Postman-like REST client for the terminal, also in Rust. Still in progress. The idea is the same as Squeal: move tools into the terminal that other people run as Electron apps.",
                    }
                    ProjectEntry {
                        slug: "metru.dev",
                        description: "This site, built with Dioxus and Rust, deployed as a full-stack WASM app. It's my playground for learning Rust, exploring full-stack WASM, and writing about what I discover along the way.",
                    }
                }
            }

            section { class: "mb-16",
                h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                    span { class: "text-accent text-glow-cyan", ">\u{00A0}" }
                    "Community"
                }

                p { class: "mb-6 font-mono text-sm text-muted-foreground",
                    "Where I show up, and the people I share ideas with."
                }

                div { class: "rounded-xl border border-border bg-card divide-y divide-border overflow-hidden",

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "In Person"
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "I go to Coffee & Code meetups in New York and Newark, and occasionally attend Luma events from companies like Vercel. I recently went to the Gleam Gathering, a conference for the Gleam programming language, and love talking shop with other engineers face to face."
                        }
                    }

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "Discord"
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "I run a small Discord community of developers, friends and friends of friends, where we talk about the industry, what we're building, and AI. I've helped several people there prepare for interviews, improve their portfolios, and land jobs in tech."
                        }
                    }

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "Open Source"
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "I've always loved open source but hadn't found the right project until Chainalysis, where the design system functioned as internal open source: PRs, code review, shared ownership. I'm now building in the open with "
                            a {
                                class: "text-accent hover:underline",
                                href: "/projects",
                                target: "_blank",
                                rel: "noreferrer",
                                "my own projects"
                            }
                            " and looking for more ways to contribute."
                        }
                    }
                }
            }

            section { class: "mb-16",
                h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                    span { class: "text-accent text-glow-cyan", ">\u{00A0}" }
                    "Find Me"
                }

                p { class: "mb-5 font-mono text-sm text-muted-foreground",
                    "The best ways to reach me or follow what I'm up to."
                }

                div { class: "rounded-xl border border-border bg-card p-6",
                    div { class: "flex flex-wrap gap-6",
                        {
                            let channels: Vec<(&str, &str, &str)> = vec![
                                ("email", "sam(at)zanca.dev", ""),
                                ("github", "@metruzanca", "https://github.com/metruzanca"),
                                ("x", "@metruzanca", "https://x.com/metruzanca"),
                                ("linkedin", "samuele-zanca", "https://linkedin.com/in/samuele-zanca"),
                                ("bluesky", "@metru.dev", "https://bsky.app/profile/metru.dev"),
                                ("calendar", "schedule a chat", "https://cal.com/samzanca/15min"),
                            ];
                            channels.into_iter().map(|(label, handle, url)| {
                                if url.is_empty() {
                                    rsx! {
                                        div { class: "group flex flex-col gap-1.5",
                                            span { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
                                                "{label}"
                                            }
                                            span { class: "font-mono text-xs text-foreground",
                                                "{handle}"
                                            }
                                        }
                                    }
                                } else {
                                    rsx! {
                                        a {
                                            class: "group flex flex-col gap-1.5",
                                            href: "{url}",
                                            target: "_blank",
                                            rel: "noreferrer",
                                            span { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
                                                "{label}"
                                            }
                                            span { class: "font-mono text-xs text-foreground transition-colors group-hover:text-accent",
                                                "{handle}"
                                            }
                                        }
                                    }
                                }
                            })
                        }
                    }
                }
            }

            footer { class: "border-t border-border px-4 py-8",
                div { class: "flex flex-col items-start justify-between gap-2 sm:flex-row sm:items-center",
                    p { class: "font-mono text-xs text-muted-foreground",
                        span { class: "text-primary", "$" }
                        " built with synthwave \u{b7} \u{a9} 2026 Sam"
                    }
                }
            }
        }
    }
}

#[component]
fn TimelineEntry(
    position: String,
    company: String,
    company_url: String,
    location: String,
    start_date: String,
    end_date: String,
    narrative: String,
) -> Element {
    let date = utils::datetime::format_date_range(&start_date, &end_date);

    rsx! {
        div { class: "relative pl-6 border-l-2 border-border",
            div { class: "absolute -left-[5px] top-1.5 size-2 rounded-full bg-primary" }

            div { class: "flex flex-col sm:flex-row sm:items-baseline sm:justify-between gap-1",
                div {
                    h3 { class: "font-heading text-lg font-semibold text-foreground",
                        "{position}"
                    }
                    div { class: "flex items-center gap-2 flex-wrap",
                        if !company_url.is_empty() {
                            a {
                                class: "font-mono text-sm text-accent hover:underline",
                                href: "{company_url}",
                                target: "_blank",
                                rel: "noreferrer",
                                "{company}"
                            }
                        } else {
                            span { class: "font-mono text-sm text-accent",
                                "{company}"
                            }
                        }
                        if !location.is_empty() {
                            span { class: "font-mono text-xs text-muted-foreground",
                                "\u{00B7} {location}"
                            }
                        }
                    }
                }
                span { class: "font-mono text-xs text-muted-foreground shrink-0",
                    "{date}"
                }
            }

            if !narrative.is_empty() {
                p { class: "mt-3 text-sm text-muted-foreground leading-relaxed",
                    "{narrative}"
                }
            }
        }
    }
}

#[component]
fn CryptoExitNote() -> Element {
    let mut expanded = use_signal(|| false);

    rsx! {
        div { class: "relative pl-6",
            div { class: "rounded-xl border border-border bg-card px-5 py-4 cursor-pointer transition-colors hover:border-accent/30",
                onclick: move |_| expanded.with_mut(|e| *e = !*e),
                div { class: "flex items-center gap-2",
                    span { class: "font-mono text-xs uppercase tracking-widest text-muted-foreground transition-colors",
                        "leaving crypto"
                    }
                    span { class: "font-mono text-xs text-muted-foreground transition-transform",
                        if expanded() {
                            "\u{25BC}"
                        } else {
                            "\u{25B6}"
                        }
                    }
                }
                if expanded() {
                    p { class: "mt-3 text-sm text-muted-foreground leading-relaxed",
                        "After two crypto roles, I decided to leave the industry behind. While I like decentralized technologies, crypto isn't ready yet. There are too many bad actors and the space is still finding its footing. I wanted to work on something more grounded."
                    }
                }
            }
        }
    }
}

#[component]
fn ProjectEntry(slug: String, description: String) -> Element {
    let repos = github::all_repos();
    let repo = repos.iter().find(|r| r.name.eq_ignore_ascii_case(&slug));

    rsx! {
        div { class: "flex flex-col md:flex-row gap-5 items-start rounded-xl border border-border bg-card p-5",
            div { class: "shrink-0 w-full md:w-72",
                if let Some(repo) = repo {
                    crate::components::portfolio::projects::ProjectCard { repo: *repo }
                }
            }
            div { class: "flex-1",
                p { class: "text-sm text-muted-foreground leading-relaxed",
                    "{description}"
                }
            }
        }
    }
}
