#![no_main]
use libfuzzer_sys::fuzz_target;
use vial::Request;

fuzz_target!(|data: &[u8]| {
    let x = vial::Request::from_reader(data);
});
