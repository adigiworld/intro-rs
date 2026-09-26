use std::{
    io::{Read, Write},
    net::TcpStream,
    str,
};

fn main() {
    let mut stream = TcpStream::connect("localhost:8080").unwrap();
    stream.write("Hello from Client".as_bytes()).unwrap();
    let mut buf = [0; 50];
    stream.read(&mut buf).unwrap();
    println!(
        "Got response from server:{:?}",
        str::from_utf8(&buf).unwrap().trim()
    );
}
