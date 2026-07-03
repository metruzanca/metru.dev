use dioxus::prelude::*;
use ui::LabMeta;

pub const META: LabMeta = LabMeta {
    slug: "word-count",
    name: "Word Counter",
    description: "Paste text to count words, characters, and sentences.",
    tags: &["tool", "text"],
};

#[component]
pub fn App() -> Element {
    let mut text = use_signal(String::new);

    let word_count = use_memo(move || text.read().trim().split_whitespace().count());
    let char_count = use_memo(move || text.read().trim().chars().count());

    rsx! {
        div { class: "space-y-4",
            textarea {
                class: "w-full h-48 rounded-xl border border-border bg-card p-4 text-sm text-foreground placeholder:text-muted-foreground resize-y focus:border-primary/50 focus:outline-none",
                placeholder: "Paste or type text here...",
                oninput: move |e| text.set(e.value()),
            }
            div { class: "flex gap-6 font-mono text-sm",
                div {
                    span { class: "text-muted-foreground", "Words: " }
                    span { class: "text-primary font-semibold", "{word_count}" }
                }
                div {
                    span { class: "text-muted-foreground", "Characters: " }
                    span { class: "text-primary font-semibold", "{char_count}" }
                }
            }
        }
    }
}
