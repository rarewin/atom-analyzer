use std::cell::RefCell;
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::rc::Rc;

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
    atoms: Vec<Rc<RefCell<Box<dyn Atom>>>>,
}

impl std::iter::IntoIterator for QtFile {
    type Item = Rc<RefCell<Box<dyn Atom>>>;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.atoms.into_iter()
    }
}

pub fn parse_file(file_name: PathBuf) -> Result<QtFile, QtFileError> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut atoms = Vec::<Rc<RefCell<Box<dyn Atom>>>>::new();

    loop {
        match atom::parse(&mut reader) {
            Ok(a) => atoms.push(Rc::new(RefCell::new(a))),
            Err(atom::AtomParseError::NoMoreAtom) => break,
            Err(e) => {
                return Err(QtFileError::AtomParseError(e));
            }
        }
    }

    Ok(QtFile { atoms })
}
