/*Many types can be converted to a string using to_string.

The generic function parse can be used to convert strings or string literals into a typed value. This function returns a Result because it could fail.
*/

fn main() -> Result<(), std::num::ParseIntError> {
    let a = 42;
    let a_string = a.to_string();
    let b = a_string.parse::<i32>()?;
    println!("{} {}", a, b);
    Ok(())
}
