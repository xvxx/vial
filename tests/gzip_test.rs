#[cfg(feature = "compression")]
use {
    std::fs,
    vial::{Compression, Request},
};

#[test]
#[cfg(feature = "compression")]
fn gzip_header_test() {
    let request = Request::from_reader(fs::File::open("tests/http/gzip_GET.txt").unwrap()).unwrap();
    assert_eq!(request.compression(), Some(Compression::Gzip));
}
