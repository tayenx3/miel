# Miel, a programming language - How To Memory

## Affine Protection

Miel's type system revolves around a concept called "Affine Protection".
Every type is either "affine" or "normal".
Affine types get affine protection while normal types are copied on every use (except the last use).

Affine protection creates constraints by using move semantics.
When affine values are no longer used, they are immediately "dropped".
Affine types must define a "drop function" that will run when the value is dropped
as a "clean-up action".

The only inherently affine primitive type is `box T`,
while `T!E` is either normal or affine depending on its type parameters.

When you create a struct or enum type, you can have a `#affine` directive *(I'll decide on the syntax for directives later)*
which marks it as "affine".

Every value is inherently stored on the stack, `box T` helps you make heap values safely.
However, there is also a `std.mem.Alloc` function but it's deemed unsafe
because it returns `*T` and pointers don't have affine protection.

(`Vec` uses `malloc` but hopefully, i can make it safe for users)
