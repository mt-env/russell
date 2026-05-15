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

A special function, `fn main() -> Int`, is the entry point of the program. All statements in `main` will be executed sequentially.

=== Statements
A statement of the form `let <id> = <expr>;` evaluates `<expr>` and binds the result to `<id>` in the current environment. Subsequent statements may reference `<id>`.

A statement of the form `read <type> <id>;` reads a line from standard input, parses it as `<type>`, and binds the result to `<id>` in the current environment. `<type>` must be one of `Int`, `Float`, or `Bool`; any other type will cause an error.

A statement of the form `echo <type> <expr>;` evaluates `<expr>` and prints the result to standard output, followed by a newline.

A statement of the form `return <expr>;` evaluates `<expr>` and returns the result as the value of the enclosing function. All remaining statements in the function are not executed.

=== Expressions
Integers, floats, and booleans are in their canonical form.

Identifiers will be replaced by the value they represent, stored in the current environment.

A closure represents a function with a single argument. It captures the environment at the time of creation, and thus its sub-expression may reference variables in the enclosing environment.

Expressions of the form `- <A>` will evaluate to an expression equivalent to `0 - <A>`. Expressions of the form `! <B>` will evaluate to an expression equivalent to `if <B> then false else true`.

Expressions of the form `<B>(<A>, ...)` will evaluate to:
- If `<B>` evaluates to a closure, `<A>` is bound to the closure's parameter and the body is evaluated in the closure's captured environment extended with that binding. The expression will cause an error if more or fewer than one argument is provided.
- If `<B>` evaluates to a named function, each argument is bound to the corresponding parameter and the function body is evaluated. The expression will cause an error if the number of arguments does not match the number of parameters.
- If `<B>` evaluates to a constructor, each argument is bound to the corresponding field and a value of the constructor's type is produced. The expression will cause an error if the number of arguments does not match the number of fields.
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> + <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the integer `<A> + <B>`
- If `<A>` and `<B>` both evaluate to floats, the integer `<A> + <B>`
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> - <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the integer `<A> - <B>`
- If `<A>` and `<B>` both evaluate to floats, the integer `<A> - <B>`
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> * <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the integer `<A> * <B>`
- If `<A>` and `<B>` both evaluate to floats, the integer `<A> * <B>`
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> / <B>` are:
- If `<A>` and `<B>` both evaluate to integers, the integer `<A> / <B>`
- If `<A>` and `<B>` both evaluate to floats, the integer `<A> / <B>`
- Otherwise, the expression will cause a type error.

Expressions of the form `<A> |> <B>` will evaluate to an expression equivalent to `<B>(<A>)`. Note that this will cause an error if `<B>` does not have an arity of one.

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
- User-defined types, written as a `<typeId>`, introduced by `typedef` definitions.
- Function types `<type> -> <type>`. The `->` constructor is right-associative, so `A -> B -> C` denotes `A -> (B -> C)`. A closure, which always takes a single argument, has a type of this form. A named global function `fn f(x1: T1, ..., xn: Tn) -> R` is not curried; it has the multi-argument function type with parameter types `T1, ..., Tn` and return type `R`, and must be applied to all of its arguments at once.
- Generic type variables, written as a `<typeId>`. A `<typeId>` is treated as a generic type variable whenever no `typedef` of that name is in scope in the global environment. Generic variables stand for any type and are unified across their uses within a single definition.

The global environment is populated by the top-level definitions of the program:
- A `typedef <typeId> { ... }` binds `<typeId>` to a user-defined algebraic data type, and binds each constructor `<id>` to a function type producing a value of `<typeId>`. A nullary constructor is bound to a zero-argument function type, not directly to a value of `<typeId>`; constructing the ADT value requires an explicit call.
- A `fn <id>(<binding>, ...) -> <type> { ... }` binds `<id>` to the corresponding function type.

When the type system encounters a `<typeId>` in a `<type>` annotation, it looks it up in the global environment. If a `typedef` of that name exists, the annotation refers to that user-defined type. If no such `typedef` exists, the `<typeId>` is automatically assumed to be a generic type variable; no explicit quantifier is required. Within a single definition, every occurrence of the same `<typeId>` refers to the same generic variable, and distinct `<typeId>`s refer to distinct generic variables.

For example, in
```
fn id(x: A) -> A { return x; }
fn const(x: A, y: B) -> A { return x; }
```
`A` and `B` are generic because no `typedef A` or `typedef B` exists; `id` is a single-argument function from `A` to `A`, and `const` is a two-argument function from `(A, B)` to `A`. By contrast, if `typedef A { ... }` is present, then `A` in a function signature refers to that concrete type rather than a generic.

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

A program is well-typed if every definition's body type-checks against its declared signature. The entry point `main` must have type `() -> Int`.

=== Binary Operators
The grammar contains a variety of binary operators. All binary expressions are left-associative (i.e., `+`, `-`, `*`, `/`, `<`, `<=`, `>`, `>=`, `==`, `!=`, `||`, `&&`, `|>`). Function type annotations (`->`) are right-associative.

The precedence of binary expressions follows the below chart.
#table(
  columns: 3,
  [*Level*], [*Operators*], [*Notes*],
  [8 (highest)], [`f(...)`], [function call (postfix)],
  [7], [`*`, `/`], [multiplicative],
  [6], [`+`, `-`], [additive],
  [5], [`<`, `<=`, `>`, `>=`], [relational],
  [4], [`==`, `!=`], [equality],
  [3], [`&&`], [logical and],
  [2], [`||`], [logical or],
  [1 (lowest)], [`|>`], [pipe],
)
