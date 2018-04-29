use std::error::Error;
use std::str::FromStr;
use std::fmt;
use regex::Regex;
use dtm::{TuringMachine, TapeSymbol, State, Move};

lazy_static! {
    static ref SYMBOL_REGEX: Regex = Regex::new(r"[0-9A-Za-z]+").unwrap();
}

#[derive(Debug)]
pub enum RuntimeError {
    OutOfBounds,
    Finished,
    InvalidSymbol(TapeSymbol)
}

impl Error for RuntimeError {
    fn description(&self) -> &str {
        match *self {
            RuntimeError::OutOfBounds => "out of bounds",
            RuntimeError::Finished => "finished",
            RuntimeError::InvalidSymbol(..) => "invalid symbol"
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            RuntimeError::OutOfBounds => write!(f, "Head moved out of bounds"),
            RuntimeError::Finished => write!(f, "Program is already finished"),
            RuntimeError::InvalidSymbol(ref symbol) => write!(f, "Invalid tape symbol {}", symbol)
        }
    }
}

type RtResult<T> = Result<T, RuntimeError>;

pub struct Tape {
    cells: Vec<TapeSymbol>,
    head: usize
}

impl Tape {
    pub fn new(input: Vec<TapeSymbol>) -> Self {
        Self {
            cells: input,
            head: 0
        }
    }

    pub fn left(&mut self) -> RtResult<()> {
        if self.head == 0 {
            Err(RuntimeError::OutOfBounds)
        } else {
            self.head -= 1;
            Ok(())
        }
    }

    pub fn right(&mut self) {
        self.head += 1;

        while self.cells.len() <= self.head {
            self.cells.push(TapeSymbol::Blanco)
        }
    }

    pub fn mov(&mut self, mov: &Move) -> RtResult<()> {
        match *mov {
            Move::Left => self.left(),
            Move::None => Ok(()),
            Move::Right => {
                self.right();
                Ok(())
            }
        }
    }

    pub fn read(&self) -> &TapeSymbol {
        &self.cells[self.head]
    }

    pub fn write(&mut self, symbol: TapeSymbol) {
        self.cells[self.head] = symbol;
    }

    pub fn dump(&self) {
        for (i, v) in self.cells.iter().enumerate() {
            if i == self.head {
                print!("[{}]", v);
            } else {
                print!(" {} ", v);
            }
        }
        println!("");
    }
}

impl FromStr for Tape {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let symbols = input
            .split(char::is_whitespace)
            .filter(|c| c != &"")
            .map(TapeSymbol::from_str)
            .collect::<Result<_, _>>()?;

        Ok(Tape::new(symbols))
    }
}

impl FromStr for TapeSymbol {
    type Err = ParseError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        if input == "_" {
            Ok(TapeSymbol::Blanco)
        } else if SYMBOL_REGEX.is_match(input) {
            Ok(TapeSymbol::Symbol(input.to_owned()))
        } else {
            Err(ParseError(input.to_owned()))
        }
    }
}

#[derive(Debug)]
pub struct ParseError(String);

impl Error for ParseError {
    fn description(&self) -> &str {
        "invalid symbol format"
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid format for symbol '{}'", self.0)
    }
}

#[derive(Debug)]
pub enum Outcome {
    Accept,
    Reject
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Outcome::Accept => write!(f, "Accept"),
            Outcome::Reject => write!(f, "Reject")
        }
    }
}

pub struct Interpreter<'t> {
    dtm: &'t TuringMachine,
    tape: Tape,
    state: &'t State
}

impl<'t> Interpreter<'t> {
    pub fn new(dtm: &'t TuringMachine, tape: Tape) -> Interpreter {
        let state = dtm.initial();

        Self {
            dtm,
            tape,
            state
        }
    }

    pub fn step(&mut self) -> RtResult<()> {
        if self.has_finished() {
            return Err(RuntimeError::Finished);
        }

        let current = self.tape.read().clone();
        if let Some(&(ref to, ref write, ref mov)) = self.dtm.get(self.state, &current) {
            self.state = to;
            self.tape.write(write.clone());
            self.tape.mov(mov)
        } else {
            Err(RuntimeError::InvalidSymbol(current))
        }
    }

    pub fn has_finished(&self) -> bool {
        self.state == &self.dtm.accept || self.state == &self.dtm.reject
    }

    pub fn outcome(&self) -> Option<Outcome> {
        if self.state == &self.dtm.accept {
            Some(Outcome::Accept)
        } else if self.state == &self.dtm.reject {
            Some(Outcome::Reject)
        } else {
            None
        }
    }

    pub fn dump(&self) {
        self.tape.dump()
    } 

    pub fn execute(mut self, debug: bool) -> RtResult<Outcome> {
        if debug {
            self.dump();
        }

        while !self.has_finished() {
            self.step()?;

            if debug {
                self.dump();
            }
        }

        Ok( self.outcome().unwrap())
    }
}