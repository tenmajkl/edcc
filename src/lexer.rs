
use regex::Regex;

// todo \n
#[derive(Debug)]
pub enum TokenKind {
    Number(u8),
    Char(char),
    Addition(char),
    Multiplication(char),
    Percent,
    Logical(String),
    Not,
    Set(String),
    DoubleAddition(String),
    Compare(String),
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

impl TokenKind {
    pub fn from_id(id: u8, value: &str) -> Option<TokenKind> {
        match id {
            0 => Some(TokenKind::Number(value.parse().unwrap())),
            1 => Some(TokenKind::Set(value.to_string())),
            2 => Some(TokenKind::DoubleAddition(value.to_string())),
            3 => Some(TokenKind::Addition(value.chars().nth(0).unwrap())),
            4 => Some(TokenKind::Multiplication(value.chars().nth(0).unwrap())),
            5 => Some(TokenKind::Percent),
            6 => Some(TokenKind::Logical(value.to_string())),
            7 => Some(TokenKind::Not),
            8 => Some(TokenKind::Compare(value.to_string())),
            9 => Some(TokenKind::If),
            10 => Some(TokenKind::For),
            11 => Some(TokenKind::While),
            12 => Some(TokenKind::OpenParentheses),
            13 => Some(TokenKind::CloseParentheses),
            14 => Some(TokenKind::OpenBrace),
            15 => Some(TokenKind::CloseBrace),
            16 => Some(TokenKind::Comma),
            17 => Some(TokenKind::Semicolon),
            18 => Some(TokenKind::Char(value.chars().nth(1).unwrap())),
            19 => Some(TokenKind::Identifier(value.to_string())),
             _ => None
        }
    }
}

#[derive(Debug)]
pub struct Error {
    line: u16,
    message: String
}

#[derive(Debug)]
pub struct Token {
    kind: TokenKind,
    line: u16, 
    pos: u16,
}

pub fn lex(input: &String) -> Result<Vec<Token>, Error> {
    let re = Regex::new(r"
        (?<NUMBER>[0-9]+)
        |(?<SET>=|\+=|\-=|\*=|/=)
        |(?<DOUBLE_ADDITION>\+\+|\-\-)
        |(?<ADDITION>\+|\-)
        |(?<MULTIPLICATION>\*|/)
        |(?<PERCENT>%)
        |(?<LOGICAL>\|\||&&)
        |(?<NOT>!)
        |(?<COMPARE>==|!=|\<|\>|\<=|\>=)
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

    let mut result = Vec::new();
    let mut line:u16 = 0;
    let mut pos:u16 = 0; 
    for token in re.captures_iter(input) {
        let mut id: u8 = 0;
        for re_match in token.iter().skip(1) {
            if re_match != None {
                if id == 20 { // NEWLINE
                    line += 1;
                    pos = 0;
                    break;
                }

                if id == 21 { // WHITESPACE
                    break;
                }

                if id == 22 { // ERROR
                    return Err(Error {line: 0, message: "unexpected token".to_string()});
                }

                
                result.push(
                    Token {
                        kind: TokenKind::from_id(id, re_match.unwrap().as_str()).unwrap(),
                        line, 
                        pos,
                    }
                );

                pos += re_match.unwrap().len() as u16;
                break;
            }
            id += 1;

        }
    }

    Ok(result)
}
