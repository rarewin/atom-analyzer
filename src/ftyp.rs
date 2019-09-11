#[derive(Debug)]
pub struct Ftyp<'a> {
    pub size: u32,
    pub type_str: &'a [u8; 4],
    pub major_brand: u32,
}

pub fn parse_ftyp<'a>(input: &'a [u8]) -> Result<Ftyp<'a>, &'static str> {
    let size = &input[..4];

    println!("{:?}", size);

    Ok(Ftyp {
        size: 0,
        type_str: b"ftyp",
        major_brand: 0,
    })
}
