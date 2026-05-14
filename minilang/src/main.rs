mod lexer;
use std::{env, fs, process};

fn token_type_name(token_type: &lexer::TokenType) -> &'static str {
    match token_type {
        lexer::TokenType::Ident(_) => "Ident",
        lexer::TokenType::Number(_) => "Number",
        lexer::TokenType::EqEq => "EqEq",
        lexer::TokenType::BangEq => "BangEq",
        lexer::TokenType::Le => "Le",
        lexer::TokenType::Ge => "Ge",
        lexer::TokenType::And => "And",
        lexer::TokenType::Or => "Or",
        lexer::TokenType::Plus => "Plus",
        lexer::TokenType::Minus => "Minus",
        lexer::TokenType::Star => "Star",
        lexer::TokenType::Slash => "Slash",
        lexer::TokenType::Lt => "Lt",
        lexer::TokenType::Gt => "Gt",
        lexer::TokenType::LParen => "LParen",
        lexer::TokenType::RParen => "RParen",
        lexer::TokenType::LBrace => "LBrace",
        lexer::TokenType::RBrace => "RBrace",
        lexer::TokenType::Print => "Print",
        lexer::TokenType::Let => "Let",
        lexer::TokenType::While => "While",
        lexer::TokenType::IntKw => "IntKw",
        lexer::TokenType::BoolKw => "BoolKw",
        lexer::TokenType::True => "True",
        lexer::TokenType::False => "False",
        lexer::TokenType::Fun => "Fun",
        lexer::TokenType::Return => "Return",
        lexer::TokenType::If => "If",
        lexer::TokenType::Else => "Else",
        lexer::TokenType::Eq => "Eq",
        lexer::TokenType::Bang => "Bang",
        lexer::TokenType::Semi => "Semi",
        lexer::TokenType::Colon => "Colon",
        lexer::TokenType::Comma => "Comma",
        lexer::TokenType::Eof => "Eof",
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
