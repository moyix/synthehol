#[macro_use] extern crate simple_error;
#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate maplit;

lalrpop_mod!(pub grammar);

mod eval;
mod ast;

use std::env;
use std::error::Error;
use std::collections::HashMap;
use simple_error::{SimpleResult,SimpleError,bail};

pub fn eval(src: &str, env: &HashMap<String,i64>) -> SimpleResult<i64> {
    let ctx = &mut ast::Context::default();
    let node = try_with!(grammar::StartParser::new().parse(ctx, src), "parse error");
    eval::eval(ctx, node, &mut |s| {
        match env.get(s) {
            Some(v) => Ok(*v),
            None => Err(SimpleError::new("undefined variable"))
        }
    })
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        bail!("usage: minisynth <expr>");
    }
    let vars : HashMap<String,i64> = hashmap!{
        "x".to_string() => 42,
        "y".to_string() => 10,
    };
    let answer = eval(&args[1], &vars)?;
    println!("vars: {:?}", vars);
    println!("{} = {}", &args[1], answer);
    Ok(())
}
