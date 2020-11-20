use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, AtomParseError};

pub const ATOM_ID: u32 = 0x6564_7473; // 'edts'

#[derive(Debug, PartialEq)]
pub struct EdtsAtom {
    pub atom_head: atom::AtomHead,
    pub elst_atom: Option<atom::elst::ElstAtom>,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<EdtsAtom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;

    let atom_type = atom_head.atom_type;
    let atom_offset = atom_head.atom_offset;
    let atom_size = atom_head.atom_size;

    if atom_type != ATOM_ID {
        return Err(atom::AtomParseError::TypeError(atom_offset + 4));
    }

    let elst_atom = match atom::elst::parse(r) {
        Ok(e) => Some(e),
        Err(AtomParseError::RequiredAtomNotFound(_)) => None,
        _ => unimplemented!(),
    };

    r.seek(SeekFrom::Start(atom_offset + atom_size))?;

    Ok(EdtsAtom {
        atom_head,
        elst_atom,
    })
}
