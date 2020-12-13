use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x6469_6e66; // 'dinf'

#[derive(Debug, PartialEq)]
pub struct DinfAtom {
    pub atom_head: AtomHead,
    pub dref_atom: Box<atom::dref::DrefAtom>,
}

impl Atom for DinfAtom {}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<DinfAtom, AtomParseError> {
    let dref_atom = if let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::dref::DrefAtom>() {
            atom.downcast::<atom::dref::DrefAtom>().unwrap()
        } else {
            panic!()
        }
    } else {
        panic!()
    };

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(DinfAtom {
        atom_head,
        dref_atom,
    })
}
