pub enum Object {
    Void,
    Lmabda(Vec<String>, Vec<Object>),
    Bool(bool),
    Integer(i64),
    Symbol(String),
    List(Vec<Object>),
}
