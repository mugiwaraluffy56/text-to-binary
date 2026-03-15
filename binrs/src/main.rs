use clap::{Parser, Subcommand};
use codec::encode::{encode, EncodeOpts};
use codec::format::Format;
use std::path::PathBuf;
use std::str::FromStr;

#[derive(Parser)]
#[command(name = "binrs", about = "Binary encoder / decoder / analyzer", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Encode {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(long, value_name = "FORMAT", default_value = "bin")]
        format: String,
        #[arg(long, value_name = "CHAR", default_value = " ")]
        sep: String,
        #[arg(long, value_name = "N")]
        width: Option<usize>,
        #[arg(long)]
        lsb: bool,
        #[arg(long)]
        nibbles: bool,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Decode {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(long, value_name = "FORMAT", default_value = "bin")]
        format: String,
        #[arg(long)]
        lsb: bool,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Inspect {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(short = 'w', long, value_name = "N", default_value = "8")]
        width: usize,
        #[arg(long, default_value = "dump")]
        style: String,
        #[arg(long)]
        summary: bool,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Xor {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: Option<PathBuf>,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: Option<PathBuf>,
        #[arg(index = 1, value_name = "A")]
        a: Option<String>,
        #[arg(index = 2, value_name = "B")]
        b: Option<String>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    And {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: Option<PathBuf>,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: Option<PathBuf>,
        #[arg(index = 1, value_name = "A")]
        a: Option<String>,
        #[arg(index = 2, value_name = "B")]
        b: Option<String>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    Or {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: Option<PathBuf>,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: Option<PathBuf>,
        #[arg(index = 1, value_name = "A")]
        a: Option<String>,
        #[arg(index = 2, value_name = "B")]
        b: Option<String>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    Not {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Shift {
        #[arg(long, value_name = "N", conflicts_with = "right")]
        left: Option<u32>,
        #[arg(long, value_name = "N", conflicts_with = "left")]
        right: Option<u32>,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Reverse {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Stats {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(long, value_name = "N", default_value = "10")]
        top: usize,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Convert {
        #[arg(long, value_name = "FORMAT", default_value = "dec")]
        from: String,
        #[arg(long, value_name = "FORMAT", default_value = "bin")]
        to: String,
        value: String,
    },
    Nand {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: Option<PathBuf>,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: Option<PathBuf>,
        #[arg(index = 1, value_name = "A")]
        a: Option<String>,
        #[arg(index = 2, value_name = "B")]
        b: Option<String>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    Nor {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: Option<PathBuf>,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: Option<PathBuf>,
        #[arg(index = 1, value_name = "A")]
        a: Option<String>,
        #[arg(index = 2, value_name = "B")]
        b: Option<String>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    Xnor {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: Option<PathBuf>,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: Option<PathBuf>,
        #[arg(index = 1, value_name = "A")]
        a: Option<String>,
        #[arg(index = 2, value_name = "B")]
        b: Option<String>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    Rotate {
        #[arg(long, value_name = "N", conflicts_with = "right")]
        left: Option<u32>,
        #[arg(long, value_name = "N", conflicts_with = "left")]
        right: Option<u32>,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Swap {
        #[arg(long)]
        bytes: bool,
        #[arg(long)]
        nibbles: bool,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Popcount {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Clz {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Ctz {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Parity {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Field {
        #[arg(long)]
        get: Option<String>,
        #[arg(long = "set")]
        set_range: Option<String>,
        #[arg(long)]
        value: Option<String>,
        #[arg(short = 'f', long = "file", value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Interleave {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: Option<PathBuf>,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: Option<PathBuf>,
        #[arg(index = 1, value_name = "A")]
        a: Option<String>,
        #[arg(index = 2, value_name = "B")]
        b: Option<String>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    Deinterleave {
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Gray {
        #[arg(long)]
        decode: bool,
        #[arg(short = 'f', long = "file", value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Bcd {
        #[arg(long)]
        decode: bool,
        #[arg(short = 'f', long = "file", value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Base64 {
        #[arg(long)]
        decode: bool,
        #[arg(short = 'f', long = "file", value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Rle {
        #[arg(long)]
        decode: bool,
        #[arg(short = 'f', long = "file", value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Crc {
        #[arg(long, default_value = "crc32")]
        algo: String,
        #[arg(short = 'f', long = "file", value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Diff {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: Option<PathBuf>,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: Option<PathBuf>,
        #[arg(long, default_value = "bin")]
        format: String,
        #[arg(index = 1, value_name = "A")]
        a: Option<String>,
        #[arg(index = 2, value_name = "B")]
        b: Option<String>,
    },
    Pattern {
        #[arg(long = "type", default_value = "zeros")]
        r#type: String,
        #[arg(long, default_value = "8")]
        count: usize,
        #[arg(long, default_value = "0")]
        seed: u64,
        #[arg(long, default_value = "bin", help = "bin|hex|dec")]
        format: String,
        #[arg(long)]
        list: bool,
        #[arg(short = 'o', long, value_name = "FILE")]
        output: Option<PathBuf>,
    },
    Float {
        #[arg(long = "f64")]
        is_f64: bool,
        #[arg(long)]
        bits: bool,
        value: String,
    },
    Specials {
        #[arg(long = "f64")]
        is_f64: bool,
    },
    All {
        #[arg(long, default_value = "dec")]
        from: String,
        value: String,
    },
    Signed {
        #[arg(long, default_value = "bin", help = "bin|hex|oct|dec")]
        from: String,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Twos {
        #[arg(long, default_value = "bin", help = "bin|hex|oct|dec")]
        from: String,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    ShowAll {
        #[arg(long, default_value = "dec", help = "bin|hex|oct|dec")]
        from: String,
        value: String,
    },
    Table {
        #[arg(long, default_value = "bin", help = "bin|hex|oct|dec")]
        from: String,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    GrayConv {
        #[arg(long)]
        decode: bool,
        #[arg(long, default_value = "dec", help = "bin|hex|oct|dec")]
        from: String,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    RawDiff {
        #[arg(long = "file-a", value_name = "FILE")]
        file_a: PathBuf,
        #[arg(long = "file-b", value_name = "FILE")]
        file_b: PathBuf,
    },
    Text {
        #[arg(long, default_value = "bin", help = "bin|hex|oct|dec")]
        from: String,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
    Format {
        #[arg(long, default_value = "bin", help = "bin|hex|oct|dec")]
        to: String,
        #[arg(long, default_value = "raw", help = "raw|bin|hex|oct|dec")]
        from: String,
        #[arg(short = 'f', long, value_name = "FILE")]
        file: Option<PathBuf>,
        #[arg(num_args(0..))]
        input: Vec<String>,
    },
}

fn read_text(file: Option<PathBuf>, args: Vec<String>) -> Result<String, String> {
    match file {
        Some(path) => std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read '{}': {}", path.display(), e)),
        None => Ok(args.join(" ")),
    }
}

fn read_bytes(file: Option<PathBuf>, args: Vec<String>) -> Result<Vec<u8>, String> {
    match file {
        Some(path) => {
            std::fs::read(&path).map_err(|e| format!("failed to read '{}': {}", path.display(), e))
        }
        None => Ok(args.join(" ").into_bytes()),
    }
}

fn read_op_side(file: Option<PathBuf>, arg: Option<String>, side: &str) -> Result<String, String> {
    match file {
        Some(path) => std::fs::read_to_string(&path)
            .map_err(|e| format!("failed to read '{}': {}", path.display(), e)),
        None => arg.ok_or_else(|| {
            format!(
                "missing operand {}: provide --file-{} or a positional argument",
                side, side
            )
        }),
    }
}

fn write_text(output: Option<PathBuf>, content: &str) -> Result<(), String> {
    match output {
        Some(path) => std::fs::write(&path, content)
            .map_err(|e| format!("failed to write '{}': {}", path.display(), e)),
        None => {
            println!("{}", content);
            Ok(())
        }
    }
}

fn write_bytes(output: Option<PathBuf>, content: &[u8]) -> Result<(), String> {
    match output {
        Some(path) => std::fs::write(&path, content)
            .map_err(|e| format!("failed to write '{}': {}", path.display(), e)),
        None => {
            use std::io::Write;
            std::io::stdout()
                .write_all(content)
                .map_err(|e| format!("failed to write to stdout: {}", e))
        }
    }
}

fn parse_fmt(s: &str) -> Result<Format, String> {
    Format::from_str(s)
}

fn parse_range(s: &str) -> Result<(u8, u8), String> {
    let parts: Vec<&str> = s.split(':').collect();
    if parts.len() != 2 {
        return Err(format!(
            "invalid range '{}': expected format START:END",
            s
        ));
    }
    let start: u8 = parts[0]
        .parse()
        .map_err(|_| format!("invalid start '{}' in range", parts[0]))?;
    let end: u8 = parts[1]
        .parse()
        .map_err(|_| format!("invalid end '{}' in range", parts[1]))?;
    Ok((start, end))
}

fn run() -> Result<(), String> {
    let cli = Cli::parse();

    match cli.command {
        Command::Encode {
            file,
            output,
            format,
            sep,
            width,
            lsb,
            nibbles,
            input,
        } => {
            let fmt = parse_fmt(&format)?;
            let text = read_text(file, input)?;
            let opts = EncodeOpts {
                format: fmt,
                sep,
                width,
                lsb_first: false,
                nibble_groups: nibbles,
            };
            let result = if lsb {
                codec::encode::encode_with_lsb(text.as_bytes(), &opts)
            } else {
                encode(text.as_bytes(), &opts)
            };
            write_text(output, &result)
        }

        Command::Decode {
            file,
            output,
            format,
            lsb,
            input,
        } => {
            let fmt = parse_fmt(&format)?;
            let text = read_text(file, input)?;
            let bytes = codec::decode::decode_format(&text, fmt, lsb)?;
            write_bytes(output, &bytes)
        }

        Command::Inspect {
            file,
            output,
            width,
            style,
            summary,
            input,
        } => {
            let bytes = read_bytes(file, input)?;
            let mut result = analyze::inspect::inspect_with_style(&bytes, &style, width)?;
            if summary {
                result.push('\n');
                result.push('\n');
                result.push_str(&analyze::inspect::inspect_summary(&bytes));
            }
            write_text(output, &result)
        }

        Command::Xor {
            file_a,
            file_b,
            a,
            b,
            output,
        } => {
            let sa = read_op_side(file_a, a, "a")?;
            let sb = read_op_side(file_b, b, "b")?;
            let result = ops::xor(&sa, &sb)?;
            write_text(output, &result)
        }

        Command::And {
            file_a,
            file_b,
            a,
            b,
            output,
        } => {
            let sa = read_op_side(file_a, a, "a")?;
            let sb = read_op_side(file_b, b, "b")?;
            let result = ops::and(&sa, &sb)?;
            write_text(output, &result)
        }

        Command::Or {
            file_a,
            file_b,
            a,
            b,
            output,
        } => {
            let sa = read_op_side(file_a, a, "a")?;
            let sb = read_op_side(file_b, b, "b")?;
            let result = ops::or(&sa, &sb)?;
            write_text(output, &result)
        }

        Command::Not {
            file,
            output,
            input,
        } => {
            let text = read_text(file, input)?;
            let result = ops::not(&text)?;
            write_text(output, &result)
        }

        Command::Shift {
            left,
            right,
            file,
            output,
            input,
        } => {
            let (n, go_left) = match (left, right) {
                (Some(n), None) => (n, true),
                (None, Some(n)) => (n, false),
                _ => return Err("provide exactly one of --left or --right".to_string()),
            };
            let text = read_text(file, input)?;
            let result = ops::shift(&text, n, go_left)?;
            write_text(output, &result)
        }

        Command::Reverse {
            file,
            output,
            input,
        } => {
            let text = read_text(file, input)?;
            let result = ops::reverse(&text)?;
            write_text(output, &result)
        }

        Command::Stats { file, top, input } => {
            let bytes = read_bytes(file, input)?;
            let result = analyze::stats::compute_and_format(&bytes, top);
            println!("{}", result);
            Ok(())
        }

        Command::Convert { from, to, value } => {
            let from_fmt = parse_fmt(&from)?;
            let to_fmt = parse_fmt(&to)?;
            let result = convert::base::convert(&value, from_fmt, to_fmt)?;
            println!("{}", result);
            Ok(())
        }

        Command::Nand {
            file_a,
            file_b,
            a,
            b,
            output,
        } => {
            let sa = read_op_side(file_a, a, "a")?;
            let sb = read_op_side(file_b, b, "b")?;
            let result = ops::nand(&sa, &sb)?;
            write_text(output, &result)
        }

        Command::Nor {
            file_a,
            file_b,
            a,
            b,
            output,
        } => {
            let sa = read_op_side(file_a, a, "a")?;
            let sb = read_op_side(file_b, b, "b")?;
            let result = ops::nor(&sa, &sb)?;
            write_text(output, &result)
        }

        Command::Xnor {
            file_a,
            file_b,
            a,
            b,
            output,
        } => {
            let sa = read_op_side(file_a, a, "a")?;
            let sb = read_op_side(file_b, b, "b")?;
            let result = ops::xnor(&sa, &sb)?;
            write_text(output, &result)
        }

        Command::Rotate {
            left,
            right,
            file,
            output,
            input,
        } => {
            let (n, go_left) = match (left, right) {
                (Some(n), None) => (n, true),
                (None, Some(n)) => (n, false),
                _ => return Err("provide exactly one of --left or --right".to_string()),
            };
            let text = read_text(file, input)?;
            let result = ops::rotate(&text, n, go_left)?;
            write_text(output, &result)
        }

        Command::Swap {
            bytes,
            nibbles,
            file,
            output,
            input,
        } => {
            if !bytes && !nibbles {
                return Err("provide at least one of --bytes or --nibbles".to_string());
            }
            let text = read_text(file, input)?;
            let result = if bytes {
                ops::swap_bytes(&text)?
            } else {
                ops::swap_nibbles(&text)?
            };
            write_text(output, &result)
        }

        Command::Popcount { file, input } => {
            let text = read_text(file, input)?;
            let result = ops::format_popcount_output(&text)?;
            println!("{}", result);
            Ok(())
        }

        Command::Clz { file, input } => {
            let text = read_text(file, input)?;
            let result = ops::format_clz_output(&text)?;
            println!("{}", result);
            Ok(())
        }

        Command::Ctz { file, input } => {
            let text = read_text(file, input)?;
            let result = ops::format_ctz_output(&text)?;
            println!("{}", result);
            Ok(())
        }

        Command::Parity { file, input } => {
            let text = read_text(file, input)?;
            let result = ops::parity(&text)?;
            println!("{}", result);
            Ok(())
        }

        Command::Field {
            get,
            set_range,
            value,
            file,
            output,
            input,
        } => {
            let text = read_text(file, input)?;
            if let Some(range_str) = get {
                let (start, end) = parse_range(&range_str)?;
                let result = ops::bit_field_get(&text, start, end)?;
                write_text(output, &result)
            } else if let Some(range_str) = set_range {
                let (start, end) = parse_range(&range_str)?;
                let vbits = value
                    .ok_or_else(|| "field set requires --value BITS".to_string())?;
                let result = ops::bit_field_set(&text, start, end, &vbits)?;
                write_text(output, &result)
            } else {
                Err("provide either --get START:END or --set START:END".to_string())
            }
        }

        Command::Interleave {
            file_a,
            file_b,
            a,
            b,
            output,
        } => {
            let sa = read_op_side(file_a, a, "a")?;
            let sb = read_op_side(file_b, b, "b")?;
            let result = ops::interleave(&sa, &sb)?;
            write_text(output, &result)
        }

        Command::Deinterleave {
            file,
            output,
            input,
        } => {
            let text = read_text(file, input)?;
            let result = ops::deinterleave(&text)?;
            write_text(output, &result)
        }

        Command::Gray {
            decode,
            file,
            output,
            input,
        } => {
            if decode {
                let text = read_text(file, input)?;
                let bytes = codec::encode::decode_gray(&text)?;
                let opts = EncodeOpts {
                    format: Format::Bin,
                    sep: " ".to_string(),
                    width: None,
                    lsb_first: false,
                    nibble_groups: false,
                };
                let result = codec::encode::encode(&bytes, &opts);
                write_text(output, &result)
            } else {
                let bytes = read_bytes(file, input)?;
                let result = codec::encode::encode_gray(&bytes, " ", None);
                write_text(output, &result)
            }
        }

        Command::Bcd {
            decode,
            file,
            output,
            input,
        } => {
            if decode {
                let text = read_text(file, input)?;
                let bytes = codec::encode::decode_bcd(&text)?;
                let opts = EncodeOpts {
                    format: Format::Bin,
                    sep: " ".to_string(),
                    width: None,
                    lsb_first: false,
                    nibble_groups: false,
                };
                let result = codec::encode::encode(&bytes, &opts);
                write_text(output, &result)
            } else {
                let bytes = read_bytes(file, input)?;
                let result = codec::encode::encode_bcd(&bytes, " ", None);
                write_text(output, &result)
            }
        }

        Command::Base64 {
            decode,
            file,
            output,
            input,
        } => {
            if decode {
                let text = read_text(file, input)?;
                let bytes = codec::encode::decode_base64(&text)?;
                write_bytes(output, &bytes)
            } else {
                let bytes = read_bytes(file, input)?;
                let result = codec::encode::encode_base64(&bytes);
                write_text(output, &result)
            }
        }

        Command::Rle {
            decode,
            file,
            output,
            input,
        } => {
            if decode {
                let text = read_text(file, input)?;
                let bytes = codec::encode::decode_run_length_binary(&text)?;
                write_bytes(output, &bytes)
            } else {
                let bytes = read_bytes(file, input)?;
                let result = codec::encode::encode_run_length_binary(&bytes);
                write_text(output, &result)
            }
        }

        Command::Crc { algo, file, input } => {
            let bytes = read_bytes(file, input)?;
            let result = convert::checksum::compute(&bytes, &algo)?;
            println!("{}", result);
            Ok(())
        }

        Command::Diff {
            file_a,
            file_b,
            format,
            a,
            b,
        } => {
            let fmt = parse_fmt(&format)?;
            let sa = read_op_side(file_a, a, "a")?;
            let sb = read_op_side(file_b, b, "b")?;
            let result = analyze::diff::diff_binary(&sa, &sb, fmt)?;
            println!("{}", result);
            Ok(())
        }

        Command::Pattern {
            r#type,
            count,
            seed,
            format,
            list,
            output,
        } => {
            if list {
                println!("{}", generate::patterns::list_patterns());
                return Ok(());
            }
            let bytes = generate::patterns::generate(&r#type, count, seed)?;
            let result = match format.as_str() {
                "hex" => generate::patterns::display_hex(&bytes, " ", Some(8)),
                "dec" => generate::patterns::display_dec(&bytes, " ", Some(8)),
                _ => generate::patterns::display(&bytes, " ", Some(8)),
            };
            write_text(output, &result)
        }

        Command::Float {
            is_f64,
            bits,
            value,
        } => {
            let result = analyze::float::inspect_float(&value, is_f64, bits)?;
            println!("{}", result);
            Ok(())
        }

        Command::All { from, value } => {
            let from_fmt = parse_fmt(&from)?;
            let result = convert::base::convert_all(&value, from_fmt)?;
            println!("{}", result);
            Ok(())
        }

        Command::Specials { is_f64 } => {
            if is_f64 {
                println!("{}", analyze::float::float_special_values_f64());
            } else {
                println!("{}", analyze::float::float_special_values_f32());
            }
            Ok(())
        }

        Command::Signed { from, file, input } => {
            let fmt = parse_fmt(&from)?;
            let text = read_text(file, input)?;
            let result = convert::base::interpret_signed(&text, fmt)?;
            println!("{}", result);
            Ok(())
        }

        Command::Twos { from, file, input } => {
            let fmt = parse_fmt(&from)?;
            let text = read_text(file, input)?;
            let result = convert::base::to_twos_complement(&text, fmt)?;
            println!("{}", result);
            Ok(())
        }

        Command::ShowAll { from, value } => {
            let from_fmt = parse_fmt(&from)?;
            let result = convert::base::convert_show_all_bases(&value, from_fmt)?;
            println!("{}", result);
            Ok(())
        }

        Command::Table { from, file, input } => {
            let fmt = parse_fmt(&from)?;
            let text = read_text(file, input)?;
            let bytes = codec::decode::decode_format(&text, fmt, false)?;
            println!("{}", convert::base::format_byte_table(&bytes));
            Ok(())
        }

        Command::GrayConv { decode, from, file, input } => {
            let fmt = parse_fmt(&from)?;
            let text = read_text(file, input)?;
            let result = if decode {
                convert::base::convert_gray_decode(&text, fmt)?
            } else {
                convert::base::convert_gray_encode(&text, fmt)?
            };
            println!("{}", result);
            Ok(())
        }

        Command::RawDiff { file_a, file_b } => {
            let a = std::fs::read(&file_a)
                .map_err(|e| format!("failed to read '{}': {}", file_a.display(), e))?;
            let b = std::fs::read(&file_b)
                .map_err(|e| format!("failed to read '{}': {}", file_b.display(), e))?;
            println!("{}", analyze::diff::diff_bytes_raw(&a, &b));
            Ok(())
        }

        Command::Text { from, file, input } => {
            let text = read_text(file, input)?;
            let bytes = match from.to_ascii_lowercase().as_str() {
                "hex" | "hexadecimal" => codec::decode::decode_hex_str(&text)?,
                "bin" | "binary" => codec::decode::decode_bin_str(&text)?,
                "oct" | "octal" => codec::decode::decode_oct_str(&text)?,
                "dec" | "decimal" => codec::decode::decode_dec_str(&text)?,
                _ => return Err(format!("unknown format '{}': use bin, hex, oct, dec", from)),
            };
            println!("{}", codec::decode::decode_text(&bytes));
            Ok(())
        }

        Command::Format { to, from, file, input } => {
            let to_fmt = parse_fmt(&to)?;
            let bytes = if from == "raw" {
                read_bytes(file, input)?
            } else {
                let src_fmt = parse_fmt(&from)?;
                let text = read_text(file, input)?;
                codec::encode::parse_any_bytes(&text, src_fmt)?
            };
            println!("{}", codec::decode::bytes_to_format(&bytes, to_fmt));
            Ok(())
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
