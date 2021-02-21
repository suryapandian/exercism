fn swap(x: i32, y: i32) -> (i32, i32) {
    return (y, x);
}

fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

fn make_nothing() -> () {
    return ();
}

fn make_nothing2() {
    // this function will return () if nothing is specified to return
}

fn main() {
    // return a tuple of return values
    let result = swap(123, 321);
    println!("{} {}", result.0, result.1);

    // destructure the tuple into two variables names
    let (a, b) = swap(result.0, result.1);
    println!("{} {}", a, b);

    let a = make_nothing();
    let b = make_nothing2();

    // Printing a debug string for a and b
    // Because it's hard to print nothingness
    println!("The value of a: {:?}", a);
    println!("The value of b: {:?}", b);

    println!("The value of b: {:?}", add(2, 5));
}
