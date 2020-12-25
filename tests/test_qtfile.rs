use std::path::PathBuf;

use fixed::{
    types::extra::{U16, U8},
    FixedU16, FixedU32,
};

use atom_analyzer::atom::{self, ftyp, mdat, moov, wide};
use atom_analyzer::element::{qtfile_datetime, qtfile_matrix};
use atom_analyzer::qtfile;

use atom_analyzer::atom::ctts::CompositionOffsetTableEntry;
use atom_analyzer::atom::stsc::SampleToChunk;

#[test]
fn test_camouflage_vga_mov_manual() {
    let file_name = PathBuf::from("tests/samples/camouflage_vga.mov");
    let mut qt = qtfile::parse_file(file_name).unwrap();

    let ftyp = qt.next().unwrap();

    assert!(ftyp.is::<ftyp::FtypAtom>());
    assert_eq!(
        ftyp.downcast_ref::<ftyp::FtypAtom>(),
        Some(&ftyp::FtypAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0,
                atom_size: 20,
                atom_type: atom::ftyp::ATOM_ID,
            },
            major_brand: ftyp::Brand::QuickTimeMovieFile,
            minor_version: 0x00000200,
            compatible_brands: vec![ftyp::Brand::QuickTimeMovieFile]
        })
    );

    let wide = qt.next().unwrap();

    assert!(wide.is::<wide::WideAtom>());
    assert_eq!(
        wide.downcast_ref::<wide::WideAtom>(),
        Some(&wide::WideAtom {
            atom_head: atom::AtomHead {
                atom_offset: 20,
                atom_size: 8,
                atom_type: atom::wide::ATOM_ID,
            },
        }),
    );

    let mdat = qt.next().unwrap();

    assert!(mdat.is::<mdat::MdatAtom>());
    assert_eq!(
        mdat.downcast_ref::<mdat::MdatAtom>(),
        Some(&mdat::MdatAtom {
            atom_head: atom::AtomHead {
                atom_offset: 28,
                atom_size: 0x6170,
                atom_type: atom::mdat::ATOM_ID,
            },
        }),
    );

    let moov = qt.next().unwrap();

    assert!(moov.is::<moov::MoovAtom>());

    let moov = moov.downcast_ref::<moov::MoovAtom>().unwrap();

    assert_eq!(
        moov.atom_head,
        atom::AtomHead {
            atom_type: atom::moov::ATOM_ID,
            atom_offset: 0x618c,
            atom_size: 0x476,
        }
    );

    assert_eq!(
        moov.mvhd_atom,
        Some(Box::new(atom::mvhd::MvhdAtom {
            atom_head: atom::AtomHead {
                atom_type: atom::mvhd::ATOM_ID,
                atom_offset: 0x6194,
                atom_size: 0x6c,
            },
            atom_version: 0,
            atom_flags: [0, 0, 0],
            creation_time: qtfile_datetime::QtFileDateTime::new(0),
            modification_time: qtfile_datetime::QtFileDateTime::new(0),
            time_scale: 1000,
            duration: 1000,
            preferred_rate: 0x10000,
            preferred_volume: 0x100,
            matrix_structure: qtfile_matrix::QtFileMatrix::new(&[
                0x10000, 0, 0, 0, 0x10000, 0, 0, 0, 0x40000000
            ]),
            preview_time: qtfile_datetime::QtFileDateTime::new(0),
            preview_duration: 0,
            poster_time: qtfile_datetime::QtFileDateTime::new(0),
            selection_time: qtfile_datetime::QtFileDateTime::new(0),
            selection_duration: 0,
            current_time: qtfile_datetime::QtFileDateTime::new(0),
            next_track_id: 2,
        })),
    );

    assert_eq!(moov.trak_atom.len(), 1);

    let trak_atom = &moov.trak_atom[0];

    assert_eq!(
        trak_atom.atom_head,
        atom::AtomHead {
            atom_offset: 0x6200,
            atom_size: 0x3e1,
            atom_type: atom::trak::ATOM_ID,
        },
    );

    let tkhd_atom = &trak_atom.tkhd_atom;

    assert_eq!(
        tkhd_atom,
        &Box::new(atom::tkhd::TkhdAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0x6208,
                atom_size: 0x5c,
                atom_type: atom::tkhd::ATOM_ID,
            },
            atom_version: 0,
            atom_flags: [0, 0, 3],
            creation_time: qtfile_datetime::QtFileDateTime::new(0),
            modification_time: qtfile_datetime::QtFileDateTime::new(0),
            track_id: 1,
            reserved0: 0,
            duration: 1000,
            reserved1: [0, 0, 0, 0, 0, 0, 0, 0],
            layer: 0,
            alternate_group: 0,
            volume: FixedU16::<U8>::from_num(0),
            reserved2: 0,
            matrix_structure: qtfile_matrix::QtFileMatrix::new(&[
                0x10000, 0, 0, 0, 0x10000, 0, 0, 0, 0x40000000
            ]),
            track_width: FixedU32::<U16>::from_num(640),
            track_height: FixedU32::<U16>::from_num(400),
        }),
    );

    let edts_atom = &trak_atom.edts_atom;

    assert_eq!(
        edts_atom,
        &Some(Box::new(atom::edts::EdtsAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0x6264,
                atom_size: 0x24,
                atom_type: atom::edts::ATOM_ID,
            },
            elst_atom: Some(Box::new(atom::elst::ElstAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x626c,
                    atom_size: 0x1c,
                    atom_type: atom::elst::ATOM_ID,
                },
                atom_version: 0,
                atom_flags: [0, 0, 0],
                number_of_entries: 1,
                edit_list_table: vec![atom::elst::EditListTableEntry {
                    track_duration: 1000,
                    media_time: 1024,
                    media_rate: FixedU32::<U16>::from_num(1),
                }],
            })),
        })),
    );

    let mdia_atom = &trak_atom.mdia_atom;

    assert_eq!(
        mdia_atom.atom_head,
        atom::AtomHead {
            atom_offset: 0x6288,
            atom_size: 0x359,
            atom_type: atom::mdia::ATOM_ID,
        },
    );

    let mdhd_atom = &mdia_atom.mdhd_atom;

    assert_eq!(
        mdhd_atom,
        &Box::new(atom::mdhd::MdhdAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0x6290,
                atom_size: 0x20,
                atom_type: atom::mdhd::ATOM_ID,
            }
        }),
    );

    let hdlr_atom = &mdia_atom.hdlr_atom;

    assert_eq!(
        hdlr_atom,
        &Some(Box::new(atom::hdlr::HdlrAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0x62b0,
                atom_size: 0x2d,
                atom_type: atom::hdlr::ATOM_ID,
            },
            atom_version: 0,
            atom_flags: [0, 0, 0],
            component_type: atom::hdlr::ComponentType::Mhlr,
            component_sub_type: atom::hdlr::ComponentSubType::VideoMedia,
            component_manufacturer: 0,
            component_flags: 0,
            component_flags_mask: 0,
            component_name: "\u{c}VideoHandler".into()
        })),
    );

    let minf_atom = &mdia_atom.minf_atom.as_ref().unwrap();

    assert_eq!(
        minf_atom.atom_head,
        atom::AtomHead {
            atom_offset: 0x62dd,
            atom_size: 0x304,
            atom_type: atom::minf::ATOM_ID,
        },
    );

    let (vmhd_atom, hdlr_atom, dinf_atom, stbl_atom) = match &minf_atom.media_info {
        atom::minf::MediaInfo::VideoMediaInfo {
            vmhd_atom,
            hdlr_atom,
            dinf_atom,
            stbl_atom,
        } => (vmhd_atom, hdlr_atom, dinf_atom, stbl_atom),
        _ => panic!(),
    };

    assert_eq!(
        vmhd_atom,
        &Box::new(atom::vmhd::VmhdAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0x62e5,
                atom_size: 0x14,
                atom_type: atom::vmhd::ATOM_ID,
            },
        }),
    );

    assert_eq!(
        hdlr_atom,
        &Box::new(atom::hdlr::HdlrAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0x62f9,
                atom_size: 0x2c,
                atom_type: atom::hdlr::ATOM_ID,
            },
            atom_version: 0,
            atom_flags: [0, 0, 0],
            component_type: atom::hdlr::ComponentType::Dhlr,
            component_sub_type: atom::hdlr::ComponentSubType::Unknown(0x7572_6c20),
            component_manufacturer: 0,
            component_flags: 0,
            component_flags_mask: 0,
            component_name: "\u{b}DataHandler".into()
        }),
    );

    assert_eq!(
        dinf_atom,
        &Some(Box::new(atom::dinf::DinfAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0x6325,
                atom_size: 0x24,
                atom_type: atom::dinf::ATOM_ID,
            },
            dref_atom: Box::new(atom::dref::DrefAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x632d,
                    atom_size: 0x1c,
                    atom_type: atom::dref::ATOM_ID,
                },
                atom_version: 0,
                atom_flags: [0, 0, 0],
                number_of_entries: 1,
                data_references: vec![atom::dref::DataReferenceType::Url {
                    atom_head: atom::AtomHead {
                        atom_offset: 0x633d,
                        atom_size: 0x0c,
                        atom_type: 0x7572_6c20, // @todo
                    },
                    url: "\u{0}\u{0}\u{0}\u{1}".into()
                }],
            }),
        }),),
    );

    assert_eq!(
        stbl_atom,
        &Some(Box::new(atom::stbl::StblAtom {
            atom_head: atom::AtomHead {
                atom_offset: 0x6349,
                atom_size: 0x298,
                atom_type: atom::stbl::ATOM_ID,
            },
            stsd_atom: Some(Box::new(atom::stsd::StsdAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x6351,
                    atom_size: 0xa8,
                    atom_type: atom::stsd::ATOM_ID,
                },
                atom_version: 0,
                atom_flags: [0, 0, 0],
                number_of_entries: 1,
                sample_description_table: vec![atom::stsd::SampleDescription {
                    sample_description_size: 0x98,
                    data_format: 0x6176_6331,
                    reserved: [0, 0, 0, 0, 0, 0],
                    data_reference_index: 1,
                    data: vec![]
                }]
            })),
            stts_atom: Some(Box::new(atom::stts::SttsAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x63f9,
                    atom_size: 0x18,
                    atom_type: atom::stts::ATOM_ID,
                },
                atom_version: 0,
                atom_flags: [0, 0, 0],
                number_of_entries: 1,
                time_to_sample_table: vec![atom::stts::TimeToSampleEntry {
                    sample_count: 0x1e,
                    sample_duration: 0x200,
                }],
            })),
            stss_atom: Some(Box::new(atom::stss::StssAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x6411,
                    atom_size: 0x14,
                    atom_type: atom::stss::ATOM_ID,
                },
                atom_version: 0,
                atom_flags: [0, 0, 0],
                number_of_entries: 1,
                sync_sample_table: vec![1,]
            })),
            ctts_atom: Some(Box::new(atom::ctts::CttsAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x6425,
                    atom_size: 0x100,
                    atom_type: atom::ctts::ATOM_ID,
                },
                atom_version: 0,
                atom_flags: [0, 0, 0],
                entry_count: 30,
                composition_offset_table: vec![
                    CompositionOffsetTableEntry::new(1, 0x400),
                    CompositionOffsetTableEntry::new(1, 0xa00),
                    CompositionOffsetTableEntry::new(1, 0x400),
                    CompositionOffsetTableEntry::new(1, 0),
                    CompositionOffsetTableEntry::new(1, 0x200),
                    CompositionOffsetTableEntry::new(1, 0xa00),
                    CompositionOffsetTableEntry::new(1, 0x400),
                    CompositionOffsetTableEntry::new(1, 0),
                    CompositionOffsetTableEntry::new(1, 0x200),
                    CompositionOffsetTableEntry::new(1, 0xa00),
                    CompositionOffsetTableEntry::new(1, 0x400),
                    CompositionOffsetTableEntry::new(1, 0),
                    CompositionOffsetTableEntry::new(1, 0x200),
                    CompositionOffsetTableEntry::new(1, 0xa00),
                    CompositionOffsetTableEntry::new(1, 0x400),
                    CompositionOffsetTableEntry::new(1, 0),
                    CompositionOffsetTableEntry::new(1, 0x200),
                    CompositionOffsetTableEntry::new(1, 0xa00),
                    CompositionOffsetTableEntry::new(1, 0x400),
                    CompositionOffsetTableEntry::new(1, 0),
                    CompositionOffsetTableEntry::new(1, 0x200),
                    CompositionOffsetTableEntry::new(1, 0xa00),
                    CompositionOffsetTableEntry::new(1, 0x400),
                    CompositionOffsetTableEntry::new(1, 0),
                    CompositionOffsetTableEntry::new(1, 0x200),
                    CompositionOffsetTableEntry::new(1, 0xa00),
                    CompositionOffsetTableEntry::new(1, 0x400),
                    CompositionOffsetTableEntry::new(1, 0),
                    CompositionOffsetTableEntry::new(1, 0x200),
                    CompositionOffsetTableEntry::new(1, 0x400),
                ]
            })),
            stsc_atom: Some(Box::new(atom::stsc::StscAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x6525,
                    atom_size: 0x1c,
                    atom_type: atom::stsc::ATOM_ID,
                },
                atom_version: 0,
                atom_flags: [0, 0, 0],
                number_of_entries: 1,
                sample_to_chunk_table: vec![SampleToChunk::new(1, 30, 1),]
            })),
            stsz_atom: Some(Box::new(atom::stsz::StszAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x6541,
                    atom_size: 0x8c,
                    atom_type: atom::stsz::ATOM_ID,
                },
                atom_version: 0,
                atom_flags: [0, 0, 0],
                sample_size: 0,
                number_of_entries: 30,
                sample_size_table: vec![
                    0x5c82, 0x0187, 0x003d, 0x0033, 0x001f, 0x005a, 0x0024, 0x0017, 0x0017, 0x003a,
                    0x001a, 0x0012, 0x0012, 0x0039, 0x001b, 0x0012, 0x0012, 0x0034, 0x0019, 0x0012,
                    0x0013, 0x0018, 0x0019, 0x0012, 0x0012, 0x0018, 0x0018, 0x0012, 0x0012, 0x0019,
                ],
            })),
        })),
    );
}
