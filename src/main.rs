#[macro_use] extern crate simple_error;
#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

mod eval;
mod synthesize;
mod ast;
mod abstract_interpret;

use std::env;
use std::error::Error;
use simple_error::bail;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        bail!("usage: minisynth <spec> <template>");
    }

    let ast_ctx = &mut ast::Context::default();
    let config = z3::Config::new();
    let z3_ctx = &z3::Context::new(&config);
    let spec_node = try_with!(grammar::StartParser::new().parse(ast_ctx, &args[1]), "parse error");
    let templ_node = try_with!(grammar::StartParser::new().parse(ast_ctx, &args[2]), "parse error");
    let answer = synthesize::synthesize(z3_ctx, ast_ctx, spec_node, templ_node)?;
    println!("{:?}", answer);
    Ok(())
}
