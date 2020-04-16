#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate clap;
extern crate rpassword;
extern crate serde;
extern crate chrono;

mod routes;
mod schema;
mod auth;
mod models;

use clap::{App, AppSettings, Arg, SubCommand};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rpassword::read_password_from_tty;
use std::{env, io};
use auth::NewUser;
use auth::User;

#[database("fodmap")]
pub struct FodMapDatabase(diesel::PgConnection);

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn main() -> io::Result<()> {
    let matches = App::new("FodMap")
        .version("0.1.0")
        .author("Hayden Hughes <hayden@firemail.cc>")
        .about("Self-hosted meal tracker")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(SubCommand::with_name("server").about("Run web server"))
        .subcommand(
            SubCommand::with_name("adduser")
                .about("Add a user account")
                .long_about("Requires the DATABASE_URL environment vairable to be set")
                .arg(
                    Arg::with_name("USERNAME")
                        .help("Sets the username for the new user")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("deluser")
                .about("Delete a user account")
                .long_about("Requires the DATABASE_URL environment vairable to be set")
                .arg(
                    Arg::with_name("USERNAME")
                        .help("Secify the username of the user to delete")
                        .required(true),
                ),
        )
        .get_matches();

    if matches.is_present("server") {
        rocket::ignite()
            .mount(
                "/",
                routes![
                    routes::index,
                    auth::routes::login,
                    auth::routes::user_login,
                    auth::routes::user_logout,
                ],
            )
            .mount(
                "/static",
                StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")),
            )
            .attach(Template::fairing())
            .attach(FodMapDatabase::fairing())
            .launch();
    }

    if matches.is_present("adduser") {
        let conn = establish_connection();

        let username = matches
            .subcommand_matches("adduser")
            .unwrap()
            .value_of("USERNAME")
            .unwrap();

        let password = read_password_from_tty(Some("Password: "))?;

        let new_user = NewUser::new(username, password.as_str()).expect("Error creating new user");

        diesel::insert_into(schema::users::table)
            .values(&new_user)
            .execute(&conn)
            .expect("Error saving new user");
    }

    if matches.is_present("deluser") {
        let conn = establish_connection();

        let username = matches
            .subcommand_matches("deluser")
            .unwrap()
            .value_of("USERNAME")
            .unwrap();

        diesel::delete(schema::users::table)
            .filter(User::with_username(username))
            .execute(&conn)
            .expect("Error deleting new user");
    }

    Ok(())
}
