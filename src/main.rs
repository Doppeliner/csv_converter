#![feature(proc_macro_hygiene, decl_macro)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate rocket;

///Calls the ignite function from Rocket to start the server
fn main() {
    rocket::ignite()
        .mount("/", routes![csv_converter::convert_csv_to_json])
        .launch();
}
