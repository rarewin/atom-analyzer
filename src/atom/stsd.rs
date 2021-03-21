use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x7374_7364; // 'stsd'

#[derive(Debug, PartialEq)]
pub struct SampleDescription {
    pub sample_description_size: u32,
    pub data_format: u32,
    pub reserved: [u8; 6],
    pub data_reference_index: u16,
    pub data: Vec<u8>,
}

#[atom(version)]
#[derive(Debug, PartialEq)]
pub struct StsdAtom {
    pub number_of_entries: u32,
    pub sample_description_table: Vec<SampleDescription>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<StsdAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let number_of_entries = r.read_u32::<BigEndian>()?;

    let mut sample_description_table = Vec::new();

    for _ in 0..number_of_entries {
        let sample_description_size = r.read_u32::<BigEndian>()?;
        let data_format = r.read_u32::<BigEndian>()?;
        let mut reserved = [0_u8; 6];
        r.read_exact(&mut reserved)?;
        let data_reference_index = r.read_u16::<BigEndian>()?;

        let data = Vec::new();

        // @todo
        // r.take((sample_description_size - 16) as u64)
        //    .read_to_end(&mut data)?;
        r.seek(SeekFrom::Current((sample_description_size - 16) as i64))?;

        sample_description_table.push(SampleDescription {
            sample_description_size,
            data_format,
            reserved,
            data_reference_index,
            data,
        });
    }

    Ok(StsdAtom {
        atom_head,
        atom_version,
        atom_flags,
        number_of_entries,
        sample_description_table,
    })
}
