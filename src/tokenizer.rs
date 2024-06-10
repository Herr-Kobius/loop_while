use crate::token::Token;
use crate::syntax::Syntax;

#[derive(Debug)]
pub struct Tokenizer {
    source: String,
    tokens: Vec<Token>
} 

impl Tokenizer {
    pub fn new(source: String) -> Self {
        Self { 
            source, 
            tokens: vec![] }
    }


    pub fn tokenize(&mut self) -> Vec<Token> {
        let syntax = Syntax::from("syntax.json");
        //println!("{:#?}", syntax); 
        while !self.source.is_empty() {
            let begin_len = self.source.len();
            for (pattern, token) in &syntax.keywords {
                if self.source.starts_with(pattern) {
                    self.tokens.push(token.clone());
                    self.source = self.source.split_off(pattern.len())
                }
            }

            if self.source.is_empty() {break;}

            for (pattern, token) in &syntax.symbols {
                if self.source.starts_with(pattern) {
                    self.tokens.push(token.clone());
                    self.source = self.source.split_off(pattern.len())
                }
            }

            if self.source.is_empty() {break;}

            if self.source.starts_with(syntax.var_prefix) {
                let mut i = 1;
                while self.source.chars().collect::<Vec<char>>()[i].is_ascii_digit() {
                    i += 1;
                }
                let source_cpm = self.source.clone();
                let (first, last) = source_cpm.split_at(i);
                self.source = String::from(last);
                self.tokens.push(Token::Variable(String::from(first)));
            }

            if self.source.is_empty() {break;}

            if self.source.chars().collect::<Vec<char>>()[0].is_ascii_digit() {
                let mut i = 1;
                while self.source.chars().collect::<Vec<char>>()[i].is_ascii_digit() {
                    i += 1;
                }
                let source_cpm = self.source.clone();
                let (first, last) = source_cpm.split_at(i);
                self.source = String::from(last);
                self.tokens.push(Token::Number(first.parse::<u128>().expect("Could'self not convert input to u128")));
            }
        
            if begin_len == self.source.len() {
                panic!("Unknown Keyword or Symbol")
            }
        }

        self.tokens.clone()
    }
}
