pub mod schema;

use self::schema::todos;
use diesel::{self, Insertable, Queryable};

#[database("dev")]
pub struct Database(diesel::SqliteConnection);

#[derive(Debug, Queryable)]
pub struct Todo {
    pub id: i32,
    pub name: String,
    pub done: bool,
}

#[derive(Insertable)]
#[table_name = "todos"]
pub struct NewTodo {
    pub name: String,
    pub done: bool,
}

impl NewTodo {
    pub fn new(name: String) -> NewTodo {
        NewTodo { name, done: false }
    }
}
