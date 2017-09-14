#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Ilegal,
    EOF,
    WS,
    Ident(String),
    Asterisk,
    Comma,
    ParLeft,
    ParRight,
    Select,
    From,
    Insert,
    Into,
    Values,
}

fn is_whitespace(ch: char) -> bool {
    ch == ' ' || ch == '\t' || ch == '\n'
}

fn is_letter(ch: char) -> bool {
    (ch >= 'a' && ch <= 'z') || (ch >= 'A' && ch <= 'Z')
}

fn is_digit(ch: char) -> bool {
    ch >= '0' && ch <= '9'
}

#[derive(Debug)]
struct Lexer {
    pos: i32,
    buffer: Vec<char>,
}

impl Lexer {
    pub fn new(s: String) -> Lexer {
        Lexer {
            pos: 0,
            buffer: s.chars().collect(),
        }
    }

    fn read(&mut self) -> Option<char> {
        let pos = self.pos as usize;
        if self.buffer.len() > pos {
            self.pos += 1;
            Some(self.buffer[pos])
        } else {
            None
        }
    }


    fn unread(&mut self) {
        if self.pos > 0 {
            self.pos -= 1;
        }
    }

    pub fn scan_text(&mut self) -> Vec<Token> {
        let mut tokens = vec![];
        loop {
            let token = self.scan();
            tokens.push(token.clone());
            if token == Token::EOF {
                break;
            }
        }
        tokens
    }

    pub fn scan(&mut self) -> Token {
        let opt_ch = self.read();
        match opt_ch {
            Some(ch) => {
                if is_whitespace(ch) {
                    self.unread();
                }
                if is_letter(ch) {
                    self.unread();
                }
                match ch {
                    '*' => Token::Asterisk,
                    ',' => Token::Comma,
                    '(' => Token::ParLeft,
                    ')' => Token::ParRight,
                    _ => Token::Ilegal,
                }
            }
            None => Token::EOF,
        }
    }

    fn scan_whitespace(&mut self) -> Token {
        loop {
            let opt_ch = self.read();
            match opt_ch {
                Some(ch) => {
                    if !is_whitespace(ch) {
                        self.unread();
                        break;
                    }
                }
                None => {
                    break;
                }
            }
        }
        Token::WS
    }

    fn scan_ident(&mut self) -> Token {
        let mut buff = String::new();
        loop {
            let opt_ch = self.read();
            match opt_ch {
                Some(ch) => {
                    if !is_letter(ch) && !is_digit(ch) && ch != '_' {
                        self.unread();
                        break;
                    }
                    buff.push(ch);
                }
                None => break,
            }
        }
        Token::Ident(buff)
    }
}
