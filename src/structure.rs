use regex::Regex;

#[derive(Debug)]
struct Program {
    statements: Vec<Statement>,
}

#[derive(Debug)]
struct Statement {
    pattern: Option<Pattern>,
    actions: Vec<Action>,
}

#[derive(Debug)]
enum Pattern {
    BEGIN,
    END,
    Variable(Variable),
    Field(isize),
    Regex(Regex),
    And(Box<Pattern>, Box<Pattern>),
    Or(Box<Pattern>, Box<Pattern>),
    Not(Box<Pattern>),
    Equal(Box<Pattern>, Box<Pattern>),
    Match(Variable, Regex),
    NoMatch(Variable, Regex),
}

#[derive(Debug)]
struct Action {
    
}

#[derive(Debug)]
struct Variable {
    name: String,
}