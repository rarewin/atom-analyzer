pub mod free;
pub mod ftyp;
pub mod mdat;
pub mod moov;
pub mod mvhd;
pub mod wide;

use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};
use thiserror::Error;

#[derive(Debug, PartialEq)]
pub enum Atom {
    Ftyp(Box<ftyp::FtypAtom>),
    Mdat(Box<mdat::MdatAtom>),
    Wide(Box<wide::WideAtom>),
    Free(Box<free::FreeAtom>),
    Moov(Box<moov::MoovAtom>),
    Mvhd(Box<mvhd::MvhdAtom>),
}

#[derive(Debug, PartialEq)]
pub struct AtomHead {
    pub atom_offset: u64,
    pub atom_size: u64,
    pub atom_type: u32,
}

#[derive(Debug, Error)]
pub enum AtomSeekError {
    #[error("failed to seek at {0}")]
    SeekFailed(u64),
    #[error("type error at {0}")]
    TypeError(u64),
    #[error("unexpected error at {0}")]
    UnexpectedError(u64),

    #[error("")]
    NoMoreAtom,

    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

impl Atom {
    pub fn get_offset(&self) {
        match self {
            Atom::Ftyp(f) => f.atom_offset,
            Atom::Mdat(m) => m.atom_offset,
            Atom::Wide(w) => w.atom_offset,
            Atom::Free(f) => f.atom_offset,
            Atom::Moov(m) => m.atom_head.atom_offset,
            Atom::Mvhd(m) => m.atom_head.atom_offset,
        };
    }
}

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Atom, AtomSeekError> {
    let atom_head = parse_atom_head(r)?;

    r.seek(SeekFrom::Start(atom_head.atom_offset))?;

    match atom_head.atom_type {
        ftyp::ATOM_ID => Ok(Atom::Ftyp(Box::new(ftyp::parse(r)?))),
        wide::ATOM_ID => Ok(Atom::Wide(Box::new(wide::parse(r)?))),
        mdat::ATOM_ID => Ok(Atom::Mdat(Box::new(mdat::parse(r)?))),
        free::ATOM_ID => Ok(Atom::Free(Box::new(free::parse(r)?))),
        moov::ATOM_ID => Ok(Atom::Moov(Box::new(moov::parse(r)?))),
        mvhd::ATOM_ID => Ok(Atom::Mvhd(Box::new(mvhd::parse(r)?))),
        _ => unimplemented!("unknown atom"),
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
pub fn parse_atom_head<R: Read + Seek>(r: &mut R) -> Result<AtomHead, AtomSeekError> {
    let atom_offset = r.seek(SeekFrom::Current(0))?;

    let atom_size = match r.read_u32::<BigEndian>() {
        Ok(val) => val as u64,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::UnexpectedEof {
                return Err(AtomSeekError::NoMoreAtom);
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
        r.read_u64::<BigEndian>().unwrap()
    } else {
        atom_size
    };

    Ok(AtomHead {
        atom_offset,
        atom_size,
        atom_type,
    })
}
