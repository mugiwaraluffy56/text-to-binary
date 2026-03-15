use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
pub enum Format {
    Bin,
    Hex,
    Oct,
    Dec,
}

impl FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "bin" | "binary" => Ok(Format::Bin),
            "hex" | "hexadecimal" => Ok(Format::Hex),
            "oct" | "octal" => Ok(Format::Oct),
            "dec" | "decimal" => Ok(Format::Dec),
            _ => Err(format!("unknown format '{}': use bin, hex, oct, or dec", s)),
        }
    }
}

impl fmt::Display for Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Format::Bin => "binary",
            Format::Hex => "hex",
            Format::Oct => "octal",
            Format::Dec => "decimal",
        })
    }
}

pub fn format_byte(byte: u8, fmt: Format) -> String {
    match fmt {
        Format::Bin => format!("{:08b}", byte),
        Format::Hex => format!("{:02x}", byte),
        Format::Oct => format!("{:03o}", byte),
        Format::Dec => format!("{}", byte),
    }
}

pub fn parse_token(s: &str, fmt: Format) -> Result<u8, String> {
    let val = match fmt {
        Format::Bin => u8::from_str_radix(s.trim_start_matches("0b"), 2),
        Format::Hex => u8::from_str_radix(
            s.trim_start_matches("0x").trim_start_matches("0X"),
            16,
        ),
        Format::Oct => u8::from_str_radix(s.trim_start_matches("0o"), 8),
        Format::Dec => u8::from_str_radix(s, 10),
    };
    val.map_err(|_| format!("invalid {} token '{}': expected value 0-255", fmt, s))
}

pub fn parse_bytes(input: &str, fmt: Format) -> Result<Vec<u8>, String> {
    input.split_whitespace().map(|t| parse_token(t, fmt)).collect()
}
