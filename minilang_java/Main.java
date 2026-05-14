package minilang_java;

import java.nio.file.Files;
import java.nio.file.Paths;
import java.util.List;

public class Main {
    private static String tokenValue(Token token) {
        if (token.type == TokenType.IDENTIFIER || token.type == TokenType.NUMBER) {
            return " (" + token.lexeme + ")";
        }
        return "";
    }

    public static void main(String[] args) {
        String source;
        try {
            source = new String(Files.readAllBytes(Paths.get(args[0])));
        } catch (Exception e) {
            System.err.println("Error reading file: " + e.getMessage());
            return;
        }

        List<Token> tokens = Lexer.tokenize(source);
        
        System.out.printf("%-3s %-8s %-8s %s%n", "#", "TYPE", "VALUE", "POSITION");
        for (int i = 0; i < tokens.size(); i++) {
            Token token = tokens.get(i);
            System.out.printf("%-3d %-8s %-8s %d:%d%n", i, token.type, tokenValue(token), token.line, token.column);
        }
    }
}
