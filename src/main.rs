use crate::datatypes::process_command;
use std::env;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

pub mod datatypes;

const MAX_MESSAGE_SIZE: usize = 128;

fn main() {
    let default_port: Result<String, &str> = Ok("8080".to_string());

    match env::var("RUDEST_PORT").or(default_port) {
        Ok(port_string) => match port_string.parse::<usize>() {
            Ok(port) => {
                let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
                println!("listening on port {}", port);

                for stream in listener.incoming() {
                    match stream {
                        Ok(stream) => {
                            thread::spawn(|| handle_connection(stream));
                        }
                        Err(err) => println!("error opening stream: {}", err),
                    }
                }
            }
            Err(err) => println!("error parsing port: {}", err),
        },
        Err(err) => println!("error getting port: {}", err),
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; MAX_MESSAGE_SIZE];

    loop {
        match stream.read(&mut buf) {
            Ok(len) => {
                if len == 0 {
                    break;
                }

                process_command(&buf[0..len]);

                match std::str::from_utf8(&buf[0..len]) {
                    Ok(msg) => {
                        println!("msg: {}", msg);
                        match stream.write(&buf[0..len]) {
                            Ok(len) => {
                                println!("sent length {}", len);
                            }
                            Err(err) => println!("error writing: {}", err),
                        }

                        if msg.trim() == "quit" {
                            match stream.shutdown(Shutdown::Both) {
                                Ok(_) => break,
                                Err(err) => println!("error shutting down stream: {}", err),
                            }
                        }
                    }
                    Err(err) => println!("error receiving message: {}", err),
                }
            }
            Err(err) => println!("error reading: {}", err),
        }
    }
}
