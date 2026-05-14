mod lexer;
use std::{env, fs, process};

fn token_type_name(token_type: &lexer::TokenType) -> &'static str {
    match token_type {
        lexer::TokenType::Ident(_) => "IDENT",
        lexer::TokenType::Number(_) => "NUMBER",
        lexer::TokenType::LParen => "LPAREN",
        lexer::TokenType::RParen => "RPAREN",
        lexer::TokenType::Plus => "PLUS",
        lexer::TokenType::Print => "PRINT",
        lexer::TokenType::Eof => "EOF",
    }
}

fn token_value(token: &lexer::Token) -> String {
    match &token.token_type {
        lexer::TokenType::Ident(name) => format!("\"{}\"", name),
        lexer::TokenType::Number(n) => n.to_string(),
        _ => token.value.clone().unwrap_or_default(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: minilang <filename>");
        process::exit(1);
    }

    let filename = &args[1];
    let contents = fs::read_to_string(filename).expect("Could not read file");
    let tokens = lexer::tokenize(&contents);

    println!("{:<3} {:<8} {:<8} POSITION", "#", "TYPE", "VALUE");
    for (i, token) in tokens.iter().enumerate() {
        println!(
            "{:<3} {:<8} {:<8} ({}, {})",
            i + 1,
            token_type_name(&token.token_type),
            token_value(token),
            token.line,
            token.column
        );
    }
}
