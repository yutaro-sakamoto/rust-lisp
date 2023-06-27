use crate::lexer::*;
use crate::object::*;

use std::collections::*;

//#[derive(Debug, Clone, PartialEq)]
//pub enum Object {
//    Void,
//    Integer(i64),
//    Bool(bool),
//    Symbol(String),
//    Lambda(Vec<String>, Vec<Object>),
//    List(Vec<Object>),
//}

#[derive(Debug, Clone, PartialEq)]
pub struct ParseError {
    err: String,
}

pub fn parse(source: &str) -> Result<Object, ParseError> {
    parse_list(&mut tokenize(source))
}

pub fn parse_list(tokens: &mut VecDeque<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop_front();
    if token != Some(Token::LParen) {
        return Err(ParseError {
            err: format!("Insuffient tokens"),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        match tokens.pop_front() {
            None => {
                return Err(ParseError {
                    err: format!("Insufficient tokens"),
                })
            }
            Some(t) => match t {
                Token::Integer(n) => list.push(Object::Integer(n)),
                Token::Symbol(s) => {
                    list.push(Object::Symbol(s));
                }
                Token::LParen => {
                    tokens.push_front(Token::LParen);
                    let sub_list = parse_list(tokens)?;
                    list.push(sub_list);
                }
                Token::RParen => {
                    return Ok(Object::List(list));
                }
            },
        }
    }
    Err(ParseError {
        err: format!("Insufficient tokens!!!!!"),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let tokens = parse("(+ 1 2)").unwrap();
        assert_eq!(
            tokens,
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ])
        )
    }
}
