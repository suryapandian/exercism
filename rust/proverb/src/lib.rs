pub fn build_proverb(list: &[&str]) -> String {
    let mut result = format!("");
    let length = list.len();
    if length == 0 {
        return result;
    }
    let last_line = format!("And all for the want of a {}.", list[0]);
    if length == 1 {
        return last_line;
    }
    for (i, item) in list.iter().enumerate() {
        if i == 0 {
            result = format!("For want of a {}", item);
            continue;
        }
        result = format!("{} the {} was lost.", result, item);
        if i != length - 1 {
            result = format!("{}\nFor want of a {}", result, item);
        }
    }
    format!("{}\n{}", result, last_line)
}
