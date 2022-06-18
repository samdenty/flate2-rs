extern crate flate2_expose;

#[test]
fn zero_write_is_error() {
    let mut buf = [0u8];
    let writer = flate2_expose::write::DeflateEncoder::new(
        &mut buf[..],
        flate2_expose::Compression::default(),
    );
    assert!(writer.finish().is_err());
}
