use std::collections::HashMap;

use crate::frontend::types::Spanned;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TypeId(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct TypeParamId(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ValueId(usize);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct ExprId(usize);

#[derive(Copy, Clone, Eq, Hash, PartialEq, Debug)]
pub enum Identifier {
    TypeId(TypeId),
    TypeParamId(TypeParamId),
    ValueId(ValueId),
}

#[derive(PartialEq, Debug)]
pub enum ResolvedDefnKind {
    Typedef {
        id: TypeId,
        params: Vec<TypeParamId>,
        arms: Vec<ResolvedConstructorArm>,
    },

    Fn {
        id: ValueId,
        params: Vec<ResolvedBinding>,
        ret_ty: ResolvedType,
        body: Vec<ResolvedStmt>,
    },
}

pub type ResolvedDefn = Spanned<ResolvedDefnKind>;

impl ResolvedDefn {
    pub fn mk_typedef(
        offset: usize,
        id: TypeId,
        params: Vec<TypeParamId>,
        arms: Vec<ResolvedConstructorArm>,
    ) -> Self {
        Spanned {
            offset,
            node: ResolvedDefnKind::Typedef { id, params, arms },
        }
    }

    pub fn mk_fn(
        offset: usize,
        id: ValueId,
        params: Vec<ResolvedBinding>,
        ret_ty: ResolvedType,
        body: Vec<ResolvedStmt>,
    ) -> Self {
        Spanned {
            offset,
            node: ResolvedDefnKind::Fn {
                id,
                params,
                ret_ty,
                body,
            },
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ResolvedStmtKind {
    Let(ValueId, ResolvedExpr),
    Read(ValueId, ResolvedType),
    Echo(ResolvedType, ResolvedExpr),
    Return(ResolvedExpr),
}

pub type ResolvedStmt = Spanned<ResolvedStmtKind>;

impl ResolvedStmt {
    pub fn mk_let(offset: usize, id: ValueId, expr: ResolvedExpr) -> Self {
        Spanned {
            offset,
            node: ResolvedStmtKind::Let(id, expr),
        }
    }

    pub fn mk_read(offset: usize, id: ValueId, typ: ResolvedType) -> Self {
        Spanned {
            offset,
            node: ResolvedStmtKind::Read(id, typ),
        }
    }

    pub fn mk_echo(offset: usize, typ: ResolvedType, expr: ResolvedExpr) -> Self {
        Spanned {
            offset,
            node: ResolvedStmtKind::Echo(typ, expr),
        }
    }

    pub fn mk_return(offset: usize, expr: ResolvedExpr) -> Self {
        Spanned {
            offset,
            node: ResolvedStmtKind::Return(expr),
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ResolvedExprKind {
    Int(i64),
    Bool(bool),
    Float(f64),

    Id(ValueId),

    Fn(ResolvedBinding, Box<ResolvedExpr>),

    Neg(Box<ResolvedExpr>),
    FNeg(Box<ResolvedExpr>),
    Bang(Box<ResolvedExpr>),

    Call(Box<ResolvedExpr>, Vec<ResolvedExpr>),

    Plus(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Minus(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FPlus(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FMinus(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Mult(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FMult(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Div(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FDiv(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Pipe(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Lt(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Gt(Box<ResolvedExpr>, Box<ResolvedExpr>),
    LtEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    GtEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FLt(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FGt(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FLtEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    FGtEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Eq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    NotEq(Box<ResolvedExpr>, Box<ResolvedExpr>),
    Or(Box<ResolvedExpr>, Box<ResolvedExpr>),
    And(Box<ResolvedExpr>, Box<ResolvedExpr>),

    If(Box<ResolvedExpr>, Box<ResolvedExpr>, Box<ResolvedExpr>),
    Match(Box<ResolvedExpr>, Vec<ResolvedMatchArm>),
}

#[derive(PartialEq, Debug)]
pub struct ResolvedExprData {
    id: ExprId,
    kind: ResolvedExprKind,
}

pub type ResolvedExpr = Spanned<ResolvedExprData>;

impl ResolvedExpr {
    pub fn new(offset: usize, id: ExprId, kind: ResolvedExprKind) -> Self {
        Spanned {
            offset,
            node: ResolvedExprData { id, kind },
        }
    }
}

#[derive(PartialEq, Debug)]
pub enum ResolvedTypeKind {
    Int,
    Float,
    Bool,

    TypeId(TypeId),
    TypeParam(TypeParamId),
    TypeApp(TypeId, Vec<ResolvedType>),

    Fn(Box<ResolvedType>, Box<ResolvedType>),
}

pub type ResolvedType = Spanned<ResolvedTypeKind>;

impl ResolvedType {
    pub(super) fn mk_int(offset: usize) -> Self {
        Spanned {
            offset,
            node: ResolvedTypeKind::Int,
        }
    }

    pub(super) fn mk_float(offset: usize) -> Self {
        Spanned {
            offset,
            node: ResolvedTypeKind::Float,
        }
    }

    pub(super) fn mk_bool(offset: usize) -> Self {
        Spanned {
            offset,
            node: ResolvedTypeKind::Bool,
        }
    }

    pub(super) fn mk_typeid(offset: usize, id: TypeId) -> Self {
        Spanned {
            offset,
            node: ResolvedTypeKind::TypeId(id),
        }
    }

    pub(super) fn mk_typeparam(offset: usize, id: TypeParamId) -> Self {
        Spanned {
            offset,
            node: ResolvedTypeKind::TypeParam(id),
        }
    }

    pub(super) fn mk_typeapp(offset: usize, id: TypeId, args: Vec<ResolvedType>) -> Self {
        Spanned {
            offset,
            node: ResolvedTypeKind::TypeApp(id, args),
        }
    }

    pub(super) fn mk_fn(offset: usize, domain: ResolvedType, codomain: ResolvedType) -> Self {
        Spanned {
            offset,
            node: ResolvedTypeKind::Fn(Box::new(domain), Box::new(codomain)),
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ResolvedBindingData {
    id: ValueId,
    typ: ResolvedType,
}

pub type ResolvedBinding = Spanned<ResolvedBindingData>;

impl ResolvedBinding {
    pub fn new(offset: usize, id: ValueId, typ: ResolvedType) -> Self {
        Spanned {
            offset,
            node: ResolvedBindingData { id, typ },
        }
    }
}

impl ResolvedBindingData {
    pub fn new(id: ValueId, typ: ResolvedType) -> Self {
        Self { id, typ }
    }
}

#[derive(PartialEq, Debug)]
pub struct ResolvedMatchArmData {
    id: ValueId,
    bindings: Vec<ValueId>,
    expr: ResolvedExpr,
}

pub type ResolvedMatchArm = Spanned<ResolvedMatchArmData>;

impl ResolvedMatchArm {
    pub fn new(offset: usize, id: ValueId, bindings: Vec<ValueId>, expr: ResolvedExpr) -> Self {
        Self {
            offset,
            node: ResolvedMatchArmData { id, bindings, expr },
        }
    }
}

#[derive(PartialEq, Debug)]
pub struct ResolvedConstructorArmData {
    id: ValueId,
    bindings: Vec<ResolvedBinding>,
}

pub type ResolvedConstructorArm = Spanned<ResolvedConstructorArmData>;

impl ResolvedConstructorArm {
    pub fn new(offset: usize, id: ValueId, bindings: Vec<ResolvedBinding>) -> Self {
        Self {
            offset,
            node: ResolvedConstructorArmData { id, bindings },
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum ResolveErrorKind<'src> {
    DuplicateValue { name: &'src str },
    DuplicateType { name: &'src str },
    UnknownValue { name: &'src str },
    UnknownType { name: &'src str },
    MissingMainFn,
}

pub type ResolveError<'src> = Spanned<ResolveErrorKind<'src>>;
pub type ResolveResult<'src, T> = Result<T, ResolveError<'src>>;

pub struct ResolverCtx<'src> {
    num_typeids: usize,
    num_typeparamids: usize,
    num_valueids: usize,
    num_exprs: usize,

    pub(super) ids: HashMap<Identifier, &'src str>,

    env: Vec<Env<'src>>,
}

impl<'src> ResolverCtx<'src> {
    pub(crate) fn new() -> Self {
        Self {
            num_typeids: 0,
            num_typeparamids: 0,
            num_valueids: 0,
            num_exprs: 0,
            ids: HashMap::new(),
            env: vec![Env::new()],
        }
    }

    pub(super) fn next_typeid(&mut self) -> TypeId {
        let id = TypeId(self.num_typeids);
        self.num_typeids += 1;
        id
    }

    pub(super) fn next_typeparamid(&mut self) -> TypeParamId {
        let id = TypeParamId(self.num_typeparamids);
        self.num_typeparamids += 1;
        id
    }

    pub(super) fn next_valueid(&mut self) -> ValueId {
        let id = ValueId(self.num_valueids);
        self.num_valueids += 1;
        id
    }

    pub(super) fn next_exprid(&mut self) -> ExprId {
        let id = ExprId(self.num_exprs);
        self.num_exprs += 1;
        id
    }

    pub(super) fn add_value(&mut self, name: &'src str) -> ValueId {
        let id = self.next_valueid();
        self.ids.insert(Identifier::ValueId(id), name);
        // insert into the current scope so lookups find newly-declared names
        if let Some(env) = self.env.last_mut() {
            env.values.insert(name, Identifier::ValueId(id));
        } else {
            unreachable!(
                "COMPILER BUG: improper management of scopes when adding value {}. please report this error",
                name
            );
        }
        id
    }

    pub(super) fn add_value_nodup(&mut self, name: &'src str, at: usize) -> ValueId {
        match self.lookup_value(name) {
            Some(val) => {
                self.add_duplicate_value_error(name, at);
                val
            }
            None => self.add_value(name),
        }
    }

    pub(super) fn add_type(&mut self, name: &'src str) -> TypeId {
        let id = self.next_typeid();
        self.ids.insert(Identifier::TypeId(id), name);
        if let Some(env) = self.env.last_mut() {
            env.values.insert(name, Identifier::TypeId(id));
        } else {
            unreachable!(
                "COMPILER BUG: improper management of scopes when adding type {}. please report this error",
                name
            );
        }
        id
    }

    /// adds a type to the current scope but bans duplicate names. if a duplicate is found, an error
    /// is added to the context, but the existing type is returned so that resolution can continue
    ///
    /// # Arguments - `name` - the name of the type to add - `at` - the offset in the source code
    /// where the type is being added
    ///
    /// # Returns
    ///
    /// the `TypeId` of the newly added type, or the existing type if a duplicate was found
    ///
    /// # Examples
    ///
    /// this succeeds, and returns the type id of the newly added type:
    ///
    /// ```rust
    /// let mut ctx = ResolverCtx::new();
    /// let result = ctx.add_type_nodup("MyType", 42); // adds MyType to the current scope
    /// ```
    ///
    /// this fails, and returns the type id of the existing type, while also adding an error to the
    /// context:
    ///
    /// ```rust
    /// let mut ctx = ResolverCtx::new();
    /// ctx.add_type_nodup("MyType", 42); // adds MyType to the current scope
    /// ctx.add_type_nodup("MyType", 43); // fails, adds an error to the context, and returns the existing type id
    /// ```
    pub(super) fn add_type_nodup(&mut self, name: &'src str, at: usize) -> TypeId {
        match self.lookup_type(name) {
            Some(val) => {
                self.add_duplicate_type_error(name, at);
                val
            }
            None => self.add_type(name),
        }
    }

    pub(super) fn add_typeparam(&mut self, name: &'src str) -> TypeParamId {
        let id = self.next_typeparamid();
        self.ids.insert(Identifier::TypeParamId(id), name);
        if let Some(env) = self.env.last_mut() {
            env.values.insert(name, Identifier::TypeParamId(id));
        } else {
            unreachable!(
                "COMPILER BUG: improper management of scopes when adding type param {}. please report this error",
                name
            );
        }
        id
    }

    pub(super) fn push_scope(&mut self) {
        self.env.push(Env::new());
    }

    pub(super) fn pop_scope(&mut self) {
        self.env.pop();
    }

    pub(super) fn lookup(&self, name: &str) -> Option<Identifier> {
        for env in self.env.iter().rev() {
            if let Some(id) = env.values.get(name) {
                return Some(*id);
            }
        }
        None
    }

    pub(super) fn lookup_value(&self, name: &str) -> Option<ValueId> {
        match self.lookup(name) {
            Some(Identifier::ValueId(id)) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn lookup_type(&self, name: &str) -> Option<TypeId> {
        match self.lookup(name) {
            Some(Identifier::TypeId(id)) => Some(id),
            _ => None,
        }
    }

    pub(crate) fn lookup_typeparam(&self, name: &str) -> Option<TypeParamId> {
        match self.lookup(name) {
            Some(Identifier::TypeParamId(id)) => Some(id),
            _ => None,
        }
    }
}

pub struct Env<'src> {
    values: HashMap<&'src str, Identifier>,
}

impl<'src> Default for Env<'src> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'src> Env<'src> {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
}

pub struct ResolveSuccess<'src> {
    pub resolved_defns: Vec<ResolvedDefn>,
    pub main_id: ValueId,
    pub id_to_name: HashMap<Identifier, &'src str>,
}
