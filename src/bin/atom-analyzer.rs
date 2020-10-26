use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use atom_analyzer::qtfile;

#[derive(Clap)]
#[clap(name=env!("CARGO_PKG_NAME"))]
struct Opts {
    #[clap(name = "INPUT")]
    input: PathBuf,
}

fn main() -> Result<()> {
    let opts = Opts::parse();

    let t = qtfile::parse_file(opts.input)?;
    println!("{:#?}", t);

    Ok(())
}
