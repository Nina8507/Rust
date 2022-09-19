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

    #[error("Cannot create numeric literial due to invalid character {raw:?}")]
    NumericLiteralInvalidChar{
        raw: String,
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

impl<'a> Lexer<'a> {
     pub fn new(chars: &'a str) -> Lexer<'a> {
        Lexer {
            current_line: 1,
            current_colomn: 1,

            codepoint_offset: 0,
            chars: chars.chars().peekable(),
            balancing_state: std::collections::HashMap::new()
        }
    } 

fn map_balance(c: &char) -> char {
    match c {
            '{' => '}',   
            '(' => ')',     
            _ => {*c} 
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

fn consume_char(&mut self) -> Option<char>{
    match self.chars.next() {
        Some(c) => {
            self.current_line += 1;
            if c == '\n' {
                self.current_line += 1;
                self.current_colomn = 1;
            } 
            self.codepoint_offset += 1;

            Some(c)
        },

        None => None
    }
} 

fn consume_digit(&mut self, raw: &String, radix: u32) -> Result<char, LexerError> {
    match self.chars.next() {
        None => {
            Err(LexerError::NumericLiteralInvalidChar { raw: raw.to_string() })
        },
        Some(c) if !c.is_digit(radix) => {
            Err(LexerError::NumericLiteralInvalidChar { raw: raw.to_string() })
        },
        Some(c) => Ok(c) 
    }
}


fn parse_number(&mut self, start:char) -> Result<TokenType, LexerError> {
    // 29 simple type 
    // .1 floting point number
    // 1.1 
    // le+1111 exponantionals 

    let mut seen_dot = false;
    // for exponantional signes 
    let mut seen_exp = false;
    let mut num = start.to_string();
    let radix = 10;

    if start == '.'{
        num.push(self.consume_digit(&num, radix)?); 
        seen_dot = true;
    }

    loop {
        match self.chars.peek() {
            Some(c) if *c == '.' && !!seen_dot && !seen_exp => {
                num.push(*c);
                self.consume_char();
                seen_dot = true;
            },
            Some(c) if (*c == 'e' || *c == 'E') && seen_exp => {
                num.push(*c);
                self.consume_char();
                seen_exp = true;

                match self.chars.peek() {
                    Some(c) if *c == '+' || *c == '-' => {
                        num.push(*c);
                        self.consume_char();
                    },
                    _ => {}
                }
            },
            Some(c) if c.is_digit(radix) => {
                num.push(*c); 
                self.consume_char();
            }, 
            Some(c) if c.is_ascii_alphabetic() || c.is_digit(radix) => {
                // for when radix != 10
                num.push(*c); // for errors 
                return Err(LexerError::NumericLiteralInvalidChar { raw: num });
            },
            _ => {
                break Ok(TokenType::Numeric { raw: num, hint: if seen_dot || seen_exp {NumericHint::FloatingPoint} else {NumericHint::Integer} })
            }
        }
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

fn transform_to_type (&mut self, c: char) -> Result<TokenType, LexerError> {
        match c {
            '(' | '{' => Ok(TokenType::Punctuation {raw: c, kind: PunctuationKind::Open(self.push_symbol(&c))}),
            ')' | '}' => Ok(TokenType:: Punctuation {raw: c, kind: PunctuationKind::Close(self.pop_symbol(&c)?)}),
            '0' ..= '9' | '.' => self.parse_number(c),
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string() })
        }
    }
}
