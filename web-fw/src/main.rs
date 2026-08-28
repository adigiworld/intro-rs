mod router;

fn main() {
    let mut app = router::Router::new("127.0.0.1", 8080);
    app.get("/", |_req, mut res| {
        res.send("Home Router\n".to_string());
    });
    app.get("/about", |_req, mut res| {
        res.send("About Router\n".to_string());
    });
    app.get("/contact", |_req, mut res| {
        res.send("Contact Route\n".to_string());
    });
    app.post("/user", |req, mut res| {
        println!("{:?}, \n{}", req.headers, req.post_body);
        res.send(req.post_body.to_string());
    });

    app.serve();

    println!("Hello, world!");
}
