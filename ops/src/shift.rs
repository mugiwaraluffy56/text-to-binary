use codec::format::{Format, parse_bytes};

fn to_bin(bytes: &[u8]) -> String {
    bytes
        .iter()
        .map(|&b| format!("{:08b}", b))
        .collect::<Vec<_>>()
        .join(" ")
}

fn parse(s: &str) -> Result<Vec<u8>, String> {
    parse_bytes(s, Format::Bin)
}

pub fn shift(a: &str, n: u32, left: bool) -> Result<String, String> {
    let av = parse(a)?;
    let result: Vec<u8> = if left {
        av.iter().map(|&x| x.wrapping_shl(n)).collect()
    } else {
        av.iter().map(|&x| x.wrapping_shr(n)).collect()
    };
    Ok(to_bin(&result))
}

pub fn rotate(a: &str, n: u32, left: bool) -> Result<String, String> {
    let av = parse(a)?;
    let r = (n % 8) as u32;
    let result: Vec<u8> = if left {
        av.iter().map(|&x| x.rotate_left(r)).collect()
    } else {
        av.iter().map(|&x| x.rotate_right(r)).collect()
    };
    Ok(to_bin(&result))
}
