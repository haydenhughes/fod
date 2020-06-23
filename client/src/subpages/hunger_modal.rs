use seed::prelude::*;

pub struct Model {
    is_active: bool,
}

#[derive(Clone)]
pub enum Msg {
    Toggle,
}

pub fn init() -> Model {
    Model { is_active: false }
}

pub fn update(msg: Msg, model: &mut Model) {
    match msg {
        Msg::Toggle => model.is_active = !model.is_active,
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    let hunger_scale = vec![
        "Starvation, need to eat now, hunger pains, shaky, light headed",
        "Slight pain in stomach, hard to concentrate, lack of energy",
        "Beginning of physical signs of hunger, stomach growling sometimes",
        "Could eat if it were suggested",
        "Satisfied",
        "Feel food in stomach",
        "Stomach sticks out",
        "Bloated, clothes feel tight, sleepy and drained",
        "Definitely full, stomach uncomfortable, no energy, physically sick",
    ];
    div![
        class![
            "modal",
            "is-active" => model.is_active

        ],
        div![class!["modal-background"], simple_ev(Ev::Click, Msg::Toggle)],
        div![
            class!["modal-content"],
            div![
                class!["box"],
                h1![class!["title"], "Hunger Level Scale"],
                table![
                    class!["table", "is-hoverable"],
                    thead![tr![th!["Rating"], th!["Physical Sensation"]]],
                    tbody![hunger_scale.iter().enumerate().map(|(i, level)| {
                        tr![
                            class![
                                "is-selected" => i == 2 || i == 3 || i == 4 || i == 5
                            ],
                            td![(i + 1).to_string()],
                            td![level]
                        ]
                    })]
                ],
                p![strong!["Note:"], " the green ratings is the ideal zone"]
            ]
        ],
        button![
            class!["modal-close", "is-large"],
            attrs! { At::Type => "button", At::AriaLabel => "close" },
            simple_ev(Ev::Click, Msg::Toggle)
        ]
    ]
}
