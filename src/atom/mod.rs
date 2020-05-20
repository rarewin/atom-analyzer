use std::error;
use std::fmt;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct AtomHead {
    pub atom_type: u32,
    pub atom_size: u64,
    pub atom_offset: u64,
}

#[derive(Debug, PartialEq)]
pub enum Atom {
    Moov { atom_head: AtomHead },
    Ftyp { atom_head: AtomHead },
    General { atom_head: AtomHead },
}

#[derive(Error, Debug)]
pub enum AtomParseError {
    #[error("failed to seek")]
    SeekError,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Atom, Box<dyn error::Error>> {
    let atom_offset = r.seek(SeekFrom::Current(0))?;
    let atom_size = r.read_u32::<BigEndian>()? as u64;
    let atom_type = r.read_u32::<BigEndian>()?;

    if atom_size == 0 {
        unimplemented!("atom with zero size is not implemented yet");
    }

    // extended size
    let atom_size = if atom_size == 1 {
        r.read_u64::<BigEndian>().unwrap()
    } else {
        atom_size
    };

    let atom_head = AtomHead {
        atom_offset,
        atom_size,
        atom_type,
    };

    match atom_type {
        _ => {
            r.seek(SeekFrom::Start(atom_offset + atom_size))?;
            Ok(Atom::General { atom_head })
        }
    }
}

#[cfg(test)]
mod test_atom {
    #[test]
    fn test_free_atom() {}
}
