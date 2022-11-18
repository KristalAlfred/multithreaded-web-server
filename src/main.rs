use std::{
    net::{TcpListener, TcpStream}, 
    io::{BufReader, BufRead, Write}, fs
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    listener.incoming().for_each(|stream| {
        handle_connection(stream.unwrap());
    });
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
        let html = fs::read_to_string("index.html").unwrap();
        let length = html.len();
        let response = format!("HTTP/1.1 200 OK\r\nContent-Length: {length}\r\n\r\n{html}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let response = "HTTP/1.1 404 NOT FOUND\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
    }
}
