use std::collections::HashSet as Set;
use std::error::Error;
use std::fmt;
use dtm::{TuringMachine, State, TapeSymbol, Symbol, Transition, Transitions, InputAlphabet, Alphabet};
use ast::{Program, TransitionArm, Pattern};

#[derive(Debug)]
pub enum ConstructionError {
    MissingTransitions(State, Set<TapeSymbol>),
    Redefinition(State, Pattern),
    DuplicateSymbol(Symbol),
    UndefinedSymbol(Symbol),
    UndefinedState(State),
    WildcardInArm(State)
}

impl Error for ConstructionError {
    fn description(&self) -> &str {
        match *self {
            ConstructionError::MissingTransitions(..) => "missing transition(s)",
            ConstructionError::Redefinition(..) => "state redefinition",
            ConstructionError::DuplicateSymbol(..) => "duplicate alphabet symbol",
            ConstructionError::UndefinedSymbol(..) => "undefined symbol",
            ConstructionError::UndefinedState(..) => "undefined state",
            ConstructionError::WildcardInArm(..) => "wildcard in arm"
        }
    }
}

impl fmt::Display for ConstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConstructionError::MissingTransitions(ref state, ref symbols) => {
                write!(f, "Missing transitions for state {}, symbol", state)?;

                let len = symbols.len();
                for (i, symbol) in symbols.iter().enumerate() {
                    if i == len {
                        write!(f, " and ")?;
                    } else if i != 0 {
                        write!(f, ", ")?;
                    } else if i == 0 && len == 1 {
                        write!(f, " ")?;
                    } else {
                        write!(f, "s ")?;
                    }

                    write!(f, "{}", symbol)?;
                }

                Ok(())
            },
            ConstructionError::Redefinition(ref state, ref symbol) => {
                write!(f, "Redefinition of transition for state {}, symbol {}", state, symbol)
            },
            ConstructionError::DuplicateSymbol(ref symbol) => {
                write!(f, "Duplicate symbol {} in alphabet", symbol)
            },
            ConstructionError::UndefinedSymbol(ref symbol) => {
                write!(f, "Undefined symbol {}", symbol)
            },
            ConstructionError::UndefinedState(ref state) => {
                write!(f, "Undefined state {}", state)
            },
            ConstructionError::WildcardInArm(ref state) => {
                write!(f, "Runaway wildcard in arm of state {}", state)
            }
        }
    }
}

pub type ConstructionResult<T> = Result<T, ConstructionError>;

fn check_alphabet(alphabet: InputAlphabet) -> ConstructionResult<Alphabet> {
    let mut v = Vec::new();

    for symbol in alphabet {
        let tape_symbol = TapeSymbol::Symbol(symbol.clone());
        if v.contains(&tape_symbol) {
            return Err(ConstructionError::DuplicateSymbol(symbol));
        } else {
            v.push(tape_symbol);
        }
    }

    v.push(TapeSymbol::Blanco);

    Ok(v)
}

fn check_arm(arm: &TransitionArm, states: &Vec<State>, alphabet: &Alphabet) -> ConstructionResult<()> {
    if let Pattern::TapeSymbol(ref tape_sym) = arm.read {
        if let &TapeSymbol::Symbol(ref sym) = tape_sym {
            if !alphabet.contains(&tape_sym) {
                return Err(ConstructionError::UndefinedSymbol(sym.clone()));
            }
        }
    }

    if let Pattern::TapeSymbol(ref tape_sym) = arm.write {
        if let &TapeSymbol::Symbol(ref sym) = tape_sym {
            if !alphabet.contains(&tape_sym) {
                return Err(ConstructionError::UndefinedSymbol(sym.clone()));
            }
        }
    }

    if !states.contains(&arm.to) {
        return Err(ConstructionError::UndefinedState(arm.to.clone()));
    }

    Ok(())
}

fn check_arms(from: &State, arms: Vec<TransitionArm>, states: &Vec<State>, alphabet: &Alphabet) -> ConstructionResult<Transition> {
    let mut transition = Transition::new();
    let mut seen = Set::new();
    let mut wildcard = None;

    for arm in arms {
        check_arm(&arm, states, alphabet)?;

        match arm.read {
            Pattern::TapeSymbol(sym) => {
                if !seen.insert(sym.clone()) {
                    return Err(ConstructionError::Redefinition(from.clone(), Pattern::TapeSymbol(sym)));
                }

                let write = match arm.write {
                    Pattern::TapeSymbol(sym) => Ok(sym),
                    Pattern::Wildcard => Err(ConstructionError::WildcardInArm(from.clone()))
                }?;

                transition.insert(sym, (arm.to, write, arm.mov));
            },
            Pattern::Wildcard => {
                if wildcard.is_some() {
                    return Err(ConstructionError::Redefinition(from.clone(), Pattern::Wildcard));
                } else {
                    wildcard = Some(arm);
                }
            }
        }
    }

    let alphabet = alphabet.iter().cloned().collect::<Set<_>>();
    let missing = alphabet.difference(&seen).cloned().collect::<Set<_>>();

    match (missing.len(), wildcard) {
        (0, _) => Ok(transition),
        (_, None) => Err(ConstructionError::MissingTransitions(from.clone(), missing)),
        (_, Some(wildcard)) => {
            for item in missing {
                let write = match wildcard.write {
                    Pattern::TapeSymbol(ref symbol) => symbol.clone(),
                    Pattern::Wildcard => item.clone()
                };

                transition.insert(item, (wildcard.to.clone(), write, wildcard.mov.clone()));
            }

            Ok(transition)
        }
    }
}

pub fn construct_dtm(mut program: Program) -> ConstructionResult<TuringMachine> {
    let alphabet = check_alphabet(program.alphabet)?;
    let mut states = program.states;
    states.push(program.accept.clone());
    states.push(program.reject.clone());
    states.dedup();

    let mut delta = Transitions::new();

    for (from, arms) in program.transitions.drain_all() {
        let mut t = check_arms(&from, arms, &states, &alphabet)?;
        delta.insert(from, t);
    }

    Ok(TuringMachine::new(alphabet, states, program.accept, program.reject, delta))
}