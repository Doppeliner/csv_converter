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
///records stored in a Rocket JsonValue
///
///# Arguments
///
///* `data` - The raw form-data passed in from the post request, retrieved by Rocket
#[post("/submit", data = "<data>")]
fn convert_csv_to_json(data: Data) -> Result<JsonValue, Status> {

    let csv_trimmed: String = trim_data(data, "text/csv")?;
    let json_string: String = csv_string_to_json_string(csv_trimmed)?;

    //Converting from String to a serde_json Value
    let v: Value = match serde_json::from_str(&json_string) {
        Ok(value) => value,
        Err(_value) => return Err(Status::InternalServerError)
    };

    //Converting from a serde_json Value to a rocket JsonValue
    let rocket_json_value = json!(v);

    Ok(rocket_json_value)
}

///Returns a string containing the body of a raw Rocket Data object having removed the header. If
///multiple files are included, it will only return the first file
///
///# Arguments
///
///* `data` - The raw form-data passed in from the post request, retrieved by Rocket
///
///* `content_type` - A string that defines the expected content type. If you recieve a file that
///is not of the expected type, it will throw an Unprocessible Entity error. If passed an empty
///string "", it will allow any file type.
fn trim_data(data:Data, content_type: &str) -> Result<String, Status> { 
    let mut stream = data.open();
    let mut data_string = String::new();
    stream.read_to_string(&mut data_string);

    let mut data_id = String::new();
    let mut data_trimmed = String::new();

    for (count, d) in data_string.lines().enumerate() {
        //Storing the first line of the Rocket DataStream generated from the input file
        //This line is comprised of hyphens and an identifier
        if count == 0 {
            data_id.push_str(&d);
        }

        //Throwing an error if the submitted file is not of the specified. If no type is specified,
        //this check is skipped.
        if count == 2 && content_type != "" && d != format!("Content-Type: {}", content_type) {
            return Err(Status::UnprocessableEntity);
        }

        //As the first four lines of the DataStream do not contain csv records, they are ignored
        if count > 3 {
            //If we reach an empty line or a line matching the DataStream identifier, we have
            //reached the end of readable csv records. Otherwise, we save the line
            if d.contains(&data_id) || d == "" {
                break;
            } else {
                data_trimmed.push_str(&d);
                data_trimmed.push('\n');
            }
        }
    }

    Ok(data_trimmed)
}

///Returns a string containing a list of JSON objects when given an input string containing CSV
///records
///
///# Arguments
///
///* `csv` - A string containing a list of CSV records. Headers and Entries are delimited by commas
///while records are delimited by newline characters
fn csv_string_to_json_string(csv: String) -> Result<String, Status> {

    //Creating two readers so we can iterate through Reader.headers() and Reader.records()
    //simultaneously without borrowing conflicts
    let mut rdr1 = Reader::from_reader(csv.as_bytes());
    let mut rdr2 = Reader::from_reader(csv.as_bytes());

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

    Ok(json_string)
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
