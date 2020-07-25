use crate::ast;
use ast::Node;
use simple_error::{bail,SimpleResult};

pub fn eval<L>(
    ctx: &mut ast::Context,
    node: ast::NodeId,
    lookup: &mut L,
) -> SimpleResult<i64>
where
    L: for<'a> FnMut(&'a str) -> SimpleResult<i64>
{
    match *ctx.node_ref(node) {
        Node::Const(i) => Ok(i),
        Node::Identifier(i) => {
            let s = ctx.interned(i);
            lookup(s)
        },
        Node::Addition(l,r) => {
            let l = eval(ctx, l, lookup)?;
            let r = eval(ctx, r, lookup)?;
            Ok(l + r)
        },
        Node::Subtraction(l,r) => {
            let l = eval(ctx, l, lookup)?;
            let r = eval(ctx, r, lookup)?;
            Ok(l - r)
        }
        Node::Multiplication(l,r) => {
            let l = eval(ctx, l, lookup)?;
            let r = eval(ctx, r, lookup)?;
            Ok(l * r)
        },
        Node::Division(l,r) => {
            let l = eval(ctx, l, lookup)?;
            let r = eval(ctx, r, lookup)?;
            if r == 0 {
                bail!("division by zero");
            }
            Ok(l / r)
        },
        Node::RightShift(l,r) => {
            let l = eval(ctx, l, lookup)?;
            let r = eval(ctx, r, lookup)?;
            Ok(l >> r)
        },
        Node::LeftShift(l,r) => {
            let l = eval(ctx, l, lookup)?;
            let r = eval(ctx, r, lookup)?;
            Ok(l << r)
        },
        Node::Negation(e) => {
            let e = eval(ctx, e, lookup)?;
            Ok(-e)
        },
        Node::Conditional(cond,true_br,false_br) => {
            let cond = eval(ctx, cond, lookup)?;
            Ok(if cond != 0 {
                eval(ctx, true_br, lookup)?
            }
            else {
                eval(ctx, false_br, lookup)?
            })
        },
    }
}
