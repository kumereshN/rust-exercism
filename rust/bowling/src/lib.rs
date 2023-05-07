#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}
#[derive(Debug, PartialEq)]
enum Frame {
    Open,
    Spare,
    Strike
}
#[derive(Debug, PartialEq)]
pub enum RollingState {
    First(u8),
    Second(u8, u16),
    Complete
}
pub struct BowlingGame {
    frames: Vec<Frame>,
    rolls: Vec<u16>,
    rolling_state: RollingState
}

use RollingState::*;
use Frame::*;
use Error::*;

impl Default for BowlingGame {
    fn default() -> Self {
        BowlingGame::new()
    }
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: vec![],
            rolls: vec![],
            rolling_state: First(1)
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match self.rolling_state {
            // Can't roll if game is complete.
            Complete => return Err(GameComplete),

            // Can't roll if there are not enough pins.
            First(_) if pins > 10 => return Err(NotEnoughPinsLeft),
            Second(_, n) if n + pins > 10 => return Err(NotEnoughPinsLeft),

            // Game is complete if Open on 10th frame, or after the final extra roll.
            Second(10, n) if n + pins < 10 => {
                self.frames.push(Open);
                self.rolling_state = Complete;
            },
            Second(11, _) => {
                self.rolling_state = Complete;
            }

            // We roll a second time if we don't get a Strike with the first roll.
            First(f) if pins < 10 => {
                self.rolling_state = Second(f, pins);
            },

            // Less than 10 pins on the second roll is an Open.
            Second(f, n) if n + pins < 10 => {
                self.frames.push(Open);
                self.rolling_state = First(f + 1);
            },

            // This is the extra roll on the virtual bonus frame that occurs if we get a strike on the 10th frame
            // We get a second bonus roll after it.
            First(11) if pins == 10 => {
                self.rolling_state = Second(11, 0);
            }

            // All pins on first roll is Strike.
            First(f) if pins == 10 => {
                self.frames.push(Strike);
                self.rolling_state = First(f + 1);
            },

            // Spare on 10th frame leads to a single bonus attempt
            // (or a second roll of a virtual bonus frame with 0 on first roll).
            Second(10, n) if n + pins == 10 => {
                self.frames.push(Spare);
                self.rolling_state = Second(11, 0);
            },

            // All pins on the second attempt is a Spare
            Second(f, n) if n + pins == 10 => {
                self.frames.push(Spare);
                self.rolling_state = First(f + 1);
            },

            _ => unreachable!()
        }
        self.rolls.push(pins);
        Ok(())
    }


    pub fn score(&self) -> Option<u16> {
        if self.rolling_state != Complete {
            return None;
        }

         let mut res = 0;
         let mut i = 0;

        for frame in &self.frames {
            match frame {
                Open => {
                    let a = self.rolls.get(i)?;
                    let b = self.rolls.get(i+1)?;
                    res += a + b;
                    i += 2;
                }
                Spare => {
                    let a = self.rolls.get(i+2)?;
                    res += 10 + a;
                    i += 2;
                }
                Strike => {
                    let a = self.rolls.get(i+1)?;
                    let b = self.rolls.get(i+2)?;
                    res += 10 + a + b;
                    i += 1;
                }
            }
        }
        Some(res)
    }
}


