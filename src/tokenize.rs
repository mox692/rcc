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

struct Lexer {
    // input string.
    pub input: String,
    // current position.
    pub pos: usize,
    // current watching charactor.
    pub char: char,
    // input string size.
    pub len: usize,
    // vec of generated tokens.
    pub token_vec: Vec<Token>,
}
impl Lexer {
    fn new(input: String) -> Self {
        return Lexer {
            input: input.clone(),
            pos: 0,
            char: input.chars().nth(0).unwrap(),
            len: input.chars().count(),
            token_vec: Vec::<Token>::new(),
        };
    }
    fn cur_char(&self) -> char {
        return self.char;
    }
    fn next(&mut self) -> &mut Self {
        self.pos += 1;
        self.char = self.input.chars().nth(self.pos).unwrap();
        return self;
    }
    fn next_nth(&mut self, n: usize) -> &mut Self {
        self.pos += n;
        self.char = self.input.chars().nth(self.pos).unwrap();
        return self;
    }
    fn next_char(&mut self) -> char {
        self.pos += 1;
        self.char = self.input.chars().nth(self.pos).unwrap();
        return self.char;
    }
    fn get_nth_next(&self, n: usize) -> char {
        return self.input.chars().nth(self.pos + n).unwrap();
    }
    fn push_tok(&mut self, tok: Token) {
        &self.token_vec.push(tok);
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
    RETURN, // return
    EQ,     // ==
    NEQ,    // !=
    // 命名は、「(右辺と比較した際に)左辺は」
    LE, // <=
    LT, // <
    BE, // >=
    BT, // >
    IF,
    ELIF,
    ELSE,
}

impl TokenKind {
    fn to_string(&self) -> &str {
        match self {
            TokenKind::EOF => "EOF",
            TokenKind::INI => "INI",
            TokenKind::NUM => "NUM",
            TokenKind::PUNCT => "PUNCT",
            TokenKind::IDENT => "IDENT",
            TokenKind::RETURN => "RETURN",
            TokenKind::EQ => "EQ",
            TokenKind::NEQ => "NEQ",
            TokenKind::LT => "LT",
            TokenKind::LE => "LE",
            TokenKind::BT => "BT",
            TokenKind::BE => "BE",
            TokenKind::IF => "IF",
            TokenKind::ELIF => "ELIF",
            TokenKind::ELSE => "ELSE",
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
            TokenKind::RETURN => write!(f, "RETURN"),
            TokenKind::EQ => write!(f, "EQ"),
            TokenKind::NEQ => write!(f, "NEQ"),
            TokenKind::LT => write!(f, "LT"),
            TokenKind::LE => write!(f, "LE"),
            TokenKind::BT => write!(f, "BT"),
            TokenKind::BE => write!(f, "BE"),
            TokenKind::IF => write!(f, "IF"),
            TokenKind::ELIF => write!(f, "ELIF"),
            TokenKind::ELSE => write!(f, "ELSE"),
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

fn call_eq(string: &String, ind: &mut usize) -> Token {
    let next_char = string.chars().nth(*ind + 1).unwrap();
    let tok: Token;
    if next_char.eq(&'=') {
        tok = Token::new_token(TokenKind::EQ, 0, String::from("=="));
        // 2つ目の=を指すようになる.
        *ind += 1;
    } else {
        tok = Token::new_token(TokenKind::PUNCT, 0, String::from("="));
    }
    return tok;
}

fn call_neq(string: &String, ind: &mut usize) -> Token {
    let next_char = string.chars().nth(*ind + 1).unwrap();
    if next_char.eq(&'=') {
        *ind += 1;
        return Token::new_token(TokenKind::NEQ, 0, String::from("!="));
    }
    println!("expect '=', but got {}", next_char);
    panic!()
}

// >
fn call_big(string: &String, ind: &mut usize) -> Token {
    let next_char = string.chars().nth(*ind + 1).unwrap();
    if next_char.eq(&'=') {
        *ind += 1;
        return Token::new_token(TokenKind::BE, 0, String::from(">="));
    }
    return Token::new_token(TokenKind::BT, 0, String::from(">"));
}

fn call_less(string: &String, ind: &mut usize) -> Token {
    let next_char = string.chars().nth(*ind + 1).unwrap();
    if next_char.eq(&'=') {
        *ind += 1;
        return Token::new_token(TokenKind::LE, 0, String::from("<="));
    }
    return Token::new_token(TokenKind::LT, 0, String::from("<"));
}

pub fn tokenize(string: &String) -> Vec<Token> {
    let mut _l = Lexer::new(string.clone());
    let _tok = Token {
        kind: TokenKind::INI,
        value: 0,
        char: String::from(""),
        next_token: None,
    };

    _l.push_tok(_tok);

    loop {
        let _char = _l.cur_char();
        if _char.eq(&'\0') {
            let tok = Token::new_token(TokenKind::EOF, 0, String::from("\0"));
            _l.push_tok(tok);
            break;
        }

        if _char.is_ascii_punctuation() {
            let tok = match _char {
                '+' => Token::new_token(TokenKind::PUNCT, 0, String::from("+")),
                '-' => Token::new_token(TokenKind::PUNCT, 0, String::from("-")),
                '*' => Token::new_token(TokenKind::PUNCT, 0, String::from("*")),
                '/' => Token::new_token(TokenKind::PUNCT, 0, String::from("/")),
                ';' => Token::new_token(TokenKind::PUNCT, 0, String::from(";")),
                '(' => Token::new_token(TokenKind::PUNCT, 0, String::from("(")),
                ')' => Token::new_token(TokenKind::PUNCT, 0, String::from(")")),
                // TODO: もう少しきれいに.
                '=' => call_eq(&_l.input, &mut _l.pos),
                '!' => call_neq(&_l.input, &mut _l.pos),
                '>' => call_big(&_l.input, &mut _l.pos),
                '<' => call_less(&_l.input, &mut _l.pos),
                _ => {
                    panic!("Unknown token.");
                }
            };
            _l.push_tok(tok);
            _l.next_char();
            continue;
        }

        if _char.is_ascii_digit() {
            let mut cur_num: i32 = _char.to_digit(10).unwrap() as i32;
            _l.next_char();
            loop {
                // 最後の文字をreadし終わったら
                // TODO: remove len.
                if _l.pos == _l.len {
                    break;
                }
                let char = _l.cur_char();
                // 数値でない or 終端に達したら.
                if !char.is_ascii_digit() {
                    break;
                }
                cur_num = cur_num * 10 + char.to_digit(10).unwrap() as i32;
                _l.next_char();
            }
            let tok = Token::new_token(TokenKind::NUM, cur_num, String::from(""));
            _l.push_tok(tok);
            continue;
        }

        // local variable or C specific keyword.
        // ひとまずアルファベットで構成された文字列なら許可する.
        // TODO: local valは2文字目以降は数字·記号も許可する.
        if _char.is_ascii_alphabetic() {
            let mut cur_str = read_to_whitespace(&mut _l);
            // specify token kind by cur_str.
            // TODO: use hashmap
            let tok_kind: TokenKind;
            match cur_str.as_str() {
                "return" => tok_kind = TokenKind::RETURN,
                "if" => tok_kind = TokenKind::IF,
                "else" => {
                    // read whitespace.
                    _l.next();
                    if expect_and_read(&mut _l, "if") {
                        tok_kind = TokenKind::ELIF;
                        cur_str.push_str(" if");
                    } else {
                        tok_kind = TokenKind::ELSE
                    }
                }
                _ => tok_kind = TokenKind::IDENT,
            }
            let tok = Token::new_token(tok_kind, 0, cur_str);
            _l.push_tok(tok);
            continue;
        }

        // whitespaceは飛ばす
        if _char.is_whitespace() {
            _l.next();
            continue;
        }
        panic!("something wrong...")
    }
    // return tok_vec;
    return _l.token_vec;
}
fn read_to_whitespace(_l: &mut Lexer) -> String {
    let mut cur_str: String = _l.cur_char().to_string();
    _l.next();
    loop {
        let char = _l.cur_char();
        // 文字でない or 終端に達したら.
        if !char.is_alphabetic() {
            break;
        }
        cur_str.push(char);
        _l.next();
    }
    return cur_str;
}

// expect compares the character currently pointed to by Lexer
// with the string passed as an argument and returns true if they match.
fn expect(_l: &mut Lexer, str: &str) -> bool {
    for (i, c) in str.chars().enumerate() {
        if _l.get_nth_next(i).eq(&c) {
            return false;
        }
    }
    return true;
}

// similar to expect, but it advance l.cur as side effect if it return true.
fn expect_and_read(_l: &mut Lexer, str: &str) -> bool {
    for (i, c) in str.chars().enumerate() {
        if _l.get_nth_next(i).ne(&c) {
            return false;
        }
    }
    _l.next_nth(str.len());
    return true;
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
    // TODO: 上の関数と合わせて、見直す.
    pub fn get_next_nth_tok(&self, offset: usize) -> Token {
        return self.tokens[self.cur + offset as usize].clone();
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
