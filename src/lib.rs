pub mod ftyp;

use ftyp::Ftyp;

#[derive(Debug)]
pub struct AtomFile<'a> {
    pub ftyp: Ftyp<'a>,
}

impl<'a> AtomFile<'a> {
    pub fn new(input: &'a [u8]) -> Result<AtomFile<'a>, &'static str> {
        let ftyp = ftyp::parse_ftyp(input);
        Ok(AtomFile {
            ftyp: Ftyp {
                size: 0,
                type_str: b"ftyp",
                major_brand: 0,
            },
        })
    }
}
