pub fn build_proverb(list: &[&str]) -> String {
    match list.first() {
        // If the list is empty, return an empty string.
        None => String::new(),
        // If the list is not empty, return a string with the following format:
        Some(word) => list.windows(2)
            .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
            // Creates an iterator that yields an element exactly once.
            // Adapt a single value into a chain of other kinds of iteration.
            .chain(std::iter::once(format!("And all for the want of a {}.",word)))
            .collect::<Vec<_>>()
            .join("\n"),
    }
}
