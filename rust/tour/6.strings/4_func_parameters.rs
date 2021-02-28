/*
String literals and strings are generally passed around as a string slice to functions. This offers a lot of flexibility for most scenarios where you don't actually have to pass ownership.
*/
fn say_it_loud(msg: &str) {
    println!("{}!!!", msg.to_string().to_uppercase());
}

fn main() {
    // say_it_loud can borrow &'static str as a &str
    say_it_loud("hello");
    // say_it_loud can also borrow String as a &str
    say_it_loud(&String::from("goodbye"));
}
