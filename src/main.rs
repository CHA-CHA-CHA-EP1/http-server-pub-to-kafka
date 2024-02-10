use actix_web::{HttpServer, App, get};

use http_serv_to_kafka::configs;

#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let get_env = match configs::config::get_env() {
        Ok(get_env) => get_env,
        Err(e) => {
            println!("Error: {:?}", e);
            std::process::exit(1);
        }
    };

    println!("Server port: {}", get_env.server_port);

    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind(("0.0.0.0", 24132))?
    .run()
    .await
}
