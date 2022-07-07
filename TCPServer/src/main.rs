use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7373").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut buffer = [0; 256];

        match stream.read(&mut buffer) {
            Ok(buf_size) => {
                println!("Request: {}, Size: {}", String::from_utf8_lossy(&buffer[..]), buf_size);
                stream.write(&buffer[0..buf_size]).unwrap();
                stream.flush().unwrap();
            }
            Err(e) => {
                println!("Error: {:#?}", e);
            }
        }
    }
}
