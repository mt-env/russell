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
