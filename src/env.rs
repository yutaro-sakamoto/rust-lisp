use crate::object::Object;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub struct Env {
    pub parent: Option<Rc<RefCell<Env>>>,
    pub vars: HashMap<String, Object>,
}

fn find_symbol(env: &Rc<RefCell<Env>>, s: String) -> Result<Object, String> {
    let e = env.as_ref().borrow();
    match e.vars.get(&s) {
        None => match &e.parent {
            None => Err(format!("Unbound symbol: {}", s)),
            Some(parent_env) => find_symbol(&parent_env, s),
        },
        Some(object) => Ok(object.clone()),
    }
}
