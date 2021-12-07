use std::mem::{size_of, size_of_val};

// #[repr(packed(8))]
#[repr(C)]
#[derive(Default)]
struct Foo {
    a: u8,
    b: u64,
    c: u8,
    d: u32,
}

fn main() {
    let foo = &Foo::default();
    let foo_addr = foo as *const Foo;
    let foo_a_addr = (&foo.a) as *const _ as usize;
    let foo_b_addr = (&foo.b) as *const _ as usize;
    let foo_c_addr = (&foo.c) as *const _ as usize;
    let foo_d_addr = (&foo.d) as *const _ as usize;
    let foo_b_rel_offset = foo_b_addr - foo_a_addr;
    let foo_c_total_offset = foo_c_addr - foo_a_addr;
    let foo_c_rel_offset = foo_c_addr - foo_b_addr;
    let foo_d_total_offset = foo_d_addr - foo_a_addr;
    let foo_d_rel_offset = foo_d_addr - foo_c_addr;
    let foo_b_padding = foo_b_rel_offset - size_of_val(&foo.a);
    let foo_c_padding = foo_c_rel_offset - size_of_val(&foo.b);
    let foo_d_padding = foo_d_rel_offset - size_of_val(&foo.c);
    println!("sizeof Foo: {}", size_of::<Foo>());
    println!("Foo @ {:?}", foo_addr);
    println!("  .a: offset= 0: rel_offset=0, padding=0");
    println!("  .b: offset={:>2}, rel_offset={}, padding={}", foo_b_rel_offset, foo_b_rel_offset, foo_b_padding);
    println!("  .c: offset={:>2}, rel_offset={}, padding={}", foo_c_total_offset, foo_c_rel_offset, foo_c_padding);
    println!("  .d: offset={:>2}, rel_offset={}, padding={}", foo_d_total_offset, foo_d_rel_offset, foo_d_padding);
}
