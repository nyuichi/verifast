#![allow(dead_code)]

enum E<'a> {
    A(&'a i32),
    B,
}

fn id<'a>(e: E<'a>) -> E<'a> {
    e
}
