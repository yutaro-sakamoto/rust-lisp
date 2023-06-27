use crate::env::*;
use crate::object::*;

type EvalResult = Result<Object, String>;

pub fn eval(object: Object, env: &mut Env) -> EvalResult {
    match &object {
        Object::List(list) => eval_list(list, env),
        other => Ok(other.clone()),
    }
}

fn eval_list(list: &Vec<Object>, env: &mut Env) -> EvalResult {
    match &list[0] {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => eval_binary_op(list, env),
            "if" => eval_if(list, env),
            "lambda" => eval_function_definition(list, env),
            _ => eval_function_call(&s, list, env),
        },
        _ => Err(format!("The first element of the list must be a symbol")),
    }
}

fn eval_binary_op(list: &Vec<Object>, env: &mut Env) -> EvalResult {
    if list.len() != 3 {
        return Err(format!("A binary operator must take exactly two arguments"));
    }
    match &list[0] {
        Object::Symbol(s) => match s.as_str() {
            "+" => {
                if let (Object::Integer(a), Object::Integer(b)) = (&list[1], &list[2]) {
                    Ok(Object::Integer(a + b))
                } else {
                    Err(format!("The binary operator '+' must take two integers"))
                }
            }
            _ => Err(format!("eval_binary_op is implemented partially")),
        },
        _ => Err(format!("eval_binary_op is implemented partially")),
    }
}

fn eval_if(list: &Vec<Object>, env: &mut Env) -> EvalResult {
    Err(format!("fail: eval_if is not implemented"))
}

fn eval_function_definition(list: &Vec<Object>, env: &mut Env) -> EvalResult {
    Err(format!("fail: eval_function_definition is not implemented"))
}

fn eval_function_call(s: &String, list: &Vec<Object>, env: &mut Env) -> EvalResult {
    Err(format!("fail: eval_functoin_call is not implemented"))
}

mod tests {
    use super::*;

    use crate::parser::*;
    use std::collections::HashMap;

    #[test]
    fn test_evaluator() {
        let object = parse("(+ 1 2)").unwrap();
        let mut env = Env {
            parent: None,
            vars: HashMap::new(),
        };
        let result = eval(object, &mut env);
        assert_eq!(result.unwrap_or(Object::Void), Object::Integer(3));
    }
}
