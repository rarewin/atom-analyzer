#![allow(clippy::transmute_ptr_to_ref)] // for mopa
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use crate::atom::{self, Atom, AtomHead, AtomParseError};

pub const ATOM_ID: u32 = 0x6d69_6e66; // 'minf'

#[derive(Debug, PartialEq)]
pub struct MinfAtom {
    pub atom_head: AtomHead,
    pub media_info: MediaInfo,
}

impl Atom for MinfAtom {}

#[derive(Debug, PartialEq)]
pub enum MediaInfo {
    VideoMediaInfo {
        vmhd_atom: Box<atom::vmhd::VmhdAtom>,
        hdlr_atom: Box<atom::hdlr::HdlrAtom>,
        dinf_atom: Option<Box<atom::dinf::DinfAtom>>,
    },
    SoundMediaInfo {
        smhd_atom: Box<atom::smhd::SmhdAtom>,
    },
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<MinfAtom, AtomParseError> {
    let media_info = if let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::vmhd::VmhdAtom>() {
            let vmhd_atom = atom.downcast::<atom::vmhd::VmhdAtom>().unwrap(); // @todo
            let hdlr_atom = if let Ok(atom) = atom::parse(r) {
                if atom.is::<atom::hdlr::HdlrAtom>() {
                    atom.downcast::<atom::hdlr::HdlrAtom>().unwrap() // @todo
                } else {
                    return Err(AtomParseError::RequiredAtomNotFound(atom::hdlr::ATOM_ID));
                }
            } else {
                return Err(AtomParseError::RequiredAtomNotFound(atom::hdlr::ATOM_ID));
            };

            let mut dinf_atom: Option<Box<atom::dinf::DinfAtom>> = None;

            while let Ok(atom) = atom::parse(r) {
                if atom.is::<atom::dinf::DinfAtom>() {
                    dinf_atom = Some(atom.downcast::<atom::dinf::DinfAtom>().unwrap());
                // @todo
                } else {
                    dbg!(&atom);
                }
            }

            MediaInfo::VideoMediaInfo {
                vmhd_atom,
                hdlr_atom,
                dinf_atom,
            }
        } else if atom.is::<atom::smhd::SmhdAtom>() {
            let smhd_atom = atom.downcast::<atom::smhd::SmhdAtom>().unwrap(); // @todo

            MediaInfo::SoundMediaInfo { smhd_atom }
        } else {
            todo!("{:?}", atom)
        }
    } else {
        todo!()
    };

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(MinfAtom {
        atom_head,
        media_info,
    })
}
