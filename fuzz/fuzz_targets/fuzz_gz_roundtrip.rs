#![no_main]
use flate2_expose::read::GzDecoder;
use flate2_expose::write::GzEncoder;
use flate2_expose::Compression;
use libfuzzer_sys::fuzz_target;
use std::io::prelude::*;

fuzz_target!(|data: &[u8]| {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    let result = encoder.finish().unwrap();
    let mut r = GzDecoder::new(&result[..]);
    let mut ret = Vec::new();
    r.read_to_end(&mut ret).unwrap();
    assert!(ret == data);
});
