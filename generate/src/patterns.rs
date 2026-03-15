pub fn generate(pattern_type: &str, count: usize, seed: u64) -> Result<Vec<u8>, String> {
    if count == 0 {
        return Ok(Vec::new());
    }
    match pattern_type {
        "zeros" => Ok(vec![0x00u8; count]),
        "ones" => Ok(vec![0xFFu8; count]),
        "alt" => Ok(vec![0xAAu8; count]),
        "alt2" => Ok(vec![0x55u8; count]),
        "walk" | "walk-high" => {
            let mut out = Vec::with_capacity(count);
            let mut bit: u8 = 0x80;
            for _ in 0..count {
                out.push(bit);
                bit = if bit == 0x01 { 0x80 } else { bit >> 1 };
            }
            Ok(out)
        }
        "walk-low" => {
            let mut out = Vec::with_capacity(count);
            let mut bit: u8 = 0x01;
            for _ in 0..count {
                out.push(bit);
                bit = if bit == 0x80 { 0x01 } else { bit << 1 };
            }
            Ok(out)
        }
        "inv-walk" => {
            let mut out = Vec::with_capacity(count);
            let mut bit: u8 = 0x80;
            for _ in 0..count {
                out.push(!bit);
                bit = if bit == 0x01 { 0x80 } else { bit >> 1 };
            }
            Ok(out)
        }
        "inc" => {
            let out: Vec<u8> = (0..count).map(|i| (i % 256) as u8).collect();
            Ok(out)
        }
        "dec" => {
            let out: Vec<u8> = (0..count)
                .map(|i| (255u32.wrapping_sub(i as u32 % 256)) as u8)
                .collect();
            Ok(out)
        }
        "prng" => {
            let mut state: u64 = seed;
            let mut out = Vec::with_capacity(count);
            for _ in 0..count {
                state = state
                    .wrapping_mul(6364136223846793005u64)
                    .wrapping_add(1442695040888963407u64);
                let val = ((state >> 33) ^ state) as u8;
                out.push(val);
            }
            Ok(out)
        }
        "checkerboard" => {
            let out: Vec<u8> = (0..count)
                .map(|i| if i % 2 == 0 { 0xAA } else { 0x55 })
                .collect();
            Ok(out)
        }
        "nibble" => {
            let out: Vec<u8> = (0..count)
                .map(|i| if i % 2 == 0 { 0xF0 } else { 0x0F })
                .collect();
            Ok(out)
        }
        _ => Err(format!(
            "unknown pattern type '{}': use zeros, ones, alt, alt2, walk, walk-high, walk-low, inv-walk, inc, dec, prng, checkerboard, nibble",
            pattern_type
        )),
    }
}

pub fn display(bytes: &[u8], sep: &str, width: Option<usize>) -> String {
    let tokens: Vec<String> = bytes.iter().map(|&b| format!("{:08b}", b)).collect();
    match width {
        Some(w) if w > 0 => tokens
            .chunks(w)
            .map(|c| c.join(sep))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => tokens.join(sep),
    }
}

pub fn display_hex(bytes: &[u8], sep: &str, width: Option<usize>) -> String {
    let tokens: Vec<String> = bytes.iter().map(|&b| format!("{:02x}", b)).collect();
    match width {
        Some(w) if w > 0 => tokens
            .chunks(w)
            .map(|c| c.join(sep))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => tokens.join(sep),
    }
}

pub fn display_dec(bytes: &[u8], sep: &str, width: Option<usize>) -> String {
    let tokens: Vec<String> = bytes.iter().map(|&b| format!("{}", b)).collect();
    match width {
        Some(w) if w > 0 => tokens
            .chunks(w)
            .map(|c| c.join(sep))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => tokens.join(sep),
    }
}

pub fn list_patterns() -> &'static str {
    "zeros, ones, alt, alt2, walk, walk-high, walk-low, inv-walk, inc, dec, prng, checkerboard, nibble"
}
