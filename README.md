# binrs

binrs is a command-line tool for working with binary data. It encodes, decodes, inspects, and analyzes bytes. By default it works on raw input passed as arguments, but can also read from files. It supports a wide range of formats including binary, hex, octal, decimal, Gray code, BCD, Base64, and run-length encoded bitstreams.

binrs is written in Rust with no runtime dependencies beyond clap.

[![Build status](https://github.com/yourname/binrs/actions/workflows/ci.yml/badge.svg)](https://github.com/yourname/binrs/actions)
[![Crates.io](https://img.shields.io/crates/v/binrs.svg)](https://crates.io/crates/binrs)

Licensed under MIT.

## Quick examples

Encode a string to binary:

```
$ binrs encode "hello"
01101000 01100101 01101100 01101100 01101111
```

Switch to hex, add a custom separator, wrap at 3 bytes per line:

```
$ binrs encode "hello" --format hex --sep , --width 3
68,65,6c
6c,6f
```

Inspect raw bytes with a hexdump:

```
$ binrs inspect "hello world"
00000000  68 65 6c 6c 6f 20 77 6f  72 6c 64  |hello world|
```

Compute checksums:

```
$ binrs crc "hello" --algo all
CRC-8:       0x92
CRC-16:      0x34fd
CRC-32:      3610a686
Adler-32:    11e60398
Fletcher-16: c8f0
XOR:         0xed
Sum:         0x01
```

Inspect an IEEE 754 float:

```
$ binrs float 3.14
value:    3.14
hex:      4048f5c3
sign:     0 (+)
exponent: 10000000 (128, biased) -> 1
mantissa: 00100011110101110000101
```

Diff two binary strings bit by bit:

```
$ binrs diff "01000001" "01000010"
bit 6: 0 -> 1
bit 7: 1 -> 0
```

## Installation

From crates.io:

```
$ cargo install binrs
```

From source:

```
$ git clone https://github.com/yourname/binrs
$ cd binrs
$ cargo build --release
$ ./target/release/binrs --version
```

The binary is called `binrs`.

## Commands

### Encode / Decode

```
binrs encode "hello"
binrs encode "hello" --format hex
binrs encode "hello" --format hex --sep , --width 3
binrs encode "hello" --nibbles          # 0110.1000 0110.0101 ...
binrs encode "hello" --lsb              # LSB-first bit order
binrs encode -f file.bin

binrs decode "01101000 01100101 01101100 01101100 01101111"
binrs decode "68 65 6c 6c 6f" --format hex
```

### Inspect

```
binrs inspect "hello world"             # default hexdump
binrs inspect "hello world" --style xxd
binrs inspect "hello world" --style bits
binrs inspect "hello world" --style od
binrs inspect -f file.bin --summary
```

### Bitwise operations

```
binrs xor  "01000001" "01010101"
binrs and  "01000001" "11110000"
binrs or   "01000001" "00001111"
binrs nand "01000001" "11111111"
binrs nor  "01000001" "00000000"
binrs xnor "01000001" "01000001"
binrs not  "01000001"

# two-operand ops also accept files
binrs xor --file-a a.bin --file-b b.bin
```

### Bit manipulation

```
binrs shift --left 2 "01000001"
binrs shift --right 3 "01000001"
binrs rotate --left 1 "01000001"
binrs reverse "01000001"
binrs swap --bytes "01000001 01000010"
binrs swap --nibbles "01000001"

binrs field --get 0:3 "01000001"
binrs field --set 0:3 --value 1111 "01000001"

binrs interleave "01000001" "01000010"
binrs deinterleave "0100000001000010"
```

### Bit analysis

```
binrs popcount "01000001 11111111"
binrs clz "00000011"
binrs ctz "11000000"
binrs parity "01000001"
```

### Encodings

```
binrs gray "hello"
binrs gray --decode "..."

binrs bcd "hello"
binrs bcd --decode "..."

binrs base64 "hello"
binrs base64 --decode "aGVsbG8="

binrs rle "hello"
binrs rle --decode "..."
```

### Statistics

```
binrs stats "hello world"
# entropy, hamming weight, byte frequency, bit-position histogram,
# run-length analysis, chi-square, byte-pair frequency

binrs stats -f file.bin --top 20
```

### Checksums

```
binrs crc "hello"
binrs crc "hello" --algo crc8
binrs crc "hello" --algo crc16
binrs crc "hello" --algo crc32
binrs crc "hello" --algo adler32
binrs crc "hello" --algo fletcher16
binrs crc "hello" --algo xor
binrs crc "hello" --algo all
binrs crc -f file.bin --algo crc32
```

Supported algorithms: CRC-8/SMBUS, CRC-8/MAXIM, CRC-16/CCITT, CRC-16/ARC, CRC-32/PKZIP, Adler-32, Fletcher-16, XOR, sum.

### Conversion

```
binrs convert --from dec --to bin 65
binrs convert --from hex --to bin FF
binrs convert --from bin --to hex 01000001

binrs all --from dec 65         # all bases at once
binrs show-all --from dec 65    # full representation

binrs signed --from bin "01000001 10000001"
binrs twos   --from bin "01000001"

binrs gray-conv "01000001"
binrs gray-conv --decode "00100001"
```

### IEEE 754

```
binrs float 3.14
binrs float --f64 3.14159265358979
binrs float --bits 3f800000
binrs specials
binrs specials --f64
```

### Diff

```
binrs diff "01000001" "01000010"
binrs diff --format hex "41" "42"
binrs raw-diff --file-a a.bin --file-b b.bin
```

### Pattern generation

```
binrs pattern --type zeros --count 8
binrs pattern --type ones  --count 8
binrs pattern --type walk  --count 8
binrs pattern --type prng  --count 16 --seed 42
binrs pattern --type inc   --count 8 --format hex
binrs pattern --list
```

### Formatting

```
binrs table --from bin "01000001 01000010"
# bin / hex / oct / dec / signed table per byte

binrs format --from bin --to hex "01000001 01000010"
binrs text --from hex "68 65 6c 6c 6f"
```

## All commands

| Command | Description |
|---|---|
| `encode` | Convert input to binary, hex, octal, or decimal |
| `decode` | Convert binary/hex/octal/decimal back to bytes |
| `inspect` | Hexdump in dump, xxd, bits, or od style |
| `xor` `and` `or` `nand` `nor` `xnor` | Two-operand bitwise ops |
| `not` | Bitwise NOT |
| `shift` | Shift bits left or right per byte |
| `rotate` | Circular bit rotation per byte |
| `reverse` | Reverse bits in each byte |
| `swap` | Byte order or nibble swap |
| `popcount` | Count set bits |
| `clz` `ctz` | Leading / trailing zero count |
| `parity` | Even or odd parity per byte |
| `field` | Extract or set a bit field range |
| `interleave` `deinterleave` | Morton code bit interleaving |
| `gray` | Gray code encode/decode (raw bytes) |
| `bcd` | BCD encode/decode |
| `base64` | Base64 encode/decode |
| `rle` | Run-length encode/decode bitstreams |
| `stats` | Entropy, hamming weight, frequency histograms |
| `crc` | CRC-8/16/32, Adler-32, Fletcher-16, XOR, sum |
| `convert` | Convert a number between bases |
| `all` | Print all bases for one value |
| `show-all` | Full numeric representation of a value |
| `signed` | Interpret bytes as signed i8 |
| `twos` | Two's complement |
| `gray-conv` | Gray code for numeric values |
| `table` | Multi-column breakdown per byte |
| `format` | Reformat bytes between bases |
| `text` | Decode encoded input to UTF-8 text |
| `float` | IEEE 754 float bit breakdown |
| `specials` | IEEE 754 special values table |
| `diff` | Bit-level diff of two encoded inputs |
| `raw-diff` | Diff two raw binary files |
| `pattern` | Generate test byte patterns |

## Building

binrs requires Rust 1.85.0 or newer.

```
$ git clone https://github.com/yourname/binrs
$ cd binrs
$ cargo build --release
```

To run tests:

```
$ cargo test --workspace
```

## License

MIT
