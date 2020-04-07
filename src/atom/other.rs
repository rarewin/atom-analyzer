use std::io::{Read, Seek, SeekFrom};

use std::error;

use crate::atom;

#[derive(Debug, PartialEq)]
pub struct OtherAtom {
    pub atom_head: atom::AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<OtherAtom, Box<dyn error::Error>> {
    let atom_head = atom::parse_atom_head(r)?;

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(OtherAtom { atom_head })
}
