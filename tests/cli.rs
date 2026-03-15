use std::process::Command;

fn binrs(args: &[&str]) -> (String, String, bool) {
    let output = Command::new(env!("CARGO_BIN_EXE_binrs"))
        .args(args)
        .output()
        .expect("failed to run binrs");
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    (stdout, stderr, output.status.success())
}

#[test]
fn encode_hello_bin() {
    let (out, _, ok) = binrs(&["encode", "hello"]);
    assert!(ok);
    assert_eq!(
        out.trim(),
        "01101000 01100101 01101100 01101100 01101111"
    );
}

#[test]
fn encode_hello_hex() {
    let (out, _, ok) = binrs(&["encode", "hello", "--format", "hex"]);
    assert!(ok);
    assert_eq!(out.trim(), "68 65 6c 6c 6f");
}

#[test]
fn encode_hello_dec() {
    let (out, _, ok) = binrs(&["encode", "hello", "--format", "dec"]);
    assert!(ok);
    assert_eq!(out.trim(), "104 101 108 108 111");
}

#[test]
fn decode_bin_to_text() {
    let (out, _, ok) = binrs(&[
        "text",
        "--from",
        "bin",
        "01101000 01100101 01101100 01101100 01101111",
    ]);
    assert!(ok);
    assert_eq!(out.trim(), "hello");
}

#[test]
fn decode_hex_to_text() {
    let (out, _, ok) = binrs(&["text", "--from", "hex", "68 65 6c 6c 6f"]);
    assert!(ok);
    assert_eq!(out.trim(), "hello");
}

#[test]
fn xor_same_bytes() {
    let (out, _, ok) = binrs(&["xor", "01000001", "01000001"]);
    assert!(ok);
    assert_eq!(out.trim(), "00000000");
}

#[test]
fn not_zeros() {
    let (out, _, ok) = binrs(&["not", "00000000"]);
    assert!(ok);
    assert_eq!(out.trim(), "11111111");
}

#[test]
fn convert_dec_to_bin() {
    let (out, _, ok) = binrs(&["convert", "--from", "dec", "--to", "bin", "65"]);
    assert!(ok);
    assert_eq!(out.trim(), "1000001");
}

#[test]
fn convert_hex_to_dec() {
    let (out, _, ok) = binrs(&["convert", "--from", "hex", "--to", "dec", "ff"]);
    assert!(ok);
    assert_eq!(out.trim(), "255");
}

#[test]
fn crc32_hello() {
    let (out, _, ok) = binrs(&["crc", "--algo", "crc32", "hello"]);
    assert!(ok);
    assert!(out.contains("3610a686"));
}

#[test]
fn float_one() {
    let (out, _, ok) = binrs(&["float", "1.0"]);
    assert!(ok);
    assert!(out.contains("Normal"));
    assert!(out.contains("3f800000"));
}

#[test]
fn float_inf() {
    let (out, _, ok) = binrs(&["float", "inf"]);
    assert!(ok);
    assert!(out.contains("Infinity"));
}

#[test]
fn pattern_zeros() {
    let (out, _, ok) = binrs(&["pattern", "--type", "zeros", "--count", "4"]);
    assert!(ok);
    let lines: Vec<&str> = out.trim().split('\n').collect();
    assert!(lines[0].contains("00000000"));
}

#[test]
fn pattern_list() {
    let (out, _, ok) = binrs(&["pattern", "--list"]);
    assert!(ok);
    assert!(out.contains("zeros"));
    assert!(out.contains("ones"));
    assert!(out.contains("prng"));
}

#[test]
fn inspect_dump() {
    let (out, _, ok) = binrs(&["inspect", "AB"]);
    assert!(ok);
    assert!(out.contains("41"));
    assert!(out.contains("42"));
}

#[test]
fn inspect_xxd() {
    let (out, _, ok) = binrs(&["inspect", "--style", "xxd", "AB"]);
    assert!(ok);
    assert!(out.contains("4142"));
}

#[test]
fn inspect_bits_style() {
    let (out, _, ok) = binrs(&["inspect", "--style", "bits", "A"]);
    assert!(ok);
    assert!(out.contains("b7"));
    assert!(out.contains("0x41"));
}

#[test]
fn base64_roundtrip() {
    let (encoded, _, ok) = binrs(&["base64", "hello"]);
    assert!(ok);
    assert_eq!(encoded.trim(), "aGVsbG8=");

    let (decoded, _, ok2) = binrs(&["base64", "--decode", "aGVsbG8="]);
    assert!(ok2);
    assert_eq!(decoded.trim(), "hello");
}

#[test]
fn gray_roundtrip() {
    let (encoded, _, ok) = binrs(&["gray", "A"]);
    assert!(ok);
    let (decoded_out, _, ok2) = binrs(&["gray", "--decode", encoded.trim()]);
    assert!(ok2);
    assert_eq!(decoded_out.trim(), "01000001");
}

#[test]
fn popcount_output() {
    let (out, _, ok) = binrs(&["popcount", "11111111"]);
    assert!(ok);
    assert!(out.contains("8 set bits"));
}

#[test]
fn rotate_left_wraps() {
    let (out, _, ok) = binrs(&["rotate", "--left", "1", "10000000"]);
    assert!(ok);
    assert_eq!(out.trim(), "00000001");
}

#[test]
fn swap_nibbles() {
    let (out, _, ok) = binrs(&["swap", "--nibbles", "11110000"]);
    assert!(ok);
    assert_eq!(out.trim(), "00001111");
}

#[test]
fn diff_identical() {
    let (out, _, ok) = binrs(&["diff", "01000001", "01000001"]);
    assert!(ok);
    assert!(out.contains("IDENTICAL"));
}

#[test]
fn diff_different() {
    let (out, _, ok) = binrs(&["diff", "01000001", "01000010"]);
    assert!(ok);
    assert!(out.contains("DIFFERENT"));
}

#[test]
fn invalid_command_exits_nonzero() {
    let (_, _, ok) = binrs(&["notacommand"]);
    assert!(!ok);
}

#[test]
fn unknown_format_error() {
    let (_, err, ok) = binrs(&["encode", "hello", "--format", "yaml"]);
    assert!(!ok);
    assert!(err.contains("Error"));
}
