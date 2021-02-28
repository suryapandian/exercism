/*
Common methods:

    push_str to add more utf-8 bytes to the end of a string.
    replace to replace sequences of utf-8 bytes with others.
    to_lowercase/to_uppercase for case changes.
    trim for trimming space

*/
fn main() {
    let mut helloworld = String::from("hello");
    helloworld.push_str(" world");
    helloworld = helloworld + "!";
    println!("{}", helloworld);
    let surya = helloworld.replace("hello", "surya rocks the ");
    println!("{}", surya);
    let trim_surya = surya.trim();
    println!("{}", trim_surya);
}
