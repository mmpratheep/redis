use std::net::TcpListener;

fn main() {
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        if stream.is_err() {
            println!("Error: {:?}", stream.err());
            continue;
        }
        println!("Connection established!");
    }
}
