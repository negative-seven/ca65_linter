mod ast;

use ast::{
    ExplicitLabel, Expression, ImplicitLabel, InstructionOperand, NumberLiteralBase, Statement,
};
use lalrpop_util::lalrpop_mod;
use std::{error::Error, io::Read};

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

    let mut unreferenced_labels = Vec::new();
    let mut decimal_address_references = Vec::new();
    for statement in &statements {
        match statement {
            Statement::ExplicitLabel(ExplicitLabel { identifier, .. })
            | Statement::ImplicitLabel(ImplicitLabel { identifier }) => {
                unreferenced_labels.push(identifier.string.clone());
            }
            Statement::Instruction(_) => (),
        }
    }
    for statement in statements {
        match statement {
            Statement::Instruction(instruction) => match instruction.operand {
                InstructionOperand::Direct(Expression::Identifier(identifier)) => {
                    unreferenced_labels
                        .iter()
                        .position(|e| *e == identifier.string)
                        .inspect(|i| {
                            unreferenced_labels.remove(*i);
                        });
                }
                InstructionOperand::Direct(Expression::NumberLiteral(ref literal)) => {
                    if literal.base == NumberLiteralBase::Decimal {
                        decimal_address_references.push(instruction);
                    }
                }
                InstructionOperand::None | InstructionOperand::Immediate(_) => (),
            },
            Statement::ExplicitLabel(_) | Statement::ImplicitLabel(_) => (),
        }
    }

    for unreferenced_label in &unreferenced_labels {
        println!("unreferenced label: {unreferenced_label}");
    }

    for decimal_address_reference in &decimal_address_references {
        println!("memory referenced via decimal address: {decimal_address_reference}");
    }

    Ok(())
}
