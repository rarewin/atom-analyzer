use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

// use clap::App;

fn main() {
    let f = File::open("test/hoge.mov").expect("file open error");
    let mut reader = BufReader::new(f);
    let mut buf = [0 as u8; 8];

    if let Ok(n) = reader.read(&mut buf) {
        let b = &buf[..n];
        println!("{:?}", b);
    }

    // let opts = App::new(env!("CARGO_PKG_NAME"));

    // let matches = opts.get_matches();
}
