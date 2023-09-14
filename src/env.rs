use crate::object::Object;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, PartialEq, Default)]
pub struct Env {
    pub parent: Option<Rc<RefCell<Env>>>,
    pub vars: HashMap<String, Object>,
}

impl Env {
    pub fn get(&self, s: &str) -> Result<Object, String> {
        match self.vars.get(s) {
            None => match &self.parent {
                None => Err(format!("Unbound symbol: {}", s)),
                Some(parent_env) => parent_env.as_ref().borrow().get(s),
            },
            Some(object) => Ok(object.clone()),
        }
    }

    pub fn put(&mut self, s: &str, object: Object) {
        self.vars.insert(s.to_string(), object);
    }

    pub fn new() -> Self {
        Env {
            parent: None,
            vars: HashMap::new(),
        }
    }

    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env {
            parent: Some(parent),
            vars: HashMap::new(),
        }
    }
}
