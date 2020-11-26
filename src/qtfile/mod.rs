use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;

use thiserror::Error;

use super::atom::{self, Atom, AtomParseError};

#[derive(Error, Debug)]
pub enum QtFileError {
    #[error("atom size `{0}' is invalid")]
    InvalidAtomSize(u64),

    #[error(transparent)]
    AtomParseError(#[from] AtomParseError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

#[derive(Debug)]
pub struct QtFile {
    atoms: Vec<Box<dyn Atom>>,
}

impl Iterator for QtFile {
    type Item = Box<dyn Atom>;

    fn next(&mut self) -> Option<Self::Item> {
        self.atoms.reverse();
        let atom = self.atoms.pop();
        self.atoms.reverse();

        atom
    }
}

pub fn parse_file(file_name: PathBuf) -> Result<QtFile, QtFileError> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut atoms = Vec::<Box<dyn Atom>>::new();

    loop {
        match atom::parse(&mut reader) {
            Ok(a) => atoms.push(a),
            Err(atom::AtomParseError::NoMoreAtom) => break,
            Err(e) => {
                return Err(QtFileError::AtomParseError(e));
            }
        }
    }

    Ok(QtFile { atoms })
}
