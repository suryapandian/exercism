pub fn raindrops(n: u32) -> String {
    match (n % 3, n % 5, n % 7) {
        (0, 0, 0) => "PlingPlangPlong".to_owned(),
        (0, 0, _) => "PlingPlang".to_owned(),
        (_, 0, 0) => "PlangPlong".to_owned(),
        (0, _, 0) => "PlingPlong".to_owned(),
        (0, _, _) => "Pling".to_owned(),
        (_, 0, _) => "Plang".to_owned(),
        (_, _, 0) => "Plong".to_owned(),
        (_, _, _) => n.to_string(),
    }

    /*
    iter, map and collect

    let factors = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let output = factors
        .iter()
        .map(|factor| if n % factor.0 == 0 { factor.1 } else { "" })
        .collect::<String>();

    match output.is_empty() {
        true => n.to_string(),
        false => output,
    }

    */

    /*
    String concatenation

    let mut str = String::new();
    if n % 3 == 0 {
        str.push_str("Pling")
    }
    if n % 5 == 0 {
        str.push_str("Plang")
    }
    if n % 7 == 0 {
        str.push_str("Plong")
    }
    if str.is_empty() {
        str = n.to_string();
        // str.push_str(&format!("{}", n));
    }
    str
    */
}
