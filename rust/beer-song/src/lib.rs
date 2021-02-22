pub fn verse(n: u32) -> String {
    let nth_beer_verse = bottle_verse(n);

    if n > 0 {
        let is_last = if n == 1 { "it" } else { "one" };
        return format!(
            "{} on the wall, {}.\nTake {} down and pass it around, {} on the wall.\n",
            nth_beer_verse,
            nth_beer_verse,
            is_last,
            bottle_verse(n - 1)
        );
    }

    format!(
        "No more bottles of beer on the wall, {}.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n",nth_beer_verse
    )
}

pub fn bottle_verse(n: u32) -> String {
    if n > 1 {
        return format!("{} bottles of beer", n);
    }
    if n == 1 {
        return format!("{} bottle of beer", n);
    }
    String::from("no more bottles of beer")
}

pub fn sing(start: u32, end: u32) -> String {
    let mut x = start;
    let mut result = String::from("");
    while x > end {
        result = format!("{}{}\n", result, verse(x));
        x -= 1;
    }
    format!("{}{}", result, verse(x))
}
