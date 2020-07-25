use crate::ast;
use ast::Node;
use simple_error::SimpleResult;

pub trait AbstractInterpret {
    type Output;

    fn constant(&mut self, c: i64) -> Self::Output;
    fn add(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output;
    fn sub(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output;
    fn mul(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output;
    fn div(&mut self, l: &Self::Output, r: &Self::Output) -> SimpleResult<Self::Output>;
    fn shr(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output;
    fn shl(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output;
    fn neq(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output;
    fn eq(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output;
    fn neg(&mut self, l: &Self::Output) -> Self::Output;
    fn lookup(&mut self, var: &str) -> SimpleResult<Self::Output>;
}

pub fn interpret<A>(
    interpreter: &mut A,
    ctx: &mut ast::Context,
    node: ast::NodeId,
) -> SimpleResult<A::Output>
where
    A: AbstractInterpret,
{
    match *ctx.node_ref(node) {
        Node::Const(i) => Ok(interpreter.constant(i)),
        Node::Identifier(i) => {
            let s = ctx.interned(i);
            interpreter.lookup(s)
        },
        Node::Addition(l,r) => {
            let l = interpret(interpreter, ctx, l)?;
            let r = interpret(interpreter, ctx, r)?;
            Ok(interpreter.add(&l, &r))
        },
        Node::Subtraction(l,r) => {
            let l = interpret(interpreter, ctx, l)?;
            let r = interpret(interpreter, ctx, r)?;
            Ok(interpreter.sub(&l, &r))
        }
        Node::Multiplication(l,r) => {
            let l = interpret(interpreter, ctx, l)?;
            let r = interpret(interpreter, ctx, r)?;
            Ok(interpreter.mul(&l, &r))
        },
        Node::Division(l,r) => {
            let l = interpret(interpreter, ctx, l)?;
            let r = interpret(interpreter, ctx, r)?;
            interpreter.div(&l, &r)
        },
        Node::RightShift(l,r) => {
            let l = interpret(interpreter, ctx, l)?;
            let r = interpret(interpreter, ctx, r)?;
            Ok(interpreter.shr(&l, &r))
        },
        Node::LeftShift(l,r) => {
            let l = interpret(interpreter, ctx, l)?;
            let r = interpret(interpreter, ctx, r)?;
            Ok(interpreter.shl(&l, &r))
        },
        Node::Negation(e) => {
            let e = interpret(interpreter, ctx, e)?;
            Ok(interpreter.neg(&e))
        },
        Node::Conditional(cond,true_br,false_br) => {
            let cond = interpret(interpreter, ctx, cond)?;
            let true_br = interpret(interpreter, ctx, true_br)?;
            let false_br = interpret(interpreter, ctx, false_br)?;
            let zero = interpreter.constant(0); 
            let eq_zero = interpreter.eq(&cond,&zero);
            let neq_zero = interpreter.neq(&cond,&zero);
            let true_br = interpreter.mul(&neq_zero,&true_br);
            let false_br = interpreter.mul(&eq_zero,&false_br);
            Ok(interpreter.add(&true_br,&false_br))
        },
    }
}
