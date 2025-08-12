use std::str::Chars;

use crate::token::TokenType;

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

fn skip_whitespaces<'a>(source: &'a Chars, start: usize) -> &'a Chars {
    let current = 
}

pub fn lex(source: &str) -> Vec<TokenType> {
    let mut vec: Vec<TokenType> = vec![];
    let line: usize = 0;
    let current: usize = 0;
    let chars = source.chars();

    for (index, c) in chars.enumerate() {
        match c {
            ';' => vec.push(TokenType::SemicolonToken),
            '(' => vec.push(TokenType::LeftParenToken),
            ')' => vec.push(TokenType::RightParenToken),
            '{' => vec.push(TokenType::LeftBraceToken),
            '}' => vec.push(TokenType::RightBraceToken),
            '[' => vec.push(TokenType::LeftBracketToken),
            ']' => vec.push(TokenType::RightBracketToken),
            ',' => vec.push(TokenType::CommaToken),
            '~' => vec.push(TokenType::TildeToken),
            ':' => vec.push(TokenType::ColonToken),
            '@' => vec.push(TokenType::AtToken),
            '.' => {}
            _ => vec.push(TokenType::Unknown),
        }
    }

    vec
}
