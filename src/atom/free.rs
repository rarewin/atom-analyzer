use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x6672_6565; // 'free'

#[derive(Debug, PartialEq, Atom)]
pub struct FreeAtom {
    pub atom_head: AtomHead,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<FreeAtom, AtomParseError> {
    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(FreeAtom { atom_head })
}
