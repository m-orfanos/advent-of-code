# Match a pattern within a string

There are dozens of string-searching algorithms but you usually don't need to know how to implement them; instead rely of the programming language's standard library.

```rust
let haystack = "....";
let needle = "...";

// returns the index of the FIRST match, otherwise None
let i: Option<usize> = haystack.find(needle);

// returns the index of the LAST match, otherwise None
let j: Option<usize> = haystack.rfind(needle);
```
