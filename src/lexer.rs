use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;
use std::str::Chars;

use crate::token::{Token, TokenType};

// DotToken, .
// DotDotDotToken, ...
// EqualsToken, =
// EqualsEqualsToken, ==
// EqualsEqualsEqualsToken, ===
// EqualsGreaterThanToken, =<
// PlusToken, +
// PlusPlusToken, ++
// PlusEqualsToken, +=
// MinusToken, -
// MinusMinusToken, --
// MinusEqualsToken, -=
// AsteriskToken, *
// AsteriskEqualsToken, *=
// AsteriskAsteriskToken, **
// AsteriskAsteriskEqualsToken, **=
// SlashToken, /
// SlashEqualsToken, /=
// PercentToken, %
// PercentEqualsToken, %=
// ExclamationToken, !
// ExclamationEqualsToken, !=
// ExclamationEqualsEqualsToken, !==
// QuestionToken, ?
// QuestionQuestionToken, ??
// QuestionQuestionEqualsToken, ??=
// QuestionDotToken, ?.
// LessThanToken, <
// LessThanEqualsToken, <=
// LessThanSlashToken, </
// LessThanLessThanToken, <<
// LessThanLessThanEqualsToken, <<=
// GreaterThanToken, >
// GreaterThanEqualsToken, >=
// GreaterThanGreaterThanToken, >>
// GreaterThanGreaterThanEqualsToken, >>=
// GreaterThanGreaterThanGreaterThanToken, >>>
// GreaterThanGreaterThanGreaterThanEqualsToken, >>>=
// AmpersandToken, &
// AmpersandEqualsToken, &=
// AmpersandAmpersandToken, &&
// AmpersandAmpersandEqualsToken, &&=
// BarToken, |
// BarEqualsToken, |=
// BarBarToken, ||
// BarBarEqualsToken, ||=
// CaretToken, ^
// CaretEqualsToken, ^=

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

#[derive(Debug)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,

    line: usize,
    current: usize,
    start: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            line: 1,
            current: 0,
            start: 0,
        }
    }

    fn lex(&mut self) -> impl Coroutine<Yield = Token, Return = ()> + '_ {
        loop {
            self.skip();
        }
    }

    fn skip(&mut self) {
        while let Some(&c) = self.source.peek() {
            match c {
                ' ' | '\r' | '\t' => {
                    self.current += 1;
                    _ = self.source.next();
                }
                '\n' => {
                    self.current = 1;
                    self.line += 1;
                    _ = self.source.next();
                }
                _ => break,
            }
        }
    }

    fn next_token(&mut self) -> Result<Token, LexerError> {
        let Some(c) = self.source.peek() else {
            return Ok(Token::new(TokenType::EndOfFile, self.line, self.current));
        };
    }
}

pub fn lex(source: &str) -> Vec<TokenType> {
    let mut vec: Vec<TokenType> = vec![];
    let mut line: usize = 0;
    let mut current: usize = 0;
    let chars = source.chars();

    for c in chars {
        match c {
            ' ' | '\r' | '\t' => current += 1,
            '\n' => {
                current += 1;
                line += 1;
            }
            ';' => {
                vec.push(TokenType::SemicolonToken);
                current += 1;
            }
            '(' => {
                vec.push(TokenType::LeftParenToken);
                current += 1;
            }
            ')' => {
                vec.push(TokenType::RightParenToken);
                current += 1;
            }
            '{' => {
                vec.push(TokenType::LeftBraceToken);
                current += 1;
            }
            '}' => {
                vec.push(TokenType::RightBraceToken);
                current += 1;
            }
            '[' => {
                vec.push(TokenType::LeftBracketToken);
                current += 1;
            }
            ']' => {
                vec.push(TokenType::RightBracketToken);
                current += 1;
            }
            ',' => {
                vec.push(TokenType::CommaToken);
                current += 1;
            }
            '~' => {
                vec.push(TokenType::TildeToken);
                current += 1;
            }
            ':' => {
                vec.push(TokenType::ColonToken);
                current += 1;
            }
            '@' => {
                vec.push(TokenType::AtToken);
                current += 1;
            }
            '.' => {}
            _ => vec.push(TokenType::Unknown),
        }
    }

    vec
}
