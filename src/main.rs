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

#[post("/v0/data", format = "application/json", data = "<node>")]
fn json_create(node: JSON<Node>) -> JSON<Value> {
    let db = establish_connection();
    let new_node = Node::new(&db, &node.key, &node.value);
    let result = Node::create(&db, new_node);
    JSON(json!({
        "result": result
    }))
}

#[put("/v0/data/<id>", format = "application/json", data = "<node>")]
fn json_update(id: String, node: JSON<Node>) -> JSON<Value> {
    let db = establish_connection();
    let new_value = &node.value;
    let result = Node::update_key(&db, &id, &new_value);
    JSON(json!({
        "result": result
    }))
}

#[delete("/v0/data/<id>", format = "application/json")]
fn json_delete(id: String) -> JSON<Value> {
    let db = establish_connection();
    let result = Node::delete_key(&db, &id);
    JSON(json!({
        "status": result
    }))
}

fn main() {
    println!("Server is running");
    rocket::ignite()
        .mount("/api", routes![json_get, json_get_all, json_create, json_update, json_delete])
        .mount("/client", routes![index, files])
        .launch();
}
