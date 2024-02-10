use actix_web::{HttpServer, App, get};

use http_serv_to_kafka::configs;

#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let get_env = configs::config::get_env();
    println!("Config: {:?}", get_env);
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("0.0.0.0", 24132))?
    .run()
    .await
}
