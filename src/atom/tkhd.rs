use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

use std::error;

use crate::atom;
use crate::element;

pub const ATOM_ID: u32 = 0x746b6864; // 'tkhd'

#[derive(Debug, PartialEq)]
pub struct TkhdAtom {
    pub atom_head: atom::AtomHead,
    pub atom_version: u8,
    pub atom_flags: [u8; 3],
    pub creation_time: element::qtfile_datetime::QtFileDateTime,
    pub modification_time: element::qtfile_datetime::QtFileDateTime,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<TkhdAtom, Box<dyn error::Error>> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(Box::new(atom::AtomSeekError::TypeError));
    }

    let atom_version = r.read_u8()?;
    let mut atom_flags = [0 as u8; 3];

    r.read(&mut atom_flags)?;

    let creation_time = element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);
    let modification_time =
        element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(TkhdAtom {
        atom_head,
        atom_version,
        atom_flags,
        creation_time,
        modification_time,
    })
}
