use std::fmt;
use std::io::{Read, Seek};

use byteorder::{BigEndian, ReadBytesExt};
use fixed::{
    types::extra::{U16, U30},
    FixedU32,
};

use crate::element::ElementParseError;

#[derive(PartialEq)]
pub struct QtFileMatrix {
    a: FixedU32<U16>,
    b: FixedU32<U16>,
    c: FixedU32<U16>,
    d: FixedU32<U16>,
    t_x: FixedU32<U16>,
    t_y: FixedU32<U16>,
    u: FixedU32<U30>,
    v: FixedU32<U30>,
    w: FixedU32<U30>,
}

impl QtFileMatrix {
    pub fn new(value: &[u32; 9]) -> Self {
        QtFileMatrix {
            a: FixedU32::<U16>::from_bits(value[0]),
            b: FixedU32::<U16>::from_bits(value[1]),
            c: FixedU32::<U16>::from_bits(value[3]),
            d: FixedU32::<U16>::from_bits(value[4]),
            t_x: FixedU32::<U16>::from_bits(value[6]),
            t_y: FixedU32::<U16>::from_bits(value[7]),
            u: FixedU32::<U30>::from_bits(value[2]),
            v: FixedU32::<U30>::from_bits(value[5]),
            w: FixedU32::<U30>::from_bits(value[8]),
        }
    }

    pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Self, ElementParseError> {
        let mut matrix = [0_u32; 9];

        for element in &mut matrix {
            *element = r.read_u32::<BigEndian>()?;
        }

        Ok(QtFileMatrix::new(&matrix))
    }
}

impl fmt::Display for QtFileMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[[{}, {}, {}], [{}, {}, {}], [{}, {}, {}]]",
            self.a, self.b, self.u, self.c, self.d, self.v, self.t_x, self.t_y, self.w,
        )
    }
}

impl fmt::Debug for QtFileMatrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entry(&format_args!("[{:?}, {:?}, {:?}]", self.a, self.b, self.u))
            .entry(&format_args!("[{:?}, {:?}, {:?}]", self.c, self.d, self.v))
            .entry(&format_args!(
                "[{:?}, {:?}, {:?}]",
                self.t_x, self.t_y, self.w
            ))
            .finish()
    }
}

#[cfg(test)]
mod test_qtfile_matrix {
    use crate::element::qtfile_matrix;

    #[test]
    fn test_matrix_all_zero() {
        let t = qtfile_matrix::QtFileMatrix::new(&[0 as u32; 9]);
        assert_eq!(format!("{}", t), "[[0, 0, 0], [0, 0, 0], [0, 0, 0]]");
    }

    #[test]
    fn test_matrix_all_one() {
        let t = qtfile_matrix::QtFileMatrix::new(&[
            0x10000, 0x10000, 0x40000000, 0x10000, 0x10000, 0x40000000, 0x10000, 0x10000,
            0x40000000,
        ]);
        assert_eq!(format!("{}", t), "[[1, 1, 1], [1, 1, 1], [1, 1, 1]]");
    }
}
