use std::fmt::Debug;
use std::io::{Read, Seek, SeekFrom};

use byteorder::{BigEndian, ReadBytesExt};

use crate::atom::{Atom, AtomHead, AtomParseError};
use atom_derive::atom;

pub const ATOM_ID: u32 = 0x6864_6c72; // 'hdlr'

#[atom(version)]
#[derive(Debug, PartialEq)]
pub struct HdlrAtom {
    pub component_type: ComponentType,
    pub component_sub_type: ComponentSubType,
    pub component_manufacturer: u32,
    pub component_flags: u32,
    pub component_flags_mask: u32,
    pub component_name: String,
}

#[derive(Debug, PartialEq)]
pub enum ComponentType {
    Mhlr,
    Dhlr,
    Invalid(u32),
}

impl ComponentType {
    pub fn new(t: u32) -> Self {
        match t {
            0x6d68_6c72 => ComponentType::Mhlr,
            0x6468_6c72 => ComponentType::Dhlr,
            _ => ComponentType::Invalid(t),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ComponentSubType {
    VideoMedia,
    SoundMedia,
    TimedMetadataMedia,
    TimecodeMedia,
    TextMedia,
    ClosedCaptioningMedia,
    SubtitleMedia,
    MusicMedia,
    Mpeg1Media,
    SpriteMedia,
    TweenMedia,
    ModifierTracks,
    TrackReferences,
    ThreeDimensionalMedia,
    StreamingMedia,
    HintMedia,
    VrMedia,
    MovieMedia,
    Unknown(u32),
}

impl ComponentSubType {
    pub fn new(t: u32) -> Self {
        match t {
            0x7669_6465 => ComponentSubType::VideoMedia, // 'vide'
            0x736f_756e => ComponentSubType::SoundMedia, // 'soun'
            0x6d65_7461 => ComponentSubType::TimedMetadataMedia, // 'meta'
            0x746d_6364 => ComponentSubType::TimecodeMedia, // 'tmcd'
            0x7465_7874 => ComponentSubType::TextMedia,  // 'text'
            0x636c_6370 => ComponentSubType::ClosedCaptioningMedia, // 'clcp'
            0x7362_746c => ComponentSubType::SubtitleMedia, // 'sbtl'
            0x6d75_7369 => ComponentSubType::MusicMedia, // 'musi'
            0x4d50_4547 => ComponentSubType::Mpeg1Media, // 'MPEG
            0x7370_7274 => ComponentSubType::SpriteMedia, // 'sprt'
            0x7477_656e => ComponentSubType::TweenMedia, //'twen'
            0x7164_3364 => ComponentSubType::ThreeDimensionalMedia, // 'qd3d'
            0x7374_726d => ComponentSubType::StreamingMedia, // 'strm
            _ => Self::Unknown(t),
        }
    }
}

pub fn parse<R: Read + Seek>(r: &mut R, atom_head: AtomHead) -> Result<HdlrAtom, AtomParseError> {
    let atom_version = r.read_u8()?;
    let mut atom_flags = [0_u8; 3];
    r.read_exact(&mut atom_flags)?;

    let component_type = ComponentType::new(r.read_u32::<BigEndian>()?);
    let component_sub_type = ComponentSubType::new(r.read_u32::<BigEndian>()?);

    let component_manufacturer = r.read_u32::<BigEndian>()?;
    let component_flags = r.read_u32::<BigEndian>()?;
    let component_flags_mask = r.read_u32::<BigEndian>()?;

    let remain = atom_head.atom_offset + atom_head.atom_size - r.seek(SeekFrom::Current(0))?;

    let mut component_name = String::new();

    let mut handle = r.take(remain);
    handle.read_to_string(&mut component_name)?;

    Ok(HdlrAtom {
        atom_head,
        atom_version,
        atom_flags,
        component_type,
        component_sub_type,
        component_manufacturer,
        component_flags,
        component_flags_mask,
        component_name,
    })
}
