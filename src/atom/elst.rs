use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, AtomParseError};

pub const ATOM_ID: u32 = 0x65_6c_73_74; // 'elst'

#[derive(Debug, PartialEq)]
pub struct ElstAtom {
    pub atom_head: atom::AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<ElstAtom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;

    let atom_type = atom_head.atom_type;
    let atom_offset = atom_head.atom_offset;
    let atom_size = atom_head.atom_size;

    if atom_type != ATOM_ID {
        return Err(atom::AtomParseError::TypeError(atom_offset + 4));
    }

    r.seek(SeekFrom::Start(atom_offset + atom_size))?;

    Ok(ElstAtom { atom_head })
}