use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Seek, SeekFrom};
use std::path::PathBuf;

use byteorder::{BigEndian, ReadBytesExt};
use thiserror::Error;

use super::atom::{self, Atom};

#[derive(Error, Debug)]
pub enum QtFileError {
    #[error("atom size `{0}' is invalid")]
    InvalidAtomSize(u64),

    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    AtomSeekError(#[from] atom::AtomSeekError),
}

#[derive(Debug)]
pub struct QtFile {
    atoms: Vec<Atom>,
}

impl Iterator for QtFile {
    type Item = Atom;

    fn next(&mut self) -> Option<Self::Item> {
        self.atoms.reverse();
        let atom = self.atoms.pop();
        self.atoms.reverse();

        atom
    }
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

pub fn parse_file(file_name: PathBuf) -> Result<QtFile, QtFileError> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut atoms = Vec::<Atom>::new();

    loop {
        match atom::parse(&mut reader) {
            Ok(a) => atoms.push(a),
            Err(atom::AtomSeekError::NoMoreAtom) => break,
            Err(e) => {
                return Err(QtFileError::AtomSeekError(e));
            }
        }
    }

    Ok(QtFile { atoms })
}

#[cfg(test)]
mod test_qtfile {

    use crate::atom;
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
        let mut q = qtfile::parse_file("tests/samples/camouflage_vga.mov".into()).unwrap();

        // println!("{:#?}", q);

        let ftyp = q.next();

        assert_eq!(
            ftyp,
            Some(atom::Atom::Ftyp(Box::new(atom::ftyp::FtypAtom {
                atom_offset: 0,
                atom_size: 0x14,
                major_brand: atom::ftyp::Brand::QuickTimeMovieFile,
                minor_version: 0x0200,
                compatible_brands: vec![atom::ftyp::Brand::QuickTimeMovieFile],
            })))
        );
    }
}
