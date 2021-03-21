use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x7472_616b; // 'trak'

#[atom]
#[derive(Debug, PartialEq)]
pub struct TrakAtom {
    pub tkhd_atom: Box<atom::tkhd::TkhdAtom>,
    pub edts_atom: Option<Box<atom::edts::EdtsAtom>>,
    pub mdia_atom: Box<atom::mdia::MdiaAtom>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<TrakAtom, AtomParseError> {
    let mut tkhd_atom: Option<Box<atom::tkhd::TkhdAtom>> = None;
    let mut edts_atom: Option<Box<atom::edts::EdtsAtom>> = None;
    let mut mdia_atom: Option<Box<atom::mdia::MdiaAtom>> = None;

    let atom_tail = atom_head.atom_offset + atom_head.atom_size;

    while let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::tkhd::TkhdAtom>() {
            tkhd_atom = Some(atom.downcast::<atom::tkhd::TkhdAtom>().unwrap()); // @todo
        } else if atom.is::<atom::edts::EdtsAtom>() {
            edts_atom = Some(atom.downcast::<atom::edts::EdtsAtom>().unwrap()); // @todo
        } else if atom.is::<atom::mdia::MdiaAtom>() {
            mdia_atom = Some(atom.downcast::<atom::mdia::MdiaAtom>().unwrap()); // @todo
        } else {
            eprintln!("{:?}", atom);
        }

        if r.seek(SeekFrom::Current(0))? >= atom_tail {
            break;
        }
    }

    let tkhd_atom = match tkhd_atom {
        Some(a) => a,
        None => {
            return Err(atom::AtomParseError::RequiredAtomNotFound(
                atom::tkhd::ATOM_ID,
            ))
        }
    };

    let mdia_atom = match mdia_atom {
        Some(a) => a,
        None => {
            return Err(atom::AtomParseError::RequiredAtomNotFound(
                atom::mdia::ATOM_ID,
            ))
        }
    };

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(TrakAtom {
        atom_head,
        tkhd_atom,
        edts_atom,
        mdia_atom,
    })
}
