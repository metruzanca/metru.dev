use dioxus::prelude::*;

#[component]
pub fn Toggle(
    checked: Signal<bool>,
    class: Option<String>,
) -> Element {
    let mut classes = String::from(
        "relative h-6 w-11 shrink-0 rounded-full border transition-colors",
    );

    let toggled = checked();

    if toggled {
        classes.push_str(" border-primary bg-primary/30 shadow-glow-pink");
    } else {
        classes.push_str(" border-border bg-muted");
    }

    let knob_classes = if toggled {
        "absolute top-0.5 size-4 rounded-full transition-all left-[22px] bg-primary"
    } else {
        "absolute top-0.5 size-4 rounded-full transition-all left-0.5 bg-muted-foreground"
    };

    if let Some(c) = class {
        classes.push(' ');
        classes.push_str(&c);
    }

    rsx! {
        button {
            r#type: "button",
            role: "switch",
            "aria-checked": "{toggled}",
            "aria-label": "Toggle",
            class: classes,
            onclick: move |_| {
                let current = *checked.read();
                *checked.write() = !current;
            },
            span {
                class: knob_classes,
            }
        }
    }
}
