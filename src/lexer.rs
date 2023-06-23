use std::collections::*;

#[derive(Debug, Clone, PartialEq)]

pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

pub fn tokenize(program: &str) -> VecDeque<Token> {
    let program2 = program.replace("(", " ( ").replace(")", " ) ");
    let words = program2.split_whitespace();
    let mut tokens = VecDeque::new();
    for word in words {
        match word {
            "(" => tokens.push_back(Token::LParen),
            ")" => tokens.push_back(Token::RParen),
            _ => match word.parse::<i64>() {
                Ok(i) => tokens.push_back(Token::Integer(i)),
                Err(_) => tokens.push_back(Token::Symbol(word.to_string())),
            },
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let tokens = tokenize("(+ 1 2)");
        assert_eq!(
            tokens,
            VecDeque::from([
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ])
        )
    }
}
