extern crate rust_recap;

mod common;

#[test]
fn it_adds_two() {
    let nb = common::setup();
    assert_eq!(nb, 72);
    assert_eq!(rust_recap::tests::add_two(3), 5);
}
