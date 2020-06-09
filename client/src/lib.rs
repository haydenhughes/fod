#[macro_use]
extern crate seed;

mod components;
mod models;
mod pages;

use seed::prelude::*;

const LOGIN: &str = "login";
const CREATE_USER: &str = "create_user";

#[derive(Clone)]
pub enum Page {
    Login,
    CreateUser,
}

impl Page {
    fn init(url: &Url) -> Self {
        match url.path().last().map(|s| s.as_str()) {
            None => Page::Login,
            Some(LOGIN) => Self::Login,
            Some(CREATE_USER) => Self::CreateUser,
            _ => Self::Login,
        }
    }
}

pub struct Model {
    base_url: Url,
    page: Page,
    login: pages::login::Model,
    create_user: pages::create_user::Model,
}

#[derive(Clone)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),

    Login(pages::login::Msg),
    CreateUser(pages::create_user::Msg),
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn login(self) -> Url {
        self.base_url().add_path_part(LOGIN)
    }

    pub fn create_user(self) -> Url {
        self.base_url().add_path_part(CREATE_USER)
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        base_url: url.to_base_url(),
        login: pages::login::init(url.to_base_url()),
        create_user: pages::create_user::init(url.to_base_url()),
        page: Page::init(&url),
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(&url),

        Msg::Login(msg) => {
            pages::login::update(msg, &mut model.login, &mut orders.proxy(Msg::Login))
        }

        Msg::CreateUser(msg) => pages::create_user::update(
            msg,
            &mut model.create_user,
            &mut orders.proxy(Msg::CreateUser),
        ),
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    match &model.page {
        Page::Login => pages::login::view(&model.login).map_msg(Msg::Login),
        Page::CreateUser => pages::create_user::view(&model.create_user).map_msg(Msg::CreateUser),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}
