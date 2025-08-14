use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub enum LexerError {
    UnexpectedCharacter { line: usize, column: usize },
    StringNotTerminated { line: usize, column: usize },
    InvalidNumber { line: usize, column: usize },
    CommentNotTerminated { line: usize, column: usize },
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnexpectedCharacter { line, column } => {
                write!(f, "[{}:{}] Unexpected Character", line, column)
            }
            LexerError::StringNotTerminated { line, column } => {
                write!(f, "[{}:{}] Unterminated string literal", line, column)
            }
            LexerError::InvalidNumber { line, column } => {
                write!(f, "[{}:{}] Invalid number literal", line, column)
            }
            LexerError::CommentNotTerminated { line, column } => {
                write!(f, "[{}:{}] Unterminated comment", line, column)
            }
        }
    }
}

impl Error for LexerError {}

impl LexerError {
    pub fn report(&self) {
        eprintln!("{}", self);
    }
}
