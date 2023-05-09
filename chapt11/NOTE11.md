# Traits and Generics
Generics and traits are closely related: generic functions use traits in bounds to spell out what types of arguments they can be applied to.

## Using Traits
There is one unusual rule about trait methods: the trait itself must be in scope. Otherwise, all its methods are hidden.

