use crate::Urls;
use seed::prelude::*;

pub fn feather_icon<Msg>(icon: &str, width: Option<u32>, height: Option<u32>) -> Node<Msg> {
    custom![
        Tag::from("feather-icon"),
        attrs! {
            At::from("icon") => icon,
            At::Width => {
                if let Some(width) = width {
                    AtValue::Some(width.to_string())
                } else {
                    AtValue::Ignored
                }},
            At::Height => if let Some(height) = height { AtValue::Some(height.to_string()) } else { AtValue::Ignored },
        }
    ]
}

pub fn form_reset<Msg: 'static + Clone>(on_click: Msg) -> Node<Msg> {
    div![
        class!["control"],
        button![
            attrs! {At::Type => "reset"},
            class!["button", "is-link", "is-light"],
            simple_ev(Ev::Click, on_click),
            "Reset"
        ]
    ]
}

pub fn notification<Msg: 'static + Clone>(text: &str, class: &str, on_delete: Msg) -> Node<Msg> {
    div![
        class!["notification", class],
        button![class!["delete"], simple_ev(Ev::Click, on_delete)],
        text
    ]
}

pub fn nav<Msg>(base_url: &Url) -> Node<Msg> {

    nav![
        class!["navbar"],
        attrs! {
            At::AriaRoleDescription => "navigation",
            At::AriaLabel => "main navigation",
        },
        div![
            class!["navbar-brand"],
            a![class!["navbar-item"], attrs! { At::Href => "/" }, "Fodmap"],
        ],
        div![
            class!["navbar-menu"],
            div![
                class!["navbar-start"],
                a![
                    class!["navbar-item"],
                    "Metrics"
                ],
            ],
            div![
                class!["navbar-end"],
                button![
                    class!["navbar-item", "button", "is-outlined", "is-warning", "is-inverted"],
                    "Logout"
                ]
            ]
        ]
    ]
}
