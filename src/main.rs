use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

    listener.incoming().for_each(|stream| {
        println!("New connection: {:?}", stream.unwrap());
    });
}
