use super::{Model, Msg};
use crate::components::{feather_icon, form_reset};
use seed::prelude::*;

pub fn create_user_inputs(model: &Model) -> Vec<Node<Msg>> {
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
                        "is-danger" => model.is_taken
                    ],
                    attrs! {
                        At::Type => "text",
                        At::Placeholder => "User Name",
                        At::AutoFocus => true.as_at_value()
                    },
                    input_ev(Ev::Input, Msg::UserNameChanged)
                ],
            ],
            if model.is_taken {
                p![class!["help", "is-danger"], "User name already taken!"]
            } else {
                empty![]
            }
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
                    class!["input"],
                    attrs! {
                        At::Type => "password",
                        At::Placeholder => "Password"
                    },
                    input_ev(Ev::Input, Msg::PasswordChanged)
                ],
            ]
        ],
        div![
            class!["field"],
            label![class!["label"], "Verify Password"],
            div![
                class!["control", "has-icons-left"],
                span![
                    class!["icon", "is-small", "is-left"],
                    feather_icon("lock", None, None)
                ],
                input![
                    class![
                        "input",
                        "is-danger" => !model.is_verified
                    ],
                    attrs! {
                        At::Type => "password",
                        At::Placeholder => "Verify Password"
                    },
                    input_ev(Ev::Input, Msg::VerifyPasswordChanged)
                ],
            ],
            if !model.is_verified {
                p![class!["help", "is-danger"], "Passwords are not the same"]
            } else {
                empty![]
            }
        ]
    ]
}

pub fn create_user_controls() -> Node<Msg> {
    div![
        class!["field", "is-grouped"],
        div![
            class!["control"],
            input![
                attrs! {At::Type => "submit", At::Value => "Create"},
                class!["button", "is-link"],
                ev(Ev::Click, |event| {
                    event.prevent_default();
                    Msg::CreateClicked
                })
            ]
        ],
        form_reset(Msg::ResetClicked)
    ]
}
