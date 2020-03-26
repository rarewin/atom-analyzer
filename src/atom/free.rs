use std::io::{Read, Seek, SeekFrom};

use std::error;

use crate::atom;

pub const ATOM_ID: u32 = 0x66726565; // 'free'

#[derive(Debug, PartialEq)]
pub struct FreeAtom {
    pub atom_offset: u64,
    pub atom_size: u64,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<FreeAtom, Box<dyn error::Error>> {
    let atom_head = atom::parse_atom_head(r)?;

    let atom_offset = atom_head.atom_offset;
    let atom_size = atom_head.atom_size;
    let atom_type = atom_head.atom_type;

    if atom_type != ATOM_ID {
        return Err(Box::new(atom::AtomSeekError::TypeError));
    }

    r.seek(SeekFrom::Start(atom_offset + atom_size))?;

    Ok(FreeAtom {
        atom_offset,
        atom_size,
    })
}