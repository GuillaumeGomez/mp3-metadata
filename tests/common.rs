extern crate reqwest;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn get_file<P: AsRef<Path>>(p: P) {
    let p = p.as_ref();
    if p.exists() {
        return;
    }
    let mut resp = reqwest::blocking::get(&format!("https://guillaume-gomez.fr/rust-test/{}",
                                         p.file_name()
                                          .expect("file_name() failed")
                                          .to_str()
                                          .expect("to_str() failed")))
                           .expect("reqwest::get() failed");
    assert!(resp.status().is_success());

    let mut content = Vec::new();
    resp.read_to_end(&mut content).expect("read_to_string() failed");

    let mut file = File::create(p).expect("cannot create file");
    file.write_all(&content).expect("write_all() failed");
}
