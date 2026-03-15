use codec::format::Format;

fn parse_value(value: &str, from: Format) -> Result<Vec<u8>, String> {
    let s = value.trim();
    let tokens: Vec<&str> = s.split_whitespace().collect();
    tokens
        .iter()
        .map(|tok| {
            let v: u128 = match from {
                Format::Bin => u128::from_str_radix(tok.trim_start_matches("0b"), 2),
                Format::Hex => u128::from_str_radix(
                    tok.trim_start_matches("0x").trim_start_matches("0X"),
                    16,
                ),
                Format::Oct => u128::from_str_radix(tok.trim_start_matches("0o"), 8),
                Format::Dec => u128::from_str_radix(tok, 10),
            }
            .map_err(|_| format!("invalid {} token '{}'", from, tok))?;
            if v > 255 {
                return Err(format!("value {} exceeds 0xFF for byte conversion", v));
            }
            Ok(v as u8)
        })
        .collect()
}

fn parse_single_u128(value: &str, from: Format) -> Result<u128, String> {
    let s = value.trim();
    match from {
        Format::Bin => u128::from_str_radix(s.trim_start_matches("0b"), 2),
        Format::Hex => u128::from_str_radix(
            s.trim_start_matches("0x").trim_start_matches("0X"),
            16,
        ),
        Format::Oct => u128::from_str_radix(s.trim_start_matches("0o"), 8),
        Format::Dec => u128::from_str_radix(s, 10),
    }
    .map_err(|_| format!("invalid {} value: '{}'", from, s))
}

pub fn convert(value: &str, from: Format, to: Format) -> Result<String, String> {
    let n = parse_single_u128(value, from)?;
    Ok(match to {
        Format::Bin => format!("{:b}", n),
        Format::Hex => format!("{:x}", n),
        Format::Oct => format!("{:o}", n),
        Format::Dec => format!("{}", n),
    })
}

pub fn convert_all(value: &str, from: Format) -> Result<String, String> {
    let bytes = parse_value(value, from)?;
    let mut lines = Vec::new();
    lines.push(format!("input ({}):", from));
    for (i, &b) in bytes.iter().enumerate() {
        let g = b ^ (b >> 1);
        let signed = b as i8;
        let twos = (!b).wrapping_add(1);
        lines.push(format!(
            "  [{:02}] bin={:08b}  hex={:02x}  oct={:03o}  dec={:3}  gray={:08b}  signed={:4}  twos_comp={:08b}",
            i, b, b, b, b, g, signed, twos
        ));
    }
    Ok(lines.join("\n"))
}

pub fn interpret_signed(value: &str, from: Format) -> Result<String, String> {
    let bytes = parse_value(value, from)?;
    let lines: Vec<String> = bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            let signed = b as i8;
            format!("  [{:02}] 0x{:02x} ({:08b}) = {} (signed i8)", i, b, b, signed)
        })
        .collect();
    Ok(lines.join("\n"))
}

pub fn to_twos_complement(value: &str, from: Format) -> Result<String, String> {
    let bytes = parse_value(value, from)?;
    let lines: Vec<String> = bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            let twos = (!b).wrapping_add(1);
            let signed_orig = b as i8;
            format!(
                "  [{:02}] {:08b} ({:3}) -> twos_comp = {:08b} ({:3})",
                i, b, signed_orig, twos, twos as i8
            )
        })
        .collect();
    Ok(lines.join("\n"))
}

pub fn convert_gray_encode(value: &str, from: Format) -> Result<String, String> {
    let bytes = parse_value(value, from)?;
    let lines: Vec<String> = bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            let g = b ^ (b >> 1);
            format!(
                "  [{:02}] {:08b} ({:3}) -> gray = {:08b} ({:3})",
                i, b, b, g, g
            )
        })
        .collect();
    Ok(lines.join("\n"))
}

pub fn convert_gray_decode(value: &str, from: Format) -> Result<String, String> {
    let bytes = parse_value(value, from)?;
    let lines: Vec<String> = bytes
        .iter()
        .enumerate()
        .map(|(i, &g)| {
            let mut n = g;
            n ^= n >> 4;
            n ^= n >> 2;
            n ^= n >> 1;
            format!(
                "  [{:02}] gray {:08b} ({:3}) -> binary = {:08b} ({:3})",
                i, g, g, n, n
            )
        })
        .collect();
    Ok(lines.join("\n"))
}

pub fn convert_show_all_bases(value: &str, from: Format) -> Result<String, String> {
    let n = parse_single_u128(value, from)?;
    let bytes_repr: Vec<u8> = {
        let mut v = Vec::new();
        let mut tmp = n;
        if tmp == 0 {
            v.push(0u8);
        } else {
            while tmp > 0 {
                v.push((tmp & 0xFF) as u8);
                tmp >>= 8;
            }
            v.reverse();
        }
        v
    };
    let gray_bytes: Vec<String> = bytes_repr
        .iter()
        .map(|&b| format!("{:08b}", b ^ (b >> 1)))
        .collect();
    let signed_bytes: Vec<String> = bytes_repr
        .iter()
        .map(|&b| format!("{}", b as i8))
        .collect();
    Ok(format!(
        "value: {}\n  binary:      {:b}\n  hex:         {:x}\n  octal:       {:o}\n  decimal:     {}\n  gray code:   {}\n  signed(i8):  {}",
        value.trim(),
        n, n, n, n,
        gray_bytes.join(" "),
        signed_bytes.join(" ")
    ))
}

pub fn format_byte_table(bytes: &[u8]) -> String {
    let mut lines = vec!["idx  bin       hex  oct  dec  signed".to_string()];
    for (i, &b) in bytes.iter().enumerate() {
        lines.push(format!(
            "{:3}  {:08b}  {:02x}   {:03o}  {:3}  {:4}",
            i,
            b,
            b,
            b,
            b,
            b as i8
        ));
    }
    lines.join("\n")
}
