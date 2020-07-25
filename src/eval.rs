use crate::ast;
use simple_error::{bail,SimpleError,SimpleResult};
use crate::abstract_interpret;
use std::collections::HashMap;
use abstract_interpret::{AbstractInterpret,interpret};

struct ConcreteEval<'a> {
    env: &'a HashMap<String,i64>,
}

impl AbstractInterpret for ConcreteEval<'_> {
    type Output = i64;

    fn constant(&mut self, c: i64) -> Self::Output { c }

    fn add(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l + r }

    fn sub(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l - r }

    fn mul(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l * r }

    fn div(&mut self, l: &Self::Output, r: &Self::Output) -> SimpleResult<Self::Output> {
        if *r == 0 {
            bail!("division by zero");
        }
        Ok(l / r)
    }
    fn shr(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l >> r }
    fn shl(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l << r }
    fn neq(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { (l == r) as i64 }
    fn eq(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { (l != r) as i64 }
    fn neg(&mut self, e: &Self::Output) -> Self::Output { -e }
    fn lookup(&mut self, var: &str) -> SimpleResult<Self::Output> {
        match self.env.get(var) {
            Some(v) => Ok(*v),
            None => Err(SimpleError::new("undefined variable"))
        }
    }
}

pub fn eval(
    ctx: &mut ast::Context,
    node: ast::NodeId,
    env: &HashMap<String,i64>
) -> SimpleResult<i64> {
    let eval = &mut ConcreteEval { env };
    interpret(eval, ctx, node)
}
