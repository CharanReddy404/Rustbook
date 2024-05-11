use controll_test_run;

mod common;

#[test]
fn it_adds_two() {
    // cargo test --test integration_test
    common::setup();
    assert_eq!(4, controll_test_run::add_two(2));
}
