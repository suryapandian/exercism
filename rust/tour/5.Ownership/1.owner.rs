/*
Scope-Based Resource Management

Rust uses the end of scope as the place to deconstruct and deallocate a resource.

The term for this deconstruction and deallocation is called a drop.

Memory detail:

    Rust does not have garbage collection.
    This is also called Resource Aquisition Is Initialization ( RAII ) in C++
    When a struct is dropped, the struct itself is dropped first, then its children are dropped individually, and so on..

*/

struct Foo {
    x: i32,
}

fn do_something(f: Foo) {
    println!("{}", f.x);
    // f is dropped here
}

fn main() {
    // We instantiate structs and bind to variables
    // to create memory resources
    let foo = Foo { x: 42 };
    // foo is the owner

    println!("{}", foo.x);
    // foo is dropped here

    /*
        Moving Ownership

    When an owner is passed as an argument to a function, ownership is moved to the function parameter.

    After a move the variable in the original function can no longer be used.

    Memory details:

        During a move the stack memory of the owners value is copied to the function call's parameter stack memory.
    */
    do_something(foo);
    // foo can no longer be used
}
