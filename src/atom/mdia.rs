use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, AtomParseError};

pub const ATOM_ID: u32 = 0x6d_64_69_61; // 'mdia'

#[derive(Debug, PartialEq)]
pub struct MdiaAtom {
    pub atom_head: atom::AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MdiaAtom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(atom::AtomParseError::TypeError(atom_head.atom_offset));
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(MdiaAtom { atom_head })
}
