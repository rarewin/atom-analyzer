use std::fs::File;
use std::io::BufReader;

use atom_analyzer::atom::ftyp::{self, Brand, FtypAtom};
use atom_analyzer::atom::mdat::{self, MdatAtom};
use atom_analyzer::atom::wide::{self, WideAtom};

#[test]
fn test_camouflage_vga_mov() {
    let file_name = "tests/samples/camouflage_vga.mov";
    let f = File::open(file_name).expect("file open error");
    let mut reader = BufReader::new(f);

    let t = ftyp::parse(&mut reader);

    assert_eq!(
        Some(FtypAtom {
            atom_offset: 0,
            atom_size: 20,
            major_brand: Brand::QuickTimeMovieFile,
            minor_version: 0x00000200,
            compatible_brands: vec![Brand::QuickTimeMovieFile]
        }),
        t
    );

    let t = wide::parse(&mut reader);

    assert_eq!(
        Some(WideAtom {
            atom_offset: 20,
            atom_size: 8
        }),
        t
    );

    let t = mdat::parse(&mut reader);

    assert_eq!(
        Some(MdatAtom {
            atom_offset: 28,
            atom_size: 0x6170
        }),
        t
    );
}
