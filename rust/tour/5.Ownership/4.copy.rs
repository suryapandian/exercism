fn main() {
    let mut foo = 42;
    let f = &mut foo;
    let bar = *f; // get a copy of the owner's value
    *f = 13; // set the reference's owner's value
    println!("{}", bar);
    println!("{}", foo);
}
