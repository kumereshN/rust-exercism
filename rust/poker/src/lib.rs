use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use itertools::Itertools;
/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
#[derive(Copy, Clone, PartialOrd, PartialEq, Ord, Eq, Hash, Debug)]
enum Card{
    Number(u32),
    Jack,
    Queen,
    King,
    Ace
}

impl Card {
    fn from_char(c: char) -> Option<Self> {
        match c {
            '1' => Some(Card::Number(10)),
            '2'..='9' => Some(Card::Number(c.to_digit(10).unwrap())),
            'J' => Some(Card::Jack),
            'Q' => Some(Card::Queen),
            'K' => Some(Card::King),
            'A' => Some(Card::Ace),
            _ => None,
        }
    }
}

#[derive(PartialOrd, PartialEq, Ord, Eq)]
enum Hand{
    HighCard(Vec<Card>),
    OnePair(Vec<Card>),
    TwoPair(Vec<Card>),
    ThreeOfAKind(Vec<Card>),
    Straight((Vec<Card>,u32)),
    Flush(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    StraightFlush(Vec<Card>)
}

impl Hand {
    fn from_cards(mut cards: Vec<Card>) -> Self {
        let mut counts = cards
            .iter()
            .fold(HashMap::<Card, u32>::new(), |mut acc, c| {
                *acc.entry(*c).or_default() += 1;
                acc
            });
        let mut sorted_counts: Vec<u32> = counts
            .drain()
            .map(|(_k, v)| v)
            .collect();

        sorted_counts.sort_unstable_by(|a, b| Ord::cmp(&b, &a));

        match sorted_counts.as_slice() {
            [4, ..] => Hand::FourOfAKind(cards),
            [3,2, ..] => Hand::FullHouse(cards),
            [3, ..] => Hand::ThreeOfAKind(cards),
            [2,2, ..] => Hand::TwoPair(cards),
            [2, ..] => Hand::OnePair(cards),
            [1, ..] => {
                cards.sort_unstable_by(|a, b| Ord::cmp(&a, &b));
                let total_seq_of_cards: u32 = cards
                    .windows(2)
                    .filter_map(|c|{
                        let (c1, c2) = (c[0], c[1]);
                        match (c1, c2) {
                            (Card::Number(x), Card::Number(y)) => {
                                let seq = y.saturating_sub(x);
                                if seq == 1 {
                                    Some(seq)
                                } else {
                                    None
                                }
                            }
                            _ => None
                        }
                    })
                    .sum();

                if total_seq_of_cards == 5 {
                    Hand::Straight((cards, total_seq_of_cards + 1))
                } else {
                    Hand::HighCard(cards)
                }
            }
            _ => panic!("Can't build hand from {cards:?} and {sorted_counts:?}")
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
                    (Hand::Straight(c1), Hand::Straight(c2)) => {
                        let (c1_card, c1_seq) = (&c1.0, c1.1);
                        let (c2_card, c2_seq) = (&c2.0, c2.1);
                        if c1_seq.cmp(&c2_seq) == Ordering::Equal {
                            c1_card.cmp(c2_card)
                        } else {
                            c1_seq.cmp(&c2_seq)
                        }
                    },
                    (Hand::HighCard(c1), Hand::HighCard(c2)) => {
                        let chain = c1
                            .iter()
                            .zip(c2)
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
                           c1.iter().last().cmp(&c2.iter().last())
                       } else {
                           c1_count.cmp(&c2_count)
                       }
                    },
                    _ => h1.cmp(h2)
                }
            })
            .iter()
            .map(|(_, h)| *h)
            .collect::<Vec<&str>>()

    }
    // todo!("Out of {hands:?}, which hand wins?")
}
