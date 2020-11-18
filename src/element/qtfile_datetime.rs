use std::fmt;
use std::io::{Read, Seek};

use anyhow::Result;
use byteorder::{BigEndian, ReadBytesExt};
use chrono::prelude::*;
use lazy_static::lazy_static;
use time::Duration;

lazy_static! {
    pub static ref REFERENCE_DATETIME: DateTime<Utc> = Utc.ymd(1904, 1, 1).and_hms(0, 0, 0);
}

#[derive(PartialEq)]
pub struct QtFileDateTime {
    value: u32,
    utc: DateTime<Utc>,
}

impl QtFileDateTime {
    pub fn new(value: u32) -> Self {
        let utc = *REFERENCE_DATETIME + Duration::seconds(value as i64);
        QtFileDateTime { value, utc }
    }

    pub fn parse<R: Read + Seek>(r: &mut R) -> Result<Self> {
        Ok(QtFileDateTime::new(r.read_u32::<BigEndian>()?))
    }
}

impl fmt::Display for QtFileDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.utc.format("%Y-%m-%d %H:%M:%S"))
    }
}

impl fmt::Debug for QtFileDateTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} (0x{:x})",
            self.utc.format("%Y-%m-%d %H:%M:%S"),
            self.value
        )
    }
}

#[cfg(test)]
mod test_qtfile_datetime {
    extern crate time;

    use crate::element::qtfile_datetime;
    use chrono::prelude::*;

    #[test]
    fn test_qtfile_zero() {
        let t = qtfile_datetime::QtFileDateTime::new(0);

        assert_eq!(
            t,
            qtfile_datetime::QtFileDateTime {
                value: 0,
                utc: Utc.ymd(1904, 1, 1).and_hms(0, 0, 0)
            }
        );

        assert_eq!(format!("{}", t), "1904-01-01 00:00:00");
    }

    #[test]
    fn test_qtfile_3600sec() {
        let t = qtfile_datetime::QtFileDateTime::new(3600);

        assert_eq!(
            t,
            qtfile_datetime::QtFileDateTime {
                value: 3600,
                utc: Utc.ymd(1904, 1, 1).and_hms(1, 0, 0)
            }
        );

        assert_eq!(format!("{}", t), "1904-01-01 01:00:00");
    }
}
