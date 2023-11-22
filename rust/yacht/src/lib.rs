use std::collections::{HashMap, HashSet};

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

impl Category {
    fn as_u8(&self) -> u8 {
        match self {
            Category::Ones => 1,
            Category::Twos => 2,
            Category::Threes => 3,
            Category::Fours => 4,
            Category::Fives => 5,
            Category::Sixes => 6,
            Category::Yacht => 1,
            Category::LittleStraight => 15,
            _ => panic!("Incorrect Category")
        }
    }
}

type Dice = [u8; 5];
pub fn score(_dice: Dice, _category: Category) -> u8 {
    let hashset = _dice.into_iter().collect::<HashSet<u8>>();

    let hmap = _dice
        .iter()
        .fold(HashMap::<u8, u8>::new(), |mut acc, n| {
            *acc.entry(*n).or_insert(0) += 1;
            acc
        });

    match _category {
        Category::Yacht => {
            if hashset.len() == _category.as_u8() as usize {
                50
            } else {
                0
            }
        },
        Category::Ones | Category::Twos | Category::Threes | Category::Fours | Category::Fives | Category::Sixes => {
            _dice.into_iter().filter(|&n| n == _category.as_u8()).sum::<u8>()
        },
        Category::FullHouse => {
            let mut values = hmap.values().collect::<Vec<&u8>>();
            values.sort_unstable();

            if hmap.len() == 2 && values == vec![&2u8, &3u8]{
                _dice.iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            let highest_count_of_number_tuple = hmap.iter().max_by_key(|(_,&y)|y).unwrap();
            if highest_count_of_number_tuple.1 >= &4 {
                highest_count_of_number_tuple.0 * 4
            } else {
                0
            }
        }
        _ => todo!("Fill in here")
    }
}
