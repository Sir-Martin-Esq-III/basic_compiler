mod Lexer;
mod Tokens;

const EOF: char = '\0';

const test: &str = " IF+-123 foo*THEN/ ";
fn main() {
    let mut l = Lexer::Lexer::new(test);

    while l.peek() != EOF {
        let token = l.get_token();
        match token {
            Some(tok) => println!("{:?}", tok),
            None => println!("invalid token"),
        }
    }
}
