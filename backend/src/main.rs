#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;

mod controllers;
mod db;
mod schema;
mod services;

use actix_web::{App, HttpServer};
use controllers::movies;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\x1b[32m Server Starting 🕑\x1b[0m");
    let server = HttpServer::new(|| App::new().configure(movies::router))
        .bind(("127.0.0.1", 8080))?
        .run();

    print!("\x1B[2J\x1B[1;1H");

    println!(
        "\x1b[32m Server running on port \x1b[1m\x1b[33m[{}] 🚀\x1b[0m",
        8080
    );
    server.await?;

    print!("\x1B[2J\x1B[1;1H");
    println!("Server closing 🕑");

    Ok(())
}
