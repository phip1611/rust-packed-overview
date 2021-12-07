# Behaviour of `packed(N)` in Rust: Short Experimental Overview

TL;DR: In most cases you probably want to use `packed`/`packed(1)` and use a wrapping struct,
if alignment is necessary.

Recently I learned, that the `packed(N)`-attribute [[0]] doesn't work as I 
expected. I thought, that `packed` always means that any padding 
between the fields is stripped, and that `N` stands for the alignment of the
struct itself. **This is not true!!** I think this is counterintuitive.

In fact, `packed` can only be used to lower the alignment of a type plus the alignment of
each single field. It can never increase the alignment. This means, that the
`#[repr(packed)]` modifier on a struct aligns each single field to a 
one byte boundary, which will in fact eliminate any padding between fields.
For example a struct with `packed(2)` representation can still contain padding.

The fact that you can't mix `packed(N)` and `align(N)` on the same struct implies,
that if you want to have a N-aligned struct without any padding between the fields
and exact control over the type layout, you have to do the following:

```rust
#[repr(align(4096))]
struct Aligned4096<T>(T);
// plus impl convenient methods

#[repr(C, packed)]
struct Foo {
    a: u8,
    b: u64,
    c: u16,
    d: u8,
}
// plus impl convenient methods

fn main() {
    let aligned_foo = Aligned4096(Foo::new());
}
```

Hint: `packed` is equal to `packed(1)`.

A typical use case for packed structs is the exchange of data via an ABI, for 
example via a wire. With `repr(C)` we get a guaranteed ordering of the fields,
because Rust doesn't have a stable ABI yet.

## Experiment

I did some experiments with the code in `src/main.rs`. These are my results:

| N                   | total size of Foo | padding a <-> b | padding b <-> c | padding c <-> d |
|---------------------|-------------------|-----------------|-----------------|-----------------|
| no packed attribute | 24                | 7               | 0               | 3               |
| 1                   | 14                | 0               | 0               | 0               |
| 2                   | 16                | 1               | 0               | 1               |
| 4                   | 20                | 3               | 0               | 3               |
| 8                   | 24                | 7               | 0               | 3               |

- All numbers are in bytes.
- These results were obtained with Rust 1.57.0 stable.

[0]: https://doc.rust-lang.org/reference/type-layout.html