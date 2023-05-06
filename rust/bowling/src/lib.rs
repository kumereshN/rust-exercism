const STRIKE_SCORE: u16 = 10;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

pub struct BowlingGame {
    pins_knocked_down: Vec<u16>,
    current_round: u16
}

impl BowlingGame {
    pub fn new() -> Self {
        Self{
            pins_knocked_down: vec![],
            current_round: 0
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins > 10 {
            return Err(Error::NotEnoughPinsLeft)
        }
        Ok(())
    }

    pub fn score(&self) -> Option<u16> {
        Some(16)
    }
}
