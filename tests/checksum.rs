use binrs::convert::checksum::compute;

#[test]
fn crc8_empty() {
    let result = compute(b"", "crc8").unwrap();
    assert!(result.contains("0x00"));
}

#[test]
fn crc32_known() {
    let result = compute(b"hello", "crc32").unwrap();
    assert!(result.contains("3610a686"));
}

#[test]
fn adler32_known() {
    let result = compute(b"Wikipedia", "adler32").unwrap();
    assert!(result.contains("11e60398"));
}

#[test]
fn xor_checksum_known() {
    let result = compute(b"\xFF\xFF", "xor").unwrap();
    assert!(result.contains("0x00"));
}

#[test]
fn xor_checksum_single() {
    let result = compute(b"\xAB", "xor").unwrap();
    assert!(result.contains("0xab"));
}

#[test]
fn sum_checksum_wrapping() {
    let result = compute(b"\xFF\x01", "sum").unwrap();
    assert!(result.contains("0x00"));
}

#[test]
fn all_algo() {
    let result = compute(b"test", "all").unwrap();
    assert!(result.contains("CRC-8"));
    assert!(result.contains("CRC-32"));
    assert!(result.contains("Adler-32"));
}

#[test]
fn unknown_algo_error() {
    assert!(compute(b"test", "md5").is_err());
}

#[test]
fn crc16_ccitt_known() {
    let result = compute(b"123456789", "crc16").unwrap();
    assert!(result.contains("29b1"));
}

#[test]
fn fletcher16_known() {
    let result = compute(b"abcde", "fletcher16").unwrap();
    assert!(result.contains("c8f0"));
}
