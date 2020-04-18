#[derive(Debug, Clone)]
pub enum Constant {
    Number(f64),
    String(String),
}

impl From<f64> for Constant {
    fn from(item: f64) -> Self { Constant::Number(item) }
}

impl From<&str> for Constant {
    fn from(item: &str) -> Self { Constant::String(String::from(item)) }
}