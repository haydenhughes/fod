mod components;

use crate::Urls;
use crate::components::nav;
use chrono::NaiveDateTime;
use fodmap_common::Entry;
use seed::prelude::*;

pub struct Model {
    base_url: Url,
    entries: Option<Vec<Entry>>,
    page: usize,
    limit: usize,
    before: Option<NaiveDateTime>,
}

#[derive(Clone)]
pub enum Msg {
    Logout,
    LoggedOut,
    UpdateEntries,
    RecvEntries((fetch::Status, Vec<Entry>)),
}

pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Model {
    // orders.send_msg(Msg::UpdateEntries);

    Model {
        base_url: url,
        entries: None,
        page: 1,
        limit: 20,
        before: None,
    }
}

pub fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Logout => {
            let request = Request::new("/api/auth/logout").method(Method::Delete);

            orders.perform_cmd(async {
                fetch(request).await.expect("Unable to logout");
                Msg::LoggedOut
            });
        }
        Msg::LoggedOut => Urls::new(&model.base_url).login().go_and_load(),
        Msg::UpdateEntries => {
            orders.skip();

            let request = Request::new("/api/entries").method(Method::Get);

            orders.perform_cmd(async {
                let response = fetch(request).await.expect("Unable to fetch entries");
                Msg::RecvEntries((
                    response.status(),
                    response.json().await.unwrap_or(Vec::new()),
                ))
            });
        }
        Msg::RecvEntries((status, entries)) => {
            if status.is_ok() {
                model.entries = Some(entries)
            } else {
                Urls::new(&model.base_url).login().go_and_load();
            }
        }
    }
}

pub fn view(model: &Model) -> Node<Msg> {
    nav(&model.base_url)

}
