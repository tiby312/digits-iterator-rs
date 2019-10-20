use super::*;

#[test]
#[should_panic]
fn invalid_base_1() {
    let _ = 1u8.digits_with_base(1);
}

#[test]
#[should_panic]
fn invalid_base_37() {
    let _ = 1u8.digits_with_base(37);
}

#[test]
fn simple_extension_test() {
    let digits: Vec<_> = 42.digits_with_base(2).collect();

    assert_eq!(digits[..], [1_u8, 0, 1, 0, 1, 0]);
}
