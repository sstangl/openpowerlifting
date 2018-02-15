#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate rocket;
extern crate rocket_contrib;
extern crate serde;
#[macro_use] extern crate serde_derive;
use rocket_contrib::Template;
use rocket::State;

extern crate dotenv;
use std::env;
use std::process;

extern crate server;
use server::opldb;
use server::opldb::CachedFilter;
use server::pages;


#[get("/u/<username>")]
fn lifter_handler(username: String, opldb: State<opldb::OplDb>) -> Option<Template> {
    let lifter_id = match opldb.get_lifter_id(&username) {
        None => return None,
        Some(id) => id,
    };

    let context = pages::lifter::Context::new(&opldb, lifter_id);
    Some(Template::render("lifter", &context))
}


fn rocket(opldb: opldb::OplDb) -> rocket::Rocket {
    // Initialize the server.
    rocket::ignite()
        .manage(opldb)
        .mount("/", routes![lifter_handler])
        .attach(Template::fairing())
}


fn get_envvar_or_exit(key: &str) -> String {
    env::var(key).map_err(|_| {
        eprintln!("Environment variable '{}' not set.", key);
        process::exit(1);
    }).unwrap()
}


fn main() {
    // Populate std::env with the contents of any .env file.
    dotenv::from_filename("server.env").ok();

    let lifters_csv = get_envvar_or_exit("LIFTERS_CSV");
    let meets_csv = get_envvar_or_exit("MEETS_CSV");
    let entries_csv = get_envvar_or_exit("ENTRIES_CSV");

    let opldb = match opldb::OplDb::from_csv(&lifters_csv, &meets_csv, &entries_csv) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("Error loading OplDb: {}", e);
            process::exit(1);
        }
    };

    println!("OplDb loaded in {}MB.", opldb.size_bytes() / 1024 / 1024);

    // Run the server loop.
    rocket(opldb).launch();
}
