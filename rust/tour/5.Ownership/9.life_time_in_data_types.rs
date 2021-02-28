/*Lifetimes In Data Types

Similarly to functions, data types can be parameterized with lifetime specifiers of its members.

Rust validates that the containing data structure of the references never lasts longer than the owners its references point to.

We can't have structs running around with references pointing to nothingness!*/

struct Foo<'a> {
    i: &'a i32,
}

fn main() {
    let x = 42;
    let foo = Foo { i: &x };
    println!("{}", foo.i);
}
