extern crate tricktracktruck_planning_import as importer;

#[test]
fn check_empty_file() {
    let rotations = importer::parser::read_file("tests/empty.txt").unwrap();
    assert_eq!(0, rotations.len());
}