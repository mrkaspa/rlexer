use lexer::{Lexer, Token};

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    SelectStatement {
        fields: Vec<String>,
        table: String,
    },
    InsertStatement {
        cols: Vec<String>,
        values: Vec<String>,
        table: String,
    },
}

struct NextFn {
    call: fn(&mut Parser) -> Result<Option<NextFn>, String>,
}

type NextFnCall = Result<Option<NextFn>, String>;

#[derive(Clone)]
pub struct Parser {
    lexer: Lexer,
    stmt: Option<Statement>,
    buf: Option<Token>,
}

impl Parser {
    pub fn new(query: String) -> Self {
        let l = Lexer::new(query);
        Parser {
            lexer: l,
            stmt: None,
            buf: None,
        }
    }

    pub fn parse(&mut self) -> Result<Statement, String> {
        let mut fun = NextFn { call: Self::init };
        loop {
            let next_fun_opt = (fun.call)(self)?;
            match next_fun_opt {
                Some(next_fun) => fun = next_fun,
                None => break,
            }
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
        self.lexer.unread();
    }

    fn init(p: &mut Parser) -> NextFnCall {
        let token = p.scan_ignore_whitespace();
        match token {
            Token::Select => Ok(Some(NextFn {
                call: Self::select_sentence,
            })),
            Token::Insert => Ok(Some(NextFn {
                call: Self::insert_sentence,
            })),
            _ => Err(String::from("Bad statement begining")),
        }
    }

    fn select_sentence(p: &mut Parser) -> NextFnCall {
        p.stmt = Some(Statement::SelectStatement {
            fields: vec![],
            table: String::new(),
        });
        Ok(Some(NextFn { call: Self::end }))
    }

    fn insert_sentence(p: &mut Parser) -> NextFnCall {
        p.stmt = Some(Statement::InsertStatement {
            cols: vec![],
            values: vec![],
            table: String::new(),
        });
        Ok(Some(NextFn {
            call: Self::into_keyword,
        }))
    }

    fn into_keyword(p: &mut Parser) -> NextFnCall {
        let token = p.scan_ignore_whitespace();
        if token != Token::Into {
            Err(String::from("Into keyword expected"))
        } else {
            Ok(Some(NextFn {
                call: Self::get_table_name,
            }))
        }
    }

    fn get_table_name(p: &mut Parser) -> NextFnCall {
        let token = p.scan_ignore_whitespace();
        match token {
            Token::Ident(table_name) => match p.stmt {
                Some(ref mut stmt) => match stmt {
                    &mut Statement::InsertStatement { ref mut table, .. } => {
                        *table = table_name.clone();
                        Ok(Some(NextFn {
                            call: Self::extract_cols,
                        }))
                    }
                    _ => Err(String::from("Wrong statement type")),
                },
                None => Err(String::from("Statement not created")),
            },
            _ => Err(String::from("Into keyword expected")),
        }
    }

    fn extract_cols(p: &mut Parser) -> NextFnCall {
        let mut p_copy = p.clone();
        let veci_opt = Self::get_into_parentheses(&mut p_copy);
        let veci = if let Ok(veci) = veci_opt {
            *p = p_copy;
            veci
        } else {
            vec![]
        };
        match p.stmt {
            Some(ref mut stmt) => match stmt {
                &mut Statement::InsertStatement { ref mut cols, .. } => {
                    *cols = veci;
                    Ok(Some(NextFn {
                        call: Self::values_keyword,
                    }))
                }
                _ => Err(String::from("Wrong statement type")),
            },
            None => Err(String::from("Statement not created")),
        }
    }

    fn values_keyword(p: &mut Parser) -> NextFnCall {
        let token = p.scan_ignore_whitespace();
        println!("--->{:?}", token);
        if token != Token::Values {
            Err(String::from("Values keyword expected"))
        } else {
            Ok(Some(NextFn {
                call: Self::extract_values,
            }))
        }
    }

    fn extract_values(p: &mut Parser) -> NextFnCall {
        let veci = Self::get_into_parentheses(p)?;
        match p.stmt {
            Some(ref mut stmt) => match stmt {
                &mut Statement::InsertStatement { ref mut values, .. } => {
                    *values = veci;
                    Ok(Some(NextFn { call: Self::end }))
                }
                _ => Err(String::from("Wrong statement type")),
            },
            None => Err(String::from("Statement not created")),
        }
    }

    fn get_into_parentheses(p: &mut Parser) -> Result<Vec<String>, String> {
        let mut token = p.scan_ignore_whitespace();
        if token != Token::ParLeft {
            return Err(String::from("( expected"));
        }
        let mut veci = vec![];
        loop {
            token = p.scan_ignore_whitespace();
            match token {
                Token::Ident(value) => {
                    veci.push(value);
                    token = p.scan_ignore_whitespace();
                    if token != Token::Comma {
                        p.unscan();
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
        Ok(veci)
    }

    fn end(_p: &mut Parser) -> NextFnCall {
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use lexer::Token;

    #[test]
    fn it_scans() {
        let mut p = Parser::new(String::from(
            "INSERT INTO tbl (name, email) VALUES (demo, demo)",
        ));
        let token = p.scan();
        assert_eq!(token, Token::Insert);
    }

    #[test]
    fn it_stores_buf() {
        let mut p = Parser::new(String::from(
            "INSERT INTO tbl (name, email) VALUES (demo, demo)",
        ));
        let token = p.scan();
        p.unscan();
        assert_eq!(p.buf, Some(token));
    }
}
