mod disk_io;
mod network;
use serde_json::{Result, Value};
// use std::io::{self, Read};
use disk_io::{Run, DiskIO};
use network::{NetworkIO};
fn main() {
    let the_file = r#"{
        "FirstName": "John",
        "LastName": "Doe",
        "Age": 43,
        "Address": {
            "Street": "Downing Street 10",
            "City": "London",
            "Country": "Great Britain"
        },
        "PhoneNumbers": [
            "+44 1234567",
            "+44 2345678"
        ]
    }"#;

    let json: serde_json::Value =
        serde_json::from_str(the_file).expect("JSON was not well-formatted");
    println!("{}", json["Address"]["Street"]);

    let read_task = DiskIO::new(String::from("test.txt"), 0, String::from(""));
    read_task.run();
    let write_task = DiskIO::new(String::from("test_write.txt"), 2, String::from(""));
    write_task.run();

    let create_task = DiskIO::new(String::from("test_writing.txt"), 1, String::from("ahhh\n\nahhh"));
    create_task.run();

    let tcp_con_task = NetworkIO::new(
        0,
        String::from("127.0.0.1"),
        4444,
        String::from("")
    );
    tcp_con_task.run();
}