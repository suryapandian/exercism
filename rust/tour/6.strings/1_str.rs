fn main() {
    let a: &'static str = "Ferris 
    says:\t\"hello\\ \r 🦀";
    println!("{} {}", a, a.len());

    //Use a \ at the end of a line if you don't want a line break.
    println!(
        "hello \
    world"
    ); // notice that the spacing before w is ignored

    /*
        Raw String Literals

    Raw strings allow us to write a sequence of characters verbatim by starting with r#" and ending with "#. It lets us insert characters that might otherwise confuse a normal string as literals (like double quotes and backslashes).
    */

    let b: &'static str = r#"
        <div class="advice">
            Raw strings are useful for some situations.
        </div>
        "#;
    println!("{}", b);
    let hello_html = include_str!("hello.html");
    println!("{}", hello_html);
}
