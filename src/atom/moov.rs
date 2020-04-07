use std::error;
use std::io::{Read, Seek, SeekFrom};

use crate::atom;

pub const ATOM_ID: u32 = 0x6d6f6f76; // 'moov'

#[derive(Debug, PartialEq)]
pub struct MoovAtom {
    pub atom_head: atom::AtomHead,
    pub mvhd_atom: Option<atom::mvhd::MvhdAtom>,
    pub trak_atoms: Vec<atom::trak::TrakAtom>,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<MoovAtom, Box<dyn error::Error>> {
    let atom_head = atom::parse_atom_head(r)?;

    if atom_head.atom_type != ATOM_ID {
        return Err(Box::new(atom::AtomSeekError::TypeError));
    }

    let mut mvhd_atom = None;
    let mut trak_atoms = Vec::<atom::trak::TrakAtom>::new();

    let mut seek = r.seek(SeekFrom::Current(0))?;

    while seek < atom_head.atom_offset + atom_head.atom_size {
        match atom::parse(r)? {
            atom::Atom::Mvhd(m) => {
                if mvhd_atom != None {
                    panic!("duplicated mvhd-atom");
                } else {
                    mvhd_atom = Some(*m)
                }
            }
            atom::Atom::Trak(t) => trak_atoms.push(*t),
            _ => {}
        };
        seek = r.seek(SeekFrom::Current(0))?; // @todo サイズを取得して足せばよい……
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;

    Ok(MoovAtom {
        atom_head,
        mvhd_atom,
        trak_atoms,
    })
}
