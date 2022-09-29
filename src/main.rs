use std::io::Read;
use std::net::TcpListener;

fn main() {
    println!("Hello, TCP!");
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0; 128];
                println!("stream: {stream:?}");

                loop {
                    match stream.read(&mut buf) {
                        Ok(len) => {
                            let msg = std::str::from_utf8(&buf[0..len]).unwrap();
                            println!("message: {msg}");

                            if msg == "quit" {
                                break;
                            }
                        }
                        Err(_) => todo!(),
                    }
                }
            }
            Err(e) => println!("stream error: {e:?}"),
        }
    }
}
