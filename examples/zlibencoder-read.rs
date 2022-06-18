extern crate flate2_expose;

use flate2_expose::read::ZlibEncoder;
use flate2_expose::Compression;
use std::fs::File;
use std::io::prelude::*;

// Open file and debug print the compressed contents
fn main() {
    println!("{:?}", open_hello_world().unwrap());
}

// Opens sample file, compresses the contents and returns a Vector or error
// File implements Read
fn open_hello_world() -> std::io::Result<Vec<u8>> {
    let f = File::open("examples/hello_world.txt")?;
    let mut z = ZlibEncoder::new(f, Compression::fast());
    let mut result = Vec::new();
    z.read_to_end(&mut result)?;
    Ok(result)
}
