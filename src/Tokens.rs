#[derive(Debug)]
pub enum token_type {
    EOF = -1,
    NEWLINE = 0,
    NUMBER = 1,
    IDENT = 2,
    STRING = 3,

    // keywords
    LABEL = 101,
    GOTO = 102,
    PRINT = 103,
    INPUT = 104,
    LET = 105,
    IF = 106,
    THEN = 107,
    ENDIF = 108,
    WHILE = 109,
    REPEAT = 110,
    ENDWHILE = 111,

    //operators
    EQ = 201,
    PLUS = 202,
    MINUS = 203,
    ASTERISK = 204,
    SLASH = 205,
    EQEQ = 206,
    NOTEQ = 207,
    LT = 208,
    LTEQ = 209,
    GT = 210,
    GTEQ = 211,
}
#[derive(Debug)]
pub struct Token {
    //text needs to be a string because of tokens like == and <<
    text: String,
    kind: token_type,
}

impl Token {
    pub fn new(text: String, kind: token_type) -> Token {
        Token { text, kind }
    }

    pub fn get_keyword(text: &str) -> Option<token_type> {
        match text {
            "LABEL" => Some(token_type::LABEL),
            "GOTO" => Some(token_type::GOTO),
            "PRINT" => Some(token_type::PRINT),
            "INPUT" => Some(token_type::INPUT),
            "LET" => Some(token_type::LET),
            "IF" => Some(token_type::IF),
            "THEN" => Some(token_type::THEN),
            "ENDIF" => Some(token_type::ENDIF),
            "WHILE" => Some(token_type::WHILE),
            "REPEAT" => Some(token_type::REPEAT),
            "ENDWHILE" => Some(token_type::ENDWHILE),
            _ => None,
        }
    }
}
