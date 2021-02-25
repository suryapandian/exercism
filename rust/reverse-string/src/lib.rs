// pub fn reverse(input: &str) -> String {
//     input.chars().rev().collect()
// }

pub fn reverse(input: &str) -> String {
    let mut length = input.len();
    let mut result = String::new();
    if length == 0 {
        return result;
    }

    let char_vec: Vec<char> = input.chars().collect();
    length = char_vec.len();
    for x in 0..length {
        result.push(char_vec[length - 1 - x]);
    }

    result
}
