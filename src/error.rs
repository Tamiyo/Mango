pub struct CompileError {
    pub error: String
}

impl From<&str> for CompileError {
    fn from(error: &str) -> CompileError {
        CompileError {
            error: error.to_string(),
        }
    }
}

impl From<String> for CompileError {
    fn from(error: String) -> CompileError {
        CompileError {
            error,
        }
    }
}

pub struct ParseError {
    pub error: String,
}

impl From<&str> for ParseError {
    fn from(error: &str) -> ParseError {
        ParseError {
            error: error.to_string(),
        }
    }
}

impl From<String> for ParseError {
    fn from(error: String) -> ParseError {
        ParseError {
            error,
        }
    }
}