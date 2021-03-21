use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x6d6f_6f76; // 'moov'

#[atom]
#[derive(Debug, PartialEq)]
pub struct MoovAtom {
    pub mvhd_atom: Option<Box<atom::mvhd::MvhdAtom>>,
    pub trak_atom: Vec<atom::trak::TrakAtom>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<MoovAtom, AtomParseError> {
    let mut mvhd_atom = None;
    let mut trak_atom = Vec::new();

    let atom_tail = atom_head.atom_offset + atom_head.atom_size;

    if atom_head.atom_type != ATOM_ID {
        return Err(AtomParseError::TypeError(atom_head.atom_offset));
    }

    while let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::mvhd::MvhdAtom>() {
            mvhd_atom = Some(atom.downcast::<atom::mvhd::MvhdAtom>().unwrap()); // @todo
        } else if atom.is::<atom::trak::TrakAtom>() {
            trak_atom.push(*atom.downcast::<atom::trak::TrakAtom>().unwrap());
        } else {
            eprintln!("{:?}", atom);
        }

        if r.seek(SeekFrom::Current(0))? >= atom_tail {
            break;
        }
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(MoovAtom {
        atom_head,
        mvhd_atom,
        trak_atom,
    })
}
