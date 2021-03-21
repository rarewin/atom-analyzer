use std::fmt::Debug;
use std::io::{Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};
use fixed::{
    types::extra::{U16, U8},
    FixedU16, FixedU32,
};

use crate::atom::{Atom, AtomHead, AtomParseError};
use crate::element;
use atom_derive::atom;

#[atom(version)]
#[derive(Debug, PartialEq)]
pub struct TkhdAtom {
    pub creation_time: element::qtfile_datetime::QtFileDateTime,
    pub modification_time: element::qtfile_datetime::QtFileDateTime,
    pub track_id: u32,
    pub reserved0: u32,
    pub duration: u32,
    pub reserved1: [u8; 8],
    pub layer: u16,
    pub alternate_group: u16,
    pub volume: FixedU16<U8>,
    pub reserved2: u16,
    pub matrix_structure: element::qtfile_matrix::QtFileMatrix,
    pub track_width: FixedU32<U16>,
    pub track_height: FixedU32<U16>,
}

pub const ATOM_ID: u32 = 0x746b_6864; // 'tkhd'

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<TkhdAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let creation_time = element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);
    let modification_time =
        element::qtfile_datetime::QtFileDateTime::new(r.read_u32::<BigEndian>()?);

    let track_id = r.read_u32::<BigEndian>()?;
    let reserved0 = r.read_u32::<BigEndian>()?;
    let duration = r.read_u32::<BigEndian>()?;

    let mut reserved1 = [0_u8; 8];
    r.read_exact(&mut reserved1)?;

    let layer = r.read_u16::<BigEndian>()?;
    let alternate_group = r.read_u16::<BigEndian>()?;
    let volume = FixedU16::<U8>::from_bits(r.read_u16::<BigEndian>()?);
    let reserved2 = r.read_u16::<BigEndian>()?;

    let matrix_structure = element::qtfile_matrix::QtFileMatrix::parse(r)?;

    let track_width = FixedU32::<U16>::from_bits(r.read_u32::<BigEndian>()?);
    let track_height = FixedU32::<U16>::from_bits(r.read_u32::<BigEndian>()?);

    Ok(TkhdAtom {
        atom_head,
        atom_version,
        atom_flags,
        creation_time,
        modification_time,
        track_id,
        reserved0,
        duration,
        reserved1,
        layer,
        alternate_group,
        volume,
        reserved2,
        matrix_structure,
        track_width,
        track_height,
    })
}
