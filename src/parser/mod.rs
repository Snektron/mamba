lalrpop_mod!(pub grammar, "/parser/grammar.rs");

pub use self::grammar::ProgramParser as Parser;
