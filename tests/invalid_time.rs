extern crate mp3_metadata;

use std::time::Duration;

mod common;

#[test]
fn invalid_time() {
    common::get_file("assets/invalid_time.mp3");
    let meta = mp3_metadata::read_from_file("assets/invalid_time.mp3").expect("File error");
    assert_eq!(meta.duration, Duration::new(162, 434285248));
}
