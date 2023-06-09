use crate::parser::tests::check_script_parser;
use boa_ast::{
    expression::literal::Literal,
    statement::{Block, Break, Labelled, LabelledItem, WhileLoop},
    Statement, StatementListItem,
};
use boa_interner::{Interner, Sym};
use boa_macros::utf16;

#[test]
fn inline() {
    check_script_parser(
        "while (true) break;",
        vec![Statement::WhileLoop(WhileLoop::new(
            Literal::from(true).into(),
            Break::new(None).into(),
        ))
        .into()],
        &mut Interner::default(),
    );
}

#[test]
fn new_line() {
    check_script_parser(
        "while (true)
            break;",
        vec![Statement::WhileLoop(WhileLoop::new(
            Literal::from(true).into(),
            Break::new(None).into(),
        ))
        .into()],
        &mut Interner::default(),
    );
}

#[test]
fn inline_block_semicolon_insertion() {
    check_script_parser(
        "while (true) {break}",
        vec![Statement::WhileLoop(WhileLoop::new(
            Literal::from(true).into(),
            Block::from(vec![StatementListItem::Statement(Statement::Break(
                Break::new(None),
            ))])
            .into(),
        ))
        .into()],
        &mut Interner::default(),
    );
}

#[test]
fn new_line_semicolon_insertion() {
    let interner = &mut Interner::default();
    check_script_parser(
        "test: while (true) {
            break test
        }",
        vec![Statement::Labelled(Labelled::new(
            LabelledItem::Statement(Statement::WhileLoop(WhileLoop::new(
                Literal::from(true).into(),
                Block::from(vec![StatementListItem::Statement(Statement::Break(
                    Break::new(Some(interner.get_or_intern_static("test", utf16!("test")))),
                ))])
                .into(),
            ))),
            interner.get_or_intern_static("test", utf16!("test")),
        ))
        .into()],
        interner,
    );
}

#[test]
fn inline_block() {
    check_script_parser(
        "while (true) {break;}",
        vec![Statement::WhileLoop(WhileLoop::new(
            Literal::from(true).into(),
            Block::from(vec![StatementListItem::Statement(Statement::Break(
                Break::new(None),
            ))])
            .into(),
        ))
        .into()],
        &mut Interner::default(),
    );
}

#[test]
fn new_line_block() {
    let interner = &mut Interner::default();
    check_script_parser(
        "test: while (true) {
            break test;
        }",
        vec![Statement::Labelled(Labelled::new(
            LabelledItem::Statement(Statement::WhileLoop(WhileLoop::new(
                Literal::from(true).into(),
                Block::from(vec![StatementListItem::Statement(Statement::Break(
                    Break::new(Some(interner.get_or_intern_static("test", utf16!("test")))),
                ))])
                .into(),
            ))),
            interner.get_or_intern_static("test", utf16!("test")),
        ))
        .into()],
        interner,
    );
}

#[test]
fn reserved_label() {
    let interner = &mut Interner::default();
    check_script_parser(
        "await: while (true) {
            break await;
        }",
        vec![Statement::Labelled(Labelled::new(
            LabelledItem::Statement(Statement::WhileLoop(WhileLoop::new(
                Literal::from(true).into(),
                Block::from(vec![StatementListItem::Statement(Statement::Break(
                    Break::new(Some(Sym::AWAIT)),
                ))])
                .into(),
            ))),
            Sym::AWAIT,
        ))
        .into()],
        interner,
    );

    let interner = &mut Interner::default();
    check_script_parser(
        "yield: while (true) {
            break yield;
        }",
        vec![Statement::Labelled(Labelled::new(
            LabelledItem::Statement(Statement::WhileLoop(WhileLoop::new(
                Literal::from(true).into(),
                Block::from(vec![StatementListItem::Statement(Statement::Break(
                    Break::new(Some(Sym::YIELD)),
                ))])
                .into(),
            ))),
            Sym::YIELD,
        ))
        .into()],
        interner,
    );
}

#[test]
fn new_line_block_empty() {
    check_script_parser(
        "while (true) {
            break;
        }",
        vec![Statement::WhileLoop(WhileLoop::new(
            Literal::from(true).into(),
            Block::from(vec![StatementListItem::Statement(Statement::Break(
                Break::new(None),
            ))])
            .into(),
        ))
        .into()],
        &mut Interner::default(),
    );
}

#[test]
fn new_line_block_empty_semicolon_insertion() {
    check_script_parser(
        "while (true) {
            break
        }",
        vec![Statement::WhileLoop(WhileLoop::new(
            Literal::from(true).into(),
            Block::from(vec![StatementListItem::Statement(Statement::Break(
                Break::new(None),
            ))])
            .into(),
        ))
        .into()],
        &mut Interner::default(),
    );
}
