// AST (Abstract Syntax Tree) definitions

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    Import(String),
    Definition {
        name: String,
        value: Expression,
    },
    Function {
        name: String,
        params: Vec<String>,
        body: Vec<Statement>,
        is_async: bool,
    },
    CustomBlock {
        language: String,
        code: String,
    },
    Main {
        body: Vec<Statement>,
    },
    Expression(Expression),
    Return(Option<Expression>),
    If {
        condition: Expression,
        then_body: Vec<Statement>,
        else_body: Option<Vec<Statement>>,
    },
    Loop {
        body: Vec<Statement>,
    },
    For {
        variable: String,
        iterable: Expression,
        body: Vec<Statement>,
    },
    While {
        condition: Expression,
        body: Vec<Statement>,
    },
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Integer(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Identifier(String),
    Binary {
        left: Box<Expression>,
        op: BinaryOp,
        right: Box<Expression>,
    },
    Unary {
        op: UnaryOp,
        operand: Box<Expression>,
    },
    Call {
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    Await(Box<Expression>),
    UiSprite {
        sprite_type: String,
        properties: Vec<(String, Expression)>,
    },
    List(Vec<Expression>),
    Dict(Vec<(String, Expression)>),
    Index {
        object: Box<Expression>,
        index: Box<Expression>,
    },
    Member {
        object: Box<Expression>,
        member: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    And,
    Or,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
    Negate,
    Not,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ownership {
    Own,
    Ref,
    Copy,
}
