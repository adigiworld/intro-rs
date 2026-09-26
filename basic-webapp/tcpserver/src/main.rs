use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    let listenet = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("Running on port 8080");
    for stream in listenet.incoming() {
        let mut stream = stream.unwrap();
        println!("Connection established");
        let mut buf = [0; 1024];
        stream.read(&mut buf).unwrap();
        stream.write(&mut buf).unwrap();
    }
}
