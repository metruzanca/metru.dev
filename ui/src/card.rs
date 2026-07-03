use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum CardVariant {
    Default,
    Featured,
    Grid,
}

#[component]
pub fn Card(
    variant: Option<CardVariant>,
    class: Option<String>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or(CardVariant::Default);

    let mut classes = String::from("rounded-xl");

    match variant {
        CardVariant::Default => {
            classes.push_str(" border border-border bg-card p-6 transition-colors hover:border-primary/50");
        }
        CardVariant::Featured => {
            classes.push_str(" border border-primary/40 bg-card p-6 shadow-glow-pink");
        }
        CardVariant::Grid => {
            classes.push_str(" relative overflow-hidden border border-border bg-card p-6 synth-grid");
        }
    }

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
