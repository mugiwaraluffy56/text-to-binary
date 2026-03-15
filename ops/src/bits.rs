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

pub fn reverse(a: &str) -> Result<String, String> {
    let av = parse(a)?;
    Ok(to_bin(
        &av.iter().map(|&x| x.reverse_bits()).collect::<Vec<_>>(),
    ))
}

pub fn swap_bytes(a: &str) -> Result<String, String> {
    let mut av = parse(a)?;
    av.reverse();
    Ok(to_bin(&av))
}

pub fn swap_nibbles(a: &str) -> Result<String, String> {
    let av = parse(a)?;
    let result: Vec<u8> = av
        .iter()
        .map(|&x| ((x & 0x0F) << 4) | ((x & 0xF0) >> 4))
        .collect();
    Ok(to_bin(&result))
}

pub fn popcount(a: &str) -> Result<(Vec<u32>, u32), String> {
    let av = parse(a)?;
    let per_byte: Vec<u32> = av.iter().map(|&x| x.count_ones()).collect();
    let total: u32 = per_byte.iter().sum();
    Ok((per_byte, total))
}

pub fn count_leading_zeros(a: &str) -> Result<Vec<u32>, String> {
    let av = parse(a)?;
    Ok(av.iter().map(|&x| x.leading_zeros()).collect())
}

pub fn count_trailing_zeros(a: &str) -> Result<Vec<u32>, String> {
    let av = parse(a)?;
    Ok(av.iter().map(|&x| x.trailing_zeros()).collect())
}

pub fn parity(a: &str) -> Result<String, String> {
    let av = parse(a)?;
    let lines: Vec<String> = av
        .iter()
        .map(|&x| {
            let ones = x.count_ones();
            let par = if ones % 2 == 0 { "even" } else { "odd" };
            format!("0x{:02x}: {} parity ({} set bits)", x, par, ones)
        })
        .collect();
    Ok(lines.join("\n"))
}

pub fn format_popcount_output(bytes_bin: &str) -> Result<String, String> {
    let (per_byte, total) = popcount(bytes_bin)?;
    let tokens: Vec<&str> = bytes_bin.split_whitespace().collect();
    let total_bits = tokens.len() * 8;
    let pct = if total_bits > 0 {
        total as f64 / total_bits as f64 * 100.0
    } else {
        0.0
    };
    let mut lines = Vec::new();
    for (i, (tok, cnt)) in tokens.iter().zip(per_byte.iter()).enumerate() {
        lines.push(format!("{:04}: {} -> {} set bits", i, tok, cnt));
    }
    lines.push(format!(
        "total: {} set bits / {} total bits ({:.2}%)",
        total, total_bits, pct
    ));
    Ok(lines.join("\n"))
}

pub fn format_clz_output(bytes_bin: &str) -> Result<String, String> {
    let tokens: Vec<&str> = bytes_bin.split_whitespace().collect();
    let clz = count_leading_zeros(bytes_bin)?;
    let lines: Vec<String> = tokens
        .iter()
        .zip(clz.iter())
        .map(|(tok, cnt)| format!("{} -> {} leading zeros", tok, cnt))
        .collect();
    Ok(lines.join("\n"))
}

pub fn format_ctz_output(bytes_bin: &str) -> Result<String, String> {
    let tokens: Vec<&str> = bytes_bin.split_whitespace().collect();
    let ctz = count_trailing_zeros(bytes_bin)?;
    let lines: Vec<String> = tokens
        .iter()
        .zip(ctz.iter())
        .map(|(tok, cnt)| format!("{} -> {} trailing zeros", tok, cnt))
        .collect();
    Ok(lines.join("\n"))
}
