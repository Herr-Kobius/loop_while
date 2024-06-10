use std::thread::sleep;
use std::time::Duration;

use crate::token::{Token, Keyword, Symbol};
use crate::values::Values;

pub struct VM {
    values: Values
}

impl VM {
    pub fn new(values: Values) -> Self {
        Self{
            values
        }
    }

    pub fn execute_program(&mut self, mut program_tokens: Vec<Token>, debug: bool) {
        while !program_tokens.is_empty() {
            if debug {
                sleep(Duration::from_millis(70));
                println!("{:?}\n", program_tokens);
            }
            
            match &program_tokens[0] {
                Token::Keyword(Keyword::LOOP) => {
                    let loop_var = match &program_tokens[1] {
                        Token::Variable(t) => t.clone(),
                        _ => panic!("Expected Token Variable")
                    };
                    let end_index = program_tokens.len()-program_tokens.iter().rev().position(|r| r == &Token::Keyword(Keyword::END)).unwrap()-1;
                    if debug {println!("LOOP Inner Tokens:\n {:#?}", program_tokens[3..end_index].to_vec());}
                    for _ in 0..self.values.get(&loop_var) {
                        self.execute_program(program_tokens[3..end_index].to_vec(), debug)
                    }
                    program_tokens = program_tokens.split_at(end_index+1).1.to_vec();
                },
                Token::Keyword(Keyword::WHILE) => {
                    let loop_var = match &program_tokens[1] {
                        Token::Variable(t) => t.clone(),
                        _ => panic!("Expected Token Variable")
                    };
                    let end_index = program_tokens.len()-program_tokens.iter().rev().position(|r| r == &Token::Keyword(Keyword::END)).unwrap()-1;
                    if debug {println!("WHILE Inner Tokens:\n {:#?}", program_tokens[3..end_index].to_vec());}
                    while self.values.get(&loop_var)!=0 {
                        self.execute_program(program_tokens[5..end_index].to_vec(), debug)
                    }
                    program_tokens = program_tokens.split_at(end_index+1).1.to_vec();
                }
                
                Token::Variable(var_set) => {
                    let var_get = match &program_tokens[2] {
                        Token::Variable(t) => t.clone(),
                        _ => panic!("Expected Token Variable")
                    };
                    let num = match &program_tokens[4] {
                        Token::Number(t) => t.clone() as i128,
                        _ => panic!("Expected Token Number")
                    };
    
                    let value:i128;
                    match &program_tokens[3] {
                        Token::Symbol(Symbol::ADD) => {
                            value = self.values.get(&var_get) + num;
                        }
                        Token::Symbol(Symbol::SUB) => {
                            value = self.values.get(&var_get) - num;
                        }
                        
                        token => {panic!("Unknown Symbol: {:?}", token)}
                    }
                    self.values.set(var_set.clone(), value);
                    
                    program_tokens = program_tokens.split_at(6).1.to_vec();
                }
                _ => {}
            }
        }
    }

    pub fn get_result(&self) -> i128{
        self.values.get(&"x0".to_string())
    }
}
