use std::collections::{HashMap, HashSet};

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
    Straight(u32),
    Flush(Vec<Card>),
    FullHouse(Vec<Card>),
    FourOfAKind(Vec<Card>),
    StraightFlush(Vec<Card>)
}

impl Hand {
    fn from_cards(cards: Vec<Card>) -> Self {
        let mut counts = cards
            .iter()
            .fold(HashMap::<Card, u32>::new(), |mut acc, c| {
                *acc.entry(*c).or_default() += 1;
                acc
            });
        let sorted_counts: Vec<u32> = counts
            .drain()
            .map(|(_k, v)| v)
            .collect();

        match sorted_counts.as_slice() {
            [4, ..] => Hand::FourOfAKind(cards),
            [3,2, ..] => Hand::FullHouse(cards),
            [3, ..] => Hand::ThreeOfAKind(cards),
            [2,2, ..] => Hand::TwoPair(cards),
            [2, ..] => Hand::OnePair(cards),
            [1, ..] => {
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

                if total_seq_of_cards > 0 {
                    Hand::Straight(total_seq_of_cards + 1)
                } else {
                    Hand::OnePair(cards)
                }

            },
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

        let best_hands = zip_hands.iter().max_by(|(h1, _), (|h2, _)| h1.cmp(h2)).unwrap().1;
        vec![best_hands]
    }
    // todo!("Out of {hands:?}, which hand wins?")
}
