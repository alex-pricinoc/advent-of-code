use std::error;
use std::fmt;

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + 'static>>;

#[repr(transparent)]
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Item(u8);

#[derive(Debug)]
pub struct ParseItemError(u8);

impl error::Error for ParseItemError {}

impl fmt::Display for ParseItemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is not a valid item", self.0)
    }
}

impl TryFrom<u8> for Item {
    type Error = ParseItemError;

    fn try_from(value: u8) -> std::result::Result<Self, Self::Error> {
        match value {
            b'a'..=b'z' | b'A'..=b'Z' => Ok(Item(value)),
            _ => Err(ParseItemError(value)),
        }
    }
}

impl Item {
    pub fn score(self) -> usize {
        match self {
            Item(b'a'..=b'z') => 1 + (self.0 - b'a') as usize,
            Item(b'A'..=b'Z') => 27 + (self.0 - b'A') as usize,
            _ => unreachable!(),
        }
    }
}

impl fmt::Debug for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0 as char)
    }
}
