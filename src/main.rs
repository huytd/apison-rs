#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
extern crate dotenv;

pub mod utils;
pub mod schema;
pub mod models;

pub mod nodes;

use std::io;
use std::path::{Path, PathBuf};
use rocket_contrib::{JSON, Value};
use rocket::response::NamedFile;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use utils::*;

use models::Node;

// Static web client

#[get("/")]
fn index() -> io::Result<NamedFile> {
    NamedFile::open("www/index.html")
}

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("www/").join(file)).ok()
}

// API

#[get("/v0/data", format = "application/json")]
fn json_get_all() -> JSON<Value> {
    let db = establish_connection();
    let result = Node::get_all_nodes(&db);
    JSON(json!({
        "results": result
    }))
}

#[get("/v0/data/<id>", format = "application/json")]
fn json_get(id: String) -> JSON<Value> {
    let db = establish_connection();
    let found = Node::get_by_key(&db, &id);
    let mut result = json!({ "result": "" });
    if found.is_some() {
        result = json!({ "results": found.unwrap() });
    }
    JSON(result)
}

#[post("/v0/data/<id>", format = "application/json")]
fn json_create(id: String) -> JSON<Value> {
    JSON(json!({
        "status": "true",
    }))
}

fn main() {
    println!("Server is running");
    rocket::ignite()
        .mount("/api", routes![json_get, json_get_all, json_create])
        .mount("/client", routes![index, files])
        .launch();
}
