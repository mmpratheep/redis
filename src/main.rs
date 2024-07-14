use std::io::{BufRead, BufReader, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(open_stream) => {
                handle_connection(open_stream);
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let pong = "+PONG\r\n";
    let ping = "PING";
    let mut buf_reader = BufReader::new(stream.try_clone().expect("Failed to clone stream"));
    loop {
        let request: Vec<String> = buf_reader.by_ref().lines()
            .map(|line| line.unwrap())
            .take_while(|line| line.contains(ping))
            .collect();
        println!("Received request {}", request.len());
        stream.write_all(pong.as_bytes()).expect("unable to send response");
    }
}