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

pub fn interleave(a: &str, b: &str) -> Result<String, String> {
    let av = parse(a)?;
    let bv = parse(b)?;
    check_len(&av, &bv)?;
    let tokens: Vec<String> = av
        .iter()
        .zip(&bv)
        .map(|(&x, &y)| {
            let mut result: u16 = 0;
            for i in 0..8u32 {
                let xa = ((x >> (7 - i)) & 1) as u16;
                let ya = ((y >> (7 - i)) & 1) as u16;
                result |= xa << (15 - i * 2);
                result |= ya << (14 - i * 2);
            }
            format!("{:016b}", result)
        })
        .collect();
    Ok(tokens.join(" "))
}

pub fn deinterleave(a: &str) -> Result<String, String> {
    let tokens: Vec<&str> = a.split_whitespace().collect();
    let mut xa_bytes: Vec<u8> = Vec::with_capacity(tokens.len());
    let mut xb_bytes: Vec<u8> = Vec::with_capacity(tokens.len());
    for tok in &tokens {
        if tok.len() != 16 {
            return Err(format!(
                "deinterleave expects 16-bit tokens, got '{}' (len {})",
                tok,
                tok.len()
            ));
        }
        let val = u16::from_str_radix(tok, 2)
            .map_err(|_| format!("invalid 16-bit binary token '{}'", tok))?;
        let mut xa = 0u8;
        let mut xb = 0u8;
        for i in 0..8u32 {
            let even_bit = (val >> (15 - i * 2)) & 1;
            let odd_bit = (val >> (14 - i * 2)) & 1;
            xa |= (even_bit as u8) << (7 - i);
            xb |= (odd_bit as u8) << (7 - i);
        }
        xa_bytes.push(xa);
        xb_bytes.push(xb);
    }
    let a_str = to_bin(&xa_bytes);
    let b_str = to_bin(&xb_bytes);
    Ok(format!("A: {}\nB: {}", a_str, b_str))
}
