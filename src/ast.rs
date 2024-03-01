use std::fmt::Display;

#[derive(Clone, Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

pub trait Spanned {
    fn span(&self) -> Span;
}

#[derive(Clone, Debug)]
pub struct Identifier {
    pub string: String,
    pub span: Span,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

impl Spanned for Identifier {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NumberLiteralBase {
    Decimal,
    Hexadecimal,
}

#[derive(Clone, Debug)]
pub struct NumberLiteral {
    pub value: u16,
    pub base: NumberLiteralBase,
    pub span: Span,
}

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.base {
            NumberLiteralBase::Decimal => write!(f, "{}", self.value),
            NumberLiteralBase::Hexadecimal => write!(f, "${:x}", self.value),
        }
    }
}

impl Spanned for NumberLiteral {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, Debug)]
pub enum Expression {
    Identifier(Identifier),
    NumberLiteral(NumberLiteral),
}

impl Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Identifier(e) => write!(f, "{e}"),
            Expression::NumberLiteral(e) => write!(f, "{e}"),
        }
    }
}

impl Spanned for Expression {
    fn span(&self) -> Span {
        match self {
            Expression::Identifier(e) => e.span(),
            Expression::NumberLiteral(e) => e.span(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum InstructionOperandKind {
    None,
    Immediate(Expression),
    Direct(Expression),
}

#[derive(Clone, Debug)]
pub struct InstructionOperand {
    pub kind: InstructionOperandKind,
    pub span: Span,
}

impl Display for InstructionOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            InstructionOperandKind::None => std::fmt::Result::Ok(()),
            InstructionOperandKind::Immediate(e) => write!(f, "#{e}"),
            InstructionOperandKind::Direct(e) => write!(f, "{e}"),
        }
    }
}

impl Spanned for InstructionOperand {
    fn span(&self) -> Span {
        self.span.clone()
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub mnemonic: Identifier,
    pub operand: InstructionOperand,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.mnemonic, self.operand)
    }
}

impl Spanned for Instruction {
    fn span(&self) -> Span {
        Span {
            start: self.mnemonic.span().start,
            end: self.operand.span().end,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ImplicitLabel {
    pub identifier: Identifier,
}

impl Display for ImplicitLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.identifier)
    }
}

impl Spanned for ImplicitLabel {
    fn span(&self) -> Span {
        self.identifier.span()
    }
}

#[derive(Clone, Debug)]
pub struct ExplicitLabel {
    pub identifier: Identifier,
    pub value: Expression,
}

impl Display for ExplicitLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} := {}", self.identifier, self.value)
    }
}

impl Spanned for ExplicitLabel {
    fn span(&self) -> Span {
        Span {
            start: self.identifier.span.start,
            end: self.value.span().end,
        }
    }
}

#[derive(Clone, Debug)]
pub enum Statement {
    ExplicitLabel(ExplicitLabel),
    ImplicitLabel(ImplicitLabel),
    Instruction(Instruction),
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::ExplicitLabel(s) => write!(f, "{s}"),
            Statement::ImplicitLabel(s) => write!(f, "{s}"),
            Statement::Instruction(s) => write!(f, "{s}"),
        }
    }
}

impl Spanned for Statement {
    fn span(&self) -> Span {
        match self {
            Statement::ExplicitLabel(s) => s.span(),
            Statement::ImplicitLabel(s) => s.span(),
            Statement::Instruction(s) => s.span(),
        }
    }
}
