use std::mem;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

// ANCHOR: token
#[derive(Debug, PartialEq)]
pub enum Token {
    // keywords
    Integer(i64),
    Name(String),
    String(String),
    // end
    Eos,
}
// ANCHOR_END: token

#[derive(Debug)]
// ANCHOR: lex
pub struct Lex {
    input: File,
    ahead: Token,
}
// ANCHOR_END: lex

impl Lex {
    pub fn new(input: File) -> Self {
        Lex {
            input,
            ahead: Token::Eos,
        }
    }

// ANCHOR: peek_next
pub fn next(&mut self) -> Token {
        if self.ahead == Token::Eos {
            self.do_next()
        } else {
            mem::replace(&mut self.ahead, Token::Eos)
        }
    }
/* go back to the last token, useless now
pub fn peek(&mut self) -> &Token {
        if self.ahead == Token::Eos {
            self.ahead = self.do_next();
        }
        &self.ahead
    }
*/ 
fn do_next(&mut self) -> Token {
    let ch = self.read_char();
    match ch {
        
        '\n' | '\r' | '\t' | ' ' => self.do_next(),

       
        '"' => match self.read_char() {
            '0'..='9' => {
                
                self.putback_char();
                self.read_number(false)
            },
            '-' => match self.read_char(){
                '0'..='9' => {
                
                    self.putback_char();
                    self.read_number(true)
                },
                _ => panic!("Expected a digit after \" in this fake LOGO language"),
            }
            'a'..='z' | 'A'..='Z' => {
                self.putback_char();
                self.read_string(ch)
            }, // 'a'..='z' | 'A'..='Z' | '_
            _ => panic!("Expected a digit after \" in this fake LOGO language"),
        },

        
        'A'..='Z' | 'a'..='z' | '_' => self.read_name(ch),
        '\0' => Token::Eos,
        '/' => match self.read_char() {
            '/' => {
                self.skip_comment();
                self.do_next()
            },
            _ => {
                self.putback_char();
                panic!("Invalid char {ch}");
            },
        },
        _ => panic!("Invalid char {ch}"),
    }
}

    #[allow(clippy::unused_io_amount)]
    fn read_char(&mut self) -> char {
        let mut buf: [u8; 1] = [0];
        self.input.read(&mut buf).unwrap();
        buf[0] as char
    }
    fn putback_char(&mut self) {
        self.input.seek(SeekFrom::Current(-1)).unwrap();
    }

    
    fn read_number(&mut self, is_negative: bool) -> Token {
        let mut n: i64 = 0;
        loop {
            let ch = self.read_char();
            match ch {
                '0'..='9' => {
                    let digit = ch.to_digit(10).unwrap() as i64;
                    n = n * 10 + digit;
                },
                _ => {
                    self.putback_char();
                    break;
                },
            }
        }
        if is_negative {
            n = -n;
        }
        Token::Integer(n)
    }

    fn read_name(&mut self, first: char) -> Token {
        let mut s = first.to_string();

        loop {
            let ch = self.read_char();
            if ch.is_alphanumeric() || ch == '_' {
                s.push(ch);
            } else {
                self.putback_char();
                break;
            }
        }

        match &s as &str { // TODO optimize by hash
            _          => Token::Name(s),
        }
    }

    fn read_string(&mut self, quote: char) -> Token {
        let mut s = String::new();
        loop {
            match self.read_char() {
                '\n' | '\0' => panic!("unfinished string"),
                ch if ch == ' ' => break,
                ch => s.push(ch),
            }
        }
        Token::String(s)
    }

    // '--' has been read
    fn skip_comment(&mut self) {
        while let ch = self.read_char() {
            if ch == '\n' || ch == '\r' {
                break;
            }
        }
    }
}