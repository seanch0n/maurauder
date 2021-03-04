use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

pub struct NetworkIO {
    task_subtype: i32,
    ip_address: String,
    port: i32,
    data: String
}


impl NetworkIO {
    pub fn new(task_subtype: i32, ip_address: String, port: i32, data: String) -> NetworkIO {
        NetworkIO {
            task_subtype: task_subtype,
            ip_address: ip_address,
            port: port,
            data: data
        }
    }

    pub fn run(&self) {
        match self.task_subtype {
            0 => self.tcp_connect(),
            // 1 => udp_connect(),
            _ => println!("ERRRor")
        }
    }

    fn tcp_connect(&self) {
        let mut conn_str = String::new();
        conn_str.push_str(&self.ip_address);
        conn_str.push_str(":");
        conn_str.push_str(&self.port.to_string());
        match TcpStream::connect(conn_str.clone()) {
            Ok(mut stream) => {
                println!("Okay we did it");
            },
            Err(e) => {
                println!("Failed to connect: {} {}", e, conn_str.clone());
            }
        }
        println!("Connected!");
    }
}