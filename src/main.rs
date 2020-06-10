//! # FodMap
//!
//! Provides a binary to run the FodMap server which hooks into the functions exposed by
//! `fodmap_server` and serves the static files out of `fodmap_client`.

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate fodmap_server;
extern crate rocket_contrib;

use fodmap_server::{routes, FodmapDbConn};
use rocket_contrib::serve::Options;
use rocket_contrib::serve::StaticFiles;

// TODO: Default credentials so that the admin can create users

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
            "/api/entries",
            routes![
                routes::entries::list_entries,
                routes::entries::get_entry,
                routes::entries::create_entry
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
