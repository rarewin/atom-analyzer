use std::io::{Read, Seek, SeekFrom};

use std::error;

use crate::atom;

pub const ATOM_ID: u32 = 0x77696465; // 'wide'

#[derive(Debug, PartialEq)]
pub struct WideAtom {
    pub atom_head: atom::AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<WideAtom, Box<dyn error::Error>> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(Box::new(atom::AtomSeekError::TypeError));
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(WideAtom { atom_head })
}
