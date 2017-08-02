use chrono::prelude::*;
use model::{PackageInfo, Rotation};
use super::Error;

#[test]
#[allow(non_snake_case)]
fn test_packageinfo_parse_AB012() {
    let packageinfo = PackageInfo::parse("AB012").unwrap();
    assert_that!(packageinfo.package_type; equals "AB");
    assert_that!(packageinfo.count       ; equals   12);
}

#[test]
#[allow(non_snake_case)]
fn test_packageinfo_parse_AB01() {
    match PackageInfo::parse("AB01").err().unwrap() {
        Error::InvalidLength { actual: actual_length, expected: expected_length } => {
            assert_that!(actual_length  ; equals 4);
            assert_that!(expected_length; equals 5);
        },
        e => assert!(false, "Should result in invalid length: {:?}", e),
    }
}

#[test]
#[allow(non_snake_case)]
fn test_packageinfo_parse_ABA01() {
    match PackageInfo::parse("ABA01").err().unwrap() {
        Error::IntParse(_) => (),
        e                    => assert!(false, "Should result in int parsing error: {:?}", e),
    }
}

#[test]
fn test_rotation_parse_empty() {
    match Rotation::parse("").err().unwrap() {
        Error::InvalidLength { actual: actual_length, expected: expected_length } => {
            assert_that!(actual_length  ; equals   0);
            assert_that!(expected_length; equals 120);
        },
        e => assert!(false, "Should result in invalid length error: {:?}", e),
    }
}

#[test]
fn test_rotation_parse_130_characters_long() {
    match Rotation::parse("0123456789112345678921234567893123456789412345678951234567896123456789712345678981234567899123456789012345678911234567892123456789").err().unwrap() {
        Error::InvalidLength { actual: actual_length, expected: expected_length } => {
            assert_that!(actual_length  ; equals 130);
            assert_that!(expected_length; equals 120);
        },
        e => assert!(false, "Should result in invalid length error: {:?}", e),
    }
}

#[test]
fn test_rotation_parse_sample() {
    let line = "01234567892017031320170319YYYYYNN01234567890123456789ABCDEFGHIJKLMNOPQ01234567890123456789ABCDEFGHIJAB012CD345EF678     ";
    let rotation = match Rotation::parse(line) {
        Ok(rotation) => rotation,
        Err(e)       => { println!("{:?}", e); panic!("Parsing error"); }
    };

    assert_that!(rotation.id                      ; equals "0123456789");

    assert_that!(rotation.period_start            ; equals NaiveDate::from_ymd(2017, 03, 13));
    assert_that!(rotation.period_end              ; equals NaiveDate::from_ymd(2017, 03, 19));

    assert_that!(rotation.days[&Weekday::Mon]     ; equals true);
    assert_that!(rotation.days[&Weekday::Tue]     ; equals true);
    assert_that!(rotation.days[&Weekday::Wed]     ; equals true);
    assert_that!(rotation.days[&Weekday::Thu]     ; equals true);
    assert_that!(rotation.days[&Weekday::Fri]     ; equals true);
    assert_that!(rotation.days[&Weekday::Sat]     ; equals false);
    assert_that!(rotation.days[&Weekday::Sun]     ; equals false);

    assert_that!(rotation.vehicle_id              ; equals "0123456789");
    assert_that!(rotation.vehicle_type            ; equals "0123456789ABCDEFGHIJKLMNOPQ");

    assert_that!(rotation.driver_id               ; equals "0123456789");
    assert_that!(rotation.driver_name             ; equals "0123456789ABCDEFGHIJ");

    assert_that!(rotation.packages[0].package_type; equals "AB");
    assert_that!(rotation.packages[0].count       ; equals   12);
    assert_that!(rotation.packages[1].package_type; equals "CD");
    assert_that!(rotation.packages[1].count       ; equals  345);
    assert_that!(rotation.packages[2].package_type; equals "EF");
    assert_that!(rotation.packages[2].count       ; equals  678);
}