# Changelog

## Unreleased

### Added
- `inspect` with four styles: dump, xxd, bits, od
- `inspect --summary` flag for quick byte summary
- All bitwise ops: xor, and, or, nand, nor, xnor, not
- `shift`, `rotate`, `reverse`, `swap` (bytes or nibbles)
- `popcount`, `clz`, `ctz`, `parity`
- `field --get` and `field --set` for bit field manipulation
- `interleave` and `deinterleave` (Morton encoding)
- `gray` encode/decode on raw bytes
- `gray-conv` for numeric gray code conversion
- `bcd` encode/decode
- `base64` encode/decode
- `rle` run-length encode/decode on binary bitstreams
- `stats` with entropy, bit-position histogram, chi-square, byte-pair frequency
- `crc` with CRC-8/SMBUS, CRC-8/MAXIM, CRC-16/CCITT, CRC-16/ARC, CRC-32, Adler-32, Fletcher-16, XOR, sum
- `convert` between bin/hex/oct/dec (u128 range)
- `all`, `show-all` to display all representations at once
- `signed`, `twos` for signed and two's complement interpretation
- `table` for per-byte multi-column breakdown
- `format` to reformat bytes from one base to another
- `text` to decode encoded input back to printable text
- `float` for full IEEE 754 f32/f64 breakdown
- `specials` for IEEE 754 special values table
- `diff` for bit-level comparison of two encoded inputs
- `raw-diff` for comparing two raw binary files
- `pattern` with 12 pattern types (zeros, ones, alt, walk, prng, ...)
- `encode --nibbles` for nibble-group display (0110.1000)
- `encode --lsb` and `decode --lsb` for LSB-first bit ordering
- `encode --format hex|oct|dec` for multi-format encoding

## 0.1.0

- Initial release with `encode` and `decode` for binary representation
- File input/output via `-f` and `-o`
