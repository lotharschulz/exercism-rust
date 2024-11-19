const CHILDRENS: [&str; 12] = [
    "Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph",
    "Kincaid", "Larry",
];

// public function named `plants` that takes two parameters:
// `_diagram` is a reference to a string slice representing the garden diagram.
// `_student` is a reference to a string slice representing the student's name.
// The function returns a vector of string slices.
pub fn plants(_diagram: &str, _student: &str) -> Vec<&'static str> {
    // Find the position of the student in the CHILDRENS array and multiply by 2 to get the starting index
    let position = CHILDRENS.iter().position(|&n| n == _student).unwrap() * 2;

    // Function to map characters to plant names
    let apply_char = |c: char| match c {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => "",
    };

    // Iterate over each line in the diagram, take the two characters
    // corresponding to the student's plants,
    // map them to plant names using apply_char,
    // and collect the results into a vector
    _diagram
        .lines()
        .flat_map(|line| line[position..=position + 1].chars().map(apply_char))
        .collect()
}
