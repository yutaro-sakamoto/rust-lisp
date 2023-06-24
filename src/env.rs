use crate::object::Object;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
pub struct Env {
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String, Object>,
}

//pub fn find_symbol(env: &Env, s: String) -> Result<Object, String> {
//    match env.borrow_mut().get(s) {
//        None => Err(format!("Unbound symbol: {}", s)),
//        Some(s) => Ok(s),
//    }
//}
