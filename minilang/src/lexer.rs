#[derive(Debug, Clone, PartialEq)]
pub enum TokenType { 
    // keywords
    Let, Fun, Return, If, Else, While,
    Print, IntKw, BoolKw, True, False,
    // two-char ops
    EqEq, BangEq, Le, Ge, And, Or,
    // single-char ops & punct
    Plus, Minus, Star, Slash,
    Lt, Gt, Eq, Bang,
    LParen, RParen, LBrace, RBrace,
    Colon, Semi, Comma,
    // pattern-matched
    Ident(String), Number(i32),
    // synthetic
    Eof,
}

#[derive(Debug, Clone)]
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

    let keywords = [
        ("print", TokenType::Print),
        ("let", TokenType::Let),
        ("while", TokenType::While),
        ("int", TokenType::IntKw),
        ("bool", TokenType::BoolKw),
        ("true", TokenType::True),
        ("false", TokenType::False),
        ("fun", TokenType::Fun),
        ("return", TokenType::Return),
        ("if", TokenType::If),
        ("else", TokenType::Else),
    ];
    
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

            let token_type = keywords
                .iter()
                .find(|(kw, _)| *kw == word)
                .map(|(_, tt)| tt.clone())
                .unwrap_or(TokenType::Ident(word.clone()));

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
        let next_char = chars.clone().nth(1);
        let (token_type, consume_second) = match c {
            '(' => (TokenType::LParen, false),
            ')' => (TokenType::RParen, false),
            '+' => (TokenType::Plus, false),
            '-' => (TokenType::Minus, false),
            '*' => (TokenType::Star, false),
            '/' => (TokenType::Slash, false),
            '=' => {
                if next_char == Some('=') {
                    (TokenType::EqEq, true)
                } else {
                    (TokenType::Eq, false)
                }
            }
            ',' => (TokenType::Comma, false),
            ';' => (TokenType::Semi, false),
            ':' => (TokenType::Colon, false),
            '<' => {
                if next_char == Some('=') {
                    (TokenType::Le, true)
                } else {
                    (TokenType::Lt, false)
                }
            }
            '>' => {
                if next_char == Some('=') {
                    (TokenType::Ge, true)
                } else {
                    (TokenType::Gt, false)
                }
            }
            '!' => {
                if next_char == Some('=') {
                    (TokenType::BangEq, true)
                } else {
                    (TokenType::Bang, false)
                }
            }
            '&' => {
                if next_char == Some('&') {
                    (TokenType::And, true)
                } else {
                    panic!("Unexpected character: & (did you mean &&?)")
                }
            }
            '|' => {
                if next_char == Some('|') {
                    (TokenType::Or, true)
                } else {
                    panic!("Unexpected character: | (did you mean ||?)")
                }
            }
            '{' => (TokenType::LBrace, false),
            '}' => (TokenType::RBrace, false),
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
        if consume_second {
            chars.next();
            column += 1;
        }
    }

    tokens.push(Token {
        token_type: TokenType::Eof,
        line,
        column,
        value: None,
    });

    tokens
}