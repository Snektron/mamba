pub mod construct;

use std::fmt;
use utility::MultiMap;
use dtm::{Alphabet, State, Move, TapeSymbol};

pub struct Program {
    pub alphabet: Alphabet,
    pub accept: State,
    pub reject: State,
    pub states: Vec<State>,
    pub transitions: MultiMap<State, TransitionArm>
}

pub struct TransitionArm {
    pub read: Pattern,
    pub to: State,
    pub write: Pattern,
    pub mov: Move
}

#[derive(Debug, Clone)]
pub enum Pattern {
    TapeSymbol(TapeSymbol),
    Wildcard
}

impl fmt::Display for Pattern {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Pattern::TapeSymbol(ref symbol) => symbol.fmt(f),
            Pattern::Wildcard => write!(f, "*")
        }
    }
}