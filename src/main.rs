use std::{
    net::{TcpListener, TcpStream}, 
    io::{BufReader, BufRead, Write}, 
    fs, 
    thread, 
    time::Duration
};
use multithreaded_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    let pool = ThreadPool::new(10);

    listener.incoming().take(2).for_each(|stream| {
        pool.execute(|| {
            handle_connection(stream.unwrap());
        });
    });

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            println!("Will sleep now for 5 seconds.");
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
