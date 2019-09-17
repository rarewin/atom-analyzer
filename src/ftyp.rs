#[derive(Debug)]
pub struct Ftyp {
    pub size: u32,
    pub type_str: [u8; 4],
    pub major_brand: [u8; 4],
    pub minor_version: [u8; 4],
    pub compatible_brands: Vec<[u8; 4]>,
}

pub fn parse_ftyp<'a>(input: &'a [u8]) -> Result<Ftyp, &'static str> {
    if input.len() < 8 {
        return Err("input data is too short");
    }

    let size = input[..4]
        .iter()
        .fold(0, |val: u32, x| (val << 8) + *x as u32);

    let mut type_str: [u8; 4] = [0; 4];
    type_str.clone_from_slice(&input[4..8]);

    let mut major_brand: [u8; 4] = [0; 4];
    major_brand.clone_from_slice(&input[8..12]);

    let mut minor_version: [u8; 4] = [0; 4];
    minor_version.clone_from_slice(&input[12..16]);

    let mut compatible_brands = Vec::<[u8; 4]>::new();

    for i in 0..((size - 16) / 4) {
        let mut brand: [u8; 4] = [0; 4];
        brand.clone_from_slice(&input[((4 * i + 16) as usize)..((4 * i + 20) as usize)]);
        compatible_brands.push(brand);
    }

    Ok(Ftyp {
        size,
        type_str,
        major_brand,
        minor_version,
        compatible_brands,
    })
}
