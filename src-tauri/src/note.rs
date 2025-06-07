use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Copy, Hash, PartialEq, Eq, Debug)]
#[serde(tag="note")]
pub enum Note {
    C, CSharp, D, DSharp, E, F, FSharp, G, GSharp, A, ASharp, B, 
}

impl Note {
    pub fn to_freq(&self) -> f32 {
        // in 3rd
        match self {
            Self::C => 130.8,
            Self::CSharp => 138.6,
            Self::D => 146.8,
            Self::DSharp => 155.6,
            Self::E => 164.8,
            Self::F => 174.6,
            Self::FSharp => 185.0,
            Self::G => 196.0,
            Self::GSharp => 207.7,
            Self::A => 220.0,
            Self::ASharp => 233.1,
            Self::B => 246.9,
        }
    }

    pub fn to_freq_octave(&self, o: i32) -> f32 {
        let m = 2f32.powi(o - 3);
        self.to_freq()*m
    }

    pub fn from_index(i: usize) -> Self {
         match i % 12 {
            0 => Self::C,
            1 => Self::CSharp,
            2 => Self::D,
            3 => Self::DSharp,
            4 => Self::E,
            5 => Self::F,
            6 => Self::FSharp,
            7 => Self::G,
            8 => Self::GSharp,
            9 => Self::A,
            10 => Self::ASharp,
            11 => Self::B, 
            _ => { panic!() }
        }
    }
}

fn default_octave() -> i32 { 3 }
fn default_position() -> u64 { 0 }
fn default_length() -> u64 { 1 }

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct SpecifiedNote {
    #[serde(flatten)]
    pub note: Note,

    #[serde(default="default_octave")]
    pub octave: i32,

    #[serde(default="default_position")]
    pub position: u64,

    #[serde(default="default_length")]
    pub length: u64,
}

impl SpecifiedNote {
    pub fn to_freq(&self) -> f32 {
        self.note.to_freq_octave(self.octave)
    }
}
