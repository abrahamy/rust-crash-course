use diesel::{insert_into, sql_query, RunQueryDsl};
use rocket::request::Form;

use self::schema::*;
use crate::models::*;

#[derive(FromForm)]
struct TodoForm {
    name: String,
}

#[get("/")]
pub fn list(conn: Database) -> String {
    let todo_list = sql_query("SELECT * FROM todos")
        .load(&conn)
        .expect("Could not select todos.");

    format!("{:#?}", todo_list)
}

#[post("/", data = "<data>")]
pub fn create(conn: Database, data: Form<TodoForm>) -> String {
    let todo = insert_into(todos::table)
        .values(&NewTodo::new(data.name))
        .get_result::<Todo>(&conn)
        .unwrap();

    format!("Created: {:#?}", todo)
}
