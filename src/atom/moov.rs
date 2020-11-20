use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, AtomParseError};

pub const ATOM_ID: u32 = 0x6d6f_6f76; // 'moov'

#[derive(Debug, PartialEq)]
pub struct MoovAtom {
    pub atom_head: atom::AtomHead,
    pub mvhd_atom: Option<atom::mvhd::MvhdAtom>,
    pub trak_atom: Vec<atom::trak::TrakAtom>,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MoovAtom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;
    let mut mvhd_atom = None;
    let mut trak_atom = Vec::new();

    if atom_head.atom_type != ATOM_ID {
        return Err(AtomParseError::TypeError(atom_head.atom_offset));
    }

    while let Ok(atom) = atom::parse(r) {
        match atom {
            atom::Atom::Mvhd(m) => mvhd_atom = Some(*m),
            atom::Atom::Trak(t) => trak_atom.push(*t),
            _ => eprintln!("{:?}", atom),
        }
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(MoovAtom {
        atom_head,
        mvhd_atom,
        trak_atom,
    })
}
