use std::fs::File;
use std::io::BufReader;

use atom_analyzer::atom::{self, ftyp, mdat, moov, wide};

#[test]
fn test_camouflage_vga_mov_manual() {
    let file_name = "tests/samples/camouflage_vga.mov";
    let f = File::open(file_name).expect("file open error");
    let mut reader = BufReader::new(f);

    assert_eq!(
        atom::parse(&mut reader).unwrap(),
        atom::Atom::Ftyp(Box::new(ftyp::FtypAtom {
            atom_offset: 0,
            atom_size: 20,
            major_brand: ftyp::Brand::QuickTimeMovieFile,
            minor_version: 0x00000200,
            compatible_brands: vec![ftyp::Brand::QuickTimeMovieFile]
        }))
    );

    assert_eq!(
        atom::parse(&mut reader).unwrap(),
        atom::Atom::Wide(Box::new(wide::WideAtom {
            atom_offset: 20,
            atom_size: 8
        })),
    );

    assert_eq!(
        atom::parse(&mut reader).unwrap(),
        atom::Atom::Mdat(Box::new(mdat::MdatAtom {
            atom_offset: 28,
            atom_size: 0x6170
        })),
    );

    assert_eq!(
        atom::parse(&mut reader).unwrap(),
        atom::Atom::Moov(Box::new(moov::MoovAtom {
            atom_offset: 0x618c,
            atom_size: 0x476,
        })),
    );
}
