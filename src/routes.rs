use crate::auth::User;
use rocket::response::Redirect;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

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
pub fn index(user: Auth) -> Response {
    let context: HashMap<String, String> = HashMap::new();

    gen_response("index", &user, &context)
}
