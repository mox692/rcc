pub struct Token {
    pub kind: TokenKind,
    pub value: Option<i32>,
    pub char: Option<String>,
    pub next_token: Option<Box<Token>>,
}
impl Token {
    fn new_token(kind: TokenKind, value: Option<i32>, char: Option<String>) -> Token {
        let tok: Token = Token {
            kind: kind,
            value: None,
            char: None,
            next_token: None,
        };
        return tok;
    }
}

pub enum TokenKind {
    INI,
    NUM,
    PUNCT,
    EOF,
}
impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::INI => write!(f, "INI"),
            TokenKind::NUM => write!(f, "NUM"),
            TokenKind::PUNCT => write!(f, "PUNCT"),
        }
    }
}

pub fn tokenize(string: &String) -> Vec<Token> {
    let mut ind = 0;
    let len = string.len();
    // tok vec.
    let mut tok_vec = Vec::<Token>::new();
    // first tok.
    let tok = Token {
        kind: TokenKind::INI,
        value: None,
        char: None,
        next_token: None,
    };

    tok_vec.push(tok);

    loop {
        if ind == len {
            println!("all read done!!!");
            let tok = Token::new_token(TokenKind::EOF, None, None);
            tok_vec.push(tok);
            break;
        }
        let char = string.chars().nth(ind).unwrap();

        if char.is_ascii_punctuation() {
            println!("{} is punct!", string.chars().nth(ind).unwrap());
            let tok = Token::new_token(TokenKind::PUNCT, None, Some("+".to_string()));
            tok_vec.push(tok);
            ind += 1;
            continue;
        }
        if char.is_ascii_digit() {
            println!("{} is digit!", string.chars().nth(ind).unwrap());
            let tok = Token::new_token(TokenKind::PUNCT, None, None);
            tok_vec.push(tok);
            ind += 1;
            continue;
        }
        panic!("something wrong...")
    }
    return tok_vec;
}
