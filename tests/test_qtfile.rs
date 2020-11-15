use std::path::PathBuf;

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
        })),)
    );
}
