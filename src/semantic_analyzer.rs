use crate::core::LexerResult;

pub struct SemanticAnalyzer {}

impl SemanticAnalyzer {
    fn can_convert(&self, a: LexerResult, b: LexerResult) -> i32 {
        return 0;
    }

    pub fn analyze(&self, a: LexerResult, b: LexerResult) -> (bool, LexerResult, LexerResult) {
        if a.token_type == b.token_type {
            return (true, a, b);
        } else if self.can_convert(a.clone(), b.clone()) == 1 {
            let mut ta = a.clone();
            ta.token_type = b.token_type;
            return (true, ta, b);
        } else if self.can_convert(a.clone(), b.clone()) == 2 {
            let mut tb = b.clone();
            tb.token_type = a.token_type;
            return (true, a, tb);
        } else {
            return (false, a, b);
        }
    }
}