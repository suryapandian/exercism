fn main() {
    let x = 42;
    if x < 42 {
        println!("less than 42");
    } else if x == 42 {
        println!("is 42");
    } else {
        println!("greater than 42");
    }

    //loop

    let mut x = 0;
    loop {
        x += 1;
        if x == 42 {
            break;
        }
    }

    //returning values from loop
    let mut z = 0;
    let v = loop {
        z += 1;
        if z == 13 {
            break "found the 13";
        }
    };
    println!("from loop: {}", v);

    //while
    println!("{}", x);
    x = 0;
    while x != 97 {
        x += 1;
    }
    println!("{}", x);

    //for
    for x in 0..5 {
        println!("{}", x);
    }

    /*The .. operator creates an iterator that generates numbers from a start number up to but not including an end number.

    The ..= operator creates an iterator that generates numbers from a start number up to and including an end number.*/

    for x in 0..=5 {
        println!("{}", x);
    }

    let y = 5;
    match y {
        0 => {
            println!("found zero");
        }
        // we can match against multiple values
        1 | 2 => {
            println!("found 1 or 2!");
        }
        // we can match against ranges
        3..=9 => {
            println!("found a number 3 to 9 inclusively");
        }
        // we can bind the matched number to a variable
        matched_num @ 10..=100 => {
            println!("found {} number between 10 to 100!", matched_num);
        }
        // this is the default match that must exist if not all cases are handled
        _ => {
            println!("found something else!");
        }
    }
}
