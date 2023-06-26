use crate::env::*;
use crate::parser::*;

type EvalResult = Result<(), String>;

fn eval_list(list: &Vec<Object>, env: &mut Env) -> EvalResult {
    match &list[0] {
        Object::Symbol(s) => match s.as_str() {
            "+" | "-" | "*" | "/" | "<" | ">" | "=" | "!=" => eval_binary_op(list, env),
            "if" => eval_if(list, env),
            "lambda" => eval_function_definition(list, env),
            _ => eval_function_call(&s, list, env),
        },
        _ => Err(format!("fail: eval_list")),
    }
}

fn eval_binary_op(list: &Vec<Object>, env: &mut Env) -> EvalResult {
    Err(format!("fail: eval_binary_op is not implemented"))
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
