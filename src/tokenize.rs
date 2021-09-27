#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    // Only used by NUM_TOKEN, and int other types always 0.
    pub value: i32,
    pub char: String,
    pub next_token: Option<Box<Token>>,
}
impl Token {
    fn new_token(kind: TokenKind, value: i32, char: String) -> Token {
        let tok: Token = Token {
            kind: kind,
            value: value,
            char: char,
            next_token: None,
        };
        return tok;
    }
}

#[derive(Clone)]
struct Tokens(Vec<Token>);

#[derive(Copy, Clone)]
pub enum TokenKind {
    INI,
    NUM,
    PUNCT,
    EOF,
    IDENT,
}
impl TokenKind {
    fn to_string(&self) -> &str {
        match self {
            TokenKind::EOF => "EOF",
            TokenKind::INI => "INI",
            TokenKind::NUM => "NUM",
            TokenKind::PUNCT => "PUNCT",
            TokenKind::IDENT => "IDENT",
        }
    }
}
impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            TokenKind::EOF => write!(f, "EOF"),
            TokenKind::INI => write!(f, "INI"),
            TokenKind::NUM => write!(f, "NUM"),
            TokenKind::PUNCT => write!(f, "PUNCT"),
            TokenKind::IDENT => write!(f, "IDENT"),
        }
    }
}
impl PartialEq for TokenKind {
    // もっといい実装があるかも.
    fn eq(&self, other: &Self) -> bool {
        self.to_string().eq(other.to_string())
    }
}
impl Eq for TokenKind {}

pub fn tokenize(string: &String) -> Vec<Token> {
    let mut ind = 0;
    let len = string.len();
    // tok vec.
    let mut tok_vec = Vec::<Token>::new();
    // first tok.
    let tok = Token {
        kind: TokenKind::INI,
        value: 0,
        char: String::from(""),
        next_token: None,
    };

    tok_vec.push(tok);

    loop {
        let char = string.chars().nth(ind).unwrap();

        // terminated character.
        if char.eq(&'\0') {
            let tok = Token::new_token(TokenKind::EOF, 0, String::from("\0"));
            tok_vec.push(tok);
            break;
        }

        // tokenize punct.
        if char.is_ascii_punctuation() {
            let tok = match char {
                '+' => Token::new_token(TokenKind::PUNCT, 0, String::from("+")),
                '-' => Token::new_token(TokenKind::PUNCT, 0, String::from("-")),
                '*' => Token::new_token(TokenKind::PUNCT, 0, String::from("*")),
                '/' => Token::new_token(TokenKind::PUNCT, 0, String::from("/")),
                ';' => Token::new_token(TokenKind::PUNCT, 0, String::from(";")),
                '=' => Token::new_token(TokenKind::PUNCT, 0, String::from("=")),
                _ => {
                    panic!("Unknown token.");
                }
            };
            tok_vec.push(tok);
            ind += 1;
            continue;
        }

        // tokenize num.
        if char.is_ascii_digit() {
            let mut cur_num: i32 = char.to_digit(10).unwrap() as i32;
            ind += 1;
            loop {
                // 最後の文字をreadし終わったら
                // TODO: remove len.
                if ind == len {
                    break;
                }
                let char = string.chars().nth(ind).unwrap();
                // 数値でない or 終端に達したら.
                if !char.is_ascii_digit() {
                    break;
                }
                cur_num = cur_num * 10 + char.to_digit(10).unwrap() as i32;
                ind += 1;
            }
            let tok = Token::new_token(TokenKind::NUM, cur_num, String::from(""));
            tok_vec.push(tok);
            continue;
        }

        // local variable.
        // ひとまずアルファベットで構成された文字列なら許可する.
        if char.is_ascii_alphabetic() {
            let mut cur_str: String = char.to_string();
            ind += 1;
            loop {
                let char = string.chars().nth(ind).unwrap();
                // 数値でない or 終端に達したら.
                if !char.is_alphabetic() {
                    break;
                }
                cur_str.push(char);
                ind += 1;
            }
            let tok = Token::new_token(TokenKind::IDENT, 0, cur_str);
            tok_vec.push(tok);
            continue;
        }

        // whitespaceは飛ばす
        if char.is_whitespace() {
            ind += 1;
            continue;
        }
        panic!("something wrong...")
    }
    return tok_vec;
}

#[derive(Clone)]
pub struct TokenReader {
    pub tokens: Vec<Token>,
    pub cur: usize,
}
impl TokenReader {
    // return cur's index Token.
    pub fn cur_tok(&self) -> Token {
        return self.tokens[self.cur as usize].clone();
    }
    // increment cur, and return its self
    pub fn next_tok(&mut self) -> &mut Self {
        self.next();
        return self;
    }
    pub fn get_next_tok(&self) -> Token {
        return self.tokens[self.cur + 1 as usize].clone();
    }
    // next counts up current position.
    pub fn next(&mut self) {
        self.cur += 1;
    }
    // expect cur token.
    pub fn expect(&self, s: &str) -> bool {
        if self.tokens[self.cur].char == s {
            true
        } else {
            false
        }
    }
    // report unexpected result.
    pub fn error(&self, message: String) {
        print!(":::::::::ERROR:::::::::\nmessage: {}\n", message);
        print_token_info(&self.cur_tok());
        print!(":::::::::::::::::::::::\n");
        panic!("");
    }
}

pub fn NewTokenReader(token: Vec<Token>) -> TokenReader {
    return TokenReader {
        tokens: token,
        cur: 0,
    };
}

// Debug tokens which tokenizer generate.
pub fn debug_tokens(flag: bool, tokens: &Vec<Token>) {
    if !flag {
        return;
    }
    println!("////////TOKEN DEBUG START////////");
    for tok in tokens.iter() {
        print_token_info(tok);
    }
    println!("////////TOKEN DEBUG END////////");
}

fn print_token_info(tok: &Token) {
    match tok.kind {
        TokenKind::NUM => {
            println!("kind: {}, val: {}", tok.kind, tok.value,)
        }
        TokenKind::PUNCT | TokenKind::IDENT => {
            println!("kind: {}, char: {}", tok.kind, tok.char,)
        }
        _ => {
            println!("kind: {}", tok.kind,)
        }
    }
}
