/// Scoring categories for the Yacht dice game.
///
/// Each category represents a different scoring rule for evaluating a hand of five dice.
#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

/// Type alias for a hand of five dice.
type Dice = [u8; 5];

/// Scores a hand of dice according to the specified category.
///
/// # Arguments
/// * `dice` - An array of five die values (1-6)
/// * `category` - The scoring category to apply
///
/// # Returns
/// The score for the given hand and category.
pub fn score(dice: Dice, category: Category) -> u8 {
    match category {
        Category::Ones => score_number(dice, 1),
        Category::Twos => score_number(dice, 2),
        Category::Threes => score_number(dice, 3),
        Category::Fours => score_number(dice, 4),
        Category::Fives => score_number(dice, 5),
        Category::Sixes => score_number(dice, 6),
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if dice.iter().all(|&x| x == dice[0]) {
                50
            } else {
                0
            }
        }
        Category::FullHouse => score_full_house(dice),
        Category::FourOfAKind => score_four_of_a_kind(dice),
        Category::LittleStraight => score_little_straight(dice),
        Category::BigStraight => score_big_straight(dice),
    }
}

/// Scores a hand for a specific number (1-6).
///
/// Sums all dice matching the specified number.
///
/// # Arguments
/// * `dice` - An array of five die values
/// * `number` - The number to match (1-6)
///
/// # Returns
/// The sum of all dice matching the specified number.
fn score_number(dice: Dice, number: u8) -> u8 {
    dice.iter().filter(|&&x| x == number).sum()
}

/// Scores a hand as a full house (three of a kind and a pair).
///
/// A full house consists of three dice showing one number and two dice showing another number.
/// If valid, returns the sum of all dice; otherwise returns 0.
///
/// # Arguments
/// * `dice` - An array of five die values
///
/// # Returns
/// The sum of all dice if a full house is present, 0 otherwise.
fn score_full_house(dice: Dice) -> u8 {
    let mut sorted = dice;
    sorted.sort_unstable();
    // Full house is: three of one kind and two of another
    let is_full_house =
        (sorted[0] == sorted[2] && sorted[3] == sorted[4] && sorted[2] != sorted[3])
            || (sorted[0] == sorted[1] && sorted[2] == sorted[4] && sorted[1] != sorted[2]);
    if is_full_house { dice.iter().sum() } else { 0 }
}

/// Scores a hand as four of a kind.
///
/// Four of a kind requires at least four dice showing the same number.
/// Returns 4 times that number if present, 0 otherwise.
///
/// # Arguments
/// * `dice` - An array of five die values
///
/// # Returns
/// Four times the value of the matching number, or 0 if no four of a kind exists.
fn score_four_of_a_kind(dice: Dice) -> u8 {
    let mut counts = [0u8; 7];
    for &d in &dice {
        counts[d as usize] += 1;
    }
    // Check if any number appears 4 or 5 times
    for (i, &count) in counts.iter().enumerate().skip(1) {
        if count >= 4 {
            return (i as u8) * 4;
        }
    }
    0
}

/// Scores a hand as a little straight [1, 2, 3, 4, 5].
///
/// A little straight requires all five dice showing the values 1 through 5 in any order.
/// If valid, returns 30; otherwise returns 0.
///
/// # Arguments
/// * `dice` - An array of five die values
///
/// # Returns
/// 30 if a little straight is present, 0 otherwise.
fn score_little_straight(dice: Dice) -> u8 {
    // Little straight is [1, 2, 3, 4, 5]
    let mut sorted = dice;
    sorted.sort_unstable();
    if sorted == [1, 2, 3, 4, 5] { 30 } else { 0 }
}

/// Scores a hand as a big straight [2, 3, 4, 5, 6].
///
/// A big straight requires all five dice showing the values 2 through 6 in any order.
/// If valid, returns 30; otherwise returns 0.
///
/// # Arguments
/// * `dice` - An array of five die values
///
/// # Returns
/// 30 if a big straight is present, 0 otherwise.
fn score_big_straight(dice: Dice) -> u8 {
    // Big straight is [2, 3, 4, 5, 6]
    let mut sorted = dice;
    sorted.sort_unstable();
    if sorted == [2, 3, 4, 5, 6] { 30 } else { 0 }
}
