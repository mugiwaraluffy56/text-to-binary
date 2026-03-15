fn crc8_table(poly: u8) -> [u8; 256] {
    let mut table = [0u8; 256];
    for i in 0usize..256 {
        let mut crc = i as u8;
        for _ in 0..8 {
            if crc & 0x80 != 0 {
                crc = (crc << 1) ^ poly;
            } else {
                crc <<= 1;
            }
        }
        table[i] = crc;
    }
    table
}

fn crc8_reflected_table(poly: u8) -> [u8; 256] {
    let mut table = [0u8; 256];
    for i in 0usize..256 {
        let mut crc = i as u8;
        for _ in 0..8 {
            if crc & 0x01 != 0 {
                crc = (crc >> 1) ^ poly;
            } else {
                crc >>= 1;
            }
        }
        table[i] = crc;
    }
    table
}

fn crc8(data: &[u8]) -> u8 {
    let table = crc8_table(0x07);
    let mut crc = 0u8;
    for &b in data {
        crc = table[(crc ^ b) as usize];
    }
    crc
}

fn crc8_maxim(data: &[u8]) -> u8 {
    let table = crc8_reflected_table(0x8C);
    let mut crc = 0u8;
    for &b in data {
        crc = table[(crc ^ b) as usize];
    }
    crc
}

fn crc16_ccitt(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    for &b in data {
        let pos = ((crc >> 8) ^ b as u16) as u8;
        let mut x = (pos as u16) << 8;
        for _ in 0..8 {
            if x & 0x8000 != 0 {
                x = (x << 1) ^ 0x1021;
            } else {
                x <<= 1;
            }
        }
        crc = (crc << 8) ^ x;
    }
    crc
}

fn crc16_arc(data: &[u8]) -> u16 {
    let mut crc: u16 = 0x0000;
    for &b in data {
        crc ^= b as u16;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc = (crc >> 1) ^ 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    crc
}

fn crc32(data: &[u8]) -> u32 {
    let poly = 0xEDB88320u32;
    let mut table = [0u32; 256];
    for i in 0..256u32 {
        let mut crc = i;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ poly;
            } else {
                crc >>= 1;
            }
        }
        table[i as usize] = crc;
    }
    let mut crc = 0xFFFFFFFFu32;
    for &b in data {
        let idx = ((crc ^ b as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ table[idx];
    }
    crc ^ 0xFFFFFFFF
}

fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65521;
    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for &byte in data {
        a = (a + byte as u32) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}

fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;
    for &byte in data {
        sum1 = sum1.wrapping_add(byte as u16) % 255;
        sum2 = sum2.wrapping_add(sum1) % 255;
    }
    (sum2 << 8) | sum1
}

fn xor_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc ^ b)
}

fn sum_checksum(data: &[u8]) -> u8 {
    data.iter().fold(0u8, |acc, &b| acc.wrapping_add(b))
}

pub fn compute(data: &[u8], algo: &str) -> Result<String, String> {
    match algo.to_ascii_lowercase().as_str() {
        "crc8" | "crc8-smbus" => {
            let v = crc8(data);
            Ok(format!(
                "CRC-8/SMBUS: 0x{:02x} ({}) [{:08b}]",
                v, v, v
            ))
        }
        "crc8-maxim" | "crc8maxim" => {
            let v = crc8_maxim(data);
            Ok(format!(
                "CRC-8/MAXIM: 0x{:02x} ({}) [{:08b}]",
                v, v, v
            ))
        }
        "crc16" | "crc16-ccitt" | "crc16ccitt" => {
            let v = crc16_ccitt(data);
            Ok(format!(
                "CRC-16/CCITT: 0x{:04x} ({}) [{:016b}]",
                v, v, v
            ))
        }
        "crc16-arc" | "crc16arc" => {
            let v = crc16_arc(data);
            Ok(format!(
                "CRC-16/ARC: 0x{:04x} ({}) [{:016b}]",
                v, v, v
            ))
        }
        "crc32" | "crc32-pkzip" => {
            let v = crc32(data);
            Ok(format!(
                "CRC-32/PKZIP: 0x{:08x} ({}) [{:032b}]",
                v, v, v
            ))
        }
        "adler32" => {
            let v = adler32(data);
            Ok(format!(
                "Adler-32: 0x{:08x} ({}) [{:032b}]",
                v, v, v
            ))
        }
        "fletcher16" => {
            let v = fletcher16(data);
            Ok(format!(
                "Fletcher-16: 0x{:04x} ({}) [{:016b}]",
                v, v, v
            ))
        }
        "xor" | "xor-checksum" => {
            let v = xor_checksum(data);
            Ok(format!(
                "XOR checksum: 0x{:02x} ({}) [{:08b}]",
                v, v, v
            ))
        }
        "sum" | "sum-checksum" => {
            let v = sum_checksum(data);
            Ok(format!(
                "SUM checksum: 0x{:02x} ({}) [{:08b}]",
                v, v, v
            ))
        }
        "all" => {
            let lines = vec![
                format!("CRC-8/SMBUS:   0x{:02x}", crc8(data)),
                format!("CRC-8/MAXIM:   0x{:02x}", crc8_maxim(data)),
                format!("CRC-16/CCITT:  0x{:04x}", crc16_ccitt(data)),
                format!("CRC-16/ARC:    0x{:04x}", crc16_arc(data)),
                format!("CRC-32/PKZIP:  0x{:08x}", crc32(data)),
                format!("Adler-32:      0x{:08x}", adler32(data)),
                format!("Fletcher-16:   0x{:04x}", fletcher16(data)),
                format!("XOR checksum:  0x{:02x}", xor_checksum(data)),
                format!("SUM checksum:  0x{:02x}", sum_checksum(data)),
            ];
            Ok(lines.join("\n"))
        }
        _ => Err(format!(
            "unknown checksum algorithm '{}': use crc8, crc8-maxim, crc16, crc16-arc, crc32, adler32, fletcher16, xor, sum, all",
            algo
        )),
    }
}
