Simple Rust, GraphQL, and SQLite example
========================================

[![Build Status](https://travis-ci.org/mhallin/juniper-example-todo-backend.svg?branch=master)](https://travis-ci.org/mhallin/juniper-example-todo-backend)

This is a small repository connected to [Juniper] that serves an endpoint
similar to [Todo-Backend] but using GraphQL instead. It uses [SQLite] as the
database with [Diesel] as the ORM, and [Iron] as the web server.

Running
-------

Make sure you've got [Rust] installed, then run the following:

```sh
# If you haven't got the Diesel CLI installed already:
cargo install --no-default-features --features sqlite diesel_cli

# Clone this repository
git clone https://github.com/mhallin/juniper-example-todo-backend.git

# Create an empty test database
diesel migration run

# Build and run the server
cargo run
```

If all of that succeeded, you should have a [GraphiQL] interface running at
http://localhost:8080.

Interacting
-----------

Here are some example queries you can run through GraphiQL:

```graphql
# Create a new todo item and get the ID back
mutation {
  addTodo(title: "Write documentation", completed: false) {
    id
    title
    completed
  }
}
```

```graphql
# List all todo items in the database
{
  todoItems {
    id
    title
    completed
  }
}
```

```graphql
# Mark an item as completed
mutation {
  updateTodo(id: 1, completed: true) {
    id
  }
}
```

You can also check out the documentation in the "Docs" pane of GraphiQL. This
view is automatically generated from the documentation strings in the source.
Check out [src/schema.rs](src/schema.rs) to see how it looks.


[Juniper]: https://github.com/mhallin/juniper
[Todo-Backend]: https://www.todobackend.com
[SQLite]: https://sqlite.org
[Diesel]: http://diesel.rs
[Iron]: http://ironframework.io
[Rust]: https://www.rust-lang.org
[GraphiQL]: https://github.com/graphql/graphiql
