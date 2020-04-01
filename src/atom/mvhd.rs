use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek};

use std::error;

use crate::atom;
use crate::element;

pub const ATOM_ID: u32 = 0x6d766864; // 'mvhd'

#[derive(Debug, PartialEq)]
pub struct MvhdAtom {
    pub atom_head: atom::AtomHead,
    pub atom_version: u8,
    pub atom_flags: [u8; 3],
    pub creation_time: element::qtfile_datetime::QtFileDateTime,
    pub modification_time: element::qtfile_datetime::QtFileDateTime,
    pub time_scale: u32,
    pub duration: u32,
    pub preferred_rate: u32,
    pub preferred_volume: u16,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MvhdAtom, Box<dyn error::Error>> {
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
    let time_scale = r.read_u32::<BigEndian>()?;
    let duration = r.read_u32::<BigEndian>()?;
    let preferred_rate = r.read_u32::<BigEndian>()?;
    let preferred_volume = r.read_u16::<BigEndian>()?;

    Ok(MvhdAtom {
        atom_head,
        atom_version,
        atom_flags,
        creation_time,
        modification_time,
        time_scale,
        duration,
        preferred_rate,
        preferred_volume,
    })
}
