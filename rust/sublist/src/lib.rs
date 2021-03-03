#[derive(Debug, PartialEq)]

pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::cmp::Eq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        return Comparison::Equal;
    }
    if first_list.len() > second_list.len() {
        if superlist(first_list, second_list) {
            return Comparison::Superlist;
        }
        return Comparison::Unequal;
    }

    if superlist(second_list, first_list) {
        return Comparison::Sublist;
    }
    Comparison::Unequal
}

fn superlist<T: PartialEq + std::cmp::Eq>(first_list: &[T], second_list: &[T]) -> bool {
    let second_list_length = second_list.len();
    if second_list_length == 0 {
        return true;
    }

    let first_list_windows = first_list.windows(second_list.len());
    for first_list_window in first_list_windows {
        if first_list_window == second_list {
            return true;
        }
    }
    false
}
