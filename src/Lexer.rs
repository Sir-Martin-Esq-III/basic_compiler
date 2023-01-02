use crate::Tokens::{token_type, Token};

const special_chars: [char; 5] = ['\r', '\t', '\n', '\\', '%'];

pub struct Lexer<'a> {
    pub source: &'a str,
    pub cur_char: char,
    pub cur_pos: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer<'a> {
        Lexer {
            source: input,
            cur_char: input.chars().next().unwrap(),
            cur_pos: usize::MIN,
        }
    }

    pub fn next_char(&mut self) {
        self.cur_pos += 1;

        if self.cur_pos >= self.source.len() {
            self.cur_char = '\0';
        } else {
            self.cur_char = self.source.chars().nth(self.cur_pos).unwrap();
        }
    }

    pub fn peek(&self) -> char {
        if self.cur_pos + 1 >= self.source.len() {
            return '\0';
        }
        return self.source.chars().nth(self.cur_pos + 1).unwrap();
    }

    pub fn abort(&self, message: &str) {
        panic!("LEXER ERROR: {:?}", message);
    }

    pub fn skip_whitespace(&mut self) {
        while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
            self.next_char();
        }
    }

    pub fn skip_comment(&mut self) {
        if self.cur_char == '/' && self.peek() == '/' {
            while self.cur_char != '\n' {
                self.next_char();
            }
        }
    }

    pub fn get_token(&mut self) -> Option<Token> {
        let token: Option<Token>;
        self.skip_whitespace();
        self.skip_comment();
        match self.cur_char {
            '+' => {
                token = Some(Token::new(
                    format!("{:?}", &self.cur_char),
                    token_type::PLUS,
                ))
            }
            '-' => {
                token = Some(Token::new(
                    format!("{:?}", &self.cur_char),
                    token_type::MINUS,
                ))
            }
            '*' => {
                token = Some(Token::new(
                    format!("{:?}", &self.cur_char),
                    token_type::ASTERISK,
                ))
            }
            '/' => {
                token = Some(Token::new(
                    format!("{:?}", &self.cur_char),
                    token_type::SLASH,
                ))
            }
            '\n' => {
                token = Some(Token::new(
                    format!("{:?}", &self.cur_char),
                    token_type::NEWLINE,
                ))
            }
            '\0' => token = Some(Token::new(format!("{:?}", &self.cur_char), token_type::EOF)),
            '=' => {
                if self.peek() == '=' {
                    token = Some(Token::new(
                        format!("{:?}{:?}", self.cur_char.to_string(), "="),
                        token_type::EQEQ,
                    ));
                    self.next_char();
                } else {
                    token = Some(Token::new(format!("{:?}", &self.cur_char), token_type::EQ))
                }
            }
            '>' => {
                if self.peek() == '=' {
                    token = Some(Token::new(
                        format!("{:?}{:?}", self.cur_char.to_string(), "="),
                        token_type::GTEQ,
                    ));
                    self.next_char();
                } else {
                    token = Some(Token::new(format!("{:?}", &self.cur_char), token_type::GT))
                }
            }
            '<' => {
                if self.peek() == '=' {
                    token = Some(Token::new(
                        format!("{:?}{:?}", self.cur_char.to_string(), "="),
                        token_type::LTEQ,
                    ));
                    self.next_char();
                } else {
                    token = Some(Token::new(format!("{:?}", &self.cur_char), token_type::LT))
                }
            }
            '!' => {
                if self.peek() == '=' {
                    token = Some(Token::new(
                        format!("{:?}{:?}", self.cur_char.to_string(), "="),
                        token_type::NOTEQ,
                    ));
                    self.next_char();
                } else {
                    //current !var is not supported
                    token = None
                }
            }
            '\"' => {
                self.next_char();
                let start = self.cur_pos;
                while self.cur_char != '\"' {
                    if special_chars.contains(&self.cur_char) {
                        self.abort("Found special characters in string");
                    }
                    self.next_char();
                }
                let rng = start..self.cur_pos;

                token = Some(Token::new(
                    format!("{:?}", &self.source[rng]),
                    token_type::STRING,
                ))
            }
            _ => {
                if self.cur_char.is_ascii_digit() {
                    let start = self.cur_pos;

                    while self.peek().is_ascii_digit() {
                        self.next_char();
                    }
                    if self.peek() == '.' {
                        self.next_char();

                        if !self.peek().is_ascii_digit() {
                            self.abort("No digit found after decimal point")
                        }

                        while self.peek().is_ascii_digit() {
                            self.next_char();
                        }
                    }
                    let rng = start..self.cur_pos + 1;

                    token = Some(Token::new(
                        format!("{:?}", &self.source[rng]),
                        token_type::NUMBER,
                    ))
                } else if self.cur_char.is_alphabetic() {
                    let start = self.cur_pos;

                    while self.peek().is_alphanumeric() {
                        self.next_char();
                    }
                    let rng = start..self.cur_pos + 1;
                    let token_slice = &self.source[rng];
                    let token_kind = Token::get_keyword(&token_slice.to_uppercase());
                    match token_kind {
                        None => {
                            token =
                                Some(Token::new(format!("{:?}", token_slice), token_type::IDENT))
                        }
                        Some(tok) => token = Some(Token::new(format!("{:?}", token_slice), tok)),
                    }
                } else {
                    token = None;
                }
            }
        };
        self.next_char();
        token
    }
}
