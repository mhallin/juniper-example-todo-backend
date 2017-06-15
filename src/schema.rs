use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use juniper::{Context as JuniperContext, FieldResult, ResultExt};

use models::{NewTodo, Todo};


pub struct Context {
    pub connection: SqliteConnection,
}

impl JuniperContext for Context {}

graphql_object!(Todo: () |&self| {
    description: "A todo item that that can be marked as completed"

    field id() -> i32 as "The unique id of the todo item" {
        self.id
    }

    field title() -> &str as "The user-editable title" {
        &self.title
    }
    
    field completed() -> bool as "Determines whether the user has completed the item or not" {
        self.completed
    }
});

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field todoItems(&executor) -> FieldResult<Vec<Todo>>
        as "Get all todo items in the system ordered by date"
    {
        use ::db::todos::dsl;

        dsl::todos.order(dsl::id)
            .load::<Todo>(&executor.context().connection)
            .to_field_err()
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field add_todo(&executor, title: String, completed: bool) -> FieldResult<Todo>
        as "Create a new todo item and return it"
    {
        use ::db::todos::dsl;

        executor.context().connection.transaction(|| {
            let new_post = NewTodo {
                title: &title,
                completed: completed,
            };

            diesel::insert(&new_post).into(::db::todos::table)
                .execute(&executor.context().connection)?;

            dsl::todos.order(dsl::id.desc())
                .first::<Todo>(&executor.context().connection)
        }).to_field_err()
    }

    field update_todo(&executor, id: i32, completed: Option<bool>, title: Option<String>) -> FieldResult<Option<Todo>>
        as "Update an existing todo item.
        
        Will only update the provided fields - if either `completed` or `title`
        are omitted or null, they will be ignored.
        
        The mutation will return null if no todo item with the specified ID could be found."
    {
        use ::db::todos::dsl;

        let updated = jtry!(diesel::update(dsl::todos.find(id))
            .set((
                completed.map(|completed| dsl::completed.eq(completed)),
                title.map(|title| dsl::title.eq(title)),
            ))
            .execute(&executor.context().connection));

        if updated == 0 {
            Ok(None)
        }
        else {
            Ok(Some(jtry!(dsl::todos.find(id)
                .get_result::<Todo>(&executor.context().connection))))
        }
    }
});
