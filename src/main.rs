mod encode;
mod decode;

use encode::{encode};
use decode::{decode};
use std::{io::{self, Write}};

trait Codec {
    fn encode(&self, input: &str) -> Vec<u32>;
    fn decode(&self, input: &[u32]) -> String;
}

enum Methods {
    Encode,
    Decode,
}

struct Converter;

impl Codec for Converter {
    fn encode(&self, input: &str) -> Vec<u32> {encode(&input)}
    fn decode(&self, input: &[u32]) -> String {decode(&input)}
}

impl Converter {
    fn run(&self, method: Methods, input: &str) {
        match method {
            Methods::Encode => {
                for val in self.encode(&input) {
                    print!("{} ", val);
                }
                println!();
            },

            Methods::Decode => {
                let numbers: Vec<u32> = input
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();
                // println!("{}", self.decode(&numbers))
                println!("{}", self.decode(&numbers));
            },
        }
    }
}

fn main () {

    print!("Mode: ");
    io::stdout().flush().unwrap();
    let mut mode = String::new();
    io::stdin().read_line(&mut mode).expect("Failed to read line");

    print!("Enter text to convert to binary: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    match mode.trim() {
        "encode" => Converter.run(Methods::Encode, input.trim()),
        "decode" => Converter.run(Methods::Decode, input.trim()),
        _ => println!("Unknown mode"),
    }
}