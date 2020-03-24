use byteorder::{BigEndian, ReadBytesExt};
use std::io::{Read, Seek, SeekFrom};

#[derive(Debug, PartialEq)]
pub struct MdatAtom {
    pub atom_offset: u64,
    pub atom_size: u64,
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Option<MdatAtom> {
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
        // atom_type should be "mdat"
        if value != 0x6d646174 {
            return None;
        }
    } else {
        return None;
    }

    match r.seek(SeekFrom::Current((atom_size as i64) - 8)) {
        Ok(_) => Some(MdatAtom {
            atom_offset,
            atom_size,
        }),
        Err(_) => None,
    }
}
