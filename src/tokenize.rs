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
    // stringが終端文字を含まないので、このlenを使って終端を判断する.
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
        if ind >= len - 1 {
            println!("all read done!!!");
            let tok = Token::new_token(TokenKind::EOF, 0, String::from(""));
            tok_vec.push(tok);
            break;
        }
        let char = string.chars().nth(ind).unwrap();

        if char.is_ascii_punctuation() {
            println!("{} is punct!", string.chars().nth(ind).unwrap());
            let tok = Token::new_token(TokenKind::PUNCT, 0, String::from("+"));
            tok_vec.push(tok);
            ind += 1;
            continue;
        }
        if char.is_ascii_digit() {
            let mut cur_num: i32 = char.to_digit(10).unwrap() as i32;
            ind += 1;
            loop {
                // 最後の文字をreadし終わったら
                if ind == len {
                    break;
                }
                let char = string.chars().nth(ind).unwrap();
                println!("ind is {} , char is {}", ind, char);
                // 数値でない or 終端に達したら.
                if !char.is_ascii_digit() {
                    break;
                }
                cur_num = cur_num * 10 + char.to_digit(10).unwrap() as i32;
                ind += 1;
            }
            println!("cur_num is {}", cur_num);
            let tok = Token::new_token(TokenKind::NUM, cur_num, String::from(""));
            tok_vec.push(tok);
            continue;
        }
        panic!("something wrong...")
    }
    return tok_vec;
}
