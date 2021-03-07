use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};
use std::rc::Rc;

use crate::atom::{self, Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x6564_7473; // 'edts'

#[atom]
#[derive(Debug, PartialEq)]
pub struct EdtsAtom {
    pub elst_atom: Option<Rc<RefCell<Box<atom::elst::ElstAtom>>>>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<EdtsAtom, AtomParseError> {
    let elst_atom_head = atom::parse_atom_head(r)?;
    let elst_atom = match atom::elst::parse(r, elst_atom_head) {
        Ok(e) => Some(Rc::new(RefCell::new(Box::new(e)))),
        Err(AtomParseError::RequiredAtomNotFound(_)) => None,
        _ => unimplemented!(),
    };

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(EdtsAtom {
        atom_head,
        elst_atom,
    })
}
