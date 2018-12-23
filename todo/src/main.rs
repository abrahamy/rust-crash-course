#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;

mod controllers;
mod models;

use self::controllers::todos;

fn main() {
    rocket::ignite()
        .mount("/", routes![todos::list, todos::create])
        .launch();
}
