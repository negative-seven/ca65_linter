mod ast;

use ast::{
    ExplicitLabel, Expression, ImplicitLabel, InstructionOperandKind, NumberLiteralBase, Span,
    Spanned, Statement,
};
use lalrpop_util::lalrpop_mod;
use std::{error::Error, io::Read};

lalrpop_mod!(pub ca65);

#[allow(clippy::too_many_lines)] // TODO
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

    let source_line_character_ranges = {
        let newline_indices = source.char_indices().filter(|c| c.1 == '\n').map(|c| c.0);
        let start_indices = std::iter::once(0).chain(newline_indices.clone().map(|i| i + 1));
        let end_indices = newline_indices.chain(std::iter::once(source.len()));
        start_indices.zip(end_indices).map(|(a, b)| a..b)
    };

    #[allow(clippy::items_after_statements)]
    struct SourceLine<'a> {
        text: &'a str,
        character_index: usize,
        line_index: usize,
    }

    let mut source_by_lines = Vec::new();
    for (line_index, character_index_range) in source_line_character_ranges.enumerate() {
        let text = &source[character_index_range.clone()];
        let text = text.strip_suffix('\r').unwrap_or(text);
        source_by_lines.push(SourceLine {
            text,
            character_index: character_index_range.start,
            line_index,
        });
    }

    rule_violations.sort_by_key(|v| v.span.start);
    for rule_violation in rule_violations {
        let line = source_by_lines
            .iter()
            .rev()
            .find(|line| rule_violation.span.start >= line.character_index)
            .unwrap();

        println!("rule violation: {}", rule_violation.description);
        println!(
            "at line {}, position {}",
            line.line_index + 1,
            rule_violation.span.start - line.character_index,
        );
        println!("{}", line.text);
        let rule_violation_span_within_line = Span {
            start: rule_violation.span.start - line.character_index,
            end: rule_violation.span.end - line.character_index,
        };
        println!(
            "{}{}",
            " ".repeat(rule_violation_span_within_line.start),
            "^".repeat(rule_violation_span_within_line.end - rule_violation_span_within_line.start)
        );
        println!();
    }

    Ok(())
}

struct RuleViolation {
    description: String,
    span: Span,
}
