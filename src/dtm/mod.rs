pub mod interpret;

use std::collections::HashMap;
use std::fmt;

pub type State = String;
pub type Symbol = String;
pub type InputAlphabet = Vec<Symbol>;
pub type Alphabet = Vec<TapeSymbol>;
pub type Transition = HashMap<TapeSymbol, (State, TapeSymbol, Move)>;
pub type Transitions = HashMap<State, Transition>;

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub enum TapeSymbol {
    Symbol(Symbol),
    Blanco
}

impl TapeSymbol {
    fn write_latex(&self, f: &mut fmt::Formatter, math_mode: bool) -> fmt::Result {
        match (self, math_mode) {
            (&TapeSymbol::Symbol(ref symbol), false) => write!(f, "{}", symbol),
            (&TapeSymbol::Symbol(ref symbol), true) => write!(f, "\\text{{{}}}", symbol),
            (&TapeSymbol::Blanco, false) => write!(f, "$b$"),
            (&TapeSymbol::Blanco, true) => write!(f, "b")
        }
    }
}

impl fmt::Display for TapeSymbol {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TapeSymbol::Symbol(ref sym) => write!(f, "{}", sym),
            TapeSymbol::Blanco => write!(f, "_")
        }
    }
}

pub struct TuringMachine {
    alphabet: Alphabet,
    states: Vec<State>,
    accept: State,
    reject: State,
    delta: Transitions
}

impl TuringMachine {
    pub fn new(alphabet: Alphabet, states: Vec<State>, accept: State, reject: State, delta: Transitions) -> Self {
        Self {
            alphabet,
            states,
            accept,
            reject,
            delta
        }
    }

    pub fn latex<'a>(&'a self) -> LatexPrinter<'a> {
        LatexPrinter {
            dtm: self
        }
    }

    pub fn initial(&self) -> &State {
        self.states.first().expect("Ill-formed turing machine")
    }

    pub fn get(&self, state: &State, symbol: &TapeSymbol) -> Option<&(State, TapeSymbol, Move)> {
        self.delta.get(state).and_then(|t| t.get(symbol))
    }
}

#[derive(Clone)]
pub enum Move {
    Left,
    None,
    Right
}

impl Move {
    fn write_latex(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Move::Left => write!(f, "-1"),
            Move::None => write!(f, "0"),
            Move::Right => write!(f, "+1")
        }
    }
}

pub struct LatexPrinter<'a> {
    dtm: &'a TuringMachine
}

impl<'a> LatexPrinter<'a> {
    fn write_alphabet(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "$\\Gamma = \\{{")?;
        let mut first = true;

        for symbol in self.dtm.alphabet.iter() {
            if first {
                first = false;
            } else {
                write!(f, ", ")?;
            }

            symbol.write_latex(f, true)?;
        }

        writeln!(f, "\\}}\\newline$")
    }

    fn write_transitions(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "\\begin{{tabular}}{{c|{1:c<0$}}}", self.dtm.alphabet.len(), "")?;
        write!(f, "q/s")?;

        for symbol in self.dtm.alphabet.iter() {
            write!(f, "& ")?;
            symbol.write_latex(f, false)?;
        }

        writeln!(f, "\\\\\\hline")?;

        for from in self.dtm.states.iter().filter(|x| *x != &self.dtm.accept && *x != &self.dtm.reject) {
            write!(f, "{}", from)?;
            let t = self.dtm.delta.get(from).expect("Ill-formed turing machine");
            for symbol in self.dtm.alphabet.iter() {
                let &(ref to, ref w, ref mov) = t.get(symbol).expect("Ill-formed turing machine");

                write!(f, " & ({}, ", to)?;
                w.write_latex(f, false)?;
                write!(f, ", ")?;
                mov.write_latex(f)?;
                write!(f, ")")?;
            }
            writeln!(f, "\\\\")?;
        }

        write!(f, "\\end{{tabular}}")
    }
}

impl<'a> fmt::Display for LatexPrinter<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.write_alphabet(f)?;
        self.write_transitions(f)
    }
}