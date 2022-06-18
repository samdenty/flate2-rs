extern crate flate2_expose;

use flate2_expose::read::DeflateEncoder;
use flate2_expose::Compression;
use std::io;
use std::io::prelude::*;

// Print the Deflate compressed representation of hello world
fn main() {
    println!("{:?}", deflateencoder_read_hello_world().unwrap());
}

// Return a vector containing the Deflate compressed version of hello world
fn deflateencoder_read_hello_world() -> io::Result<Vec<u8>> {
    let mut result = Vec::new();
    let c = b"hello world";
    let mut deflater = DeflateEncoder::new(&c[..], Compression::fast());
    deflater.read_to_end(&mut result)?;
    Ok(result)
}
