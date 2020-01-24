/*
 * Simple example of decoding a .toml file using deserialize
 * TOML into a Rust struct
 */

extern crate toml; // 0.5.0
extern crate serde; // 1.0

use serde::{Deserialize};
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    global_name:    Option<String>,
    address:        Option<Address>,
    country:        Option<Vec<Country>>
}

#[derive(Debug, Deserialize)]
struct Address {
    number:         Option<u64>,
    street:         Option<String>
}

#[derive(Debug, Deserialize)]
struct Country {
    isocode:        Option<u64>,
    isostring:      Option<String>
}

fn main() {
    // Read config.toml into a string `data` to then parse into toml::from_str
    let data = fs::read_to_string("./config.toml").expect("Unable to read file");

    let decoded: Config = toml::from_str(&data).unwrap();
    println!("{:#?}", decoded)
    // Can now access decoded.country etc
}