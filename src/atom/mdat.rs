use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x6d64_6174; // 'mdat'

#[derive(Debug, PartialEq, Atom)]
pub struct MdatAtom {
    pub atom_head: AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<MdatAtom, AtomParseError> {
    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(MdatAtom { atom_head })
}
