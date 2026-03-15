pub fn format_bar(fraction: f64, width: usize) -> String {
    let filled = ((fraction * width as f64).round() as usize).min(width);
    let bar: String = std::iter::repeat('\u{2588}').take(filled).collect();
    let empty: String = std::iter::repeat(' ').take(width - filled).collect();
    format!("[{}{}]", bar, empty)
}

fn bit_position_histogram(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "  (no data)".to_string();
    }
    let mut counts = [0u64; 8];
    for &b in bytes {
        for shift in 0..8u32 {
            if (b >> (7 - shift)) & 1 == 1 {
                counts[shift as usize] += 1;
            }
        }
    }
    let total = bytes.len() as f64;
    let mut lines = Vec::new();
    for i in 0..8 {
        let frac = counts[i] as f64 / total;
        let bar = format_bar(frac, 30);
        lines.push(format!(
            "  b{} (bit {}): {:5} ({:6.2}%) {}",
            7 - i,
            i,
            counts[i],
            frac * 100.0,
            bar
        ));
    }
    lines.join("\n")
}

fn run_length_analysis(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "  (no data)".to_string();
    }
    let mut bits: Vec<u8> = Vec::with_capacity(bytes.len() * 8);
    for &b in bytes {
        for shift in (0..8).rev() {
            bits.push((b >> shift) & 1);
        }
    }
    let mut max_ones = 0usize;
    let mut max_zeros = 0usize;
    let mut cur_ones = 0usize;
    let mut cur_zeros = 0usize;
    let mut total_ones = 0u64;
    let mut total_zeros = 0u64;
    for &bit in &bits {
        if bit == 1 {
            cur_ones += 1;
            cur_zeros = 0;
            total_ones += 1;
            if cur_ones > max_ones {
                max_ones = cur_ones;
            }
        } else {
            cur_zeros += 1;
            cur_ones = 0;
            total_zeros += 1;
            if cur_zeros > max_zeros {
                max_zeros = cur_zeros;
            }
        }
    }
    format!(
        "  total set bits:   {}\n  total clear bits: {}\n  longest run of 1s: {}\n  longest run of 0s: {}",
        total_ones, total_zeros, max_ones, max_zeros
    )
}

fn byte_pair_frequency(bytes: &[u8], top_n: usize) -> String {
    if bytes.len() < 2 {
        return "  (not enough data)".to_string();
    }
    let mut pair_freq = std::collections::HashMap::new();
    for window in bytes.windows(2) {
        let key = (window[0], window[1]);
        *pair_freq.entry(key).or_insert(0usize) += 1;
    }
    let mut pairs: Vec<((u8, u8), usize)> = pair_freq.into_iter().collect();
    pairs.sort_by(|a, b| b.1.cmp(&a.1));
    let total_pairs = bytes.len() - 1;
    let show = top_n.min(pairs.len());
    if show == 0 {
        return "  (no pairs)".to_string();
    }
    let lines: Vec<String> = pairs
        .iter()
        .take(show)
        .map(|&((a, b), count)| {
            format!(
                "  0x{:02x} 0x{:02x}: {} ({:.2}%)",
                a,
                b,
                count,
                count as f64 / total_pairs as f64 * 100.0
            )
        })
        .collect();
    lines.join("\n")
}

fn chi_square(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "  (no data)".to_string();
    }
    let mut freq = [0usize; 256];
    for &b in bytes {
        freq[b as usize] += 1;
    }
    let expected = bytes.len() as f64 / 256.0;
    let chi: f64 = freq
        .iter()
        .map(|&c| {
            let diff = c as f64 - expected;
            diff * diff / expected
        })
        .sum();
    let df = 255.0f64;
    let interpretation = if chi < df * 0.5 {
        "very low (possible pattern or repeated data)"
    } else if chi < df * 0.9 {
        "below expected (slightly non-uniform)"
    } else if chi < df * 1.1 {
        "near expected (approximately uniform)"
    } else if chi < df * 2.0 {
        "above expected (non-uniform distribution)"
    } else {
        "very high (strong non-uniformity or structured data)"
    };
    format!(
        "  chi-square stat: {:.2} (df=255)\n  expected ~255 for uniform distribution\n  interpretation: {}",
        chi, interpretation
    )
}

fn printable_stats(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "  (no data)".to_string();
    }
    let total = bytes.len();
    let printable = bytes
        .iter()
        .filter(|&&b| b.is_ascii_graphic() || b == b' ')
        .count();
    let null_count = bytes.iter().filter(|&&b| b == 0).count();
    let high_count = bytes.iter().filter(|&&b| b >= 0x80).count();
    let printable_ratio = printable as f64 / total as f64;
    format!(
        "  printable ASCII: {} ({:.2}%)\n  null bytes:      {} ({:.2}%)\n  high bytes (>=80): {} ({:.2}%)",
        printable,
        printable_ratio * 100.0,
        null_count,
        null_count as f64 / total as f64 * 100.0,
        high_count,
        high_count as f64 / total as f64 * 100.0
    )
}

pub fn compute_and_format(bytes: &[u8], top_n: usize) -> String {
    if bytes.is_empty() {
        return "no input".to_string();
    }

    let mut freq = [0usize; 256];
    let mut set_bits: u64 = 0;
    for &b in bytes {
        freq[b as usize] += 1;
        set_bits += b.count_ones() as u64;
    }

    let total_bytes = bytes.len();
    let total_bits = total_bytes * 8;
    let clear_bits = total_bits as u64 - set_bits;
    let hamming_pct = set_bits as f64 / total_bits as f64 * 100.0;

    let n = total_bytes as f64;
    let entropy: f64 = freq
        .iter()
        .filter(|&&c| c > 0)
        .map(|&c| {
            let p = c as f64 / n;
            -p * p.log2()
        })
        .sum();

    let unique_bytes = freq.iter().filter(|&&c| c > 0).count();

    let mut sorted: Vec<(u8, usize)> = freq
        .iter()
        .enumerate()
        .filter(|&(_, &c)| c > 0)
        .map(|(b, &c)| (b as u8, c))
        .collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));

    let mut out = format!(
        "bytes        : {}\nbits         : {}\nset bits     : {} ({:.2}%)\nclear bits   : {} ({:.2}%)\nhamming wt   : {}/{}\nentropy      : {:.4} bits/byte\nunique bytes : {}\n",
        total_bytes,
        total_bits,
        set_bits,
        hamming_pct,
        clear_bits,
        100.0 - hamming_pct,
        set_bits,
        total_bits,
        entropy,
        unique_bytes,
    );

    let show = top_n.min(sorted.len());
    if show > 0 {
        out.push_str(&format!("\ntop {} bytes:\n", show));
        for &(byte, count) in sorted.iter().take(show) {
            let glyph = if byte.is_ascii_graphic() || byte == b' ' {
                format!("'{}'", byte as char)
            } else {
                "   ".to_string()
            };
            out.push_str(&format!(
                "  0x{:02x} {} : {} ({:.2}%)\n",
                byte,
                glyph,
                count,
                count as f64 / total_bytes as f64 * 100.0,
            ));
        }
    }

    out.push_str("\nbit-position histogram (0=MSB):\n");
    out.push_str(&bit_position_histogram(bytes));
    out.push('\n');

    out.push_str("\nrun-length analysis:\n");
    out.push_str(&run_length_analysis(bytes));
    out.push('\n');

    out.push_str("\nbyte-pair frequency (top 5):\n");
    out.push_str(&byte_pair_frequency(bytes, 5));
    out.push('\n');

    out.push_str("\nchi-square test:\n");
    out.push_str(&chi_square(bytes));
    out.push('\n');

    out.push_str("\nprintable / special byte counts:\n");
    out.push_str(&printable_stats(bytes));

    out.trim_end().to_string()
}
