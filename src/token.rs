use std::io;
use thiserror::Error;

//pub mod macros;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("IO Error")]
    FileIO(#[from] io::Error),

    #[error("Was expecting {expected:?}, found {found:?}")]
    MissingExpectedSymbol{
        expected: &'static str,
        found: Token,
    },

    #[error("Depth for symbol {symbol:?} is 0 cannot find opening symbol.")]
    MissingBalancedSymbol{
        symbol: char,
        open: char, 
    },

    #[error("Depth for symbol {symbol:?} is 0 cannot find opening symbol.")]
    UnknownSymbol {
        symbol: String,
    },

    #[error("Cannot create numeric literial due to invalid character {raw:?}")]
    NumericLiteralInvalidChar{
        raw: String,
        invalid: char, 
    },
}

pub type Token = TokenType;
 
#[derive(Debug)]
pub struct Punctuation {
    pub raw: char,
    pub kind: PunctuationKind,
}

#[derive(Debug)]
pub enum NumericHint {
    Integer, 
    FloatingPoint,
    Any, 
}


#[derive(Debug)]
pub enum TokenType {
    Identifier(String),
    IntegerLiteral(i32),
    Operator(String),
    Letter(String),
    Digit(i32),
    // Types(String),
    // MIGHT(String),
    // NOMORE(String),
    // WAY(String),
    // MAY(String),
    // GIVE(String),
    // SCRIBBLE(String),
    // STUFF(String),
    // DURING(String),

    //Comma, colon, Left paran, right paran, main, left braces and right braces
    Punctuation{raw: char, kind:PunctuationKind},
    Numeric{raw : String, hint:NumericHint},
    String(String), 
    EOT,
    Error,

    // for errors
    Unknown(char),
}

#[derive(Debug, PartialEq)]
pub enum PunctuationKind {
    // ( , {
    Open(BalancingDepthType),
    // ), }
    Close(BalancingDepthType),
    // , , :
    Separator,
    // @
    Start,
}

// to keep track of the positions in the hash map
pub type BalancingDepthType = i32;