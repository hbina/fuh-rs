# fuh-rs 
## Fold Universality Hutton in Rust

Implementation of Graham Hutton's A Tutorial on the Universality and Expressiveness of Fold in Rust.

## TODO

1. While the author claims that `fold` have the universal property, what does it say when we still need some other constructs to do meaningful computations?
For instance, we still need `Vec` for growable arrays, `if-else` for logic and basically all other non-`fold` operations.
2. To that point, how to do `if-else` using `fold`?
3. Return a generic `Iterator` instead of `Vec`.
