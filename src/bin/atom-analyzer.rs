use std::fs::File;
use std::io::BufReader;

use anyhow::Result;
use clap::{App, Arg};

use atom_analyzer::atom;

fn main() -> Result<()> {
    let m = App::new(env!("CARGO_PKG_NAME"))
        .arg(
            Arg::with_name("INPUT")
                .help("Sets the input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let file_name = m.value_of("INPUT").unwrap();
    let f = File::open(file_name).expect("file open error");
    let mut reader = BufReader::new(f);

    loop {
        let t = atom::parse(&mut reader)?;

        println!("{:#?}", t);
    }
}
