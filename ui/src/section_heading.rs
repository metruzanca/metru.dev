use dioxus::prelude::*;

#[component]
pub fn SectionHeading(
    id: String,
    eyebrow: String,
    title: String,
    description: String,
) -> Element {
    rsx! {
        div {
            id: "{id}",
            class: "scroll-mt-20",
            p { class: "font-mono text-xs uppercase tracking-[0.2em] text-accent", "{eyebrow}" }
            h2 { class: "mt-2 font-display text-2xl font-bold uppercase tracking-tight text-foreground md:text-3xl", "{title}" }
            p { class: "mt-2 max-w-2xl text-pretty leading-relaxed text-muted-foreground", "{description}" }
        }
    }
}
