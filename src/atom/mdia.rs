use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, AtomParseError};

pub const ATOM_ID: u32 = 0x6d_64_69_61; // 'mdia'

#[derive(Debug, PartialEq)]
pub struct MdiaAtom {
    pub atom_head: atom::AtomHead,
    pub mdhd_atom: atom::mdhd::MdhdAtom,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MdiaAtom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(atom::AtomParseError::TypeError(atom_head.atom_offset));
    }

    let mut mdhd_atom: Option<atom::mdhd::MdhdAtom> = None;

    let atom_tail = atom_head.atom_offset + atom_head.atom_size;

    while let Ok(atom) = atom::parse(r) {
        match atom {
            atom::Atom::Mdhd(m) => mdhd_atom = Some(*m),
            _ => eprintln!("{:?}", atom),
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
