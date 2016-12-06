#[derive(Debug)]
enum Token {
    Identifier(String),
    StringLiteral(String),
    LeftParenthesis,
    RightParenthesis,
    Semicolon,
}

mod ast {
    #[derive(Debug)]
    pub enum Node {
        IdentifierReference(String),
        StringLiteral(String),
        CallExpression { target: Box<Node>, arguments: Box<Node> },
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

macro_rules! expect_punctuation(
    ($p:pat, $e:expr) => (
        match $e {
            Some(&$p) => (),
            Some(t) => {
                //println!("Error: expected '{:?}' but got '{:?}' instead", $p, t);
                println!("Error: expected '<nyi>' but got '{:?}' instead", t);
                std::process::exit(1);
            },
            None => {
                //println!("Error: expected '{:?}' but reached end of file instead", $p);
                println!("Error: expected '<nyi>' but reached end of file instead");
                std::process::exit(1);
            }
        }
    )
);

fn parse_argument_list(tokens: &mut std::slice::Iter<Token>) -> ast::Node {
    expect_punctuation!(Token::LeftParenthesis, tokens.next());
    let ast = match tokens.next() {
        Some(&Token::StringLiteral(ref string)) => {
            ast::Node::StringLiteral(string.clone())
        },
        Some(t) => {
            println!("Error: expected 'Token::String' but got '{:?}' instead", t);
            std::process::exit(1);
        }
        None => {
            println!("Error: expected 'Token::String' but reached end of file instead");
            std::process::exit(1);
        }
    };
    expect_punctuation!(Token::RightParenthesis, tokens.next());
    ast
}

fn parse_call_expression(mut tokens: &mut std::slice::Iter<Token>) -> ast::Node {
    match tokens.next() {
        Some(&Token::Identifier(ref identifier)) => {
            ast::Node::CallExpression {
                target: Box::new(ast::Node::IdentifierReference(identifier.clone())),
                arguments: Box::new(parse_argument_list(&mut tokens)),
            }
        },
        Some(t) => {
            println!("Error: expected 'Token::Identifier' but got '{:?}' instead", t);
            std::process::exit(1);
        },
        None => {
            println!("Error: expected 'Token::Identifier' but reached end of file instead");
            std::process::exit(1);
        }
    }
}

fn parse(tokens: &Vec<Token>) -> ast::Node {
    // TODO: Obviously this is incorrect, but is enough to get hello world example working
    let mut tokens = tokens.iter();
    let ast = parse_call_expression(&mut tokens);
    expect_punctuation!(Token::Semicolon, tokens.next());
    ast
}

fn main() {
    let input = "print('Hello, world!');";
    let tokens = scan(input);
    let ast = parse(&tokens);
    println!("Input:\n\n\t{}\n\n", input);
    println!("Tokens:\n\n\t{:?}\n\n", tokens);
    println!("Ast:\n\n\t{:?}\n", ast);
}
