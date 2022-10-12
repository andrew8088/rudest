use std::io::Read;
use std::net::{Shutdown, TcpListener, TcpStream};
use std::thread;

fn main() {
    println!("Hello, TCP!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_connection(stream));
            }
            Err(e) => println!("stream error: {e:?}"),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buf = [0; 128];
    println!("stream: {stream:?}");
    loop {
        match stream.read(&mut buf) {
            Ok(len) => {
                if len == 0 {
                    println!("goodbye!");
                    break;
                }

                let msg = std::str::from_utf8(&buf[0..len]).unwrap().trim();
                println!("message: {msg}");

                if msg == "quit" {
                    match stream.shutdown(Shutdown::Both) {
                        Ok(_) => break,
                        Err(_) => break,
                    }
                }
            }
            Err(e) => {
                println!("error when reading: {}", e);
            }
        }
    }
}
