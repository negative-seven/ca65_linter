mod ast;

use ast::{ExplicitLabel, Statement};
use lalrpop_util::lalrpop_mod;
use std::{
    collections::{BTreeMap, BTreeSet},
    error::Error,
    io::Read,
};

use crate::ast::{Expression, InstructionOperand};

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

    let mut unreferenced_labels = BTreeSet::new();
    for statement in &statements {
        match statement {
            Statement::ExplicitLabel(explicit_label) => {
                unreferenced_labels.insert(explicit_label.identifier.string.clone());
            }
            Statement::ImplicitLabel(implicit_label) => {
                unreferenced_labels.insert(implicit_label.identifier.string.clone());
            }
            Statement::Instruction(_) => (),
        }
    }
    for statement in statements {
        match statement {
            Statement::Instruction(instruction) => match instruction.operand {
                InstructionOperand::Direct(Expression::Identifier(identifier)) => {
                    unreferenced_labels.remove(&identifier.string);
                }
                InstructionOperand::Direct(Expression::NumberLiteral(_))
                | InstructionOperand::None
                | InstructionOperand::Immediate(_) => (),
            },
            Statement::ExplicitLabel(_) | Statement::ImplicitLabel(_) => (),
        }
    }

    for unreferenced_label in &unreferenced_labels {
        println!("unreferenced label: {unreferenced_label}");
    }

    Ok(())
}
