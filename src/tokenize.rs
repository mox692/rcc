use crate::errors::{display_around_pos, init_error};

#[derive(Clone)]
pub struct Token {
    pub kind: TokenKind,
    // Only used by NUM_TOKEN, and int other types always 0.
    pub value: i32,
    pub char: String,
    pub next_token: Option<Box<Token>>,

    // Position from the beginning of the input source.
    pub pos: usize,
}
impl Token {
    fn new_token(kind: TokenKind, value: i32, char: String, cur_pos: usize) -> Token {
        let tok: Token = Token {
            kind: kind,
            value: value,
            char: char,
            next_token: None,
            pos: cur_pos,
        };
        return tok;
    }
    // このtokenのinputからのoffset値(token構造体にあるposではない)を返す.
    pub fn input_pos(&self) -> usize {
        return self.pos - self.char.len();
    }
    pub fn len(&self) -> usize {
        if self.kind == TokenKind::NUM {
            return self.value.to_string().len();
        }
        return self.char.len();
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
    fn cur_pos(&self) -> usize {
        return self.pos;
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
    // expect compares the character currently pointed to by Lexer
    // with the string passed as an argument and returns true if they match.
    fn expect(&mut self, str: &str) -> bool {
        for (i, c) in str.chars().enumerate() {
            if self.get_nth_next(i).eq(&c) {
                return false;
            }
        }
        return true;
    }
    // similar to expect, but it advance l.cur as side effect if it return true.
    fn expect_and_read(&mut self, str: &str) -> bool {
        for (i, c) in str.chars().enumerate() {
            if self.get_nth_next(i).ne(&c) {
                return false;
            }
        }
        self.next_nth(str.len());
        return true;
    }
    fn read_to_whitespace(&mut self) -> String {
        let mut cur_str: String = self.cur_char().to_string();
        self.next();
        loop {
            let char = self.cur_char();
            // 文字でない or 終端に達したら.
            if !char.is_alphabetic() {
                break;
            }
            cur_str.push(char);
            self.next();
        }
        return cur_str;
    }
}

#[derive(Clone)]
struct Tokens(Vec<Token>);

#[derive(Debug, Clone, Eq, PartialEq)]
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
    FOR,
    TYPE(Type),
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Type {
    None,
    INT,
}
impl Type {
    pub fn size(&self) -> usize {
        match &self {
            | Type::INT => 8,
            | _ => panic!("unknown size"),
        }
    }
}

// read chars from lexer' current position and if it matches some specific string,
// then return it as Token.
fn read_punct(l: &mut Lexer) -> Token {
    // multi char.
    if l.expect_and_read("==") {
        return Token::new_token(TokenKind::EQ, 0, String::from("=="), l.cur_pos());
    } else if l.expect_and_read("!=") {
        return Token::new_token(TokenKind::NEQ, 0, String::from("!="), l.cur_pos());
    } else if l.expect_and_read("<=") {
        return Token::new_token(TokenKind::LE, 0, String::from("<="), l.cur_pos());
    } else if l.expect_and_read(">=") {
        return Token::new_token(TokenKind::BE, 0, String::from(">="), l.cur_pos());
    }
    // single char.
    if l.expect_and_read("=") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from("="), l.cur_pos());
    } else if l.expect_and_read("<") {
        return Token::new_token(TokenKind::LT, 0, String::from("<"), l.cur_pos());
    } else if l.expect_and_read(">") {
        return Token::new_token(TokenKind::BT, 0, String::from(">"), l.cur_pos());
    } else if l.expect_and_read("+") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from("+"), l.cur_pos());
    } else if l.expect_and_read("-") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from("-"), l.cur_pos());
    } else if l.expect_and_read("*") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from("*"), l.cur_pos());
    } else if l.expect_and_read("/") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from("/"), l.cur_pos());
    } else if l.expect_and_read(";") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from(";"), l.cur_pos());
    } else if l.expect_and_read("(") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from("("), l.cur_pos());
    } else if l.expect_and_read(")") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from(")"), l.cur_pos());
    } else if l.expect_and_read("{") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from("{"), l.cur_pos());
    } else if l.expect_and_read("}") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from("}"), l.cur_pos());
    } else if l.expect_and_read(",") {
        return Token::new_token(TokenKind::PUNCT, 0, String::from(","), l.cur_pos());
    }
    panic!("");
}

pub fn tokenize(string: String) -> Vec<Token> {
    init_error(string.clone());

    let mut l = Lexer::new(string.clone());

    let tok = Token {
        kind: TokenKind::INI,
        value: 0,
        char: String::from(""),
        next_token: None,
        pos: 0,
    };

    l.push_tok(tok);

    loop {
        let char = l.cur_char();
        if char.eq(&'\0') {
            let tok =
                Token::new_token(TokenKind::EOF, 0, String::from("\0"), l.cur_pos());
            l.push_tok(tok);
            break;
        }

        if char.is_ascii_punctuation() {
            let tok = read_punct(&mut l);
            l.push_tok(tok);
            continue;
        }

        if char.is_ascii_digit() {
            let mut cur_num: i32 = char.to_digit(10).unwrap() as i32;
            l.next_char();
            loop {
                // 最後の文字をreadし終わったら
                // TODO: remove len.
                if l.pos == l.len {
                    break;
                }
                let char = l.cur_char();
                // 数値でない or 終端に達したら.
                if !char.is_ascii_digit() {
                    break;
                }
                cur_num = cur_num * 10 + char.to_digit(10).unwrap() as i32;
                l.next_char();
            }
            let tok =
                Token::new_token(TokenKind::NUM, cur_num, String::from(""), l.cur_pos());
            l.push_tok(tok);
            continue;
        }

        // local variable or C specific keyword.
        // ひとまずアルファベットで構成された文字列なら許可する.
        // TODO: local valは2文字目以降は数字·記号も許可する.
        if char.is_ascii_alphabetic() {
            let mut cur_str = l.read_to_whitespace();
            // specify token kind by cur_str.
            // TODO: use hashmap
            let tok_kind: TokenKind;
            match cur_str.as_str() {
                | "for" => tok_kind = TokenKind::FOR,
                | "return" => tok_kind = TokenKind::RETURN,
                | "if" => tok_kind = TokenKind::IF,
                | "int" => tok_kind = TokenKind::TYPE(Type::INT),
                | "else" => {
                    // read whitespace.
                    l.next();
                    if l.expect_and_read("if") {
                        tok_kind = TokenKind::ELIF;
                        cur_str.push_str(" if");
                    } else {
                        tok_kind = TokenKind::ELSE
                    }
                }
                | _ => tok_kind = TokenKind::IDENT,
            }
            let tok = Token::new_token(tok_kind, 0, cur_str, l.cur_pos());
            l.push_tok(tok);
            continue;
        }

        // whitespaceは飛ばす
        if char.is_whitespace() {
            l.next();
            continue;
        }
        panic!("something wrong...")
    }

    return l.token_vec;
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
    pub fn cur_input_pos(&self) -> usize {
        return self.cur_tok().input_pos();
    }
    pub fn cur_tok_len(&self) -> usize {
        return self.cur_tok().len();
    }
    // increment cur, and return its self
    pub fn next_tok(&mut self) -> &mut Self {
        self.next();
        return self;
    }
    pub fn next_nth_tok(&mut self, n: usize) -> &mut Self {
        let mut i = 0;
        loop {
            if i == n {
                break;
            }
            self.next();
            i += 1;
        }
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
    pub fn error(&self, input_pos: usize, message: String, tok_len: usize) -> ! {
        // TODO: refactor
        let str = display_around_pos(input_pos);
        print!("input pos: {}\n", input_pos);
        print!("tok len: {}\n", tok_len);
        print!("Err place:\n");
        print!("{}\n", str);

        if input_pos < 9 {
            for _ in 0..input_pos {
                print!(" ");
            }
        } else {
            for _ in 0..10 {
                print!(" ");
            }
        }
        for _ in 0..tok_len {
            print!("^");
        }
        println!("");
        println!("Err message: {}", message);
        panic!()
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
        | TokenKind::NUM => {
            println!("kind: {:?}, val: {}, pos: {}", tok.kind, tok.value, tok.pos)
        }
        | TokenKind::PUNCT | TokenKind::IDENT => {
            println!("kind: {:?}, char: {}, pos: {}", tok.kind, tok.char, tok.pos)
        }
        | _ => {
            println!("kind: {:?}, pos: {}", tok.kind, tok.pos)
        }
    }
}
