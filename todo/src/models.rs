use self::schema::todos;
use diesel::{pg::PgConnection, Insertable, Queryable};
use rocket_contrib::database;

#[database("dev")]
pub struct Database(PgConnection);

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

pub(crate) mod schema {
    table! {
        todos (id) {
            id -> Integer,
            name -> Text,
            done -> Bool,
        }
    }
}
