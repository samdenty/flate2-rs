extern crate flate2_expose;

use flate2_expose::bufread::ZlibDecoder;
use flate2_expose::write::ZlibEncoder;
use flate2_expose::Compression;
use std::io;
use std::io::prelude::*;

// Compress a sample string and print it after transformation.
fn main() {
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(b"Hello World").unwrap();
    let bytes = e.finish().unwrap();
    println!("{}", decode_bufreader(bytes).unwrap());
}

// Uncompresses a Zlib Encoded vector of bytes and returns a string or error
// Here &[u8] implements BufRead
fn decode_bufreader(bytes: Vec<u8>) -> io::Result<String> {
    let mut z = ZlibDecoder::new(&bytes[..]);
    let mut s = String::new();
    z.read_to_string(&mut s)?;
    Ok(s)
}
