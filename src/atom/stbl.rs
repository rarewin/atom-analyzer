use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x7374_626c; // 'stbl'

#[derive(Debug, PartialEq)]
pub struct StblAtom {
    pub atom_head: AtomHead,
    pub stsd_atom: Box<atom::stsd::StsdAtom>,
    pub stts_atom: Box<atom::stts::SttsAtom>,
    pub stss_atom: Box<atom::stss::StssAtom>,
    pub ctts_atom: Box<atom::ctts::CttsAtom>,
    pub stsc_atom: Box<atom::stsc::StscAtom>,
    pub stsz_atom: Box<atom::stsz::StszAtom>,
}

impl Atom for StblAtom {}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<StblAtom, AtomParseError> {
    let stsd_atom = if let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::stsd::StsdAtom>() {
            atom.downcast::<atom::stsd::StsdAtom>().unwrap() // @todo
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    let stts_atom = if let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::stts::SttsAtom>() {
            atom.downcast::<atom::stts::SttsAtom>().unwrap() // @todo
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    let stss_atom = if let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::stss::StssAtom>() {
            atom.downcast::<atom::stss::StssAtom>().unwrap() // @todo
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    let ctts_atom = if let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::ctts::CttsAtom>() {
            atom.downcast::<atom::ctts::CttsAtom>().unwrap() // @todo
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    let stsc_atom = if let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::stsc::StscAtom>() {
            atom.downcast::<atom::stsc::StscAtom>().unwrap() // @todo
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    let stsz_atom = if let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::stsz::StszAtom>() {
            atom.downcast::<atom::stsz::StszAtom>().unwrap() // @todo
        } else {
            unimplemented!();
        }
    } else {
        unimplemented!();
    };

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(StblAtom {
        atom_head,
        stsd_atom,
        stts_atom,
        stss_atom,
        ctts_atom,
        stsc_atom,
        stsz_atom,
    })
}
