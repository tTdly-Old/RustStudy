use std::{net::TcpListener, net::TcpStream, io::prelude::*, fs};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let contents = fs::read_to_string("hello.html").unwrap();
    let respone = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n{}",
        contents.len(),
        contents
    );

    stream.write(respone.as_bytes()).unwrap();
    stream.flush().unwrap();
}