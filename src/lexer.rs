use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum LexerError {
    #[error("IO Error")]
    FileIO(#[from] io::Error),

    #[error("Was expecting {expected:?}, found {found:?}")]
    MissingExpectedSymbol{
        expected: TokenType,
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
}
 
#[derive(Debug)]
pub struct Punctuation {
    pub raw: char,
    pub kind: PunctuationKind,
}

pub type Token = TokenType;

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
    EOT,
    Error,

    // for errors
    Unknown(char),
}

#[derive(Debug)]
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
type BalancingDepthType = i32;

#[derive(Debug)]
pub struct Lexer<'a> {
    // human readable format
    pub current_line: usize,
    pub current_colomn: usize,

    // raw format in terms of bytes, at what char index you are
    pub codepoint_offset: usize,

    chars: std::iter::Peekable<std::str::Chars<'a>>, 

    // to maintain an open and close state for each punctuation 
    balancing_state: std::collections::HashMap<char, BalancingDepthType>,
    
}

impl<'a_, 'a> Lexer<'a_> {
     pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer {
            current_line: 1,
            current_colomn: 1,

            codepoint_offset: 0,
            chars: chars.chars().peekable(),
            balancing_state: std::collections::HashMap::new(),
        }
    } 

fn map_balance(c: &char) -> char {
    match c {
            '{' => '}',   
            '(' => ')',      
    }
}

// if the char is not in the hash table insert it
fn push_symbol (&mut self, c: &char) -> BalancingDepthType{
    if let Some(v) = self.balancing_state.get_mut(&c) { 
            *v += 1;
            *v
    } 
    else {
        self.balancing_state.insert(*c, 0);
        0
    }
}

fn pop_symbol (&mut self, c: &char) -> Result<BalancingDepthType, LexerError> {
    if let Some(v) = self.balancing_state.get_mut(&Lexer::map_balance(&c)) {
        if *v >= 1 {
        *v -= 1;
        Ok(*v) }
     else {
    Err(LexerError::MissingBalancedSymbol { symbol: *c, open: Lexer::map_balance(&c) })
         }
    }
    else {Err(LexerError::MissingBalancedSymbol { symbol: *c, open: Lexer::map_balance(&c) })} 
}

fn transform_to_type (&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' => Ok(TokenType::Punctuation {raw: c, kind: PunctuationKind::Open(self.push_symbol(&c))}),
            ')' => Ok(TokenType:: Punctuation {raw: c, kind: PunctuationKind::Close(self.pop_symbol(&c)?)}),
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string() })
        }
    }
}

fn consume_char(&mut self) -> Option<char>{
    match self.chars.next() {
        Some(c) => {
            self.current_line += 1;
            if c == '\n' {
                self.current_line += 1;
                self.current_column = 1;
            } 
            self.codepoint_offset += 1;

            Some(c)
        },

        None => None
    }
} 

fn skip_whitespace(&mut self) {
    while let Some(c) = self.chars.peek()  {
        if !c.is_whitespace(){
            break;
        }
        self.consume_char();
    }
}

pub fn next_token (&mut self) -> Result<TokenType, LexerError> {
    self.skip_whitespace();

    if let Some(c) = self.consume_char() {
        self.transform_to_type(c)
    } else {
        Ok(TokenType::EOT)
    }
}