pub mod ftyp;

use ftyp::FtypAtom;

#[derive(Debug, PartialEq)]
enum Atom {
    Ftyp(FtypAtom),
}
