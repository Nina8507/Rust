use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
 enum Error {
    #[error("IO Error")]
    FileIO(#[from] io::Error),

    #[error("Missing expected symbol.")]
    MissingExpectedSymbol{
        expected: TokenType,
        found: Token,
    }
}

 type Token = TokenType;

 enum TokenType {
    IDENTIFIER(String),
    INTEGER_LITERAL(i32),
    OPERATOR(String),
    LETTER(String),
    DIGIT(i32),
    TYPES(String),
    MIGHT(String),
    NOMORE(String),
    WAY(String),
    MAY(String),
    GIVE(String),
    SCRIBBLE(String),
    STUFF(String),
    DURING(String),

    //Comma, colon, Left paran, right paran, main, left braces and right braces
    SYMBOL{raw: char, kind:PunctuationKind},
    EOT,
    ERROR,

    // for errors
    UNKNOWN(char),
}

#[derive(Debug)]
 enum PunctuationKind {
    // ( , {
    Open(usize),
    // ), }
    Close(usize),
    // , , :
    Separator,
    // @
    Start,
}

 struct Lexer {
    // human readable format
     current_line: usize,
     current_colomn: usize,

    // raw format in terms of bytes, at what char index you are
     codepoint_offset: usize,

    chars: std::itera, 
    balancing_state: std::coll
    
}

impl Lexer {
     fn new(chars: &a str) -> Lexer {
        Lexer {
            current_line: 1,
            current_colomn: 1,

            codepoint_offset: 0,
            chars: chars.chars().Peekable()
        }
    } 

     fn transform_to_type (c: char) -> Option<TokenType> {
        match c {
            pub enum TokenType
            '(' => Some(TokenType::Punctuation {raw: c, kind: PunctuationKind::Open(0)}
        }
    }
}