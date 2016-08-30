extern crate mp3_header;

#[test]
fn basic() {
    let frames = mp3_header::read_from_file("assets/normal.mp3").expect("File error");
    let mut out = String::new();
    for frame in frames {
        out = format!("{}{:?}\n", &out, frame);
    }
    panic!("{}", out);
}
