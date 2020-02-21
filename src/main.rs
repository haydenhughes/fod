#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
extern crate bcrypt;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod form;
mod models;
mod routes;
mod schema;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

#[database("fodmap")]
pub struct FodMapDatabase(diesel::PgConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![routes::index, routes::login])
        .mount(
            "/static",
            StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
        )
        .attach(Template::fairing())
        .attach(FodMapDatabase::fairing())
        .launch();
}
