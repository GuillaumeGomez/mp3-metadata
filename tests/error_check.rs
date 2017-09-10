extern crate mp3_metadata;

mod common;

#[test]
fn error_check() {
    common::get_file("assets/error.mp3");
    let meta = mp3_metadata::read_from_file("assets/error.mp3");//.expect("File error");
}
