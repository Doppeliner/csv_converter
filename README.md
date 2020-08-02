# csv_converter

This is a simple API to easily convert CSV files into a JSON response via a REST endpoint.

## Examples

You can integrate this functionality into any rocket server you're running by simply adding the convert_csv_to_json function to your rocket routes in main, like so:
```//Calls the ignite function from Rocket to start the server
fn main() {
    rocket::ignite()
        .mount("/", routes![csv_converter::convert_csv_to_json])
        .launch();
}```
