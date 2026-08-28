// mod request;
// use crate::request::Request;
// use crate::response::Response;
// use crate::request;
// use crate::response;
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
};

pub struct Router {
    host: String,
    port: u16,
    routes: Vec<Route>,
}

struct Route {
    method: String,
    route: String,
    handler: Handler,
}

type Handler = fn(Request, Response);

impl Router {
    pub fn new(host: &str, port: u16) -> Self {
        Router {
            host: host.to_string(),
            port,
            routes: vec![],
        }
    }
    pub fn get(&mut self, route: &str, handler: Handler) {
        let route = Route {
            method: "GET".to_string(),
            route: route.to_string(),
            handler,
        };
        self.routes.push(route);
    }
    pub fn post(&mut self, route: &str, handler: Handler) {
        let route = Route {
            method: "POST".to_string(),
            route: route.to_string(),
            handler,
        };
        self.routes.push(route);
    }
    pub fn serve(self) {
        let listner = TcpListener::bind((self.host, self.port)).unwrap();
        let (tx, rx) = std::sync::mpsc::channel::<TcpStream>();
        let rx_arc = Arc::new(std::sync::Mutex::new(rx));

        let arc = Arc::new(self.routes);

        for _ in 0..2 {
            let rx_clone = rx_arc.clone();
            let route_clone = arc.clone();

            thread::spawn(move || {
                loop {
                    let route_clone2 = route_clone.clone();
                    let stream: TcpStream = rx_clone.lock().unwrap().recv().unwrap();
                    Self::handle_connection(stream, route_clone2);
                    println!("{:?}", thread::current().id());
                }
            });
        }
        for incoming in listner.incoming() {
            match incoming {
                Ok(stream) => {
                    tx.send(stream).unwrap()
                    // let route_clone = arc.clone();
                    // thread::spawn(move || {
                    //
                    //     Self::handle_connection(stream, route_clone);
                    //     println!("{:?}", thread::current().id());
                    // });
                }
                Err(err) => {
                    // eprintln!("{}", err);
                }
            }
        }
    }
    fn handle_connection(mut stream: TcpStream, routes: Arc<Vec<Route>>) {
        let mut buffer: [u8; 1024] = [0; 1024];
        let _size = stream.read(&mut buffer).unwrap();
        let data = std::str::from_utf8(&buffer).unwrap();
        // println!("{}", data);
        let request = Request::new(data.to_string());
        let response = Response { stream: stream };
        for route in routes.iter() {
            if route.method == request.method && route.route == request.uri {
                (route.handler)(request, response);
                break;
            }
        }
    }
}

#[derive(Default)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub headers: std::collections::HashMap<String, String>,
    pub post_body: String,
}

impl Request {
    pub fn new(request_data: String) -> Self {
        if let Some((request_line, rest)) = request_data.split_once("\r\n") {
            let segments: Vec<&str> = request_line.split_whitespace().collect();
            if segments.len() == 3 {
                let mut headers = std::collections::HashMap::new();
                let mut post_body = String::new();
                Self::extract(&rest, &mut headers, &mut post_body);
                // println!("Headers : {:?}", headers);
                Self {
                    method: segments[0].to_string(),
                    uri: segments[1].to_string(),
                    headers: headers,
                    post_body: post_body,
                }
            } else {
                Self::default()
            }
        } else {
            Self::default()
        }
    }
    fn extract(
        request_data: &str,
        headers: &mut std::collections::HashMap<String, String>,
        post_body: &mut String,
    ) {
        if let Some((header_line, rest)) = request_data.split_once("\r\n") {
            if header_line.is_empty() {
                post_body.push_str(rest);
            } else {
                let key_value = header_line.split_once(":").unwrap();
                headers.insert(key_value.0.to_string(), key_value.1.trim().to_lowercase());
                Self::extract(rest, headers, post_body);
            }
        }
    }
}

pub struct Response {
    stream: TcpStream,
}

impl Response {
    pub fn send(&mut self, output: String) {
        self.stream
            .write(format!("HTTP/1.1 200 OK\r\n\r\n{}", output).as_bytes())
            .unwrap();
    }
}
