# `self_assign`

Perform refactor-safe explicit self assigning.

## Usage

```rust
struct S {
    a: u64,
    b: String,
    c: String,
}

let mut s = S {
    a: 0,
    b: "test".into(),
    c: "foo".into(),
};

let c = "bar".to_string();

self_assign::self_assign! {
    s = S {
        a: 1,
        b: _,
        c,
    };
};

assert_eq!(s.a, 1);
assert_eq!(s.b, "test");
assert_eq!(s.c, "bar");
```
