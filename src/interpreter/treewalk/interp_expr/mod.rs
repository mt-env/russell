use std::{collections::HashMap, rc::Rc};

use crate::{
    frontend::parser::ast::{Binding, Expr, ExprKind, ParsedExpr},
    interpreter::treewalk::{Env, interp_fn::interp_fn, types::Value},
};

pub(super) fn interp_expr<'a>(expr: &'a ParsedExpr<'a>, env: Rc<Env<'a>>) -> Rc<Value<'a>> {
    match &expr.kind {
        ExprKind::Int(num) => Value::Int(*num).into(),
        ExprKind::Float(num) => Value::Float(*num).into(),
        ExprKind::Bool(val) => Value::Bool(*val).into(),
        ExprKind::Id(id) => interp_id(id, env),
        ExprKind::Fn(binding, expr) => Value::Closure(Rc::clone(&env), binding.clone(), expr.clone()).into(),
        ExprKind::Neg(expr) => interp_neg(expr, env),
        ExprKind::Bang(expr) => interp_bang(expr, env),
        ExprKind::Call(func, args) => interp_call(func, args.iter().collect(), env),
        ExprKind::Plus(left, right) => interp_arith_binop(left, right, env, |l, r| l + r, |l, r| l + r),
        ExprKind::Minus(left, right) => interp_arith_binop(left, right, env, |l, r| l - r, |l, r| l - r),
        ExprKind::Mult(left, right) => interp_arith_binop(left, right, env, |l, r| l * r, |l, r| l * r),
        ExprKind::Div(left, right) => interp_arith_binop(left, right, env, |l, r| l / r, |l, r| l / r),
        ExprKind::Pipe(left, right) => interp_call(right, vec![left], env),
        ExprKind::Less(left, right) => interp_cmp_binop(left, right, env, |l, r| l < r, |l, r| l < r),
        ExprKind::LessEq(left, right) => interp_cmp_binop(left, right, env, |l, r| l <= r, |l, r| l <= r),
        ExprKind::Greater(left, right) => interp_cmp_binop(left, right, env, |l, r| l > r, |l, r| l > r),
        ExprKind::GreaterEq(left, right) => interp_cmp_binop(left, right, env, |l, r| l >= r, |l, r| l >= r),
        ExprKind::Eq(left, right) => interp_cmp_binop(left, right, env, |l, r| l == r, |l, r| l == r),
        ExprKind::NotEq(left, right) => interp_cmp_binop(left, right, env, |l, r| l != r, |l, r| l != r),
        ExprKind::Or(left, right) => interp_if(left, &Expr::parsed(ExprKind::Bool(true)), right, env),
        ExprKind::And(left, right) => interp_if(left, right, &Expr::parsed(ExprKind::Bool(false)), env),
        ExprKind::If(cond, then_expr, else_expr) => interp_if(cond, then_expr, else_expr, env),
        ExprKind::Match(expr, arms) => interp_match(expr, arms, env),
    }
}

fn interp_id<'a>(id: &'a str, env: Rc<Env<'a>>) -> Rc<Value<'a>> {
    match env.lookup(id) {
        Some(val) => Rc::clone(&val),
        None => panic!("FATAL ERROR: unbound identifier {id}"),
    }
}

fn interp_neg<'a>(expr: &'a ParsedExpr<'a>, env: Rc<Env<'a>>) -> Rc<Value<'a>> {
    match &*interp_expr(expr, env) {
        Value::Int(num) => Value::Int(-num).into(),
        Value::Float(num) => Value::Float(-num).into(),
        val => panic!("FATAL ERROR: expected numeric value, found {val:?}"),
    }
}

fn interp_bang<'a>(expr: &'a ParsedExpr<'a>, env: Rc<Env<'a>>) -> Rc<Value<'a>> {
    match &*interp_expr(expr, env) {
        Value::Bool(val) => Value::Bool(!val).into(),
        val => panic!("FATAL ERROR: expected boolean value, found {val:?}"),
    }
}

fn bind_args<'a>(env: Rc<Env<'a>>, params: Vec<&'a Binding<'a>>, args: Vec<Rc<Value<'a>>>) -> Rc<Env<'a>> {
    if params.len() != args.len() {
        panic!("FATAL ERROR: expected {} arguments, found {}", params.len(), args.len());
    }

    let mut local_env = Rc::clone(&env);
    for (binding, arg) in params.into_iter().zip(args) {
        local_env = Env::extend(local_env, binding.id.clone(), arg);
    }

    local_env
}

fn interp_call<'a>(func: &'a ParsedExpr<'a>, args: Vec<&'a ParsedExpr<'a>>, env: Rc<Env<'a>>) -> Rc<Value<'a>> {
    match &*interp_expr(func, Rc::clone(&env)) {
        Value::Closure(closure_env, binding, body) => {
            let local_env = bind_args(
                Rc::clone(closure_env),
                vec![binding],
                args.iter().map(|arg| interp_expr(arg, Rc::clone(&env))).collect()
            );
            interp_expr(body, local_env)
        }

        Value::Fn(name, bindings, stmts) => {
            let local_env = bind_args(
                env.global(),
                bindings.iter().collect(),
                args.iter().map(|arg| interp_expr(arg, Rc::clone(&env))).collect()
            );
            interp_fn(name, stmts.iter().collect(), local_env)
        }

        Value::Constructor(name, adt_type, bindings) => {
            if bindings.len() != args.len() {
                panic!("FATAL ERROR: expected {} arguments, found {}", bindings.len(), args.len());
            }
            let mut field_vals = HashMap::new();
            for (binding, arg) in bindings.iter().zip(args) {
                field_vals.insert(binding.id.clone(), interp_expr(arg, Rc::clone(&env)));
            }
            Value::Adt(adt_type.clone(), name.clone(), field_vals).into()
        }

        val => panic!("FATAL ERROR: expected function value, found {val:?}"),
    }
}

fn interp_arith_binop<'a>(
    left: &'a ParsedExpr<'a>,
    right: &'a ParsedExpr<'a>,
    env: Rc<Env<'a>>,
    int_op: fn(i64, i64) -> i64,
    float_op: fn(f64, f64) -> f64,
) -> Rc<Value<'a>> {
    let left_val = interp_expr(left, Rc::clone(&env));
    let right_val = interp_expr(right, env);
    match (&*left_val, &*right_val) {
        (Value::Int(l), Value::Int(r)) => Value::Int(int_op(*l, *r)).into(),
        (Value::Float(l), Value::Float(r)) => Value::Float(float_op(*l, *r)).into(),
        (l, r) => panic!("FATAL ERROR: type mismatch: {l:?} and {r:?}"),
    }
}

fn interp_cmp_binop<'a>(
    left: &'a ParsedExpr<'a>,
    right: &'a ParsedExpr<'a>,
    env: Rc<Env<'a>>,
    int_op: fn(i64, i64) -> bool,
    float_op: fn(f64, f64) -> bool,
) -> Rc<Value<'a>> {
    let left_val = interp_expr(left, Rc::clone(&env));
    let right_val = interp_expr(right, env);
    match (&*left_val, &*right_val) {
        (Value::Int(l), Value::Int(r)) => Value::Bool(int_op(*l, *r)).into(),
        (Value::Float(l), Value::Float(r)) => Value::Bool(float_op(*l, *r)).into(),
        (l, r) => panic!("FATAL ERROR: type mismatch: {l:?} and {r:?}"),
    }
}

fn interp_if<'a>(cond: &'a ParsedExpr<'a>, then_expr: &'a ParsedExpr<'a>, else_expr: &'a ParsedExpr<'a>, env: Rc<Env<'a>>) -> Rc<Value<'a>> {
    let cond_val = interp_expr(cond, Rc::clone(&env));
    match &*cond_val {
        Value::Bool(true) => interp_expr(then_expr, env),
        Value::Bool(false) => interp_expr(else_expr, env),
        val => panic!("FATAL ERROR: expected boolean value, found {val:?}"),
    }
}

fn interp_match<'a>(expr: &'a ParsedExpr<'a>, arms: &'a Vec<(&'a str, Vec<Binding<'a>>, ParsedExpr<'a>)>, env: Rc<Env<'a>>) -> Rc<Value<'a>> {
    // check that the value is an ADT
    let expr_val = interp_expr(expr, Rc::clone(&env));
    let Value::Adt(adt_type, constructor, fields) = &*expr_val else {
        panic!("FATAL ERROR: expected ADT value in match expression, found {:?}", expr_val);
    };

    // find the correct constructor and bind it
    for (arm_constructor, arm_bindings, arm_expr) in arms {
        if constructor != arm_constructor {
            continue;
        }
        if fields.len() != arm_bindings.len() {
            panic!("FATAL ERROR: expected {} fields in constructor {}, found {}", arm_bindings.len(), constructor, fields.len());
        }
        let mut local_env = Rc::clone(&env);
        for arm_binding in arm_bindings {
            let Some(field_val) = fields.get(&arm_binding.id) else {
                panic!("FATAL ERROR: no field named {} in constructor {}", arm_binding.id, constructor);
            };
            local_env = Env::extend(local_env, arm_binding.id.clone(), Rc::clone(field_val));
        }
        return interp_expr(arm_expr, local_env);
    }
    panic!("FATAL ERROR: no match arms matched");
}
