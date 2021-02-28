/*Multiple Lifetimes

Lifetime specifiers allow us to be explicit with certain scenarios the compiler cannot resolve itself by distinguishing all of a function signature component's lifetimes.
*/
struct Foo {
    x: i32,
}

// foo_b and the return value share the same lifetime
// foo_a has an unrelated lifetime
fn do_something<'a, 'b>(foo_a: &'a Foo, foo_b: &'b Foo) -> &'b i32 {
    println!("{}", foo_a.x);
    println!("{}", foo_b.x);
    return &foo_b.x;
}

fn main() {
    let foo_a = Foo { x: 42 };
    let foo_b = Foo { x: 12 };
    let x = do_something(&foo_a, &foo_b);
    // foo_a is dropped here because only foo_b's lifetime exist beyond here
    println!("{}", x);
    // x is dropped here
    // foo_b is dropped here
}
