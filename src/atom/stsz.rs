use std::fmt::Debug;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x7374_737a; // 'stsz'

#[derive(Debug, PartialEq, Atom)]
pub struct StszAtom {
    pub atom_head: AtomHead,
    pub atom_version: u8,
    pub atom_flags: [u8; 3],
    pub sample_size: u32,
    pub number_of_entries: u32,
    pub sample_size_table: Vec<u32>,
}

pub fn parse<R: Read>(r: &mut R, atom_head: AtomHead) -> Result<StszAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let sample_size = r.read_u32::<BigEndian>()?;
    let number_of_entries = r.read_u32::<BigEndian>()?;

    let mut sample_size_table = Vec::new();

    for _ in 0..number_of_entries {
        sample_size_table.push(r.read_u32::<BigEndian>()?);
    }

    Ok(StszAtom {
        atom_head,
        atom_version,
        atom_flags,
        sample_size,
        number_of_entries,
        sample_size_table,
    })
}
