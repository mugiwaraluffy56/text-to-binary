use crate::encode::reverse_bits;
use crate::format::{Format, parse_bytes};

pub fn decode(input: &str, fmt: Format) -> Result<Vec<u8>, String> {
    parse_bytes(input, fmt)
}

pub fn decode_lsb(input: &str, fmt: Format) -> Result<Vec<u8>, String> {
    let bytes = parse_bytes(input, fmt)?;
    Ok(reverse_bits(&bytes))
}

pub fn decode_text(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| {
            if b.is_ascii_graphic() || b == b' ' || b == b'\n' || b == b'\t' {
                b as char
            } else {
                '.'
            }
        })
        .collect()
}

pub fn decode_format(input: &str, fmt: Format, lsb: bool) -> Result<Vec<u8>, String> {
    if lsb {
        decode_lsb(input, fmt)
    } else {
        decode(input, fmt)
    }
}

pub fn decode_hex_str(s: &str) -> Result<Vec<u8>, String> {
    let cleaned: String = s
        .chars()
        .filter(|c| c.is_ascii_hexdigit())
        .collect();
    if cleaned.len() % 2 != 0 {
        return Err(format!(
            "hex string has odd length {}: cannot decode to bytes",
            cleaned.len()
        ));
    }
    cleaned
        .as_bytes()
        .chunks(2)
        .map(|pair| {
            let s = std::str::from_utf8(pair).unwrap();
            u8::from_str_radix(s, 16).map_err(|_| format!("invalid hex pair '{}'", s))
        })
        .collect()
}

pub fn decode_bin_str(s: &str) -> Result<Vec<u8>, String> {
    parse_bytes(s, Format::Bin)
}

pub fn decode_oct_str(s: &str) -> Result<Vec<u8>, String> {
    parse_bytes(s, Format::Oct)
}

pub fn decode_dec_str(s: &str) -> Result<Vec<u8>, String> {
    parse_bytes(s, Format::Dec)
}

pub fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| format!("{:02x}", b)).collect::<Vec<_>>().join(" ")
}

pub fn bytes_to_bin(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| format!("{:08b}", b)).collect::<Vec<_>>().join(" ")
}

pub fn bytes_to_oct(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| format!("{:03o}", b)).collect::<Vec<_>>().join(" ")
}

pub fn bytes_to_dec(bytes: &[u8]) -> String {
    bytes.iter().map(|&b| format!("{}", b)).collect::<Vec<_>>().join(" ")
}

pub fn bytes_to_format(bytes: &[u8], fmt: Format) -> String {
    match fmt {
        Format::Bin => bytes_to_bin(bytes),
        Format::Hex => bytes_to_hex(bytes),
        Format::Oct => bytes_to_oct(bytes),
        Format::Dec => bytes_to_dec(bytes),
    }
}
