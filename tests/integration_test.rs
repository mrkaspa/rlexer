extern crate parser;

use parser::lexer::{Lexer, Token};
use Token::*;

#[test]
fn it_scans() {
    let mut l = Lexer::new(String::from("*"));
    assert_eq!(l.scan(), Token::Asterisk);
}

#[test]
fn it_scans_text() {
    let mut l = Lexer::new(String::from("select * from user"));
    assert_eq!(
        l.scan_text(),
        vec![
            Ident(String::from("select")),
            WS,
            Asterisk,
            WS,
            Ident(String::from("from")),
            WS,
            Ident(String::from("user")),
            EOF,
        ]
    );
}
