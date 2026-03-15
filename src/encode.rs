pub fn str_to_ascii (s: &str) -> Vec<u32> {
    s.chars().map(|c| c as u32).collect()
}

pub fn to_binary_vec (vec: &[u32]) -> Vec<u32> {

    let mut binary = vec![];

    for i in vec.iter() {
        binary.push(num_to_binary(*i));
    }

    binary
}

pub fn num_to_binary (num: u32) -> u32 {

    if num == 0 {
        return 0 as u32;
    }

    (num % 2) + 10 * num_to_binary((num / 2))
}

pub fn encode(input: &str) -> Vec<u32> {
        let ascii_values = str_to_ascii(&input);
        let binary_values = to_binary_vec(&ascii_values);

        binary_values
}