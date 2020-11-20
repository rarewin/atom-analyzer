use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, AtomParseError};

pub const ATOM_ID: u32 = 0x7472_616b; // 'trak'

#[derive(Debug, PartialEq)]
pub struct TrakAtom {
    pub atom_head: atom::AtomHead,
    pub tkhd_atom: atom::tkhd::TkhdAtom,
    pub edts_atom: Option<atom::edts::EdtsAtom>,
    pub mdia_atom: atom::mdia::MdiaAtom,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<TrakAtom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;
    let mut tkhd_atom: Option<atom::tkhd::TkhdAtom> = None;
    let mut edts_atom: Option<atom::edts::EdtsAtom> = None;
    let mut mdia_atom: Option<atom::mdia::MdiaAtom> = None;

    if atom_head.atom_type != ATOM_ID {
        return Err(atom::AtomParseError::TypeError(atom_head.atom_offset));
    }

    let atom_tail = atom_head.atom_offset + atom_head.atom_size;

    while let Ok(atom) = atom::parse(r) {
        match atom {
            atom::Atom::Tkhd(m) => tkhd_atom = Some(*m),
            atom::Atom::Edts(e) => edts_atom = Some(*e),
            atom::Atom::Mdia(m) => mdia_atom = Some(*m),
            _ => eprintln!("{:?}", atom),
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
