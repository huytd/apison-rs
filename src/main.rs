#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde_json;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;

use std::io;
use std::path::{Path, PathBuf};
use rocket_contrib::{JSON, Value};
use rocket::response::NamedFile;
use diesel::prelude::*;
use diesel::pg::PgConnection;

pub mod schema;
pub mod models;

pub mod nodes;

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

#[get("/v0/data/<id>", format = "application/json")]
fn json_get(id: String) -> JSON<Value> {
    JSON(json!({
        "status": "true",
    }))
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
        .mount("/api", routes![json_get])
        .mount("/client", routes![index, files])
        .launch();
}
