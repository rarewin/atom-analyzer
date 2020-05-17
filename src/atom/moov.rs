use std::io::{Read, Seek, SeekFrom};

use std::error;

use crate::atom;

pub const ATOM_ID: u32 = 0x6d6f6f76; // 'moov'

#[derive(Debug, PartialEq)]
pub struct MoovAtom {
    pub atom_head: atom::AtomHead,
    pub mvhd_atom: Option<atom::mvhd::MvhdAtom>,
    pub trak_atoms: Vec<atom::trak::TrakAtom>,
    pub udta_atom: Option<atom::udta::UdtaAtom>,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MoovAtom, Box<dyn error::Error>> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(Box::new(atom::AtomSeekError::TypeError));
    }

    // prfl-atom may be exist

    let mvhd_atom = match atom::parse(r)? {
        atom::Atom::Mvhd(m) => Some(*m),
        _ => unimplemented!("..."),
    };

    let mut trak_atoms = Vec::<atom::trak::TrakAtom>::new();
    let mut udta_atom = None;

    let mut curpos = r.seek(SeekFrom::Current(0))?;

    while curpos < atom_head.atom_offset + atom_head.atom_size {
        match atom::parse(r)? {
            atom::Atom::Trak(t) => trak_atoms.push(*t),
            atom::Atom::Udta(u) => udta_atom = Some(*u),
            _ => unimplemented!(""),
        };
        curpos = r.seek(SeekFrom::Current(0))?;
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(MoovAtom {
        atom_head,
        mvhd_atom,
        trak_atoms,
        udta_atom,
    })
}
