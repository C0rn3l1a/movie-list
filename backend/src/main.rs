#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod schema;

use crate::db::establish_connection;
use db::movie::{create_movie, Movie};
use diesel::prelude::*;
use std::io::stdin;

fn main() {
    use schema::movies::dsl::*;
    let connection = establish_connection();

    println!("Whats the name of the movie?");
    let mut title = String::new();
    stdin().read_line(&mut title).unwrap();
    let title = &title[..(title.len() - 1)]; // Drop the newline character

    println!("Did you watch the movie? Y/N");
    let mut seenstr = String::new();
    stdin().read_line(&mut seenstr).unwrap();
    let seenstr = &seenstr[..(seenstr.len() - 1)]; // Drop the newline character
    let seenbool = match seenstr {
        "Y" => true,
        "N" => false,
        _ => false,
    };

    println!("Who is the owner?");
    let mut owner = String::new();
    stdin().read_line(&mut owner).unwrap();
    let owner = &owner[..(owner.len() - 1)]; // Drop the newline character
    let ownerid = match owner.parse::<i32>() {
        Ok(n) => n,
        Err(_) => 1,
    };

    let movie = create_movie(&connection, title, &seenbool, &ownerid);
    println!("\nSaved draft {} with id {}", title, movie.id);
    println!("==============================");

    let results = movies
        .load::<Movie>(&connection)
        .expect("Error loading movies");

    println!("Displaying {} movies", results.len());
    println!("==========");
    for movie in results {
        println!("name: {} | seen: {}", movie.name, movie.seen);
        println!("----------\n");
    }
}
