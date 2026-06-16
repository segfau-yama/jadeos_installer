use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum TypographyTag {
    Div,
    Span,
    P,
    H1,
    H2,
    H3,
    Label,
    Code,
}

#[derive(Props, Clone, PartialEq)]
pub struct TypographyProps {
    #[props(default = TypographyTag::Div)]
    tag: TypographyTag,
    #[props(default = String::new())]
    class: String,
    children: Element,
}

#[component]
pub fn Typography(props: TypographyProps) -> Element {
    match props.tag {
        TypographyTag::Div => rsx! {
            div {
                class: "{props.class}",
                {props.children}
            }
        },
        TypographyTag::Span => rsx! {
            span {
                class: "{props.class}",
                {props.children}
            }
        },
        TypographyTag::P => rsx! {
            p {
                class: "{props.class}",
                {props.children}
            }
        },
        TypographyTag::H1 => rsx! {
            h1 {
                class: "{props.class}",
                {props.children}
            }
        },
        TypographyTag::H2 => rsx! {
            h2 {
                class: "{props.class}",
                {props.children}
            }
        },
        TypographyTag::H3 => rsx! {
            h3 {
                class: "{props.class}",
                {props.children}
            }
        },
        TypographyTag::Label => rsx! {
            label {
                class: "{props.class}",
                {props.children}
            }
        },
        TypographyTag::Code => rsx! {
            code {
                class: "{props.class}",
                {props.children}
            }
        },
    }
}
