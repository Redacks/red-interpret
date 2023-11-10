#[derive(Clone)]
pub enum RuntimeTypes {
    Number(i64),
    String(String),
}

impl RuntimeTypes {
    pub fn as_number(&self) -> Option<i64> {
        match self {
            RuntimeTypes::Number(value) => Some(*value),
            RuntimeTypes::String(string) => {
                if let Ok(value) = string.parse::<i64>() {
                    Some(value)
                } else {
                    None
                }
            }
        }
    }
    pub fn as_string(&self) -> Option<String> {
        match self {
            RuntimeTypes::Number(val) => Some(val.to_string()),
            RuntimeTypes::String(val) => Some(val.to_owned()),
        }
    }
}
