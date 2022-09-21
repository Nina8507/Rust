extern crate clap;
extern crate std;

pub mod lexer;
pub mod token;
pub mod macros;

//use clap::{builder::*, SubCommand};
use lexer::Lexer;
use token::TokenType;


fn main() -> std::io::Result<()> {

    let text = std::fs::read_to_string("./input_file.txt").expect("Error loading file.");  
    
    let lexer = Lexer::new(&text);
    let mut lexer = lexer.clone();
                loop {
                    match lexer.next_token() {
                        Ok(TokenType::EOT) => {print!("EOT break......"); break; },
                        Ok(tok) => print!("{0:?}", tok),
                        Err(err) => print!("{0:?}", err),
                    }
                }
  

    /*let app = App::new("!Kap").version("1.0")
        .subcommand(SubCommand::with_name("debug").args_from_usage(
            "
            --show....      'tokens', 'AST'
            <INPUT>         'File loding'
            "
        ))
        .subcommand(SubCommand::with_name("input"))
     .get_matches();

    match app.subcommand() {
        ("debug", Some(sub_matches)) => {
            let filename = sub_matches.value_of("config").unwrap();
            let text = std::fs::read_to_string(filename)?;
            let lexer = Lexer::new(&text);
            let shows = sub_matches.values_of("show").unwrap_or_default().collect::<Vec<&str>>();

            if shows.contains(&"input") {
                let mut lexer = lexer.clone();
                loop {
                    match lexer.next_token() {
                        Ok(TokenType::EOT) => {print!("EOT break......"); break; },
                        Ok(tok) => print!("{0:?}", tok),
                        Err(err) => print!("{0:?}", err),
                    }
                }
            }
        }
        _ => {} 
    }*/

    Ok(( ))
}
