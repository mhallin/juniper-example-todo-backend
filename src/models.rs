use db::todos;

#[derive(Queryable)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}

#[derive(Insertable)]
#[table_name="todos"]
pub struct NewTodo<'a> {
    pub title: &'a str,
    pub completed: bool,
}
