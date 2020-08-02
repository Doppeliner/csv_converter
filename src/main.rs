#![feature(proc_macro_hygiene, decl_macro)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate rocket;

use rocket::data::Data;
use rocket_contrib::json::JsonValue;
use rocket::http::Status;

//This is abstracted so the user can define their own routes if they use this library
#[post("/submit", data = "<data>")]
fn call_convert_csv_to_json(data: Data) -> Result<JsonValue, Status>{
    Ok(csv_converter::convert_csv_to_json(data)?)
}

//Calls the ignite function from Rocket to start the server
fn main() {
    rocket::ignite()
        .mount("/", routes![call_convert_csv_to_json])
        .launch();
}
