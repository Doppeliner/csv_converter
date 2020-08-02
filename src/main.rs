#![feature(proc_macro_hygiene, decl_macro)]
#![warn(clippy::all)]
#![warn(rust_2018_idioms)]

#[macro_use]
extern crate rocket;

use csv::Reader;
use rocket::data::Data;
use rocket::http::Status;
use rocket_contrib::json::JsonValue;
use serde_json::Value;
use std::io::Read;

///Takes a CSV from a POST request and responds with a list of JSON Objects constructed from those
///records stored in a Rocket JsonValue
///
///# Arguments
///
///* `data` - The raw form-data passed in from the post request, retrieved by Rocket
#[post("/submit", data = "<data>")]
fn convert_csv_to_json(data: Data) -> Result<JsonValue, Status> {
    let csv_trimmed: String = trim_data(data, "text/csv")?;
    let serde_json_value: Value = csv_string_to_json_value(csv_trimmed)?;

    //Converting from a serde_json Value to a rocket JsonValue
    let rocket_json_value = rocket_contrib::json!(serde_json_value);

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
fn trim_data(data: Data, content_type: &str) -> Result<String, Status> {
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

///Returns a serde_json Value from a given string of CSV records
///
///# Arguments
///
///* `csv` - A string containing a list of CSV records. Headers and Entries are delimited by commas
///while records are delimited by newline characters
fn csv_string_to_json_value(csv: String) -> Result<Value, Status> {
    
    let mut rdr = Reader::from_reader(csv.as_bytes());
    
    //The final list of JSON objects
    let mut objects_vec = Vec::new();

    //Cloning the headers from the reader to prevent borrowing errors later on
    let headers = rdr.headers().map_err(|_| Status::UnprocessableEntity)?.clone();

    for result in rdr.records() {
        let record = result.map_err(|_| Status::UnprocessableEntity)?;

        //This represents an individial JSON Object converted from a single CSV record
        let mut object_map = serde_json::Map::new();

        //Zipping the headers and records together so as we iterate through the records, we have
        //easier access to the individual entry and the matching header
        let zipped_iter = headers.iter().zip(record.iter());

        for z in zipped_iter {
            object_map.insert(z.0.to_string(), serde_json::json!(z.1));
        }

        objects_vec.push(serde_json::Value::Object(object_map));
    }

    let json_val = serde_json::Value::Array(objects_vec);
    Ok(json_val)
}

///Calls the ignite function from Rocket to start the server
fn main() {
    rocket::ignite()
        .mount("/", routes![convert_csv_to_json])
        .launch();
}

#[cfg(test)]
mod tests {
    #[test]
    fn csv_string_to_json_value_success() {
        assert_eq!(
            super::csv_string_to_json_value("ID,Name\n1,Bob".to_string()),
            Ok(serde_json::Value::Array(vec![serde_json::json!({ "ID" : "1", "Name" : "Bob"})]))
        )
    }

    #[test]
    fn csv_string_to_json_value_record_failure() {
        assert_eq!(
            super::csv_string_to_json_value("ID,Name\n1".to_string()),
            Err(rocket::http::Status::UnprocessableEntity)
        )
    }

    #[test]
    fn csv_string_to_json_value_header_failure() {
        assert_eq!(
            super::csv_string_to_json_value("ID\n1,2".to_string()),
            Err(rocket::http::Status::UnprocessableEntity)
        )
    }
}
