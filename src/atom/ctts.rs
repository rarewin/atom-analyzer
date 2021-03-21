use std::fmt::Debug;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x6374_7473; // 'ctts'

#[atom(version)]
#[derive(Debug, PartialEq)]
pub struct CttsAtom {
    pub entry_count: u32,
    pub composition_offset_table: Vec<CompositionOffsetTableEntry>,
}

#[derive(Debug, PartialEq)]
pub struct CompositionOffsetTableEntry {
    pub sample_count: u32,
    pub composition_offset: u32,
}

impl CompositionOffsetTableEntry {
    pub fn new(sample_count: u32, composition_offset: u32) -> Self {
        Self {
            sample_count,
            composition_offset,
        }
    }
}

pub fn parse<R: Read>(r: &mut R, atom_head: AtomHead) -> Result<CttsAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let entry_count = r.read_u32::<BigEndian>()?;

    let mut composition_offset_table = Vec::new();

    for _ in 0..entry_count {
        let sample_count = r.read_u32::<BigEndian>()?;
        let composition_offset = r.read_u32::<BigEndian>()?;
        composition_offset_table.push(CompositionOffsetTableEntry {
            sample_count,
            composition_offset,
        });
    }

    Ok(CttsAtom {
        atom_head,
        atom_version,
        atom_flags,
        entry_count,
        composition_offset_table,
    })
}
