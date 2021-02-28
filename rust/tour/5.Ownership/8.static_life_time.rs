/*
Static Lifetimes

A static variable is a memory resource created at compile-time that exists through a program start to finish. They must have their types explicitly specified.

A static lifetime is a memory resource that lasts indefinitely to the end of a program. Note that by this definition some static lifetime resources can be created at runtime.

Resources with static lifetimes have a special lifetime specifier 'static.

'static resources will never drop.

If static lifetime resources contain references they must all be 'static (anything less would not live long enough).

Memory detail:

    Modifying static variables is inherently dangerous because they are globally accessable to be read from by anyone introducing the possibility of a data race. We'll talk about the challenges of global data later.
    Rust allows the use of unsafe { ... } blocks to perform some operations that the compiler cannot make memory guarantees about. The R̸͉̟͈͔̄͛̾̇͜U̶͓͖͋̅Ṡ̴͉͇̃̉̀T̵̻̻͔̟͉́͆Ơ̷̥̟̳̓͝N̶̨̼̹̲͛Ö̵̝͉̖̏̾̔M̶̡̠̺̠̐͜Î̷̛͓̣̃̐̏C̸̥̤̭̏͛̎͜O̶̧͚͖͔̊͗̇͠N̸͇̰̏̏̽̃ should not be talked about casually.

    */

static PI: f64 = 3.1415;

fn main() {
    // static variables can also be scoped to a function
    static mut SECRET: &'static str = "swordfish";

    // string literals have a 'static lifetime
    let msg: &'static str = "Hello World!";
    let p: &'static f64 = &PI;
    println!("{} {}", msg, p);

    // You can break some rules, but you must be explicit
    unsafe {
        // we can set SECRET to a string literal because it is also `static
        SECRET = "abracadabra";
        println!("{}", SECRET);
    }
}
