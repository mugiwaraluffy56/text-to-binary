use crate::format::{Format, format_byte, parse_bytes};

pub struct EncodeOpts {
    pub format: Format,
    pub sep: String,
    pub width: Option<usize>,
    pub lsb_first: bool,
    pub nibble_groups: bool,
}

impl Default for EncodeOpts {
    fn default() -> Self {
        EncodeOpts {
            format: Format::Bin,
            sep: " ".to_string(),
            width: None,
            lsb_first: false,
            nibble_groups: false,
        }
    }
}

fn layout_tokens(tokens: &[String], sep: &str, width: Option<usize>) -> String {
    match width {
        Some(w) if w > 0 => tokens
            .chunks(w)
            .map(|c| c.join(sep))
            .collect::<Vec<_>>()
            .join("\n"),
        _ => tokens.join(sep),
    }
}

pub fn encode(input: &[u8], opts: &EncodeOpts) -> String {
    let tokens: Vec<String> = input
        .iter()
        .map(|&b| {
            let byte = if opts.lsb_first { b.reverse_bits() } else { b };
            if opts.nibble_groups && opts.format == Format::Bin {
                let s = format!("{:08b}", byte);
                format!("{}.{}", &s[..4], &s[4..])
            } else {
                format_byte(byte, opts.format)
            }
        })
        .collect();
    layout_tokens(&tokens, &opts.sep, opts.width)
}

pub fn encode_gray(input: &[u8], sep: &str, width: Option<usize>) -> String {
    let tokens: Vec<String> = input
        .iter()
        .map(|&b| {
            let g = b ^ (b >> 1);
            format!("{:08b}", g)
        })
        .collect();
    layout_tokens(&tokens, sep, width)
}

pub fn decode_gray(input: &str) -> Result<Vec<u8>, String> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut result = Vec::with_capacity(tokens.len());
    for tok in tokens {
        let g = u8::from_str_radix(tok, 2)
            .map_err(|_| format!("invalid binary token '{}' in gray decode", tok))?;
        let mut n = g;
        n ^= n >> 4;
        n ^= n >> 2;
        n ^= n >> 1;
        result.push(n);
    }
    Ok(result)
}

pub fn encode_bcd(input: &[u8], sep: &str, width: Option<usize>) -> String {
    let tokens: Vec<String> = input
        .iter()
        .map(|&b| {
            let hi = (b >> 4) & 0x0F;
            let lo = b & 0x0F;
            format!("{:04b} {:04b}", hi, lo)
        })
        .collect();
    layout_tokens(&tokens, sep, width)
}

pub fn decode_bcd(input: &str) -> Result<Vec<u8>, String> {
    let tokens: Vec<&str> = input.split_whitespace().collect();
    if tokens.len() % 2 != 0 {
        return Err(format!(
            "BCD decode requires even number of 4-bit tokens, got {}",
            tokens.len()
        ));
    }
    let mut result = Vec::with_capacity(tokens.len() / 2);
    for pair in tokens.chunks(2) {
        let hi = u8::from_str_radix(pair[0], 2)
            .map_err(|_| format!("invalid 4-bit token '{}' in BCD decode", pair[0]))?;
        let lo = u8::from_str_radix(pair[1], 2)
            .map_err(|_| format!("invalid 4-bit token '{}' in BCD decode", pair[1]))?;
        if hi > 15 || lo > 15 {
            return Err(format!("BCD nibble out of range: {:04b} {:04b}", hi, lo));
        }
        result.push((hi << 4) | lo);
    }
    Ok(result)
}

pub const B64_ALPHABET: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode_base64(input: &[u8]) -> String {
    let mut out = String::with_capacity((input.len() + 2) / 3 * 4);
    for chunk in input.chunks(3) {
        let b0 = chunk[0];
        let b1 = if chunk.len() > 1 { chunk[1] } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] } else { 0 };
        let combined = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
        out.push(B64_ALPHABET[((combined >> 18) & 0x3F) as usize] as char);
        out.push(B64_ALPHABET[((combined >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            out.push(B64_ALPHABET[((combined >> 6) & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(B64_ALPHABET[(combined & 0x3F) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

pub fn decode_base64(input: &str) -> Result<Vec<u8>, String> {
    let mut lut = [0xFFu8; 256];
    for (i, &c) in B64_ALPHABET.iter().enumerate() {
        lut[c as usize] = i as u8;
    }
    lut[b'=' as usize] = 0;

    let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
    if cleaned.len() % 4 != 0 {
        return Err(format!(
            "base64 input length {} is not a multiple of 4",
            cleaned.len()
        ));
    }

    let mut out = Vec::with_capacity(cleaned.len() / 4 * 3);
    for chunk in cleaned.as_bytes().chunks(4) {
        let c0 = chunk[0];
        let c1 = chunk[1];
        let c2 = chunk[2];
        let c3 = chunk[3];
        if lut[c0 as usize] == 0xFF
            || lut[c1 as usize] == 0xFF
            || lut[c2 as usize] == 0xFF
            || lut[c3 as usize] == 0xFF
        {
            return Err(format!(
                "invalid base64 character in chunk '{}'",
                std::str::from_utf8(chunk).unwrap_or("?")
            ));
        }
        let combined = ((lut[c0 as usize] as u32) << 18)
            | ((lut[c1 as usize] as u32) << 12)
            | ((lut[c2 as usize] as u32) << 6)
            | (lut[c3 as usize] as u32);
        out.push(((combined >> 16) & 0xFF) as u8);
        if c2 != b'=' {
            out.push(((combined >> 8) & 0xFF) as u8);
        }
        if c3 != b'=' {
            out.push((combined & 0xFF) as u8);
        }
    }
    Ok(out)
}

pub fn encode_run_length_binary(input: &[u8]) -> String {
    if input.is_empty() {
        return String::new();
    }
    let mut bits: Vec<u8> = Vec::with_capacity(input.len() * 8);
    for &b in input {
        for shift in (0..8).rev() {
            bits.push((b >> shift) & 1);
        }
    }
    let mut runs: Vec<(u8, usize)> = Vec::new();
    let mut current_bit = bits[0];
    let mut count = 1usize;
    for &bit in bits.iter().skip(1) {
        if bit == current_bit {
            count += 1;
        } else {
            runs.push((current_bit, count));
            current_bit = bit;
            count = 1;
        }
    }
    runs.push((current_bit, count));
    runs.iter()
        .map(|(bit, cnt)| format!("{}{}", bit, cnt))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn decode_run_length_binary(input: &str) -> Result<Vec<u8>, String> {
    if input.trim().is_empty() {
        return Ok(Vec::new());
    }
    let mut bits: Vec<u8> = Vec::new();
    for tok in input.split_whitespace() {
        if tok.len() < 2 {
            return Err(format!("invalid RLE token '{}': too short", tok));
        }
        let (bit_char, count_str) = tok.split_at(1);
        let bit: u8 = match bit_char {
            "0" => 0,
            "1" => 1,
            _ => return Err(format!("invalid RLE token '{}': bit must be 0 or 1", tok)),
        };
        let count: usize = count_str
            .parse()
            .map_err(|_| format!("invalid RLE token '{}': count parse failed", tok))?;
        for _ in 0..count {
            bits.push(bit);
        }
    }
    if bits.len() % 8 != 0 {
        return Err(format!(
            "RLE decoded to {} bits which is not a multiple of 8",
            bits.len()
        ));
    }
    let mut result = Vec::with_capacity(bits.len() / 8);
    for chunk in bits.chunks(8) {
        let mut byte = 0u8;
        for (i, &bit) in chunk.iter().enumerate() {
            byte |= bit << (7 - i);
        }
        result.push(byte);
    }
    Ok(result)
}

pub fn reverse_bits(bytes: &[u8]) -> Vec<u8> {
    bytes.iter().map(|&b| b.reverse_bits()).collect()
}

pub fn encode_with_lsb(input: &[u8], opts: &EncodeOpts) -> String {
    let reversed: Vec<u8> = reverse_bits(input);
    let adjusted_opts = EncodeOpts {
        format: opts.format,
        sep: opts.sep.clone(),
        width: opts.width,
        lsb_first: false,
        nibble_groups: opts.nibble_groups,
    };
    encode(&reversed, &adjusted_opts)
}

pub fn parse_any_bytes(input: &str, fmt: Format) -> Result<Vec<u8>, String> {
    parse_bytes(input, fmt)
}
