use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use clap::Clap;

use atom_analyzer::atom::{self, AtomParseError};

#[derive(Clap)]
#[clap(name=env!("CARGO_PKG_NAME"))]
struct Opts {
    #[clap(name = "INPUT")]
    input: PathBuf,
}

fn main() -> Result<(), AtomParseError> {
    let opts = Opts::parse();

    let f = File::open(opts.input)?;
    let mut reader = BufReader::new(f);

    loop {
        let t = atom::parse(&mut reader)?;

        println!("{:#?}", t);
    }
}
