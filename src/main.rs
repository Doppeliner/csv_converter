#![feature(proc_macro_hygiene, decl_macro)]
#![warn(clippy::all)] 
#![warn(rust_2018_idioms)] 

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket::data::Data;
use rocket_contrib::json::JsonValue;
use serde_json::Value;
use std::io::Read;

#[post("/submit", data = "<data>")]
fn convert_csv_to_json(data: Data) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let mut stream = data.open();
    let mut csv = String::new();
    stream.read_to_string(&mut csv)?;

    let mut csv_deux = String::new();

    for (count, c) in csv.lines().enumerate() {
        if c == "" && count != 3 {
            break;
        }

        if count > 3 {
            csv_deux.push_str(c);
            csv_deux.push('\n');
        }
    }

    let mut rdr1 = csv::Reader::from_reader(csv_deux.as_bytes());
    let mut rdr2 = csv::Reader::from_reader(csv_deux.as_bytes());

    let mut json_string = String::new();

    json_string.push_str("[");

    let headers = rdr1.headers()?;

    for result in rdr2.records() {
        let record = result?;

        json_string.push_str("{\n");

        let special_iter = headers.iter().zip(record.iter());

        for s in special_iter {
            json_string.push_str(&format!("\t\"{}\": \"{}\",\n", s.0, s.1));
        }

        json_string.pop();
        json_string.pop();
        json_string.push('\n');

        json_string.push_str("},\n");
    }

    json_string.pop();
    json_string.pop();
    json_string.push_str("]\n");

    let v: Value = serde_json::from_str(&json_string)?;
    let j = json!(v);

    Ok(j)
}

fn main() {
    rocket::ignite()
        .mount(
            "/",
            routes![
                convert_csv_to_json
            ],
        )
        .launch();
}