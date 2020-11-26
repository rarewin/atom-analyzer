use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x6564_7473; // 'edts'

#[derive(Debug, PartialEq)]
pub struct EdtsAtom {
    pub atom_head: AtomHead,
    pub elst_atom: Option<Box<atom::elst::ElstAtom>>,
}

impl Atom for EdtsAtom {}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<EdtsAtom, AtomParseError> {
    let elst_atom_head = atom::parse_atom_head(r)?;
    let elst_atom = match atom::elst::parse(r, elst_atom_head) {
        Ok(e) => Some(Box::new(e)),
        Err(AtomParseError::RequiredAtomNotFound(_)) => None,
        _ => unimplemented!(),
    };

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(EdtsAtom {
        atom_head,
        elst_atom,
    })
}
