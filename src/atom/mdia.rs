use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x6d_64_69_61; // 'mdia'

#[derive(Debug, PartialEq)]
pub struct MdiaAtom {
    pub atom_head: atom::AtomHead,
    pub mdhd_atom: Box<atom::mdhd::MdhdAtom>,
}

impl Atom for MdiaAtom {}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<MdiaAtom, AtomParseError> {
    let mut mdhd_atom: Option<Box<atom::mdhd::MdhdAtom>> = None;

    let atom_tail = atom_head.atom_offset + atom_head.atom_size;

    while let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::mdhd::MdhdAtom>() {
            mdhd_atom = Some(atom.downcast::<atom::mdhd::MdhdAtom>().unwrap());
        } else {
            eprintln!("{:?}", atom);
        }

        if r.seek(SeekFrom::Current(0))? >= atom_tail {
            break;
        }
    }

    let mdhd_atom = match mdhd_atom {
        Some(a) => a,
        None => {
            return Err(atom::AtomParseError::RequiredAtomNotFound(
                atom::mdhd::ATOM_ID,
            ))
        }
    };

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(MdiaAtom {
        atom_head,
        mdhd_atom,
    })
}
