#![allow(clippy::transmute_ptr_to_ref)] // for mopa
pub mod ctts;
pub mod dinf;
pub mod dref;
pub mod edts;
pub mod elst;
pub mod free;
pub mod ftyp;
pub mod hdlr;
pub mod mdat;
pub mod mdhd;
pub mod mdia;
pub mod minf;
pub mod moov;
pub mod mvhd;
pub mod smhd;
pub mod stbl;
pub mod stco;
pub mod stsc;
pub mod stsd;
pub mod stss;
pub mod stsz;
pub mod stts;
pub mod tkhd;
pub mod trak;
pub mod tref;
pub mod vmhd;
pub mod wide;

use std::fmt::{self, Debug};
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use thiserror::Error;

use crate::element::ElementParseError;

pub trait Atom: mopa::Any + std::fmt::Debug {}

mopafy!(Atom);

#[derive(PartialEq, Clone)]
pub struct AtomHead {
    pub atom_offset: u64,
    pub atom_size: u64,
    pub atom_type: u32,
}

#[derive(Debug, Error)]
pub enum AtomParseError {
    #[error("failed to seek at {0}")]
    SeekFailed(u64),
    #[error("atom type error at {0}")]
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

#[derive(Debug)]
pub struct UnimplementedAtom {
    pub atom_head: AtomHead,
}

impl Atom for UnimplementedAtom {}

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

pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Box<dyn Atom>, AtomParseError> {
    let atom_head = parse_atom_head(r)?;

    let atom: Box<dyn Atom> = match atom_head.atom_type {
        ftyp::ATOM_ID => Box::new(ftyp::parse(r, atom_head)?),
        wide::ATOM_ID => Box::new(wide::parse(r, atom_head)?),
        mdat::ATOM_ID => Box::new(mdat::parse(r, atom_head)?),
        free::ATOM_ID => Box::new(free::parse(r, atom_head)?),
        moov::ATOM_ID => Box::new(moov::parse(r, atom_head)?),
        mvhd::ATOM_ID => Box::new(mvhd::parse(r, atom_head)?),
        trak::ATOM_ID => Box::new(trak::parse(r, atom_head)?),
        tkhd::ATOM_ID => Box::new(tkhd::parse(r, atom_head)?),
        edts::ATOM_ID => Box::new(edts::parse(r, atom_head)?),
        elst::ATOM_ID => Box::new(elst::parse(r, atom_head)?),
        mdia::ATOM_ID => Box::new(mdia::parse(r, atom_head)?),
        mdhd::ATOM_ID => Box::new(mdhd::parse(r, atom_head)?),
        hdlr::ATOM_ID => Box::new(hdlr::parse(r, atom_head)?),
        minf::ATOM_ID => Box::new(minf::parse(r, atom_head)?),
        vmhd::ATOM_ID => Box::new(vmhd::parse(r, atom_head)?),
        dinf::ATOM_ID => Box::new(dinf::parse(r, atom_head)?),
        dref::ATOM_ID => Box::new(dref::parse(r, atom_head)?),
        smhd::ATOM_ID => Box::new(smhd::parse(r, atom_head)?),
        stbl::ATOM_ID => Box::new(stbl::parse(r, atom_head)?),
        stsd::ATOM_ID => Box::new(stsd::parse(r, atom_head)?),
        stts::ATOM_ID => Box::new(stts::parse(r, atom_head)?),
        stss::ATOM_ID => Box::new(stss::parse(r, atom_head)?),
        ctts::ATOM_ID => Box::new(ctts::parse(r, atom_head)?),
        stsc::ATOM_ID => Box::new(stsc::parse(r, atom_head)?),
        stsz::ATOM_ID => Box::new(stsz::parse(r, atom_head)?),
        stco::ATOM_ID => Box::new(stco::parse(r, atom_head)?),
        tref::ATOM_ID => Box::new(tref::parse(r, atom_head)?),
        _ => {
            r.seek(SeekFrom::Start(atom_head.atom_offset + atom_head.atom_size))?;
            Box::new(UnimplementedAtom { atom_head })
        }
    };

    Ok(atom)
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
