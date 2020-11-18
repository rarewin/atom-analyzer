use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{self, AtomParseError};

#[derive(Debug, PartialEq)]
pub enum Brand {
    QuickTimeMovieFile,
    Other(u32),
}

pub const ATOM_ID: u32 = 0x6674_7970; // 'ftyp'

/// Returns a brand enum value
///
/// # Arguments
///
/// * `val` - A 32-bit unsigned integer
fn match_brand(val: u32) -> Brand {
    match val {
        0x7174_2020 => Brand::QuickTimeMovieFile,
        _ => Brand::Other(val),
    }
}

#[derive(Debug, PartialEq)]
pub struct FtypAtom {
    pub atom_offset: u64,
    pub atom_size: u64,
    pub major_brand: Brand,
    pub minor_version: u32,
    pub compatible_brands: Vec<Brand>,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<FtypAtom, AtomParseError> {
    let atom_head = atom::parse_atom_head(r)?;

    let atom_offset = atom_head.atom_offset;
    let atom_size = atom_head.atom_size;
    let atom_type = atom_head.atom_type;

    if atom_type != ATOM_ID {
        return Err(AtomParseError::TypeError(atom_offset + 4));
    }

    let major_brand = match_brand(r.read_u32::<BigEndian>()?);
    let minor_version = r.read_u32::<BigEndian>()?;

    let compatible_brands = if let Ok(offset) = r.seek(SeekFrom::Current(0)) {
        let mut b = Vec::<Brand>::new();
        for i in 0..((atom_size - (offset - atom_offset)) / 4) {
            b.push(if let Ok(value) = r.read_u32::<BigEndian>() {
                match_brand(value)
            } else {
                return Err(AtomParseError::UnexpectedError(atom_offset + 8 + i * 4));
            })
        }
        b
    } else {
        return Err(AtomParseError::UnexpectedError(atom_offset + 8));
    };

    Ok(FtypAtom {
        atom_offset,
        atom_size,
        major_brand,
        minor_version,
        compatible_brands,
    })
}

#[cfg(test)]
mod test_ftyp {
    use crate::atom::ftyp::{self, Brand};

    use std::io::Cursor;

    #[test]
    fn test_simple_ftyp() {
        let test: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x14, 0x66, 0x74, 0x79, 0x70, 0x71, 0x74, 0x20, 0x20, 0x20, 0x04,
            0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut buf = Cursor::new(test);

        let atom = ftyp::parse(&mut buf);

        assert_eq!(
            atom.unwrap(),
            ftyp::FtypAtom {
                atom_offset: 0,
                atom_size: 20,
                major_brand: ftyp::Brand::QuickTimeMovieFile,
                minor_version: 0x20040600,
                compatible_brands: vec![Brand::Other(0)]
            }
        );
    }

    #[test]
    fn test_extended_size_ftyp() {
        let test: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x01, 0x66, 0x74, 0x79, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x1c, 0x71, 0x74, 0x20, 0x20, 0x20, 0x04, 0x06, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut buf = Cursor::new(test);

        assert_eq!(
            ftyp::parse(&mut buf).unwrap(),
            ftyp::FtypAtom {
                atom_offset: 0,
                atom_size: 0x1c,
                major_brand: ftyp::Brand::QuickTimeMovieFile,
                minor_version: 0x20040600,
                compatible_brands: vec![Brand::Other(0)]
            }
        );
    }

    #[test]
    fn test_invalid_ftyp() {
        let test: Vec<u8> = vec![
            0x00, 0x00, 0x00, 0x10, 0x67, 0x74, 0x79, 0x70, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00,
            0x00, 0x00,
        ];

        let mut buf = Cursor::new(test);

        assert!(ftyp::parse(&mut buf).is_err());
    }
}
