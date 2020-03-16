use crate::atom::{Atom, AtomType};

use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek};

#[derive(Debug, PartialEq)]
enum MajorBrand {
    QuickTimeMovieFile,
    Other,
}

/// Returns a Major_Brand enum value
///
/// # Arguments
///
/// * `val` - A 32-bit unsigned integer
fn match_major_brand(val: u32) -> MajorBrand {
    match val {
        0x71742020 => MajorBrand::QuickTimeMovieFile,
        _ => MajorBrand::Other,
    }
}

#[derive(Debug, PartialEq)]
pub struct FtypAtom {
    atom_offset: u64,
    atom_size: u64,
    major_brand: MajorBrand,
}

impl Atom for FtypAtom {
    fn get_offset(&self) -> u64 {
        self.atom_offset
    }

    fn get_size(&self) -> u64 {
        self.atom_size
    }

    fn get_type(&self) -> AtomType {
        AtomType::Ftyp
    }
}

#[allow(unused_variables, dead_code)]
pub fn parse<R: Read + Seek>(r: &mut R, atom_offset: u64) -> Option<FtypAtom> {
    let mut atom_size = if let Ok(v) = r.read_u32::<BigEndian>() {
        v as u64
    } else {
        return None;
    };

    let atom_type = if let Ok(v) = r.read_u32::<BigEndian>() {
        // atom_type should be "ftyp"
        if v != 0x66747970 {
            return None;
        } else {
            v
        }
    } else {
        return None;
    };

    // extended size
    if atom_size == 1 {
        atom_size = r.read_u64::<BigEndian>().unwrap();
    }

    let major_brand = match_major_brand(r.read_u32::<BigEndian>().unwrap());

    Some(FtypAtom {
        atom_offset,
        atom_size,
        major_brand,
    })
}

#[cfg(test)]
mod test_ftyp {
    use crate::atom::{ftyp, Atom, AtomType};

    use std::io::Cursor;

    #[test]
    fn test_simple_ftyp() {
        let test: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x04, 0x66, 0x74, 0x79, 0x70, 0x71, 0x74, 0x20, 0x20,
        ];
        let mut buf = Cursor::new(test);

        let atom = ftyp::parse(&mut buf, 0);

        assert_eq!(
            atom,
            Some(ftyp::FtypAtom {
                atom_offset: 0,
                atom_size: 4,
                major_brand: ftyp::MajorBrand::QuickTimeMovieFile
            })
        );

        let a = atom.unwrap();

        assert_eq!(a.get_offset(), 0);
        assert_eq!(a.get_size(), 4);
        assert_eq!(a.get_type(), AtomType::Ftyp);
    }

    #[test]
    fn test_extended_size_ftyp() {
        let test: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x01, 0x66, 0x74, 0x79, 0x70, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00, 0x71, 0x74, 0x20, 0x20,
        ];
        let mut buf = Cursor::new(test);

        assert_eq!(
            ftyp::parse(&mut buf, 0),
            Some(ftyp::FtypAtom {
                atom_offset: 0,
                atom_size: 0x01_00000000,
                major_brand: ftyp::MajorBrand::QuickTimeMovieFile
            })
        );
    }

    #[test]
    fn test_invalid_ftyp() {
        let test: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x01, 0x67, 0x74, 0x79, 0x70, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00,
        ];

        let mut buf = Cursor::new(test);

        assert_eq!(ftyp::parse(&mut buf, 0), None);
    }
}
