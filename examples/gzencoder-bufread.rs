extern crate flate2_expose;

use flate2_expose::bufread::GzEncoder;
use flate2_expose::Compression;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

// Open file and debug print the contents compressed with gzip
fn main() {
    println!("{:?}", open_hello_world().unwrap());
}

// Opens sample file, compresses the contents and returns a Vector or error
// File wrapped in a BufReader implements Bufread
fn open_hello_world() -> io::Result<Vec<u8>> {
    let f = File::open("examples/hello_world.txt")?;
    let b = BufReader::new(f);
    let mut gz = GzEncoder::new(b, Compression::fast());
    let mut buffer = Vec::new();
    gz.read_to_end(&mut buffer)?;
    Ok(buffer)
}
