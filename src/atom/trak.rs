use std::error;
use std::io::{Read, Seek, SeekFrom};

use crate::atom;

pub const ATOM_ID: u32 = 0x7472616b; // 'trak'

#[derive(Debug, PartialEq)]
pub struct TrakAtom {
    pub atom_head: atom::AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<TrakAtom, Box<dyn error::Error>> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(Box::new(atom::AtomSeekError::TypeError));
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(TrakAtom { atom_head })
}
