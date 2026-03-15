fn format_f32(bits: u32) -> String {
    let sign_bit = (bits >> 31) & 1;
    let exp_raw = ((bits >> 23) & 0xFF) as u32;
    let mantissa = bits & 0x7FFFFF;

    let sign_str = if sign_bit == 0 { "+" } else { "-" };
    let exp_biased = exp_raw as i32 - 127;

    let category = if exp_raw == 0xFF {
        if mantissa == 0 {
            if sign_bit == 0 { "+Infinity" } else { "-Infinity" }
        } else if (mantissa >> 22) & 1 == 1 {
            "qNaN"
        } else {
            "sNaN"
        }
    } else if exp_raw == 0 {
        if mantissa == 0 {
            if sign_bit == 0 { "+Zero" } else { "-Zero" }
        } else {
            "Subnormal"
        }
    } else {
        "Normal"
    };

    let value_repr = match category {
        "Normal" => {
            let mantissa_str: String = (0..23)
                .rev()
                .map(|i| char::from_digit((mantissa >> i) & 1, 10).unwrap())
                .collect();
            format!(
                "{}1.{} x 2^{}",
                sign_str, mantissa_str, exp_biased
            )
        }
        "Subnormal" => {
            let mantissa_str: String = (0..23)
                .rev()
                .map(|i| char::from_digit((mantissa >> i) & 1, 10).unwrap())
                .collect();
            format!("{}0.{} x 2^-126 (subnormal)", sign_str, mantissa_str)
        }
        _ => category.to_string(),
    };

    let full_binary: String = (0..32)
        .rev()
        .map(|i| char::from_digit((bits >> i) & 1, 10).unwrap())
        .collect();

    let mantissa_str: String = (0..23)
        .rev()
        .map(|i| char::from_digit((mantissa >> i) & 1, 10).unwrap())
        .collect();

    let exp_str: String = (0..8)
        .rev()
        .map(|i| char::from_digit((exp_raw >> i) & 1, 10).unwrap())
        .collect();

    format!(
        "f32 breakdown:\n  hex:      0x{:08x}\n  binary:   {}\n  sign:     bit[31] = {} ({})\n  exponent: bits[30:23] = {} (raw={}, bias=127, adjusted={})\n  mantissa: bits[22:0] = {}\n  category: {}\n  value:    {}",
        bits,
        full_binary,
        sign_bit,
        sign_str,
        exp_str,
        exp_raw,
        exp_biased,
        mantissa_str,
        category,
        value_repr
    )
}

fn format_f64(bits: u64) -> String {
    let sign_bit = (bits >> 63) & 1;
    let exp_raw = ((bits >> 52) & 0x7FF) as u32;
    let mantissa = bits & 0x000FFFFFFFFFFFFF;

    let sign_str = if sign_bit == 0 { "+" } else { "-" };
    let exp_biased = exp_raw as i64 - 1023;

    let category = if exp_raw == 0x7FF {
        if mantissa == 0 {
            if sign_bit == 0 { "+Infinity" } else { "-Infinity" }
        } else if (mantissa >> 51) & 1 == 1 {
            "qNaN"
        } else {
            "sNaN"
        }
    } else if exp_raw == 0 {
        if mantissa == 0 {
            if sign_bit == 0 { "+Zero" } else { "-Zero" }
        } else {
            "Subnormal"
        }
    } else {
        "Normal"
    };

    let value_repr = match category {
        "Normal" => {
            let mantissa_str: String = (0..52)
                .rev()
                .map(|i| char::from_digit(((mantissa >> i) & 1) as u32, 10).unwrap())
                .collect();
            format!("{}1.{} x 2^{}", sign_str, mantissa_str, exp_biased)
        }
        "Subnormal" => {
            let mantissa_str: String = (0..52)
                .rev()
                .map(|i| char::from_digit(((mantissa >> i) & 1) as u32, 10).unwrap())
                .collect();
            format!("{}0.{} x 2^-1022 (subnormal)", sign_str, mantissa_str)
        }
        _ => category.to_string(),
    };

    let full_binary: String = (0..64)
        .rev()
        .map(|i| char::from_digit(((bits >> i) & 1) as u32, 10).unwrap())
        .collect();

    let mantissa_str: String = (0..52)
        .rev()
        .map(|i| char::from_digit(((mantissa >> i) & 1) as u32, 10).unwrap())
        .collect();

    let exp_str: String = (0..11)
        .rev()
        .map(|i| char::from_digit((exp_raw >> i) & 1, 10).unwrap())
        .collect();

    format!(
        "f64 breakdown:\n  hex:      0x{:016x}\n  binary:   {}\n  sign:     bit[63] = {} ({})\n  exponent: bits[62:52] = {} (raw={}, bias=1023, adjusted={})\n  mantissa: bits[51:0] = {}\n  category: {}\n  value:    {}",
        bits,
        full_binary,
        sign_bit,
        sign_str,
        exp_str,
        exp_raw,
        exp_biased,
        mantissa_str,
        category,
        value_repr
    )
}

pub fn inspect_f32_value(s: &str) -> Result<String, String> {
    let v: f32 = s
        .trim()
        .parse()
        .map_err(|_| format!("cannot parse '{}' as f32", s))?;
    let bits = v.to_bits();
    Ok(format_f32(bits))
}

pub fn inspect_f64_value(s: &str) -> Result<String, String> {
    let v: f64 = s
        .trim()
        .parse()
        .map_err(|_| format!("cannot parse '{}' as f64", s))?;
    let bits = v.to_bits();
    Ok(format_f64(bits))
}

pub fn inspect_f32_hex(s: &str) -> Result<String, String> {
    let cleaned = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    let bits = u32::from_str_radix(cleaned, 16)
        .map_err(|_| format!("cannot parse '{}' as 32-bit hex", s))?;
    Ok(format_f32(bits))
}

pub fn inspect_f64_hex(s: &str) -> Result<String, String> {
    let cleaned = s.trim().trim_start_matches("0x").trim_start_matches("0X");
    let bits = u64::from_str_radix(cleaned, 16)
        .map_err(|_| format!("cannot parse '{}' as 64-bit hex", s))?;
    Ok(format_f64(bits))
}

pub fn inspect_float(value: &str, is_f64: bool, bits_mode: bool) -> Result<String, String> {
    if bits_mode {
        if is_f64 {
            inspect_f64_hex(value)
        } else {
            inspect_f32_hex(value)
        }
    } else if is_f64 {
        inspect_f64_value(value)
    } else {
        inspect_f32_value(value)
    }
}

pub fn float_special_values_f32() -> String {
    let cases: &[(&str, u32)] = &[
        ("+Zero", 0x00000000),
        ("-Zero", 0x80000000),
        ("+Inf", 0x7F800000),
        ("-Inf", 0xFF800000),
        ("qNaN", 0x7FC00000),
        ("sNaN", 0x7F800001),
        ("1.0", 0x3F800000),
        ("-1.0", 0xBF800000),
        ("Max f32", 0x7F7FFFFF),
        ("Min subnorm", 0x00000001),
    ];
    let lines: Vec<String> = cases
        .iter()
        .map(|(name, bits)| {
            let f = f32::from_bits(*bits);
            format!("  {:12} 0x{:08x}  {}", name, bits, f)
        })
        .collect();
    format!("f32 special values:\n{}", lines.join("\n"))
}

pub fn float_special_values_f64() -> String {
    let cases: &[(&str, u64)] = &[
        ("+Zero", 0x0000000000000000),
        ("-Zero", 0x8000000000000000),
        ("+Inf", 0x7FF0000000000000),
        ("-Inf", 0xFFF0000000000000),
        ("qNaN", 0x7FF8000000000000),
        ("1.0", 0x3FF0000000000000),
        ("-1.0", 0xBFF0000000000000),
    ];
    let lines: Vec<String> = cases
        .iter()
        .map(|(name, bits)| {
            let f = f64::from_bits(*bits);
            format!("  {:12} 0x{:016x}  {}", name, bits, f)
        })
        .collect();
    format!("f64 special values:\n{}", lines.join("\n"))
}
