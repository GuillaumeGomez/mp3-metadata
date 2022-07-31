#![feature(test)]

extern crate test;

use std::fs::File;
use std::io::Read;

#[bench]
fn bench_read_from_slice(b: &mut test::Bencher) {
    let mut buf = Vec::new();
    match File::open("assets/test.mp3") {
        Ok(mut fd) => {
            fd.read_to_end(&mut buf).expect("read_to_end failed");
        }
        Err(e) => panic!("File::open failed: {:?}", e),
    }
    b.iter(|| {
        mp3_metadata::read_from_slice(&buf).expect("read_from_slice failed");
    });
}
