use std::io::{Read, Seek, SeekFrom};

use crate::atom;

pub const ATOM_ID: u32 = 0x6d64_6174; // 'mdat'

#[derive(Debug, PartialEq)]
pub struct MdatAtom {
    pub atom_offset: u64,
    pub atom_size: u64,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MdatAtom, atom::AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;

    let atom_offset = atom_head.atom_offset;
    let atom_size = atom_head.atom_size;
    let atom_type = atom_head.atom_type;

    if atom_type != ATOM_ID {
        return Err(atom::AtomParseError::TypeError(atom_offset));
    }

    r.seek(SeekFrom::Start(atom_offset + atom_size))?;

    Ok(MdatAtom {
        atom_offset,
        atom_size,
    })
}
