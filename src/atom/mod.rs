pub mod ftyp;
pub mod mdat;
pub mod wide;

use ftyp::FtypAtom;
use mdat::MdatAtom;
use wide::WideAtom;

#[derive(Debug, PartialEq)]
enum Atom {
    Ftyp(Box<FtypAtom>),
    Mdat(Box<MdatAtom>),
    Wide(Box<WideAtom>),
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
