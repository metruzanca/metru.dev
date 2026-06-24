use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum BadgeVariant {
    Online,
    New,
    Beta,
    Pro,
    Offline,
}

#[component]
pub fn Badge(
    variant: Option<BadgeVariant>,
    class: Option<String>,
    children: Element,
) -> Element {
    let variant = variant.unwrap_or(BadgeVariant::Online);

    let variant_classes = match variant {
        BadgeVariant::Online => "border-accent/40 bg-accent/10 text-accent",
        BadgeVariant::New => "border-primary/40 bg-primary/10 text-primary",
        BadgeVariant::Beta => "border-neon-purple/40 bg-neon-purple/10 text-neon-purple",
        BadgeVariant::Pro => "border-neon-amber/40 bg-neon-amber/10 text-neon-amber",
        BadgeVariant::Offline => "border-border bg-muted text-muted-foreground",
    };

    let mut classes = String::from("inline-flex items-center gap-1.5 rounded-full border px-3 py-1 font-mono text-xs uppercase tracking-widest");
    classes.push(' ');
    classes.push_str(variant_classes);

    if let Some(c) = class {
        classes.push(' ');
        classes.push_str(&c);
    }

    rsx! {
        span {
            class: classes,
            span { class: "size-1.5 rounded-full bg-current" }
            {children}
        }
    }
}
