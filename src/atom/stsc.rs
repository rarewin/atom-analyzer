use std::fmt::Debug;
use std::io::Read;

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x7374_7363; // 'stsc'

#[derive(Debug, PartialEq)]
pub struct StscAtom {
    pub atom_head: AtomHead,
    pub atom_version: u8,
    pub atom_flags: [u8; 3],
    pub number_of_entries: u32,
    pub sample_to_chunk_table: Vec<SampleToChunk>,
}

#[derive(Debug, PartialEq)]
pub struct SampleToChunk {
    pub first_chunk: u32,
    pub samples_per_chunk: u32,
    pub sample_description_id: u32,
}

impl SampleToChunk {
    pub fn new(first_chunk: u32, samples_per_chunk: u32, sample_description_id: u32) -> Self {
        Self {
            first_chunk,
            samples_per_chunk,
            sample_description_id,
        }
    }
}

impl Atom for StscAtom {}

pub fn parse<R: Read>(r: &mut R, atom_head: AtomHead) -> Result<StscAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let number_of_entries = r.read_u32::<BigEndian>()?;

    let mut sample_to_chunk_table = Vec::new();

    for _ in 0..number_of_entries {
        let first_chunk = r.read_u32::<BigEndian>()?;
        let samples_per_chunk = r.read_u32::<BigEndian>()?;
        let sample_description_id = r.read_u32::<BigEndian>()?;
        sample_to_chunk_table.push(SampleToChunk::new(
            first_chunk,
            samples_per_chunk,
            sample_description_id,
        ));
    }

    Ok(StscAtom {
        atom_head,
        atom_version,
        atom_flags,
        number_of_entries,
        sample_to_chunk_table,
    })
}
