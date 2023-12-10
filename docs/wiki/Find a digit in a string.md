# Find a digit in a string

```rust
fn find(line: &String) -> u32 {
    line.chars()
        .find(|ch| ch.is_digit(10))
        .map(|ch| ch.to_digit(10))
        .unwrap()
        .unwrap()
}
```
