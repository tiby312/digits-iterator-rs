use super::*;

#[test]
fn iteration_for_1_digit() {
    let mut it = Digits::new(3_u32);

    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
}

#[test]
fn iteration_for_0() {
    let mut it = Digits::new(0_u32);

    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), None);
}

#[test]
fn iteration_for_some_number() {
    let mut it = Digits::new(1234_u32);

    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), Some(4));
    assert_eq!(it.next(), None);
}

#[test]
fn iteration_for_a_multiple_of_the_base() {
    let mut it = Digits::new(100_u32);

    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), None);
}

#[test]
fn iteration_for_the_max_value() {
    let it = Digits::new(18_446_744_073_709_551_615_u64);
    let digits = vec![1, 8, 4, 4, 6, 7, 4, 4, 0, 7, 3, 7, 0, 9, 5, 5, 1, 6, 1, 5];

    for (digit, expect) in it.zip(digits.into_iter()) {
        assert_eq!(digit, expect);
    }
}

#[test]
fn reverse_iteration_for_1_digit() {
    let mut it = Digits::new(3_u32);

    assert_eq!(it.next_back(), Some(3));
    assert_eq!(it.next_back(), None);

    let mut it = Digits::new(3_u32);

    assert_eq!(it.next_back(), Some(3));
    assert_eq!(it.next(), None);
}

#[test]
fn reverse_iteration_for_0() {
    let mut it = Digits::new(0_u32);

    assert_eq!(it.next_back(), Some(0));
    assert_eq!(it.next_back(), None);

    let mut it = Digits::new(0_u32);

    assert_eq!(it.next_back(), Some(0));
    assert_eq!(it.next(), None);
}

#[test]
fn reverse_iteration_for_some_number() {
    let mut it = Digits::new(1234_u32);

    assert_eq!(it.next_back(), Some(4));
    assert_eq!(it.next_back(), Some(3));
    assert_eq!(it.next_back(), Some(2));
    assert_eq!(it.next_back(), Some(1));
    assert_eq!(it.next_back(), None);
}

#[test]
fn reverse_iteration_for_a_multiple_of_the_base() {
    let mut it = Digits::new(100_u32);

    assert_eq!(it.next_back(), Some(0));
    assert_eq!(it.next_back(), Some(0));
    assert_eq!(it.next_back(), Some(1));
    assert_eq!(it.next_back(), None);
}

#[test]
fn reverse_iteration_for_the_max_value() {
    let it = Digits::new(18_446_744_073_709_551_615_u64).rev();
    let digits = vec![5, 1, 6, 1, 5, 5, 9, 0, 7, 3, 7, 0, 4, 4, 7, 6, 4, 4, 8, 1];

    for (digit, expect) in it.zip(digits.into_iter()) {
        assert_eq!(digit, expect);
    }
}

#[test]
fn iteration_with_hexa_number() {
    let mut it = Digits::with_base(0xAB1D_u32, 16);

    assert_eq!(it.next(), Some(10));
    assert_eq!(it.next(), Some(11));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(13));
    assert_eq!(it.next_back(), None);
}
