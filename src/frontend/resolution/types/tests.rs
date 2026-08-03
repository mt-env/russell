use super::{Identifier, ResolverCtx};

#[test]
fn lookup_returns_none_for_unknown_name() {
    let ctx = ResolverCtx::new();
    assert!(ctx.lookup("missing").is_none());
}

#[test]
fn add_value_registers_name_in_current_scope() {
    let mut ctx = ResolverCtx::new();
    let value_id = ctx.add_value("x");

    assert!(matches!(
        ctx.lookup("x"),
        Some(Identifier::ValueId(id)) if id == value_id
    ));
}

#[test]
fn lookup_prefers_inner_scope_then_restores_outer_on_pop() {
    let mut ctx = ResolverCtx::new();
    let outer = ctx.add_value("x");

    ctx.push_scope();
    let inner = ctx.add_value("x");
    assert!(matches!(
        ctx.lookup("x"),
        Some(Identifier::ValueId(id)) if id == inner
    ));

    ctx.pop_scope();
    assert!(matches!(
        ctx.lookup("x"),
        Some(Identifier::ValueId(id)) if id == outer
    ));
}

#[test]
fn lookup_does_not_leak_inner_bindings_after_pop() {
    let mut ctx = ResolverCtx::new();
    ctx.push_scope();
    ctx.add_value("temp");
    assert!(ctx.lookup("temp").is_some());

    ctx.pop_scope();
    assert!(ctx.lookup("temp").is_none());
}

#[test]
fn lookup_finds_bindings_across_multiple_nested_scopes() {
    let mut ctx = ResolverCtx::new();
    let outer = ctx.add_value("outer");
    ctx.push_scope();
    ctx.add_value("mid");
    ctx.push_scope();
    let inner = ctx.add_value("inner");

    assert!(matches!(
        ctx.lookup("outer"),
        Some(Identifier::ValueId(id)) if id == outer
    ));
    assert!(matches!(
        ctx.lookup("inner"),
        Some(Identifier::ValueId(id)) if id == inner
    ));
}

#[test]
fn add_value_same_name_in_same_scope_rebinds_to_latest_id() {
    let mut ctx = ResolverCtx::new();
    let first = ctx.add_value("x");
    let second = ctx.add_value("x");

    assert!(first != second);
    assert!(matches!(
        ctx.lookup("x"),
        Some(Identifier::ValueId(id)) if id == second
    ));
}

#[test]
fn next_id_generators_return_unique_ids() {
    let mut ctx = ResolverCtx::new();

    let v1 = ctx.next_valueid();
    let v2 = ctx.next_valueid();
    assert!(v1 != v2);

    let t1 = ctx.next_typeid();
    let t2 = ctx.next_typeid();
    assert!(t1 != t2);

    let p1 = ctx.next_typeparamid();
    let p2 = ctx.next_typeparamid();
    assert!(p1 != p2);
}

#[test]
fn popping_global_scope_removes_bindings() {
    let mut ctx = ResolverCtx::new();
    ctx.add_value("x");
    assert!(ctx.lookup("x").is_some());
    ctx.pop_scope();
    assert!(ctx.lookup("x").is_none());
}
