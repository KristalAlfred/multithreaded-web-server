use std::{
    net::{TcpListener, TcpStream}, 
    io::{BufReader, BufRead, Write}, 
    fs, 
    thread, 
    time::Duration
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

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html")
    };
    
    let html = fs::read_to_string(filename).unwrap();
    let length = html.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{html}");
    stream.write_all(response.as_bytes()).unwrap();
    
}
