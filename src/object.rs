#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Void,
    Lambda(Vec<String>, Vec<Object>),
    Bool(bool),
    Integer(i64),
    Symbol(String),
    List(Vec<Object>),
}
