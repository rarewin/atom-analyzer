use std::fs::File;
use std::io::BufReader;

use atom_analyzer::atom::ftyp;
use atom_analyzer::atom::ftyp::{FtypAtom, MajorBrand};

#[test]
fn test_camouflage_vga_mov() {
    let file_name = "tests/samples/camouflage_vga.mov";
    let f = File::open(file_name).expect("file open error");
    let mut reader = BufReader::new(f);

    let t = ftyp::parse(&mut reader, 0);

    assert_eq!(
        Some(FtypAtom {
            atom_offset: 0,
            atom_size: 20,
            major_brand: MajorBrand::QuickTimeMovieFile,
        }),
        t
    );
}
