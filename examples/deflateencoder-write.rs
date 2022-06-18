extern crate flate2_expose;

use flate2_expose::write::DeflateEncoder;
use flate2_expose::Compression;
use std::io::prelude::*;

// Vec<u8> implements Write to print the compressed bytes of sample string
fn main() {
    let mut e = DeflateEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"Hello World").unwrap();
    println!("{:?}", e.finish().unwrap());
}
