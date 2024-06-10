use std::collections::HashMap;

#[derive(Debug)]
pub struct Values(HashMap<String, i128>);

impl Values {
    pub fn new(input: Vec<String>) -> Self {
        let mut this = Self(HashMap::from([(String::from("x0"), 0i128)]));
        for input_value_index in 2..input.len() {
            let name = String::from(format!("x{}", input_value_index-1));
            let value = input[input_value_index].parse::<i128>().expect("Could't not convert input to i128");
            this.set(name, value);
        }
        this
    }
    pub fn get(&self, name: &String) -> i128 {
        if self.0.contains_key(name) {
            *self.0.get(name).unwrap()
        }
        else {
            0
        }
    }
    pub fn set(&mut self, name: String, value: i128) {
        self.0.insert(name, value);
    }
}