use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomParseError};

pub const ATOM_ID: u32 = 0x6d6f_6f76; // 'moov'

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Atom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;
    let mut mvhd_atom = None;
    let mut trak_atom = Vec::new();

    if atom_head.atom_type != ATOM_ID {
        return Err(AtomParseError::TypeError(atom_head.atom_offset));
    }

    while let Ok(atom) = atom::parse(r) {
        match atom {
            Atom::Mvhd { .. } => mvhd_atom = Some(Box::new(atom)),
            Atom::Trak { .. } => trak_atom.push(atom),
            _ => eprintln!("{:?}", atom),
        }
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(Atom::Moov {
        atom_head,
        mvhd_atom,
        trak_atom,
    })
}
