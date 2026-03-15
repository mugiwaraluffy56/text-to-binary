pub fn binary_to_num (binary: u32) -> u32 {
    let mut result = 0u32;
    let mut place = 1u32;
    let mut n = binary;

    while n > 0 {
        result += (n % 10) * place;
        n /= 10;
        place *= 2;
    }
    result
}

pub fn ascii_to_str (ascii_values: &[u32]) -> String {
    ascii_values.iter()
        .filter_map(|&n| char::from_u32(n))
        .collect()
} 

pub fn decode(input: &[u32]) -> String {
    let ascii_values: Vec<u32> = input.iter().map(|&b| binary_to_num(b)).collect();
    ascii_to_str(&ascii_values)
}
