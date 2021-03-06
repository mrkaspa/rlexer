extern crate parser;

use parser::lexer::{Lexer, Token};
use parser::parser::{Parser, Statement};

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
            Token::Ident(String::from("select")),
            Token::WS,
            Token::Asterisk,
            Token::WS,
            Token::Ident(String::from("from")),
            Token::WS,
            Token::Ident(String::from("user")),
            Token::EOF,
        ]
    );
}

#[test]
fn it_parses_insert() {
    let mut p = Parser::new(String::from(
        "INSERT INTO tbl (name, email) VALUES (demo, demo)",
    ));
    let res = p.parse().expect("Error parsing");
    assert_eq!(
        res,
        Statement::InsertStatement {
            table: String::from("tbl"),
            cols: vec![String::from("name"), String::from("email")],
            values: vec![String::from("demo"), String::from("demo")],
        }
    );
}

#[test]
fn it_parses_no_columns() {
    let mut p = Parser::new(String::from("INSERT INTO tbl VALUES (demo, demo)"));
    let res = p.parse().expect("Error parsing");
    assert_eq!(
        res,
        Statement::InsertStatement {
            table: String::from("tbl"),
            cols: vec![],
            values: vec![String::from("demo"), String::from("demo")],
        }
    );
}
