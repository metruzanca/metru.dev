use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum ButtonVariant {
    Default,
    Outline,
    Secondary,
    Ghost,
    Destructive,
    Link,
}

#[derive(Clone, PartialEq)]
pub enum ButtonSize {
    Default,
    Xs,
    Sm,
    Lg,
    Icon,
    IconXs,
    IconSm,
    IconLg,
}

#[component]
pub fn Button(
    variant: Option<ButtonVariant>,
    size: Option<ButtonSize>,
    class: Option<String>,
    onclick: Option<EventHandler<()>>,
    disabled: Option<bool>,
    aria_label: Option<String>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or(ButtonVariant::Default);
    let size = size.unwrap_or(ButtonSize::Default);
    let disabled = disabled.unwrap_or(false);

    let mut classes = String::from("inline-flex shrink-0 items-center justify-center rounded-lg border border-transparent bg-clip-padding text-sm font-medium whitespace-nowrap transition-all outline-none select-none");

    if matches!(variant, ButtonVariant::Link) {
        classes.push_str(" focus-visible:ring-0 focus-visible:border-0");
    } else {
        classes.push_str(" focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 active:not-aria-[haspopup]:translate-y-px");
    }

    if disabled {
        classes.push_str(" pointer-events-none opacity-50");
    }

    classes.push_str(" [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg]:size-4");

    match variant {
        ButtonVariant::Default => {
            classes.push_str(" bg-primary text-primary-foreground hover:bg-primary/80");
        }
        ButtonVariant::Outline => {
            classes.push_str(" border-border bg-background hover:bg-muted hover:text-foreground");
        }
        ButtonVariant::Secondary => {
            classes.push_str(" bg-secondary text-secondary-foreground hover:bg-secondary/80");
        }
        ButtonVariant::Ghost => {
            classes.push_str(" hover:bg-muted hover:text-foreground");
        }
        ButtonVariant::Destructive => {
            classes.push_str(" bg-destructive/20 text-destructive hover:bg-destructive/30 focus-visible:ring-destructive/40");
        }
        ButtonVariant::Link => {
            classes.push_str(" text-primary underline-offset-4 hover:underline");
        }
    }

    match size {
        ButtonSize::Default => {
            classes.push_str(" h-8 gap-1.5 px-2.5");
        }
        ButtonSize::Xs => {
            classes.push_str(" h-6 gap-1 rounded-md px-2 text-xs [&_svg]:size-3");
        }
        ButtonSize::Sm => {
            classes.push_str(" h-7 gap-1 rounded-md px-2.5 text-[0.8rem] [&_svg]:size-3.5");
        }
        ButtonSize::Lg => {
            classes.push_str(" h-9 gap-1.5 px-2.5");
        }
        ButtonSize::Icon => {
            classes.push_str(" size-8");
        }
        ButtonSize::IconXs => {
            classes.push_str(" size-6 rounded-md [&_svg]:size-3");
        }
        ButtonSize::IconSm => {
            classes.push_str(" size-7 rounded-md");
        }
        ButtonSize::IconLg => {
            classes.push_str(" size-9");
        }
    }

    if let Some(c) = class {
        classes.push(' ');
        classes.push_str(&c);
    }

    let aria = aria_label.clone();

    match onclick {
        Some(handler) => {
            rsx! {
                button {
                    class: classes,
                    onclick: move |_| handler.call(()),
                    disabled,
                    "aria-label": aria.clone().unwrap_or_default(),
                    {children}
                }
            }
        }
        None => {
            rsx! {
                button {
                    class: classes,
                    disabled,
                    "aria-label": aria.unwrap_or_default(),
                    {children}
                }
            }
        }
    }
}
