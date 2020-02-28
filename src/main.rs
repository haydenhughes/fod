extern crate tokio;
extern crate warp;
#[macro_use]
extern crate diesel;
extern crate bcrypt;
extern crate clap;
extern crate rpassword;
extern crate serde;
extern crate tera;

// mod auth;
mod db;
mod models;
mod routes;
mod schema;

use clap::{App, AppSettings, Arg, SubCommand};
use diesel::prelude::*;
use models::{NewUser, User};
use rpassword::read_password_from_tty;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tera::Tera;

#[tokio::main]
async fn main() -> io::Result<()> {
    let matches = App::new("FodMap")
        .version("0.1.0")
        .author("Hayden Hughes <hayden@firemail.cc>")
        .about("Self-hosted pantry inventory system")
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
        let tera = match Tera::new("templates/**/*.html") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };

        let tera = Arc::new(tera);

        warp::serve(routes::routes(tera))
            .run("127.0.0.1:3000".parse::<SocketAddr>().unwrap())
            .await
    }

    if matches.is_present("adduser") {
        let conn = db::establish_connection();

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
        let conn = db::establish_connection();

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
