mod encode;
mod decode;

use encode::{encode};
use decode::{decode};
use std::{fs::{self, File}, io::{self, Write}};

trait Codec {
    fn encode(&self, input: &String) -> Vec<u32>;
    // fn decode(&self, input: &Vec<u32>) -> String;
}

enum Methods {
    Encode,
    // Decode,
}

struct Converter;

impl Codec for Converter {
    fn encode(&self, input: &String) -> Vec<u32> {encode(&input)}
}

impl Converter {
    fn run(&self, method: Methods, input: &String) -> Vec<u32>{
        match method {
            Methods::Encode => self.encode(input),
            // Methods::Decode => self.decode(input),
        }
    }
}

fn main () {

    print!("Enter text to convert to binary: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    // let input = fs::read_to_string("./src/input.txt").expect("Failed to read line");

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let binary_values = Converter.run(Methods::Encode, &input);

    // let mut file = File::create("./src/numbers.txt").unwrap();
    // for n in &binary_values {
        // if *n == 1010 {
            // writeln!(file, "{} ", n).unwrap();
            // continue;
        // }
        // write!(file, "{} ", n).unwrap();
    // }

    for (idx, i) in binary_values.iter().enumerate() {
        if idx == binary_values.len() - 1 {
            break;
        }

        print!("{} ",i);
        io::stdout().flush().unwrap();
    }
    print!("\n")

}