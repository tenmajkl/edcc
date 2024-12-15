use std::iter::Peekable;

use regex::Regex;

#[derive(Debug)]
pub enum Token {
    Number(u8),
    Char(char),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Or,
    And,
    Not,
    Identifier(String),
    If,
    For,
    While,
    OpenParentheses,
    CloseParentheses,
    OpenBrace,
    CloseBrace,
    Comma,
    Semicolon,
}

impl Token {
    pub fn from_id(id: u8, value: &str) -> Option<Token> {
        match id {
             0 => Some(Token::Number(value.parse().unwrap())),                     
             1 => Some(Token::Plus),
             2 => Some(Token::Minus),
             3 => Some(Token::Star),
             4 => Some(Token::Slash),
             5 => Some(Token::Percent),
             6 => Some(Token::Or),
             7 => Some(Token::And),
             8 => Some(Token::Not),
             9 => Some(Token::If),
             10 => Some(Token::For),
             11 => Some(Token::While),
             12 => Some(Token::OpenParentheses),
             13 => Some(Token::CloseParentheses),
             14 => Some(Token::OpenBrace),
             15 => Some(Token::CloseBrace),
             16 => Some(Token::Comma),
             17 => Some(Token::Semicolon),
             18 => Some(Token::Char(value.chars().nth(0).unwrap())),
             19 => Some(Token::Identifier(value.to_string())),
             _ => None
        }
    }
}

pub struct Error {
    line: u16,
    message: String
}


pub fn lex(input: &String) -> Result<Vec<Token>, Error> {
    const RE_LEN: u8 = 21;
    let re = Regex::new(r"
        (?<NUMBER>[0-9]+)
        |(?<PLUS>\+)
        |(?<MINUS>-)
        |(?<STAR>\*)
        |(?<SLASH>/)
        |(?<PERCENT>%)
        |(?<OR>\|\|)
        |(?<AND>&&)
        |(?<NOT>!)
        |(?<IF>if)
        |(?<FOR>for)
        |(?<WHILE>while)
        |(?<OPEN_PARENTHESES>\()
        |(?<CLOSE_PARENTHESES>\))
        |(?<OPEN_BRACE>\{)
        |(?<CLOSE_BRACE>\})
        |(?<COMMA>,)
        |(?<SEMICOLON>;)
        |(?<CHAR>'.')
        |(?<IDENTIFIER>[a-zA-Z_][a-zA-Z0-9_]*)
        |(?<NEWLINE>\n)
        |(?<WHITESPACE>\s+)
        |(?<ERROR>(.+))
        ".replace("\n", "").replace(" ", "").as_str()).unwrap(); // dirty hack to make in on one line lol

//    let re = Regex::new("(?<NUMBER>[0-9]+)|(?<SEX>\\+)").unwrap();


    for token in re.captures_iter(input) {
        let mut id: u8 = 0;
        for re_match in token.iter().skip(1) {
            if re_match != None {
                println!("{:?}", Token::from_id(id, re_match.unwrap().as_str()));
                continue;
            }
            id += 1;

        }

        if id == RE_LEN {
            return Err(Error {line: 0, message: "unexpected token".to_string()});
        }
    }

    Ok(Vec::new())
}
