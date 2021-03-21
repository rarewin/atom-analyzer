use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x766d_6864; // 'vmhd'

#[atom]
#[derive(Debug, PartialEq)]
pub struct VmhdAtom {}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<VmhdAtom, AtomParseError> {
    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(VmhdAtom { atom_head })
}
