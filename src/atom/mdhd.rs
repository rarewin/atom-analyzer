use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x6d64_6864; // 'mdhd'

#[derive(Debug, PartialEq, Atom)]
pub struct MdhdAtom {
    pub atom_head: AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<MdhdAtom, AtomParseError> {
    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(MdhdAtom { atom_head })
}
