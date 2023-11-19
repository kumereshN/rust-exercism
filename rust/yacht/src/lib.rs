use std::collections::HashSet;

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
            _ => panic!("Incorrect Category")
        }
    }
}

type Dice = [u8; 5];
pub fn score(_dice: Dice, _category: Category) -> u8 {
    let hashset = _dice.into_iter().collect::<HashSet<u8>>();

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
            if hashset.len() == 2 {
                let first_number = *hashset.iter().last().unwrap();
                let count_of_first_number = _dice.into_iter().filter(|&n| n == first_number).count();
                if matches!(count_of_first_number, 2 | 3) {
                    _dice.iter().sum::<u8>()
                } else {
                    0
                }
            } else {
                0
            }
        }
        _ => todo!("Fill in here")
    }
}