use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::fs;
// Run trait executes the task
pub trait Run {
    fn run(&self); 
}

pub struct DiskIO {
    filename: String,
    task_subtype: i32,
    contents: String
}

impl DiskIO {
    pub fn new(filename: String, task_subtype: i32, contents: String) -> DiskIO {
        DiskIO{filename: filename, task_subtype: task_subtype, contents: contents}
    }
    pub fn run(&self) {
        match self.task_subtype {
            0 => self.read(),
            1 => self.write(),
            2 => self.create(),
            _ => println!("ERRRRRROR"),
        }
    }

    fn read(&self) {
        println!("reading!");
        let contents = fs::read_to_string(&self.filename)
        .expect("Something went wrong reading the file");

        println!("With text:\n{}", contents); 
    }
    fn write(&self) {
        println!("Writing...");
        let path = Path::new(&self.filename);
        let display = path.display();
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(&self.contents.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why),
            Ok(_) => println!("successfully wrote to {}", display),
        }
    }

    fn create(&self) {
        println!("Creating file...");
        let path = Path::new(&self.filename);
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why),
            Ok(file) => file,
        };
    }
}
/*
    disk_io - read/write/copy/encrypt disk interactions. Task organization:
    "task_type": "disk_io",
    "task_details": {
        "task_subtype": "read",
        "params": {
            "working_dir": "/path/to/dir",
            "filename": "bananas.txt",
        }
    }
*/