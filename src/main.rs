#[macro_use] extern crate juniper;

#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate diesel;

extern crate iron;
extern crate mount;
extern crate logger;

extern crate dotenv;

use std::env;

use iron::prelude::*;
use juniper::iron_handlers::{GraphQLHandler, GraphiQLHandler};

mod db;
mod models;
mod schema;

/*
A context object is used in Juniper to provide out-of-band access to global
data when resolving fields. We use it here to pass a database connection
to the Query and Mutation types.

Since this function is called once for every request, it will create a
database connection per request. A more realistic solution would be to use
the "r2d2" crate for connection pooling, and the "persistent" crate to pass
data into Iron requests.
*/
fn context_factory(_: &mut Request) -> schema::Context {
    schema::Context {
        connection: db::establish_connection(),
    }
}

fn main() {
    let graphql_endpoint = GraphQLHandler::new(
        context_factory,
        schema::QueryRoot,
        schema::MutationRoot,
    );

    let graphiql_endpoint = GraphiQLHandler::new("/graphql");

    let mut mount = mount::Mount::new();
    mount.mount("/", graphiql_endpoint);
    mount.mount("/graphql", graphql_endpoint);

    let (logger_before, logger_after) = logger::Logger::new(None);

    let mut chain = Chain::new(mount);
    chain.link_before(logger_before);
    chain.link_after(logger_after);

    let host = env::var("LISTEN").unwrap_or("0.0.0.0:8080".to_owned());
    println!("GraphQL server started on {}", host);
    Iron::new(chain).http(host.as_str()).unwrap();
}
