#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist_if_else<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    if first_list == second_list {
        Comparison::Equal
    } else if second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|window| window == second_list)
    {
        Comparison::Superlist
    } else if first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|window| window == first_list)
    {
        Comparison::Sublist
    } else {
        Comparison::Unequal
    }
}

pub fn sublist_pattern_matching<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    let superlist = second_list.is_empty()
        || first_list
            .windows(second_list.len())
            .any(|window| window == second_list);
    let sublist = first_list.is_empty()
        || second_list
            .windows(first_list.len())
            .any(|window| window == first_list);
    match (superlist, sublist) {
        (true, true) => Comparison::Equal,
        (true, false) => Comparison::Superlist,
        (false, true) => Comparison::Sublist,
        _ => Comparison::Unequal,
    }
}
