pub fn find<T: Ord, V: AsRef<[T]>>(array: V, item: T) -> Option<usize> {
    // T: Ord - The type T must implement the Ord trait, which allows comparison
    // V: AsRef<[T]> - The type V must be convertible to a slice of T
    // array: V - The input array, which can be converted to a slice of T
    // item: T - The item to search for in the array
    // -> Option<usize> - The function returns an Option containing the index of the found item, or None if not found

    let a = array.as_ref(); // Convert the input into a slice
    let mut low = 0; // Initialize the lower bound of the search range
    let mut high = a.len(); // Initialize the upper bound of the search range

    while low < high {
        // Continue searching while the range is valid
        let mid = (low + high) / 2; // Calculate the middle index
        let mid_val = &a[mid]; // Get the value at the middle index

        if mid_val > &item {
            // If the middle value is greater than the item
            high = mid; // Narrow the search range to the lower half
        } else if mid_val < &item {
            // If the middle value is less than the item
            low = mid + 1; // Narrow the search range to the upper half
        } else {
            // If the middle value is equal to the item
            return Some(mid); // Return the index of the found item
        }
    }
    None // Return None if the item is not found
}
