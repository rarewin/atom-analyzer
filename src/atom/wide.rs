use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x7769_6465; // 'wide'

#[derive(Debug, PartialEq)]
pub struct WideAtom {
    pub atom_head: AtomHead,
}

impl Atom for WideAtom {}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<WideAtom, AtomParseError> {
    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(WideAtom { atom_head })
}
