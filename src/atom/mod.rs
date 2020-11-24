pub mod edts;
pub mod elst;
pub mod free;
pub mod ftyp;
pub mod mdat;
pub mod mdhd;
pub mod mdia;
pub mod moov;
pub mod mvhd;
pub mod tkhd;
pub mod trak;
pub mod wide;

use std::fmt;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use fixed::{
    types::extra::{U16, U8},
    FixedU16, FixedU32,
};
use thiserror::Error;

use crate::element::{self, ElementParseError};

#[derive(Debug, PartialEq)]
pub enum Atom {
    Ftyp {
        atom_head: AtomHead,
        major_brand: ftyp::Brand,
        minor_version: u32,
        compatible_brands: Vec<ftyp::Brand>,
    },
    Mdat {
        atom_head: AtomHead,
    },
    Wide {
        atom_head: AtomHead,
    },
    Free {
        atom_head: AtomHead,
    },
    Moov {
        atom_head: AtomHead,
        mvhd_atom: Option<Box<Atom>>,
        trak_atom: Vec<Atom>,
    },
    Mvhd {
        atom_head: AtomHead,
        atom_version: u8,
        atom_flags: [u8; 3],
        creation_time: element::qtfile_datetime::QtFileDateTime,
        modification_time: element::qtfile_datetime::QtFileDateTime,
        time_scale: u32,
        duration: u32,
        preferred_rate: u32,
        preferred_volume: u16,
        matrix_structure: element::qtfile_matrix::QtFileMatrix,
        preview_time: element::qtfile_datetime::QtFileDateTime,
        preview_duration: u32,
        poster_time: element::qtfile_datetime::QtFileDateTime,
        selection_time: element::qtfile_datetime::QtFileDateTime,
        selection_duration: u32,
        current_time: element::qtfile_datetime::QtFileDateTime,
        next_track_id: u32,
    },
    Trak {
        atom_head: AtomHead,
        tkhd_atom: Box<Atom>,
        edts_atom: Option<Box<Atom>>,
        mdia_atom: self::mdia::MdiaAtom,
    },
    Tkhd {
        atom_head: AtomHead,
        atom_version: u8,
        atom_flags: [u8; 3],
        creation_time: element::qtfile_datetime::QtFileDateTime,
        modification_time: element::qtfile_datetime::QtFileDateTime,
        track_id: u32,
        reserved0: u32,
        duration: u32,
        reserved1: [u8; 8],
        layer: u16,
        alternate_group: u16,
        volume: FixedU16<U8>,
        reserved2: u16,
        matrix_structure: element::qtfile_matrix::QtFileMatrix,
        track_width: FixedU32<U16>,
        track_height: FixedU32<U16>,
    },
    Edts {
        atom_head: AtomHead,
        elst_atom: Option<self::elst::ElstAtom>,
    },
    Elst(Box<elst::ElstAtom>),
    Mdia(Box<mdia::MdiaAtom>),
    Mdhd(Box<mdhd::MdhdAtom>),
    Unimplemented(AtomHead),
}

#[derive(PartialEq)]
pub struct AtomHead {
    pub atom_offset: u64,
    pub atom_size: u64,
    pub atom_type: u32,
}

#[derive(Debug, Error)]
pub enum AtomParseError {
    #[error("failed to seek at {0}")]
    SeekFailed(u64),
    #[error("type error at {0}")]
    TypeError(u64),
    #[error("required atom {0} was not found")]
    RequiredAtomNotFound(u32),
    #[error("unexpected error at {0}")]
    UnexpectedError(u64),

    #[error("")]
    NoMoreAtom,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    ElementParseError(#[from] ElementParseError),
}

impl fmt::Debug for AtomHead {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = [0; 4];
        BigEndian::write_u32(&mut buf, self.atom_type);

        f.debug_struct("AtomHead")
            .field("offset", &format_args!("0x{:016x}", self.atom_offset))
            .field("size", &format_args!("0x{:016x}", self.atom_size))
            .field(
                "type",
                &format_args!(
                    "{:?} (0x{:08x})",
                    buf.iter().map(|c| char::from(*c)).collect::<String>(),
                    self.atom_type
                ),
            )
            .finish()
    }
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Atom, AtomParseError> {
    let atom_head = parse_atom_head(r)?;

    r.seek(SeekFrom::Start(atom_head.atom_offset))?;

    match atom_head.atom_type {
        ftyp::ATOM_ID => Ok(ftyp::parse(r)?),
        wide::ATOM_ID => Ok(wide::parse(r)?),
        mdat::ATOM_ID => Ok(mdat::parse(r)?),
        free::ATOM_ID => Ok(free::parse(r)?),
        moov::ATOM_ID => Ok(moov::parse(r)?),
        mvhd::ATOM_ID => Ok(mvhd::parse(r)?),
        trak::ATOM_ID => Ok(trak::parse(r)?),
        tkhd::ATOM_ID => Ok(tkhd::parse(r)?),
        edts::ATOM_ID => Ok(edts::parse(r)?),
        elst::ATOM_ID => Ok(Atom::Elst(Box::new(elst::parse(r)?))),
        mdia::ATOM_ID => Ok(Atom::Mdia(Box::new(mdia::parse(r)?))),
        mdhd::ATOM_ID => Ok(Atom::Mdhd(Box::new(mdhd::parse(r)?))),
        _ => {
            r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
            Ok(Atom::Unimplemented(atom_head))
        }
    }
}

/// Returns an AtomHead from `r`
///
/// # Arguments
///
/// * `r` - input data
///
/// # Examples
///
/// ```
/// extern crate atom_analyzer;
/// use std::io::Cursor;
/// use atom_analyzer::atom::{parse_atom_head, AtomHead};
///
/// let test: Vec<u8> = vec![
///     0x00, 0x00, 0x00, 0x08, 0x66, 0x74, 0x79, 0x70,
/// ];
/// let mut buf = Cursor::new(test);
///
/// assert_eq!(
///     parse_atom_head(&mut buf).unwrap(),
///     AtomHead{
///         atom_offset: 0,
///         atom_size: 8,
///         atom_type: 0x66747970,
///     }
/// );
/// ```
pub fn parse_atom_head<R: Read + Seek>(r: &mut R) -> Result<AtomHead, AtomParseError> {
    let atom_offset = r.seek(SeekFrom::Current(0))?;

    let atom_size = match r.read_u32::<BigEndian>() {
        Ok(val) => val as u64,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(AtomParseError::NoMoreAtom);
            } else {
                panic!();
            }
        }
    };

    let atom_type = r.read_u32::<BigEndian>()?;

    if atom_size == 0 {
        unimplemented!("atom with zero size is not implemented yet");
    }

    // extended size
    let atom_size = if atom_size == 1 {
        r.read_u64::<BigEndian>()?
    } else {
        atom_size
    };

    Ok(AtomHead {
        atom_offset,
        atom_size,
        atom_type,
    })
}
