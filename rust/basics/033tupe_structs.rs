struct Location(i32, i32);
struct Marker;

fn main() {
    // This is still a struct on a stack
    let loc = Location(42, 32);
    println!("{}, {}", loc.0, loc.1);
    let _m = Marker; //unit like struct,  a unit is another word for an empty tuple (). This is why this kind of struct is called Unit-like.
}
