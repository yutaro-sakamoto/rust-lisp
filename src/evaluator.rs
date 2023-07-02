use crate::env::*;
use crate::object::*;
use std::cell::RefCell;
use std::rc::Rc;

type EvalResult = Result<Object, String>;

pub fn eval(object: &Object, env: &mut Rc<RefCell<Env>>) -> EvalResult {
    match object {
        Object::List(list) => eval_list(list, env),
        other => Ok(other.clone()),
    }
}

fn eval_list(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> EvalResult {
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

fn eval_binary_op(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> EvalResult {
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

fn eval_if(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> EvalResult {
    if list.len() != 4 {
        return Err(format!("Invalid number of arguments for if statement"));
    }

    let cond_obj = eval(&list[1], env)?;
    let cond = match cond_obj {
        Object::Bool(b) => b,
        _ => return Err(format!("Condition must be a boolean")),
    };

    if cond {
        return eval(&list[2], env);
    } else {
        return eval(&list[3], env);
    }
}

fn eval_function_definition(list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> EvalResult {
    let params = match &list[1] {
        Object::List(list) => {
            let mut params = Vec::new();
            for param in list {
                match param {
                    Object::Symbol(s) => params.push(s.clone()),
                    _ => return Err(format!("Invalid lambda parameter")),
                }
            }
            params
        }
        _ => return Err(format!("Invalid lambda")),
    };

    let body = match &list[2] {
        Object::List(list) => list.clone(),
        _ => return Err(format!("Invalid lambda")),
    };
    Ok(Object::Lambda(params, body))
}

fn eval_function_call(s: &String, list: &Vec<Object>, env: &mut Rc<RefCell<Env>>) -> EvalResult {
    let lambda = env.borrow().get(s);
    match lambda {
        Err(_) => Err(format!("Unbound symbol: {}", s)),
        Ok(func) => match func {
            Object::Lambda(params, body) => {
                let mut new_env = Rc::new(RefCell::new(Env::extend(env.clone())));
                for (i, param) in params.iter().enumerate() {
                    let val = eval(&list[i+1], env)?;
                    new_env.borrow_mut().put(param, val);
                }
                return eval(&Object::List(body), &mut new_env);
            }
            _ => Err(format!("Not a lambda: {}", s)),
        }
    }  
}

mod tests {
    use super::*;

    use crate::parser::*;
    use std::collections::HashMap;

    #[test]
    fn test_evaluator() {
        let object = parse("(+ 1 2)").unwrap();
        let mut env = Rc::new(RefCell::new(Env::new()));
        let result = eval(&object, &mut env);
        assert_eq!(result.unwrap_or(Object::Void), Object::Integer(3));
    }
}
