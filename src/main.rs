enum Token {
    Identifier,
    StringLiteral,
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

fn scan_identifier(first: char, chars: &mut std::str::Chars) {
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
    println!("{}", identifier);
    chars.nth(identifier.chars().count() - 2);
}

fn scan_string(chars: &mut std::str::Chars) {
    let mut string = String::new();
    while let Some(c) = chars.next() {
        match c {
            '\'' => { println!("'{}'", string); return; },
            _ => string.push(c),
        }
    }
    println!("unterminated string! '{}", string);
}

fn scan(src: &str) {
    let mut chars = src.chars();
    while let Some(c) = chars.next() {
        match c {
            '(' => println!("("),
            ')' => println!(")"),
            ';' => println!(";"),
            '\'' => scan_string(&mut chars),
            _ => scan_identifier(c, &mut chars),
        }
    }
}

fn main() {
    let input = "print('Hello, world!');";
    println!("Input:\n\n{}\n\nTokens:\n", input);
    scan(input);
}
