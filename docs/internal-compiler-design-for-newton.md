# Internal Compiler Design for Newton

This document talks about some specifics on the internal design of the compiler. Hopefully, you can learn more about the compiler here.

## The `Type` API

Initially, the `Type` API was very simple. You could create simple, or more complex types, and types used to hold values. This was dropped, since types don't really need to know what values they hold, that's up to the AST.

