use crate::noise_maker::FreqType;
use derive_more::Display;
use std::fmt;

#[derive(Clone, Copy, Display)]
pub enum NoteLetter {
    C = 0,
    D = 2,
    E = 4,
    F = 5,
    G = 7,
    A = 9,
    B = 11,
}
#[derive(Clone, Copy, Display)]
#[allow(dead_code)]
pub enum Accidental {
    #[display(fmt = "b")]
    Flat = -1,
    #[display(fmt = "#")]
    Sharp = 1,
    #[display(fmt = "")]
    None = 0,
}

#[derive(Clone, Copy)]
pub struct Note {
    pub letter: NoteLetter,
    pub accidental: Accidental,
    pub octave: u8,
}

impl Note {
    pub fn new(letter: NoteLetter, accidental: Accidental, octave: u8) -> Self {
        Note {
            letter,
            accidental,
            octave,
        }
    }

    pub fn into_u8(self) -> u8 {
        ((self.letter as i8 + self.accidental as i8) % 12) as u8 + self.octave * 12
    }

    /// https://en.wikipedia.org/wiki/Musical_note#Note_frequency_(hertz)
    pub fn freq(self) -> FreqType {
        2.0_f64.powf((self.into_u8() as FreqType - 69.0) / 12.0) * 440.0
    }
}

impl From<u8> for Note {
    fn from(val: u8) -> Self {
        use Accidental::*;
        use NoteLetter::*;
        let octave = (val / 12) as u8;
        match val % 12 {
            0 => Note::new(C, None, octave),
            1 => Note::new(C, Sharp, octave),
            2 => Note::new(D, None, octave),
            3 => Note::new(D, Sharp, octave),
            4 => Note::new(E, None, octave),
            5 => Note::new(F, None, octave),
            6 => Note::new(F, Sharp, octave),
            7 => Note::new(G, None, octave),
            8 => Note::new(G, Sharp, octave),
            9 => Note::new(A, None, octave),
            10 => Note::new(A, Sharp, octave),
            11 => Note::new(B, None, octave),
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{}{}{}",
            self.letter,
            self.accidental,
            self.octave as i8 - 1
        )
    }
}
