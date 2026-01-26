use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("socket.io.min.js");
    let client = reqwest::blocking::Client::new();
    let source = client
        .get("https://cdn.socket.io/4.0.1/socket.io.min.js")
        .send()
        .unwrap();
    if !source.status().is_success() {
        panic!("Request error: {}", source.status());
    }
    File::create(dest_path)
        .unwrap()
        .write_all(source.text().unwrap().as_bytes())
        .unwrap();
}
