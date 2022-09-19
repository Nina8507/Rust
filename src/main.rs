pub mod lexer;

use lexer::*;


fn main() {
    let mut lexer = Lexer::new(".2 ( { ) } 22 34222 333 ( )");
    
    loop {
        match lexer.next_token() {  
            Ok(TokenType::EOT) => break,
            Ok(tok) => println!("{0:?}", tok),
            Err(err) => println!("{0:?}", err),

        }
    }
}
