use dioxus::prelude::*;

#[component]
pub fn HowIWork() -> Element {
    rsx! {
        main { class: "mx-auto max-w-3xl px-4 pt-14 pb-24 md:pt-20",
            header { class: "mb-12",
                h1 { class: "font-display text-4xl font-extrabold uppercase text-foreground md:text-6xl",
                    span { class: "text-accent text-glow-cyan", "> " }
                    "how I work"
                }
                div { class: "mt-4 flex flex-wrap items-center gap-3 font-mono text-xs text-muted-foreground",
                    span { class: "rounded-md border border-border px-2 py-0.5",
                        "version 1.0 \u{2014} June 2026"
                    }
                }
                p { class: "mt-4 max-w-xl font-mono text-sm text-muted-foreground",
                    "Inspired by "
                    a {
                        class: "text-accent hover:underline",
                        href: "https://den.dev/how-i-work/",
                        target: "_blank",
                        rel: "noreferrer",
                        "Den Delimarsky"
                    }
                    " and "
                    a {
                        class: "text-accent hover:underline",
                        href: "https://github.com/hepwori/wwi/",
                        target: "_blank",
                        rel: "noreferrer",
                        "Isaac Hepworth"
                    }
                    ". This is a living document that describes how I operate\u{2014}it's context for teammates, not a prescription for how you should work."
                }
            }

            section { class: "mb-16",
                h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                    span { class: "text-primary text-glow-pink", ">\u{00A0}" }
                    "Principles"
                }

                p { class: "mb-5 font-mono text-xs text-muted-foreground",
                    "The ideas that guide how I approach building software and working with others."
                }

                div { class: "rounded-xl border border-border bg-card divide-y divide-border overflow-hidden",

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "Ship in phases: work, pretty, fast."
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "First, get the infrastructure and data flowing correctly. Then layer in the UI and polish the experience. Finally, profile and optimize. Don't chase perfection up front\u{2014}deliver value incrementally."
                        }
                    }

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "Bias for action."
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "I'd rather build a quick prototype and learn from it than debate in the abstract. When there's disagreement, I build a proof-of-concept and put it in front of people. The best product wins, not my ego."
                        }
                    }

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "UX and DX are the same thing."
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "A feature isn't done until both the end user and the next developer can use it. I care about clean APIs, clear error messages, and readable code as much as pixel-perfect interfaces."
                        }
                    }

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "Feedback is how we get better."
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "I welcome direct feedback and give it in return. If something bothers me or has room for improvement, I'll have that conversation. Radical candor matters\u{2014}be kind, but be clear."
                        }
                    }

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "No blame, just prevent recurrence."
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "When things break, I run blameless postmortems. The goal isn't finding who broke what. It's understanding the chain of events, writing a test that catches it next time, and moving on. Systems fail, not people."
                        }
                    }

                    div { class: "px-6 py-5",
                        h3 { class: "font-mono text-sm font-semibold text-accent",
                            "Have fun solving problems."
                        }
                        p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                            "Every project has something fun about it\u{2014}a puzzle, a constraint, a chance to learn. I find that thing and lean into it. If I'm not learning, I'm stagnating."
                        }
                    }
                }
            }

            section { class: "mb-16",
                h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                    span { class: "text-primary text-glow-pink", ">\u{00A0}" }
                    "Notes"
                }

                p { class: "mb-5 font-mono text-xs text-muted-foreground",
                    "The practical stuff\u{2014}how I communicate, structure my time, and prefer to collaborate."
                }

                div { class: "rounded-xl border border-border bg-card divide-y divide-border overflow-hidden",

                    div { class: "px-6 py-4",
                        p { class: "text-sm text-foreground font-medium",
                            "Communication"
                        }
                        ul { class: "mt-2 list-disc pl-4 space-y-1.5 text-sm text-muted-foreground leading-relaxed",
                            li { "Documentation takes precedence over meetings. If something isn't written down, it might as well not exist." }
                            li { "I set up meetings to hash out ideas and build partnerships\u{2014}not for status updates. That can be async." }
                            li { "Before scheduling a meeting, ask: \u{201C}Can I write this down and will it be useful for others?\u{201D}" }
                            li { "For business decisions, I prefer email or a shared doc\u{2014}paper trails matter. For day-to-day coordination, whatever the team prefers." }
                            li { "Most email is not urgent. If something is truly urgent, reach out through faster channels." }
                            li { "I don't answer emails or attend meetings during vacation. I encourage the same." }
                        }
                    }

                    div { class: "px-6 py-4",
                        p { class: "text-sm text-foreground font-medium",
                            "Focus and time"
                        }
                        ul { class: "mt-2 list-disc pl-4 space-y-1.5 text-sm text-muted-foreground leading-relaxed",
                            li { "Mornings are for deep work\u{2014}creative tasks, complex problems. Afternoons are for reviews, emails, and coordination." }
                            li { "I work on a single monitor to reduce visual noise and stay in flow." }
                            li { "I work within a typical day. Evenings and weekends are for family, side projects, and recharging." }
                            li { "If you get an email from me late at night, I'm triaging my inbox\u{2014}not expecting an immediate response." }
                            li { "I book regular time for self-development and encourage others to do the same." }
                        }
                    }

                    div { class: "px-6 py-4",
                        p { class: "text-sm text-foreground font-medium",
                            "Shipping"
                        }
                        ul { class: "mt-2 list-disc pl-4 space-y-1.5 text-sm text-muted-foreground leading-relaxed",
                            li { "Before writing code, I clarify acceptance criteria. If it's not clear, I go ask. 5 minutes now saves a day of waste." }
                            li { "I prefer milestone-based work with defined scope. Smaller, shippable increments build trust and momentum." }
                            li { "I stick to the top-right corner of the Impact/Effort matrix. High signal, low drag." }
                            li { "Every incident gets a postmortem with concrete follow-ups that turn into actual tickets." }
                        }
                    }

                    div { class: "px-6 py-4",
                        p { class: "text-sm text-foreground font-medium",
                            "Tooling"
                        }
                        ul { class: "mt-2 list-disc pl-4 space-y-1.5 text-sm text-muted-foreground leading-relaxed",
                            li {
                                "I live in the terminal. My editor is "
                                a {
                                    class: "text-accent hover:underline",
                                    href: "https://zed.dev",
                                    target: "_blank",
                                    rel: "noreferrer",
                                    "Zed"
                                }
                                "\u{2014}native, fast, no Electron. My AI coding tool is "
                                a {
                                    class: "text-accent hover:underline",
                                    href: "https://github.com/anomalyco/opencode",
                                    target: "_blank",
                                    rel: "noreferrer",
                                    "OpenCode"
                                }
                                ", running locally with whatever model I want via OpenRouter."
                            }
                            li { "I use AI heavily for boilerplate, exploration, and parallel agents. But I write critical code by hand. If it touches billing, auth, or data integrity, I know every line." }
                            li { "I use git worktrees to juggle multiple features simultaneously. Each branch gets its own directory and mental space." }
                            li { "Compiled languages are a forcing function for quality. If it compiles, there's a lower chance of runtime surprises." }
                        }
                    }

                    div { class: "px-6 py-4",
                        p { class: "text-sm text-foreground font-medium",
                            "Collaboration"
                        }
                        ul { class: "mt-2 list-disc pl-4 space-y-1.5 text-sm text-muted-foreground leading-relaxed",
                            li { "I thrive in small teams where I wear multiple hats. I'd rather own a feature end-to-end than be siloed." }
                            li { "I don't micromanage. Once we've agreed on scope, I trust you to execute and will be here if you're blocked." }
                            li { "My door is always open. Blocked? Need clarification? Have a question? Ask me." }
                            li { "I value honesty, inclusiveness, and directness. If I'm missing the mark, call me out on it." }
                            li { "I have zero tolerance for toxicity. Rude or dismissive behavior doesn't fly." }
                            li { "Health and family always come first. Go take care of what matters. Work will be here when you're back." }
                        }
                    }
                }
            }

            section { class: "mb-16",
                h2 { class: "font-heading text-2xl font-bold tracking-tight text-foreground mb-6",
                    span { class: "text-primary text-glow-pink", ">\u{00A0}" }
                    "Inspiration"
                }

                p { class: "mb-5 font-mono text-xs text-muted-foreground",
                    "Some of my favorite reads that shaped how I think about building software."
                }

                div { class: "grid grid-cols-1 gap-3 sm:grid-cols-2",
                    {
                        let links: Vec<(&str, &str, &str)> = vec![
                            ("Simplicity, Please", "A manifesto for reducing complexity in software development.", "https://www.infoq.com/articles/simplicity-manifesto-development/"),
                            ("No Hello Club", "Skip the pleasantries and get to the point in chat.", "https://nohello.club/"),
                            ("37signals", "Opinionated takes on business, design, and staying small.", "https://37signals.com/"),
                            ("Software Craftsmanship", "Raising the bar for professional software development.", "https://manifesto.softwarecraftsmanship.org/"),
                            ("Agile Manifesto", "The foundational document for agile software development.", "https://agilemanifesto.org/"),
                        ];
                        links.into_iter().map(|(title, description, url)| {
                            rsx! {
                                a {
                                    class: "group rounded-xl border border-border bg-card p-5 transition-colors hover:border-accent/40",
                                    href: "{url}",
                                    target: "_blank",
                                    rel: "noreferrer",
                                    h4 { class: "font-mono text-sm font-semibold text-accent group-hover:underline",
                                        "{title}"
                                    }
                                    p { class: "mt-1 text-sm text-muted-foreground leading-relaxed",
                                        "{description}"
                                    }
                                }
                            }
                        })
                    }
                }
            }

            footer { class: "border-t border-border px-4 py-8",
                div { class: "flex flex-col items-start justify-between gap-2 sm:flex-row sm:items-center",
                    p { class: "font-mono text-xs text-muted-foreground",
                        span { class: "text-primary", "$" }
                        " built with synthwave \u{2014} \u{a9} 2026 Sam"
                    }
                }
            }
        }
    }
}
