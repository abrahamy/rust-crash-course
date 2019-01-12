#![feature(custom_attribute, decl_macro, proc_macro_hygiene, uniform_paths)]

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate rocket;

mod controllers;
mod models;

use controllers::TodoCtrl;
use models::Database;

fn main() {
    rocket::ignite()
        .attach(Database::fairing())
        .mount("/", routes![TodoCtrl::list, TodoCtrl::create])
        .launch();
}
