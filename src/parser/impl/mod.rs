use std;
use chrono;

use std::collections::HashMap;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::vec::Vec;

use chrono::prelude::*;

use num::traits::FromPrimitive;

use ::model::{PackageInfo, Rotation};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Io(std::io::Error),
    ChronoParse(chrono::format::ParseError),
    IntParse(std::num::ParseIntError),
    InvalidLength { expected: usize, actual: usize, },
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<chrono::format::ParseError> for Error {
    fn from(err: chrono::format::ParseError) -> Error {
        Error::ChronoParse(err)
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error::IntParse(err)
    }
}

impl PackageInfo {
    fn parse(spec: &str) -> Result<PackageInfo> {
        if spec.len() != 5 {
            Err(Error::InvalidLength { expected: 5, actual: spec.len() })
        } else {
            Ok(PackageInfo {
                package_type: spec[0..2].to_string(),
                count       : spec[2..5].parse()?,
            })
        }
    }
}

impl Rotation {
    fn parse(line: &str) -> Result<Rotation> {
        if line.len() != 120 { return Err(Error::InvalidLength { expected: 120, actual: line.len(), }); }
        Ok(Rotation {
            id   : line[0 .. 10 ].to_string(),
            period_start: NaiveDate::parse_from_str(&line[10 .. 18], "%Y%m%d")?,
            period_end: NaiveDate::parse_from_str(&line[18 .. 26], "%Y%m%d")?,

            days : {
                let mut map = HashMap::new();
                for weekday_number in 0usize .. 7 {
                    let weekday = Weekday::from_i8(weekday_number as i8).unwrap();
                    map.insert(weekday, &line[26 + weekday_number .. 27 + weekday_number] == "Y");
                }
                map
            },

            vehicle_id  : line[33 .. 43].to_string(),
            vehicle_type: line[43 .. 70].to_string(),

            driver_id  : line[70 ..  80].to_string(),
            driver_name: line[80 .. 100].to_string(),

            packages: [PackageInfo::parse(&line[100 .. 105])?, PackageInfo::parse(&line[105 .. 110])?, PackageInfo::parse(&line[110 .. 115])?],
        })
    }
}

pub fn read_file(filepath: &str) -> Result<Vec<Rotation>> {
    let f      = File::open(filepath)?;
    let reader = BufReader::new(f);

    let mut rotations = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let rotation = Rotation::parse(&line)?;
        rotations.push(rotation);
    }

    Ok(rotations)
}



#[cfg(test)]
mod test;
