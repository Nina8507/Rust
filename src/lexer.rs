use crate::token::*;

#[derive(Debug, Clone)]
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

macro_rules! try_consume {
    ($self: tt, $($inner:tt), *) => {
        if let Some(c) = $self.chars.peek() {
            if try_consume!(impl c, $($inner), *) {
                let temp = *c;
                $self.consume_char();
                Some(temp)
            } else {
                None
            }
        } else {
            None
        }
    };

    // impl recursive macro 
    (impl, ) => (false);
    (impl $c:tt, $item:tt) => (*$c == $item);
    (impl $c:tt, $item:tt, $($rest:tt), +) => (try_consume!(impl $c, $item) || try_consume!(impl $c, $($rest), *)); 
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

fn parse_digits(&mut self, radix: u32, allow_empty: bool) -> Result<String, LexerError> {
    let mut raw = String::new();

    loop {
        match self.chars.peek() {
            None => {
                break if allow_empty || raw.len() > 0 {
                    Ok(raw)
                } else {
                    Err(LexerError::MissingExpectedSymbol { 
                            expected:"<digit>", 
                            found: TokenType::EOT })
                }
            }
            Some(c) if c.is_digit(radix) || (*c == '_' && raw.len() > 0) => raw.push(*c),
            Some(c) if !c.is_ascii_alphabetic() && *c != '_' => break Ok(raw),
            Some(c) => {
                break Err(LexerError::NumericLiteralInvalidChar { raw, invalid: *c, })
            }
        }
    }
}

fn parse_number(&mut self, start:char) -> Result<TokenType, LexerError> {
    let mut raw  = start.to_string();
    let radix = 10;
    let mut hint = NumericHint::Integer; 

    if start == '.'{
        raw += &self.parse_digits(radix, false)?;
        hint = NumericHint::FloatingPoint;
    } else if start.is_digit(radix) {
        raw  += &self.parse_digits(radix, true)?;

            if let Some(c) = try_consume!(self, '.') {
            raw.push(c);
            raw += &self.parse_digits(radix, false)?;
            hint = NumericHint::FloatingPoint;
            } 
        } else {
        return Err(LexerError::NumericLiteralInvalidChar { raw, invalid: start, }); 
    }

    if let Some(c) = try_consume!(self, 'e', 'E') {
        hint = NumericHint::FloatingPoint;
        raw.push(c);
        if let Some(c) = try_consume!(self, '+', '-') {
        raw.push(c);
        }

    raw += &self.parse_digits(radix, false)?;
    }

    Ok(TokenType::Numeric { raw, hint, }) 
}

fn parse_string(&mut self) -> Result<TokenType, LexerError> {
    let mut buffer = String::new();

    loop {
        match self.chars.next() {
            Some('"') => break Ok(TokenType::String(buffer)),
            Some(c) => buffer.push(c),
            None => break Err(LexerError::MissingExpectedSymbol { expected: "\"", found: TokenType::EOT })
        }
    }
}

fn parse_identifier(&mut self, start:char) -> Result<TokenType, LexerError> {
    let radix = 10; 
    let mut buffer = start.to_string();

    loop {
        match self.chars.peek() {
            Some(c) if c.is_ascii_alphabetic() || c.is_digit(radix) || *c == '_' => buffer.push(self.chars.next().unwrap()), 
            _ => break Ok(TokenType::Identifier(buffer)) 
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
            '(' | '{' => Ok(TokenType::Punctuation {raw: c, kind: PunctuationKind::Open(self.push_symbol(&c)),}),
            ')' | '}' => Ok(TokenType:: Punctuation {raw: c, kind: PunctuationKind::Close(self.pop_symbol(&c)?),}),
            '0' ..= '9' | '.' => self.parse_number(c),
            ';' => Ok(TokenType::Punctuation { raw: c, kind: PunctuationKind::Separator }),
            c if c.is_ascii_alphabetic() || c == '_' => self.parse_identifier(c), 
            '"' => self.parse_string(),
            _ => Err(LexerError::UnknownSymbol { symbol: c.to_string(), }),
        }
    }
}
