#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

use rocket::http::RawStr;
use rocket::{Request, data::Data};
use serde::Serialize;
use rocket_contrib::json::Json;
use csv::Reader;
use csv::Error;
use std::fs::File;
use std::io::Read;

#[derive(Serialize)]
struct Guitar {
    number_of_strings: i32,
    pickup_config: &'static str
}

#[get("/")]
fn index() -> &'static str {
    return "Welcome to the index!";
}

#[get("/hello")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/hello/<name>")]
fn hello_name(name: &RawStr) -> String {
    format!("Hello, {}!", name.as_str())
}

#[get("/json")]
fn test_json() -> Json<Guitar> {
    Json(
        Guitar {
            number_of_strings: 6,
            pickup_config: "HH"
        }
    )
}
    
#[post("/submit_csv", format = "text/csv", data = "<data>")]
fn handle_csv(data: Data) -> &'static str {
    return "Recieved text/csv";
}

#[post("/submit", data = "<data>")]
fn convert_csv_to_json(data: Data) -> &'static str {
    let mut stream = data.open();
    let mut csv = String::new();
    stream.read_to_string(&mut csv);

    println!("{}", csv);
    return "Data was printed";
}

fn main() {
    rocket::ignite().mount("/", routes![
        index, 
        hello,
        hello_name,
        test_json,
        handle_csv,
        convert_csv_to_json
    ]).launch();
}
