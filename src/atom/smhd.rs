use std::fmt::Debug;
use std::io::{Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};
use fixed::{types::extra::U8, FixedI16};

use crate::atom::{Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x736d_6864; // 'smhd'

#[derive(Debug, PartialEq, Atom)]
pub struct SmhdAtom {
    pub atom_head: AtomHead,
    pub balance: FixedI16<U8>,
    pub reserved: u16,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<SmhdAtom, AtomParseError> {
    let balance = FixedI16::<U8>::from_bits(r.read_i16::<BigEndian>()?);
    let reserved = r.read_u16::<BigEndian>()?;

    Ok(SmhdAtom {
        atom_head,
        balance,
        reserved,
    })
}
