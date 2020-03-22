use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, PartialEq)]
pub struct MoovAtom {
    pub atom_offset: u64,
    pub atom_size: u64,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Option<MoovAtom> {
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
        // atom_type should be "moov"
        if value != 0x6d6f6f76 {
            panic!("moov atom not found at 0x{:x}", atom_offset);
            return None;
        }
    } else {
        return None;
    }

    Some(MoovAtom {
        atom_offset,
        atom_size,
    })
}
