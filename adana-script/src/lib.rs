mod ast;
mod compute;
mod parser;
mod prelude;
mod require_dynamic_lib;
mod string_parser;
use std::collections::BTreeMap;

use adana_script_core::TreeNodeValue;
pub use compute::compute;

use slab_tree::Tree;

use crate::ast::to_ast;
use crate::parser::parse_instructions;

pub fn print_ast(script: &str) -> anyhow::Result<()> {
    let (rest, instructions) = parse_instructions(script).map_err(|e| {
        anyhow::Error::msg(format!(
            "PRINT AST ERROR: could not parse instructions. {e}",
        ))
    })?;

    anyhow::ensure!(
        rest.trim().is_empty(),
        format!(
            "PRINT AST ERROR: rest is not empty! {instructions:#?} => {rest}",
        )
    );

    let mut dummy_ctx = BTreeMap::new();
    for instruction in instructions {
        let mut tree: Tree<TreeNodeValue> = Tree::new();

        println!("==================INSTRUCTION================");
        println!("{instruction:#?}");
        to_ast(&mut dummy_ctx, instruction, &mut tree, &None)?;

        let mut tree_fmt = String::new();
        tree.write_formatted(&mut tree_fmt)?;
        println!("===================AST TREE==================");
        print!("{tree_fmt}");
    }
    Ok(())
}

// keep this
#[cfg(test)]
mod tests;
