/*
Graceful Error Handling

Result is so common that Rust has a powerful operator ? for working with them. These two statements are equivalent:

do_something_that_might_fail()?

match do_something_that_might_fail() {
    Ok(v) => v,
    Err(e) => return Err(e),
}
*/

fn do_something_that_might_fail(i: i32) -> Result<f32, String> {
    if i == 42 {
        Ok(13.0)
    } else {
        Err(String::from("this is not the right number"))
    }
}

fn main() -> Result<(), String> {
    // Look at how much code we saved!
    let v = do_something_that_might_fail(12)?;
    println!("found {}", v);
    Ok(())
}

/*Operator ? can only be used in functions that return a result*/
