use std::io::{Read, Seek};

use anyhow::{Error, Result};
use byteorder::{BigEndian, ReadBytesExt};

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
    pub matrix_structure: element::qtfile_matrix::QtFileMatrix,
    pub preview_time: element::qtfile_datetime::QtFileDateTime,
    pub preview_duration: u32,
    pub poster_time: element::qtfile_datetime::QtFileDateTime,
    pub selection_time: element::qtfile_datetime::QtFileDateTime,
    pub selection_duration: u32,
    pub current_time: element::qtfile_datetime::QtFileDateTime,
    pub next_track_id: u32,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MvhdAtom> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(Error::new(atom::AtomSeekError::TypeError(
            atom_head.atom_offset,
        )));
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

    let mut reserved = [0 as u8; 10];
    r.read(&mut reserved)?;

    let mut matrix = [0 as u32; 9];
    for i in 0..matrix.len() {
        matrix[i] = r.read_u32::<BigEndian>()?;
    }

    let matrix_structure = element::qtfile_matrix::QtFileMatrix::new(&matrix);

    let preview_time = element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);
    let preview_duration = r.read_u32::<BigEndian>()?;

    let poster_time = element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);

    let selection_time = element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);
    let selection_duration = r.read_u32::<BigEndian>()?;

    let current_time = element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);

    let next_track_id = r.read_u32::<BigEndian>()?;

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
