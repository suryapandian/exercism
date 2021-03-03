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

    let mut repeat_index = 0usize;
    let mut second_list_index = 0usize;

    for (first_index, first_ele) in first_list.iter().enumerate() {
        if *first_ele != second_list[second_list_index] {
            second_list_index = 0;
            continue;
        }

        if second_list_index == (second_list_length - 1) {
            return true;
        }
        second_list_index += 1usize;

        if *first_ele == second_list[0] && repeat_index == 0 && first_index > 0 {
            repeat_index = first_index;
        }
    }

    if repeat_index > 0 {
        return superlist(&first_list[repeat_index..first_list.len()], second_list);
    }
    false
}
