extern crate tricktracktruck_planning_import;
extern crate chrono;

#[macro_use(assert_that)]
extern crate tricktracktruck_macros;

use tricktracktruck_planning_import::parser;
use chrono::prelude::*;

#[test]
fn check_sample_file() {
    let rotations = parser::read_file("tests/sample.txt").unwrap();
    assert_that!(rotations.len(); equals 1);

    let rotation = &rotations[0];

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
