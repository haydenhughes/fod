mod components;

use fodmap_common::Session;
use crate::Urls;
use seed::prelude::*;

#[derive(Clone)]
pub struct Model {
    base_url: Url,
    user_name: String,
    password: String,
    is_taken: bool,
    is_verified: bool,
}

impl Model {
    pub fn to_session(&self) -> Session {
        Session {
            name: &self.user_name,
            password: &self.password,
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    // Inputs
    UserNameChanged(String),
    PasswordChanged(String),
    VerifyPasswordChanged(String),

    // Buttons
    CreateClicked,
    ResetClicked,

    // Server interactions
    UserCreated(fetch::Status),
}

pub fn init(url: Url) -> Model {
    Model {
        base_url: url,
        user_name: String::default(),
        password: String::default(),
        is_taken: false,
        is_verified: true,
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        // inputs
        Msg::UserNameChanged(user_name) => {
            model.user_name = user_name;
            if model.is_taken {
                model.is_taken = false
            }
        }
        Msg::PasswordChanged(password) => model.password = password,
        Msg::VerifyPasswordChanged(password) => model.is_verified = model.password == password,

        // buttons
        Msg::ResetClicked => {
            model.user_name = String::default();
            model.password = String::default()
        }
        Msg::CreateClicked => {
            if model.is_verified
                && model.user_name != String::default()
                && model.password != String::default()
            {
                let request = {
                    Request::new("/api/auth/users")
                        .method(Method::Post)
                        .json(&model.to_session())
                        .expect("Unable to serialize data")
                };

                orders.perform_cmd(async {
                    Msg::UserCreated(
                        fetch(request)
                            .await
                            .expect("Create user HTTP request failed")
                            .status(),
                    )
                });
            }
        }

        // server responses
        Msg::UserCreated(status) => {
            model.is_taken = status.is_error();

            if !model.is_taken {
                Urls::new(&model.base_url).login().go_and_load();
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    section![
        class!["hero", "is-fullheight"],
        div![
            class!["hero-body"],
            div![
                class!["container"],
                h1![class!["title"], "Create User"],
                h2![class!["subtitle"], "Create a new user account"],
                form![
                    components::create_user_inputs(model),
                    components::create_user_controls()
                ]
            ]
        ]
    ]
}
