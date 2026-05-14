package minilang_java;

public enum TokenType {
    // Keywords
    LET, FUN, RETURN, IF, ELSE, WHILE, PRINT, INT, BOOL,
    TRUE, FALSE, 
    // Two-char operators
    EQEQ, NOTEQ, LEQ, GEQ, AND, OR,
    // One-char operators
    EQ, LT, GT, PLUS, MINUS, STAR, SLASH, BANG,
    // Punctuation
    LPAREN, RPAREN, LBRACE, RBRACE, SEMICOLON, COMMA, COLON, 
    // Literals
    IDENTIFIER, NUMBER,
    // End of file
    EOF,
}