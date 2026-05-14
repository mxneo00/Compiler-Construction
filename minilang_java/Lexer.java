package minilang_java;

import java.util.ArrayList;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

public class Lexer {
    private static final Map<String, TokenType> KEYWORDS = new HashMap<>();
    static {
        KEYWORDS.put("let", TokenType.LET);
        KEYWORDS.put("fun", TokenType.FUN);
        KEYWORDS.put("return", TokenType.RETURN);
        KEYWORDS.put("if", TokenType.IF);
        KEYWORDS.put("else", TokenType.ELSE);
        KEYWORDS.put("while", TokenType.WHILE);
        KEYWORDS.put("print", TokenType.PRINT);
        KEYWORDS.put("int", TokenType.INT);
        KEYWORDS.put("bool", TokenType.BOOL);
        KEYWORDS.put("true", TokenType.TRUE);
        KEYWORDS.put("false", TokenType.FALSE);
    }

    public static List<Token> tokenize(String source) {
        List<Token> tokens = new ArrayList<>();
        int i = 0;
        int line = 1;
        int column = 1;

        while (i < source.length()) {
            char c = source.charAt(i);
            if (Character.isWhitespace(c)) {
                if (c == '\n') {
                    line++;
                    column = 1;
                } else  {
                    column++;
                }

                i++;
                continue;
            }

            if (Character.isLetter(c)) {
                int startColumn = column;
                StringBuilder sb = new StringBuilder();
                while ( i < source.length() && Character.isLetterOrDigit(c)) {
                    sb.append(source.charAt(i));
                    i++;
                    column++;
                }
                String word = sb.toString();
                TokenType keyword = KEYWORDS.get(word);
                if (keyword != null) tokens.add(Token.eof(keyword, line, startColumn));
                    else tokens.add(Token.identifier(word, line, startColumn));
                continue;
            }

            if (Character.isDigit(c)) {
                int startColumn = column;
                StringBuilder sb = new StringBuilder();
                while (i < source.length() && Character.isDigit(source.charAt(i))) {
                    sb.append(source.charAt(i));
                    i++;
                    column++;
                }
                String number = sb.toString();
                tokens.add(Token.number(number, line, startColumn));
                continue;
            }

            int startColumn = column;
            char next = (i + 1 < source.length()) ? source.charAt(i + 1) : '\0';
            switch (c) {
                case '+': tokens.add(Token.eof(TokenType.PLUS, line, startColumn)); break;
                case '-': tokens.add(Token.eof(TokenType.MINUS, line, startColumn)); break;
                case '*': tokens.add(Token.eof(TokenType.STAR, line, startColumn)); break;
                case '/': tokens.add(Token.eof(TokenType.SLASH, line, startColumn)); break;
                case '(': tokens.add(Token.eof(TokenType.LPAREN, line, startColumn)); break;
                case ')': tokens.add(Token.eof(TokenType.RPAREN, line, startColumn)); break;
                case '{': tokens.add(Token.eof(TokenType.LBRACE, line, startColumn)); break;
                case '}': tokens.add(Token.eof(TokenType.RBRACE, line, startColumn)); break;
                case ';': tokens.add(Token.eof(TokenType.SEMICOLON, line, startColumn)); break;
                case '=':
                    if (next == '=') {
                        tokens.add(Token.eof(TokenType.EQEQ, line, startColumn));
                        i++;
                        column++;
                    } else {
                        tokens.add(Token.eof(TokenType.EQ, line, startColumn));
                    }
                    break;
                case '!':
                    if (next == '=') {
                        tokens.add(Token.eof(TokenType.NOTEQ, line, startColumn));
                        i++;
                        column++;
                    } else {
                        throw new RuntimeException("Unexpected character: " + c + " at " + line + ":" + column);
                    }
                    break;
                default:
                    throw new RuntimeException("Unexpected character: " + c + " at " + line + ":" + column);
            }
        }
        tokens.add(Token.eof(line, column));
        return tokens;
    }
}
