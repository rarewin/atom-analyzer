use std::io::{Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{self, Atom, AtomParseError};
use crate::element;

pub const ATOM_ID: u32 = 0x6d76_6864; // 'mvhd'

#[derive(Debug, PartialEq)]
pub struct MvhdAtom {}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Atom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(AtomParseError::TypeError(atom_head.atom_offset));
    }

    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];

    r.read_exact(&mut atom_flags)?;

    let creation_time = element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);
    let modification_time =
        element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);
    let time_scale = r.read_u32::<BigEndian>()?;
    let duration = r.read_u32::<BigEndian>()?;
    let preferred_rate = r.read_u32::<BigEndian>()?;
    let preferred_volume = r.read_u16::<BigEndian>()?;

    let mut reserved = [0_u8; 10];
    r.read_exact(&mut reserved)?;

    let matrix_structure = element::qtfile_matrix::QtFileMatrix::parse(r)?;
    let preview_time = element::qtfile_datetime::QtFileDateTime::parse(r)?;
    let preview_duration = r.read_u32::<BigEndian>()?;

    let poster_time = element::qtfile_datetime::QtFileDateTime::parse(r)?;

    let selection_time = element::qtfile_datetime::QtFileDateTime::parse(r)?;
    let selection_duration = r.read_u32::<BigEndian>()?;

    let current_time = element::qtfile_datetime::QtFileDateTime::parse(r)?;

    let next_track_id = r.read_u32::<BigEndian>()?;

    Ok(Atom::Mvhd {
        atom_head,
        atom_version,
        atom_flags,
        creation_time,
        modification_time,
        time_scale,
        duration,
        preferred_rate,
        preferred_volume,
        matrix_structure,
        preview_time,
        preview_duration,
        poster_time,
        selection_time,
        selection_duration,
        current_time,
        next_track_id,
    })
}
