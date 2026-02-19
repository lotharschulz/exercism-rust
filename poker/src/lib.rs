/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let mut winners = Vec::new();
    let mut best_value: Option<HandValue> = None;

    for &hand in hands {
        // Compute a comparable value for each hand once.
        let value = rank_hand(hand);
        match best_value.as_ref() {
            None => {
                best_value = Some(value);
                winners.push(hand);
            }
            Some(best) if value > *best => {
                // New best hand found: reset winner list.
                best_value = Some(value);
                winners.clear();
                winners.push(hand);
            }
            // Same strength and tie-break sequence: keep all winners.
            Some(best) if value == *best => winners.push(hand),
            Some(_) => {}
        }
    }

    winners
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd)]
struct HandValue {
    category: HandCategory,
    tiebreak: Vec<u8>,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
enum HandCategory {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, Copy, Clone)]
struct Card {
    rank: u8,
    suit: char,
}

fn rank_hand(hand: &str) -> HandValue {
    // Parse fixed-size 5-card hand from the input string.
    let cards = parse_hand(hand);

    // Count occurrences by rank index (2..=14, where Ace is 14).
    let mut rank_counts = [0_u8; 15];
    for card in cards {
        rank_counts[usize::from(card.rank)] += 1;
    }

    // Precompute descending ranks for high-card style comparisons.
    let mut ranks_desc = cards.map(|card| card.rank).to_vec();
    ranks_desc.sort_unstable_by(|a, b| b.cmp(a));

    let is_flush = cards.iter().all(|card| card.suit == cards[0].suit);
    // Returns the straight high card if this hand is a straight.
    let straight_high = straight_high_card(&rank_counts);

    // (count, rank) groups, sorted by count first then rank, both descending.
    let mut groups = rank_groups(&rank_counts);
    groups.sort_unstable_by(|a, b| b.cmp(a));

    // Straight flush and flush are decided before rank-group based categories.
    if is_flush {
        if let Some(high) = straight_high {
            return HandValue {
                category: HandCategory::StraightFlush,
                tiebreak: vec![high],
            };
        }

        return HandValue {
            category: HandCategory::Flush,
            tiebreak: ranks_desc,
        };
    }

    if let Some(high) = straight_high {
        return HandValue {
            category: HandCategory::Straight,
            tiebreak: vec![high],
        };
    }

    // Remaining categories are determined by rank multiplicities.
    match groups.as_slice() {
        [(4, quad), (1, kicker)] => HandValue {
            category: HandCategory::FourOfAKind,
            tiebreak: vec![*quad, *kicker],
        },
        [(3, triplet), (2, pair)] => HandValue {
            category: HandCategory::FullHouse,
            tiebreak: vec![*triplet, *pair],
        },
        [(3, triplet), (1, first_kicker), (1, second_kicker)] => {
            let mut kickers = vec![*first_kicker, *second_kicker];
            kickers.sort_unstable_by(|a, b| b.cmp(a));

            let mut tiebreak = vec![*triplet];
            tiebreak.extend(kickers);

            HandValue {
                category: HandCategory::ThreeOfAKind,
                tiebreak,
            }
        }
        [(2, first_pair), (2, second_pair), (1, kicker)] => {
            let (high_pair, low_pair) = if first_pair > second_pair {
                (*first_pair, *second_pair)
            } else {
                (*second_pair, *first_pair)
            };

            HandValue {
                category: HandCategory::TwoPair,
                tiebreak: vec![high_pair, low_pair, *kicker],
            }
        }
        [(2, pair), (1, kicker_a), (1, kicker_b), (1, kicker_c)] => {
            let mut kickers = vec![*kicker_a, *kicker_b, *kicker_c];
            kickers.sort_unstable_by(|a, b| b.cmp(a));

            let mut tiebreak = vec![*pair];
            tiebreak.extend(kickers);

            HandValue {
                category: HandCategory::OnePair,
                tiebreak,
            }
        }
        _ => HandValue {
            category: HandCategory::HighCard,
            tiebreak: ranks_desc,
        },
    }
}

fn parse_hand(hand: &str) -> [Card; 5] {
    let mut cards = hand.split_whitespace().map(parse_card);

    // Input format guarantees exactly five cards.
    [
        cards.next().expect("missing card 1"),
        cards.next().expect("missing card 2"),
        cards.next().expect("missing card 3"),
        cards.next().expect("missing card 4"),
        cards.next().expect("missing card 5"),
    ]
}

fn parse_card(token: &str) -> Card {
    // Last character is suit; prefix is rank text.
    let suit = token.chars().last().expect("card token is empty");
    let rank = match &token[..token.len() - 1] {
        "A" => 14,
        "K" => 13,
        "Q" => 12,
        "J" => 11,
        "10" => 10,
        "9" => 9,
        "8" => 8,
        "7" => 7,
        "6" => 6,
        "5" => 5,
        "4" => 4,
        "3" => 3,
        "2" => 2,
        _ => panic!("invalid rank in card: {token}"),
    };

    Card { rank, suit }
}

fn rank_groups(rank_counts: &[u8; 15]) -> Vec<(u8, u8)> {
    let mut groups = Vec::new();
    // Only ranks that appear in the hand are included.
    for rank in 2..=14 {
        let count = rank_counts[rank as usize];
        if count > 0 {
            groups.push((count, rank));
        }
    }
    groups
}

fn straight_high_card(rank_counts: &[u8; 15]) -> Option<u8> {
    let mut unique_ranks = Vec::with_capacity(5);
    for rank in 2..=14 {
        if rank_counts[rank as usize] > 0 {
            unique_ranks.push(rank);
        }
    }

    if unique_ranks.len() != 5 {
        // Duplicates cannot form a straight.
        return None;
    }

    // Wheel straight: A-2-3-4-5 counts as 5-high.
    if unique_ranks == [2, 3, 4, 5, 14] {
        return Some(5);
    }

    let is_consecutive = unique_ranks.windows(2).all(|pair| pair[1] == pair[0] + 1);

    if is_consecutive {
        unique_ranks.last().copied()
    } else {
        None
    }
}
