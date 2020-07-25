#[macro_use] extern crate simple_error;
#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate maplit;

lalrpop_mod!(pub grammar);

mod eval;
mod ast;
mod abstract_interpret;

use std::env;
use std::error::Error;
use std::collections::HashMap;
use simple_error::bail;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: minisynth <expr>");
    }
    let vars : HashMap<String,i64> = hashmap!{
        "x".to_string() => 42,
        "y".to_string() => 10,
    };

    let ctx = &mut ast::Context::default();
    let node = try_with!(grammar::StartParser::new().parse(ctx, &args[1]), "parse error");
    let answer = eval::eval(ctx, node, &vars)?;
    
    println!("vars: {:?}", vars);
    println!("{} = {}", &args[1], answer);
    Ok(())
}
