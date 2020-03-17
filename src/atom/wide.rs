use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, PartialEq)]
pub struct WideAtom {
    pub atom_offset: u64,
    pub atom_size: u64,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Option<WideAtom> {
    let atom_offset = if let Ok(offset) = r.seek(SeekFrom::Current(0)) {
        offset
    } else {
        return None;
    };

    let atom_size = if let Ok(value) = r.read_u32::<BigEndian>() {
        value as u64
    } else {
        return None;
    };

    if let Ok(value) = r.read_u32::<BigEndian>() {
        // atom_type should be "wide"
        if value != 0x77696465 {
            return None;
        }
    } else {
        return None;
    }

    Some(WideAtom {
        atom_offset,
        atom_size,
    })
}
