use lexer::{Lexer, Token};

#[derive(Debug, Clone)]
pub enum Statement {
    SelectStatement { fields: Vec<String>, table: String },
    InsertStatement {
        cols: Vec<String>,
        values: Vec<String>,
        table: String,
    },
}

struct NextFn {
    call: Box<FnOnce(&mut Parser) -> Result<Option<NextFn>, String>>,
}

type NextFnCall = Result<Option<NextFn>, String>;

pub struct Parser<'a> {
    lexer: &'a mut Lexer,
    stmt: Option<Statement>,
    buf: Option<Token>,
}

impl<'a> Parser<'a> {
    fn new(l: &'a mut Lexer) -> Self {
        Parser {
            lexer: l,
            stmt: None,
            buf: None,
        }
    }

    pub fn parse(&mut self) -> Result<Statement, String> {
        let mut fun = NextFn { call: Box::new(Self::init) };
        loop {
            let next_res = (fun.call)(self);
            match next_res {
                Ok(next_fun_opt) => {
                    match next_fun_opt {
                        Some(next_fun) => fun = next_fun,
                        None => break,
                    }
                }
                Err(error) => return Err(error),
            };
        }
        match self.stmt {
            Some(ref stmt) => Ok(stmt.clone()),
            None => Err(String::from("Could not parse the string")),
        }
    }

    fn scan(&mut self) -> Token {
        match self.buf {
            Some(ref res) => res.clone(),
            None => {
                let token = self.lexer.scan();
                self.buf = Some(token.clone());
                token
            }
        }
    }

    fn scan_ignore_whitespace(&mut self) -> Token {
        let mut token = self.lexer.scan();
        if token == Token::WS {
            token = self.lexer.scan();
        }
        token
    }

    fn unscan(&mut self) {
        self.buf = None;
    }

    fn init(p: &mut Parser) -> NextFnCall {
        let token = p.scan_ignore_whitespace();
        match token {
            Token::Select => Ok(Some(NextFn { call: Box::new(Self::select_sentence) })),
            Token::Insert => Ok(Some(NextFn { call: Box::new(Self::insert_sentence) })),
            _ => Err(String::from("Bad statement begining")),
        }
    }

    fn select_sentence(p: &mut Parser) -> NextFnCall {
        p.stmt = Some(Statement::SelectStatement {
                          fields: vec![],
                          table: String::new(),
                      });
        Ok(Some(NextFn { call: Box::new(Self::into_keyword) }))
    }

    fn insert_sentence(p: &mut Parser) -> NextFnCall {
        p.stmt = Some(Statement::InsertStatement {
                          cols: vec![],
                          values: vec![],
                          table: String::new(),
                      });
        Ok(Some(NextFn { call: Box::new(Self::into_keyword) }))
    }

    fn into_keyword(p: &mut Parser) -> NextFnCall {
        let token = p.scan_ignore_whitespace();
        if token != Token::Into {
            Err(String::from("Into keyword expected"))
        } else {
            Ok(Some(NextFn { call: Box::new(Self::get_table_name) }))
        }
    }

    fn get_table_name(p: &mut Parser) -> NextFnCall {
        let token = p.scan_ignore_whitespace();
        match token {
            Token::Ident(table_name) => {
                match p.stmt {
                    Some(ref mut stmt) => {
                        match stmt {
                            &mut Statement::SelectStatement { ref mut table, .. } => {
                                *table = table_name.clone();
                                Ok(Some(NextFn { call: Box::new(Self::end) }))
                            }
                            _ => Err(String::from("Wrong statement type")),
                        }
                    }
                    None => Err(String::from("Statement not created")),
                }
            }
            _ => Err(String::from("Into keyword expected")),
        }
    }

    fn extract_values(p: &mut Parser)
                          -> Box<FnOnce(&mut Box<Parser>) -> NextFnCall + > {
        Self::extract_into_parentheses(NextFn { call: Box::new(Self::end) },
                                       Box::new(|ref mut p, values| {}))
    }

    fn extract_into_parentheses(next: NextFn,
                                    mut do_func: Box<FnMut(&mut Box<Parser>, Vec<String>) + >)
                                    -> Box<FnOnce(&mut Box<Parser>) -> NextFnCall + > {
        Box::new(move |p: &mut Box<Parser>| -> NextFnCall {
            let mut token = p.scan_ignore_whitespace();
            if token != Token::ParLeft {
                return Err(String::from("( expected"));
            }
            let mut values = vec![];
            loop {
                token = p.scan_ignore_whitespace();
                match token {
                    Token::Ident(value) => {
                        values.push(value);
                        token = p.scan_ignore_whitespace();
                        if token != Token::Comma {
                            break;
                        }
                    }
                    _ => return Err(String::from("Ident token expected")),
                }
            }
            token = p.scan_ignore_whitespace();
            if token != Token::ParRight {
                return Err(String::from(") expected"));
            }
            do_func(p, values);
            Ok(Some(next))
        })
    }

    fn end(p: &mut Parser) -> NextFnCall {
        unimplemented!()
    }
}
