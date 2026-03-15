use codec::format::{Format, parse_bytes};

fn parse_input(s: &str, fmt: Format) -> Result<Vec<u8>, String> {
    parse_bytes(s, fmt)
}

pub fn diff_binary(a: &str, b: &str, fmt: Format) -> Result<String, String> {
    let av = parse_input(a, fmt)?;
    let bv = parse_input(b, fmt)?;

    let max_len = av.len().max(bv.len());
    let mut lines = Vec::new();
    let mut diff_bytes = 0usize;
    let mut diff_bits = 0usize;
    let mut same_bytes = 0usize;

    for i in 0..max_len {
        match (av.get(i), bv.get(i)) {
            (Some(&xa), Some(&xb)) => {
                let bin_a = format!("{:08b}", xa);
                let bin_b = format!("{:08b}", xb);
                if xa == xb {
                    same_bytes += 1;
                    lines.push(format!("byte {:04x}: {} == {}", i, bin_a, bin_b));
                } else {
                    diff_bytes += 1;
                    let xor = xa ^ xb;
                    let bit_diffs = xor.count_ones() as usize;
                    diff_bits += bit_diffs;
                    let marker: String = (0..8)
                        .map(|bit| {
                            if (xor >> (7 - bit)) & 1 == 1 {
                                '^'
                            } else {
                                ' '
                            }
                        })
                        .collect();
                    lines.push(format!("byte {:04x}: {} != {}", i, bin_a, bin_b));
                    lines.push(format!("           {}    {}", marker, marker));
                    lines.push(format!(
                        "           xor={:08b} ({} bit{} differ)",
                        xor,
                        bit_diffs,
                        if bit_diffs == 1 { "" } else { "s" }
                    ));
                }
            }
            (Some(&xa), None) => {
                diff_bytes += 1;
                diff_bits += 8;
                lines.push(format!(
                    "byte {:04x}: {:08b} only in A",
                    i, xa
                ));
            }
            (None, Some(&xb)) => {
                diff_bytes += 1;
                diff_bits += 8;
                lines.push(format!(
                    "byte {:04x}: {:08b} only in B",
                    i, xb
                ));
            }
            (None, None) => {}
        }
    }

    let total_bits = max_len * 8;
    let similarity = if total_bits > 0 {
        (total_bits - diff_bits) as f64 / total_bits as f64 * 100.0
    } else {
        100.0
    };

    lines.push(String::new());
    lines.push(format!("summary:"));
    lines.push(format!("  A length: {} bytes", av.len()));
    lines.push(format!("  B length: {} bytes", bv.len()));
    lines.push(format!("  same bytes:  {}", same_bytes));
    lines.push(format!("  diff bytes:  {}", diff_bytes));
    lines.push(format!("  diff bits:   {} / {} total bits", diff_bits, total_bits));
    lines.push(format!("  similarity:  {:.2}%", similarity));

    if diff_bytes == 0 {
        lines.push("  result: IDENTICAL".to_string());
    } else {
        lines.push("  result: DIFFERENT".to_string());
    }

    Ok(lines.join("\n"))
}

pub fn diff_bytes_raw(a: &[u8], b: &[u8]) -> String {
    let max_len = a.len().max(b.len());
    let mut lines = Vec::new();
    let mut diff_bytes = 0usize;
    let mut diff_bits = 0usize;

    for i in 0..max_len {
        match (a.get(i), b.get(i)) {
            (Some(&xa), Some(&xb)) => {
                if xa != xb {
                    diff_bytes += 1;
                    let xor = xa ^ xb;
                    diff_bits += xor.count_ones() as usize;
                    let marker: String = (0..8)
                        .map(|bit| {
                            if (xor >> (7 - bit)) & 1 == 1 { '^' } else { ' ' }
                        })
                        .collect();
                    lines.push(format!("byte {:04x}: {:08b} != {:08b}", i, xa, xb));
                    lines.push(format!("           {}    {}", marker, marker));
                }
            }
            (Some(&xa), None) => {
                diff_bytes += 1;
                diff_bits += 8;
                lines.push(format!("byte {:04x}: {:08b} only in A", i, xa));
            }
            (None, Some(&xb)) => {
                diff_bytes += 1;
                diff_bits += 8;
                lines.push(format!("byte {:04x}: {:08b} only in B", i, xb));
            }
            (None, None) => {}
        }
    }

    if lines.is_empty() {
        lines.push("no differences found (sequences are identical)".to_string());
    } else {
        lines.push(format!(
            "\n{} byte(s) differ, {} bit(s) differ",
            diff_bytes, diff_bits
        ));
    }

    lines.join("\n")
}
