# csv_converter

This is a simple API to easily convert CSV files into a JSON response via a REST endpoint. Additionally, this endpoint will check and ensure that you have submitted a CSV file and that it is formatted correctly. 

## Usage

### Running this crate standalone

### Using this crate in your project

Here's an example of how you would make calls to thi library using your own main file with Rocket. This allows you to define your own route names and parameters so you don't run into route conflicts with pre-existing code.
```#[macro_use]
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
}```
