use rocket::response::Redirect;

#[catch(401)]
pub fn unauthorized() -> Redirect {
    Redirect::to(uri!(crate::auth::routes::login: failed = false, logout = true))
}
