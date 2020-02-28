mod app;

use std::sync::Arc;
use serde::Serialize;
use tera::{Context, Tera};
use warp::Filter;
use warp::Reply;

pub struct Template<T: Serialize> {
    name: &'static str,
    context: T,
}

impl<T: serde::Serialize> Template<T> {
    pub fn new(name: &'static str, context: T) -> Self {
        Template { name, context }
    }

    pub fn render(self, tera: &Tera) -> impl Reply {
        let render = Context::from_serialize(self.context)
            .and_then(|context| tera.render(self.name, &context))
            .unwrap_or_else(|err| err.to_string());

        warp::reply::html(render)
    }
}

pub fn routes(tera: Arc<Tera>) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    app::index()
}
