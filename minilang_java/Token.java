package minilang_java;

public class Token {
    public final TokenType type;
    public final String lexeme;
    public final int line;
    public final Integer column;

    public Token(TokenType type, String lexeme, int line, Integer column) {
        this.type = type;
        this.lexeme = lexeme;
        this.line = line;
        this.column = column;
    }

    public static Token eof(TokenType type, int line, int column) {
        return new Token(type, "", line, column);
    }

    public static Token identifier(String lexeme, int line, int column) {
        return new Token(TokenType.IDENTIFIER, lexeme, line, column);
    }

    public static Token number(String lexeme, int line, int column) {
        return new Token(TokenType.NUMBER, lexeme, line, column);
    }
}
