use std::{collections::HashMap, rc::Rc};

use crate::{
    frontend::parser::ast::{Binding, Expr},
    interpreter::treewalk::{Env, interp_fn::interp_fn, types::Value},
};

pub(super) fn interp_expr(expr: &Expr, env: Rc<Env>) -> Rc<Value> {
    match expr {
        Expr::Int(num) => Value::Int(*num).into(),
        Expr::Float(num) => Value::Float(*num).into(),
        Expr::Bool(val) => Value::Bool(*val).into(),
        Expr::Id(id) => interp_id(id, env),
        Expr::Fn(binding, expr) => Value::Closure(Rc::clone(&env), binding.clone(), expr.clone()).into(),
        Expr::Neg(expr) => interp_neg(expr, env),
        Expr::Bang(expr) => interp_bang(expr, env),
        Expr::Call(func, args) => interp_call(func, args.iter().collect(), env),
        Expr::Plus(left, right) => interp_arith_binop(left, right, env, |l, r| l + r, |l, r| l + r),
        Expr::Minus(left, right) => interp_arith_binop(left, right, env, |l, r| l - r, |l, r| l - r),
        Expr::Mult(left, right) => interp_arith_binop(left, right, env, |l, r| l * r, |l, r| l * r),
        Expr::Div(left, right) => interp_arith_binop(left, right, env, |l, r| l / r, |l, r| l / r),
        Expr::Pipe(left, right) => interp_call(right, vec![left], env),
        Expr::Less(left, right) => interp_cmp_binop(left, right, env, |l, r| l < r, |l, r| l < r),
        Expr::LessEq(left, right) => interp_cmp_binop(left, right, env, |l, r| l <= r, |l, r| l <= r),
        Expr::Greater(left, right) => interp_cmp_binop(left, right, env, |l, r| l > r, |l, r| l > r),
        Expr::GreaterEq(left, right) => interp_cmp_binop(left, right, env, |l, r| l >= r, |l, r| l >= r),
        Expr::Eq(left, right) => interp_cmp_binop(left, right, env, |l, r| l == r, |l, r| l == r),
        Expr::NotEq(left, right) => interp_cmp_binop(left, right, env, |l, r| l != r, |l, r| l != r),
        Expr::Or(left, right) => interp_if(left, &Expr::Bool(true), right, env),
        Expr::And(left, right) => interp_if(left, right, &Expr::Bool(false), env),
        Expr::If(cond, then_expr, else_expr) => interp_if(cond, then_expr, else_expr, env),
        Expr::Match(expr, arms) => interp_match(expr, arms, env),
    }
}

fn interp_id(id: &String, env: Rc<Env>) -> Rc<Value> {
    match env.lookup(id) {
        Some(val) => Rc::clone(&val),
        None => panic!("FATAL ERROR: unbound identifier {id}"),
    }
}

fn interp_neg(expr: &Expr, env: Rc<Env>) -> Rc<Value> {
    match &*interp_expr(expr, env) {
        Value::Int(num) => Value::Int(-num).into(),
        Value::Float(num) => Value::Float(-num).into(),
        val => panic!("FATAL ERROR: expected numeric value, found {val:?}"),
    }
}

fn interp_bang(expr: &Expr, env: Rc<Env>) -> Rc<Value> {
    match &*interp_expr(expr, env) {
        Value::Bool(val) => Value::Bool(!val).into(),
        val => panic!("FATAL ERROR: expected boolean value, found {val:?}"),
    }
}

fn bind_args(env: Rc<Env>, params: Vec<&Binding>, args: Vec<&Expr>) -> Rc<Env> {
    if params.len() != args.len() {
        panic!("FATAL ERROR: expected {} arguments, found {}", params.len(), args.len());
    }

    let mut local_env = Rc::clone(&env);
    for (binding, arg) in params.into_iter().zip(args) {
        let arg_val = interp_expr(arg, Rc::clone(&local_env));
        local_env = local_env.extend(binding.id.clone(), arg_val);
    }

    local_env
}

fn interp_call(func: &Expr, args: Vec<&Expr>, env: Rc<Env>) -> Rc<Value> {
    match &*interp_expr(func, Rc::clone(&env)) {
        Value::Closure(closure_env, binding, body) => {
            let local_env = bind_args(Rc::clone(closure_env), vec![binding], args);
            interp_expr(body, local_env)
        }

        Value::Fn(name, bindings, stmts) => {
            let local_env = bind_args(Rc::clone(&env), bindings.iter().collect(), args);
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

fn interp_arith_binop(
    left: &Expr,
    right: &Expr,
    env: Rc<Env>,
    int_op: fn(i64, i64) -> i64,
    float_op: fn(f64, f64) -> f64,
) -> Rc<Value> {
    let left_val = interp_expr(left, Rc::clone(&env));
    let right_val = interp_expr(right, env);
    match (&*left_val, &*right_val) {
        (Value::Int(l), Value::Int(r)) => Value::Int(int_op(*l, *r)).into(),
        (Value::Float(l), Value::Float(r)) => Value::Float(float_op(*l, *r)).into(),
        (l, r) => panic!("FATAL ERROR: type mismatch: {l:?} and {r:?}"),
    }
}

fn interp_cmp_binop(
    left: &Expr,
    right: &Expr,
    env: Rc<Env>,
    int_op: fn(i64, i64) -> bool,
    float_op: fn(f64, f64) -> bool,
) -> Rc<Value> {
    let left_val = interp_expr(left, Rc::clone(&env));
    let right_val = interp_expr(right, env);
    match (&*left_val, &*right_val) {
        (Value::Int(l), Value::Int(r)) => Value::Bool(int_op(*l, *r)).into(),
        (Value::Float(l), Value::Float(r)) => Value::Bool(float_op(*l, *r)).into(),
        (l, r) => panic!("FATAL ERROR: type mismatch: {l:?} and {r:?}"),
    }
}

fn interp_if(cond: &Expr, then_expr: &Expr, else_expr: &Expr, env: Rc<Env>) -> Rc<Value> {
    let cond_val = interp_expr(cond, Rc::clone(&env));
    match &*cond_val {
        Value::Bool(true) => interp_expr(then_expr, env),
        Value::Bool(false) => interp_expr(else_expr, env),
        val => panic!("FATAL ERROR: expected boolean value, found {val:?}"),
    }
}

fn interp_match(expr: &Expr, arms: &Vec<(String, Vec<Binding>, Expr)>, env: Rc<Env>) -> Rc<Value> {
    let expr_val = interp_expr(expr, Rc::clone(&env));
    match &*expr_val {
        Value::Adt(adt_type, constructor, fields) => {
            for (arm_constructor, arm_bindings, arm_expr) in arms {
                if constructor == arm_constructor {
                    let mut local_env = Rc::clone(&env);
                    for binding in arm_bindings {
                        match fields.get(&binding.id) {
                            Some(val) => local_env = local_env.extend(binding.id.clone(), Rc::clone(val)),
                            None => panic!("FATAL ERROR: expected field {} in constructor {}, found none", binding.id, constructor),
                        }
                    }
                    return interp_expr(arm_expr, local_env);
                }
            }
            panic!("FATAL ERROR: no match arms matched");
        }

        val => panic!("FATAL ERROR: expected ADT value in match expression, found {val:?}"),
    }
}
