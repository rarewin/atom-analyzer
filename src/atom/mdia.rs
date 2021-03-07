use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};
use std::rc::Rc;

use crate::atom::{self, Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x6d_64_69_61; // 'mdia'

#[atom]
#[derive(Debug, PartialEq)]
pub struct MdiaAtom {
    pub mdhd_atom: Rc<RefCell<Box<atom::mdhd::MdhdAtom>>>,
    pub hdlr_atom: Option<Rc<RefCell<Box<atom::hdlr::HdlrAtom>>>>,
    pub minf_atom: Option<Rc<RefCell<Box<atom::minf::MinfAtom>>>>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<MdiaAtom, AtomParseError> {
    let mut mdhd_atom = None;
    let mut hdlr_atom = None;
    let mut minf_atom = None;

    let atom_tail = atom_head.atom_offset + atom_head.atom_size;

    while let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::mdhd::MdhdAtom>() {
            mdhd_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::mdhd::MdhdAtom>().unwrap(),
            ))); // @todo
        } else if atom.is::<atom::hdlr::HdlrAtom>() {
            hdlr_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::hdlr::HdlrAtom>().unwrap(),
            ))); // @todo
        } else if atom.is::<atom::minf::MinfAtom>() {
            minf_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::minf::MinfAtom>().unwrap(),
            ))); // @todo
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
        hdlr_atom,
        minf_atom,
    })
}
