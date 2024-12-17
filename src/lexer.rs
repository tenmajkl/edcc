
use regex::Regex;

// todo add eq, neq, setz, pack it further, by priority \n
#[derive(Debug)]
pub enum TokenKind {
    Number(u8),
    Char(char),
    Addition(char),
    Multiplication(char),
    Percent,
    Logical(String),
    Not,
    Set,
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
             0 => Some(TokenKind::Plus),
             0 => Some(TokenKind::Minus),
             0 => Some(TokenKind::Star),
             0 => Some(TokenKind::Slash),
             0 => Some(TokenKind::Percent),
             0 => Some(TokenKind::Or),
             0 => Some(TokenKind::And),
             0 => Some(TokenKind::Not),
             0 => Some(TokenKind::LessThan),
             0 => Some(TokenKind::GreaterThan),
             0 => Some(TokenKind::LessThanEqual),
             0 => Some(TokenKind::GreaterThanEqual),
             0 => Some(TokenKind::If),
             0 => Some(TokenKind::For),
             0 => Some(TokenKind::While),
             0 => Some(TokenKind::OpenParentheses),
             0 => Some(TokenKind::CloseParentheses),
             0 => Some(TokenKind::OpenBrace),
             0 => Some(TokenKind::CloseBrace),
             0 => Some(TokenKind::Comma),
             0 => Some(TokenKind::Semicolon),
             0 => Some(TokenKind::Char(value.chars().nth(1).unwrap())),
             0 => Some(TokenKind::Identifier(value.to_string())),
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
        |(?<ADDITION>\+\-)
        |(?<MULTIPLICATION>\*/)
        |(?<PERCENT>%)
        |(?<LOGICAL>\|\||&&)
        |(?<NOT>!)
        |(?<SET>=|\+=|\-=|\*=|/=)
        |(?<COMPARE>)
        |(?<LT>\<)
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
                if id == 25 {
                    break;
                }

                if id == 26 {
                    return Err(Error {line: 0, message: "unexpected token".to_string()});
                }


                if id == 24 {
                    line += 1;
                    pos = 0;
                    break;
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
