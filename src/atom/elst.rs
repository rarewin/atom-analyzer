use std::fmt::Debug;
use std::io::{Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};
use fixed::{types::extra::U16, FixedU32};

use crate::atom::{Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x65_6c_73_74; // 'elst'

#[atom(version)]
#[derive(Debug, PartialEq)]
pub struct ElstAtom {
    pub number_of_entries: u32,
    pub edit_list_table: Vec<EditListTableEntry>,
}

#[derive(Debug, PartialEq)]
pub struct EditListTableEntry {
    pub track_duration: u32,
    pub media_time: u32,
    pub media_rate: FixedU32<U16>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<ElstAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let number_of_entries = r.read_u32::<BigEndian>()?;

    let mut edit_list_table = Vec::new();

    for _ in 0..number_of_entries {
        let track_duration = r.read_u32::<BigEndian>()?;
        let media_time = r.read_u32::<BigEndian>()?;
        let media_rate = FixedU32::<U16>::from_bits(r.read_u32::<BigEndian>()?);

        edit_list_table.push(EditListTableEntry {
            track_duration,
            media_time,
            media_rate,
        });
    }

    Ok(ElstAtom {
        atom_head,
        atom_version,
        atom_flags,
        number_of_entries,
        edit_list_table,
    })
}
