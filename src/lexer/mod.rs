use std::collections::VecDeque;

use log::warn;
use token::{Token, TokenKind};

use crate::common::{error::Result, flag::{Flag, FlagKind}, text::Location};

pub mod token;

pub fn tokenise(source: String) -> Result<Vec<Token>> {
    let chars       = source.chars().collect::<Vec<char>>();

    let mut cursor  = Location::zero(source);
    let mut tokens = Vec::new();

    while cursor.index() < chars.len() {
        let next = next(&chars, &mut cursor);
        if let Some(next) = next {
            tokens.push(next);
        }
    }

    Ok(tokens)
}

fn next(chars: &Vec<char>, cursor: &mut Location) -> Option<Token> {
    match peek(chars, cursor) {
        c => {
            let ret = Some(Token::new(
                    TokenKind::Null, 
                    cursor.clone().span_to(cursor.clone()), 
                    Flag::Error { 
                        kind: FlagKind::Lexical, 
                        error: format!("unexpected character: {}", c)
                    }
            ));
            cursor.advance();
            ret
        }
    }
}

fn peek(chars: &Vec<char>, cursor: &mut Location) -> char {
    chars[cursor.index()]
}

fn pop(chars: &Vec<char>, cursor: &mut Location) -> char {
    let c = peek(chars, cursor);
    cursor.advance();
    c
}
