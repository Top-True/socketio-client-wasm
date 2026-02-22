const SOCKET_IO_URL: &str = "https://cdn.socket.io/4.6.0/socket.io.min.js";

use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("socket.io.min.js");
    let client = reqwest::blocking::Client::new();
    let source = client.get(SOCKET_IO_URL).send().unwrap();
    if !source.status().is_success() {
        panic!("Request error: {}", source.status());
    }
    File::create(dest_path)
        .unwrap()
        .write_all(source.text().unwrap().as_bytes())
        .unwrap();
}
