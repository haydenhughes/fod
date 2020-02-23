mod context;

use crate::auth::User;
use crate::models::Item;
use crate::FodMapDatabase;
use context::IndexContext;
use diesel::prelude::*;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;

type Response = Result<Template, Redirect>;
type Auth = Option<User>;

/// Generate a response type to handle login redirects
pub fn gen_response<T: serde::Serialize>(
    template: &'static str,
    user: &Auth,
    context: &T,
) -> Response {
    user.as_ref()
        .ok_or(Redirect::to(uri!(
            crate::auth::routes::login: failed = false
        )))
        .and(Ok(Template::render(template, context)))
}

#[get("/")]
pub fn index(conn: FodMapDatabase, user: Auth) -> Result<Response, diesel::result::Error> {
    Item::all()
        .get_results(&conn.0)
        .map(|items| IndexContext::new(items))
        .and_then(|context| Ok(gen_response("index", &user, &context)))
}
