use std::io::{Read, Seek, SeekFrom};

use std::error;

use crate::atom;

pub const ATOM_ID: u32 = 0x7472616b; // 'trak'

#[derive(Debug, PartialEq)]
pub struct TrakAtom {
    pub atom_head: atom::AtomHead,
    pub tkhd_atom: atom::tkhd::TkhdAtom,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<TrakAtom, Box<dyn error::Error>> {
    let head = atom::parse_atom_head(r)?;

    let atom_offset = head.atom_offset;
    let atom_size = head.atom_size;
    let atom_type = head.atom_type;

    let atom_head = atom::AtomHead {
        atom_offset,
        atom_size,
        atom_type,
    };

    if atom_type != ATOM_ID {
        return Err(Box::new(atom::AtomSeekError::TypeError));
    }

    let tkhd_atom = match atom::parse(r)? {
        atom::Atom::Tkhd(t) => *t,
        _ => unimplemented!("???"),
    };

    r.seek(SeekFrom::Start(atom_offset + atom_size))?;

    Ok(TrakAtom {
        atom_head,
        tkhd_atom,
    })
}
