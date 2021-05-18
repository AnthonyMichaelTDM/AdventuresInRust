use organizing_tests as adder; //brings the adder into scope

mod common; //brings the 'common' submodule into scope

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}