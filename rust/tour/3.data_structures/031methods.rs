/*
static methods — methods that belong to a type itself are called using the :: operator.

instance methods — methods that belong to an instance of a type are called using the . operator.

 */

fn main() {
    // Using a static method to create an instance of String
    let s = String::from("Hello world!");
    // Using a method on the instance
    println!("{} is {} characters long.", s, s.len());
}
