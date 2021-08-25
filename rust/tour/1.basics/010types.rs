fn main() {
    let x = 12; // by default this is i32
    // x = 3 -> wil throw error since variables are immutable by default i.e) their values cannot be changed
    let a = 12u8;
    let b = 4.3; // by default this is f64
    let c = 4.3f32;
    let bv = true;
    let t = (13, false);
    let sentence = "hello world!";
    println!(
        "{} {} {} {} {} {} {} {}",
        x, a, b, c, bv, t.0, t.1, sentence
    );

    //Variable names are always in snake_case.
    let x: f64 = 3.14159;
    println!("{}", x);

    // rust can also declare and initialize later, but this is rarely done
    let x;
    x = 0;
    println!("{}", x);

    // type conversion

    let a = 13u8;
    let b = 7u32;
    let c = a as u32 + b;
    println!("{}", c);

    let t = true;
    println!("{}", t as u8);

    //Unlike variables, constants must always have explicit types.
    const PI: f32 = 3.14159;

    //mutable variables
    let mut x = 42;
    println!("{}", x);
    x = 13;
    println!("{}", x);

    //tuple
    let t = (13, false);
    println!("{} {}", t.1, PI);

    //array
    let nums: [i32; 3] = [1, 2, 3];
    println!("{:?}", nums);
    println!("{}", nums[1]);
}
