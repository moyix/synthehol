use std::collections::{HashMap,HashSet};
use crate::abstract_interpret::{interpret,AbstractInterpret};
use crate::ast::{self,NodeId};
use simple_error::{SimpleResult,bail};
use z3;
use z3::ast::{Ast,Dynamic};
use z3::Pattern;

struct Synthesize<'a,'ctx> 
where
    'ctx: 'a {
    ctx: &'ctx z3::Context,
    // all variables
    vars: &'a mut HashMap<String, z3::ast::BV<'ctx>>,
    // holes
    holes: &'a mut HashMap<z3::ast::BV<'ctx>, String>,
    // constants
    const_vars: &'a mut HashSet<z3::ast::BV<'ctx>>,
}

impl<'a,'ctx> AbstractInterpret for Synthesize<'a,'ctx> {
    type Output = z3::ast::BV<'ctx>;
    fn constant(&mut self, c: i64) -> Self::Output {
        z3::ast::BV::from_i64(self.ctx, c, 64)
    }
    fn add(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l.bvadd(&r) }
    fn sub(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l.bvsub(&r) }
    fn mul(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l.bvmul(&r) }
    fn div(&mut self, l: &Self::Output, r: &Self::Output) -> SimpleResult<Self::Output> {
        Ok(l.bvsdiv(&r))
    }
    fn shr(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l.bvlshr(&r) }
    fn shl(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output { l.bvshl(&r) }
    fn neq(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output {
        l._eq(r).not().ite(&self.constant(0), &self.constant(1))
    }
    fn eq(&mut self, l: &Self::Output, r: &Self::Output) -> Self::Output {
        l._eq(r).ite(&self.constant(0), &self.constant(1))
    }
    fn neg(&mut self, l: &Self::Output) -> Self::Output { l.bvneg() }
    fn lookup(&mut self, var: &str) -> SimpleResult<Self::Output> {
        if !self.vars.contains_key(var) {
            let c = z3::ast::BV::fresh_const(self.ctx, var, 64);
            self.vars.insert(var.to_string(), c.clone());
            if var.starts_with("h") {
                self.holes.insert(c, var.to_string());
            }
            else {
                self.const_vars.insert(c);
            }
        }

        Ok(self.vars[var].clone())
    }
}

pub fn synthesize<'a>(
    z3_ctx: &'a z3::Context,
    ast_ctx: &mut ast::Context,
    specification: NodeId,
    template: NodeId,
) -> SimpleResult<HashMap<String,i64>>
{

    let mut vars = HashMap::new();
    let mut holes = HashMap::new();
    let mut const_vars = HashSet::new();
    
    let synth = &mut Synthesize {
        ctx: z3_ctx,
        vars: &mut vars,
        holes: &mut holes,
        const_vars: &mut const_vars,
    };

    let specification = interpret(synth, ast_ctx, specification)?;
    if !synth.holes.is_empty() {
        bail!("specification is not allowed to have holes");
    }

    let template = interpret(synth, ast_ctx, template)?;

    let const_vars: Vec<_> = synth.const_vars.iter().collect();
    // specification == template
    let templ_eq_spec = specification._eq(&template);
    // if no constant vars, then our goal is just to make those equal
    // otherwise, we want that equality to hold over all possible choices
    // for the constant vars
    let goal = if const_vars.is_empty() {
        templ_eq_spec
    }
    else {
        let varspec: Vec<Dynamic> = const_vars
            .iter()
            .map(
                |&v| Dynamic::from_ast(v)
            ).collect();
        let varref: Vec<&Dynamic> = varspec.iter().collect();
        let pat = Pattern::new(z3_ctx, &varref[..]);
        z3::ast::forall_const(
            z3_ctx,
            &[],
            &[&pat],
            &templ_eq_spec.into())
            .as_bool()
            .unwrap()
    };

    // do the solving
    let solver = z3::Solver::new(z3_ctx);
    solver.assert(&goal);
    if solver.check() == z3::SatResult::Sat {
        let model = solver.get_model();
        let mut answers = HashMap::new();
        for (hole, name) in holes {
            let val = model.eval(&hole).unwrap();
            let ival = val.as_i64().unwrap();
            answers.insert(name, ival);
        }
        Ok(answers)
    }
    else {
        bail!("no solution");
    }
}
