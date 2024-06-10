use std::fs;
use std::collections::HashMap;

use crate::token::{Token, Keyword, Symbol};

#[derive(Debug)]
pub struct Syntax {
    pub keywords:HashMap<String, Token>,
    pub symbols:HashMap<String, Token>,
    pub var_prefix: char
} 

impl Syntax {
    pub fn from(path: &str) -> Self {
        let file_content = fs::read_to_string(path).expect("Could't read file");
        let syntax_json:serde_json::Value = serde_json::from_str(file_content.as_str()).expect("JSON was not well-formatted");

        let keywords_json = syntax_json.get("keywords").expect("Missing Keywords");
        let symbols_json = syntax_json.get("symbols").expect("Missing symbols");

        let mut keywords = HashMap::new();
        let mut symbols = HashMap::new();
        let var_prefix: char;

        for (key, value) in keywords_json.as_object().unwrap() {
            let token = match key.as_str() {
                "loop" => Keyword::LOOP,
                "while"=> Keyword::WHILE,
                "do"=> Keyword::DO,
                "end"=> Keyword::END,
                _ => {panic!("Unknown Keyword")}
            };
            keywords.insert(String::from(value.as_str().unwrap()), Token::Keyword(token));
        }

        for (key, value) in symbols_json.as_object().unwrap() {
            let token = match key.as_str() {
                "eol" => Symbol::EOL,
                "assignment"=> Symbol::ASSIGNMENT,
                "add"=> Symbol::ADD,
                "sub"=> Symbol::SUB,
                "ueq"=> Symbol::UEQ,
                _ => {panic!("Unknown Symbol")}
            };
            symbols.insert(String::from(value.as_str().unwrap()), Token::Symbol(token));
        }
        var_prefix = syntax_json.get("variables").unwrap().get("prefix").unwrap().as_str().unwrap().chars().last().unwrap();
        Self{keywords, symbols, var_prefix}
    
    }
}
