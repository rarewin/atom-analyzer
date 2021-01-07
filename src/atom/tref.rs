use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x7472_6566; // 'tref'

#[derive(Debug, PartialEq)]
pub struct TrefAtom {
    pub atom_head: AtomHead,
}

impl Atom for TrefAtom {}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<TrefAtom, AtomParseError> {
    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(TrefAtom { atom_head })
}
