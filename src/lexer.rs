#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Integer(i64),
    Symbol(String),
    LParen,
    RParen,
}

pub fn tokenize(program: &str) -> Vec<Token> {
    let program2 = program.replace("(", " ( ").replace(")", " ) ");
    let words = program2.split_whitespace();
    let mut tokens: Vec<Token> = Vec::new();
    for word in words {
        match word {
            "(" => tokens.push(Token::LParen),
            ")" => tokens.push(Token::RParen),
            _ => match word.parse::<i64>() {
                Ok(i) => tokens.push(Token::Integer(i)),
                Err(_) => tokens.push(Token::Symbol(word.to_string())),
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
            vec![
                Token::LParen,
                Token::Symbol("+".to_string()),
                Token::Integer(1),
                Token::Integer(2),
                Token::RParen,
            ]
        )
    }
}
