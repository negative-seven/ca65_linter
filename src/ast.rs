use std::fmt::Display;

#[derive(Debug)]
pub struct Identifier {
    pub string: String,
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.string)
    }
}

#[derive(Debug)]
pub struct NumberLiteral(pub u16);

impl Display for NumberLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
pub enum InstructionOperand {
    None,
    Immediate(Expression),
    Direct(Expression),
}

impl Display for InstructionOperand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionOperand::None => std::fmt::Result::Ok(()),
            InstructionOperand::Immediate(e) => write!(f, "#{e}"),
            InstructionOperand::Direct(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug)]
pub struct Instruction {
    pub mnemonic: Identifier,
    pub operand: InstructionOperand,
}

impl Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.mnemonic, self.operand)
    }
}

#[derive(Debug)]
pub struct ImplicitLabel {
    pub identifier: Identifier,
}

impl Display for ImplicitLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:", self.identifier)
    }
}

#[derive(Debug)]
pub struct ExplicitLabel {
    pub identifier: Identifier,
    pub value: Expression,
}

impl Display for ExplicitLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} := {}", self.identifier, self.value)
    }
}

#[derive(Debug)]
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
