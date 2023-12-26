use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
enum Card{
    Number(u8),
    Jack,
    Queen,
    King,
    Ace
}

impl Card {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Card::Number(10)),
            '2'..='9' => Some(Card::Number(c.to_digit(10).unwrap() as u8)),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

impl From<Card> for u8 {
    fn from(origin: Card) -> u8 {
        match origin {
            Card::Ace => 14,
            Card::King => 13,
            Card::Queen => 12,
            Card::Jack => 11,
            Card::Number(x) => x
        }
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq)]
enum Hand{
    HighCard(Vec<Card>),
    OnePair(Vec<Card>, Card),
    TwoPair(Vec<Card>, Card),
    ThreeOfAKind(Vec<Card>, Card),
    Straight(Vec<Card>,u8),
    Flush(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    StraightFlush(Vec<Card>)
}

impl Hand {
    fn from_cards(mut cards: Vec<Card>) -> Self {
        let counts = cards
            .iter()
            .fold(HashMap::<Card, u32>::new(), |mut acc, c| {
                *acc.entry(*c).or_default() += 1;
                acc
            });
        let mut sorted_counts: Vec<u32> = counts
            .iter()
            .map(|(_k, &v)| v)
            .collect();

        cards.sort_unstable_by(|a, b| Ord::cmp(&a, &b));
        sorted_counts.sort_unstable_by(|a, b| Ord::cmp(&b, &a));

        match sorted_counts.as_slice() {
            [4, ..] => Hand::FourOfAKind(cards),
            [3,2, ..] => Hand::FullHouse(cards),
            [3, ..] => {
                let max_three_pair_cards = *counts.iter().filter(|(_, &count)| count == 3).max().unwrap().0;
                Hand::ThreeOfAKind(cards, max_three_pair_cards)
            },
            [2,2, ..] => {
                let max_two_pair_cards = *counts.iter().filter(|(_, &count)| count == 2).max().unwrap().0;
                Hand::TwoPair(cards, max_two_pair_cards)
            },
            [2, ..] => {
                let one_pair_card = *counts.iter().max_by_key(|(_, &count)| count).unwrap().0;
                Hand::OnePair(cards, one_pair_card)
            },
            [1, ..] => {
                let total_seq_of_cards: u8 = cards
                    .windows(2)
                    .filter_map(|c|{
                        let v1: u8 = c[0].into();
                        let v2: u8 = c[1].into();
                        let seq = v2.saturating_sub(v1);
                        // seq >= 4 if there is an ace card in a sequence, consider it a straight
                        match seq {
                            1 => Some(1),
                            9 if c[1] == Card::Ace => Some(1),
                            _ => None
                        }
                    })
                    .sum::<u8>();

                if total_seq_of_cards == 4 {
                    Hand::Straight(cards, total_seq_of_cards)
                } else {
                    Hand::HighCard(cards)
                }
            }
            _ => panic!("Can't build hand from {cards:?} and {sorted_counts:?}")
        }

    }
    fn compare_against_high_cards(hand_one: &[Card], hand_two: &[Card]) -> Ordering {
        let chain = hand_one
            .iter()
            .zip(hand_two)
            .collect::<Vec<(&Card, &Card)>>();

        let mut c1_count: u8 = 0;
        let mut c2_count: u8 = 0;

        for (&c1, &c2) in chain.iter() {
            match c1.cmp(&c2) {
                Ordering::Greater => {
                    c1_count += 1;
                    c2_count = c2_count.saturating_sub(1);
                },
                Ordering::Less => {
                    c2_count += 1;
                    c1_count = c1_count.saturating_sub(1);
                }
                _ => ()
            }
        }
        if c1_count.cmp(&c2_count) == Ordering::Equal {
            hand_one.iter().last().cmp(&hand_two.iter().last())
        } else {
            c1_count.cmp(&c2_count)
        }
    }
    fn compare_against_pair_cards(hand_one: &[Card], max_pair_card_1: &Card, hand_two: &[Card], max_pair_card_2: &Card) -> Ordering {
        if max_pair_card_1.cmp(max_pair_card_2) == Ordering::Equal {

            let kicker_cards_vec_1 = hand_one.iter().filter(|&c| c != max_pair_card_1).copied().collect::<Vec<Card>>();
            let kicker_cards_vec_2 = hand_two.iter().filter(|&c| c != max_pair_card_2).copied().collect::<Vec<Card>>();

            Hand::compare_against_high_cards(&kicker_cards_vec_1, &kicker_cards_vec_2)
        } else {
            max_pair_card_1.cmp(max_pair_card_2)
        }
    }
}


pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {

    let hset: HashSet<_> = hands
        .iter()
        .fold(HashSet::new(), |mut acc, &x| {
            acc.insert(x);
            acc
        });

    if hset.len() == 1 {
        hands.to_vec()
    } else {
        let vec_hands = hands
            .iter()
            .map(|&h| {
                Hand::from_cards(
                    h
                        .chars()
                        .filter_map(|c|{
                            Card::from_char(c)
                        })
                        .collect::<Vec<Card>>()
                )
            })
            .collect::<Vec<Hand>>();

        let zip_hands: Vec<(Hand, &str)> = vec_hands
            .into_iter()
            .zip(hands)
            .map(|(h, &o)|{
                (h, o)
            })
            .collect();

        zip_hands
            .iter()
            .max_set_by(|(h1, _), (|h2, _)| {
                match (h1, h2) {
                    (Hand::Straight(c1_card, c1_seq), Hand::Straight(c2_card, c2_seq)) => {
                        if c1_seq.cmp(c2_seq) == Ordering::Equal {
                            c1_card.cmp(c2_card)
                        } else {
                            c1_seq.cmp(c2_seq)
                        }
                    },
                    (Hand::HighCard(c1), Hand::HighCard(c2)) => {
                        Hand::compare_against_high_cards(c1, c2)
                    },
                    (Hand::OnePair(v1, one_pair_card_1), Hand::OnePair(v2, one_pair_card_2)) => {
                        Hand::compare_against_pair_cards(v1, one_pair_card_1, v2, one_pair_card_2)
                    },
                    (Hand::TwoPair(v1, two_pair_card_1), Hand::TwoPair(v2, two_pair_card_2)) => {
                        Hand::compare_against_pair_cards(v1, two_pair_card_1, v2, two_pair_card_2)
                    },
                    (Hand::ThreeOfAKind(v1, three_pair_card_1), Hand::ThreeOfAKind(v2, three_pair_card_2)) => {
                        Hand::compare_against_pair_cards(v1, three_pair_card_1, v2, three_pair_card_2)
                    }
                    _ => h1.cmp(h2)
                }
            })
            .iter()
            .map(|(_, h)| *h)
            .collect::<Vec<&str>>()

    }
    // todo!("Out of {hands:?}, which hand wins?")
}
