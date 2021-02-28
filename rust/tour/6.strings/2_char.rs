fn main() {
    // collect the characters as a vector of char
    let chars = "hi 🦀".chars().collect::<Vec<char>>();
    println!("{}", chars.len()); // should be 4
                                 // since chars are 4 bytes we can convert to u32
    println!("{}", chars[3] as u32);
}
