use crate::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

pub struct ParseError {
    err: String,
}

pub fn parse_list(tokens: Vec<Token>) -> Result<Object, ParseError> {
    let token = tokens.pop();
    if token != Some(Token::LParen) {
        return Err(ParseError);
    }

    let mut list: Vec<Object> = Vec::new();
    while !token.is_empty() {
        match tokens.pop() {
            None => Err(ParseError {
                err: format!("Insufficient tokens"),
            }),
            Some(t) => match t {
                Token::Integer(n) => list.push(Object::Integer(n)),
                Token::Symbol(s) => list.push(Object::Symbol(n)),
                Token::LParen => {
                    tokens.push(Token::LParen);
                    let sub_list = parse_list(tokens)?;
                    list.push(sub_list);
                }
                Token::RParen => {
                    return Ok(Object::List(list));
                }
            },
        }
    }
}

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_tokenize() {
//        let tokens = tokenize("(+ 1 2)");
//        assert_eq!(
//            tokens,
//            vec![
//                Token::LParen,
//                Token::Symbol("+".to_string()),
//                Token::Integer(1),
//                Token::Integer(2),
//                Token::RParen,
//            ]
//        )
//    }
//}
