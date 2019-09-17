pub mod ftyp;

use ftyp::Ftyp;

#[derive(Debug)]
pub struct AtomFile {
    pub ftyp: Ftyp,
}

impl<'a> AtomFile {
    pub fn new(input: &'a [u8]) -> Result<AtomFile, &'static str> {
        let ftyp = ftyp::parse_ftyp(input)?;
        Ok(AtomFile { ftyp })
    }
}

#[test]
fn test_ftyp() {
    let buf = [
        0x00, 0x00, 0x00, 0x20, 0x66, 0x74, 0x79, 0x70, 0x69, 0x73, 0x6f, 0x6d, 0x00, 0x00, 0x02,
        0x00, 0x69, 0x73, 0x6f, 0x6d, 0x69, 0x73, 0x6f, 0x32, 0x61, 0x76, 0x63, 0x31, 0x6d, 0x70,
        0x34, 0x31, 0x00, 0x00, 0x00, 0x08, 0x66, 0x72, 0x65, 0x65, 0x00, 0x00, 0x62, 0x5c, 0x6d,
        0x64, 0x61, 0x74,
    ];
    let atom = AtomFile::new(&buf).unwrap();

    println!("{:#?}", atom);

    assert_eq!(atom.ftyp.size, 0x20);
    assert_eq!(atom.ftyp.type_str, [b'f', b't', b'y', b'p']);
    assert_eq!(atom.ftyp.major_brand, [b'i', b's', b'o', b'm']);
    assert_eq!(atom.ftyp.minor_version, [0x00, 0x00, 0x02, 0x00]);

    assert_eq!(atom.ftyp.compatible_brands.len(), 4);
    assert_eq!(atom.ftyp.compatible_brands[0], [b'i', b's', b'o', b'm']);
    assert_eq!(atom.ftyp.compatible_brands[1], [b'i', b's', b'o', b'2']);
    assert_eq!(atom.ftyp.compatible_brands[2], [b'a', b'v', b'c', b'1']);
    assert_eq!(atom.ftyp.compatible_brands[3], [b'm', b'p', b'4', b'1']);
}
