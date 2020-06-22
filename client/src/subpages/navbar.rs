use crate::Urls;
use seed::prelude::*;

pub struct Model {
    base_url: Url,
    is_hamburger_active: bool,
}

#[derive(Clone)]
pub enum Msg {
    Logout,
    LoggedOut,
    ToggleHamburger,
}

pub fn init(url: Url) -> Model {
    Model {
        base_url: url,
        is_hamburger_active: false,
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleHamburger => model.is_hamburger_active = !model.is_hamburger_active,
        Msg::Logout => {
            let request = Request::new("/api/auth/logout").method(Method::Delete);

            orders.perform_cmd(async {
                fetch(request).await.expect("Unable to logout");
                Msg::LoggedOut
            });
        }
        Msg::LoggedOut => Urls::new(&model.base_url).login().go_and_load(),
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let mut hamburger = nodes![];
    hamburger.resize(3, span![attrs! { At::Hidden => "true" }]);
    nav![
        class!["navbar"],
        attrs! {
            At::AriaRoleDescription => "navigation",
            At::AriaLabel => "main navigation",
        },
        div![
            class!["navbar-brand"],
            a![
                class!["navbar-item"],
                attrs! { At::Href => Urls::new(&model.base_url).home() },
                "Fodmap"
            ],
            a![
                class![
                    "navbar-burger",
                    "is-active" => model.is_hamburger_active
                ],
                attrs! {
                    At::AriaRoleDescription => "button",
                    At::AriaLabel => "menu",
                    At::AriaExpanded => "false"
                },
                simple_ev(Ev::Click, Msg::ToggleHamburger),
                hamburger
            ]
        ],
        div![
            class![
                "navbar-menu",
                "is-active" => model.is_hamburger_active
            ],
            div![
                class!["navbar-start"],
                a![
                    class!["navbar-item"],
                    attrs! { At::Href => Urls::new(&model.base_url).metrics() },
                    "Metrics"
                ]
            ],
            div![
                class!["navbar-end"],
                div![
                    class!["navbar-item"],
                    div![
                        class!["buttons"],
                        button![
                            class!["navbar-item", "button"],
                            simple_ev(Ev::Click, Msg::Logout),
                            "Logout"
                        ]
                    ]
                ]
            ]
        ]
    ]
}
