use super::{Model, Msg};
use crate::components::{feather_icon, form_reset};
use seed::prelude::*;

pub fn login_inputs(model: &Model) -> Vec<Node<Msg>> {
    nodes![
        div![
            class!["field"],
            label![class!["label"], "User Name"],
            div![
                class!["control", "has-icons-left"],
                span![
                    class!["icon", "is-small", "is-left"],
                    feather_icon("user", None, None)
                ],
                input![
                    class![
                        "input",
                        "is-danger" => !model.is_valid
                    ],
                    attrs! {
                        At::Type => "text",
                        At::Placeholder => "User Name",
                        At::AutoFocus => true.as_at_value()
                    },
                    input_ev(Ev::Input, Msg::UserNameChanged)
                ],
            ],
        ],
        div![
            class!["field"],
            label![class!["label"], "Password"],
            div![
                class!["control", "has-icons-left"],
                span![
                    class!["icon", "is-small", "is-left"],
                    feather_icon("lock", None, None)
                ],
                input![
                    class![
                        "input",
                        "is-danger" => !model.is_valid
                    ],
                    attrs! {
                        At::Type => "password",
                        At::Placeholder => "Password"
                    },
                    input_ev(Ev::Input, Msg::PasswordChanged)
                ]
            ]
        ]
    ]
}


pub fn login_controls() -> Node<Msg> {
    div![
        class!["field", "is-grouped"],
        div![
            class!["control"],
            input![
                attrs! {At::Type => "submit", At::Value => "Login"},
                class!["button", "is-link"],
                ev(Ev::Click, |event| {
                    event.prevent_default();
                    Msg::LoginClicked
                }),
            ]
        ],
        form_reset(Msg::ResetClicked)
    ]
}
