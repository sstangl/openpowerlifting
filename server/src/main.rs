#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(custom_derive)]

extern crate dotenv;
use std::env;
use std::process;

extern crate server;
use server::opldb;

/*
fn rocket() -> rocket::Rocket {
    // Initialize an r2d2 database connection pool.
    let db_path = env::var("DATABASE_PATH").expect("DATABASE_PATH is not set.");
    let db_pool = schema::init_pool(&db_path);

    // Pre-cache some database information at boot.
    // Because the database is read-only, this information is correct
    // for the lifetime of the server.
    let conn = DbConn(db_pool.get().expect("Failed to get a connection from pool."));
    let num_entries = queries::count_entries(&conn).expect("Failed to count entries.");
    let num_meets = queries::count_meets(&conn).expect("Failed to count meets.");

    let db_stats = DbStats {
        num_entries: num_entries,
        num_meets: num_meets,
    };

    // Initialize the server.
    rocket::ignite()
        .manage(db_pool)
        .manage(db_stats)

        .mount("/", routes![rankings_handler])
        .mount("/", routes![static_handler])
        .mount("/", routes![lifter_handler])
        .mount("/", routes![meet_handler])
        .mount("/", routes![faq_handler])
        .mount("/", routes![contact_handler])
        .mount("/", routes![data_handler])

        // Old HTML redirectors.
        .mount("/", routes![redirect_old_contact_html,
                            redirect_old_data_html,
                            redirect_old_faq_html,
                            redirect_old_lifters_html,
                            redirect_old_meet_html,
                            redirect_old_index_html])

        // Old HTML handlers.
        .mount("/", routes![meetlist_html])

        .attach(Template::fairing())
}
*/


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

    let usapl_entries = opldb.filter_entries(|e|
        opldb.get_meet(e.meet_id).federation == opldb::fields::Federation::USAPL
    );
    let _93kg_entries = opldb.filter_entries(|e|
        e.weightclasskg == opldb::fields::WeightClassKg::UnderOrEqual(93_f32)
    );
    let both = usapl_entries.intersect(&_93kg_entries);
    println!("USAPL 93kg entries count: {}", both.list.len());

    // Run the server loop.
    //rocket().launch();
}
