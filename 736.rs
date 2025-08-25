//736. Parse Lisp Expression

use std::collections::{HashMap, VecDeque};


impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let mut parser = Parser::new(expression);
        parser.eval()
    }
}

struct Parser {
    expr: Vec<char>,
    i: usize,
    scope: HashMap<String, VecDeque<i32>>,
}

impl Parser {
    fn new(expr: String) -> Self {
        Parser {
            expr: expr.chars().collect(),
            i: 0,
            scope: HashMap::new(),
        }
    }

    fn eval(&mut self) -> i32 {
        if self.expr[self.i] != '(' {
            if self.expr[self.i].is_ascii_lowercase() {
                let var = self.parse_var();
                return *self.scope.get(&var).unwrap().back().unwrap();
            } else {
                return self.parse_int();
            }
        }

        self.i += 1; 
        let op = self.expr[self.i];
        let mut ans = 0;

        if op == 'l' {
            self.i += 4; 
            let mut vars = vec![];
            loop {
                let var = self.parse_var();
                if self.expr[self.i] == ')' {
                    ans = *self.scope.get(&var).unwrap().back().unwrap();
                    break;
                }
                vars.push(var.clone());
                self.i += 1;
                let val = self.eval();
                self.scope.entry(var).or_default().push_back(val);
                self.i += 1;
                if !self.expr[self.i].is_ascii_lowercase() {
                    ans = self.eval();
                    break;
                }
            }
            for var in vars {
                self.scope.get_mut(&var).unwrap().pop_back();
            }
        } else {
            let is_add = op == 'a';
            self.i += if is_add { 4 } else { 5 };
            let a = self.eval();
            self.i += 1;
            let b = self.eval();
            ans = if is_add { a + b } else { a * b };
        }

        self.i += 1; 
        ans
    }

    fn parse_var(&mut self) -> String {
        let start = self.i;
        while self.i < self.expr.len() && self.expr[self.i] != ' ' && self.expr[self.i] != ')' {
            self.i += 1;
        }
        self.expr[start..self.i].iter().collect()
    }

    fn parse_int(&mut self) -> i32 {
        let mut sign = 1;
        if self.expr[self.i] == '-' {
            sign = -1;
            self.i += 1;
        }
        let mut v = 0;
        while self.i < self.expr.len() && self.expr[self.i].is_ascii_digit() {
            v = v * 10 + (self.expr[self.i] as u8 - b'0') as i32;
            self.i += 1;
        }
        sign * v
    }
}
