// https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
// systemfd --no-pid -s http::8000 -- cargo watch -x run
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use listenfd::ListenFd;

async fn index(_req: HttpRequest) -> impl Responder {
    "Hello World!"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().route("/", web::get().to(index)));

    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)?
    } else {
        server.bind("127.0.0.1:8000")?
    };

    println!("Starting server");
    server.run().await
}
