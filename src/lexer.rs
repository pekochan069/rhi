use std::iter::Peekable;
use std::str::Chars;

use crate::error::LexerError;
use crate::token::{StrSpan, Token, TokenType};

#[derive(Debug)]
pub struct Lexer<'a> {
    source: Peekable<Chars<'a>>,

    line: usize,
    start: usize,
    current: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source: source.chars().peekable(),
            line: 1,
            current: 0,
            start: 0,
        }
    }

    pub fn process(&mut self) -> impl IntoIterator<Item = Result<Token, LexerError>> {
        self.lex().into_iter()
    }

    pub fn lex(&mut self) -> impl Iterator<Item = Result<Token, LexerError>> {
        gen {
            loop {
                self.skip();

                match self.next_token() {
                    Ok(token) => {
                        if token.token_type == TokenType::EndOfFile {
                            return;
                        }

                        yield Ok(token);
                    }
                    Err(e) => yield Err(e),
                }
            }
        }
    }

    pub fn skip(&mut self) {
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

    pub fn advance(&mut self) -> Option<char> {
        if let Some(c) = self.source.next() {
            self.current += c.len_utf8();
            Some(c)
        } else {
            None
        }
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        match self.source.peek().copied() {
            Some(c) if c == expected => {
                // Consume the matched character and advance position counters
                _ = self.source.next();
                self.current += expected.len_utf8();
                true
            }
            _ => false,
        }
    }

    pub fn new_token(
        &self,
        token_type: TokenType,
        lexeme: Option<StrSpan>,
        literal_value: Option<StrSpan>,
    ) -> Token {
        Token::new(
            token_type,
            self.line,
            self.start,
            self.current + 1,
            lexeme,
            literal_value,
        )
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.start = self.current + 1;

        let Some(c) = self.advance() else {
            return Ok(self.new_token(TokenType::EndOfFile, None, None));
        };

        match c {
            ';' => Ok(self.new_token(TokenType::SemicolonToken, None, None)),
            '(' => Ok(self.new_token(TokenType::LeftParenToken, None, None)),
            ')' => Ok(self.new_token(TokenType::RightParenToken, None, None)),
            '{' => Ok(self.new_token(TokenType::LeftBraceToken, None, None)),
            '}' => Ok(self.new_token(TokenType::RightBraceToken, None, None)),
            '[' => Ok(self.new_token(TokenType::LeftBracketToken, None, None)),
            ']' => Ok(self.new_token(TokenType::RightBracketToken, None, None)),
            ',' => Ok(self.new_token(TokenType::CommaToken, None, None)),
            '~' => Ok(self.new_token(TokenType::TildeToken, None, None)),
            ':' => Ok(self.new_token(TokenType::ColonToken, None, None)),
            '@' => Ok(self.new_token(TokenType::AtToken, None, None)),
            '.' => {
                if self.match_char('.') {
                    if self.match_char('.') {
                        Ok(self.new_token(TokenType::DotDotDotToken, None, None))
                    } else {
                        Err(LexerError::UnexpectedCharacter {
                            line: self.line,
                            column: self.start,
                        })
                    }
                } else {
                    Ok(self.new_token(TokenType::DotToken, None, None))
                }
            }
            '+' => {
                if self.match_char('+') {
                    Ok(self.new_token(TokenType::PlusPlusToken, None, None))
                } else if self.match_char('=') {
                    Ok(self.new_token(TokenType::PlusEqualsToken, None, None))
                } else {
                    Ok(self.new_token(TokenType::PlusToken, None, None))
                }
            }
            '-' => {
                if self.match_char('-') {
                    Ok(self.new_token(TokenType::MinusMinusToken, None, None))
                } else if self.match_char('=') {
                    Ok(self.new_token(TokenType::MinusEqualsToken, None, None))
                } else {
                    Ok(self.new_token(TokenType::MinusToken, None, None))
                }
            }
            '*' => {
                if self.match_char('=') {
                    Ok(self.new_token(TokenType::AsteriskEqualsToken, None, None))
                } else if self.match_char('*') {
                    if self.match_char('=') {
                        Ok(self.new_token(TokenType::AsteriskAsteriskEqualsToken, None, None))
                    } else {
                        Ok(self.new_token(TokenType::AsteriskAsteriskToken, None, None))
                    }
                } else {
                    Ok(self.new_token(TokenType::AsteriskToken, None, None))
                }
            }
            '/' => {
                if self.match_char('/') {
                    loop {
                        let Some(next_c) = self.advance() else {
                            return Ok(self.new_token(
                                TokenType::MultiLineCommentTrivia,
                                Some(StrSpan {
                                    start: self.start + 2,
                                    end: self.current + 1,
                                }),
                                Some(StrSpan {
                                    start: self.start,
                                    end: self.current + 1,
                                }),
                            ));
                        };
                        if next_c == '\n' {
                            break;
                        }
                    }

                    let new_token = self.new_token(
                        TokenType::MultiLineCommentTrivia,
                        Some(StrSpan {
                            start: self.start + 2,
                            end: self.current + 1,
                        }),
                        Some(StrSpan {
                            start: self.start,
                            end: self.current + 1,
                        }),
                    );
                    self.line += 1;
                    self.current = 0;
                    Ok(new_token)
                } else if self.match_char('*') {
                    let start_line = self.line;
                    let mut is_jsdoc = false;

                    if self.match_char('*') {
                        is_jsdoc = true;
                    }

                    loop {
                        let Some(next_c) = self.advance() else {
                            return Err(LexerError::CommentNotTerminated {
                                line: start_line,
                                column: self.current,
                            });
                        };

                        if next_c == '\n' {
                            self.current = 0;
                            self.line += 1;
                            continue;
                        } else if next_c == '*' {
                            if self.match_char('/') {
                                break;
                            }
                        }
                    }

                    if is_jsdoc {
                        Ok(Token::new(
                            TokenType::JSDoc,
                            start_line,
                            self.start,
                            self.current,
                            Some(StrSpan {
                                start: self.start + 3,
                                end: self.current,
                            }),
                            Some(StrSpan {
                                start: self.start,
                                end: self.current,
                            }),
                        ))
                    } else {
                        Ok(Token::new(
                            TokenType::MultiLineCommentTrivia,
                            start_line,
                            self.start,
                            self.current,
                            Some(StrSpan {
                                start: self.start + 2,
                                end: self.current,
                            }),
                            Some(StrSpan {
                                start: self.start,
                                end: self.current,
                            }),
                        ))
                    }
                } else if self.match_char('=') {
                    Ok(self.new_token(TokenType::SlashEqualsToken, None, None))
                } else {
                    Ok(self.new_token(TokenType::SlashToken, None, None))
                }
            }
            '%' => {
                if self.match_char('=') {
                    Ok(self.new_token(TokenType::PercentEqualsToken, None, None))
                } else {
                    Ok(self.new_token(TokenType::PercentToken, None, None))
                }
            }
            '=' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        Ok(self.new_token(TokenType::EqualsEqualsEqualsToken, None, None))
                    } else {
                        Ok(self.new_token(TokenType::EqualsEqualsToken, None, None))
                    }
                } else if self.match_char('>') {
                    Ok(self.new_token(TokenType::EqualsGreaterThanToken, None, None))
                } else {
                    Ok(self.new_token(TokenType::EqualsToken, None, None))
                }
            }
            '!' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        Ok(self.new_token(TokenType::ExclamationEqualsEqualsToken, None, None))
                    } else {
                        Ok(self.new_token(TokenType::ExclamationEqualsToken, None, None))
                    }
                } else {
                    Ok(self.new_token(TokenType::ExclamationToken, None, None))
                }
            }
            '?' => {
                if self.match_char('?') {
                    if self.match_char('=') {
                        Ok(self.new_token(TokenType::QuestionQuestionEqualsToken, None, None))
                    } else {
                        Ok(self.new_token(TokenType::QuestionQuestionToken, None, None))
                    }
                } else if self.match_char('.') {
                    Ok(self.new_token(TokenType::QuestionDotToken, None, None))
                } else {
                    Ok(self.new_token(TokenType::QuestionToken, None, None))
                }
            }
            '<' => {
                if self.match_char('=') {
                    Ok(self.new_token(TokenType::LessThanEqualsToken, None, None))
                } else if self.match_char('/') {
                    Ok(self.new_token(TokenType::LessThanSlashToken, None, None))
                } else if self.match_char('<') {
                    if self.match_char('=') {
                        Ok(self.new_token(TokenType::LessThanLessThanEqualsToken, None, None))
                    } else {
                        Ok(self.new_token(TokenType::LessThanLessThanToken, None, None))
                    }
                } else {
                    Ok(self.new_token(TokenType::LessThanToken, None, None))
                }
            }
            '>' => {
                if self.match_char('=') {
                    Ok(self.new_token(TokenType::GreaterThanEqualsToken, None, None))
                } else if self.match_char('>') {
                    if self.match_char('>') {
                        if self.match_char('=') {
                            Ok(self.new_token(
                                TokenType::GreaterThanGreaterThanGreaterThanEqualsToken,
                                None,
                                None,
                            ))
                        } else {
                            Ok(self.new_token(
                                TokenType::GreaterThanGreaterThanGreaterThanToken,
                                None,
                                None,
                            ))
                        }
                    } else if self.match_char('=') {
                        Ok(
                            self.new_token(
                                TokenType::GreaterThanGreaterThanEqualsToken,
                                None,
                                None,
                            ),
                        )
                    } else {
                        Ok(self.new_token(TokenType::GreaterThanGreaterThanToken, None, None))
                    }
                } else {
                    Ok(self.new_token(TokenType::GreaterThanToken, None, None))
                }
            }
            '&' => {
                if self.match_char('=') {
                    Ok(self.new_token(TokenType::AmpersandEqualsToken, None, None))
                } else if self.match_char('&') {
                    if self.match_char('=') {
                        Ok(self.new_token(TokenType::AmpersandAmpersandEqualsToken, None, None))
                    } else {
                        Ok(self.new_token(TokenType::AmpersandAmpersandToken, None, None))
                    }
                } else {
                    Ok(self.new_token(TokenType::AmpersandToken, None, None))
                }
            }
            '|' => {
                if self.match_char('=') {
                    Ok(self.new_token(TokenType::BarEqualsToken, None, None))
                } else if self.match_char('|') {
                    if self.match_char('=') {
                        Ok(self.new_token(TokenType::BarBarEqualsToken, None, None))
                    } else {
                        Ok(self.new_token(TokenType::BarBarToken, None, None))
                    }
                } else {
                    Ok(self.new_token(TokenType::BarToken, None, None))
                }
            }
            '^' => {
                if self.match_char('=') {
                    Ok(self.new_token(TokenType::CaretEqualsToken, None, None))
                } else {
                    Ok(self.new_token(TokenType::CaretToken, None, None))
                }
            }
            '0' => self.numeric_literal_starting_0(),
            '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => Err(LexerError::InvalidNumber {
                line: self.line,
                column: self.start,
            }),
            _ => Err(LexerError::UnexpectedCharacter {
                line: self.line,
                column: self.start,
            }),
        }
    }

    fn numeric_literal_starting_0(&mut self) -> Result<Token, LexerError> {
        let Some(start_char) = self.source.peek() else {
            return Ok(self.new_token(
                TokenType::NumericLiteral,
                Some(StrSpan {
                    start: self.start,
                    end: self.current,
                }),
                Some(StrSpan {
                    start: self.start,
                    end: self.current,
                }),
            ));
        };

        match start_char {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.decimal(),
            'b' | 'B' => self.binary(),
            'o' | 'O' => self.octal(),
            'x' | 'X' => self.hexadecimal(),
            // 'e' | 'E' => {}
            _ => Ok(self.new_token(
                TokenType::NumericLiteral,
                Some(StrSpan {
                    start: self.start,
                    end: self.current,
                }),
                Some(StrSpan {
                    start: self.start,
                    end: self.current,
                }),
            )),
        }
    }

    fn binary(&mut self) -> Result<Token, LexerError> {
        _ = self.advance();

        let Some(next_c) = self.source.peek() else {
            return Err(LexerError::InvalidNumber {
                line: self.line,
                column: self.start,
            });
        };

        match *next_c {
            '0' | '1' => {
                _ = self.advance();

                loop {
                    let Some(next_next_c) = self.source.peek() else {
                        return Ok(self.new_token(
                            TokenType::NumericLiteral,
                            Some(StrSpan {
                                start: self.start,
                                end: self.current + 1,
                            }),
                            Some(StrSpan {
                                start: self.start,
                                end: self.current + 1,
                            }),
                        ));
                    };

                    match next_next_c {
                        ' ' | '\n' | '\r' | '\t' | ';' => break,
                        '0' | '1' => {
                            _ = self.advance();
                        }
                        _ => {
                            self.current += 1;
                            return Err(LexerError::InvalidNumber {
                                line: self.line,
                                column: self.current,
                            });
                        }
                    }
                }

                Ok(self.new_token(
                    TokenType::NumericLiteral,
                    Some(StrSpan {
                        start: self.start,
                        end: self.current + 1,
                    }),
                    Some(StrSpan {
                        start: self.start,
                        end: self.current + 1,
                    }),
                ))
            }
            _ => Err(LexerError::InvalidNumber {
                line: self.line,
                column: self.start,
            }),
        }
    }
    fn octal(&mut self) -> Result<Token, LexerError> {
        _ = self.advance();

        let Some(next_c) = self.source.peek() else {
            return Err(LexerError::InvalidNumber {
                line: self.line,
                column: self.start,
            });
        };

        match *next_c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => {
                _ = self.advance();

                loop {
                    let Some(next_next_c) = self.source.peek() else {
                        return Ok(self.new_token(
                            TokenType::NumericLiteral,
                            Some(StrSpan {
                                start: self.start,
                                end: self.current + 1,
                            }),
                            Some(StrSpan {
                                start: self.start,
                                end: self.current + 1,
                            }),
                        ));
                    };

                    match next_next_c {
                        ' ' | '\n' | '\r' | '\t' | ';' => break,
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' => {
                            _ = self.advance();
                        }
                        _ => {
                            self.current += 1;
                            return Err(LexerError::InvalidNumber {
                                line: self.line,
                                column: self.current,
                            });
                        }
                    }
                }

                Ok(self.new_token(
                    TokenType::NumericLiteral,
                    Some(StrSpan {
                        start: self.start,
                        end: self.current + 1,
                    }),
                    Some(StrSpan {
                        start: self.start,
                        end: self.current + 1,
                    }),
                ))
            }
            _ => Err(LexerError::InvalidNumber {
                line: self.line,
                column: self.start,
            }),
        }
    }
    fn decimal(&mut self) -> Result<Token, LexerError> {
        _ = self.advance();

        loop {
            let Some(next_next_c) = self.source.peek() else {
                return Ok(self.new_token(
                    TokenType::NumericLiteral,
                    Some(StrSpan {
                        start: self.start,
                        end: self.current + 1,
                    }),
                    Some(StrSpan {
                        start: self.start,
                        end: self.current + 1,
                    }),
                ));
            };

            match next_next_c {
                ' ' | '\n' | '\r' | '\t' | ';' => break,
                '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                    _ = self.advance();
                }
                'b' => {
                    _ = self.advance();
                    break;
                }
                _ => {
                    self.current += 1;
                    return Err(LexerError::InvalidNumber {
                        line: self.line,
                        column: self.current,
                    });
                }
            }
        }

        Ok(self.new_token(
            TokenType::NumericLiteral,
            Some(StrSpan {
                start: self.start,
                end: self.current + 1,
            }),
            Some(StrSpan {
                start: self.start,
                end: self.current + 1,
            }),
        ))
    }
    fn hexadecimal(&mut self) -> Result<Token, LexerError> {
        _ = self.advance();

        let Some(next_c) = self.source.peek() else {
            return Err(LexerError::InvalidNumber {
                line: self.line,
                column: self.start,
            });
        };

        match *next_c {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b' | 'c' | 'd'
            | 'e' | 'f' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' => {
                _ = self.advance();

                loop {
                    let Some(next_next_c) = self.source.peek() else {
                        return Ok(self.new_token(
                            TokenType::NumericLiteral,
                            Some(StrSpan {
                                start: self.start,
                                end: self.current + 1,
                            }),
                            Some(StrSpan {
                                start: self.start,
                                end: self.current + 1,
                            }),
                        ));
                    };

                    match next_next_c {
                        ' ' | '\n' | '\r' | '\t' | ';' => break,
                        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | 'a' | 'b'
                        | 'c' | 'd' | 'e' | 'f' | 'A' | 'B' | 'C' | 'D' | 'E' | 'F' => {
                            _ = self.advance();
                        }
                        _ => {
                            self.current += 1;
                            return Err(LexerError::InvalidNumber {
                                line: self.line,
                                column: self.current,
                            });
                        }
                    }
                }

                Ok(self.new_token(
                    TokenType::NumericLiteral,
                    Some(StrSpan {
                        start: self.start,
                        end: self.current + 1,
                    }),
                    Some(StrSpan {
                        start: self.start,
                        end: self.current + 1,
                    }),
                ))
            }
            _ => Err(LexerError::InvalidNumber {
                line: self.line,
                column: self.start,
            }),
        }
    }
    // fn exponent(&mut self) -> Result<Token, LexerError> {}
}

#[cfg(test)]
mod tests {
    use super::*;

    // ==========
    // = Tokens =
    // ==========

    #[test]
    fn test_lexer_token_semicolon() -> Result<(), String> {
        let test_string = ";";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::SemicolonToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_left_paren() -> Result<(), String> {
        let test_string = "(";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::LeftParenToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_right_paren() -> Result<(), String> {
        let test_string = ")";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::RightParenToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_left_brace() -> Result<(), String> {
        let test_string = "{";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::LeftBraceToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_right_brace() -> Result<(), String> {
        let test_string = "}";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::RightBraceToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_left_bracket() -> Result<(), String> {
        let test_string = "[";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::LeftBracketToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_right_bracket() -> Result<(), String> {
        let test_string = "]";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::RightBracketToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_comma() -> Result<(), String> {
        let test_string = ",";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::CommaToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_tilde() -> Result<(), String> {
        let test_string = "~";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::TildeToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_colon() -> Result<(), String> {
        let test_string = ":";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::ColonToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_at() -> Result<(), String> {
        let test_string = "@";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AtToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_dot() -> Result<(), String> {
        let test_string = ".";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::DotToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_dot_dot_dot() -> Result<(), String> {
        let test_string = "...";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::DotDotDotToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_plus() -> Result<(), String> {
        let test_string = "+";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::PlusToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_plus_plus() -> Result<(), String> {
        let test_string = "++";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::PlusPlusToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_plus_equals() -> Result<(), String> {
        let test_string = "+=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::PlusEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_minus() -> Result<(), String> {
        let test_string = "-";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::MinusToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_minus_minus() -> Result<(), String> {
        let test_string = "--";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::MinusMinusToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_minus_equals() -> Result<(), String> {
        let test_string = "-=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::MinusEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_asterisk() -> Result<(), String> {
        let test_string = "*";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AsteriskToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_asterisk_equals() -> Result<(), String> {
        let test_string = "*=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AsteriskEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_asterisk_asterisk() -> Result<(), String> {
        let test_string = "**";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AsteriskAsteriskToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_asterisk_asterisk_equals() -> Result<(), String> {
        let test_string = "**=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AsteriskAsteriskEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_slash() -> Result<(), String> {
        let test_string = "/";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::SlashToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_slash_equals() -> Result<(), String> {
        let test_string = "/=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::SlashEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_percent() -> Result<(), String> {
        let test_string = "%";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::PercentToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_percent_equals() -> Result<(), String> {
        let test_string = "%=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::PercentEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_equals() -> Result<(), String> {
        let test_string = "=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::EqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_equals_equals() -> Result<(), String> {
        let test_string = "==";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::EqualsEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_equals_equals_equals() -> Result<(), String> {
        let test_string = "===";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::EqualsEqualsEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_equals_greater() -> Result<(), String> {
        let test_string = "=>";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::EqualsGreaterThanToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_exclamation() -> Result<(), String> {
        let test_string = "!";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::ExclamationToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_exclamation_equals() -> Result<(), String> {
        let test_string = "!=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::ExclamationEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_exclamation_equals_equals() -> Result<(), String> {
        let test_string = "!==";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::ExclamationEqualsEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_question() -> Result<(), String> {
        let test_string = "?";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::QuestionToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_question_question() -> Result<(), String> {
        let test_string = "??";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::QuestionQuestionToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_question_question_equals() -> Result<(), String> {
        let test_string = "??=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::QuestionQuestionEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_question_dot() -> Result<(), String> {
        let test_string = "?.";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::QuestionDotToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_less() -> Result<(), String> {
        let test_string = "<";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::LessThanToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_less_equals() -> Result<(), String> {
        let test_string = "<=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::LessThanEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_less_slash() -> Result<(), String> {
        let test_string = "</";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::LessThanSlashToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_less_less() -> Result<(), String> {
        let test_string = "<<";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::LessThanLessThanToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_less_less_equals() -> Result<(), String> {
        let test_string = "<<=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::LessThanLessThanEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_greater() -> Result<(), String> {
        let test_string = ">";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::GreaterThanToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_greater_equals() -> Result<(), String> {
        let test_string = ">=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::GreaterThanEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_greater_greater() -> Result<(), String> {
        let test_string = ">>";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::GreaterThanGreaterThanToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_greater_greater_equals() -> Result<(), String> {
        let test_string = ">>=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::GreaterThanGreaterThanEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_greater_greater_greater() -> Result<(), String> {
        let test_string = ">>>";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(
                    t.token_type,
                    TokenType::GreaterThanGreaterThanGreaterThanToken
                );
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_greater_greater_greater_equals() -> Result<(), String> {
        let test_string = ">>>=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(
                    t.token_type,
                    TokenType::GreaterThanGreaterThanGreaterThanEqualsToken
                );
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_ampersand() -> Result<(), String> {
        let test_string = "&";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AmpersandToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_ampersand_ampersand() -> Result<(), String> {
        let test_string = "&&";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AmpersandAmpersandToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_ampersand_ampersand_equals() -> Result<(), String> {
        let test_string = "&&=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AmpersandAmpersandEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_ampersand_equals() -> Result<(), String> {
        let test_string = "&=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::AmpersandEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_bar() -> Result<(), String> {
        let test_string = "|";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::BarToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_bar_bar() -> Result<(), String> {
        let test_string = "||";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::BarBarToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_bar_bar_equals() -> Result<(), String> {
        let test_string = "||=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::BarBarEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_bar_equals() -> Result<(), String> {
        let test_string = "|=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::BarEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_caret() -> Result<(), String> {
        let test_string = "^";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::CaretToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
    #[test]
    fn test_lexer_token_caret_equals() -> Result<(), String> {
        let test_string = "^=";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::CaretEqualsToken);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }

    // ===================
    // = Numeric Literal =
    // ==========
    #[test]
    fn test_lexer_numeric_literal_binary() -> Result<(), String> {
        let test_string = "0b101";
        let mut lexer = Lexer::new(test_string);
        let token = lexer.next_token();

        println!("{:?}", token);

        match token {
            Ok(t) => {
                assert_eq!(t.token_type, TokenType::NumericLiteral);
                Ok(())
            }
            Err(e) => Err(e.to_string()),
        }
    }
}
