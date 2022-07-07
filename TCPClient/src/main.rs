use std::io;
use std::net::TcpStream;
use std::io::Write;
use std::io::Read;


fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:7373") {
        println!("Connected to the server!");

        loop {
            let mut message = String::new();

            io::stdin()
                .read_line(&mut message)
                .expect("Failed to read line");

            stream.write(message.as_bytes()).unwrap();

            let mut buffer = [0; 256];
            stream.read(&mut buffer).unwrap();
            println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
            stream.flush().unwrap();
        }

    } else {
        println!("Couldn't connect to server...");
    }
}
