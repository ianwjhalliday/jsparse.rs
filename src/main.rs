#[derive(Debug)]
enum Token {
    Identifier(String),
    StringLiteral(String),
    LeftParenthesis,
    RightParenthesis,
    Semicolon,
}

mod ast {
    enum Node {
        Identifier,
        StringLiteral,
        CallExpression,
    }
}

fn scan_identifier(first: char, chars: &mut std::str::Chars) -> Token {
    let mut identifier = String::new();
    identifier.push(first);

    let mut id_chars = chars.clone();
    while let Some(c) = id_chars.next() {
        match c {
            'a'...'z'|'A'...'Z'|'0'...'9' =>
                identifier.push(c),
            _ => break,
        }
    }
    chars.nth(identifier.chars().count() - 2);
    Token::Identifier(identifier)
}

fn scan_string(delimiter: char, chars: &mut std::str::Chars) -> Token {
    // TODO: invalid newlines
    // TODO: escape sequences
    let mut string = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\''|'"' if c == delimiter => return Token::StringLiteral(string),
            _ => string.push(c),
        }
    }
    println!("Error: unterminated string: {}{}", delimiter, string);
    std::process::exit(1);
}

fn scan(src: &str) -> Vec<Token> {
    let mut chars = src.chars();
    let mut tokens = Vec::new();
    while let Some(c) = chars.next() {
        tokens.push(match c {
            '(' => Token::LeftParenthesis,
            ')' => Token::RightParenthesis,
            ';' => Token::Semicolon,
            '"'|'\'' => scan_string(c, &mut chars),
            'a'...'z'|'A'...'Z' => scan_identifier(c, &mut chars),
            _ => {
                println!("Error: illegal character: '{}'", c);
                std::process::exit(1);
            }
        });
    }
    tokens
}

fn main() {
    let input = "print('Hello, world!');";
    let tokens = scan(input);
    println!("Input:\n\n{}\n\nTokens:\n\n{:?}", input, tokens);
}
