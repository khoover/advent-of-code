use anyhow::{Context, Error, Result};
use aoc_runner_derive::aoc;
use nom::{
    bytes::complete::take,
    character::complete::{newline, space1, u16},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
    Finish, Parser,
};

use super::utils::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Cards {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Cards {
    fn from_byte(b: u8, use_joker: bool) -> Option<Self> {
        use Cards::*;
        match b {
            b'2' => Some(Two),
            b'3' => Some(Three),
            b'4' => Some(Four),
            b'5' => Some(Five),
            b'6' => Some(Six),
            b'7' => Some(Seven),
            b'8' => Some(Eight),
            b'9' => Some(Nine),
            b'T' => Some(Ten),
            b'J' => Some(if use_joker { Joker } else { Jack }),
            b'Q' => Some(Queen),
            b'K' => Some(King),
            b'A' => Some(Ace),
            _ => None,
        }
    }

    fn nom_hand(input: &str, use_joker: bool) -> StrIResult<'_, [Cards; 5]> {
        take(5_usize)
            .map_res(|s: &str| {
                let bytes = s.as_bytes();
                Ok::<[Cards; 5], Error>([
                    Self::from_byte(bytes[0], use_joker).context("Bad byte")?,
                    Self::from_byte(bytes[1], use_joker).context("Bad byte")?,
                    Self::from_byte(bytes[2], use_joker).context("Bad byte")?,
                    Self::from_byte(bytes[3], use_joker).context("Bad byte")?,
                    Self::from_byte(bytes[4], use_joker).context("Bad byte")?,
                ])
            })
            .parse(input)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Hand {
    hand_type: HandType,
    cards: [Cards; 5],
}

impl Hand {
    fn new(cards: [Cards; 5]) -> Self {
        let mut sorted_cards = cards;
        sorted_cards.sort_unstable();
        let mut hand_type = if sorted_cards[0] == sorted_cards[4] {
            HandType::FiveKind
        } else if sorted_cards[0] == sorted_cards[3] || sorted_cards[1] == sorted_cards[4] {
            HandType::FourKind
        } else if (sorted_cards[0] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4])
            || (sorted_cards[0] == sorted_cards[1] && sorted_cards[2] == sorted_cards[4])
        {
            HandType::FullHouse
        } else if sorted_cards.windows(3).any(|window| window[0] == window[2]) {
            HandType::ThreeKind
        } else if (sorted_cards[0] == sorted_cards[1]
            && (sorted_cards[2] == sorted_cards[3] || sorted_cards[3] == sorted_cards[4]))
            || (sorted_cards[1] == sorted_cards[2] && sorted_cards[3] == sorted_cards[4])
        {
            HandType::TwoPair
        } else if sorted_cards.windows(2).any(|pair| pair[0] == pair[1]) {
            HandType::Pair
        } else {
            HandType::HighCard
        };
        if cards.iter().copied().any(|c| c == Cards::Joker) {
            hand_type = Self::joker_adjust(hand_type, sorted_cards);
        }
        Self { hand_type, cards }
    }

    fn joker_adjust(hand_type: HandType, sorted_cards: [Cards; 5]) -> HandType {
        use HandType::*;

        match hand_type {
            HighCard => Pair, // High card means only 1 joker, which turns into one of the other 4 cards to make a pair
            Pair => ThreeKind, // 1 or 2 jokers, 1 means it turns into 3rd of pair, 2 means it is the pair and both turn to same other for 3
            ThreeKind => FourKind, // 1 or 3 jokers, same idea; can't be 2 because full house then
            FourKind => FiveKind, // 1 or 4 jokers
            FiveKind => FiveKind, // Everything's a joker
            TwoPair => {
                if sorted_cards
                    .windows(2)
                    .filter_map(|pair| (pair[0] == pair[1]).then_some(pair[0]))
                    .any(|c| c == Cards::Joker)
                {
                    FourKind
                } else {
                    FullHouse
                }
            }
            FullHouse => FiveKind,
        }
    }
}

#[aoc(day7, part1)]
fn day7_part1(input: &str) -> Result<u64> {
    let mut hands = all_consuming(separated_list1(
        newline,
        separated_pair((|s| Cards::nom_hand(s, false)).map(Hand::new), space1, u16),
    ))
    .parse_complete(input)
    .finish()
    .map(|(_, x)| x)
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))?;

    hands.sort_unstable();
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(idx, x)| (idx + 1, x))
        .map(|(rank, (_, bid))| rank as u64 * bid as u64)
        .sum())
}

#[aoc(day7, part2)]
fn day7_part2(input: &str) -> Result<u64> {
    let mut hands = all_consuming(separated_list1(
        newline,
        separated_pair((|s| Cards::nom_hand(s, true)).map(Hand::new), space1, u16),
    ))
    .parse_complete(input)
    .finish()
    .map(|(_, x)| x)
    .map_err(|e| Error::msg(format!("Failed to parse input:\n{e:#?}")))?;

    hands.sort_unstable();
    Ok(hands
        .into_iter()
        .enumerate()
        .map(|(idx, x)| (idx + 1, x))
        .map(|(rank, (_, bid))| rank as u64 * bid as u64)
        .sum())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hand_ord() {
        use Cards::*;
        let hands = [
            Hand::new([Three, Two, Ten, Three, King]),
            Hand::new([King, Ten, Jack, Jack, Ten]),
            Hand::new([King, King, Six, Seven, Seven]),
            Hand::new([Ten, Five, Five, Jack, Five]),
            Hand::new([Queen, Queen, Queen, Jack, Ace]),
        ];
        assert!(
            hands[0] < hands[1],
            "{:?} should be less than {:?}",
            hands[0],
            hands[1]
        );
        assert!(
            hands[1] < hands[2],
            "{:?} should be less than {:?}",
            hands[1],
            hands[2]
        );
        assert!(
            hands[2] < hands[3],
            "{:?} should be less than {:?}",
            hands[2],
            hands[3]
        );
        assert!(
            hands[3] < hands[4],
            "{:?} should be less than {:?}",
            hands[3],
            hands[4]
        );
    }
}
