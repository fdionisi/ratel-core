use grammar::LiteralValue;
use grammar::OperatorType;

#[derive(Debug, PartialEq)]
pub enum KeywordKind {
    Break,
    Do,
    Case,
    Else,
    Var,
    Let,
    Catch,
    Export,
    Class,
    Extends,
    Return,
    While,
    Const,
    Finally,
    Super,
    With,
    Continue,
    For,
    Switch,
    Yield,
    Debugger,
    Function,
    This,
    Default,
    If,
    Throw,
    Import,
    Try,
    Await,
    Static,
}

#[derive(Debug, PartialEq)]
pub enum ReservedKind {
    Enum,
    Implements,
    Package,
    Protected,
    Interface,
    Private,
    Public,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    LineTermination,
    Semicolon,
    Comma,
    Colon,
    Operator(OperatorType),
    ParenOn,
    ParenOff,
    BracketOn,
    BracketOff,
    BlockOn,
    BlockOff,
    Keyword(KeywordKind),
    Reserved(ReservedKind),
    Identifier(String),
    Literal(LiteralValue),
}
