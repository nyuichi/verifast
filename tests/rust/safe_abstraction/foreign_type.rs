#![allow(dead_code)]
#![feature(extern_types)]

extern "C" {
    type Foo;
}

fn take_ptr(p: *mut Foo) {
    let _ = p;
}
