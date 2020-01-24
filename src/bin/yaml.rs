/*
 * Simple example of decoding a .yaml file using deserialize
 * YAML into a Rust struct
 */

extern crate serde; // 1.0

use serde::{Deserialize};
use std::fs;

#[derive(Debug, Deserialize)]
struct Config {
    global_name:    Option<String>,
    address:        Option<Address>,
    country:        Option<Country>
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
    // Read config.yaml into a string `data` to then parse into yaml::from_str
    let data = fs::read_to_string("./config.yaml").expect("Unable to read file");

    let decoded: Config = serde_yaml::from_str(&data).unwrap();
    println!("{:#?}", decoded)
    // Can now access decoded.country etc
}