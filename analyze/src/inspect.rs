pub fn inspect(bytes: &[u8], width: usize) -> String {
    let w = width.max(1);
    let bin_col_w = w * 9 - 1;
    let hex_col_w = w * 3 - 1;

    bytes
        .chunks(w)
        .enumerate()
        .map(|(i, chunk)| {
            let offset = i * w;
            let bin_part = chunk
                .iter()
                .map(|&b| format!("{:08b}", b))
                .collect::<Vec<_>>()
                .join(" ");
            let hex_part = chunk
                .iter()
                .map(|&b| format!("{:02x}", b))
                .collect::<Vec<_>>()
                .join(" ");
            let ascii_part: String = chunk
                .iter()
                .map(|&b| {
                    if b.is_ascii_graphic() || b == b' ' {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect();
            format!(
                "{:08x}  {:<bin_col_w$}  {:<hex_col_w$}  {}",
                offset,
                bin_part,
                hex_part,
                ascii_part,
                bin_col_w = bin_col_w,
                hex_col_w = hex_col_w,
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn inspect_xxd(bytes: &[u8], width: usize) -> String {
    let w = width.max(1);
    bytes
        .chunks(w)
        .enumerate()
        .map(|(i, chunk)| {
            let offset = i * w;
            let hex_groups: Vec<String> = chunk
                .chunks(2)
                .map(|pair| {
                    pair.iter()
                        .map(|&b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join("")
                })
                .collect();
            let hex_part = hex_groups.join(" ");
            let total_hex_width = (w / 2 + w % 2) * 5 - 1;
            let ascii_part: String = chunk
                .iter()
                .map(|&b| {
                    if b >= 0x20 && b < 0x7f {
                        b as char
                    } else {
                        '.'
                    }
                })
                .collect();
            format!(
                "{:08x}: {:<width$}  {}",
                offset,
                hex_part,
                ascii_part,
                width = total_hex_width
            )
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn inspect_bits(bytes: &[u8]) -> String {
    let header = "addr  hex   dec  bin       b7 b6 b5 b4 b3 b2 b1 b0  ascii";
    let mut lines = vec![header.to_string()];
    for (i, &b) in bytes.iter().enumerate() {
        let ascii = if b.is_ascii_graphic() || b == b' ' {
            format!("'{}'", b as char)
        } else {
            "   ".to_string()
        };
        let bits: Vec<String> = (0..8)
            .rev()
            .map(|shift| format!("{}", (b >> shift) & 1))
            .collect();
        lines.push(format!(
            "{:04x}  0x{:02x}  {:3}  {:08b}  {}  {}",
            i,
            b,
            b,
            b,
            bits.join("  "),
            ascii
        ));
    }
    lines.join("\n")
}

pub fn inspect_od(bytes: &[u8], width: usize) -> String {
    let w = width.max(1);
    bytes
        .chunks(w)
        .enumerate()
        .map(|(i, chunk)| {
            let offset = i * w;
            let oct_part = chunk
                .iter()
                .map(|&b| format!("{:03o}", b))
                .collect::<Vec<_>>()
                .join(" ");
            format!("{:07o} {}", offset, oct_part)
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn inspect_with_style(bytes: &[u8], style: &str, width: usize) -> Result<String, String> {
    match style {
        "dump" => Ok(inspect(bytes, width)),
        "xxd" => Ok(inspect_xxd(bytes, width)),
        "bits" => Ok(inspect_bits(bytes)),
        "od" => Ok(inspect_od(bytes, width)),
        _ => Err(format!(
            "unknown inspect style '{}': use dump, xxd, bits, or od",
            style
        )),
    }
}

pub fn inspect_summary(bytes: &[u8]) -> String {
    let total = bytes.len();
    let printable = bytes.iter().filter(|&&b| b.is_ascii_graphic() || b == b' ').count();
    let null_count = bytes.iter().filter(|&&b| b == 0).count();
    let high_count = bytes.iter().filter(|&&b| b >= 0x80).count();
    format!(
        "total bytes: {}\nprintable:   {}\nnull bytes:  {}\nhigh bytes:  {}",
        total, printable, null_count, high_count
    )
}
