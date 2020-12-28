use std::fmt::Debug;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x7374_636f; // 'stco'

#[derive(Debug, PartialEq)]
pub struct StcoAtom {
    pub atom_head: AtomHead,
    pub atom_version: u8,
    pub atom_flags: [u8; 3],
    pub number_of_entries: u32,
    pub chunk_offset_table: Vec<u32>,
}

impl Atom for StcoAtom {}

pub fn parse<R: Read>(r: &mut R, atom_head: AtomHead) -> Result<StcoAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let number_of_entries = r.read_u32::<BigEndian>()?;

    let mut chunk_offset_table = Vec::new();

    for _ in 0..number_of_entries {
        chunk_offset_table.push(r.read_u32::<BigEndian>()?);
    }

    Ok(StcoAtom {
        atom_head,
        atom_version,
        atom_flags,
        number_of_entries,
        chunk_offset_table,
    })
}
