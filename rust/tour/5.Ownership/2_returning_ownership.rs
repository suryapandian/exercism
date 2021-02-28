struct Foo {
    x: i32,
}

fn do_something() -> Foo {
    Foo { x: 42 }
    // ownership is moved out
}

fn main() {
    let foo = do_something();
    // foo becomes the owner
    // foo is dropped because of end of function scope

    let f = &foo;
    println!("{}", f.x);
    // f is dropped here
    // foo is dropped here
}
