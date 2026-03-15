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

pub fn bit_field_get(a: &str, start: u8, end: u8) -> Result<String, String> {
    if start > 7 || end > 7 {
        return Err(format!(
            "bit positions must be 0-7, got start={} end={}",
            start, end
        ));
    }
    if start > end {
        return Err(format!(
            "start bit {} must be <= end bit {} (0=MSB, 7=LSB)",
            start, end
        ));
    }
    let av = parse(a)?;
    let field_len = (end - start + 1) as usize;
    let lines: Vec<String> = av
        .iter()
        .map(|&x| {
            let s = format!("{:08b}", x);
            let field: String = s[start as usize..=end as usize].to_string();
            format!(
                "{:08b} bits[{}..={}] = {} ({})",
                x,
                start,
                end,
                field,
                &field[..field_len]
            )
        })
        .collect();
    Ok(lines.join("\n"))
}

pub fn bit_field_set(a: &str, start: u8, end: u8, value_bits: &str) -> Result<String, String> {
    if start > 7 || end > 7 {
        return Err(format!(
            "bit positions must be 0-7, got start={} end={}",
            start, end
        ));
    }
    if start > end {
        return Err(format!(
            "start bit {} must be <= end bit {} (0=MSB, 7=LSB)",
            start, end
        ));
    }
    let field_len = (end - start + 1) as usize;
    let vb: Vec<char> = value_bits.chars().filter(|c| *c == '0' || *c == '1').collect();
    if vb.len() != field_len {
        return Err(format!(
            "value bits length {} does not match field length {}",
            vb.len(),
            field_len
        ));
    }
    let av = parse(a)?;
    let result: Vec<u8> = av
        .iter()
        .map(|&x| {
            let mut s: Vec<char> = format!("{:08b}", x).chars().collect();
            for (i, c) in vb.iter().enumerate() {
                s[start as usize + i] = *c;
            }
            let s2: String = s.iter().collect();
            u8::from_str_radix(&s2, 2).unwrap_or(x)
        })
        .collect();
    Ok(to_bin(&result))
}
