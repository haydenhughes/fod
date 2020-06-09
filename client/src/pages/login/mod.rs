mod components;

use fodmap_common::Session;
use crate::Urls;
use seed::prelude::*;

#[derive(Clone)]
pub struct Model {
    base_url: Url,
    user_name: String,
    password: String,
    is_valid: bool,
}

#[derive(Clone)]
pub enum Msg {
    // Inputs
    UserNameChanged(String),
    PasswordChanged(String),

    // Buttons
    LoginClicked,
    ResetClicked,
    DeleteNotification,

    // Server interactions
    LoginSucceded,
    LoginFailed,
}

pub fn init(url: Url) -> Model {
    Model {
        base_url: url,
        user_name: String::default(),
        password: String::default(),
        is_valid: true,
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UserNameChanged(user_name) => model.user_name = user_name,
        Msg::PasswordChanged(password) => model.password = password,

        Msg::LoginClicked => {
            orders.skip();

            let request = Request::new("/api/auth/sessions")
                .method(Method::Post)
                .json(&Session {
                    name: &model.user_name,
                    password: &model.password,
                })
                .expect("Unable to serialize data");

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("Login request failed");

                if response.status().is_ok() {
                    Msg::LoginSucceded
                } else {
                    Msg::LoginFailed
                }
            });
        }
        Msg::ResetClicked => {
            model.user_name = String::default();
            model.password = String::default();
        }
        Msg::DeleteNotification => model.is_valid = true,

        Msg::LoginSucceded => Urls::new(model.base_url).home().go_and_load(),
        Msg::LoginFailed => model.is_valid = false,
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    section![
        if !model.is_valid {
            crate::components::notification(
                "Invalid user name or password",
                "is-danger",
                Msg::DeleteNotification,
            )
        } else {
            empty![]
        },
        class!["hero", "is-fullheight"],
        div![
            class!["hero-body"],
            div![
                class!["container"],
                h1![class!["title"], "FodMap"],
                h2![class!["subtitle"], "The self hosted food tracker"],
                form![
                    components::login_inputs(model),
                    components::login_controls()
                ]
            ]
        ]
    ]
}
