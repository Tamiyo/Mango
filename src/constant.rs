use crate::function::Function;

#[derive(Debug, Clone)]
pub enum Constant {
    Number(f64),
    String(String),
    Boolean(bool),
    None,
    Function(Function)
}

impl From<f64> for Constant {
    fn from(item: f64) -> Self { Constant::Number(item) }
}

impl From<&str> for Constant {
    fn from(item: &str) -> Self { Constant::String(String::from(item)) }
}

impl Constant {
    pub fn is_falsey(&self) -> bool {
        match self {
            Constant::Boolean(boolean) => !*boolean,
            Constant::None => true,
            _ => false,
        }
    }
}