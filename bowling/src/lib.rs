use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Frame {
    Open(u16, u16),       // A frame with two throws that did not knock down all pins
    Spare { first: u16 }, // A frame where all pins were knocked down in two throws
    Strike,               // A frame where all pins were knocked down in one throw
}

impl Default for Frame {
    fn default() -> Self {
        Self::Open(0, 0) // Default frame is an open frame with 0 pins knocked down
    }
}

impl Frame {
    fn first_throw(&self) -> u16 {
        match self {
            Frame::Open(fst, _) => *fst, // Return the first throw of an open frame
            Frame::Spare { first } => *first, // Return the first throw of a spare frame
            Frame::Strike => 10,         // A strike always knocks down 10 pins
        }
    }
    fn pin_count(&self) -> u16 {
        match self {
            Frame::Open(fst, snd) => *fst + *snd, // Sum of both throws in an open frame
            _ => 10,                              // Spare or strike always knock down 10 pins
        }
    }
}

#[derive(Debug, Default)]
struct FrameData {
    frame: Frame,               // The frame itself
    bonus_for_past_frames: u16, // Bonus points from previous frames
}

#[derive(Debug, Default)]
pub struct BowlingGame {
    frames: Vec<FrameData>,            // List of frames in the game
    current: Option<u16>,              // Current throw in progress
    bonus: (Option<u16>, Option<u16>), // Bonus throws for the 10th frame
}

impl BowlingGame {
    pub fn new() -> Self {
        Self {
            frames: Vec::with_capacity(11), // Pre-allocate space for 11 frames
            ..Default::default()
        }
    }

    fn is_done(&self) -> bool {
        if self.frames.len() < 10 {
            return false; // Game is not done if less than 10 frames
        }
        if self.frames[9].frame == Frame::Strike {
            return self.bonus.1.is_some(); // Game is done if 10th frame is a strike and both bonus throws are done
        }
        if let Frame::Spare { .. } = self.frames[9].frame {
            return self.bonus.0.is_some(); // Game is done if 10th frame is a spare and one bonus throw is done
        }
        true // Game is done if 10th frame is an open frame
    }

    fn is_in_bonus_round(&self) -> bool {
        self.frames.len() == 10 // Bonus round starts after 10 frames
    }

    fn bonus_roll(&mut self, pins: u16) -> Result<(), Error> {
        if let Some(first_bonus) = self.bonus.0 {
            // second bonus throw
            if self.frames[9].frame == Frame::Strike && first_bonus < 10 && first_bonus + pins > 10
            {
                return Err(Error::NotEnoughPinsLeft); // Error if total pins in bonus throws exceed 10
            }
            self.bonus.1 = Some(pins); // Set second bonus throw
        } else {
            self.bonus.0 = Some(pins); // Set first bonus throw
        };
        Ok(())
    }

    fn calc_bonus_for_past_frames(&self, new_frame: Frame) -> u16 {
        if let Some(FrameData { frame: prev, .. }) = self.frames.last() {
            match prev {
                Frame::Open(_, _) => 0,                         // No bonus for open frame
                Frame::Spare { .. } => new_frame.first_throw(), // Bonus for spare is the first throw of the new frame
                Frame::Strike if self.frames.len() < 2 => new_frame.pin_count(), // Bonus for strike is the total pins of the new frame
                Frame::Strike => {
                    new_frame.pin_count()
                        + match self.frames[self.frames.len() - 2].frame {
                            Frame::Strike => new_frame.first_throw(), // Additional bonus if previous frame was also a strike
                            _ => 0,
                        }
                }
            }
        } else {
            0
        }
    }

    fn mid_frame_roll(&mut self, current: u16, pins: u16) -> Result<(), Error> {
        let new_frame = match (current + pins).cmp(&10) {
            Ordering::Less => Frame::Open(current, pins), // Open frame if total pins are less than 10
            Ordering::Equal => Frame::Spare { first: current }, // Spare if total pins are exactly 10
            Ordering::Greater => return Err(Error::NotEnoughPinsLeft), // Error if total pins exceed 10
        };
        let bonus_points = self.calc_bonus_for_past_frames(new_frame); // Calculate bonus points for previous frames
        self.frames.push(FrameData {
            frame: new_frame,
            bonus_for_past_frames: bonus_points,
        });
        self.current = None; // Reset current throw
        Ok(())
    }

    fn new_frame_roll(&mut self, pins: u16) -> Result<(), Error> {
        if pins == 10 {
            self.frames.push(FrameData {
                frame: Frame::Strike, // Add a strike frame
                bonus_for_past_frames: self.calc_bonus_for_past_frames(Frame::Strike),
            });
        } else {
            self.current = Some(pins) // Set current throw if not a strike
        }
        Ok(())
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        match () {
            _ if self.is_done() => return Err(Error::GameComplete), // Error if game is already complete
            _ if pins > 10 => return Err(Error::NotEnoughPinsLeft), // Error if pins exceed 10
            _ => {}
        }
        match self.current {
            _ if self.is_in_bonus_round() => self.bonus_roll(pins), // Handle bonus round throws
            Some(current) => self.mid_frame_roll(current, pins), // Handle second throw of a frame
            _ => self.new_frame_roll(pins),                      // Handle first throw of a frame
        }
    }

    pub fn score(&self) -> Option<u16> {
        if !self.is_done() {
            return None; // Return None if game is not complete
        }

        let regular_points: u16 = self
            .frames
            .iter()
            .map(|fd| fd.frame.pin_count() + fd.bonus_for_past_frames) // Sum of all frame points and bonuses
            .sum();

        let extra_frame_bonus = match self.frames[9].frame {
            Frame::Spare { .. } => self.bonus.0.expect("not done yet!"), // Bonus for spare is the first bonus throw
            Frame::Strike => {
                self.bonus.0.expect("not done yet!")
                    + self.bonus.1.expect("not done yet!")
                    + match self.frames[8].frame {
                        Frame::Strike => self.bonus.0.unwrap(), // Additional bonus if 9th frame was also a strike
                        _ => 0,
                    }
            }
            _ => 0,
        };

        Some(regular_points + extra_frame_bonus) // Total score including extra frame bonus
    }
}
