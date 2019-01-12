pub(crate) mod TodoCtrl {
    use diesel::{insert_into, sql_query, RunQueryDsl};
    use rocket::{
        request::{Form, FromForm},
        Json,
    };

    use crate::models::{schema::todos, Database, NewTodo, Todo};

    #[derive(FromForm)]
    pub struct TodoForm {
        name: String,
    }

    #[get("/")]
    pub fn list(conn: Database) -> Json<Vec<Todo>> {
        let todo_list = sql_query("SELECT * FROM todos")
            .load(&conn)
            .expect("Could not select todos.");

        Json(todo_list)
    }

    #[post("/", data = "<data>")]
    pub fn create(conn: Database, data: Form<TodoForm>) -> Json<Todo> {
        let todo = insert_into(todos::table)
            .values(&NewTodo::new(data.name))
            .get_result::<Todo>(&conn)
            .unwrap();

        Json(todo)
    }
}
