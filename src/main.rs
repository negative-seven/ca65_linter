mod ast;

use ast::{
    ExplicitLabel, Expression, ImplicitLabel, InstructionOperandKind, NumberLiteralBase, Span,
    Statement,
};
use lalrpop_util::lalrpop_mod;
use std::{error::Error, io::Read};

use crate::ast::Spanned;

lalrpop_mod!(pub ca65);

fn main() -> Result<(), Box<dyn Error>> {
    let mut arguments = std::env::args().skip(1);
    let Some(source_filename) = arguments.next() else {
        eprintln!("missing argument: ca65 source filepath");
        std::process::exit(1);
    };

    let mut source_file = std::fs::File::open(source_filename)?;
    let mut source = String::new();
    source_file.read_to_string(&mut source)?;

    let parser = ca65::StatementsParser::new();
    let statements = match parser.parse(&source) {
        Ok(statements) => statements,
        Err(e) => {
            eprintln!("failed to parse provided source file:");
            eprintln!("{e}");
            std::process::exit(1);
        }
    };

    let mut rule_violations = Vec::new();
    let mut unreferenced_labels = Vec::new();
    for statement in &statements {
        match statement {
            Statement::ExplicitLabel(ExplicitLabel { identifier, .. })
            | Statement::ImplicitLabel(ImplicitLabel { identifier }) => {
                unreferenced_labels.push(identifier.clone());
            }
            Statement::Instruction(_) => (),
        }
    }
    for statement in statements {
        match statement {
            Statement::Instruction(instruction) => match instruction.operand.kind {
                InstructionOperandKind::Direct(Expression::Identifier(identifier)) => {
                    unreferenced_labels
                        .iter()
                        .position(|e| e.string == identifier.string)
                        .inspect(|i| {
                            unreferenced_labels.remove(*i);
                        });
                }
                InstructionOperandKind::Direct(Expression::NumberLiteral(ref literal)) => {
                    if literal.base == NumberLiteralBase::Decimal {
                        rule_violations.push(RuleViolation {
                            description: "memory referenced via decimal address".to_string(),
                            span: instruction.operand.span(),
                        });
                    }
                }
                InstructionOperandKind::None | InstructionOperandKind::Immediate(_) => (),
            },
            Statement::ExplicitLabel(_) | Statement::ImplicitLabel(_) => (),
        }
    }

    for unreferenced_label in &unreferenced_labels {
        rule_violations.push(RuleViolation {
            description: "unreferenced label".to_string(),
            span: unreferenced_label.span(),
        });
    }

    for rule_violation in rule_violations {
        println!("rule violation: {}", rule_violation.description);
        println!(
            "at: {}..{}",
            rule_violation.span.start, rule_violation.span.end
        );
        println!();
    }

    Ok(())
}

struct RuleViolation {
    description: String,
    span: Span,
}
