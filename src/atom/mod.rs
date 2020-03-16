pub mod ftyp;

trait Atom {
    fn get_offset(&self) -> u64;
    fn get_size(&self) -> u64;
    fn get_type(&self) -> AtomType;
}

#[derive(Debug, PartialEq)]
enum AtomType {
    Ftyp,
}
