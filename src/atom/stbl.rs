use std::cell::RefCell;
use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};
use std::rc::Rc;

use crate::atom::{self, Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x7374_626c; // 'stbl'

#[atom]
#[derive(Debug, PartialEq)]
pub struct StblAtom {
    pub stsd_atom: Option<Rc<RefCell<Box<atom::stsd::StsdAtom>>>>,
    pub stts_atom: Option<Rc<RefCell<Box<atom::stts::SttsAtom>>>>,
    pub stss_atom: Option<Rc<RefCell<Box<atom::stss::StssAtom>>>>,
    pub ctts_atom: Option<Rc<RefCell<Box<atom::ctts::CttsAtom>>>>,
    pub stsc_atom: Option<Rc<RefCell<Box<atom::stsc::StscAtom>>>>,
    pub stsz_atom: Option<Rc<RefCell<Box<atom::stsz::StszAtom>>>>,
    pub stco_atom: Option<Rc<RefCell<Box<atom::stco::StcoAtom>>>>,
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<StblAtom, AtomParseError> {
    let mut stsd_atom: Option<Rc<RefCell<Box<atom::stsd::StsdAtom>>>> = None;
    let mut stts_atom: Option<Rc<RefCell<Box<atom::stts::SttsAtom>>>> = None;
    let mut stss_atom: Option<Rc<RefCell<Box<atom::stss::StssAtom>>>> = None;
    let mut ctts_atom: Option<Rc<RefCell<Box<atom::ctts::CttsAtom>>>> = None;
    let mut stsc_atom: Option<Rc<RefCell<Box<atom::stsc::StscAtom>>>> = None;
    let mut stsz_atom: Option<Rc<RefCell<Box<atom::stsz::StszAtom>>>> = None;
    let mut stco_atom: Option<Rc<RefCell<Box<atom::stco::StcoAtom>>>> = None;

    let atom_tail = atom_head.atom_offset + atom_head.atom_size;

    while let Ok(atom) = atom::parse(r) {
        if atom.is::<atom::stsd::StsdAtom>() {
            stsd_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::stsd::StsdAtom>().unwrap(),
            ))); // @todo
        } else if atom.is::<atom::stts::SttsAtom>() {
            stts_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::stts::SttsAtom>().unwrap(),
            ))); // @todo
        } else if atom.is::<atom::stss::StssAtom>() {
            stss_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::stss::StssAtom>().unwrap(),
            ))); // @todo
        } else if atom.is::<atom::ctts::CttsAtom>() {
            ctts_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::ctts::CttsAtom>().unwrap(),
            ))); // @todo
        } else if atom.is::<atom::stsc::StscAtom>() {
            stsc_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::stsc::StscAtom>().unwrap(),
            ))); // @todo
        } else if atom.is::<atom::stsz::StszAtom>() {
            stsz_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::stsz::StszAtom>().unwrap(),
            ))); // @todo
        } else if atom.is::<atom::stco::StcoAtom>() {
            stco_atom = Some(Rc::new(RefCell::new(
                atom.downcast::<atom::stco::StcoAtom>().unwrap(),
            ))); // @todo
        } else {
            eprintln!("{:?}", atom);
        }

        if r.seek(SeekFrom::Current(0))? >= atom_tail {
            break;
        }
    }

    r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
    Ok(StblAtom {
        atom_head,
        stsd_atom,
        stts_atom,
        stss_atom,
        ctts_atom,
        stsc_atom,
        stsz_atom,
        stco_atom,
    })
}
