use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};
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
    atoms: Vec<Atom>,
}

pub fn get_atom_type<R: Read + Seek>(r: &mut R) -> Result<u32, QtFileError> {
    let atom_offset = r.seek(SeekFrom::Current(0))?;
    let atom_size = r.read_u32::<BigEndian>()? as u64;
    let atom_type = r.read_u32::<BigEndian>()?;

    if r.seek(SeekFrom::Start(atom_offset + atom_size)).is_err() {
        return Err(QtFileError::InvalidAtomSize(atom_size));
    }

    r.seek(SeekFrom::Start(atom_offset))?;

    Ok(atom_type)
}

pub fn parse_qtfile(file_name: &str) -> Result<QtFile, QtFileError> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut atoms = Vec::<Atom>::new();

    for _ in 0..4 {
        atoms.push(atom::parse(&mut reader)?)
    }

    Ok(QtFile { atoms })
}

#[cfg(test)]
mod test_qtfile {

    use crate::qtfile;

    use std::io::Cursor;

    #[test]
    fn test_invalid_atom_type() {
        let test: Vec<u8> = vec![0x10, 0x10, 0x10, 0x10];
        let mut buf = Cursor::new(test);

        let atom = qtfile::get_atom_type(&mut buf);

        assert!(atom.is_err());
    }

    #[test]
    fn test_parse_camouflage_vga_mov() {
        let q = qtfile::parse_qtfile("tests/samples/camouflage_vga.mov");

        println!("{:#?}", q);
    }
}
