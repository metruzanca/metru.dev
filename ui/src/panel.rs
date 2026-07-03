use dioxus::prelude::*;

#[component]
pub fn Panel(
    class: Option<String>,
    children: Element,
) -> Element {
    let mut classes = String::from("rounded-xl border border-border bg-card p-6");
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
