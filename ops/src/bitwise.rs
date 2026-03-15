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

fn check_len(a: &[u8], b: &[u8]) -> Result<(), String> {
    if a.len() != b.len() {
        Err(format!(
            "operand length mismatch: {} vs {} bytes",
            a.len(),
            b.len()
        ))
    } else {
        Ok(())
    }
}

pub fn xor(a: &str, b: &str) -> Result<String, String> {
    let av = parse(a)?;
    let bv = parse(b)?;
    check_len(&av, &bv)?;
    Ok(to_bin(
        &av.iter().zip(&bv).map(|(&x, &y)| x ^ y).collect::<Vec<_>>(),
    ))
}

pub fn and(a: &str, b: &str) -> Result<String, String> {
    let av = parse(a)?;
    let bv = parse(b)?;
    check_len(&av, &bv)?;
    Ok(to_bin(
        &av.iter().zip(&bv).map(|(&x, &y)| x & y).collect::<Vec<_>>(),
    ))
}

pub fn or(a: &str, b: &str) -> Result<String, String> {
    let av = parse(a)?;
    let bv = parse(b)?;
    check_len(&av, &bv)?;
    Ok(to_bin(
        &av.iter().zip(&bv).map(|(&x, &y)| x | y).collect::<Vec<_>>(),
    ))
}

pub fn not(a: &str) -> Result<String, String> {
    let av = parse(a)?;
    Ok(to_bin(&av.iter().map(|&x| !x).collect::<Vec<_>>()))
}

pub fn nand(a: &str, b: &str) -> Result<String, String> {
    let av = parse(a)?;
    let bv = parse(b)?;
    check_len(&av, &bv)?;
    Ok(to_bin(
        &av.iter()
            .zip(&bv)
            .map(|(&x, &y)| !(x & y))
            .collect::<Vec<_>>(),
    ))
}

pub fn nor(a: &str, b: &str) -> Result<String, String> {
    let av = parse(a)?;
    let bv = parse(b)?;
    check_len(&av, &bv)?;
    Ok(to_bin(
        &av.iter()
            .zip(&bv)
            .map(|(&x, &y)| !(x | y))
            .collect::<Vec<_>>(),
    ))
}

pub fn xnor(a: &str, b: &str) -> Result<String, String> {
    let av = parse(a)?;
    let bv = parse(b)?;
    check_len(&av, &bv)?;
    Ok(to_bin(
        &av.iter()
            .zip(&bv)
            .map(|(&x, &y)| !(x ^ y))
            .collect::<Vec<_>>(),
    ))
}
