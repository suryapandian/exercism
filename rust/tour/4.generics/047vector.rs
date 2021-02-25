/*
Vectors

Some of the most useful generic types are collection types. A vector is a variably sized list of items represented by the struct Vec.

The macro vec! lets us easily create a vector rather than manually constructing one.

Vec has the method iter() which creates an iterator from a vector, allowing us to easily put a vector into a for loop.

Memory Details:

    Vec is a struct, but internally it contains a reference to a fixed list of its items on the heap.
    A vector starts with a default capacity; when more items are added than it has capacity for, it reallocates its data on the heap to have a new fixed list with large capacity.
*/

fn main() {
    // We can be explicit with type
    let mut i32_vec = Vec::<i32>::new(); // turbofish <3
    i32_vec.push(1);
    i32_vec.push(2);
    i32_vec.push(3);

    // But look how clever Rust is about determining the type automatically
    let mut float_vec = Vec::new();
    float_vec.push(1.3);
    float_vec.push(2.3);
    float_vec.push(3.4);

    // That's a beautiful macro!
    let string_vec = vec![String::from("Hello"), String::from("World")];

    for word in string_vec.iter() {
        println!("{}", word);
    }
}
