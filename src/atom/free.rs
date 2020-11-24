use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomParseError};

pub const ATOM_ID: u32 = 0x6672_6565; // 'free'

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Atom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;

    let atom_offset = atom_head.atom_offset;
    let atom_size = atom_head.atom_size;
    let atom_type = atom_head.atom_type;

    if atom_type != ATOM_ID {
        return Err(atom::AtomParseError::TypeError(atom_offset + 4));
    }

    r.seek(SeekFrom::Start(atom_offset + atom_size))?;

    Ok(Atom::Free { atom_head })
}
