use codec::encode::{
    decode_base64, decode_bcd, decode_gray, decode_run_length_binary, encode, encode_base64,
    encode_bcd, encode_gray, encode_run_length_binary, EncodeOpts,
};
use codec::format::Format;

fn default_opts(fmt: Format) -> EncodeOpts {
    EncodeOpts {
        format: fmt,
        sep: " ".to_string(),
        width: None,
        lsb_first: false,
        nibble_groups: false,
    }
}

#[test]
fn encode_bin_basic() {
    let result = encode(b"A", &default_opts(Format::Bin));
    assert_eq!(result, "01000001");
}

#[test]
fn encode_bin_multi() {
    let result = encode(b"hi", &default_opts(Format::Bin));
    assert_eq!(result, "01101000 01101001");
}

#[test]
fn encode_hex() {
    let result = encode(b"A", &default_opts(Format::Hex));
    assert_eq!(result, "41");
}

#[test]
fn encode_oct() {
    let result = encode(b"A", &default_opts(Format::Oct));
    assert_eq!(result, "101");
}

#[test]
fn encode_dec() {
    let result = encode(b"A", &default_opts(Format::Dec));
    assert_eq!(result, "65");
}

#[test]
fn encode_with_width() {
    let opts = EncodeOpts {
        format: Format::Bin,
        sep: " ".to_string(),
        width: Some(2),
        lsb_first: false,
        nibble_groups: false,
    };
    let result = encode(b"ABC", &opts);
    assert_eq!(result, "01000001 01000010\n01000011");
}

#[test]
fn encode_nibble_groups() {
    let opts = EncodeOpts {
        format: Format::Bin,
        sep: " ".to_string(),
        width: None,
        lsb_first: false,
        nibble_groups: true,
    };
    let result = encode(b"A", &opts);
    assert_eq!(result, "0100.0001");
}

#[test]
fn encode_lsb_first() {
    let opts = EncodeOpts {
        format: Format::Bin,
        sep: " ".to_string(),
        width: None,
        lsb_first: true,
        nibble_groups: false,
    };
    let result = encode(b"\x01", &opts);
    assert_eq!(result, "10000000");
}

#[test]
fn encode_empty() {
    let result = encode(b"", &default_opts(Format::Bin));
    assert_eq!(result, "");
}

#[test]
fn gray_roundtrip() {
    let input = b"hello";
    let encoded = encode_gray(input, " ", None);
    let decoded = decode_gray(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn gray_known_values() {
    let encoded = encode_gray(&[0, 1, 2, 3, 4], " ", None);
    let tokens: Vec<&str> = encoded.split_whitespace().collect();
    assert_eq!(tokens[0], "00000000");
    assert_eq!(tokens[1], "00000001");
    assert_eq!(tokens[2], "00000011");
    assert_eq!(tokens[3], "00000010");
    assert_eq!(tokens[4], "00000110");
}

#[test]
fn bcd_roundtrip() {
    let input = b"\x12\x34\xAB";
    let encoded = encode_bcd(input, " ", None);
    let decoded = decode_bcd(&encoded).unwrap();
    assert_eq!(decoded, input);
}

#[test]
fn bcd_odd_tokens_error() {
    assert!(decode_bcd("0001").is_err());
}

#[test]
fn base64_roundtrip() {
    let cases: &[&[u8]] = &[b"", b"A", b"AB", b"ABC", b"hello world", b"\x00\xFF\x80"];
    for &input in cases {
        let encoded = encode_base64(input);
        let decoded = decode_base64(&encoded).unwrap();
        assert_eq!(decoded, input, "base64 roundtrip failed for {:?}", input);
    }
}

#[test]
fn base64_known() {
    assert_eq!(encode_base64(b"hello"), "aGVsbG8=");
    assert_eq!(encode_base64(b""), "");
    assert_eq!(encode_base64(b"Man"), "TWFu");
}

#[test]
fn base64_invalid_char() {
    assert!(decode_base64("aGVs!G8=").is_err());
}

#[test]
fn rle_roundtrip() {
    let cases: &[&[u8]] = &[b"\xFF", b"\x00", b"hello", b"\xAA\x55"];
    for &input in cases {
        let encoded = encode_run_length_binary(input);
        let decoded = decode_run_length_binary(&encoded).unwrap();
        assert_eq!(decoded, input, "RLE roundtrip failed for {:?}", input);
    }
}

#[test]
fn rle_known() {
    let encoded = encode_run_length_binary(b"\xFF");
    assert_eq!(encoded, "18");

    let encoded = encode_run_length_binary(b"\x00");
    assert_eq!(encoded, "08");
}
