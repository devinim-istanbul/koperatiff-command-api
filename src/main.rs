extern crate serde_json;
#[macro_use] extern crate serde_derive;
extern crate actix_web;
extern crate dotenv;

// Actix Web Framework
use actix_web::{App, http, HttpRequest, HttpResponse, Json, Result, server};

// Environment
use std::env;
use dotenv::{dotenv};

// PoC Request Object
#[derive(Debug, Serialize, Deserialize)]
struct Person {
    name: String,
}

// PoC Endpoint
fn index(req: Json<Person>) -> Result<String> {
    Ok(format!("{}", req.name))
}

fn main() {
    dotenv().expect("Failed to read .env file");
    let port = env::var("PORT").expect("PORT not found");
    let uri = env::var("URI").expect("URI not found");

    server::new(
        || App::new()
            .resource("/{name}", |r| r.method(http::Method::POST).with(index)))
        .bind(format!("{}:{}", uri, port)).unwrap()
        .run()
}
