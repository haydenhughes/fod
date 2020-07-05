//! # FodMap Client
//!
//! A web assembly frontend for the FodMap API.

#[macro_use]
extern crate seed;

mod components;
mod pages;
mod subpages;

use pages::*;
use seed::prelude::*;

const LOGIN: &str = "login";
const CREATE_USER: &str = "create_user";

pub enum Page {
    Login(login::Model),
    CreateUser(create_user::Model),
    Home(home::Model),
}

impl Page {
    fn init(url: &Url, orders: &mut impl Orders<Msg>) -> Self {
        match url.path().last().map(|s| s.as_str()) {
            None => Self::Home(home::init(url.to_base_url(), &mut orders.proxy(Msg::Home))),
            Some(LOGIN) => Self::Login(login::init(url.to_base_url())),
            Some(CREATE_USER) => Self::CreateUser(create_user::init(url.to_base_url())),
            _ => Self::Login(login::init(url.to_base_url())),
        }
    }

    fn login_mut(&mut self) -> Option<&mut login::Model> {
        match self {
            Self::Login(model) => Some(model),
            _ => None,
        }
    }

    fn create_user_mut(&mut self) -> Option<&mut create_user::Model> {
        match self {
            Self::CreateUser(model) => Some(model),
            _ => None,
        }
    }

    fn home_mut(&mut self) -> Option<&mut home::Model> {
        match self {
            Self::Home(model) => Some(model),
            _ => None,
        }
    }
}

pub struct Model {
    base_url: Url,
    page: Page,
}

#[derive(Clone)]
pub enum Msg {
    UrlChanged(subs::UrlChanged),

    Login(login::Msg),
    CreateUser(create_user::Msg),
    Home(home::Msg),
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn login(self) -> Url {
        self.base_url().add_path_part(LOGIN)
    }

    pub fn create_user(self) -> Url {
        self.base_url().add_path_part(CREATE_USER)
    }

    pub fn home(self) -> Url {
        self.base_url()
    }
}

fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    orders.subscribe(Msg::UrlChanged);
    Model {
        base_url: url.to_base_url(),
        page: Page::init(&url, orders),
    }
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::UrlChanged(subs::UrlChanged(url)) => model.page = Page::init(&url, orders),

        Msg::Login(msg) => login::update(
            msg,
            model.page.login_mut().unwrap(),
            &mut orders.proxy(Msg::Login),
        ),
        Msg::CreateUser(msg) => create_user::update(
            msg,
            model.page.create_user_mut().unwrap(),
            &mut orders.proxy(Msg::CreateUser),
        ),
        Msg::Home(msg) => home::update(
            msg,
            model.page.home_mut().unwrap(),
            &mut orders.proxy(Msg::Home),
        ),
    }
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    match &model.page {
        Page::Login(model) => login::view(model).map_msg(Msg::Login),
        Page::CreateUser(model) => create_user::view(model).map_msg(Msg::CreateUser),
        Page::Home(model) => home::view(model).map_msg(Msg::Home),
    }
}

#[wasm_bindgen(start)]
pub fn render() {
    App::start("app", init, update, view);
}
