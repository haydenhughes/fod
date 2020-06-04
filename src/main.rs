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
            "/auth/api",
            routes![
                routes::create_session,
                routes::delete_session,
                routes::create_user
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
