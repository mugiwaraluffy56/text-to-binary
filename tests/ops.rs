use binrs::ops::{
    and, count_leading_zeros, count_trailing_zeros, deinterleave, interleave, nand, nor, not, or,
    parity, popcount, reverse, rotate, shift, swap_bytes, swap_nibbles, xnor, xor,
};

#[test]
fn xor_basic() {
    assert_eq!(xor("01000001", "01000001").unwrap(), "00000000");
    assert_eq!(xor("11111111", "00000000").unwrap(), "11111111");
}

#[test]
fn xor_length_mismatch() {
    assert!(xor("01000001", "01000001 01000001").is_err());
}

#[test]
fn and_basic() {
    assert_eq!(and("11111111", "01000001").unwrap(), "01000001");
    assert_eq!(and("00000000", "11111111").unwrap(), "00000000");
}

#[test]
fn or_basic() {
    assert_eq!(or("00000000", "01000001").unwrap(), "01000001");
    assert_eq!(or("11111111", "00000000").unwrap(), "11111111");
}

#[test]
fn nand_basic() {
    assert_eq!(nand("11111111", "11111111").unwrap(), "00000000");
    assert_eq!(nand("00000000", "00000000").unwrap(), "11111111");
}

#[test]
fn nor_basic() {
    assert_eq!(nor("00000000", "00000000").unwrap(), "11111111");
    assert_eq!(nor("11111111", "00000000").unwrap(), "00000000");
}

#[test]
fn xnor_basic() {
    assert_eq!(xnor("01000001", "01000001").unwrap(), "11111111");
    assert_eq!(xnor("11111111", "00000000").unwrap(), "00000000");
}

#[test]
fn not_basic() {
    assert_eq!(not("00000000").unwrap(), "11111111");
    assert_eq!(not("11111111").unwrap(), "00000000");
    assert_eq!(not("01000001").unwrap(), "10111110");
}

#[test]
fn shift_left() {
    assert_eq!(shift("00000001", 1, true).unwrap(), "00000010");
    assert_eq!(shift("10000000", 1, true).unwrap(), "00000000");
}

#[test]
fn shift_right() {
    assert_eq!(shift("10000000", 1, false).unwrap(), "01000000");
    assert_eq!(shift("00000001", 1, false).unwrap(), "00000000");
}

#[test]
fn rotate_left() {
    assert_eq!(rotate("10000000", 1, true).unwrap(), "00000001");
    assert_eq!(rotate("00000001", 8, true).unwrap(), "00000001");
}

#[test]
fn rotate_right() {
    assert_eq!(rotate("00000001", 1, false).unwrap(), "10000000");
}

#[test]
fn reverse_basic() {
    assert_eq!(reverse("00000001").unwrap(), "10000000");
    assert_eq!(reverse("01000001").unwrap(), "10000010");
}

#[test]
fn swap_bytes_basic() {
    assert_eq!(
        swap_bytes("01000001 01000010").unwrap(),
        "01000010 01000001"
    );
}

#[test]
fn swap_nibbles_basic() {
    assert_eq!(swap_nibbles("11110000").unwrap(), "00001111");
    assert_eq!(swap_nibbles("10100101").unwrap(), "01011010");
}

#[test]
fn popcount_basic() {
    let (per_byte, total) = popcount("01000001").unwrap();
    assert_eq!(per_byte, vec![2]);
    assert_eq!(total, 2);

    let (per_byte, total) = popcount("11111111").unwrap();
    assert_eq!(per_byte, vec![8]);
    assert_eq!(total, 8);
}

#[test]
fn clz_basic() {
    let result = count_leading_zeros("00000001").unwrap();
    assert_eq!(result, vec![7]);

    let result = count_leading_zeros("10000000").unwrap();
    assert_eq!(result, vec![0]);
}

#[test]
fn ctz_basic() {
    let result = count_trailing_zeros("10000000").unwrap();
    assert_eq!(result, vec![7]);

    let result = count_trailing_zeros("00000001").unwrap();
    assert_eq!(result, vec![0]);
}

#[test]
fn parity_even() {
    let result = parity("00000000").unwrap();
    assert!(result.contains("even"));
}

#[test]
fn parity_odd() {
    let result = parity("00000001").unwrap();
    assert!(result.contains("odd"));
}

#[test]
fn interleave_deinterleave_roundtrip() {
    let a = "01000001";
    let b = "01000010";
    let interleaved = interleave(a, b).unwrap();
    let result = deinterleave(&interleaved).unwrap();
    assert!(result.contains(a));
    assert!(result.contains(b));
}
