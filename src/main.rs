#[macro_use] extern crate rocket;
#[macro_use] extern crate diesel;

mod schema;
mod models;
mod database;
mod repository;
mod routes;

use rocket::fs::{FileServer, relative};
use rocket_dyn_templates::Template;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use database::ResumeDb;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

async fn run_migrations(rocket: rocket::Rocket<rocket::Build>) -> rocket::fairing::Result {
    let db = ResumeDb::get_one(&rocket).await.expect("database connection");
    db.run(|conn| {
        conn.run_pending_migrations(MIGRATIONS).expect("diesel migrations");
    }).await;

    Ok(rocket)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(ResumeDb::fairing())
        .attach(Template::fairing())
        .attach(rocket::fairing::AdHoc::on_ignite("Run Migrations", |rocket| async {
            run_migrations(rocket).await.unwrap()
        }))
        .mount("/", routes::get_routes())
        .mount("/static", FileServer::from(relative!("static")))
}
