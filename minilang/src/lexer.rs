#[derive(Debug)]
pub enum TokenType { 
    Ident(String),
    Number(i64),
    LParen,
    RParen,
    Plus,
    Print,
    Eof,
}

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub column: usize,
    pub value: Option<String>,
}

pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut line = 1usize;
    let mut column = 1usize;

    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            if c == '\n' {
                line += 1;
                column = 1;
            } else {
                column += 1;
            }
            continue;
        }

        if c.is_ascii_alphabetic() {
            let start_column = column;
            let mut word = String::new();

            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_alphanumeric() {
                    word.push(ch);
                    chars.next();
                    column += 1;
                } else {
                    break;
                }
            }

            let token_type = if word == "print" {
                TokenType::Print
            } else {
                TokenType::Ident(word.clone())
            };

            let value =  match token_type {
                TokenType::Ident(_) => Some(word),
                _ => None,
            };

            tokens.push(Token {
                token_type,
                line,
                column: start_column,
                value,
            });
            continue;
        }

        if c.is_ascii_digit() {
            let start_column = column;
            let mut number = String::new();

            while let Some(&ch) = chars.peek() {
                if ch.is_ascii_digit() {
                    number.push(ch);
                    chars.next();
                    column += 1;
                } else {
                    break;
                }
            }

            tokens.push(Token {
                token_type: TokenType::Number(number.parse().unwrap()),
                line,
                column: start_column,
                value: None,
            });
            continue;
        }

        let start_column = column;
        let token_type = match c {
            '(' => TokenType::LParen,
            ')' => TokenType::RParen,
            '+' => TokenType::Plus,
            _ => panic!("Unexpected character: {}", c),
        };

        tokens.push(Token {
            token_type,
            line,
            column: start_column,
            value: None,
        });
        chars.next();
        column += 1;
    }

    tokens.push(Token {
        token_type: TokenType::Eof,
        line,
        column,
        value: None,
    });

    tokens
}