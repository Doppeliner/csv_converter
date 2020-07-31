#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

use rocket::http::RawStr;
use rocket::{Request, data::Data};
use serde::Serialize;
use serde_json::Value;
use rocket_contrib::json::{Json, JsonValue};
use csv::Reader;
use csv::Error;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

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

#[get("/json/value")]
fn test_json_value() -> JsonValue {
    json!([{
        "id": 83,
        "values": [1, 2, 3, 4]
    },
    {
        "id": 01
    }])
}
    
#[post("/submit_csv", format = "text/csv", data = "<data>")]
fn handle_csv(data: Data) -> &'static str {
    let mut stream = data.open();
    let mut csv = String::new();
    stream.read_to_string(&mut csv);

    println!("{}", csv);
    return "Recieved text/csv";
}

#[post("/submit", data = "<data>")]
fn convert_csv_to_json(data: Data) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let mut stream = data.open();
    let mut csv = String::new();
    stream.read_to_string(&mut csv);
    

    let mut csv_deux = String::new();
    let mut count = 0;
    
    for c in csv.lines() {

        if c == "" && count != 3 {
            break;
        }

        if count > 3 {
            csv_deux.push_str(c);
            csv_deux.push('\n');
        }

        count += 1;
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
 
    println!("{}", &json_string);
    let v: Value = serde_json::from_str(&json_string)?;
    println!("{:?}", &v);
    let mut j = json!(v);
    println!("{:?}", &j);

    return Ok(j);
}

fn main() {
    rocket::ignite().mount("/", routes![
        index, 
        hello,
        hello_name,
        test_json,
        test_json_value,
        handle_csv,
        convert_csv_to_json
    ]).launch();
}
