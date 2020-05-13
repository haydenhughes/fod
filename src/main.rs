#![feature(proc_macro_hygiene, decl_macro)]

extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate rocket_codegen;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate chrono;
extern crate clap;
extern crate rpassword;
extern crate serde;

mod auth;
mod models;
mod routes;
mod schema;

use auth::NewUser;
use auth::User;
use clap::{App, AppSettings, Arg, SubCommand};
use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use human_panic::setup_panic;
use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;
use rpassword::read_password_from_tty;
use std::{env, io};

#[database("fodmap")]
pub struct FodMapDatabase(diesel::PgConnection);

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

fn ask_password() -> Result<String, io::Error> {
    let password = read_password_from_tty(Some("Password: "))?;

    if password == read_password_from_tty(Some("Re-enter Password: "))? {
        Ok(password)
    } else {
        println!("Sorry, passwords do not match.");
        std::process::exit(1);
    }
}

fn main() -> io::Result<()> {
    setup_panic!();

    let matches = App::new("FodMap")
        .version("0.1.0")
        .author("Hayden Hughes <hayden@foxes.systems>")
        .about("Self-hosted health tracker")
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
            SubCommand::with_name("chpasswd")
                .about("Change a user password")
                .long_about("Requires the DATABASE_URL environment vairable to be set")
                .arg(
                    Arg::with_name("USERNAME")
                        .help("Secify the username of the user to modify the password of")
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
                    routes::index::index,
                    auth::routes::login,
                    auth::routes::user_login,
                    auth::routes::user_logout,
                ],
            )
            .register(catchers![routes::errors::unauthorized])
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

        let new_user =
            NewUser::new(username, ask_password()?.as_str()).expect("Error creating new user");

        diesel::insert_into(schema::users::table)
            .values(&new_user)
            .execute(&conn)
            .expect("Error saving new user");

        println!("Succesfully created user {}", username);
    }

    if matches.is_present("chpasswd") {
        let conn = establish_connection();

        let username = matches
            .subcommand_matches("chpasswd")
            .unwrap()
            .value_of("USERNAME")
            .unwrap();

        println!("Changing password for {}", username);

        diesel::update(schema::users::table)
            .filter(User::with_username(username))
            .set(
                schema::users::password
                    .eq(User::hash_password(ask_password()?).expect("Error hashing password")),
            )
            .execute(&conn)
            .expect("Error updating password");

        println!("Succesfully changed password of user {}", username);
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

        println!("Succesfully deleted user {}", username);
    }

    Ok(())
}
