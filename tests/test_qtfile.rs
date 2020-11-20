use std::path::PathBuf;

use fixed::{
    types::extra::{U16, U8},
    FixedU16, FixedU32,
};

use atom_analyzer::atom::{self, ftyp, mdat, moov, wide};
use atom_analyzer::element::{qtfile_datetime, qtfile_matrix};
use atom_analyzer::qtfile;

#[test]
fn test_camouflage_vga_mov_manual() {
    let file_name = PathBuf::from("tests/samples/camouflage_vga.mov");
    let mut qt = qtfile::parse_file(file_name).unwrap();

    assert_eq!(
        qt.next(),
        Some(atom::Atom::Ftyp(Box::new(ftyp::FtypAtom {
            atom_offset: 0,
            atom_size: 20,
            major_brand: ftyp::Brand::QuickTimeMovieFile,
            minor_version: 0x00000200,
            compatible_brands: vec![ftyp::Brand::QuickTimeMovieFile]
        })))
    );

    assert_eq!(
        qt.next(),
        Some(atom::Atom::Wide(Box::new(wide::WideAtom {
            atom_offset: 20,
            atom_size: 8
        }))),
    );

    assert_eq!(
        qt.next(),
        Some(atom::Atom::Mdat(Box::new(mdat::MdatAtom {
            atom_offset: 28,
            atom_size: 0x6170
        }))),
    );

    assert_eq!(
        qt.next(),
        Some(atom::Atom::Moov(Box::new(moov::MoovAtom {
            atom_head: atom::AtomHead {
                atom_type: atom::moov::ATOM_ID,
                atom_offset: 0x618c,
                atom_size: 0x476,
            },
            mvhd_atom: Some(atom::mvhd::MvhdAtom {
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
            }),
            trak_atom: vec![atom::trak::TrakAtom {
                atom_head: atom::AtomHead {
                    atom_offset: 0x6200,
                    atom_size: 0x3e1,
                    atom_type: atom::trak::ATOM_ID,
                },
                tkhd_atom: atom::tkhd::TkhdAtom {
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
                },
                edts_atom: Some(atom::edts::EdtsAtom {
                    atom_head: atom::AtomHead {
                        atom_offset: 0x6264,
                        atom_size: 0x24,
                        atom_type: atom::edts::ATOM_ID,
                    },
                    elst_atom: Some(atom::elst::ElstAtom {
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
                    }),
                },),
                mdia_atom: atom::mdia::MdiaAtom {
                    atom_head: atom::AtomHead {
                        atom_offset: 0x6288,
                        atom_size: 0x359,
                        atom_type: atom::mdia::ATOM_ID,
                    }
                }
            }],
        })),)
    );
}
