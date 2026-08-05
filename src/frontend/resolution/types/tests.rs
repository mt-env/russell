use super::{Env, Identifier, ResolverCtx};

#[test]
fn lookup_returns_none_for_unknown_name() {
    let ctx = ResolverCtx::new();

    assert_eq!(ctx.lookup("missing"), None);
    assert_eq!(ctx.lookup_value("missing"), None);
    assert_eq!(ctx.lookup_type("Missing"), None);
    assert_eq!(ctx.lookup_typeparam("'missing"), None);
}

#[test]
fn registers_each_identifier_kind() {
    let mut ctx = ResolverCtx::new();
    let value = ctx.add_value("value");
    let type_id = ctx.add_type("Type");
    let parameter = ctx.add_typeparam("'parameter");

    assert_eq!(ctx.lookup("value"), Some(Identifier::ValueId(value)));
    assert_eq!(ctx.lookup("Type"), Some(Identifier::TypeId(type_id)));
    assert_eq!(
        ctx.lookup("'parameter"),
        Some(Identifier::TypeParamId(parameter))
    );
    assert_eq!(ctx.lookup_value("Type"), None);
    assert_eq!(ctx.lookup_type("value"), None);
}

#[test]
fn inner_scope_shadows_outer_scope_and_pop_restores_it() {
    let mut ctx = ResolverCtx::new();
    let outer = ctx.add_value("x");

    ctx.push_scope();
    let inner = ctx.add_value("x");
    assert_eq!(ctx.lookup_value("x"), Some(inner));

    ctx.pop_scope();
    assert_eq!(ctx.lookup_value("x"), Some(outer));
}

#[test]
fn inner_bindings_do_not_leak_after_scope_is_popped() {
    let mut ctx = ResolverCtx::new();
    ctx.push_scope();
    ctx.add_value("temporary");

    ctx.pop_scope();

    assert_eq!(ctx.lookup_value("temporary"), None);
}

#[test]
fn lookup_walks_all_enclosing_scopes() {
    let mut ctx = ResolverCtx::new();
    let outer = ctx.add_value("outer");
    ctx.push_scope();
    let middle = ctx.add_value("middle");
    ctx.push_scope();
    let inner = ctx.add_value("inner");

    assert_eq!(ctx.lookup_value("outer"), Some(outer));
    assert_eq!(ctx.lookup_value("middle"), Some(middle));
    assert_eq!(ctx.lookup_value("inner"), Some(inner));
}

#[test]
fn rebinding_in_same_scope_uses_latest_id() {
    let mut ctx = ResolverCtx::new();
    let first = ctx.add_value("x");
    let second = ctx.add_value("x");

    assert_ne!(first, second);
    assert_eq!(ctx.lookup_value("x"), Some(second));
}

#[test]
fn id_generators_are_monotonic_and_independent() {
    let mut ctx = ResolverCtx::new();

    let first_value = ctx.next_valueid();
    let second_value = ctx.next_valueid();
    let first_type = ctx.next_typeid();
    let second_type = ctx.next_typeid();
    let first_parameter = ctx.next_typeparamid();
    let second_parameter = ctx.next_typeparamid();

    assert_ne!(first_value, second_value);
    assert_ne!(first_type, second_type);
    assert_ne!(first_parameter, second_parameter);
}

#[test]
fn default_environment_is_empty() {
    let env = Env::default();

    assert!(env.values.is_empty());
}

#[test]
#[should_panic(expected = "No current scope to add value to")]
fn adding_value_without_scope_panics() {
    let mut ctx = ResolverCtx::new();
    ctx.pop_scope();

    ctx.add_value("value");
}

#[test]
#[should_panic(expected = "No current scope to add type to")]
fn adding_type_without_scope_panics() {
    let mut ctx = ResolverCtx::new();
    ctx.pop_scope();

    ctx.add_type("Type");
}

#[test]
#[should_panic(expected = "No current scope to add type parameter to")]
fn adding_type_parameter_without_scope_panics() {
    let mut ctx = ResolverCtx::new();
    ctx.pop_scope();

    ctx.add_typeparam("'parameter");
}
