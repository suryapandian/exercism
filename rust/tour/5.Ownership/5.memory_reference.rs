/*
Passing Around Borrowed Data

Rust's rules for references might best be summarized by:

    Rust only allows there to be one mutable reference or multiple non-mutable references but not both.
    A reference must never live longer than its owner.


    */

struct Foo {
    x: i32,
}

fn do_something(a: &Foo) -> &i32 {
    return &a.x;
}
/*Fix errors in the main function to understand*/
fn main() {
    let mut foo = Foo { x: 42 };
    let x = &mut foo.x;
    *x = 13;
    // x is dropped here allow us to create a non-mutable reference
    let z = &mut foo.x;
    let y = do_something(&foo);
    println!("{} {} {}", y, *z, *x);
    // y is dropped here
    // foo is dropped here
}

// struct Foo {
//     x: i32,
// }

// fn do_something(f: &mut Foo) {
//     f.x += 1;
//     // mutable reference f is dropped here
// }

// fn main() {
//     let mut foo = Foo { x: 42 };
//     println!("{}", foo.x);
//     do_something(&mut foo);
//     println!("{}", foo.x);
//     // because all mutable references are dropped within
//     // the function do_something, we can create another.
//     do_something(&mut foo);
//     println!("{}", foo.x);
//     // foo is dropped here
// }
