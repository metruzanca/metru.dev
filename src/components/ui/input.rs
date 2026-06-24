use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum InputGlow {
    Pink,
    Cyan,
}

#[component]
pub fn Input(
    label: Option<String>,
    value: Signal<String>,
    glow: Option<InputGlow>,
    class: Option<String>,
    placeholder: Option<String>,
    id: String,
) -> Element {
    let glow = glow.unwrap_or(InputGlow::Pink);
    let placeholder = placeholder.unwrap_or_default();

    let mut input_classes = String::from(
        "h-10 w-full rounded-md border border-input bg-background px-3 text-sm text-foreground placeholder:text-muted-foreground outline-none transition-colors",
    );

    match glow {
        InputGlow::Pink => {
            input_classes.push_str(" focus:border-primary focus:shadow-glow-pink");
        }
        InputGlow::Cyan => {
            input_classes.push_str(" focus:border-accent focus:shadow-glow-cyan");
        }
    }

    if let Some(c) = class {
        input_classes.push(' ');
        input_classes.push_str(&c);
    }

    let label_element = label.map(|l| {
        rsx! {
            label {
                class: "font-mono text-xs uppercase tracking-widest text-muted-foreground",
                r#for: "{id}",
                "{l}"
            }
        }
    });

    rsx! {
        div { class: "flex flex-col gap-2",
            {label_element}
            input {
                id: "{id}",
                class: input_classes,
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |e| {
                    value.set(e.value());
                },
            }
        }
    }
}
