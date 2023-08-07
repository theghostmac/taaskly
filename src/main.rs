#[macro_use]
extern crate rocket;
mod router;
mod database;
mod models;
mod config;

use database::DatabaseRepository;
use dotenvy::dotenv;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    let db = DatabaseRepository::connect();
    rocket::build()
        .manage(db)
        .mount("/", routes![router::get_tasks, router::add_task, router::mark_as_done, router::delete_task])
}
