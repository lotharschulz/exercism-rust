/// Price in cents of a group of `n` different books, indexed by `n`.
///
/// A single book costs 800 cents; groups of different books are discounted:
/// 2 books 5%, 3 books 10%, 4 books 20%, 5 books 25%.
const GROUP_PRICE: [u32; 6] = [0, 800, 1520, 2160, 2560, 3000];

/// Calculates the lowest possible price in cents for a basket of books.
///
/// Books are identified by numbers 1-5. Groups of different books receive
/// discounts, and the basket is split into the grouping that minimizes the
/// total price.
///
/// # Examples
/// ```
/// use book_store::lowest_price;
/// assert_eq!(lowest_price(&[1, 2]), 1520);
/// assert_eq!(lowest_price(&[1, 1, 2, 2, 3, 3, 4, 5]), 5120);
/// ```
#[must_use]
pub fn lowest_price(books: &[u32]) -> u32 {
    // Count copies of each title (IDs 1-5).
    let mut counts = books.iter().fold([0u32; 5], |mut counts, &book| {
        counts[(book as usize - 1) % 5] += 1;
        counts
    });

    // Greedily form the largest possible group on each pass: every title
    // with remaining copies contributes one book.
    let mut group_sizes = Vec::new();
    loop {
        let size = counts.iter().filter(|&&count| count > 0).count();
        if size == 0 {
            break;
        }
        counts
            .iter_mut()
            .filter(|count| **count > 0)
            .for_each(|count| *count -= 1);
        group_sizes.push(size);
    }

    // Two groups of four (5120) beat a group of five plus a group of three
    // (5160), so rebalance every 5+3 pair into 4+4.
    let (fives, threes) =
        group_sizes
            .iter()
            .fold((0u32, 0u32), |(fives, threes), &size| match size {
                5 => (fives + 1, threes),
                3 => (fives, threes + 1),
                _ => (fives, threes),
            });
    let rebalanced = fives.min(threes);

    group_sizes
        .iter()
        .map(|&size| GROUP_PRICE[size])
        .sum::<u32>()
        - (GROUP_PRICE[5] + GROUP_PRICE[3] - 2 * GROUP_PRICE[4]) * rebalanced
}
