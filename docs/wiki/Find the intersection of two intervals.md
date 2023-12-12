# Find the intersection of two intervals

```rust
fn find_intersection(
	s1: u64, 
	e1: u64, 
	s2: u64, 
	e2: u64) -> Option<(u64, u64)> 
{
    if s2 > e1 || s1 > e2 {
        None
    } else {
        let s = s1.max(s2);
        let e = e1.min(e2);
        Some((s, e))
    }
}
```
