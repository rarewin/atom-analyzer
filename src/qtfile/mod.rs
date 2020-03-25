use std::error;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use super::atom::{self, Atom};

#[derive(Debug, PartialEq)]
pub enum QtFileError {
    InvalidAtomSize,
}

#[derive(Debug)]
pub struct QtFile {
    atoms: Vec<Atom>,
}

pub fn get_atom_type<R: Read + Seek>(r: &mut R) -> Result<u32, Box<dyn error::Error>> {
    let atom_offset = r.seek(SeekFrom::Current(0))?;
    let atom_size = r.read_u32::<BigEndian>()? as u64;
    let atom_type = r.read_u32::<BigEndian>()?;

    if let Err(_) = r.seek(SeekFrom::Start(atom_offset + atom_size)) {
        return Err(Box::new(QtFileError::InvalidAtomSize));
    }

    r.seek(SeekFrom::Start(atom_offset))?;

    Ok(atom_type)
}

pub fn parse_qtfile(file_name: &str) -> Result<QtFile, Box<dyn error::Error>> {
    let f = File::open(file_name)?;
    let mut reader = BufReader::new(f);
    let mut atoms = Vec::<Atom>::new();

    for _ in 0..3 {
        match get_atom_type(&mut reader)? {
            0x66747970 => {
                atoms.push(atom::Atom::Ftyp(Box::new(
                    atom::ftyp::parse(&mut reader).unwrap(),
                )));
            }
            0x77696465 => {
                atoms.push(atom::Atom::Wide(Box::new(
                    atom::wide::parse(&mut reader).unwrap(),
                )));
            }
            0x6d646174 => {
                atoms.push(atom::Atom::Mdat(Box::new(
                    atom::mdat::parse(&mut reader).unwrap(),
                )));
            }
            _ => {}
        }
    }

    Ok(QtFile { atoms })
}

impl fmt::Display for QtFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(&(self as &dyn error::Error).to_string())
    }
}

impl error::Error for QtFileError {
    fn description(&self) -> &str {
        match *self {
            QtFileError::InvalidAtomSize => "atom size is invalid",
        }
    }
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
}