use std::collections::HashMap;
use chrono::prelude::*;

#[derive(Debug,PartialEq,Eq)]
pub struct PackageInfo {
    pub package_type: String,
    pub count       : u32,
}

#[derive(Debug,PartialEq,Eq)]
pub struct Rotation {
    pub id: String,

    pub period_start: NaiveDate,
    pub period_end  : NaiveDate,

    pub days: HashMap<Weekday, bool>,

    pub vehicle_id  : String,
    pub vehicle_type: String,

    pub driver_id  : String,
    pub driver_name: String,

    pub packages: [PackageInfo; 3],
}
