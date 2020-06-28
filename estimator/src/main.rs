// https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html
// systemfd --no-pid -s http::8000 -- cargo watch -x run
use actix_web::{web, App, HttpServer, Responder};
use anyhow::Result;
use listenfd::ListenFd;
use sqlx::PgPool;

async fn index(db_pool: web::Data<PgPool>) -> impl Responder {
    // let result = sqlx::query!(
    //     r#"
    //         SELECT *
    //         FROM assemblies
    //         WHERE id = $1
    //     "#,
    //     2
    // )
    // .fetch_one(db_pool.get_ref())
    // .await;
    //
    // match result {
    //     Ok(r) => println!("{}", r.name),
    //     _ => println!("Nothing found"),
    // }

    // let mut assemblies = vec![];
    // let res = sqlx::query!(
    //     r#"
    //         SELECT *
    //         FROM assemblies
    //     "#,
    // )
    // .fetch_all(db_pool.get_ref())
    // .await;
    // // .unwrap()
    //
    // match res {
    //     Ok(r) => r.into_iter().for_each(|a| assemblies.push(a.name)),
    //     _ => println!("Nothing found"),
    // }
    //
    // assemblies.iter().for_each(|name| println!("{}", name));

    #[derive(Debug)]
    struct Assembly {
        name: String,
    }

    let res = sqlx::query_as!(
        Assembly,
        r#"
            SELECT name
            FROM assemblies
        "#,
    )
    .fetch_all(db_pool.get_ref())
    .await
    .unwrap();

    println!("{:?}", res);

    "Hello world!"
}

#[actix_rt::main]
async fn main() -> Result<()> {
    let mut listenfd = ListenFd::from_env();

    let db_pool = PgPool::new("postgres://postgres:postgres@localhost/rust").await?;

    let mut server = HttpServer::new(move || {
        App::new()
            .data(db_pool.clone())
            .route("/", web::get().to(index))
    });

    server = match listenfd.take_tcp_listener(0).unwrap() {
        Some(listener) => server.listen(listener)?,
        None => server.bind("127.0.0.1:8000")?,
    };

    println!("Starting server");
    server.run().await?;

    Ok(())
}
