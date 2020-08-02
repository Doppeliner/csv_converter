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
use rocket::http::Status;

///Takes a CSV from a POST request and responds with a list of JSON Objects constructed from those
///records
///
///# Arguments
///
///* `data` - The raw form-data passed in from the post request, retrieved by Rocket
#[post("/submit", data = "<data>")]
fn convert_csv_to_json(data: Data) -> Result<JsonValue, Status> {
    let mut stream = data.open();
    let mut csv_stream = String::new();
    stream.read_to_string(&mut csv_stream);

    let mut csv_id = String::new();
    let mut csv_trimmed = String::new();

    for (count, c) in csv_stream.lines().enumerate() {
        //Storing the first line of the Rocket DataStream generated from the input file
        //This line is comprised of hyphens and an identifier
        if count == 0 {
            csv_id.push_str(&c);
        }

        //Throwing an error if the submitted file is not a csv
        if count == 2 && c != "Content-Type: text/csv" {
            return Err(Status::UnprocessableEntity);
        }

        //As the first four lines of the DataStream do not contain csv records, they are ignored
        if count > 3 {
            //If we reach an empty line or a line matching the DataStream identifier, we have
            //reached the end of readable csv records. Otherwise, we save the line
            if c.contains(&csv_id) || c == "" {
                break;
            } else {
                csv_trimmed.push_str(c);
                csv_trimmed.push('\n');
            }
        }
    }

    //Creating two readers so we can iterate through Reader.headers() and Reader.records()
    //simultaneously without borrowing conflicts
    let mut rdr1 = Reader::from_reader(csv_trimmed.as_bytes());
    let mut rdr2 = Reader::from_reader(csv_trimmed.as_bytes());

    //Transforming the data from a CSV string to a JSON style string
    let mut json_string = String::new();

    json_string.push_str("[");

    let headers = match rdr1.headers() {
        Ok(csv_headers) => csv_headers,
        Err(_csv_headers) => return Err(Status::UnprocessableEntity)
    };

    for result in rdr2.records() {
        let record = match result{
            Ok(v) => v,
            Err(_v) => return Err(Status::UnprocessableEntity)
        };

        json_string.push_str("{\n");

        //Zipping the headers and records together so as we iterate through the records, we can
        //concatenate the matching header easily
        let zipped_iter = headers.iter().zip(record.iter());

        for z in zipped_iter {
            json_string.push_str(&format!("\t\"{}\": \"{}\",\n", z.0, z.1));
        }

        //Popping twice to remove the trailing comma on the last field
        json_string.pop();
        json_string.pop();
        json_string.push_str("\n},\n");
    }

    //Popping twice to remove the trailing comma on the last object
    json_string.pop();
    json_string.pop();
    json_string.push_str("]\n");

    //Converting from String to a serde_json Value
    let v: Value = match serde_json::from_str(&json_string) {
        Ok(value) => value,
        Err(_value) => return Err(Status::InternalServerError)
    };

    //Converting from a serde_json Value to a rocket JsonValue
    let rocket_json_value = json!(v);

    Ok(rocket_json_value)
}

///Calls the ignite function from Rocket to start the server
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
