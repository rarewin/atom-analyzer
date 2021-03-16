use std::fmt::Debug;
use std::io::{Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x7374_7473; // 'stts'

#[derive(Debug, PartialEq)]
pub struct TimeToSampleEntry {
    pub sample_count: u32,
    pub sample_duration: u32,
}

#[derive(Debug, PartialEq, Atom)]
pub struct SttsAtom {
    pub atom_head: AtomHead,
    pub atom_version: u8,
    pub atom_flags: [u8; 3],
    pub number_of_entries: u32,
    pub time_to_sample_table: Vec<TimeToSampleEntry>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<SttsAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let number_of_entries = r.read_u32::<BigEndian>()?;

    let mut time_to_sample_table = Vec::new();

    for _ in 0..number_of_entries {
        let sample_count = r.read_u32::<BigEndian>()?;
        let sample_duration = r.read_u32::<BigEndian>()?;
        time_to_sample_table.push(TimeToSampleEntry {
            sample_count,
            sample_duration,
        });
    }

    Ok(SttsAtom {
        atom_head,
        atom_version,
        atom_flags,
        number_of_entries,
        time_to_sample_table,
    })
}
