#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate fodmap_server;
extern crate rocket_contrib;

use fodmap_server::{routes, FodmapDbConn};
use rocket_contrib::serve::Options;
use rocket_contrib::serve::StaticFiles;

fn main() {
    rocket::ignite()
        .attach(FodmapDbConn::fairing())
        .mount(
            "/api/auth",
            routes![
                routes::auth::create_session,
                routes::auth::delete_session,
                routes::auth::create_user
            ],
        )
        .mount(
            "/api/meal_types",
            routes![
                routes::meal_types::list_meal_types,
                routes::meal_types::get_meal_type,
                routes::meal_types::update_meal_type,
                routes::meal_types::delete_meal_type
            ],
        )
        .mount(
            "/login",
            StaticFiles::new("client/dist", Options::NormalizeDirs | Options::Index),
        )
        .mount(
            "/create_user",
            StaticFiles::new("client/dist", Options::NormalizeDirs | Options::Index),
        )
        .launch();
}
