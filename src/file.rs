use std::convert::{Into, From};

pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H
}

impl Into<char> for File {
    fn into(self) -> char {
        match self {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

impl From<char> for File {
    fn from(value: char) -> Self {
        match value {
            'a' => File::A,
            'b' => File::B,
            'c' => File::C,
            'd' => File::D,
            'e' => File::E,
            'f' => File::F,
            'g' => File::G,
            'h' => File::H,
            _ => panic!("Invalid file letter")
        }
    }
}

impl From<u8> for File {
    fn from(value: u8) -> Self {
        Self::from(value as char)
    }
}
