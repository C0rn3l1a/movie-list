#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;

mod controllers;
mod db;
mod schema;
mod services;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer};
use controllers::movies;
use env_logger::fmt::Formatter;
// use env_logger::Env;
use log::{Level, LevelFilter, Record};
use std::io::Write;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\x1b[32m Server Starting 🕑\x1b[0m");

    env_logger::builder()
        .format_timestamp(None)
        .filter_level(LevelFilter::Info)
        .format(|buf, record| logger_format(buf, record))
        .init();

    let server = HttpServer::new(|| {
        let cors = Cors::default()
            .supports_credentials()
            .allow_any_origin()
            .allow_any_header()
            .allow_any_method();

        App::new()
            .wrap(cors)
            .wrap(Logger::new("%a \"%r\" (%T)"))
            .configure(movies::router)
    })
    .bind(("127.0.0.1", 3000))?
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

fn logger_format(buf: &mut Formatter, record: &Record) -> Result<(), std::io::Error> {
    let log_color = match record.level() {
        Level::Info => "\x1b[34m",
        Level::Error => "\x1b[31m",
        Level::Warn => "\x1b[33m",
        Level::Trace => "\x1b[32m",
        Level::Debug => "\x1b[37m\x1b[5m",
    };
    let clean_format = "\x1b[0m";
    let dim_format = "\x1b[2m";

    writeln!(
        buf,
        "{}[{}]:{}{} {}{}",
        log_color,
        record.level(),
        clean_format,
        dim_format,
        record.args(),
        clean_format
    )
}
