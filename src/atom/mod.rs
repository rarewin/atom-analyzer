pub mod ftyp;
pub mod mdat;
pub mod moov;
pub mod wide;

use ftyp::FtypAtom;
use mdat::MdatAtom;
use moov::MoovAtom;
use wide::WideAtom;

#[derive(Debug, PartialEq)]
pub enum Atom {
    Ftyp(Box<FtypAtom>),
    Mdat(Box<MdatAtom>),
    Wide(Box<WideAtom>),
    Moov(Box<MoovAtom>),
}

impl Atom {
    pub fn get_offset(&self) {
        match self {
            Atom::Ftyp(f) => f.atom_offset,
            _ => {
                unimplemented!("not implemented yet");
            }
        };
    }
}
