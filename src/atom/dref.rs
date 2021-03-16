use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{self, Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x6472_6566; // 'dref'

#[derive(Debug, PartialEq, Atom)]
pub struct DrefAtom {
    pub atom_head: AtomHead,
    pub atom_version: u8,
    pub atom_flags: [u8; 3],
    pub number_of_entries: u32,
    pub data_references: Vec<DataReferenceType>,
}

#[derive(Debug, PartialEq)]
pub enum DataReferenceType {
    MacintoshAlias {
        atom_head: AtomHead,
        information: String,
    },
    MacintoshAliasResource {
        atom_head: AtomHead,
        resource_type: i32,
        resorce_id: i16,
    },
    Url {
        atom_head: AtomHead,
        url: String,
    },
    Unknown {
        atom_head: AtomHead,
    },
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<DrefAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];

    r.read_exact(&mut atom_flags)?;

    let number_of_entries = r.read_u32::<BigEndian>()?;

    let mut data_references = Vec::new();

    for _ in 0..number_of_entries {
        let atom_head = atom::parse_atom_head(r)?;
        match atom_head.atom_type {
            0x7572_6c20 => {
                let mut url = String::new();
                r.take(atom_head.atom_size - 8).read_to_string(&mut url)?;
                data_references.push(DataReferenceType::Url { atom_head, url })
            }
            _ => {
                r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
                data_references.push(DataReferenceType::Unknown { atom_head });
            }
        }
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(DrefAtom {
        atom_head,
        atom_version,
        atom_flags,
        number_of_entries,
        data_references,
    })
}
