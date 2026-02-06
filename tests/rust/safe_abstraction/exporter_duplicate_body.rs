#![allow(dead_code)]

fn outer() {
    fn inner() {}
    inner();
}
