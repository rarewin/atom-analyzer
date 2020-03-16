use std::fs::File;
use std::io::BufReader;

// use clap::App;

use atom_analyzer::atom::ftyp;

fn main() {
    let f = File::open("./test/hoge.mov").expect("file open error");
    let mut reader = BufReader::new(f);

    // let opts = App::new(env!("CARGO_PKG_NAME"));

    let t = ftyp::parse(&mut reader, 0);

    println!("{:?}", t);
}
