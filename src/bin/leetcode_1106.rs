//! 解析布尔表达式

struct Parser<'a> {
    exp: &'a [u8],
    i: usize,
}

impl<'a> Parser<'a> {
    pub fn new(exp: &'a [u8]) -> Self {
        Parser { exp, i: 0 }
    }

    fn match_(&mut self, ch: u8) {
        if self.exp[self.i] != ch {
            panic!("expect {}, but get{}", ch as char, self.exp[self.i] as char);
        }
        self.i += 1;
    }

    fn next(&mut self) -> u8 {
        self.i += 1;
        self.exp[self.i - 1]
    }

    fn pre(&self) -> u8 {
        self.exp[self.i]
    }

    fn expr_body(&mut self) -> Vec<bool> {
        let mut result = vec![];
        self.match_(b'(');
        while self.pre() != b')' {
            match self.pre() {
                b't' => {
                    result.push(true);
                    self.match_(b't');
                }
                b'f' => {
                    result.push(false);
                    self.match_(b'f');
                }
                b'&' | b'!' | b'|' => {
                    result.push(self.expr());
                }
                b',' => self.match_(b','),
                other => unreachable!("{}", other as char),
            }
        }
        self.match_(b')');
        result
    }

    fn expr(&mut self) -> bool {
        match self.next() {
            b'!' => !self.expr_body()[0],
            b'&' => {
                self.expr_body().iter().all(|x| *x)
            }
            b'|' => {
                self.expr_body().iter().any(|x| *x)
            }
            _ => unreachable!()
        }
    }
}

pub fn parse_bool_expr(expression: String) -> bool {
    Parser::new(expression.as_bytes()).expr()
}

fn main() {
    fn test(func: fn(expression: String) -> bool) {
        assert_eq!(func(String::from("|(&(t,f,t),!(t))")), false);
        assert_eq!(func(String::from("!(f)")), true);
        assert_eq!(func(String::from("|(f,t)")), true);
        assert_eq!(func(String::from("&(t,f)")), false);
    }
    test(parse_bool_expr);
}
