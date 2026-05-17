#show heading: set align(center)

= Russell

== Lexical Syntax
Comments start from a `//` and occur until a newline character.

Russell has the following primitive data:
```
<integer> ::= [0-9]+
<float>   ::= [0-9]+.[0-9]+
<bool>    ::= true
            | false
<id>      ::= [a-z][a-zA-Z0-9_]*
<typeId>  ::= [A-Z][A-Za-z]*
```

Russell has the following keywords:
```
echo
else
false
fn
if
let
match
read
return
then
true
typedef
```

...and the following type keywords:
```
Int
Float
Bool
```

Russell has the following reserved symbols:
```
!
!=
&&
(
)
*
+
,
-
->
/
:
;
<
<=
=
==
>
>=
{
|>
||
}
```

== Syntax
A Russell program is a list of definitions, and obeys the following grammar.
```
<program> ::= <defn>, ...

<defn> ::= typedef <typeId> { <id> ( <binding> , ... ) , ... }
         | fn <id>( <binding> , ... ) -> <type> { <stmnt>; ... }

<stmnt> ::= let <id> = <expr>;
          | read <type> <id>;
          | echo <type> <expr>;
          | return <expr>;

<expr> ::= <integer>
         | <float>
         | <bool>
         | <id>
         | fn ( <binding> ) -> <expr>
         | - <expr>
         | ! <expr>
         | <expr>(<expr>)
         | <expr> + <expr>
         | <expr> - <expr>
         | <expr> * <expr>
         | <expr> / <expr>
         | <expr> |> <expr>
         | <expr> < <expr>
         | <expr> <= <expr>
         | <expr> > <expr>
         | <expr> >= <expr>
         | <expr> == <expr>
         | <expr> != <expr>
         | <expr> || <expr>
         | <expr> && <expr>
         | if <expr> then <expr> else <expr>
         | ( <expr> )
         | match <expr> { <id> ( <binding> , ...) -> <expr> , ... }

<type> ::= Int
         | Float
         | Bool
         | <typeId>
         | <type> -> <type>

<binding> ::= <id> : <type>
```

== Semantics
Here we define the expected output of a Russell program.

=== Definitions
A definition of the form `typedef <typeId> { <id>(<binding>, ...), ... }` introduces an algebraic data type named `<typeId>`. Each arm introduces a constructor `<id>` into the global environment. When called with arguments matching its bindings, a constructor produces a value of type `<typeId>`.

Constructors are global functions. A `typedef` arm `<id>(<binding>, ...)` is equivalent to introducing a global function named `<id>` that takes those bindings and returns a value of `<typeId>`. In particular, a constructor with zero bindings is a zero-arity function: writing the bare name `<id>` refers to the function value itself, and `<id>()` is required to construct the ADT value. There is no implicit nullary-constructor application --- constructors are uniformly invoked via function-call syntax, just like any other function.

A definition of the form `fn <id>(<binding>, ...) -> <type> { <stmnt>; ... }` introduces a named function `<id>` into the global environment.

A special function, `fn main() -> Int`, is the entry point of the program. All statements in `main` will be executed sequentially. The `Int` returned by `main` is the process exit code: when execution of `main` reaches a `return <expr>;`, the program terminates and `<expr>`'s value is reported to the operating system as the exit status.

=== Statements
A statement of the form `let <id> = <expr>;` evaluates `<expr>` and binds the result to `<id>` in the current environment. Subsequent statements may reference `<id>`.

A statement of the form `read <type> <id>;` reads a line from standard input, parses it as `<type>`, and binds the result to `<id>` in the current environment. `<type>` must be one of `Int`, `Float`, or `Bool`; any other type will cause an error.

A statement of the form `echo <type> <expr>;` evaluates `<expr>` and prints the result to standard output, followed by a newline. Floats are always rendered with a decimal point (e.g. `1.0`, not `1`) so that they remain visually distinguishable from integers.

A statement of the form `return <expr>;` evaluates `<expr>` and returns the result as the value of the enclosing function. All remaining statements in the function are not executed.

=== Scoping
Russell has two scopes: the global environment, populated by top-level definitions, and a local environment per function activation, populated by parameters and `let`/`read` bindings.

Within a function body, bindings introduced by `let <id> = <expr>;` and `read <type> <id>;` are visible only to statements that textually follow them in the same block; statements before the binding cannot reference `<id>`. A binding's right-hand side is evaluated in the environment that existed before the binding statement, so `let x = x;` refers to whatever `x` (if any) was already in scope, not to the `x` being introduced.

A `let` or `read` binding may shadow any earlier binding of the same `<id>`, including a function parameter or a previous `let`/`read` in the same body. After the shadowing binding, the inner name resolves to the new value; the outer binding is not destroyed but is hidden for the remainder of the block, and is not recoverable. (Note that the global environment is not subject to shadowing: see the no-name-collisions rule for global definitions.)

A `match` expression introduces a fresh local sub-scope for each arm. The arm's `<binding>`s are added to the current local environment and are visible only inside that arm's body `<expr>`; they go out of scope as soon as the arm finishes evaluating, and are not visible in sibling arms or in any statement following the `match`. Arm bindings may shadow names from the enclosing local environment, following the same rules as `let`/`read`. Closures created inside a `fn (...) -> <expr>` similarly form their own sub-scope: the closure's parameter is in scope only inside its body.

The global environment is populated all at once before any function executes, so top-level functions may be mutually recursive: a function may freely call any other top-level function (or itself) regardless of source-file order. The same holds for constructors and `typedef` references in signatures.

Russell has no global variables: the global environment contains only function, constructor, and type names introduced by top-level definitions, and there is no top-level `let`. Closures therefore have no globals to "close over"; they capture only the local environment of the enclosing function. References to top-level names from inside a closure are resolved by ordinary lookup in the global environment at the time the closure runs.

=== Expressions
Integers, floats, and booleans are in their canonical form.

Identifiers will be replaced by the value they represent, stored in the current environment.

A closure is written `fn ( <binding> ) -> <expr>` and represents an anonymous function. As reflected in the grammar, a closure takes exactly one parameter (a single `<binding>`) and has an expression body, not a block: the body is a single `<expr>` whose value is the closure's return value, with no statements, no `return` keyword, and no surrounding braces. This is intentionally distinct from a named function definition, which takes a comma-separated list of bindings and a block body of one or more statements terminated by a `return`. A closure captures the environment at the time of its creation, and its body may reference variables in the enclosing environment.

To express the effect of a multi-argument anonymous function, a closure must be curried by hand: `fn (x: A) -> fn (y: B) -> <expr>` is a closure of type `A -> (B -> C)` and must be applied one argument at a time, since each `(<expr>)` application accepts exactly one argument when the callee is a closure.

Expressions of the form `- <A>` will evaluate to an expression equivalent to `0 - <A>`. Expressions of the form `! <B>` will evaluate to an expression equivalent to `if <B> then false else true`.

Expressions of the form `<B>(<A>, ...)` will evaluate to:
- If `<B>` evaluates to a closure, `<A>` is bound to the closure's parameter and the body is evaluated in the closure's captured environment extended with that binding. The expression will cause an error if more or fewer than one argument is provided.
- If `<B>` evaluates to a named function, each argument is bound to the corresponding parameter and the function body is evaluated. The expression will cause an error if the number of arguments does not match the number of parameters.
- If `<B>` evaluates to a constructor, each argument is bound to the corresponding field and a value of the constructor's type is produced. The expression will cause an error if the number of arguments does not match the number of fields.
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> + <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the integer `<A> + <B>`
- If `<A>` and `<B>` both evaluate to floats, the float `<A> + <B>`
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> - <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the integer `<A> - <B>`
- If `<A>` and `<B>` both evaluate to floats, the float `<A> - <B>`
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> * <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the integer `<A> * <B>`
- If `<A>` and `<B>` both evaluate to floats, the float `<A> * <B>`
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> / <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the integer `<A> / <B>`
- If `<A>` and `<B>` both evaluate to floats, the float `<A> / <B>`
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> |> <B>` are evaluated by first evaluating `<B>` to a value, then applying that value to `<A>` as its sole argument. In other words, `<A> |> <B>` is equivalent to `<B>(<A>)` where `<B>` is itself an arbitrary expression that is evaluated before the call, not a syntactic insertion of `<A>` into `<B>`'s argument list. As a consequence:
- If `<B>` is an identifier bound to a unary function, `<A> |> <B>` simply calls that function with `<A>`.
- If `<B>` is a call expression such as `f(y)`, then `f(y)` is evaluated first, and the resulting value is then called with `<A>`. To thread `<A>` through `f` alongside `y`, `f(y)` must itself return a unary function (i.e., `f` must be curried).
- The expression will cause an error if `<B>` does not evaluate to a callable value of arity one.

Note in particular that `|>` is *not* syntactic sugar for inserting `<A>` as an extra argument into a call on the right-hand side; the right-hand side is an ordinary expression that must evaluate to a unary function.

Expressions of the form `<A> < <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the Boolean true if and only if `<A>` is less than `<B>`.
- If `<A>` and `<B>` both evaluate to floats, the Boolean true if and only if `<A>` is less than `<B>`.
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> <= <B>` will evaluate to an expression equivalent to `<A> < <B> || <A> == <B>`.

Expressions of the form `<A> > <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the Boolean true if and only if `<A>` is greater than `<B>`.
- If `<A>` and `<B>` both evaluate to floats, the Boolean true if and only if `<A>` is greater than `<B>`.
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> >= <B>` will evaluate to an expression equivalent to `<A> > <B> || <A> == <B>`.

Expressions of the form `<A> == <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the Boolean true if and only if `<A>` is equal to `<B>`.
- If `<A>` and `<B>` both evaluate to floats, the Boolean true if and only if `<A>` is equal to `<B>`. Note that this uses bitwise equality.
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> != <B>` will evaluate to an expression equivalent to `! (<A> == <B>)`.

Expressions of the form `<A> || <B>` will evaluate to an expression equivalent to `if <A> then true else <B>`.

Expressions of the form `<A> && <B>` will evaluate to an expression equivalent to `if <A> then <B> else false`.

Expressions of the form `if <A> then <B> else <C>` will evaluate to:
- If `<A>` is a Boolean and `<A>` is true, then `<B>`
- If `<A>` is a Boolean and `<A>` is false, then `<C>`
- Otherwise, the expression will cause a type error.

Expressions of the form `( <expr> )` will evaluate to an expression equivalent to `<expr>`.

Expressions of the form `match <A> { <id>(<binding>, ...) -> <B>, ... }` will evaluate to:
- `<A>` must evaluate to an ADT value. Each arm is tested in order; the first arm whose constructor name matches the ADT's constructor is selected. The ADT's fields are bound to the arm's bindings, and `<B>` is evaluated in the environment extended with those bindings.
- If no arm matches the constructor, the expression will cause an error.
- If `<A>` does not evaluate to an ADT value, the expression will cause a type error.

Pattern syntax mirrors construction syntax: because constructors are uniformly functions, even a nullary constructor must be written `<id>()` in an arm, never as a bare `<id>`. The parentheses are part of the constructor pattern, not optional sugar.

=== Type System
Russell is statically typed. Every expression has a type, and programs that cannot be assigned consistent types are rejected before evaluation.

The set of types is:
- The primitives `Int`, `Float`, and `Bool`.
- User-defined types, written as a `<typeId>`, introduced by `typedef` definitions. User-defined types are concrete in the current version of the language: a `typedef` introduces a single nullary type, not a parameterized type constructor.
- Function types `<type> -> <type>`. The `->` constructor is right-associative, so `A -> B -> C` denotes `A -> (B -> C)`. A closure, which always takes a single argument, has a type of this form. A named global function `fn f(x1: T1, ..., xn: Tn) -> R` is not curried; it has the multi-argument function type with parameter types `T1, ..., Tn` and return type `R`, and must be applied to all of its arguments at once.
- Generic type variables, written as a `<typeId>`. A `<typeId>` is treated as a generic type variable whenever no `typedef` of that name is in scope in the global environment. Generic variables stand for any type and are unified across their uses within a single definition.

The global environment is populated by the top-level definitions of the program:
- A `typedef <typeId> { ... }` binds `<typeId>` to a user-defined algebraic data type. Each constructor `<id>` is bound to a function type whose argument types are the constructor's binding types and whose result is the user-defined type. A nullary constructor is bound to a zero-argument function type, not directly to a value of the user-defined type; constructing the ADT value requires an explicit call.

Generics in `typedef` are not supported in the current version of the language. Every `<typeId>` mentioned inside a `typedef`'s constructor bindings must resolve to a primitive type or to another `typedef` in the global environment; a `<typeId>` that would otherwise be inferred as a generic type variable is rejected at compile time as a type error. This restriction is intended to be lifted in a future version of the language, at which point `typedef`s will become parameterized type constructors and constructors will receive polymorphic types. Until then, polymorphic data structures (e.g. `Option`, `List`, `Pair`) cannot be expressed as `typedef`s; programs that need them must either monomorphize manually (one `typedef` per concrete element type) or wait for the extension.
- A `fn <id>(<binding>, ...) -> <type> { ... }` binds `<id>` to the corresponding function type.

No name collisions are permitted in the global environment. Every `<typeId>` introduced by a `typedef`, every constructor `<id>` introduced by a `typedef` arm, and every `<id>` introduced by an `fn` definition must be distinct. Type names and value names share a single global namespace: a program that introduces the same name twice, whether as two `typedef`s, two functions, two constructors (across any `typedef`s), or any combination of these, is rejected at compile time.

When the type system encounters a `<typeId>` in a `<type>` annotation, it looks it up in the global environment. If a `typedef` of that name exists, the annotation refers to that user-defined type. If no such `typedef` exists, the `<typeId>` is automatically assumed to be a generic type variable; no explicit quantifier is required. Within a single definition, every occurrence of the same `<typeId>` refers to the same generic variable, and distinct `<typeId>`s refer to distinct generic variables. (As noted above, this implicit-generic rule does *not* apply inside `typedef` constructor bindings, where every `<typeId>` must resolve to a concrete type.)

For example, in
```
fn id(x: A) -> A { return x; }
fn const(x: A, y: B) -> A { return x; }
```
`A` and `B` are generic because no `typedef A` or `typedef B` exists; `id` is a single-argument function from `A` to `A`, and `const` is a two-argument function from `(A, B)` to `A`. By contrast, if `typedef A { ... }` is present, then `A` in a function signature refers to that concrete type rather than a generic.

Note that constructor names are `<id>`s, which are lowercase-leading; `<typeId>` is reserved for the type itself. This is inverted from the ML convention.

Russell uses Hindley-Milner let-polymorphism. The type variables of a definition are universally quantified over its whole signature, and each use site of the definition is given a fresh instantiation of those quantifiers. The same definition may therefore be used at multiple instantiations within a single program: for instance, `id` above may be called as `id(3)` (instantiating `A` to `Int`) and `id(true)` (instantiating `A` to `Bool`) in the same program. Instantiation is inferred from the surrounding context; no explicit type-application syntax is required.

Polymorphism is preserved across `let` bindings. If `let f = id;` appears in a function body, `f` is bound to a polymorphic value of type `A -> A` (with `A` fresh per use), and may itself be applied at multiple types within its scope. By contrast, names introduced by function parameters and closure parameters are monomorphic within their scope: a parameter `x: A` stands for a single fixed (though possibly still unknown) type throughout the body in which it is bound. This is the standard HM stratification: generalization occurs at top-level definitions and at `let`, but not at lambda-bound names.

As a consequence, Russell does not support first-class polymorphism: a function parameter whose declared type is generic receives a single instantiation chosen by the caller, and the function body cannot use that argument at two different types. Polymorphic recursion (a recursive call to a generic function at a different instantiation than the enclosing definition was given) is likewise not supported.

The compiler implements polymorphism by monomorphization: for every distinct instantiation of a generic definition that appears in the program, a separate specialized copy of that definition is generated. There is no single runtime-polymorphic value for a generic function; each instantiation is a distinct compiled function. A program in which the type variables of some use site cannot be resolved to concrete types after inference is a type error.

The typing rules for expressions are:
- An `<integer>` has type `Int`. A `<float>` has type `Float`. A `<bool>` has type `Bool`.
- An `<id>` has the type bound to it in the current environment.
- A closure `fn (x: T) -> e` has type `T -> U`, where `U` is the type of `e` in the environment extended with `x: T`.
- An application `<B>(<A1>, ..., <An>)` requires `<B>` to have a function type whose parameter types match `<A1>, ..., <An>` in number and type; the result has the function's return type. A closure has exactly one parameter and must be applied to exactly one argument. A named global function or constructor must be applied to all of its parameters at once.
- `- <A>` requires `<A>` to have type `Int` or `Float`, and has the same type as `<A>`.
- `! <B>` requires `<B>` to have type `Bool`, and has type `Bool`.
- The arithmetic operators `+`, `-`, `*`, `/` require both operands to have the same type, which must be `Int` or `Float`; the result has that type.
- The relational operators `<`, `<=`, `>`, `>=` require both operands to have the same type, which must be `Int` or `Float`; the result has type `Bool`.
- The equality operators `==` and `!=` require both operands to have the same type, which must be `Int` or `Float`; the result has type `Bool`.
- The logical operators `&&` and `||` require both operands to have type `Bool`; the result has type `Bool`.
- The pipe `<A> |> <B>` is typed as `<B>(<A>)`.
- `if <A> then <B> else <C>` requires `<A>` to have type `Bool`, and `<B>` and `<C>` to have the same type, which becomes the type of the whole expression.
- `match <A> { <id>(<binding>, ...) -> <B>, ... }` requires `<A>` to have a user-defined type `<typeId>`, each arm's constructor `<id>` to belong to `<typeId>`, each arm's bindings to match the constructor's field types, and each arm's body `<B>` to have the same type, which becomes the type of the whole expression.

The typing rules for statements are:
- `let <id> = <expr>;` extends the current environment with `<id>` bound to the type of `<expr>`.
- `read <type> <id>;` requires `<type>` to be one of `Int`, `Float`, or `Bool`, and extends the current environment with `<id>` bound to `<type>`.
- `echo <type> <expr>;` requires `<expr>` to have type `<type>`.
- `return <expr>;` requires `<expr>` to have the enclosing function's declared return type.

Every execution path through a function body must reach a `return` statement. A function whose body can finish without executing a `return` is rejected as a type error at compile time; control is never permitted to fall off the end of a function. A `return` does not need to be the textually last statement; any statements following it in the same block are unreachable dead code, which is permitted but never executed.

A program is well-typed if every definition's body type-checks against its declared signature. The entry point `main` must have type `() -> Int`.

=== Operator Precedence and Associativity
All binary expressions are left-associative (i.e., `+`, `-`, `*`, `/`, `<`, `<=`, `>`, `>=`, `==`, `!=`, `||`, `&&`, `|>`). Function type annotations (`->`) are right-associative. The unary operators `-` and `!` are prefix and bind tighter than any binary operator but looser than function application.

`if`, `match`, and closure (`fn (...) -> <expr>`) expressions extend as far to the right as possible: their trailing sub-expression greedily consumes any following operators, so they sit below all binary operators in precedence and must be parenthesized to appear as an operand of a binary operator.

The precedence of expressions follows the below chart.
#table(
  columns: 3,
  [*Level*], [*Operators*], [*Notes*],
  [9 (highest)], [`f(...)`], [function call (postfix)],
  [8], [unary `-`, `!`], [prefix],
  [7], [`*`, `/`], [multiplicative],
  [6], [`+`, `-`], [additive],
  [5], [`<`, `<=`, `>`, `>=`], [relational],
  [4], [`==`, `!=`], [equality],
  [3], [`&&`], [logical and],
  [2], [`||`], [logical or],
  [1], [`|>`], [pipe],
  [0 (lowest)], [`if`, `match`, `fn`], [trailing-expression forms],
)
