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
use csv::Reader;

use serde::Serialize;
use rocket_contrib::json::Json;
use rocket::http::Status;

#[derive(Serialize)]
struct Task { a: i32 }

#[post("/test/<number>")]
fn test_custom_response(number: i32) -> Result<Json<Task>, Status>{ 
    if number != 0 {
        Ok(Json(Task{ 
            a: number
        }))
    } else {
        Err(Status::NotAcceptable)
    }
}

#[post("/submit", data = "<data>")]
fn convert_csv_to_json(data: Data) -> Result<JsonValue, Box<dyn std::error::Error>> {
    let mut stream = data.open();
    let mut csv_stream = String::new();
    stream.read_to_string(&mut csv_stream);

    let mut csv_trimmed = String::new();

    let mut csv_id = String::new();

    for (count, c) in csv_stream.lines().enumerate() {

        if count == 0 {
            csv_id.push_str(&c);
        }

        if (c.contains(&csv_id) || c == "")  && count != 3 && count != 0 {
            break;
        }

        if count > 3 {
            csv_trimmed.push_str(c);
            csv_trimmed.push('\n');
        }
    }

    let mut rdr1 = Reader::from_reader(csv_trimmed.as_bytes());
    let mut rdr2 = Reader::from_reader(csv_trimmed.as_bytes());

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
                test_custom_response,
                convert_csv_to_json
            ],
        )
        .launch();
}
