use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{self, Atom, AtomHead, AtomParseError};

#[derive(Debug, PartialEq)]
pub enum Brand {
    QuickTimeMovieFile,
    Other(u32),
}

pub const ATOM_ID: u32 = 0x6674_7970; // 'ftyp'

/// Returns a brand enum value
///
/// # Arguments
///
/// * `val` - A 32-bit unsigned integer
fn match_brand(val: u32) -> Brand {
    match val {
        0x7174_2020 => Brand::QuickTimeMovieFile,
        _ => Brand::Other(val),
    }
}

#[derive(Debug, PartialEq, Atom)]
pub struct FtypAtom {
    pub atom_head: atom::AtomHead,
    pub major_brand: Brand,
    pub minor_version: u32,
    pub compatible_brands: Vec<Brand>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<FtypAtom, AtomParseError> {
    let atom_offset = atom_head.atom_offset;
    let atom_size = atom_head.atom_size;

    let major_brand = match_brand(r.read_u32::<BigEndian>()?);
    let minor_version = r.read_u32::<BigEndian>()?;

    let compatible_brands = if let Ok(offset) = r.seek(SeekFrom::Current(0)) {
        let mut b = Vec::<Brand>::new();
        for i in 0..((atom_size - (offset - atom_offset)) / 4) {
            b.push(if let Ok(value) = r.read_u32::<BigEndian>() {
                match_brand(value)
            } else {
                return Err(AtomParseError::UnexpectedError(atom_offset + 8 + i * 4));
            })
        }
        b
    } else {
        return Err(AtomParseError::UnexpectedError(atom_offset + 8));
    };

    Ok(FtypAtom {
        atom_head,
        major_brand,
        minor_version,
        compatible_brands,
    })
}
