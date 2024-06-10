#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Keyword(Keyword),
    Symbol(Symbol),
    Number(u128),
    Variable(String)
}
#[derive(Debug, Clone, PartialEq)]
pub enum Keyword {
    LOOP,
    WHILE,
    DO,
    END
}
#[derive(Debug, Clone, PartialEq)]
pub enum Symbol {
    EOL,
    ASSIGNMENT,
    ADD,
    SUB,
    UEQ
}
